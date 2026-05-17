# winterbaume-appintegrations

AWS AppIntegrations service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AppIntegrations |
| AWS model | `appintegrations` |
| Protocol | restJson1 |
| winterbaume coverage | 23/23 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/23 operations (0.0%) |
| moto coverage | 0/23 operations (0.0%) |
| floci coverage | 0/23 operations (0.0%) |
| kumo coverage | 0/23 operations (0.0%) |
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
aws appintegrations help
```

## Example

```rust
use aws_sdk_appintegrations::config::BehaviorVersion;
use winterbaume_appintegrations::AppIntegrationsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppIntegrationsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appintegrations::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appintegrations::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!(
        "AppIntegrations applications: {}",
        resp.applications().len()
    );
}
```

## Implemented APIs (23)

- `CreateApplication`
- `CreateDataIntegration`
- `CreateDataIntegrationAssociation`
- `CreateEventIntegration`
- `DeleteApplication`
- `DeleteDataIntegration`
- `DeleteEventIntegration`
- `GetApplication`
- `GetDataIntegration`
- `GetEventIntegration`
- `ListApplicationAssociations`
- `ListApplications`
- `ListDataIntegrationAssociations`
- `ListDataIntegrations`
- `ListEventIntegrationAssociations`
- `ListEventIntegrations`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateApplication`
- `UpdateDataIntegration`
- `UpdateDataIntegrationAssociation`
- `UpdateEventIntegration`
