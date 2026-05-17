# winterbaume-transfer

AWS Transfer Family service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Transfer |
| AWS model | `transfer` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 44/71 operations (62.0%) |
| stubs (routed, returns empty/default) | 0/71 operations (0.0%) |
| moto coverage | 14/71 operations (19.7%) |
| floci coverage | 0/71 operations (0.0%) |
| kumo coverage | 0/71 operations (0.0%) |
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
aws transfer list-servers
```

## Example

```rust
use aws_sdk_transfer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_transfer::TransferService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TransferService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_transfer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_transfer::Client::new(&config);

    let resp = client
        .list_servers()
        .send()
        .await
        .expect("list_servers should succeed");
    println!("Transfer servers: {}", resp.servers().len());
}
```

## Implemented APIs (44)

- `CreateAgreement`
- `CreateConnector`
- `CreateProfile`
- `CreateServer`
- `CreateUser`
- `CreateWebApp`
- `CreateWorkflow`
- `DeleteAgreement`
- `DeleteCertificate`
- `DeleteConnector`
- `DeleteProfile`
- `DeleteServer`
- `DeleteSshPublicKey`
- `DeleteUser`
- `DeleteWebApp`
- `DeleteWebAppCustomization`
- `DeleteWorkflow`
- `DescribeAgreement`
- `DescribeCertificate`
- `DescribeConnector`
- `DescribeProfile`
- `DescribeServer`
- `DescribeUser`
- `DescribeWebApp`
- `DescribeWebAppCustomization`
- `DescribeWorkflow`
- `ImportCertificate`
- `ImportSshPublicKey`
- `ListAgreements`
- `ListCertificates`
- `ListConnectors`
- `ListProfiles`
- `ListServers`
- `ListUsers`
- `ListWebApps`
- `ListWorkflows`
- `UpdateAgreement`
- `UpdateCertificate`
- `UpdateConnector`
- `UpdateProfile`
- `UpdateServer`
- `UpdateUser`
- `UpdateWebApp`
- `UpdateWebAppCustomization`

<details><summary>Not yet implemented APIs (27)</summary>

- `CreateAccess`
- `DeleteAccess`
- `DeleteHostKey`
- `DescribeAccess`
- `DescribeExecution`
- `DescribeHostKey`
- `DescribeSecurityPolicy`
- `ImportHostKey`
- `ListAccesses`
- `ListExecutions`
- `ListFileTransferResults`
- `ListHostKeys`
- `ListSecurityPolicies`
- `ListTagsForResource`
- `SendWorkflowStepState`
- `StartDirectoryListing`
- `StartFileTransfer`
- `StartRemoteDelete`
- `StartRemoteMove`
- `StartServer`
- `StopServer`
- `TagResource`
- `TestConnection`
- `TestIdentityProvider`
- `UntagResource`
- `UpdateAccess`
- `UpdateHostKey`

</details>
