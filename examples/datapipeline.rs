//! Example: Data Pipeline
//!
//! Demonstrates using aws-sdk-datapipeline with winterbaume.
//!
//! Run with:
//!   cargo run --example datapipeline --package winterbaume-examples

use aws_sdk_datapipeline::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_datapipeline::DataPipelineService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DataPipelineService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_datapipeline::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_datapipeline::Client::new(&config);

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");
    println!("Data pipelines: {}", resp.pipeline_id_list().len());
}
