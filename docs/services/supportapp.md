# winterbaume-supportapp

AWS Support App service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Support App |
| AWS model | `support-app` |
| Protocol | restJson1 |
| winterbaume coverage | 3/10 operations (30.0%) |
| stubs (routed, returns empty/default) | 0/10 operations (0.0%) |
| moto coverage | 0/10 operations (0.0%) |
| floci coverage | 0/10 operations (0.0%) |
| kumo coverage | 0/10 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws supportapp help
```

## Example

```rust
use aws_sdk_supportapp::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_supportapp::SupportAppService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SupportAppService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_supportapp::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_supportapp::Client::new(&config);
    client
        .put_account_alias()
        .account_alias("demo-account")
        .send()
        .await
        .expect("put_account_alias should succeed");
    let resp = client
        .get_account_alias()
        .send()
        .await
        .expect("get_account_alias should succeed");
    println!("Account alias: {:?}", resp.account_alias());
}
```

## Implemented APIs (3)

- `DeleteAccountAlias`
- `GetAccountAlias`
- `PutAccountAlias`

<details><summary>Not yet implemented APIs (7)</summary>

- `CreateSlackChannelConfiguration`
- `DeleteSlackChannelConfiguration`
- `DeleteSlackWorkspaceConfiguration`
- `ListSlackChannelConfigurations`
- `ListSlackWorkspaceConfigurations`
- `RegisterSlackWorkspaceForOrganization`
- `UpdateSlackChannelConfiguration`

</details>
