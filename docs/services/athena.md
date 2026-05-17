# winterbaume-athena

Athena service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Athena |
| AWS model | `athena` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 25/70 operations (35.7%) |
| stubs (routed, returns empty/default) | 0/70 operations (0.0%) |
| moto coverage | 27/70 operations (38.6%) |
| floci coverage | 0/70 operations (0.0%) |
| kumo coverage | 7/70 operations (10.0%) |
| Coverage report date | 2026-05-17 |

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
aws athena list-work-groups
```

## Example

```rust
use aws_sdk_athena::config::BehaviorVersion;
use winterbaume_athena::AthenaService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AthenaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_athena::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_athena::Client::new(&config);

    let resp = client
        .list_work_groups()
        .send()
        .await
        .expect("list_work_groups should succeed");
    println!("Work groups: {}", resp.work_groups().len());
}
```

## Implemented APIs (25)

- `CreateCapacityReservation`
- `CreateDataCatalog`
- `CreateNamedQuery`
- `CreatePreparedStatement`
- `CreateWorkGroup`
- `DeleteWorkGroup`
- `GetCapacityReservation`
- `GetDataCatalog`
- `GetNamedQuery`
- `GetPreparedStatement`
- `GetQueryExecution`
- `GetQueryResults`
- `GetQueryRuntimeStatistics`
- `GetWorkGroup`
- `ListCapacityReservations`
- `ListDataCatalogs`
- `ListNamedQueries`
- `ListQueryExecutions`
- `ListTagsForResource`
- `ListWorkGroups`
- `StartQueryExecution`
- `StopQueryExecution`
- `TagResource`
- `UntagResource`
- `UpdateCapacityReservation`

<details><summary>Not yet implemented APIs (45)</summary>

- `BatchGetNamedQuery`
- `BatchGetPreparedStatement`
- `BatchGetQueryExecution`
- `CancelCapacityReservation`
- `CreateNotebook`
- `CreatePresignedNotebookUrl`
- `DeleteCapacityReservation`
- `DeleteDataCatalog`
- `DeleteNamedQuery`
- `DeleteNotebook`
- `DeletePreparedStatement`
- `ExportNotebook`
- `GetCalculationExecution`
- `GetCalculationExecutionCode`
- `GetCalculationExecutionStatus`
- `GetCapacityAssignmentConfiguration`
- `GetDatabase` (implemented by moto)
- `GetNotebookMetadata`
- `GetResourceDashboard`
- `GetSession`
- `GetSessionEndpoint`
- `GetSessionStatus`
- `GetTableMetadata`
- `ImportNotebook`
- `ListApplicationDPUSizes`
- `ListCalculationExecutions`
- `ListDatabases` (implemented by moto)
- `ListEngineVersions`
- `ListExecutors`
- `ListNotebookMetadata`
- `ListNotebookSessions`
- `ListPreparedStatements`
- `ListSessions`
- `ListTableMetadata`
- `PutCapacityAssignmentConfiguration`
- `StartCalculationExecution`
- `StartSession`
- `StopCalculationExecution`
- `TerminateSession`
- `UpdateDataCatalog`
- `UpdateNamedQuery`
- `UpdateNotebook`
- `UpdateNotebookMetadata`
- `UpdatePreparedStatement`
- `UpdateWorkGroup`

</details>
