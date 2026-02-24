//! Example: ElastiCache
//!
//! Demonstrates using aws-sdk-elasticache with winterbaume.
//!
//! Run with:
//!   cargo run --example elasticache --package winterbaume-examples

use aws_sdk_elasticache::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_elasticache::ElastiCacheService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ElastiCacheService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticache::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_elasticache::Client::new(&config);

    client
        .create_cache_cluster()
        .cache_cluster_id("example-cluster")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(1)
        .send()
        .await
        .expect("create_cache_cluster should succeed");

    let resp = client
        .describe_cache_clusters()
        .send()
        .await
        .expect("describe_cache_clusters should succeed");
    println!("ElastiCache clusters: {}", resp.cache_clusters().len());
}
