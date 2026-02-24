//! Example: Private CA Connector for SCEP
//!
//! Demonstrates using aws-sdk-pcaconnectorscep with winterbaume.
//!
//! Run with:
//!   cargo run --example pcaconnectorscep --package winterbaume

use aws_sdk_pcaconnectorscep::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pcaconnectorscep::PcaConnectorScepService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PcaConnectorScepService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pcaconnectorscep::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pcaconnectorscep::Client::new(&config);
    let create = client
        .create_connector()
        .certificate_authority_arn("arn:aws:acm-pca:us-east-1:123:certificate-authority/demo")
        .send()
        .await
        .expect("create_connector should succeed");
    println!("Created connector: {:?}", create.connector_arn());
}
