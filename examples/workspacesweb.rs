//! Example: WorkSpaces Web
//!
//! Demonstrates using aws-sdk-workspacesweb with winterbaume.
//!
//! Run with:
//!   cargo run --example workspacesweb --package winterbaume-examples

use aws_sdk_workspacesweb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_workspacesweb::WorkspacesWebService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(WorkspacesWebService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_workspacesweb::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_workspacesweb::Client::new(&config);

    let resp = client
        .list_portals()
        .send()
        .await
        .expect("list_portals should succeed");
    println!("WorkSpaces Web portals: {}", resp.portals().len());
}
