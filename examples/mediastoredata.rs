//! Example: MediaStore Data
//!
//! Demonstrates using aws-sdk-mediastoredata with winterbaume.
//!
//! Run with:
//!   cargo run --example mediastoredata --package winterbaume-examples

use aws_sdk_mediastoredata::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediastoredata::MediaStoreDataService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaStoreDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediastoredata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediastoredata::Client::new(&config);

    // MediaStore Data requires a container endpoint URL.
    // This example demonstrates client setup for the MediaStoreData service.
    println!("MediaStoreData client ready. Use list_items() to list objects in a container.");
    let _client = client;
}
