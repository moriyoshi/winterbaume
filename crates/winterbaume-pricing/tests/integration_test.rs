use std::sync::{Arc, Mutex};

use aws_sdk_pricing::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_pricing::PricingService;

async fn make_client() -> aws_sdk_pricing::Client {
    let mock = MockAws::builder()
        .with_service(PricingService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pricing::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_pricing::Client::new(&config)
}

#[tokio::test]
async fn test_describe_services_catalogue() {
    let client = make_client().await;
    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services");
    assert!(resp.services().len() >= 5);
}

#[tokio::test]
async fn test_describe_services_filtered() {
    let client = make_client().await;
    let resp = client
        .describe_services()
        .service_code("AmazonEC2")
        .send()
        .await
        .expect("describe_services");
    assert_eq!(resp.services().len(), 1);
    assert_eq!(resp.services()[0].service_code(), "AmazonEC2");
}

#[tokio::test]
async fn test_get_attribute_values() {
    let client = make_client().await;
    let resp = client
        .get_attribute_values()
        .service_code("AmazonEC2")
        .attribute_name("instanceType")
        .send()
        .await
        .expect("get_attribute_values");
    assert!(resp.attribute_values().len() >= 3);
}

#[tokio::test]
async fn test_list_price_lists() {
    let client = make_client().await;
    let resp = client
        .list_price_lists()
        .service_code("AmazonEC2")
        .currency_code("USD")
        .effective_date(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .send()
        .await
        .expect("list_price_lists");
    assert_eq!(resp.price_lists().len(), 1);
    assert_eq!(resp.price_lists()[0].currency_code(), Some("USD"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = PricingService::new();
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

#[tokio::test]
async fn test_get_products() {
    let client = make_client().await;
    let resp = client
        .get_products()
        .service_code("AmazonEC2")
        .send()
        .await
        .expect("get_products");
    let items = resp.price_list();
    assert!(!items.is_empty(), "expected at least one product entry");
    // Each entry is a serialised JSON string per the AWS wire contract.
    let first: serde_json::Value =
        serde_json::from_str(items[0].as_str()).expect("product entry must be valid JSON");
    assert_eq!(
        first["product"]["attributes"]["servicecode"]
            .as_str()
            .unwrap_or(""),
        "AmazonEC2"
    );
}

#[tokio::test]
async fn test_get_price_list_file_url() {
    let client = make_client().await;
    // First obtain a PriceListArn via ListPriceLists.
    let list_resp = client
        .list_price_lists()
        .service_code("AmazonEC2")
        .currency_code("USD")
        .effective_date(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .send()
        .await
        .expect("list_price_lists");
    let arn = list_resp.price_lists()[0]
        .price_list_arn()
        .expect("price list ARN must be present");

    let resp = client
        .get_price_list_file_url()
        .price_list_arn(arn)
        .file_format("json")
        .send()
        .await
        .expect("get_price_list_file_url");
    let url = resp.url().expect("url must be present");
    assert!(
        url.starts_with("https://pricing."),
        "expected a pricing URL, got: {url}"
    );
}
