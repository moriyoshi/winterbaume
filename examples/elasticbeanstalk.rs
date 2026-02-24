//! Example: AWS Elastic Beanstalk
//!
//! Demonstrates using aws-sdk-elasticbeanstalk with winterbaume.
//!
//! Run with:
//!   cargo run --example elasticbeanstalk --package winterbaume-examples

use aws_sdk_elasticbeanstalk::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_elasticbeanstalk::ElasticBeanstalkService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ElasticBeanstalkService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticbeanstalk::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_elasticbeanstalk::Client::new(&config);

    // Create an application
    let app_resp = client
        .create_application()
        .application_name("my-app")
        .description("A sample Elastic Beanstalk application")
        .send()
        .await
        .expect("create_application should succeed");

    let app = app_resp.application().expect("should have application");
    println!("Created application: {:?}", app.application_name());

    // Create an environment
    let env_resp = client
        .create_environment()
        .application_name("my-app")
        .environment_name("my-env")
        .solution_stack_name("64bit Amazon Linux 2023 v4.3.0 running Python 3.11")
        .send()
        .await
        .expect("create_environment should succeed");

    println!("Created environment: {:?}", env_resp.environment_name());
    println!("Environment status: {:?}", env_resp.status());

    // List solution stacks
    let stacks = client
        .list_available_solution_stacks()
        .send()
        .await
        .expect("list_available_solution_stacks should succeed");
    println!(
        "Available solution stacks: {}",
        stacks.solution_stacks().len()
    );

    // Describe environments
    let envs = client
        .describe_environments()
        .application_name("my-app")
        .send()
        .await
        .expect("describe_environments should succeed");
    println!("Environments: {}", envs.environments().len());
}
