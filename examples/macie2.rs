//! Example: Macie2
//!
//! Demonstrates using aws-sdk-macie2 with winterbaume.
//!
//! Run with:
//!   cargo run --example macie2 --package winterbaume-examples

use aws_sdk_macie2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_macie2::Macie2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Macie2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_macie2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_macie2::Client::new(&config);

    let resp = client
        .list_findings()
        .send()
        .await
        .expect("list_findings should succeed");
    println!("Macie2 findings: {}", resp.finding_ids().len());
}
