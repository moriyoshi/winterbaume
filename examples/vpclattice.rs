//! Example: VPC Lattice
//!
//! Demonstrates using aws-sdk-vpclattice with winterbaume.
//!
//! Run with:
//!   cargo run --example vpclattice --package winterbaume-examples

use aws_sdk_vpclattice::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_vpclattice::VpcLatticeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(VpcLatticeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_vpclattice::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_vpclattice::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("VPC Lattice services: {}", resp.items().len());
}
