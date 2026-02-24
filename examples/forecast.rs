//! Example: Forecast
//!
//! Demonstrates using aws-sdk-forecast with winterbaume.
//!
//! Run with:
//!   cargo run --example forecast --package winterbaume-examples

use aws_sdk_forecast::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_forecast::ForecastService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ForecastService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_forecast::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_forecast::Client::new(&config);

    let resp = client
        .list_datasets()
        .send()
        .await
        .expect("list_datasets should succeed");
    println!("Forecast datasets: {}", resp.datasets().len());
}
