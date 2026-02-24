//! Example: Network Firewall
//!
//! Demonstrates using aws-sdk-networkfirewall with winterbaume.
//!
//! Run with:
//!   cargo run --example networkfirewall --package winterbaume-examples

use aws_sdk_networkfirewall::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_networkfirewall::NetworkFirewallService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(NetworkFirewallService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkfirewall::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_networkfirewall::Client::new(&config);

    let resp = client
        .list_firewalls()
        .send()
        .await
        .expect("list_firewalls should succeed");
    println!("Network firewalls: {}", resp.firewalls().len());
}
