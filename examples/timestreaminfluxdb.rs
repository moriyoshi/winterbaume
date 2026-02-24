//! Example: Timestream InfluxDB
//!
//! Demonstrates using aws-sdk-timestreaminfluxdb with winterbaume.
//!
//! Run with:
//!   cargo run --example timestreaminfluxdb --package winterbaume-examples

use aws_sdk_timestreaminfluxdb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreaminfluxdb::TimestreamInfluxDbService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TimestreamInfluxDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreaminfluxdb::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_timestreaminfluxdb::Client::new(&config);

    let resp = client
        .list_db_instances()
        .send()
        .await
        .expect("list_db_instances should succeed");
    println!("Timestream InfluxDB instances: {}", resp.items().len());
}
