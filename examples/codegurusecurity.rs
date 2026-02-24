//! Example: CodeGuru Security
//!
//! Demonstrates using aws-sdk-codegurusecurity with winterbaume.
//!
//! Run with:
//!   cargo run --example codegurusecurity --package winterbaume

use aws_sdk_codegurusecurity::config::BehaviorVersion;
use aws_sdk_codegurusecurity::types::{AnalysisType, ResourceId, ScanType};
use winterbaume_codegurusecurity::CodeGuruSecurityService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeGuruSecurityService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codegurusecurity::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codegurusecurity::Client::new(&config);
    let resp = client
        .create_scan()
        .scan_name("demo-scan")
        .resource_id(ResourceId::CodeArtifactId("artifact-001".to_string()))
        .scan_type(ScanType::Standard)
        .analysis_type(AnalysisType::Security)
        .send()
        .await
        .expect("create_scan");
    if let Some(arn) = resp.scan_name_arn() {
        println!("Scan: {arn}");
    }
}
