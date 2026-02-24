# Amazon WorkSpaces

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon WorkSpaces Service Amazon WorkSpaces enables you to provision virtual, cloud-based Microsoft Windows or Amazon Linux desktops for your users, known as WorkSpaces . WorkSpaces eliminates the need to procure and deploy hardware or install complex software. You can quickly add or remove users as your needs change. Users can access their virtual desktops from multiple devices or web browsers. This API Reference provides detailed information about the actions, data types, parameters, and errors of the WorkSpaces service.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-workspaces/tests/scenario_test.rs`: run a full WorkSpace lifecycle with create, tag, modify properties, stop, start, and terminate.
- Backported from `scenario_test.rs`: create an image from a WorkSpace and create a bundle from that image.
- Backported from `scenario_test.rs`: manage IP group access control rules and associate them with a directory.
- Scenario insight from EC2: include mutable binding failover for Amazon WorkSpaces where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon WorkSpaces by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon WorkSpaces resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support virtual desktop provisioning, bundles/images, directory registration, workspace state transitions, access control groups, tags, and user-facing lifecycle operations.

## Service Identity and Protocol

- AWS model slug: `workspaces`
- AWS SDK for Rust slug: `workspaces`
- Model version: `2015-04-08`
- Model file: `vendor/api-models-aws/models/workspaces/service/2015-04-08/workspaces-2015-04-08.json`
- SDK ID: `WorkSpaces`
- Endpoint prefix: `workspaces`
- ARN namespace: `workspaces`
- CloudFormation name: `WorkSpaces`
- CloudTrail event source: `workspaces.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (24), `Create` (11), `Modify` (11), `Delete` (8), `Update` (6), `Associate` (3), `Disassociate` (3), `Import` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptAccountLinkInvitation`, `AssociateConnectionAlias`, `AssociateIpGroups`, `AssociateWorkspaceApplication`, `CreateAccountLinkInvitation`, `CreateConnectClientAddIn`, `CreateConnectionAlias`, `CreateIpGroup`, `CreateStandbyWorkspaces`, `CreateTags`, `CreateUpdatedWorkspaceImage`, `CreateWorkspaceBundle`, `CreateWorkspaceImage`, `CreateWorkspaces`, `CreateWorkspacesPool`, `DeleteAccountLinkInvitation`, `DeleteClientBranding`, `DeleteConnectClientAddIn`, `DeleteConnectionAlias`, `DeleteIpGroup`, ... (+38).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccount`, `DescribeAccountModifications`, `DescribeApplicationAssociations`, `DescribeApplications`, `DescribeBundleAssociations`, `DescribeClientBranding`, `DescribeClientProperties`, `DescribeConnectClientAddIns`, `DescribeConnectionAliasPermissions`, `DescribeConnectionAliases`, `DescribeCustomWorkspaceImageImport`, `DescribeImageAssociations`, `DescribeIpGroups`, `DescribeTags`, `DescribeWorkspaceAssociations`, `DescribeWorkspaceBundles`, `DescribeWorkspaceDirectories`, `DescribeWorkspaceImagePermissions`, `DescribeWorkspaceImages`, `DescribeWorkspaceSnapshots`, ... (+7).
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeCustomWorkspaceImageImport`, `ImportClientBranding`, `ImportCustomWorkspaceImage`, `ImportWorkspaceImage`, `StartWorkspaces`, `StartWorkspacesPool`, `StopWorkspaces`, `StopWorkspacesPool`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 88 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Current Network Resource Stub Semantics

WorkSpaces currently synthesises directory and workspace networking inside WorkSpaces state.

- `CreateWorkspaces` mints a synthetic subnet ID and static private IP address for each workspace.
- If a requested directory is missing, the service auto-creates a stub directory with a synthetic workspace security group ID.
- `RegisterWorkspaceDirectory` also creates a directory-local workspace security group ID; directory registration is not checked against Directory Service or EC2.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccount`, `DescribeAccountModifications`, `DescribeApplicationAssociations`, `DescribeApplications`, `DescribeBundleAssociations`, `DescribeClientBranding`, `DescribeClientProperties`, `DescribeConnectClientAddIns`, `DescribeConnectionAliasPermissions`, `DescribeConnectionAliases`, `DescribeCustomWorkspaceImageImport`, `DescribeImageAssociations`, `DescribeIpGroups`, `DescribeTags`, `DescribeWorkspaceAssociations`, `DescribeWorkspaceBundles`, `DescribeWorkspaceDirectories`, `DescribeWorkspaceImagePermissions`, `DescribeWorkspaceImages`, `DescribeWorkspaceSnapshots`, `DescribeWorkspaces`, `DescribeWorkspacesConnectionStatus`, `DescribeWorkspacesPoolSessions`, `DescribeWorkspacesPools`
- Traits: `paginated` (5)
- Common required input members in this group: `AliasId`, `ApplicationId`, `AssociatedResourceTypes`, `BundleId`, `ImageId`, `PoolId`, `ResourceId`, `ResourceIds`, `WorkspaceId`

