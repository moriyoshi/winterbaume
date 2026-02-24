//! Example: WAFv2
//!
//! Demonstrates using aws-sdk-wafv2 with winterbaume.
//!
//! Run with:
//!   cargo run --example wafv2 --package winterbaume-examples

use aws_sdk_wafv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_wafv2::WafV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(WafV2Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_wafv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_wafv2::Client::new(&config);

    use aws_sdk_wafv2::types::Scope;
    let resp = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list_web_acls should succeed");
    println!("WAFv2 web ACLs: {}", resp.web_acls().len());
}
