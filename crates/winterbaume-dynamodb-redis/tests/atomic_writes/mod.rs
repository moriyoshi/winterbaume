//! Concurrency scenarios for conditional writes ( issue #61 ).
//!
//! A conditional write is atomic: the condition is evaluated against the
//! item and the write applied as one step. Every scenario races many tasks
//! on one item and checks an invariant that only holds if no write can land
//! between another write's condition check and its application.
//!
//! The scenarios take a client, so the same file is used for the in-memory
//! backend ( `winterbaume-dynamodb` ) and the Redis backend
//! ( `winterbaume-dynamodb-redis`, which keeps an identical copy ). They need
//! a multi-threaded runtime to race for real.

use std::collections::HashMap;
use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, BillingMode, ConditionCheck, KeySchemaElement, KeyType,
    Put, ReturnValue, ReturnValuesOnConditionCheckFailure, ScalarAttributeType, TransactWriteItem,
    Update,
};
use tokio::sync::Barrier;

/// How hard a scenario races.
#[derive(Clone, Copy)]
pub struct Load {
    /// Independent repetitions, each on a fresh item.
    pub rounds: usize,
    /// Tasks released together on that item.
    pub tasks: usize,
}

fn s(v: impl Into<String>) -> AttributeValue {
    AttributeValue::S(v.into())
}

fn n(v: impl ToString) -> AttributeValue {
    AttributeValue::N(v.to_string())
}

fn num(item: &HashMap<String, AttributeValue>, attr: &str) -> i64 {
    item.get(attr)
        .and_then(|v| v.as_n().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("attribute {attr} is not a number in {item:?}"))
}

/// Create an on-demand table keyed by `pk` (S) under a unique name.
pub async fn create_table(client: &Client, prefix: &str) -> String {
    let table = format!("{prefix}-{}", uuid::Uuid::new_v4().simple());
    client
        .create_table()
        .table_name(&table)
        .billing_mode(BillingMode::PayPerRequest)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    table
}

pub async fn drop_table(client: &Client, table: &str) {
    client
        .delete_table()
        .table_name(table)
        .send()
        .await
        .unwrap();
}

async fn get(client: &Client, table: &str, pk: &str) -> Option<HashMap<String, AttributeValue>> {
    client
        .get_item()
        .table_name(table)
        .key("pk", s(pk))
        .consistent_read(true)
        .send()
        .await
        .unwrap()
        .item
}

/// Run `tasks` copies of `job` ( given its index ), released together, and
/// return their results.
async fn race<T, F, Fut>(tasks: usize, job: F) -> Vec<T>
where
    T: Send + 'static,
    F: Fn(usize) -> Fut,
    Fut: std::future::Future<Output = T> + Send + 'static,
{
    let barrier = Arc::new(Barrier::new(tasks));
    let handles: Vec<_> = (0..tasks)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            let fut = job(i);
            tokio::spawn(async move {
                barrier.wait().await;
                fut.await
            })
        })
        .collect();
    let mut results = Vec::with_capacity(tasks);
    for handle in handles {
        results.push(handle.await.expect("task panicked"));
    }
    results
}

/// The first half of the issue: many `UpdateItem` calls with the same
/// condition. Exactly one may win; the others get the typed
/// `ConditionalCheckFailedException`.
pub async fn update_item_has_one_winner(client: &Client, load: Load) {
    let table = create_table(client, "cas-update").await;
    for round in 0..load.rounds {
        let pk = format!("k{round}");
        client
            .put_item()
            .table_name(&table)
            .item("pk", s(&pk))
            .item("rev", n(0))
            .item("n", n(0))
            .send()
            .await
            .unwrap();

        let results = race(load.tasks, |_| {
            let request = client
                .update_item()
                .table_name(&table)
                .key("pk", s(&pk))
                .update_expression("SET rev = :one ADD n :one")
                .condition_expression("rev = :zero")
                .expression_attribute_values(":zero", n(0))
                .expression_attribute_values(":one", n(1))
                .return_values(ReturnValue::AllNew);
            async move { request.send().await }
        })
        .await;

        let mut winners = 0;
        for result in results {
            match result {
                Ok(output) => {
                    winners += 1;
                    assert_eq!(num(output.attributes().unwrap(), "n"), 1);
                }
                Err(e) => assert!(
                    e.as_service_error()
                        .is_some_and(|e| e.is_conditional_check_failed_exception()),
                    "a loser must get ConditionalCheckFailedException, got {e:?}"
                ),
            }
        }
        let item = get(client, &table, &pk).await.unwrap();
        assert_eq!(
            (winners, num(&item, "n")),
            (1, 1),
            "round {round}: (winners, n) for one `rev = :zero` condition"
        );
    }
    drop_table(client, &table).await;
}