### Create

- Operations: `CreateAccountLinkInvitation`, `CreateConnectClientAddIn`, `CreateConnectionAlias`, `CreateIpGroup`, `CreateStandbyWorkspaces`, `CreateTags`, `CreateUpdatedWorkspaceImage`, `CreateWorkspaceBundle`, `CreateWorkspaceImage`, `CreateWorkspaces`, `CreateWorkspacesPool`
- Common required input members in this group: `BundleDescription`, `BundleId`, `BundleName`, `Capacity`, `ComputeType`, `ConnectionString`, `Description`, `DirectoryId`, `GroupName`, `ImageId`, `Name`, `PoolName`, `PrimaryRegion`, `ResourceId`, `SourceImageId`, `StandbyWorkspaces`, `Tags`, `TargetAccountId`, `URL`, `UserStorage`, `WorkspaceId`, `Workspaces`

### Modify

- Operations: `ModifyAccount`, `ModifyCertificateBasedAuthProperties`, `ModifyClientProperties`, `ModifyEndpointEncryptionMode`, `ModifySamlProperties`, `ModifySelfservicePermissions`, `ModifyStreamingProperties`, `ModifyWorkspaceAccessProperties`, `ModifyWorkspaceCreationProperties`, `ModifyWorkspaceProperties`, `ModifyWorkspaceState`
- Common required input members in this group: `ClientProperties`, `DirectoryId`, `EndpointEncryptionMode`, `ResourceId`, `SelfservicePermissions`, `WorkspaceAccessProperties`, `WorkspaceCreationProperties`, `WorkspaceId`, `WorkspaceState`

### Delete

- Operations: `DeleteAccountLinkInvitation`, `DeleteClientBranding`, `DeleteConnectClientAddIn`, `DeleteConnectionAlias`, `DeleteIpGroup`, `DeleteTags`, `DeleteWorkspaceBundle`, `DeleteWorkspaceImage`
- Common required input members in this group: `AddInId`, `AliasId`, `GroupId`, `ImageId`, `LinkId`, `Platforms`, `ResourceId`, `TagKeys`

### Update

- Operations: `UpdateConnectClientAddIn`, `UpdateConnectionAliasPermission`, `UpdateRulesOfIpGroup`, `UpdateWorkspaceBundle`, `UpdateWorkspaceImagePermission`, `UpdateWorkspacesPool`
- Common required input members in this group: `AddInId`, `AliasId`, `AllowCopyImage`, `ConnectionAliasPermission`, `GroupId`, `ImageId`, `PoolId`, `ResourceId`, `SharedAccountId`, `UserRules`

### Associate

- Operations: `AssociateConnectionAlias`, `AssociateIpGroups`, `AssociateWorkspaceApplication`
- Common required input members in this group: `AliasId`, `ApplicationId`, `DirectoryId`, `GroupIds`, `ResourceId`, `WorkspaceId`

### Disassociate

- Operations: `DisassociateConnectionAlias`, `DisassociateIpGroups`, `DisassociateWorkspaceApplication`
- Common required input members in this group: `AliasId`, `ApplicationId`, `DirectoryId`, `GroupIds`, `WorkspaceId`

### Import

- Operations: `ImportClientBranding`, `ImportCustomWorkspaceImage`, `ImportWorkspaceImage`
- Common required input members in this group: `ComputeType`, `Ec2ImageId`, `ImageDescription`, `ImageName`, `ImageSource`, `InfrastructureConfigurationArn`, `IngestionProcess`, `OsVersion`, `Platform`, `Protocol`, `ResourceId`

### Terminate

- Operations: `TerminateWorkspaces`, `TerminateWorkspacesPool`, `TerminateWorkspacesPoolSession`
- Common required input members in this group: `PoolId`, `SessionId`, `TerminateWorkspaceRequests`

### List

- Operations: `ListAccountLinks`, `ListAvailableManagementCidrRanges`
- Traits: `paginated` (1)
- Common required input members in this group: `ManagementCidrRangeConstraint`

### Start

