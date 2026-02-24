//! Example: EKS
//!
//! Demonstrates using aws-sdk-eks with winterbaume.
//!
//! Run with:
//!   cargo run --example eks --package winterbaume-examples

use aws_sdk_eks::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_eks::EksService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EksService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_eks::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_eks::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("EKS clusters: {}", resp.clusters().len());
}
