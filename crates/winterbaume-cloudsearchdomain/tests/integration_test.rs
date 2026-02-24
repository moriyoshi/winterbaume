use std::sync::{Arc, Mutex};

use aws_sdk_cloudsearchdomain::config::BehaviorVersion;
use aws_sdk_cloudsearchdomain::primitives::ByteStream;
use winterbaume_cloudsearchdomain::CloudSearchDomainService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_cloudsearchdomain::Client {
    let mock = MockAws::builder()
        .with_service(CloudSearchDomainService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudsearchdomain::config::Region::new("us-east-1"))
        .endpoint_url("https://search-demo-abc.us-east-1.cloudsearch.amazonaws.com")
        .load()
        .await;
    aws_sdk_cloudsearchdomain::Client::new(&config)
}

const SDF_BATCH: &str = r#"[
    {"type":"add","id":"1","fields":{"title":"Hello world"}},
    {"type":"add","id":"2","fields":{"title":"Goodbye world"}}
]"#;

#[tokio::test]
async fn test_upload_then_search() {
    let client = make_client().await;
    let upload = client
        .upload_documents()
        .content_type(aws_sdk_cloudsearchdomain::types::ContentType::ApplicationJson)
        .documents(ByteStream::from_static(SDF_BATCH.as_bytes()))
        .send()
        .await
        .expect("upload");
    assert_eq!(upload.adds(), 2);
    assert_eq!(upload.deletes(), 0);
    assert_eq!(upload.status(), Some("success"));

    let search = client.search().query("hello").send().await.expect("search");
    let hits = search.hits().expect("hits");
    assert_eq!(hits.found(), 1);
    assert_eq!(hits.hit()[0].id(), Some("1"));
}

#[tokio::test]
async fn test_search_matchall() {
    let client = make_client().await;
    client
        .upload_documents()
        .content_type(aws_sdk_cloudsearchdomain::types::ContentType::ApplicationJson)
        .documents(ByteStream::from_static(SDF_BATCH.as_bytes()))
        .send()
        .await
        .expect("upload");
    let search = client
        .search()
        .query("matchall")
        .send()
        .await
        .expect("search");
    assert_eq!(search.hits().expect("hits").found(), 2);
}

#[tokio::test]
async fn test_suggest_prefix() {
    let client = make_client().await;
    client
        .upload_documents()
        .content_type(aws_sdk_cloudsearchdomain::types::ContentType::ApplicationJson)
        .documents(ByteStream::from_static(SDF_BATCH.as_bytes()))
        .send()
        .await
        .expect("upload");
    let resp = client
        .suggest()
        .query("Hello")
        .suggester("title-suggester")
        .send()
        .await
        .expect("suggest");
    let suggest = resp.suggest().expect("suggest");
    assert_eq!(suggest.found(), 1);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CloudSearchDomainService::new();
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
