//! Example: App Mesh
//!
//! Demonstrates using aws-sdk-appmesh with winterbaume.
//!
//! Run with:
//!   cargo run --example appmesh --package winterbaume-examples

use aws_sdk_appmesh::config::BehaviorVersion;
use winterbaume_appmesh::AppMeshService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppMeshService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appmesh::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appmesh::Client::new(&config);

    let resp = client
        .list_meshes()
        .send()
        .await
        .expect("list_meshes should succeed");
    println!("Meshes: {}", resp.meshes().len());
}
