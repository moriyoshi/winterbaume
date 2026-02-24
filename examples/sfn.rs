//! Example: Step Functions
//!
//! Demonstrates using aws-sdk-sfn with winterbaume.
//!
//! Run with:
//!   cargo run --example sfn --package winterbaume-examples

use aws_sdk_sfn::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sfn::SfnService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SfnService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sfn::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sfn::Client::new(&config);

    let resp = client
        .list_state_machines()
        .send()
        .await
        .expect("list_state_machines should succeed");
    println!(
        "Step Functions state machines: {}",
        resp.state_machines().len()
    );
}
