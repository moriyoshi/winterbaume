//! Conditional writes are atomic under concurrency ( issue #61 ), Redis
//! backend. The scenarios in `atomic_writes/mod.rs` are a copy of the ones
//! `winterbaume-dynamodb` runs against its in-memory backend.
//!
//! These tests need a Redis server. Set `WINTERBAUME_TEST_REDIS_URL` (for
//! example `redis://127.0.0.1:6379/15`) to run them; without it they return
//! early. Each scenario uses uniquely named tables and deletes them
//! afterwards, so the database does not need to be empty.

mod atomic_writes;

use std::sync::Arc;

use atomic_writes::Load;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dynamodb::DynamoDbService;
use winterbaume_dynamodb_redis::RedisDynamoDbBackend;

/// A client backed by the Redis backend, or `None` when no Redis server is
/// configured.
async fn client() -> Option<Client> {
    let url = std::env::var("WINTERBAUME_TEST_REDIS_URL").ok()?;
    let backend = Arc::new(
        RedisDynamoDbBackend::from_connection_info(url.as_str())
            .await
            .expect("connect to WINTERBAUME_TEST_REDIS_URL"),
    );
    let mock = MockAws::builder()
        .with_service(DynamoDbService::with_backend(backend))
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;
    Some(Client::new(&config))
}

// Every request is several round trips to Redis, which both widens the
// window between a read and its write and makes a round slower than in
// memory, so the loads are smaller than the in-memory ones.

/// One request per task and round.
const SINGLE_REQUEST: Load = Load {
    rounds: 40,
    tasks: 48,
};

/// Several requests, or several items, per task and round.
const MULTI_STEP: Load = Load {
    rounds: 15,
    tasks: 32,
};

/// Every task retries until it wins, so a round costs `tasks`² requests.
const RETRY_LOOP: Load = Load {
    rounds: 3,
    tasks: 24,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_conditional_update_item_has_one_winner() {
    let Some(client) = client().await else { return };
    atomic_writes::update_item_has_one_winner(&client, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_guarded_increments_are_not_lost() {
    let Some(client) = client().await else { return };
    atomic_writes::guarded_increments_are_not_lost(&client, RETRY_LOOP).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_unconditional_add_is_not_lost() {
    let Some(client) = client().await else { return };
    atomic_writes::unconditional_add_is_not_lost(&client, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_conditional_put_item_has_one_creator() {
    let Some(client) = client().await else { return };
    atomic_writes::put_item_has_one_creator(&client, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_conditional_delete_item_has_one_deleter() {
    let Some(client) = client().await else { return };
    atomic_writes::delete_item_has_one_deleter(&client, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_transactions_have_one_winner() {
    let Some(client) = client().await else { return };
    atomic_writes::transaction_has_one_winner(&client, MULTI_STEP).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_condition_check_guards_another_item() {
    let Some(client) = client().await else { return };
    atomic_writes::condition_check_guards_another_item(&client, MULTI_STEP).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_partiql_list_append_is_not_lost() {
    let Some(client) = client().await else { return };
    atomic_writes::partiql_list_append_is_not_lost(&client, MULTI_STEP).await;
}
