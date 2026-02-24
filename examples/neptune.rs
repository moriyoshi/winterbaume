//! Example: Neptune
//!
//! Demonstrates using aws-sdk-neptune with winterbaume.
//!
//! Run with:
//!   cargo run --example neptune --package winterbaume-examples

use aws_sdk_neptune::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_neptune::NeptuneService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(NeptuneService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_neptune::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_neptune::Client::new(&config);

    let resp = client
        .describe_db_clusters()
        .send()
        .await
        .expect("describe_db_clusters should succeed");
    println!("Neptune DB clusters: {:?}", resp.db_clusters());
}
