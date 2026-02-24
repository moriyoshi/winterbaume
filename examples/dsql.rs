//! Example: Aurora DSQL
//!
//! Demonstrates using aws-sdk-dsql with winterbaume.
//!
//! Run with:
//!   cargo run --example dsql --package winterbaume-examples

use aws_sdk_dsql::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dsql::DsqlService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(DsqlService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dsql::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dsql::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("DSQL clusters: {}", resp.clusters().len());
}
