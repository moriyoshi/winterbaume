//! Example: Bedrock Agent
//!
//! Demonstrates using aws-sdk-bedrockagent with winterbaume.
//!
//! Run with:
//!   cargo run --example bedrockagent --package winterbaume-examples

use aws_sdk_bedrockagent::config::BehaviorVersion;
use winterbaume_bedrockagent::BedrockAgentService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BedrockAgentService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bedrockagent::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bedrockagent::Client::new(&config);

    let resp = client
        .list_agents()
        .send()
        .await
        .expect("list_agents should succeed");
    println!("Agents: {}", resp.agent_summaries().len());
}
