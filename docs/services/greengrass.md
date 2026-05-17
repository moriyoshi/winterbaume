# winterbaume-greengrass

AWS IoT Greengrass service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Greengrass |
| AWS model | `greengrass` |
| Protocol | restJson1 |
| winterbaume coverage | 71/92 operations (77.2%) |
| stubs (routed, returns empty/default) | 0/92 operations (0.0%) |
| moto coverage | 55/92 operations (59.8%) |
| floci coverage | 0/92 operations (0.0%) |
| kumo coverage | 0/92 operations (0.0%) |
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
aws greengrass list-groups
```

## Example

```rust
use aws_sdk_greengrass::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_greengrass::GreengrassService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(GreengrassService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_greengrass::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_greengrass::Client::new(&config);

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");
    println!("Greengrass groups: {}", resp.groups().len());
}
```

## Implemented APIs (71)

- `AssociateRoleToGroup`
- `CreateConnectorDefinition`
- `CreateConnectorDefinitionVersion`
- `CreateCoreDefinition`
- `CreateCoreDefinitionVersion`
- `CreateDeployment`
- `CreateDeviceDefinition`
- `CreateDeviceDefinitionVersion`
- `CreateFunctionDefinition`
- `CreateFunctionDefinitionVersion`
- `CreateGroup`
- `CreateGroupVersion`
- `CreateLoggerDefinition`
- `CreateLoggerDefinitionVersion`
- `CreateResourceDefinition`
- `CreateResourceDefinitionVersion`
- `CreateSubscriptionDefinition`
- `CreateSubscriptionDefinitionVersion`
- `DeleteConnectorDefinition`
- `DeleteCoreDefinition`
- `DeleteDeviceDefinition`
- `DeleteFunctionDefinition`
- `DeleteGroup`
- `DeleteLoggerDefinition`
- `DeleteResourceDefinition`
- `DeleteSubscriptionDefinition`
- `DisassociateRoleFromGroup`
- `GetAssociatedRole`
- `GetConnectorDefinition`
- `GetConnectorDefinitionVersion`
- `GetCoreDefinition`
- `GetCoreDefinitionVersion`
- `GetDeploymentStatus`
- `GetDeviceDefinition`
- `GetDeviceDefinitionVersion`
- `GetFunctionDefinition`
- `GetFunctionDefinitionVersion`
- `GetGroup`
- `GetGroupVersion`
- `GetLoggerDefinition`
- `GetLoggerDefinitionVersion`
- `GetResourceDefinition`
- `GetResourceDefinitionVersion`
- `GetSubscriptionDefinition`
- `GetSubscriptionDefinitionVersion`
- `ListConnectorDefinitionVersions`
- `ListConnectorDefinitions`
- `ListCoreDefinitionVersions`
- `ListCoreDefinitions`
- `ListDeployments`
- `ListDeviceDefinitionVersions`
- `ListDeviceDefinitions`
- `ListFunctionDefinitionVersions`
- `ListFunctionDefinitions`
- `ListGroupVersions`
- `ListGroups`
- `ListLoggerDefinitionVersions`
- `ListLoggerDefinitions`
- `ListResourceDefinitionVersions`
- `ListResourceDefinitions`
- `ListSubscriptionDefinitionVersions`
- `ListSubscriptionDefinitions`
- `ResetDeployments`
- `UpdateConnectorDefinition`
- `UpdateCoreDefinition`
- `UpdateDeviceDefinition`
- `UpdateFunctionDefinition`
- `UpdateGroup`
- `UpdateLoggerDefinition`
- `UpdateResourceDefinition`
- `UpdateSubscriptionDefinition`

<details><summary>Not yet implemented APIs (21)</summary>

- `AssociateServiceRoleToAccount`
- `CreateGroupCertificateAuthority`
- `CreateSoftwareUpdateJob`
- `DisassociateServiceRoleFromAccount`
- `GetBulkDeploymentStatus`
- `GetConnectivityInfo`
- `GetGroupCertificateAuthority`
- `GetGroupCertificateConfiguration`
- `GetServiceRoleForAccount`
- `GetThingRuntimeConfiguration`
- `ListBulkDeploymentDetailedReports`
- `ListBulkDeployments`
- `ListGroupCertificateAuthorities`
- `ListTagsForResource`
- `StartBulkDeployment`
- `StopBulkDeployment`
- `TagResource`
- `UntagResource`
- `UpdateConnectivityInfo`
- `UpdateGroupCertificateConfiguration`
- `UpdateThingRuntimeConfiguration`

</details>
