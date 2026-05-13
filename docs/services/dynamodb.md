# winterbaume-dynamodb

DynamoDB service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | DynamoDB |
| AWS model | `dynamodb` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 57/57 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/57 operations (0.0%) |
| moto coverage | 39/57 operations (68.4%) |
| floci coverage | 0/57 operations (0.0%) |
| kumo coverage | 20/57 operations (35.1%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws dynamodb list-tables
```

## Example

```rust
use aws_sdk_dynamodb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dynamodb::DynamoDbService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DynamoDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dynamodb::Client::new(&config);

    use aws_sdk_dynamodb::types::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    };
    client
        .create_table()
        .table_name("example-table")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table should succeed");
    let resp = client
        .list_tables()
        .send()
        .await
        .expect("list_tables should succeed");
    println!("Tables: {:?}", resp.table_names());
}
```

## Implemented APIs (57)

- `BatchExecuteStatement`
- `BatchGetItem`
- `BatchWriteItem`
- `CreateBackup`
- `CreateGlobalTable`
- `CreateTable`
- `DeleteBackup`
- `DeleteItem`
- `DeleteResourcePolicy`
- `DeleteTable`
- `DescribeBackup`
- `DescribeContinuousBackups`
- `DescribeContributorInsights`
- `DescribeEndpoints`
- `DescribeExport`
- `DescribeGlobalTable`
- `DescribeGlobalTableSettings`
- `DescribeImport`
- `DescribeKinesisStreamingDestination`
- `DescribeLimits`
- `DescribeTable`
- `DescribeTableReplicaAutoScaling`
- `DescribeTimeToLive`
- `DisableKinesisStreamingDestination`
- `EnableKinesisStreamingDestination`
- `ExecuteStatement`
- `ExecuteTransaction`
- `ExportTableToPointInTime`
- `GetItem`
- `GetResourcePolicy`
- `ImportTable`
- `ListBackups`
- `ListContributorInsights`
- `ListExports`
- `ListGlobalTables`
- `ListImports`
- `ListTables`
- `ListTagsOfResource`
- `PutItem`
- `PutResourcePolicy`
- `Query`
- `RestoreTableFromBackup`
- `RestoreTableToPointInTime`
- `Scan`
- `TagResource`
- `TransactGetItems`
- `TransactWriteItems`
- `UntagResource`
- `UpdateContinuousBackups`
- `UpdateContributorInsights`
- `UpdateGlobalTable`
- `UpdateGlobalTableSettings`
- `UpdateItem`
- `UpdateKinesisStreamingDestination`
- `UpdateTable`
- `UpdateTableReplicaAutoScaling`
- `UpdateTimeToLive`
