# winterbaume-secretsmanager

Secrets Manager service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Secrets Manager |
| AWS model | `secrets-manager` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 22/23 operations (95.7%) |
| stubs (routed, returns empty/default) | 1/23 operations (4.3%) |
| moto coverage | 21/23 operations (91.3%) |
| floci coverage | 0/23 operations (0.0%) |
| kumo coverage | 11/23 operations (47.8%) |
| fakecloud coverage | 23/23 operations (100.0%) |
| Coverage report date | 2026-07-03 |

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
aws secretsmanager list-secrets
```

## Example

```rust
use aws_sdk_secretsmanager::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_secretsmanager::SecretsManagerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SecretsManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_secretsmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_secretsmanager::Client::new(&config);

    let resp = client
        .list_secrets()
        .send()
        .await
        .expect("list_secrets should succeed");
    println!("Secrets Manager secrets: {}", resp.secret_list().len());
}
```

## Implemented APIs (22)

- `BatchGetSecretValue`
- `CancelRotateSecret`
- `CreateSecret`
- `DeleteResourcePolicy`
- `DeleteSecret`
- `DescribeSecret`
- `GetRandomPassword`
- `GetResourcePolicy`
- `GetSecretValue`
- `ListSecretVersionIds`
- `ListSecrets`
- `PutResourcePolicy`
- `PutSecretValue`
- `RemoveRegionsFromReplication`
- `ReplicateSecretToRegions`
- `RestoreSecret`
- `RotateSecret`
- `StopReplicationToReplica`
- `TagResource`
- `UntagResource`
- `UpdateSecret`
- `UpdateSecretVersionStage`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `ValidateResourcePolicy`

</details>
