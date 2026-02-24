//! Example: MSK (Kafka)
//!
//! Demonstrates using aws-sdk-kafka with winterbaume.
//!
//! Run with:
//!   cargo run --example kafka --package winterbaume-examples

use aws_sdk_kafka::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kafka::KafkaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(KafkaService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kafka::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kafka::Client::new(&config);

    let resp = client
        .list_clusters_v2()
        .send()
        .await
        .expect("list_clusters_v2 should succeed");
    println!("MSK clusters: {}", resp.cluster_info_list().len());
}
