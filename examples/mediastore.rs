//! Example: MediaStore
//!
//! Demonstrates using aws-sdk-mediastore with winterbaume.
//!
//! Run with:
//!   cargo run --example mediastore --package winterbaume-examples

use aws_sdk_mediastore::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediastore::MediaStoreService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaStoreService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediastore::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediastore::Client::new(&config);

    let resp = client
        .list_containers()
        .send()
        .await
        .expect("list_containers should succeed");
    println!("MediaStore containers: {}", resp.containers().len());
}
