//! Example: Fault Injection Simulator (FIS)
//!
//! Demonstrates using aws-sdk-fis with winterbaume.
//!
//! Run with:
//!   cargo run --example fis --package winterbaume-examples

use aws_sdk_fis::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_fis::FisService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(FisService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_fis::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_fis::Client::new(&config);

    let resp = client
        .list_experiment_templates()
        .send()
        .await
        .expect("list_experiment_templates should succeed");
    println!(
        "FIS experiment templates: {:?}",
        resp.experiment_templates()
    );
}