/// The second half of the issue: revision-guarded read-modify-write
/// increments with retry must not lose an update.
pub async fn guarded_increments_are_not_lost(client: &Client, load: Load) {
    let table = create_table(client, "cas-increment").await;
    for round in 0..load.rounds {
        let pk = format!("inc{round}");
        client
            .put_item()
            .table_name(&table)
            .item("pk", s(&pk))
            .item("rev", n(0))
            .item("n", n(0))
            .send()
            .await
            .unwrap();

        race(load.tasks, |_| {
            let client = client.clone();
            let table = table.clone();
            let pk = pk.clone();
            async move {
                loop {
                    let current = get(&client, &table, &pk).await.unwrap();
                    let rev = num(&current, "rev");
                    let result = client
                        .update_item()
                        .table_name(&table)
                        .key("pk", s(&pk))
                        .update_expression("SET rev = :next, n = :n")
                        .condition_expression("rev = :cur")
                        .expression_attribute_values(":cur", n(rev))
                        .expression_attribute_values(":next", n(rev + 1))
                        .expression_attribute_values(":n", n(num(&current, "n") + 1))
                        .send()
                        .await;
                    match result {
                        Ok(_) => return,
                        Err(e) => assert!(
                            e.as_service_error()
                                .is_some_and(|e| e.is_conditional_check_failed_exception()),
                            "unexpected error {e:?}"
                        ),
                    }
                }
            }
        })
        .await;

        let item = get(client, &table, &pk).await.unwrap();
        let expected = load.tasks as i64;
        assert_eq!(
            (num(&item, "n"), num(&item, "rev")),
            (expected, expected),
            "round {round}: (n, rev) after {expected} guarded increments"
        );
    }
    drop_table(client, &table).await;
}

/// Unconditional `ADD` is a read-modify-write inside the backend; it must not
/// lose increments either.
pub async fn unconditional_add_is_not_lost(client: &Client, load: Load) {
    let table = create_table(client, "cas-add").await;
    for round in 0..load.rounds {
        let pk = format!("add{round}");
        race(load.tasks, |_| {
            let request = client
                .update_item()
                .table_name(&table)
                .key("pk", s(&pk))
                .update_expression("ADD n :one")
                .expression_attribute_values(":one", n(1));
            async move { request.send().await.unwrap() }
        })
        .await;
        let item = get(client, &table, &pk).await.unwrap();
        assert_eq!(num(&item, "n"), load.tasks as i64, "round {round}");
    }
    drop_table(client, &table).await;
}

/// `PutItem` with `attribute_not_exists(pk)`: exactly one creator wins and
/// the stored item is the winner's. A loser that asks for it is handed the
/// item its condition was evaluated against.
pub async fn put_item_has_one_creator(client: &Client, load: Load) {
    let table = create_table(client, "cas-put").await;
    for round in 0..load.rounds {
        let pk = format!("p{round}");
        let results = race(load.tasks, |i| {
            let request = client
                .put_item()
                .table_name(&table)
                .item("pk", s(&pk))
                .item("owner", n(i))
                .condition_expression("attribute_not_exists(pk)")
                .return_values_on_condition_check_failure(
                    ReturnValuesOnConditionCheckFailure::AllOld,
                );
            async move { (i, request.send().await) }
        })
        .await;

        let stored_owner = num(&get(client, &table, &pk).await.unwrap(), "owner");
        let mut winners = Vec::new();
        for (i, result) in results {
            match result {
                Ok(_) => winners.push(i as i64),
                Err(e) => match e.into_service_error() {
                    PutItemError::ConditionalCheckFailedException(e) => {
                        // Only the winner ever writes, so the item a loser
                        // was checked against is the winner's.
                        assert_eq!(num(e.item().expect("ALL_OLD item"), "owner"), stored_owner);
                    }
                    other => {
                        panic!("a loser must get ConditionalCheckFailedException, got {other:?}")
                    }
                },
            }
        }
        assert_eq!(
            winners,
            vec![stored_owner],
            "round {round}: creators that succeeded vs. the stored owner"
        );
    }
    drop_table(client, &table).await;
}

