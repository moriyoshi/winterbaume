# winterbaume-cognitoidentity

Cognito Identity service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cognito Identity |
| AWS model | `cognito-identity` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 20/23 operations (87.0%) |
| stubs (routed, returns empty/default) | 3/23 operations (13.0%) |
| moto coverage | 10/23 operations (43.5%) |
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
aws cognito-identity list-identity-pools --max-results 10
```

## Example

```rust
use aws_sdk_cognitoidentity::config::BehaviorVersion;
use winterbaume_cognitoidentity::CognitoIdentityService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CognitoIdentityService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitoidentity::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cognitoidentity::Client::new(&config);

    let resp = client
        .list_identity_pools()
        .max_results(10)
        .send()
        .await
        .expect("list_identity_pools should succeed");
    println!("Identity pools: {}", resp.identity_pools().len());
}
```

## Implemented APIs (20)

- `CreateIdentityPool`
- `DeleteIdentities`
- `DeleteIdentityPool`
- `DescribeIdentity`
- `DescribeIdentityPool`
- `GetId`
- `GetIdentityPoolRoles`
- `GetPrincipalTagAttributeMap`
- `ListIdentities`
- `ListIdentityPools`
- `ListTagsForResource`
- `LookupDeveloperIdentity`
- `MergeDeveloperIdentities`
- `SetIdentityPoolRoles`
- `SetPrincipalTagAttributeMap`
- `TagResource`
- `UnlinkDeveloperIdentity`
- `UnlinkIdentity`
- `UntagResource`
- `UpdateIdentityPool`

<details><summary>Stubbed APIs (3) &mdash; routed but return an empty/default response</summary>

- `GetCredentialsForIdentity`
- `GetOpenIdToken`
- `GetOpenIdTokenForDeveloperIdentity`

</details>
