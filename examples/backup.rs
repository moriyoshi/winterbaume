//! Example: Backup
//!
//! Demonstrates using aws-sdk-backup with winterbaume.
//!
//! Run with:
//!   cargo run --example backup --package winterbaume-examples

use aws_sdk_backup::config::BehaviorVersion;
use winterbaume_backup::BackupService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BackupService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backup::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_backup::Client::new(&config);

    let resp = client
        .list_backup_vaults()
        .send()
        .await
        .expect("list_backup_vaults should succeed");
    println!("Backup vaults: {}", resp.backup_vault_list().len());
}
