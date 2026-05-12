# winterbaume-cognitosync

AWS Cognito Sync service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cognito Sync |
| AWS model | `cognito-sync` |
| Protocol | restJson1 |
| winterbaume coverage | 11/17 operations (64.7%) |
| stubs (routed, returns empty/default) | 0/17 operations (0.0%) |
| moto coverage | 0/17 operations (0.0%) |
| floci coverage | 0/17 operations (0.0%) |
| kumo coverage | 0/17 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws cognitosync help
```

## Example

```rust
use aws_sdk_cognitosync::config::BehaviorVersion;
use aws_sdk_cognitosync::types::{Operation as PatchOp, RecordPatch};
use winterbaume_cognitosync::CognitoSyncService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CognitoSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitosync::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cognitosync::Client::new(&config);
    client
        .update_records()
        .identity_pool_id("us-east-1:demo")
        .identity_id("us-east-1:demo-identity")
        .dataset_name("my-dataset")
        .sync_session_token("session-1")
        .record_patches(
            RecordPatch::builder()
                .op(PatchOp::Replace)
                .key("score")
                .value("42")
                .sync_count(0)
                .build()
                .expect("patch"),
        )
        .send()
        .await
        .expect("update_records");
    let resp = client
        .list_records()
        .identity_pool_id("us-east-1:demo")
        .identity_id("us-east-1:demo-identity")
        .dataset_name("my-dataset")
        .send()
        .await
        .expect("list_records");
    for r in resp.records() {
        println!("Record: {:?} = {:?}", r.key(), r.value());
    }
}
```

## Implemented APIs (11)

- `BulkPublish`
- `DeleteDataset`
- `DescribeDataset`
- `DescribeIdentityUsage`
- `GetBulkPublishDetails`
- `GetCognitoEvents`
- `ListDatasets`
- `ListRecords`
- `RegisterDevice`
- `SetCognitoEvents`
- `UpdateRecords`

<details><summary>Not yet implemented APIs (6)</summary>

- `DescribeIdentityPoolUsage`
- `GetIdentityPoolConfiguration`
- `ListIdentityPoolUsage`
- `SetIdentityPoolConfiguration`
- `SubscribeToDataset`
- `UnsubscribeFromDataset`

</details>
