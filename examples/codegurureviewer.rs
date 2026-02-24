//! Example: CodeGuru Reviewer
//!
//! Demonstrates using aws-sdk-codegurureviewer with winterbaume.
//!
//! Run with:
//!   cargo run --example codegurureviewer --package winterbaume

use aws_sdk_codegurureviewer::config::BehaviorVersion;
use aws_sdk_codegurureviewer::types::{CodeCommitRepository, Repository};
use winterbaume_codegurureviewer::CodeGuruReviewerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeGuruReviewerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codegurureviewer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codegurureviewer::Client::new(&config);
    let repo = Repository::builder()
        .code_commit(
            CodeCommitRepository::builder()
                .name("demo-repo")
                .build()
                .expect("cc"),
        )
        .build();
    let resp = client
        .associate_repository()
        .repository(repo)
        .send()
        .await
        .expect("associate");
    if let Some(arn) = resp
        .repository_association()
        .and_then(|a| a.association_arn())
    {
        println!("Association: {arn}");
    }
}
