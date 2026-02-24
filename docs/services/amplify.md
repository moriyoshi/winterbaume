# winterbaume-amplify

AWS Amplify service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Amplify |
| AWS model | `amplify` |
| Protocol | restJson1 |
| winterbaume coverage | 23/37 operations (62.2%) |
| stubs (routed, returns empty/default) | 0/37 operations (0.0%) |
| moto coverage | 0/37 operations (0.0%) |
| floci coverage | 0/37 operations (0.0%) |
| kumo coverage | 9/37 operations (24.3%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws amplify help
```

## Example

```rust
use aws_sdk_amplify::config::BehaviorVersion;
use winterbaume_amplify::AmplifyService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AmplifyService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplify::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amplify::Client::new(&config);

    let resp = client
        .list_apps()
        .send()
        .await
        .expect("list_apps should succeed");
    println!("Amplify apps: {}", resp.apps().len());
}
```

## Implemented APIs (23)

- `CreateApp`
- `CreateBranch`
- `CreateDomainAssociation`
- `DeleteApp`
- `DeleteBranch`
- `DeleteDomainAssociation`
- `DeleteJob`
- `GetApp`
- `GetBranch`
- `GetDomainAssociation`
- `GetJob`
- `ListApps`
- `ListBranches`
- `ListDomainAssociations`
- `ListJobs`
- `ListTagsForResource`
- `StartJob`
- `StopJob`
- `TagResource`
- `UntagResource`
- `UpdateApp`
- `UpdateBranch`
- `UpdateDomainAssociation`

<details><summary>Not yet implemented APIs (14)</summary>

- `CreateBackendEnvironment`
- `CreateDeployment`
- `CreateWebhook`
- `DeleteBackendEnvironment`
- `DeleteWebhook`
- `GenerateAccessLogs`
- `GetArtifactUrl`
- `GetBackendEnvironment`
- `GetWebhook`
- `ListArtifacts`
- `ListBackendEnvironments`
- `ListWebhooks`
- `StartDeployment`
- `UpdateWebhook`

</details>
