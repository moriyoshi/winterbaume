//! Example: DataBrew
//!
//! Demonstrates using aws-sdk-databrew with winterbaume.
//!
//! Run with:
//!   cargo run --example databrew --package winterbaume-examples

use aws_sdk_databrew::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_databrew::DataBrewService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DataBrewService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_databrew::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_databrew::Client::new(&config);

    let resp = client
        .list_projects()
        .send()
        .await
        .expect("list_projects should succeed");
    println!("DataBrew projects: {}", resp.projects().len());
}
