//! FIFO behaviour through the Redis backend ( issues #64 – #66 ): stored
//! high-throughput attributes, message-group gating, FIFO system attributes,
//! sequence numbers and SHA-256 content-based deduplication.
//!
//! These tests need a Redis server. Set `WINTERBAUME_TEST_REDIS_URL` (for
//! example `redis://127.0.0.1:6379/15`) to run them; without it they return
//! early. Each test uses uniquely named queues and deletes them afterwards,
//! so the database does not need to be empty.

use std::sync::Arc;

use aws_sdk_sqs::Client;
use aws_sdk_sqs::config::BehaviorVersion;
use aws_sdk_sqs::error::ProvideErrorMetadata;
use aws_sdk_sqs::types::{MessageSystemAttributeName, QueueAttributeName};
use winterbaume_core::MockAws;
use winterbaume_sqs::SqsService;
use winterbaume_sqs_redis::RedisSqsBackend;

/// A client backed by the Redis backend, or `None` when no Redis server is
/// configured.
async fn client() -> Option<Client> {
    let url = std::env::var("WINTERBAUME_TEST_REDIS_URL").ok()?;
    let backend = Arc::new(
        RedisSqsBackend::from_connection_info(url.as_str())
            .await
            .expect("connect to WINTERBAUME_TEST_REDIS_URL"),
    );
    let mock = MockAws::builder()
        .with_service(SqsService::with_backend(backend))
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sqs::config::Region::new("us-east-1"))
        .load()
        .await;
    Some(Client::new(&config))
}

fn unique_queue(prefix: &str, fifo: bool) -> String {
    let suffix = if fifo { ".fifo" } else { "" };
    format!("{prefix}-{}{suffix}", uuid::Uuid::new_v4().simple())
}

async fn create_fifo(
    client: &Client,
    prefix: &str,
    attrs: &[(QueueAttributeName, &str)],
) -> String {
    let mut req = client
        .create_queue()
        .queue_name(unique_queue(prefix, true))
        .attributes(QueueAttributeName::FifoQueue, "true");
    for (k, v) in attrs {
        req = req.attributes(k.clone(), *v);
    }
    req.send().await.unwrap().queue_url().unwrap().to_string()
}

async fn drop_queue(client: &Client, queue_url: &str) {
    client
        .delete_queue()
        .queue_url(queue_url)
        .send()
        .await
        .unwrap();
}

async fn queue_attributes(
    client: &Client,
    queue_url: &str,
) -> std::collections::HashMap<QueueAttributeName, String> {
    client
        .get_queue_attributes()
        .queue_url(queue_url)
        .attribute_names(QueueAttributeName::All)
        .send()
        .await
        .unwrap()
        .attributes()
        .cloned()
        .unwrap_or_default()
}

async fn send(
    client: &Client,
    queue_url: &str,
    body: &str,
    group: &str,
    dedup: Option<&str>,
) -> Option<String> {
    let mut req = client
        .send_message()
        .queue_url(queue_url)
        .message_body(body)
        .message_group_id(group);
    if let Some(d) = dedup {
        req = req.message_deduplication_id(d);
    }
    req.send()
        .await
        .unwrap()
        .sequence_number()
        .map(str::to_string)
}

/// Issue #64: `DeduplicationScope` / `FifoThroughputLimit` are stored on
/// create and `SetQueueAttributes`, and validated as AWS does.
#[tokio::test]
async fn high_throughput_attributes_are_stored_and_validated() {
    let Some(client) = client().await else { return };
    let queue_url = create_fifo(
        &client,
        "hi-tput",
        &[
            (QueueAttributeName::DeduplicationScope, "messageGroup"),
            (QueueAttributeName::FifoThroughputLimit, "perMessageGroupId"),
        ],
    )
    .await;
    let attrs = queue_attributes(&client, &queue_url).await;
    assert_eq!(
        attrs[&QueueAttributeName::DeduplicationScope],
        "messageGroup"
    );
    assert_eq!(
        attrs[&QueueAttributeName::FifoThroughputLimit],
        "perMessageGroupId"
    );
    assert_eq!(attrs[&QueueAttributeName::MaximumMessageSize], "1048576");

    let err = client
        .set_queue_attributes()
        .queue_url(&queue_url)
        .attributes(QueueAttributeName::DeduplicationScope, "queue")
        .send()
        .await
        .unwrap_err();
    assert_eq!(err.code(), Some("InvalidAttributeValue"));

    client
        .set_queue_attributes()
        .queue_url(&queue_url)
        .attributes(QueueAttributeName::DeduplicationScope, "queue")
        .attributes(QueueAttributeName::FifoThroughputLimit, "perQueue")
        .send()
        .await
        .unwrap();
    let attrs = queue_attributes(&client, &queue_url).await;
    assert_eq!(attrs[&QueueAttributeName::DeduplicationScope], "queue");
    assert_eq!(attrs[&QueueAttributeName::FifoThroughputLimit], "perQueue");
    drop_queue(&client, &queue_url).await;

    let err = client
        .create_queue()
        .queue_name(unique_queue("std-scope", false))
        .attributes(QueueAttributeName::DeduplicationScope, "messageGroup")
        .send()
        .await
        .unwrap_err();
    assert_eq!(err.code(), Some("InvalidAttributeName"));
}

