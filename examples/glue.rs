//! Example: Glue
//!
//! Demonstrates using aws-sdk-glue with winterbaume.
//!
//! Run with:
//!   cargo run --example glue --package winterbaume-examples

use aws_sdk_glue::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_glue::GlueService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(GlueService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_glue::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_glue::Client::new(&config);

    let resp = client
        .get_databases()
        .send()
        .await
        .expect("get_databases should succeed");
    println!("Glue databases: {}", resp.database_list().len());
}
