//! Example: Comprehend
//!
//! Demonstrates using aws-sdk-comprehend with winterbaume.
//!
//! Run with:
//!   cargo run --example comprehend --package winterbaume-examples

use aws_sdk_comprehend::config::BehaviorVersion;
use winterbaume_comprehend::ComprehendService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ComprehendService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_comprehend::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_comprehend::Client::new(&config);

    let resp = client
        .list_document_classifiers()
        .send()
        .await
        .expect("list_document_classifiers should succeed");
    println!(
        "Document classifiers: {}",
        resp.document_classifier_properties_list().len()
    );
}
