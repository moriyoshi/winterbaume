# winterbaume-workspaces

Amazon WorkSpaces service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | WorkSpaces |
| AWS model | `workspaces` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 50/91 operations (54.9%) |
| stubs (routed, returns empty/default) | 0/91 operations (0.0%) |
| moto coverage | 16/91 operations (17.6%) |
| floci coverage | 0/91 operations (0.0%) |
| kumo coverage | 0/91 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws workspaces describe-workspaces
```

## Current Network Resource Stub Semantics

WorkSpaces currently synthesises directory and workspace networking inside WorkSpaces state.

- `CreateWorkspaces` mints a synthetic subnet ID and static private IP address for each workspace.
- If a requested directory is missing, the service auto-creates a stub directory with a synthetic workspace security group ID.
- `RegisterWorkspaceDirectory` also creates a directory-local workspace security group ID; directory registration is not checked against Directory Service or EC2.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_workspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_workspaces::WorkSpacesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(WorkSpacesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_workspaces::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_workspaces::Client::new(&config);

    let resp = client
        .describe_workspaces()
        .send()
        .await
        .expect("describe_workspaces should succeed");
    println!("WorkSpaces: {}", resp.workspaces().len());
}
```

## Implemented APIs (50)

- `AssociateConnectionAlias`
- `AssociateIpGroups`
- `AuthorizeIpRules`
- `CreateConnectionAlias`
- `CreateIpGroup`
- `CreateTags`
- `CreateWorkspaceBundle`
- `CreateWorkspaceImage`
- `CreateWorkspaces`
- `CreateWorkspacesPool`
- `DeleteConnectionAlias`
- `DeleteIpGroup`
- `DeleteTags`
- `DeleteWorkspaceBundle`
- `DeleteWorkspaceImage`
- `DeregisterWorkspaceDirectory`
- `DescribeClientProperties`
- `DescribeConnectionAliasPermissions`
- `DescribeConnectionAliases`
- `DescribeIpGroups`
- `DescribeTags`
- `DescribeWorkspaceBundles`
- `DescribeWorkspaceDirectories`
- `DescribeWorkspaceImagePermissions`
- `DescribeWorkspaceImages`
- `DescribeWorkspaces`
- `DescribeWorkspacesConnectionStatus`
- `DescribeWorkspacesPools`
- `DisassociateConnectionAlias`
- `DisassociateIpGroups`
- `ModifyClientProperties`
- `ModifySelfservicePermissions`
- `ModifyWorkspaceCreationProperties`
- `ModifyWorkspaceProperties`
- `ModifyWorkspaceState`
- `RebootWorkspaces`
- `RebuildWorkspaces`
- `RegisterWorkspaceDirectory`
- `RestoreWorkspace`
- `RevokeIpRules`
- `StartWorkspaces`
- `StartWorkspacesPool`
- `StopWorkspaces`
- `StopWorkspacesPool`
- `TerminateWorkspaces`
- `TerminateWorkspacesPool`
- `UpdateConnectionAliasPermission`
- `UpdateRulesOfIpGroup`
- `UpdateWorkspaceImagePermission`
- `UpdateWorkspacesPool`

<details><summary>Not yet implemented APIs (41)</summary>

- `AcceptAccountLinkInvitation`
- `AssociateWorkspaceApplication`
- `CopyWorkspaceImage`
- `CreateAccountLinkInvitation`
- `CreateConnectClientAddIn`
- `CreateStandbyWorkspaces`
- `CreateUpdatedWorkspaceImage`
- `DeleteAccountLinkInvitation`
- `DeleteClientBranding`
- `DeleteConnectClientAddIn`
- `DeployWorkspaceApplications`
- `DescribeAccount`
- `DescribeAccountModifications`
- `DescribeApplicationAssociations`
- `DescribeApplications`
- `DescribeBundleAssociations`
- `DescribeClientBranding`
- `DescribeConnectClientAddIns`
- `DescribeCustomWorkspaceImageImport`
- `DescribeImageAssociations`
- `DescribeWorkspaceAssociations`
- `DescribeWorkspaceSnapshots`
- `DescribeWorkspacesPoolSessions`
- `DisassociateWorkspaceApplication`
- `GetAccountLink`
- `ImportClientBranding`
- `ImportCustomWorkspaceImage`
- `ImportWorkspaceImage`
- `ListAccountLinks`
- `ListAvailableManagementCidrRanges`
- `MigrateWorkspace`
- `ModifyAccount`
- `ModifyCertificateBasedAuthProperties`
- `ModifyEndpointEncryptionMode`
- `ModifySamlProperties`
- `ModifyStreamingProperties`
- `ModifyWorkspaceAccessProperties`
- `RejectAccountLinkInvitation`
- `TerminateWorkspacesPoolSession`
- `UpdateConnectClientAddIn`
- `UpdateWorkspaceBundle`

</details>
