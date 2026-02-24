//! Example: SimSpace Weaver
//!
//! Demonstrates using aws-sdk-simspaceweaver with winterbaume.
//!
//! Run with:
//!   cargo run --example simspaceweaver --package winterbaume

use aws_sdk_simspaceweaver::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_simspaceweaver::SimSpaceWeaverService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SimSpaceWeaverService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simspaceweaver::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_simspaceweaver::Client::new(&config);
    client
        .start_simulation()
        .name("demo")
        .role_arn("arn:aws:iam::123:role/SimRole")
        .send()
        .await
        .expect("start_simulation should succeed");
    let resp = client
        .list_simulations()
        .send()
        .await
        .expect("list_simulations should succeed");
    for s in resp.simulations() {
        println!("Simulation: {:?}", s.name());
    }
}
