# winterbaume-amplifyuibuilder

AWS Amplify UI Builder service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Amplify UI Builder |
| AWS model | `amplifyuibuilder` |
| Protocol | restJson1 |
| winterbaume coverage | 28/28 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/28 operations (0.0%) |
| moto coverage | 0/28 operations (0.0%) |
| floci coverage | 0/28 operations (0.0%) |
| kumo coverage | 0/28 operations (0.0%) |
| fakecloud coverage | 0/28 operations (0.0%) |
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
aws amplifyuibuilder help
```

## Example

```rust
use aws_sdk_amplifyuibuilder::config::BehaviorVersion;
use winterbaume_amplifyuibuilder::AmplifyUiBuilderService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AmplifyUiBuilderService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplifyuibuilder::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amplifyuibuilder::Client::new(&config);

    let resp = client
        .list_components()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("list_components should succeed");
    println!("Amplify UI Builder components: {}", resp.entities().len());
}
```

## Implemented APIs (28)

- `CreateComponent`
- `CreateForm`
- `CreateTheme`
- `DeleteComponent`
- `DeleteForm`
- `DeleteTheme`
- `ExchangeCodeForToken`
- `ExportComponents`
- `ExportForms`
- `ExportThemes`
- `GetCodegenJob`
- `GetComponent`
- `GetForm`
- `GetMetadata`
- `GetTheme`
- `ListCodegenJobs`
- `ListComponents`
- `ListForms`
- `ListTagsForResource`
- `ListThemes`
- `PutMetadataFlag`
- `RefreshToken`
- `StartCodegenJob`
- `TagResource`
- `UntagResource`
- `UpdateComponent`
- `UpdateForm`
- `UpdateTheme`
