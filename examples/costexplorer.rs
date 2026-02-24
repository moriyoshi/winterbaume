//! Example: Cost Explorer
//!
//! Demonstrates using aws-sdk-costexplorer with winterbaume.
//!
//! Run with:
//!   cargo run --example costexplorer --package winterbaume-examples

use aws_sdk_costexplorer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_costexplorer::CostExplorerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CostExplorerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costexplorer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_costexplorer::Client::new(&config);

    let resp = client
        .list_cost_category_definitions()
        .send()
        .await
        .expect("list_cost_category_definitions should succeed");
    println!("Cost categories: {}", resp.cost_category_references().len());
}
