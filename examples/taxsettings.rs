//! Example: Tax Settings
//!
//! Demonstrates using aws-sdk-taxsettings with winterbaume.
//!
//! Run with:
//!   cargo run --example taxsettings --package winterbaume

use aws_sdk_taxsettings::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_taxsettings::TaxSettingsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TaxSettingsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_taxsettings::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_taxsettings::Client::new(&config);
    let resp = client
        .get_tax_inheritance()
        .send()
        .await
        .expect("get_tax_inheritance should succeed");
    println!("Heritage status: {:?}", resp.heritage_status());
}
