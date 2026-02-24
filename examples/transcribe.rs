//! Example: Transcribe
//!
//! Demonstrates using aws-sdk-transcribe with winterbaume.
//!
//! Run with:
//!   cargo run --example transcribe --package winterbaume-examples

use aws_sdk_transcribe::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_transcribe::TranscribeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TranscribeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_transcribe::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_transcribe::Client::new(&config);

    let resp = client
        .list_transcription_jobs()
        .send()
        .await
        .expect("list_transcription_jobs should succeed");
    println!(
        "Transcription jobs: {}",
        resp.transcription_job_summaries().len()
    );
}
