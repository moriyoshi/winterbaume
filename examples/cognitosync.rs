//! Example: Cognito Sync
//!
//! Demonstrates using aws-sdk-cognitosync with winterbaume.
//!
//! Run with:
//!   cargo run --example cognitosync --package winterbaume

use aws_sdk_cognitosync::config::BehaviorVersion;
use aws_sdk_cognitosync::types::{Operation as PatchOp, RecordPatch};
use winterbaume_cognitosync::CognitoSyncService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CognitoSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitosync::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cognitosync::Client::new(&config);
    client
        .update_records()
        .identity_pool_id("us-east-1:demo")
        .identity_id("us-east-1:demo-identity")
        .dataset_name("my-dataset")
        .sync_session_token("session-1")
        .record_patches(
            RecordPatch::builder()
                .op(PatchOp::Replace)
                .key("score")
                .value("42")
                .sync_count(0)
                .build()
                .expect("patch"),
        )
        .send()
        .await
        .expect("update_records");
    let resp = client
        .list_records()
        .identity_pool_id("us-east-1:demo")
        .identity_id("us-east-1:demo-identity")
        .dataset_name("my-dataset")
        .send()
        .await
        .expect("list_records");
    for r in resp.records() {
        println!("Record: {:?} = {:?}", r.key(), r.value());
    }
}
