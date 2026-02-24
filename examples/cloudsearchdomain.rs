//! Example: CloudSearch Domain
//!
//! Demonstrates using aws-sdk-cloudsearchdomain with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudsearchdomain --package winterbaume

use aws_sdk_cloudsearchdomain::config::BehaviorVersion;
use aws_sdk_cloudsearchdomain::primitives::ByteStream;
use winterbaume_cloudsearchdomain::CloudSearchDomainService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
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

    let client = aws_sdk_cloudsearchdomain::Client::new(&config);
    client
        .upload_documents()
        .content_type(aws_sdk_cloudsearchdomain::types::ContentType::ApplicationJson)
        .documents(ByteStream::from_static(
            br#"[{"type":"add","id":"1","fields":{"title":"Hello"}}]"#,
        ))
        .send()
        .await
        .expect("upload should succeed");
    let resp = client
        .search()
        .query("Hello")
        .send()
        .await
        .expect("search should succeed");
    println!("Hits: {}", resp.hits().map(|h| h.found()).unwrap_or(0));
}
