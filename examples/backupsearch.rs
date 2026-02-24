//! Example: Backup Search
//!
//! Demonstrates using aws-sdk-backupsearch with winterbaume.
//!
//! Run with:
//!   cargo run --example backupsearch --package winterbaume

use aws_sdk_backupsearch::config::BehaviorVersion;
use winterbaume_backupsearch::BackupSearchService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BackupSearchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backupsearch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_backupsearch::Client::new(&config);

    let resp = client
        .list_search_jobs()
        .send()
        .await
        .expect("list_search_jobs should succeed");
    println!("Backup search jobs: {}", resp.search_jobs().len());
}
