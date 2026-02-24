//! Example: Service Catalog AppRegistry
//!
//! Demonstrates using aws-sdk-servicecatalogappregistry with winterbaume.
//!
//! Run with:
//!   cargo run --example servicecatalogappregistry --package winterbaume-examples

use aws_sdk_servicecatalogappregistry::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicecatalogappregistry::ServiceCatalogAppRegistryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ServiceCatalogAppRegistryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicecatalogappregistry::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_servicecatalogappregistry::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!("AppRegistry applications: {}", resp.applications().len());
}