- Operations: `StartWorkspaces`, `StartWorkspacesPool`
- Common required input members in this group: `PoolId`, `StartWorkspaceRequests`

### Stop

- Operations: `StopWorkspaces`, `StopWorkspacesPool`
- Common required input members in this group: `PoolId`, `StopWorkspaceRequests`

### Accept

- Operations: `AcceptAccountLinkInvitation`
- Common required input members in this group: `LinkId`

### Authorize

- Operations: `AuthorizeIpRules`
- Common required input members in this group: `GroupId`, `UserRules`

### Copy

- Operations: `CopyWorkspaceImage`
- Common required input members in this group: `Name`, `SourceImageId`, `SourceRegion`

### Deploy

- Operations: `DeployWorkspaceApplications`
- Common required input members in this group: `WorkspaceId`

### Deregister

- Operations: `DeregisterWorkspaceDirectory`
- Common required input members in this group: `DirectoryId`

### Get

- Operations: `GetAccountLink`

### Migrate

- Operations: `MigrateWorkspace`
- Common required input members in this group: `BundleId`, `SourceWorkspaceId`

### Reboot

- Operations: `RebootWorkspaces`
- Common required input members in this group: `RebootWorkspaceRequests`

### Rebuild

- Operations: `RebuildWorkspaces`
- Common required input members in this group: `RebuildWorkspaceRequests`

### Register

- Operations: `RegisterWorkspaceDirectory`

### Reject

- Operations: `RejectAccountLinkInvitation`
- Common required input members in this group: `LinkId`

### Restore

- Operations: `RestoreWorkspace`
- Common required input members in this group: `WorkspaceId`

### Revoke

