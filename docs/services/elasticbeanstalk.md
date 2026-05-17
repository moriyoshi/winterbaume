# winterbaume-elasticbeanstalk

AWS Elastic Beanstalk service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Elastic Beanstalk |
| AWS model | `elastic-beanstalk` |
| Protocol | awsQuery |
| winterbaume coverage | 7/47 operations (14.9%) |
| stubs (routed, returns empty/default) | 0/47 operations (0.0%) |
| moto coverage | 0/47 operations (0.0%) |
| floci coverage | 0/47 operations (0.0%) |
| kumo coverage | 7/47 operations (14.9%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws elasticbeanstalk help
```

## Example

```rust
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
```

## Implemented APIs (7)

- `CreateApplication`
- `CreateEnvironment`
- `DeleteApplication`
- `DescribeEnvironments`
- `ListAvailableSolutionStacks`
- `ListTagsForResource`
- `UpdateTagsForResource`

<details><summary>Not yet implemented APIs (40)</summary>

- `AbortEnvironmentUpdate`
- `ApplyEnvironmentManagedAction`
- `AssociateEnvironmentOperationsRole`
- `CheckDNSAvailability`
- `ComposeEnvironments`
- `CreateApplicationVersion`
- `CreateConfigurationTemplate`
- `CreatePlatformVersion`
- `CreateStorageLocation`
- `DeleteApplicationVersion`
- `DeleteConfigurationTemplate`
- `DeleteEnvironmentConfiguration`
- `DeletePlatformVersion`
- `DescribeAccountAttributes`
- `DescribeApplicationVersions`
- `DescribeApplications` (implemented by kumo)
- `DescribeConfigurationOptions`
- `DescribeConfigurationSettings`
- `DescribeEnvironmentHealth`
- `DescribeEnvironmentManagedActionHistory`
- `DescribeEnvironmentManagedActions`
- `DescribeEnvironmentResources`
- `DescribeEvents`
- `DescribeInstancesHealth`
- `DescribePlatformVersion`
- `DisassociateEnvironmentOperationsRole`
- `ListPlatformBranches`
- `ListPlatformVersions`
- `RebuildEnvironment`
- `RequestEnvironmentInfo`
- `RestartAppServer`
- `RetrieveEnvironmentInfo`
- `SwapEnvironmentCNAMEs`
- `TerminateEnvironment` (implemented by kumo)
- `UpdateApplication` (implemented by kumo)
- `UpdateApplicationResourceLifecycle`
- `UpdateApplicationVersion`
- `UpdateConfigurationTemplate`
- `UpdateEnvironment`
- `ValidateConfigurationSettings`

</details>