/// `DeleteItem` with `attribute_exists(pk)`: exactly one deleter wins, and
/// its `ALL_OLD` image is the item it deleted.
pub async fn delete_item_has_one_deleter(client: &Client, load: Load) {
    let table = create_table(client, "cas-delete").await;
    for round in 0..load.rounds {
        let pk = format!("d{round}");
        client
            .put_item()
            .table_name(&table)
            .item("pk", s(&pk))
            .item("round", n(round))
            .send()
            .await
            .unwrap();

        let results = race(load.tasks, |_| {
            let request = client
                .delete_item()
                .table_name(&table)
                .key("pk", s(&pk))
                .condition_expression("attribute_exists(pk)")
                .return_values(ReturnValue::AllOld);
            async move { request.send().await }
        })
        .await;

        let mut winners = 0;
        for result in results {
            match result {
                Ok(output) => {
                    winners += 1;
                    let old = output.attributes().expect("the winner gets ALL_OLD");
                    assert_eq!(num(old, "round"), round as i64);
                }
                Err(e) => assert!(
                    e.as_service_error()
                        .is_some_and(|e| e.is_conditional_check_failed_exception()),
                    "a loser must get ConditionalCheckFailedException, got {e:?}"
                ),
            }
        }
        assert_eq!(winners, 1, "round {round}: deleters that succeeded");
        assert!(get(client, &table, &pk).await.is_none());
    }
    drop_table(client, &table).await;
}