- Operations: `RevokeIpRules`
- Common required input members in this group: `GroupId`, `UserRules`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptAccountLinkInvitation` | - | - | `LinkId` | - | `AcceptAccountLinkInvitationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Accepts the account link invitation. There's currently no unlinking capability after you accept the account linking invitation. |
| `AssociateConnectionAlias` | - | - | `AliasId`, `ResourceId` | - | `AssociateConnectionAliasResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceAssociatedException`, `ResourceNotFoundException` | Associates the specified connection alias with the specified directory to enable cross-Region redirection. For more information, see Cross-Region Redirection for Amazon WorkSpaces. |
| `AssociateIpGroups` | - | - | `DirectoryId`, `GroupIds` | - | `AssociateIpGroupsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Associates the specified IP access control group with the specified directory. |
| `AssociateWorkspaceApplication` | - | - | `ApplicationId`, `WorkspaceId` | - | `AssociateWorkspaceApplicationResult` | `AccessDeniedException`, `ApplicationNotSupportedException`, `ComputeNotCompatibleException`, `IncompatibleApplicationsException`, `InvalidParameterValuesException`, `OperatingSystemNotCompatibleException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, ... (+2) | Associates the specified application to the specified WorkSpace. |
| `AuthorizeIpRules` | - | - | `GroupId`, `UserRules` | - | `AuthorizeIpRulesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Adds one or more rules to the specified IP access control group. This action gives users permission to access their WorkSpaces from the CIDR address ranges specified in the rules. |
| `CopyWorkspaceImage` | - | - | `Name`, `SourceImageId`, `SourceRegion` | - | `CopyWorkspaceImageResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException` | Copies the specified image from the specified Region to the current Region. For more information about copying images, see Copy a Custom WorkSpaces Image. |
| `CreateAccountLinkInvitation` | - | - | `TargetAccountId` | - | `CreateAccountLinkInvitationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Creates the account link invitation. |
| `CreateConnectClientAddIn` | - | - | `Name`, `ResourceId`, `URL` | - | `CreateConnectClientAddInResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceAlreadyExistsException`, `ResourceCreationFailedException`, `ResourceNotFoundException` | Creates a client-add-in for Amazon Connect within a directory. You can create only one Amazon Connect client add-in within a directory. |
| `CreateConnectionAlias` | - | - | `ConnectionString` | - | `CreateConnectionAliasResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException` | Creates the specified connection alias for use with cross-Region redirection. For more information, see Cross-Region Redirection for Amazon WorkSpaces. |
| `CreateIpGroup` | - | - | `GroupName` | - | `CreateIpGroupResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceAlreadyExistsException`, `ResourceCreationFailedException`, `ResourceLimitExceededException` | Creates an IP access control group. An IP access control group provides you with the ability to control the IP addresses from which users are allowed to access their WorkSpaces. |
| `CreateStandbyWorkspaces` | - | - | `PrimaryRegion`, `StandbyWorkspaces` | - | `CreateStandbyWorkspacesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Creates a standby WorkSpace in a secondary Region. |
| `CreateTags` | - | - | `ResourceId`, `Tags` | - | `CreateTagsResult` | `InvalidParameterValuesException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Creates the specified tags for the specified WorkSpaces resource. |
| `CreateUpdatedWorkspaceImage` | - | - | `Description`, `Name`, `SourceImageId` | - | `CreateUpdatedWorkspaceImageResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Creates a new updated WorkSpace image based on the specified source image. The new updated WorkSpace image has the latest drivers and other updates required by the Amazon WorkSpaces components. |
| `CreateWorkspaceBundle` | - | - | `BundleDescription`, `BundleName`, `ComputeType`, `ImageId`, `UserStorage` | - | `CreateWorkspaceBundleResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException` | Creates the specified WorkSpace bundle. For more information about creating WorkSpace bundles, see Create a Custom WorkSpaces Image and Bundle. |
| `CreateWorkspaceImage` | - | - | `Description`, `Name`, `WorkspaceId` | - | `CreateWorkspaceImageResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Creates a new WorkSpace image from an existing WorkSpace. |
| `CreateWorkspaces` | - | - | `Workspaces` | - | `CreateWorkspacesResult` | `InvalidParameterValuesException`, `ResourceLimitExceededException` | Creates one or more WorkSpaces. This operation is asynchronous and returns before the WorkSpaces are created. |
| `CreateWorkspacesPool` | - | - | `BundleId`, `Capacity`, `Description`, `DirectoryId`, `PoolName` | - | `CreateWorkspacesPoolResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Creates a pool of WorkSpaces. |
| `DeleteAccountLinkInvitation` | - | - | `LinkId` | - | `DeleteAccountLinkInvitationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the account link invitation. |
| `DeleteClientBranding` | - | - | `Platforms`, `ResourceId` | - | `DeleteClientBrandingResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Deletes customized client branding. Client branding allows you to customize your WorkSpace's client login portal. |
| `DeleteConnectClientAddIn` | - | - | `AddInId`, `ResourceId` | - | `DeleteConnectClientAddInResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Deletes a client-add-in for Amazon Connect that is configured within a directory. |
| `DeleteConnectionAlias` | - | - | `AliasId` | - | `DeleteConnectionAliasResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceAssociatedException`, `ResourceNotFoundException` | Deletes the specified connection alias. For more information, see Cross-Region Redirection for Amazon WorkSpaces. |
| `DeleteIpGroup` | - | - | `GroupId` | - | `DeleteIpGroupResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceAssociatedException`, `ResourceNotFoundException` | Deletes the specified IP access control group. You cannot delete an IP access control group that is associated with a directory. |
| `DeleteTags` | - | - | `ResourceId`, `TagKeys` | - | `DeleteTagsResult` | `InvalidParameterValuesException`, `ResourceNotFoundException` | Deletes the specified tags from the specified WorkSpaces resource. |
| `DeleteWorkspaceBundle` | - | - | - | - | `DeleteWorkspaceBundleResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceAssociatedException`, `ResourceNotFoundException` | Deletes the specified WorkSpace bundle. For more information about deleting WorkSpace bundles, see Delete a Custom WorkSpaces Bundle or Image. |
| `DeleteWorkspaceImage` | - | - | `ImageId` | - | `DeleteWorkspaceImageResult` | `AccessDeniedException`, `InvalidResourceStateException`, `ResourceAssociatedException` | Deletes the specified image from your account. To delete an image, you must first delete any bundles that are associated with the image and unshare the image if it is shared with other accounts. |
| `DeployWorkspaceApplications` | - | - | `WorkspaceId` | - | `DeployWorkspaceApplicationsResult` | `AccessDeniedException`, `IncompatibleApplicationsException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceInUseException`, `ResourceNotFoundException` | Deploys associated applications to the specified WorkSpace |
| `DeregisterWorkspaceDirectory` | - | - | `DirectoryId` | - | `DeregisterWorkspaceDirectoryResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Deregisters the specified directory. This operation is asynchronous and returns before the WorkSpace directory is deregistered. |
| `DescribeAccount` | - | - | - | - | `DescribeAccountResult` | `AccessDeniedException` | Retrieves a list that describes the configuration of Bring Your Own License (BYOL) for the specified account. |
| `DescribeAccountModifications` | - | - | - | - | `DescribeAccountModificationsResult` | `AccessDeniedException` | Retrieves a list that describes modifications to the configuration of Bring Your Own License (BYOL) for the specified account. |
| `DescribeApplicationAssociations` | - | `paginated` | `ApplicationId`, `AssociatedResourceTypes` | - | `DescribeApplicationAssociationsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Describes the associations between the application and the specified associated resources. |
| `DescribeApplications` | - | `paginated` | - | - | `DescribeApplicationsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Describes the specified applications by filtering based on their compute types, license availability, operating systems, and owners. |
| `DescribeBundleAssociations` | - | - | `AssociatedResourceTypes`, `BundleId` | - | `DescribeBundleAssociationsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Describes the associations between the applications and the specified bundle. |
| `DescribeClientBranding` | - | - | `ResourceId` | - | `DescribeClientBrandingResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Describes the specified client branding. Client branding allows you to customize the log in page of various device types for your users. |
| `DescribeClientProperties` | - | - | `ResourceIds` | - | `DescribeClientPropertiesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Retrieves a list that describes one or more specified Amazon WorkSpaces clients. |
| `DescribeConnectClientAddIns` | - | - | `ResourceId` | - | `DescribeConnectClientAddInsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Retrieves a list of Amazon Connect client add-ins that have been created. |
| `DescribeConnectionAliasPermissions` | - | - | `AliasId` | - | `DescribeConnectionAliasPermissionsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Describes the permissions that the owner of a connection alias has granted to another Amazon Web Services account for the specified connection alias. For more information, see Cross-Region Redirection for Amazon WorkSpaces. |
| `DescribeConnectionAliases` | - | - | - | - | `DescribeConnectionAliasesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException` | Retrieves a list that describes the connection aliases used for cross-Region redirection. For more information, see Cross-Region Redirection for Amazon WorkSpaces. |
| `DescribeCustomWorkspaceImageImport` | - | - | `ImageId` | - | `DescribeCustomWorkspaceImageImportResult` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves information about a WorkSpace BYOL image being imported via ImportCustomWorkspaceImage. |
| `DescribeImageAssociations` | - | - | `AssociatedResourceTypes`, `ImageId` | - | `DescribeImageAssociationsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Describes the associations between the applications and the specified image. |
| `DescribeIpGroups` | - | - | - | - | `DescribeIpGroupsResult` | `AccessDeniedException`, `InvalidParameterValuesException` | Describes one or more of your IP access control groups. |
| `DescribeTags` | - | - | `ResourceId` | - | `DescribeTagsResult` | `ResourceNotFoundException` | Describes the specified tags for the specified WorkSpaces resource. |
| `DescribeWorkspaceAssociations` | - | - | `AssociatedResourceTypes`, `WorkspaceId` | - | `DescribeWorkspaceAssociationsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Describes the associations betweens applications and the specified WorkSpace. |
| `DescribeWorkspaceBundles` | - | `paginated` | - | - | `DescribeWorkspaceBundlesResult` | `InvalidParameterValuesException` | Retrieves a list that describes the available WorkSpace bundles. You can filter the results using either bundle ID or owner, but not both. |
| `DescribeWorkspaceDirectories` | - | `paginated` | - | - | `DescribeWorkspaceDirectoriesResult` | `InvalidParameterValuesException` | Describes the available directories that are registered with Amazon WorkSpaces. |
| `DescribeWorkspaceImagePermissions` | - | - | `ImageId` | - | `DescribeWorkspaceImagePermissionsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Describes the permissions that the owner of an image has granted to other Amazon Web Services accounts for an image. |
| `DescribeWorkspaceImages` | - | - | - | - | `DescribeWorkspaceImagesResult` | `AccessDeniedException` | Retrieves a list that describes one or more specified images, if the image identifiers are provided. Otherwise, all images in the account are described. |
| `DescribeWorkspaceSnapshots` | - | - | `WorkspaceId` | - | `DescribeWorkspaceSnapshotsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Describes the snapshots for the specified WorkSpace. |
| `DescribeWorkspaces` | - | `paginated` | - | - | `DescribeWorkspacesResult` | `InvalidParameterValuesException`, `ResourceUnavailableException` | Describes the specified WorkSpaces. You can filter the results by using the bundle identifier, directory identifier, or owner, but you can specify only one filter at a time. |
| `DescribeWorkspacesConnectionStatus` | - | - | - | - | `DescribeWorkspacesConnectionStatusResult` | `InvalidParameterValuesException` | Describes the connection status of the specified WorkSpaces. |
| `DescribeWorkspacesPoolSessions` | - | - | `PoolId` | - | `DescribeWorkspacesPoolSessionsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Retrieves a list that describes the streaming sessions for a specified pool. |
| `DescribeWorkspacesPools` | - | - | - | - | `DescribeWorkspacesPoolsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Describes the specified WorkSpaces Pools. |
| `DisassociateConnectionAlias` | - | - | `AliasId` | - | `DisassociateConnectionAliasResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Disassociates a connection alias from a directory. Disassociating a connection alias disables cross-Region redirection between two directories in different Regions. |
| `DisassociateIpGroups` | - | - | `DirectoryId`, `GroupIds` | - | `DisassociateIpGroupsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Disassociates the specified IP access control group from the specified directory. |
| `DisassociateWorkspaceApplication` | - | - | `ApplicationId`, `WorkspaceId` | - | `DisassociateWorkspaceApplicationResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceInUseException`, `ResourceNotFoundException` | Disassociates the specified application from a WorkSpace. |
| `GetAccountLink` | - | - | - | - | `GetAccountLinkResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves account link information. |
| `ImportClientBranding` | - | - | `ResourceId` | - | `ImportClientBrandingResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Imports client branding. Client branding allows you to customize your WorkSpace's client login portal. |
| `ImportCustomWorkspaceImage` | - | - | `ComputeType`, `ImageDescription`, `ImageName`, `ImageSource`, `InfrastructureConfigurationArn`, `OsVersion`, `Platform`, `Protocol` | - | `ImportCustomWorkspaceImageResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Imports the specified Windows 10 or 11 Bring Your Own License (BYOL) image into Amazon WorkSpaces using EC2 Image Builder. The image must be an already licensed image that is in your Amazon Web Services account, and you must own the image. |
| `ImportWorkspaceImage` | - | - | `Ec2ImageId`, `ImageDescription`, `ImageName`, `IngestionProcess` | - | `ImportWorkspaceImageResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Imports the specified Windows 10 or 11 Bring Your Own License (BYOL) image into Amazon WorkSpaces. The image must be an already licensed Amazon EC2 image that is in your Amazon Web Services account, and you must own the image. |
| `ListAccountLinks` | - | `paginated` | - | - | `ListAccountLinksResult` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Lists all account links. |
| `ListAvailableManagementCidrRanges` | - | - | `ManagementCidrRangeConstraint` | - | `ListAvailableManagementCidrRangesResult` | `AccessDeniedException`, `InvalidParameterValuesException` | Retrieves a list of IP address ranges, specified as IPv4 CIDR blocks, that you can use for the network management interface when you enable Bring Your Own License (BYOL). This operation can be run only by Amazon Web Services accounts that are enabled for BYOL. |
| `MigrateWorkspace` | - | - | `BundleId`, `SourceWorkspaceId` | - | `MigrateWorkspaceResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationInProgressException`, `OperationNotSupportedException`, `ResourceNotFoundException`, `ResourceUnavailableException` | Migrates a WorkSpace from one operating system or bundle type to another, while retaining the data on the user volume. The migration process recreates the WorkSpace by using a new root volume from the target bundle image and the user volume from the last... |
| `ModifyAccount` | - | - | - | - | `ModifyAccountResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `ResourceNotFoundException`, `ResourceUnavailableException` | Modifies the configuration of Bring Your Own License (BYOL) for the specified account. |
| `ModifyCertificateBasedAuthProperties` | - | - | `ResourceId` | - | `ModifyCertificateBasedAuthPropertiesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Modifies the properties of the certificate-based authentication you want to use with your WorkSpaces. |
| `ModifyClientProperties` | - | - | `ClientProperties`, `ResourceId` | - | `ModifyClientPropertiesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Modifies the properties of the specified Amazon WorkSpaces clients. |
| `ModifyEndpointEncryptionMode` | - | - | `DirectoryId`, `EndpointEncryptionMode` | - | `ModifyEndpointEncryptionModeResponse` | `AccessDeniedException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Modifies the endpoint encryption mode that allows you to configure the specified directory between Standard TLS and FIPS 140-2 validated mode. |
| `ModifySamlProperties` | - | - | `ResourceId` | - | `ModifySamlPropertiesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Modifies multiple properties related to SAML 2.0 authentication, including the enablement status, user access URL, and relay state parameter name that are used for configuring federation with an SAML 2.0 identity provider. |
| `ModifySelfservicePermissions` | - | - | `ResourceId`, `SelfservicePermissions` | - | `ModifySelfservicePermissionsResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Modifies the self-service WorkSpace management capabilities for your users. For more information, see Enable Self-Service WorkSpace Management Capabilities for Your Users. |
| `ModifyStreamingProperties` | - | - | `ResourceId` | - | `ModifyStreamingPropertiesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Modifies the specified streaming properties. |
| `ModifyWorkspaceAccessProperties` | - | - | `ResourceId`, `WorkspaceAccessProperties` | - | `ModifyWorkspaceAccessPropertiesResult` | `AccessDeniedException`, `InvalidParameterCombinationException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Specifies which devices and operating systems users can use to access their WorkSpaces. For more information, see Control Device Access. |
| `ModifyWorkspaceCreationProperties` | - | - | `ResourceId`, `WorkspaceCreationProperties` | - | `ModifyWorkspaceCreationPropertiesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Modify the default properties used to create WorkSpaces. |
| `ModifyWorkspaceProperties` | - | - | `WorkspaceId` | - | `ModifyWorkspacePropertiesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationInProgressException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `UnsupportedWorkspaceConfigurationException` | Modifies the specified WorkSpace properties. For important information about how to modify the size of the root and user volumes, see Modify a WorkSpace. |
| `ModifyWorkspaceState` | - | - | `WorkspaceId`, `WorkspaceState` | - | `ModifyWorkspaceStateResult` | `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Sets the state of the specified WorkSpace. To maintain a WorkSpace without being interrupted, set the WorkSpace state to `ADMIN_MAINTENANCE`. |
| `RebootWorkspaces` | - | - | `RebootWorkspaceRequests` | - | `RebootWorkspacesResult` | `OperationNotSupportedException` | Reboots the specified WorkSpaces. You cannot reboot a WorkSpace unless its state is `AVAILABLE`, `UNHEALTHY`, or `REBOOTING`. |
| `RebuildWorkspaces` | - | - | `RebuildWorkspaceRequests` | - | `RebuildWorkspacesResult` | `OperationNotSupportedException` | Rebuilds the specified WorkSpace. You cannot rebuild a WorkSpace unless its state is `AVAILABLE`, `ERROR`, `UNHEALTHY`, `STOPPED`, or `REBOOTING`. |
| `RegisterWorkspaceDirectory` | - | - | - | - | `RegisterWorkspaceDirectoryResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `UnsupportedNetworkConfigurationException`, ... (+1) | Registers the specified directory. This operation is asynchronous and returns before the WorkSpace directory is registered. |
| `RejectAccountLinkInvitation` | - | - | `LinkId` | - | `RejectAccountLinkInvitationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Rejects the account link invitation. |
| `RestoreWorkspace` | - | - | `WorkspaceId` | - | `RestoreWorkspaceResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Restores the specified WorkSpace to its last known healthy state. You cannot restore a WorkSpace unless its state is ` AVAILABLE`, `ERROR`, `UNHEALTHY`, or `STOPPED`. |
| `RevokeIpRules` | - | - | `GroupId`, `UserRules` | - | `RevokeIpRulesResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `ResourceNotFoundException` | Removes one or more rules from the specified IP access control group. |
| `StartWorkspaces` | - | - | `StartWorkspaceRequests` | - | `StartWorkspacesResult` | - | Starts the specified WorkSpaces. You cannot start a WorkSpace unless it has a running mode of `AutoStop` or `Manual` and a state of `STOPPED`. |
| `StartWorkspacesPool` | - | - | `PoolId` | - | `StartWorkspacesPoolResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationInProgressException`, `OperationNotSupportedException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Starts the specified pool. You cannot start a pool unless it has a running mode of `AutoStop` and a state of `STOPPED`. |
| `StopWorkspaces` | - | - | `StopWorkspaceRequests` | - | `StopWorkspacesResult` | - | Stops the specified WorkSpaces. You cannot stop a WorkSpace unless it has a running mode of `AutoStop` or `Manual` and a state of `AVAILABLE`, `IMPAIRED`, `UNHEALTHY`, or `ERROR`. |
| `StopWorkspacesPool` | - | - | `PoolId` | - | `StopWorkspacesPoolResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationInProgressException`, `ResourceNotFoundException` | Stops the specified pool. You cannot stop a WorkSpace pool unless it has a running mode of `AutoStop` and a state of `AVAILABLE`, `IMPAIRED`, `UNHEALTHY`, or `ERROR`. |
| `TerminateWorkspaces` | - | - | `TerminateWorkspaceRequests` | - | `TerminateWorkspacesResult` | - | Terminates the specified WorkSpaces. Terminating a WorkSpace is a permanent action and cannot be undone. |
| `TerminateWorkspacesPool` | - | - | `PoolId` | - | `TerminateWorkspacesPoolResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationInProgressException`, `ResourceNotFoundException` | Terminates the specified pool. |
| `TerminateWorkspacesPoolSession` | - | - | `SessionId` | - | `TerminateWorkspacesPoolSessionResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationInProgressException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Terminates the pool session. |
| `UpdateConnectClientAddIn` | - | - | `AddInId`, `ResourceId` | - | `UpdateConnectClientAddInResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `ResourceNotFoundException` | Updates a Amazon Connect client add-in. Use this action to update the name and endpoint URL of a Amazon Connect client add-in. |
| `UpdateConnectionAliasPermission` | - | - | `AliasId`, `ConnectionAliasPermission` | - | `UpdateConnectionAliasPermissionResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationNotSupportedException`, `ResourceAssociatedException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Shares or unshares a connection alias with one account by specifying whether that account has permission to associate the connection alias with a directory. If the association permission is granted, the connection alias is shared with that account. |
| `UpdateRulesOfIpGroup` | - | - | `GroupId`, `UserRules` | - | `UpdateRulesOfIpGroupResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Replaces the current rules of the specified IP access control group with the specified rules. |
| `UpdateWorkspaceBundle` | - | - | - | - | `UpdateWorkspaceBundleResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException`, `ResourceUnavailableException` | Updates a WorkSpace bundle with a new image. For more information about updating WorkSpace bundles, see Update a Custom WorkSpaces Bundle. |
| `UpdateWorkspaceImagePermission` | - | - | `AllowCopyImage`, `ImageId`, `SharedAccountId` | - | `UpdateWorkspaceImagePermissionResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `OperationNotSupportedException`, `ResourceNotFoundException`, `ResourceUnavailableException` | Shares or unshares an image with one account in the same Amazon Web Services Region by specifying whether that account has permission to copy the image. If the copy image permission is granted, the image is shared with that account. |
| `UpdateWorkspacesPool` | - | - | `PoolId` | - | `UpdateWorkspacesPoolResult` | `AccessDeniedException`, `InvalidParameterValuesException`, `InvalidResourceStateException`, `OperationInProgressException`, `OperationNotSupportedException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Updates the specified pool. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The user is not authorized to access a resource. |
| `InvalidParameterValuesException` | `structure` | `message` | One or more parameter values are not valid. |
| `ResourceNotFoundException` | `structure` | `ResourceId`, `message` | The resource could not be found. |
| `OperationNotSupportedException` | `structure` | `message`, `reason` | This operation is not supported. |
| `InvalidResourceStateException` | `structure` | `message` | The state of the resource is not valid for this operation. |
| `ResourceLimitExceededException` | `structure` | `message` | Your resource limits have been exceeded. |
| `ResourceAlreadyExistsException` | `structure` | `message` | The specified resource already exists. |
| `ResourceUnavailableException` | `structure` | `ResourceId`, `message` | The specified resource is not available. |
| `OperationInProgressException` | `structure` | `message` | The properties of this WorkSpace are currently being modified. |
| `InternalServerException` | `structure` | `message` | Unexpected server error occured. |
| `ValidationException` | `structure` | `message` | You either haven't provided a `TargetAccountId` or are using the same value for `TargetAccountId` and `SourceAccountId`. |
| `ResourceAssociatedException` | `structure` | `message` | The resource is associated with a directory. |
| `ConflictException` | `structure` | `message` | The `TargetAccountId` is already linked or invited. |
| `ResourceInUseException` | `structure` | `ResourceId`, `message` | The specified resource is currently in use. |
| `IncompatibleApplicationsException` | `structure` | - | The specified application is not compatible with the resource. |
| `ResourceCreationFailedException` | `structure` | `message` | The resource could not be created. |
| `AcceptAccountLinkInvitationRequest` | `structure` | `ClientToken`, `LinkId` | - |
| `AcceptAccountLinkInvitationResult` | `structure` | `AccountLink` | - |
| `AssociateConnectionAliasRequest` | `structure` | `AliasId`, `ResourceId` | - |
| `AssociateConnectionAliasResult` | `structure` | `ConnectionIdentifier` | - |
| `AssociateIpGroupsRequest` | `structure` | `DirectoryId`, `GroupIds` | - |
| `AssociateIpGroupsResult` | `structure` | - | - |
| `AssociateWorkspaceApplicationRequest` | `structure` | `ApplicationId`, `WorkspaceId` | - |
| `AssociateWorkspaceApplicationResult` | `structure` | `Association` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
