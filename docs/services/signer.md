# winterbaume-signer

AWS Signer service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Signer |
| AWS model | `signer` |
| Protocol | restJson1 |
| winterbaume coverage | 19/19 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 7/19 operations (36.8%) |
| floci coverage | 0/19 operations (0.0%) |
| kumo coverage | 0/19 operations (0.0%) |
| Coverage report date | 2026-05-17 |

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
aws signer list-signing-jobs
```

## Example

```rust
use aws_sdk_signer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_signer::SignerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SignerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_signer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_signer::Client::new(&config);

    let resp = client
        .list_signing_profiles()
        .send()
        .await
        .expect("list_signing_profiles should succeed");
    println!("Signer signing profiles: {}", resp.profiles().len());
}
```

## Implemented APIs (19)

- `AddProfilePermission`
- `CancelSigningProfile`
- `DescribeSigningJob`
- `GetRevocationStatus`
- `GetSigningPlatform`
- `GetSigningProfile`
- `ListProfilePermissions`
- `ListSigningJobs`
- `ListSigningPlatforms`
- `ListSigningProfiles`
- `ListTagsForResource`
- `PutSigningProfile`
- `RemoveProfilePermission`
- `RevokeSignature`
- `RevokeSigningProfile`
- `SignPayload`
- `StartSigningJob`
- `TagResource`
- `UntagResource`