/// Issue #65: an in-flight message holds back its group; delete and
/// `ChangeMessageVisibility` to 0 release it, other groups are unaffected.
#[tokio::test]
async fn in_flight_message_blocks_its_group() {
    let Some(client) = client().await else { return };
    let queue_url = create_fifo(&client, "gate", &[]).await;
    send(&client, &queue_url, "first", "g1", Some("d1")).await;
    send(&client, &queue_url, "second", "g1", Some("d2")).await;
    send(&client, &queue_url, "other", "g2", Some("d3")).await;

    let first = client
        .receive_message()
        .queue_url(&queue_url)
        .visibility_timeout(60)
        .send()
        .await
        .unwrap();
    assert_eq!(first.messages()[0].body(), Some("first"));

    let next = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .visibility_timeout(60)
        .send()
        .await
        .unwrap();
    let bodies: Vec<_> = next.messages().iter().filter_map(|m| m.body()).collect();
    assert_eq!(bodies, ["other"]);

    // Requeue `first`: it comes back before `second`.
    client
        .change_message_visibility()
        .queue_url(&queue_url)
        .receipt_handle(first.messages()[0].receipt_handle().unwrap())
        .visibility_timeout(0)
        .send()
        .await
        .unwrap();
    let again = client
        .receive_message()
        .queue_url(&queue_url)
        .visibility_timeout(60)
        .send()
        .await
        .unwrap();
    assert_eq!(again.messages()[0].body(), Some("first"));
    let blocked = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();
    assert!(blocked.messages().is_empty());

    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(again.messages()[0].receipt_handle().unwrap())
        .send()
        .await
        .unwrap();
    let second = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();
    let bodies: Vec<_> = second.messages().iter().filter_map(|m| m.body()).collect();
    assert_eq!(bodies, ["second"]);
    drop_queue(&client, &queue_url).await;
}

/// Issue #66: FIFO system attributes are returned only when requested, and
/// sequence numbers increase and match between send and receive.
#[tokio::test]
async fn fifo_system_attributes_and_sequence_numbers() {
    let Some(client) = client().await else { return };
    let queue_url = create_fifo(&client, "sys-attrs", &[]).await;
    let s1 = send(&client, &queue_url, "a", "g1", Some("d1"))
        .await
        .unwrap();
    let s2 = send(&client, &queue_url, "b", "g2", Some("d2"))
        .await
        .unwrap();
    assert!(s2.parse::<u128>().unwrap() > s1.parse::<u128>().unwrap());
    // A duplicate returns the original sequence number.
    assert_eq!(
        send(&client, &queue_url, "a", "g1", Some("d1"))
            .await
            .unwrap(),
        s1
    );

    // `VisibilityTimeout=0` on receive means "queue default" here, so make
    // the message visible again explicitly.
    let plain = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    assert!(plain.messages()[0].attributes().is_none());
    client
        .change_message_visibility()
        .queue_url(&queue_url)
        .receipt_handle(plain.messages()[0].receipt_handle().unwrap())
        .visibility_timeout(0)
        .send()
        .await
        .unwrap();

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .message_system_attribute_names(MessageSystemAttributeName::All)
        .send()
        .await
        .unwrap();
    let messages = recv.messages();
    assert_eq!(messages.len(), 2);
    let attrs = messages[0].attributes().unwrap();
    assert_eq!(attrs[&MessageSystemAttributeName::MessageGroupId], "g1");
    assert_eq!(
        attrs[&MessageSystemAttributeName::MessageDeduplicationId],
        "d1"
    );
    assert_eq!(attrs[&MessageSystemAttributeName::SequenceNumber], s1);
    assert!(attrs.contains_key(&MessageSystemAttributeName::SenderId));
    drop_queue(&client, &queue_url).await;
}

/// Issue #66: content-based deduplication keys on the SHA-256 of the body,
/// and `DeduplicationScope=messageGroup` scopes ids to a group.
#[tokio::test]
async fn sha256_and_message_group_deduplication() {
    use sha2::{Digest, Sha256};

    let Some(client) = client().await else { return };
    let queue_url = create_fifo(
        &client,
        "sha",
        &[(QueueAttributeName::ContentBasedDeduplication, "true")],
    )
    .await;
    send(&client, &queue_url, "first", "1", None).await;
    let sha = format!("{:x}", Sha256::digest(b"first"));
    send(&client, &queue_url, "second", "2", Some(&sha)).await;
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .message_system_attribute_names(MessageSystemAttributeName::MessageDeduplicationId)
        .send()
        .await
        .unwrap();
    assert_eq!(recv.messages().len(), 1);
    assert_eq!(
        recv.messages()[0].attributes().unwrap()
            [&MessageSystemAttributeName::MessageDeduplicationId],
        sha
    );
    drop_queue(&client, &queue_url).await;

    let queue_url = create_fifo(
        &client,
        "per-group",
        &[(QueueAttributeName::DeduplicationScope, "messageGroup")],
    )
    .await;
    send(&client, &queue_url, "a", "g1", Some("same")).await;
    send(&client, &queue_url, "b", "g2", Some("same")).await;
    send(&client, &queue_url, "a-dup", "g1", Some("same")).await;
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();
    let mut bodies: Vec<_> = recv.messages().iter().filter_map(|m| m.body()).collect();
    bodies.sort();
    assert_eq!(bodies, ["a", "b"]);
    drop_queue(&client, &queue_url).await;
}
