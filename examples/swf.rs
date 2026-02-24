//! Example: SWF
//!
//! Demonstrates using aws-sdk-swf with winterbaume.
//!
//! Run with:
//!   cargo run --example swf --package winterbaume-examples

use aws_sdk_swf::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_swf::SwfService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SwfService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_swf::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_swf::Client::new(&config);

    use aws_sdk_swf::types::RegistrationStatus;
    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .expect("list_domains should succeed");
    println!("SWF domains: {}", resp.domain_infos().len());
}
