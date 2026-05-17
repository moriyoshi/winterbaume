# winterbaume-lakeformation

AWS Lake Formation service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Lake Formation |
| AWS model | `lakeformation` |
| Protocol | restJson1 |
| winterbaume coverage | 19/61 operations (31.1%) |
| stubs (routed, returns empty/default) | 1/61 operations (1.6%) |
| moto coverage | 20/61 operations (32.8%) |
| floci coverage | 0/61 operations (0.0%) |
| kumo coverage | 0/61 operations (0.0%) |
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
aws lakeformation list-resources
```

## Example

```rust
use aws_sdk_lakeformation::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_lakeformation::LakeFormationService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(LakeFormationService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lakeformation::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_lakeformation::Client::new(&config);

    let resp = client
        .list_resources()
        .send()
        .await
        .expect("list_resources should succeed");
    println!(
        "Lake Formation resources: {}",
        resp.resource_info_list().len()
    );
}
```

## Implemented APIs (19)

- `AddLFTagsToResource`
- `BatchGrantPermissions`
- `BatchRevokePermissions`
- `CreateLFTag`
- `DeleteLFTag`
- `DeregisterResource`
- `DescribeResource`
- `GetDataLakeSettings`
- `GetLFTag`
- `GetResourceLFTags`
- `GrantPermissions`
- `ListLFTags`
- `ListPermissions`
- `ListResources`
- `PutDataLakeSettings`
- `RegisterResource`
- `RemoveLFTagsFromResource`
- `RevokePermissions`
- `UpdateLFTag`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `ListDataCellsFilter`

</details>

<details><summary>Not yet implemented APIs (41)</summary>

- `AssumeDecoratedRoleWithSAML`
- `CancelTransaction`
- `CommitTransaction`
- `CreateDataCellsFilter`
- `CreateLFTagExpression`
- `CreateLakeFormationIdentityCenterConfiguration`
- `CreateLakeFormationOptIn`
- `DeleteDataCellsFilter`
- `DeleteLFTagExpression`
- `DeleteLakeFormationIdentityCenterConfiguration`
- `DeleteLakeFormationOptIn`
- `DeleteObjectsOnCancel`
- `DescribeLakeFormationIdentityCenterConfiguration`
- `DescribeTransaction`
- `ExtendTransaction`
- `GetDataCellsFilter`
- `GetDataLakePrincipal`
- `GetEffectivePermissionsForPath`
- `GetLFTagExpression`
- `GetQueryState`
- `GetQueryStatistics`
- `GetTableObjects`
- `GetTemporaryDataLocationCredentials`
- `GetTemporaryGluePartitionCredentials`
- `GetTemporaryGlueTableCredentials`
- `GetWorkUnitResults`
- `GetWorkUnits`
- `ListLFTagExpressions`
- `ListLakeFormationOptIns`
- `ListTableStorageOptimizers`
- `ListTransactions`
- `SearchDatabasesByLFTags`
- `SearchTablesByLFTags`
- `StartQueryPlanning`
- `StartTransaction`
- `UpdateDataCellsFilter`
- `UpdateLFTagExpression`
- `UpdateLakeFormationIdentityCenterConfiguration`
- `UpdateResource`
- `UpdateTableObjects`
- `UpdateTableStorageOptimizer`

</details>
