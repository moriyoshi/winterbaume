//! Example: AWS CodeArtifact
//!
//! Demonstrates using aws-sdk-codeartifact with winterbaume.
//!
//! Run with:
//!   cargo run --example codeartifact --package winterbaume-examples

use aws_sdk_codeartifact::config::BehaviorVersion;
use winterbaume_codeartifact::CodeArtifactService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeArtifactService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codeartifact::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codeartifact::Client::new(&config);

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");
    println!("CodeArtifact domains: {:?}", resp.domains());
}
