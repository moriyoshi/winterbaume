//! Example: Redshift Data
//!
//! Demonstrates using aws-sdk-redshiftdata with winterbaume.
//!
//! Run with:
//!   cargo run --example redshiftdata --package winterbaume-examples

use aws_sdk_redshiftdata::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_redshiftdata::RedshiftDataService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RedshiftDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_redshiftdata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_redshiftdata::Client::new(&config);

    let resp = client
        .list_databases()
        .workgroup_name("default")
        .database("dev")
        .send()
        .await
        .expect("list_databases should succeed");
    println!("Redshift databases: {}", resp.databases().len());
}
