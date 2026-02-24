//! Example: CloudHSM v2
//!
//! Demonstrates using aws-sdk-cloudhsmv2 with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudhsmv2 --package winterbaume-examples

use aws_sdk_cloudhsmv2::config::BehaviorVersion;
use winterbaume_cloudhsmv2::CloudHsmV2Service;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudHsmV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudhsmv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudhsmv2::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("Clusters: {}", resp.clusters().len());
}
