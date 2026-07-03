# winterbaume-ssmquicksetup

AWS Systems Manager Quick Setup service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SSM Quick Setup |
| AWS model | `ssm-quicksetup` |
| Protocol | restJson1 |
| winterbaume coverage | 6/14 operations (42.9%) |
| stubs (routed, returns empty/default) | 0/14 operations (0.0%) |
| moto coverage | 0/14 operations (0.0%) |
| floci coverage | 0/14 operations (0.0%) |
| kumo coverage | 0/14 operations (0.0%) |
| fakecloud coverage | 0/14 operations (0.0%) |
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
aws ssmquicksetup help
```

## Example

```rust
use aws_sdk_ssmquicksetup::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ssmquicksetup::SsmQuickSetupService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SsmQuickSetupService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssmquicksetup::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ssmquicksetup::Client::new(&config);
    let resp = client
        .list_quick_setup_types()
        .send()
        .await
        .expect("list_quick_setup_types should succeed");
    for t in resp.quick_setup_type_list() {
        println!("Type: {:?} v{:?}", t.r#type(), t.latest_version());
    }
}
```

## Implemented APIs (6)

- `GetConfiguration`
- `GetServiceSettings`
- `ListConfigurations`
- `TagResource`
- `UntagResource`
- `UpdateServiceSettings`

<details><summary>Not yet implemented APIs (8)</summary>

- `CreateConfigurationManager`
- `DeleteConfigurationManager`
- `GetConfigurationManager`
- `ListConfigurationManagers`
- `ListQuickSetupTypes`
- `ListTagsForResource`
- `UpdateConfigurationDefinition`
- `UpdateConfigurationManager`

</details>
