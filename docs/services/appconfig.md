# winterbaume-appconfig

AppConfig service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AppConfig |
| AWS model | `appconfig` |
| Protocol | restJson1 |
| winterbaume coverage | 45/45 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/45 operations (0.0%) |
| moto coverage | 15/45 operations (33.3%) |
| floci coverage | 0/45 operations (0.0%) |
| kumo coverage | 0/45 operations (0.0%) |
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
aws appconfig list-applications
```

## Example

```rust
use aws_sdk_appconfig::config::BehaviorVersion;
use winterbaume_appconfig::AppConfigService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppConfigService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appconfig::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appconfig::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!("Applications: {}", resp.items().len());
}
```

## Implemented APIs (45)

- `CreateApplication`
- `CreateConfigurationProfile`
- `CreateDeploymentStrategy`
- `CreateEnvironment`
- `CreateExtension`
- `CreateExtensionAssociation`
- `CreateHostedConfigurationVersion`
- `DeleteApplication`
- `DeleteConfigurationProfile`
- `DeleteDeploymentStrategy`
- `DeleteEnvironment`
- `DeleteExtension`
- `DeleteExtensionAssociation`
- `DeleteHostedConfigurationVersion`
- `GetAccountSettings`
- `GetApplication`
- `GetConfiguration`
- `GetConfigurationProfile`
- `GetDeployment`
- `GetDeploymentStrategy`
- `GetEnvironment`
- `GetExtension`
- `GetExtensionAssociation`
- `GetHostedConfigurationVersion`
- `ListApplications`
- `ListConfigurationProfiles`
- `ListDeploymentStrategies`
- `ListDeployments`
- `ListEnvironments`
- `ListExtensionAssociations`
- `ListExtensions`
- `ListHostedConfigurationVersions`
- `ListTagsForResource`
- `StartDeployment`
- `StopDeployment`
- `TagResource`
- `UntagResource`
- `UpdateAccountSettings`
- `UpdateApplication`
- `UpdateConfigurationProfile`
- `UpdateDeploymentStrategy`
- `UpdateEnvironment`
- `UpdateExtension`
- `UpdateExtensionAssociation`
- `ValidateConfiguration`
