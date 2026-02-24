//! Example: CodeCommit
//!
//! Demonstrates using aws-sdk-codecommit with winterbaume.
//!
//! Run with:
//!   cargo run --example codecommit --package winterbaume-examples

use aws_sdk_codecommit::config::BehaviorVersion;
use winterbaume_codecommit::CodeCommitService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeCommitService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codecommit::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codecommit::Client::new(&config);

    let resp = client
        .list_repositories()
        .send()
        .await
        .expect("list_repositories should succeed");
    println!("Repositories: {}", resp.repositories().len());
}
