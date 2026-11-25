//! Conditional writes are atomic under concurrency ( issue #61 ), in-memory
//! backend. The scenarios live in `atomic_writes/mod.rs`.

mod atomic_writes;

use atomic_writes::Load;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dynamodb::DynamoDbService;

async fn client() -> Client {
    let mock = MockAws::builder()
        .with_service(DynamoDbService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;
    Client::new(&config)
}

// The window between a condition check and its write is narrow, so each
// scenario repeats a wide race many times. The loads are sized so that every
// scenario but the unconditional `ADD` ( which was already atomic in this
// backend ) failed on each of 10 runs against the check-then-act code.

/// One request per task and round.
const SINGLE_REQUEST: Load = Load {
    rounds: 300,
    tasks: 64,
};

/// Several requests, or several items, per task and round.
const MULTI_STEP: Load = Load {
    rounds: 40,
    tasks: 64,
};

/// Every task retries until it wins, so a round costs `tasks`² requests.
const RETRY_LOOP: Load = Load {
    rounds: 5,
    tasks: 50,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_conditional_update_item_has_one_winner() {
    atomic_writes::update_item_has_one_winner(&client().await, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_guarded_increments_are_not_lost() {
    atomic_writes::guarded_increments_are_not_lost(&client().await, RETRY_LOOP).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_unconditional_add_is_not_lost() {
    atomic_writes::unconditional_add_is_not_lost(&client().await, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_conditional_put_item_has_one_creator() {
    atomic_writes::put_item_has_one_creator(&client().await, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_conditional_delete_item_has_one_deleter() {
    atomic_writes::delete_item_has_one_deleter(&client().await, SINGLE_REQUEST).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_transactions_have_one_winner() {
    atomic_writes::transaction_has_one_winner(&client().await, MULTI_STEP).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_condition_check_guards_another_item() {
    atomic_writes::condition_check_guards_another_item(&client().await, MULTI_STEP).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_partiql_list_append_is_not_lost() {
    atomic_writes::partiql_list_append_is_not_lost(&client().await, MULTI_STEP).await;
}

/// The images a conditional write reports come from the step that evaluated
/// the condition: `ALL_OLD` on success, and the item the condition saw on
/// failure when `ReturnValuesOnConditionCheckFailure` asks for it.
#[tokio::test]
async fn conditional_update_reports_the_images_it_saw() {
    use aws_sdk_dynamodb::types::{
        AttributeValue, ReturnValue, ReturnValuesOnConditionCheckFailure,
    };

    let client = client().await;
    let table = atomic_writes::create_table(&client, "cas-images").await;
    let update = |expected: &str| {
        client
            .update_item()
            .table_name(&table)
            .key("pk", AttributeValue::S("k".into()))
            .update_expression("SET rev = :next")
            .condition_expression("attribute_not_exists(rev) OR rev = :expected")
            .expression_attribute_values(":expected", AttributeValue::N(expected.into()))
            .expression_attribute_values(":next", AttributeValue::N("1".into()))
            .return_values(ReturnValue::AllOld)
            .return_values_on_condition_check_failure(ReturnValuesOnConditionCheckFailure::AllOld)
    };

    // The item does not exist yet: there is no old image.
    assert!(update("0").send().await.unwrap().attributes().is_none());

    // `rev` is 1 now, so expecting 1 succeeds and reports the old image.
    let old = update("1").send().await.unwrap();
    assert_eq!(
        old.attributes().unwrap().get("rev"),
        Some(&AttributeValue::N("1".into()))
    );

    // Expecting 0 fails and hands back the item the condition saw.
    let err = update("0").send().await.unwrap_err().into_service_error();
    let failed = match err {
        aws_sdk_dynamodb::operation::update_item::UpdateItemError::ConditionalCheckFailedException(e) => e,
        other => panic!("expected ConditionalCheckFailedException, got {other:?}"),
    };
    assert_eq!(failed.message(), Some("The conditional request failed"));
    assert_eq!(
        failed.item().unwrap().get("rev"),
        Some(&AttributeValue::N("1".into()))
    );

    // Without the request member the exception carries no item.
    let err = update("0")
        .set_return_values_on_condition_check_failure(None)
        .send()
        .await
        .unwrap_err()
        .into_service_error();
    match err {
        aws_sdk_dynamodb::operation::update_item::UpdateItemError::ConditionalCheckFailedException(e) => {
            assert!(e.item().is_none());
        }
        other => panic!("expected ConditionalCheckFailedException, got {other:?}"),
    }
    atomic_writes::drop_table(&client, &table).await;
}
