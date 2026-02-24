//! Example: WorkSpaces
//!
//! Demonstrates using aws-sdk-workspaces with winterbaume.
//!
//! Run with:
//!   cargo run --example workspaces --package winterbaume-examples

use aws_sdk_workspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_workspaces::WorkSpacesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(WorkSpacesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_workspaces::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_workspaces::Client::new(&config);

    let resp = client
        .describe_workspaces()
        .send()
        .await
        .expect("describe_workspaces should succeed");
    println!("WorkSpaces: {}", resp.workspaces().len());
}
