# winterbaume-sts

STS service mock implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | STS |
| AWS model | `sts` |
| Protocol | awsQuery |
| winterbaume coverage | 11/11 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/11 operations (0.0%) |
| moto coverage | 7/11 operations (63.6%) |
| floci coverage | 0/11 operations (0.0%) |
| kumo coverage | 6/11 operations (54.5%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws sts get-caller-identity
```

## Example

```rust
use aws_sdk_sts::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sts::StsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(StsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sts::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sts::Client::new(&config);

    let resp = client
        .get_caller_identity()
        .send()
        .await
        .expect("get_caller_identity should succeed");
    println!("Account: {}", resp.account().unwrap_or_default());
    println!("User ID: {}", resp.user_id().unwrap_or_default());
    println!("ARN: {}", resp.arn().unwrap_or_default());
}
```

## Implemented APIs (11)

- `AssumeRole`
- `AssumeRoleWithSAML`
- `AssumeRoleWithWebIdentity`
- `AssumeRoot`
- `DecodeAuthorizationMessage`
- `GetAccessKeyInfo`
- `GetCallerIdentity`
- `GetDelegatedAccessToken`
- `GetFederationToken`
- `GetSessionToken`
- `GetWebIdentityToken`
