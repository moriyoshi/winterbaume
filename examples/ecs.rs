//! Example: ECS
//!
//! Demonstrates using aws-sdk-ecs with winterbaume.
//!
//! Run with:
//!   cargo run --example ecs --package winterbaume-examples

use aws_sdk_ecs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ecs::EcsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EcsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ecs::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("ECS clusters: {}", resp.cluster_arns().len());
}
