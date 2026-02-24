//! Example: SSO Admin (IAM Identity Center)
//!
//! Demonstrates using aws-sdk-ssoadmin with winterbaume.
//!
//! Run with:
//!   cargo run --example ssoadmin --package winterbaume-examples

use aws_sdk_ssoadmin::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ssoadmin::SsoAdminService;

const INSTANCE_ARN: &str = "arn:aws:sso:::instance/ssoins-0123456789abcdef";

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SsoAdminService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssoadmin::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ssoadmin::Client::new(&config);

    let resp = client
        .list_instances()
        .send()
        .await
        .expect("list_instances should succeed");
    println!("SSO Admin instances: {:?}", resp.instances());

    // Create a permission set
    let ps_resp = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("ExamplePermissionSet")
        .description("Created by winterbaume example")
        .session_duration("PT4H")
        .send()
        .await
        .expect("create_permission_set should succeed");
    let ps = ps_resp.permission_set().expect("permission set present");
    println!("Created permission set: {:?}", ps.permission_set_arn());

    // List all permission sets
    let list_resp = client
        .list_permission_sets()
        .instance_arn(INSTANCE_ARN)
        .send()
        .await
        .expect("list_permission_sets should succeed");
    println!("All permission set ARNs: {:?}", list_resp.permission_sets());
}
