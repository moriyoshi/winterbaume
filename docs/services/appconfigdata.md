# winterbaume-appconfigdata

AWS AppConfig Data runtime service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AppConfig Data |
| AWS model | `appconfigdata` |
| Protocol | restJson1 |
| winterbaume coverage | 2/2 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/2 operations (0.0%) |
| moto coverage | 0/2 operations (0.0%) |
| floci coverage | 0/2 operations (0.0%) |
| kumo coverage | 0/2 operations (0.0%) |
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
aws appconfigdata help
```

## Example

```rust
use aws_sdk_appconfigdata::config::BehaviorVersion;
use winterbaume_appconfigdata::AppConfigDataService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppConfigDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appconfigdata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appconfigdata::Client::new(&config);

    let session = client
        .start_configuration_session()
        .application_identifier("MyApp")
        .environment_identifier("Prod")
        .configuration_profile_identifier("MyProfile")
        .send()
        .await
        .expect("start_configuration_session should succeed");
    println!(
        "AppConfigData session token: {}",
        session.initial_configuration_token().unwrap_or("<none>")
    );
}
```

## Implemented APIs (2)

- `GetLatestConfiguration`
- `StartConfigurationSession`
