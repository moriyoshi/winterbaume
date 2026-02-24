//! Example: Redshift
//!
//! Demonstrates using aws-sdk-redshift with winterbaume.
//!
//! Run with:
//!   cargo run --example redshift --package winterbaume-examples

use aws_sdk_redshift::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_redshift::RedshiftService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RedshiftService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_redshift::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_redshift::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("Redshift clusters: {:?}", resp.clusters());
}
