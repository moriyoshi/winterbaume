use std::sync::{Arc, Mutex};

use aws_sdk_cloudfrontkeyvaluestore::config::BehaviorVersion;
use winterbaume_cloudfrontkeyvaluestore::CloudFrontKvsService;
use winterbaume_core::{MockAws, StatefulService};

const KVS_ARN: &str = "arn:aws:cloudfront::123456789012:key-value-store/abc";

async fn make_client() -> aws_sdk_cloudfrontkeyvaluestore::Client {
    let mock = MockAws::builder()
        .with_service(CloudFrontKvsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudfrontkeyvaluestore::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_cloudfrontkeyvaluestore::Client::new(&config)
}

#[tokio::test]
async fn test_describe_lazy_seed() {
    let client = make_client().await;
    let resp = client
        .describe_key_value_store()
        .kvs_arn(KVS_ARN)
        .send()
        .await
        .expect("describe");
    assert_eq!(resp.kvs_arn(), KVS_ARN);
    assert_eq!(resp.item_count(), 0);
    assert!(resp.e_tag().starts_with("etag-"));
}

#[tokio::test]
async fn test_put_get_delete_key() {
    let client = make_client().await;
    let initial = client
        .describe_key_value_store()
        .kvs_arn(KVS_ARN)
        .send()
        .await
        .expect("describe");
    let etag = initial.e_tag().to_string();

    let put = client
        .put_key()
        .kvs_arn(KVS_ARN)
        .key("hello")
        .value("world")
        .if_match(&etag)
        .send()
        .await
        .expect("put");
    let new_etag = put.e_tag().to_string();

    let got = client
        .get_key()
        .kvs_arn(KVS_ARN)
        .key("hello")
        .send()
        .await
        .expect("get");
    assert_eq!(got.value(), "world");

    client
        .delete_key()
        .kvs_arn(KVS_ARN)
        .key("hello")
        .if_match(&new_etag)
        .send()
        .await
        .expect("delete");

    let err = client
        .get_key()
        .kvs_arn(KVS_ARN)
        .key("hello")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_put_key_etag_mismatch_returns_conflict() {
    // Strengthens the original loose `format!("{err:?}").contains("...")` check
    // into a typed-variant assertion -- see the auto-memory entry
    // `error-tests-must-assert-typed-variant` and TODO
    // `error-tests-typed-variant-assertion-sweep` for the rationale: a fuzzy
    // string check would also accept the SDK's `Unhandled` fallback.
    let client = make_client().await;
    let _ = client
        .describe_key_value_store()
        .kvs_arn(KVS_ARN)
        .send()
        .await
        .expect("describe");
    let err = client
        .put_key()
        .kvs_arn(KVS_ARN)
        .key("k")
        .value("v")
        .if_match("etag-bogus")
        .send()
        .await
        .expect_err("PutKey with stale If-Match must fail");
    assert!(matches!(
        err.as_service_error(),
        Some(
            aws_sdk_cloudfrontkeyvaluestore::operation::put_key::PutKeyError::ConflictException(_)
        )
    ));
    if let aws_sdk_cloudfrontkeyvaluestore::error::SdkError::ServiceError(svc) = &err {
        assert_eq!(svc.raw().status().as_u16(), 409);
    }
}

#[tokio::test]
async fn test_delete_key_etag_mismatch_returns_conflict() {
    let client = make_client().await;
    // Seed the store and a key first.
    let initial = client
        .describe_key_value_store()
        .kvs_arn(KVS_ARN)
        .send()
        .await
        .expect("describe");
    let etag = initial.e_tag().to_string();
    let put = client
        .put_key()
        .kvs_arn(KVS_ARN)
        .key("doomed")
        .value("v")
        .if_match(&etag)
        .send()
        .await
        .expect("put");
    let _current_etag = put.e_tag().to_string();

    let err = client
        .delete_key()
        .kvs_arn(KVS_ARN)
        .key("doomed")
        .if_match("etag-bogus")
        .send()
        .await
        .expect_err("DeleteKey with stale If-Match must fail");
    assert!(matches!(
        err.as_service_error(),
        Some(aws_sdk_cloudfrontkeyvaluestore::operation::delete_key::DeleteKeyError::ConflictException(_))
    ));

    // Key must survive a rejected conditional delete.
    let value = client
        .get_key()
        .kvs_arn(KVS_ARN)
        .key("doomed")
        .send()
        .await
        .expect("key must still exist");
    assert_eq!(value.value(), "v");
}

#[tokio::test]
async fn test_update_keys_etag_mismatch_returns_conflict() {
    let client = make_client().await;
    let _ = client
        .describe_key_value_store()
        .kvs_arn(KVS_ARN)
        .send()
        .await
        .expect("describe");

    let err = client
        .update_keys()
        .kvs_arn(KVS_ARN)
        .if_match("etag-bogus")
        .puts(
            aws_sdk_cloudfrontkeyvaluestore::types::PutKeyRequestListItem::builder()
                .key("x")
                .value("y")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("UpdateKeys with stale If-Match must fail");
    assert!(matches!(
        err.as_service_error(),
        Some(aws_sdk_cloudfrontkeyvaluestore::operation::update_keys::UpdateKeysError::ConflictException(_))
    ));
}

#[tokio::test]
async fn test_update_keys_batch() {
    let client = make_client().await;
    let initial = client
        .describe_key_value_store()
        .kvs_arn(KVS_ARN)
        .send()
        .await
        .expect("describe");
    let etag = initial.e_tag().to_string();

    let resp = client
        .update_keys()
        .kvs_arn(KVS_ARN)
        .if_match(&etag)
        .puts(
            aws_sdk_cloudfrontkeyvaluestore::types::PutKeyRequestListItem::builder()
                .key("alpha")
                .value("a")
                .build()
                .unwrap(),
        )
        .puts(
            aws_sdk_cloudfrontkeyvaluestore::types::PutKeyRequestListItem::builder()
                .key("beta")
                .value("b")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update");
    assert_eq!(resp.item_count(), 2);

    let list = client
        .list_keys()
        .kvs_arn(KVS_ARN)
        .send()
        .await
        .expect("list");
    assert_eq!(list.items().len(), 2);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CloudFrontKvsService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}
