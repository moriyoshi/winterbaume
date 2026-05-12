# winterbaume-s3tables

S3 Tables service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | S3 Tables |
| AWS model | `s3tables` |
| Protocol | restJson1 |
| winterbaume coverage | 46/49 operations (93.9%) |
| stubs (routed, returns empty/default) | 3/49 operations (6.1%) |
| moto coverage | 14/49 operations (28.6%) |
| floci coverage | 0/49 operations (0.0%) |
| kumo coverage | 12/49 operations (24.5%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws s3tables list-table-buckets
```

## Example

```rust
use aws_sdk_s3tables::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3tables::S3TablesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3TablesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3tables::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3tables::Client::new(&config);

    // Create a table bucket
    let resp = client
        .create_table_bucket()
        .name("my-table-bucket")
        .send()
        .await
        .expect("create_table_bucket should succeed");

    let arn = resp.arn();
    println!("Created table bucket ARN: {}", arn);

    // Get the table bucket
    let get_resp = client
        .get_table_bucket()
        .table_bucket_arn(arn)
        .send()
        .await
        .expect("get_table_bucket should succeed");

    println!("Table bucket name: {}", get_resp.name());
    println!("Table bucket ARN: {}", get_resp.arn());

    // List table buckets
    let list_resp = client
        .list_table_buckets()
        .send()
        .await
        .expect("list_table_buckets should succeed");

    println!("Total table buckets: {}", list_resp.table_buckets().len());
}
```

## Implemented APIs (46)

- `CreateNamespace`
- `CreateTable`
- `CreateTableBucket`
- `DeleteNamespace`
- `DeleteTable`
- `DeleteTableBucket`
- `DeleteTableBucketEncryption`
- `DeleteTableBucketMetricsConfiguration`
- `DeleteTableBucketPolicy`
- `DeleteTableBucketReplication`
- `DeleteTablePolicy`
- `DeleteTableReplication`
- `GetNamespace`
- `GetTable`
- `GetTableBucket`
- `GetTableBucketEncryption`
- `GetTableBucketMaintenanceConfiguration`
- `GetTableBucketMetricsConfiguration`
- `GetTableBucketPolicy`
- `GetTableBucketReplication`
- `GetTableBucketStorageClass`
- `GetTableEncryption`
- `GetTableMaintenanceConfiguration`
- `GetTableMetadataLocation`
- `GetTablePolicy`
- `GetTableRecordExpirationConfiguration`
- `GetTableReplication`
- `GetTableStorageClass`
- `ListNamespaces`
- `ListTableBuckets`
- `ListTables`
- `ListTagsForResource`
- `PutTableBucketEncryption`
- `PutTableBucketMaintenanceConfiguration`
- `PutTableBucketMetricsConfiguration`
- `PutTableBucketPolicy`
- `PutTableBucketReplication`
- `PutTableBucketStorageClass`
- `PutTableMaintenanceConfiguration`
- `PutTablePolicy`
- `PutTableRecordExpirationConfiguration`
- `PutTableReplication`
- `RenameTable`
- `TagResource`
- `UntagResource`
- `UpdateTableMetadataLocation`

<details><summary>Stubbed APIs (3) &mdash; routed but return an empty/default response</summary>

- `GetTableMaintenanceJobStatus`
- `GetTableRecordExpirationJobStatus`
- `GetTableReplicationStatus`

</details>