/// `TransactWriteItems` whose actions carry conditions: of many transactions
/// claiming one lock item, exactly one commits, and only its other write is
/// applied. The losers are cancelled with positional reasons.
pub async fn transaction_has_one_winner(client: &Client, load: Load) {
    let table = create_table(client, "cas-transact").await;
    client
        .put_item()
        .table_name(&table)
        .item("pk", s("gate"))
        .send()
        .await
        .unwrap();
    for round in 0..load.rounds {
        let lock = format!("lock{round}");
        client
            .put_item()
            .table_name(&table)
            .item("pk", s(&lock))
            .send()
            .await
            .unwrap();

        let results = race(load.tasks, |i| {
            let request = client
                .transact_write_items()
                .transact_items(
                    TransactWriteItem::builder()
                        .condition_check(
                            ConditionCheck::builder()
                                .table_name(&table)
                                .key("pk", s("gate"))
                                .condition_expression("attribute_exists(pk)")
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .transact_items(
                    TransactWriteItem::builder()
                        .update(
                            Update::builder()
                                .table_name(&table)
                                .key("pk", s(&lock))
                                .update_expression("SET holder = :me")
                                .condition_expression("attribute_not_exists(holder)")
                                .expression_attribute_values(":me", n(i))
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .transact_items(
                    TransactWriteItem::builder()
                        .put(
                            Put::builder()
                                .table_name(&table)
                                .item("pk", s(format!("ledger{round}-{i}")))
                                .build()
                                .unwrap(),
                        )
                        .build(),
                );
            async move { (i, request.send().await) }
        })
        .await;

        let mut winners = Vec::new();
        for (i, result) in results {
            match result {
                Ok(_) => winners.push(i as i64),
                Err(e) => {
                    let cancelled = match e.into_service_error() {
                        TransactWriteItemsError::TransactionCanceledException(e) => e,
                        other => panic!("expected a cancellation, got {other:?}"),
                    };
                    let codes: Vec<_> = cancelled
                        .cancellation_reasons()
                        .iter()
                        .map(|r| r.code().unwrap_or_default())
                        .collect();
                    assert_eq!(codes, ["None", "ConditionalCheckFailed", "None"]);
                }
            }
        }
        let holder = num(&get(client, &table, &lock).await.unwrap(), "holder");
        assert_eq!(winners, vec![holder], "round {round}: committed vs. holder");
        for i in 0..load.tasks {
            let ledger = get(client, &table, &format!("ledger{round}-{i}")).await;
            assert_eq!(
                ledger.is_some(),
                i as i64 == holder,
                "round {round}: only the winner's ledger entry may exist ( task {i} )"
            );
        }
    }
    drop_table(client, &table).await;
}

/// A `ConditionCheck` on one item guards a write to another. Transactions
/// that count while a flag is open race one that closes the flag and records
/// the count at that moment: once it is closed, nothing may count any more.
pub async fn condition_check_guards_another_item(client: &Client, load: Load) {
    let table = create_table(client, "cas-check").await;
    for round in 0..load.rounds {
        let flag = format!("flag{round}");
        let counter = format!("counter{round}");
        for (pk, attr, value) in [(&flag, "state", s("open")), (&counter, "n", n(0))] {
            client
                .put_item()
                .table_name(&table)
                .item("pk", s(pk))
                .item(attr, value)
                .send()
                .await
                .unwrap();
        }

        let closer = load.tasks / 2;
        let results = race(load.tasks, |i| {
            let request = if i == closer {
                client
                    .transact_write_items()
                    .transact_items(
                        TransactWriteItem::builder()
                            .update(
                                Update::builder()
                                    .table_name(&table)
                                    .key("pk", s(&flag))
                                    .update_expression("SET #s = :closed")
                                    .expression_attribute_names("#s", "state")
                                    .expression_attribute_values(":closed", s("closed"))
                                    .build()
                                    .unwrap(),
                            )
                            .build(),
                    )
                    .transact_items(
                        TransactWriteItem::builder()
                            .update(
                                Update::builder()
                                    .table_name(&table)
                                    .key("pk", s(&counter))
                                    .update_expression("SET closed_at = n")
                                    .build()
                                    .unwrap(),
                            )
                            .build(),
                    )
            } else {
                client
                    .transact_write_items()
                    .transact_items(
                        TransactWriteItem::builder()
                            .condition_check(
                                ConditionCheck::builder()
                                    .table_name(&table)
                                    .key("pk", s(&flag))
                                    .condition_expression("#s = :open")
                                    .expression_attribute_names("#s", "state")
                                    .expression_attribute_values(":open", s("open"))
                                    .build()
                                    .unwrap(),
                            )
                            .build(),
                    )
                    .transact_items(
                        TransactWriteItem::builder()
                            .update(
                                Update::builder()
                                    .table_name(&table)
                                    .key("pk", s(&counter))
                                    .update_expression("ADD n :one")
                                    .expression_attribute_values(":one", n(1))
                                    .build()
                                    .unwrap(),
                            )
                            .build(),
                    )
            };
            async move { request.send().await }
        })
        .await;

        let mut committed = 0;
        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(_) if i != closer => committed += 1,
                Ok(_) => {}
                Err(e) => {
                    let e = e.into_service_error();
                    assert!(
                        i != closer && e.is_transaction_canceled_exception(),
                        "task {i}: unexpected error {e:?}"
                    );
                }
            }
        }
        let item = get(client, &table, &counter).await.unwrap();
        assert_eq!(
            (committed, num(&item, "closed_at")),
            (num(&item, "n"), num(&item, "n")),
            "round {round}: (committed counts, count at closing) must both equal n"
        );
    }
    drop_table(client, &table).await;
}

/// A PartiQL `UPDATE` whose new value is computed from the stored one
/// ( `list_append` ) is a read-modify-write as well.
pub async fn partiql_list_append_is_not_lost(client: &Client, load: Load) {
    let table = create_table(client, "cas-partiql").await;
    for round in 0..load.rounds {
        let pk = format!("l{round}");
        client
            .put_item()
            .table_name(&table)
            .item("pk", s(&pk))
            .item("entries", AttributeValue::L(vec![]))
            .send()
            .await
            .unwrap();

        race(load.tasks, |i| {
            let request = client.execute_statement().statement(format!(
                "UPDATE \"{table}\" SET entries = list_append(entries, [{i}]) WHERE pk = '{pk}'"
            ));
            async move { request.send().await.unwrap() }
        })
        .await;

        let item = get(client, &table, &pk).await.unwrap();
        let mut entries: Vec<i64> = item["entries"]
            .as_l()
            .unwrap()
            .iter()
            .map(|v| v.as_n().unwrap().parse().unwrap())
            .collect();
        entries.sort_unstable();
        assert_eq!(
            entries,
            (0..load.tasks as i64).collect::<Vec<_>>(),
            "round {round}: every appended element must be present once"
        );
    }
    drop_table(client, &table).await;
}
