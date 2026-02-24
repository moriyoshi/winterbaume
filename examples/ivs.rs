//! Example: IVS
//!
//! Demonstrates using aws-sdk-ivs with winterbaume.
//!
//! Run with:
//!   cargo run --example ivs --package winterbaume-examples

use aws_sdk_ivs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ivs::IvsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(IvsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ivs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ivs::Client::new(&config);

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    println!("IVS channels: {}", resp.channels().len());
}
