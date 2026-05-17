# winterbaume-amplifybackend

AWS Amplify Backend service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Amplify Backend |
| AWS model | `amplifybackend` |
| Protocol | restJson1 |
| winterbaume coverage | 4/31 operations (12.9%) |
| stubs (routed, returns empty/default) | 0/31 operations (0.0%) |
| moto coverage | 0/31 operations (0.0%) |
| floci coverage | 0/31 operations (0.0%) |
| kumo coverage | 0/31 operations (0.0%) |
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
aws amplifybackend help
```

## Example

```rust
use aws_sdk_amplifybackend::config::BehaviorVersion;
use winterbaume_amplifybackend::AmplifyBackendService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AmplifyBackendService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplifybackend::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amplifybackend::Client::new(&config);

    let resp = client
        .create_backend()
        .app_id("d1example")
        .app_name("ExampleApp")
        .backend_environment_name("dev")
        .send()
        .await
        .expect("create_backend should succeed");
    println!(
        "AmplifyBackend created: app={:?} env={:?}",
        resp.app_id(),
        resp.backend_environment_name()
    );
}
```

## Implemented APIs (4)

- `CloneBackend`
- `CreateBackend`
- `DeleteBackend`
- `GetBackend`

<details><summary>Not yet implemented APIs (27)</summary>

- `CreateBackendAPI`
- `CreateBackendAuth`
- `CreateBackendConfig`
- `CreateBackendStorage`
- `CreateToken`
- `DeleteBackendAPI`
- `DeleteBackendAuth`
- `DeleteBackendStorage`
- `DeleteToken`
- `GenerateBackendAPIModels`
- `GetBackendAPI`
- `GetBackendAPIModels`
- `GetBackendAuth`
- `GetBackendJob`
- `GetBackendStorage`
- `GetToken`
- `ImportBackendAuth`
- `ImportBackendStorage`
- `ListBackendJobs`
- `ListS3Buckets`
- `RemoveAllBackends`
- `RemoveBackendConfig`
- `UpdateBackendAPI`
- `UpdateBackendAuth`
- `UpdateBackendConfig`
- `UpdateBackendJob`
- `UpdateBackendStorage`

</details>
