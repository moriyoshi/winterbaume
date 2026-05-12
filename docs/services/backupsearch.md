# winterbaume-backupsearch

AWS Backup Search service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Backup Search |
| AWS model | `backupsearch` |
| Protocol | restJson1 |
| winterbaume coverage | 9/12 operations (75.0%) |
| stubs (routed, returns empty/default) | 0/12 operations (0.0%) |
| moto coverage | 0/12 operations (0.0%) |
| floci coverage | 0/12 operations (0.0%) |
| kumo coverage | 0/12 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws backupsearch help
```

## Example

```rust
use aws_sdk_backupsearch::config::BehaviorVersion;
use winterbaume_backupsearch::BackupSearchService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BackupSearchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backupsearch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_backupsearch::Client::new(&config);

    let resp = client
        .list_search_jobs()
        .send()
        .await
        .expect("list_search_jobs should succeed");
    println!("Backup search jobs: {}", resp.search_jobs().len());
}
```

## Implemented APIs (9)

- `GetSearchJob`
- `ListSearchJobBackups`
- `ListSearchJobResults`
- `ListSearchJobs`
- `ListTagsForResource`
- `StartSearchJob`
- `StopSearchJob`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (3)</summary>

- `GetSearchResultExportJob`
- `ListSearchResultExportJobs`
- `StartSearchResultExportJob`

</details>
