//! Example: Database Migration Service (DMS)
//!
//! Demonstrates using winterbaume-dms directly.
//!
//! This example calls the service handler directly via MockRequest. To use
//! the AWS SDK Rust client, depend on `aws-sdk-databasemigration` ( the
//! published crate name omits the trailing `service` ).
//!
//! Run with:
//!   cargo run --example dms --package winterbaume-examples

use bytes::Bytes;
use serde_json::{Value, json};
use winterbaume_core::{MockRequest, MockService};
use winterbaume_databasemigration::DatabaseMigrationService;

fn dms_request(action: &str, body: Value) -> MockRequest {
    let mut headers = http::HeaderMap::new();
    headers.insert(
        "x-amz-target",
        format!("AmazonDMSv20160101.{action}").parse().unwrap(),
    );
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-amz-json-1.1".parse().unwrap(),
    );
    MockRequest {
        method: "POST".to_string(),
        uri: "https://dms.us-east-1.amazonaws.com".to_string(),
        headers,
        body: Bytes::from(body.to_string()),
    }
}

#[tokio::main]
async fn main() {
    let service = DatabaseMigrationService::new();

    // Create a replication instance
    let resp = service
        .handle(dms_request(
            "CreateReplicationInstance",
            json!({
                "ReplicationInstanceIdentifier": "example-instance",
                "ReplicationInstanceClass": "dms.t3.medium",
                "AllocatedStorage": 50
            }),
        ))
        .await;
    let body: Value = serde_json::from_slice(&resp.body).unwrap();
    println!("Created replication instance:");
    println!(
        "  ARN: {}",
        body["ReplicationInstance"]["ReplicationInstanceArn"]
            .as_str()
            .unwrap_or("N/A")
    );
    println!(
        "  Status: {}",
        body["ReplicationInstance"]["ReplicationInstanceStatus"]
            .as_str()
            .unwrap_or("N/A")
    );

    // Describe replication instances
    let resp = service
        .handle(dms_request("DescribeReplicationInstances", json!({})))
        .await;
    let body: Value = serde_json::from_slice(&resp.body).unwrap();
    let instances = body["ReplicationInstances"].as_array().unwrap();
    println!(
        "\nDescribeReplicationInstances: {} instance(s)",
        instances.len()
    );
    for inst in instances {
        println!(
            "  - {} ({})",
            inst["ReplicationInstanceIdentifier"].as_str().unwrap_or(""),
            inst["ReplicationInstanceClass"].as_str().unwrap_or("")
        );
    }
}
