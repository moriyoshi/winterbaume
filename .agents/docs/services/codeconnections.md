# AWS CodeConnections

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS CodeConnections This Amazon Web Services CodeConnections API Reference provides descriptions and usage examples of the operations and data types for the Amazon Web Services CodeConnections API. You can use the connections API to work with connections and installations. Connections are configurations that you use to connect Amazon Web Services resources to external code repositories. Each connection is a resource that can be given to services such as CodePipeline to connect to a third-party repository such as Bitbucket. For example, you can add the connection in CodePipeline so that it triggers your pipeline when a code change is made to your third-party code repository.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS CodeConnections workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetConnection`, `GetHost`, `GetRepositoryLink`, `GetRepositorySyncStatus`, `ListConnections`, `ListHosts`.

## Service Identity and Protocol

- AWS model slug: `codeconnections`
- AWS SDK for Rust slug: `codeconnections`
- Model version: `2023-12-01`
- Model file: `vendor/api-models-aws/models/codeconnections/service/2023-12-01/codeconnections-2023-12-01.json`
- SDK ID: `CodeConnections`
- Endpoint prefix: `codeconnections`
- ARN namespace: `codeconnections`
- CloudFormation name: `CodeConnections`
- CloudTrail event source: `codeconnections.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `List` (6), `Create` (4), `Delete` (4), `Update` (4), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateConnection`, `CreateHost`, `CreateRepositoryLink`, `CreateSyncConfiguration`, `DeleteConnection`, `DeleteHost`, `DeleteRepositoryLink`, `DeleteSyncConfiguration`, `TagResource`, `UntagResource`, `UpdateHost`, `UpdateRepositoryLink`, `UpdateSyncBlocker`, `UpdateSyncConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetConnection`, `GetHost`, `GetRepositoryLink`, `GetRepositorySyncStatus`, `GetResourceSyncStatus`, `GetSyncBlockerSummary`, `GetSyncConfiguration`, `ListConnections`, `ListHosts`, `ListRepositoryLinks`, `ListRepositorySyncDefinitions`, `ListSyncConfigurations`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `KMS`, `EC2/VPC`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/codeconnections/latest/APIReference/API_Host.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-codeconnections-connection.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/connections-github.html

Research outcomes:
- CodeConnections connects third-party source providers such as GitHub, Bitbucket, GitLab, and installed providers to AWS services such as CodePipeline and CodeBuild.
- A connection represents an authorisation relationship to an external provider and can start in a pending state until provider-side installation or authorisation completes.
- Hosts represent installed third-party provider infrastructure, such as GitHub Enterprise Server, and can be reused by multiple connections.
- Repository link resources associate a repository with a connection and can include encryption key and owner metadata.
- Connections are consumed by services through connection ARNs and require IAM permissions such as using the connection in a pipeline or build.
- Provider handshakes and installation steps are external to the AWS API but drive whether a connection becomes usable.

Parity implications:
- Model connections, hosts, provider type, pending/available/error state, repository links, tags, and IAM usage separately.
- CreateConnection should not imply provider authorisation is complete; usability should depend on connection status.
- Services that consume a connection should validate both IAM permission and connection availability.

## Operation Groups

### Get

- Operations: `GetConnection`, `GetHost`, `GetRepositoryLink`, `GetRepositorySyncStatus`, `GetResourceSyncStatus`, `GetSyncBlockerSummary`, `GetSyncConfiguration`
- Common required input members in this group: `Branch`, `ConnectionArn`, `HostArn`, `RepositoryLinkId`, `ResourceName`, `SyncType`

### List

- Operations: `ListConnections`, `ListHosts`, `ListRepositoryLinks`, `ListRepositorySyncDefinitions`, `ListSyncConfigurations`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `RepositoryLinkId`, `ResourceArn`, `SyncType`

### Create

- Operations: `CreateConnection`, `CreateHost`, `CreateRepositoryLink`, `CreateSyncConfiguration`
- Common required input members in this group: `Branch`, `ConfigFile`, `ConnectionArn`, `ConnectionName`, `Name`, `OwnerId`, `ProviderEndpoint`, `ProviderType`, `RepositoryLinkId`, `RepositoryName`, `ResourceName`, `RoleArn`, `SyncType`

### Delete

- Operations: `DeleteConnection`, `DeleteHost`, `DeleteRepositoryLink`, `DeleteSyncConfiguration`
- Common required input members in this group: `ConnectionArn`, `HostArn`, `RepositoryLinkId`, `ResourceName`, `SyncType`

### Update

- Operations: `UpdateHost`, `UpdateRepositoryLink`, `UpdateSyncBlocker`, `UpdateSyncConfiguration`
- Common required input members in this group: `HostArn`, `Id`, `RepositoryLinkId`, `ResolvedReason`, `ResourceName`, `SyncType`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateConnection` | - | - | `ConnectionName` | - | `CreateConnectionOutput` | `LimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException` | Creates a connection that can then be given to other Amazon Web Services services like CodePipeline so that it can access third-party code repositories. The connection is in pending status until the third-party connection handshake is completed from the... |
| `CreateHost` | - | - | `Name`, `ProviderEndpoint`, `ProviderType` | - | `CreateHostOutput` | `LimitExceededException` | Creates a resource that represents the infrastructure where a third-party provider is installed. The host is used when you create connections to an installed third-party provider type, such as GitHub Enterprise Server. |
| `CreateRepositoryLink` | - | - | `ConnectionArn`, `OwnerId`, `RepositoryName` | - | `CreateRepositoryLinkOutput` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Creates a link to a specified external Git repository. A repository link allows Git sync to monitor and sync changes to files in a specified Git repository. |
| `CreateSyncConfiguration` | - | - | `Branch`, `ConfigFile`, `RepositoryLinkId`, `ResourceName`, `RoleArn`, `SyncType` | - | `CreateSyncConfigurationOutput` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Creates a sync configuration which allows Amazon Web Services to sync content from a Git repository to update a specified Amazon Web Services resource. Parameters for the sync configuration are determined by the sync type. |
| `DeleteConnection` | - | - | `ConnectionArn` | - | `DeleteConnectionOutput` | `ResourceNotFoundException` | The connection to be deleted. |
| `DeleteHost` | - | - | `HostArn` | - | `DeleteHostOutput` | `ResourceNotFoundException`, `ResourceUnavailableException` | The host to be deleted. Before you delete a host, all connections associated to the host must be deleted. |
| `DeleteRepositoryLink` | - | - | `RepositoryLinkId` | - | `DeleteRepositoryLinkOutput` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `SyncConfigurationStillExistsException`, `ThrottlingException`, `UnsupportedProviderTypeException` | Deletes the association between your connection and a specified external Git repository. |
| `DeleteSyncConfiguration` | - | - | `ResourceName`, `SyncType` | - | `DeleteSyncConfigurationOutput` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `InvalidInputException`, `LimitExceededException`, `ThrottlingException` | Deletes the sync configuration for a specified repository and connection. |
| `GetConnection` | - | - | `ConnectionArn` | - | `GetConnectionOutput` | `ResourceNotFoundException`, `ResourceUnavailableException` | Returns the connection ARN and details such as status, owner, and provider type. |
| `GetHost` | - | - | `HostArn` | - | `GetHostOutput` | `ResourceNotFoundException`, `ResourceUnavailableException` | Returns the host ARN and details such as status, provider type, endpoint, and, if applicable, the VPC configuration. |
| `GetRepositoryLink` | - | - | `RepositoryLinkId` | - | `GetRepositoryLinkOutput` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Returns details about a repository link. A repository link allows Git sync to monitor and sync changes from files in a specified Git repository. |
| `GetRepositorySyncStatus` | - | - | `Branch`, `RepositoryLinkId`, `SyncType` | - | `GetRepositorySyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Returns details about the sync status for a repository. A repository sync uses Git sync to push and pull changes from your remote repository. |
| `GetResourceSyncStatus` | - | - | `ResourceName`, `SyncType` | - | `GetResourceSyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the status of the sync with the Git repository for a specific Amazon Web Services resource. |
| `GetSyncBlockerSummary` | - | - | `ResourceName`, `SyncType` | - | `GetSyncBlockerSummaryOutput` | `AccessDeniedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of the most recent sync blockers. |
| `GetSyncConfiguration` | - | - | `ResourceName`, `SyncType` | - | `GetSyncConfigurationOutput` | `AccessDeniedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Returns details about a sync configuration, including the sync type and resource name. A sync configuration allows the configuration to sync (push and pull) changes from the remote repository for a specified branch in a Git repository. |
| `ListConnections` | - | `paginated` | - | - | `ListConnectionsOutput` | `ResourceNotFoundException` | Lists the connections associated with your account. |
| `ListHosts` | - | `paginated` | - | - | `ListHostsOutput` | - | Lists the hosts associated with your account. |
| `ListRepositoryLinks` | - | `paginated` | - | - | `ListRepositoryLinksOutput` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the repository links created for connections in your account. |
| `ListRepositorySyncDefinitions` | - | - | `RepositoryLinkId`, `SyncType` | - | `ListRepositorySyncDefinitionsOutput` | `AccessDeniedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the repository sync definitions for repository links in your account. |
| `ListSyncConfigurations` | - | `paginated` | `RepositoryLinkId`, `SyncType` | - | `ListSyncConfigurationsOutput` | `AccessDeniedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of sync configurations for a specified repository. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceOutput` | `ResourceNotFoundException` | Gets the set of key-value pairs (metadata) that are used to manage the resource. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `LimitExceededException`, `ResourceNotFoundException` | Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `ResourceNotFoundException` | Removes tags from an Amazon Web Services resource. |
| `UpdateHost` | - | - | `HostArn` | - | `UpdateHostOutput` | `ConflictException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `UnsupportedOperationException` | Updates a specified host with the provided configurations. |
| `UpdateRepositoryLink` | - | - | `RepositoryLinkId` | - | `UpdateRepositoryLinkOutput` | `AccessDeniedException`, `ConditionalCheckFailedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException`, `UpdateOutOfSyncException` | Updates the association between your connection and a specified external Git repository. A repository link allows Git sync to monitor and sync changes to files in a specified Git repository. |
| `UpdateSyncBlocker` | - | - | `Id`, `ResolvedReason`, `ResourceName`, `SyncType` | - | `UpdateSyncBlockerOutput` | `AccessDeniedException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `RetryLatestCommitFailedException`, `SyncBlockerDoesNotExistException`, `ThrottlingException` | Allows you to update the status of a sync blocker, resolving the blocker and allowing syncing to continue. |
| `UpdateSyncConfiguration` | - | - | `ResourceName`, `SyncType` | - | `UpdateSyncConfigurationOutput` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException`, `ThrottlingException`, `UpdateOutOfSyncException` | Updates the sync configuration for your connection and a specified external Git repository. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `Message` | Resource not found. |
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | Received an internal server exception. |
| `InvalidInputException` | `structure` | `Message` | The input is not valid. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ConcurrentModificationException` | `structure` | `Message` | Exception thrown as a result of concurrent modification to an application. |
| `LimitExceededException` | `structure` | `Message` | Exceeded the maximum limit for connections. |
| `ResourceUnavailableException` | `structure` | `Message` | Resource not found. |
| `ResourceAlreadyExistsException` | `structure` | `Message` | Unable to create resource. |
| `UpdateOutOfSyncException` | `structure` | `Message` | The update is out of sync. |
| `CreateConnectionInput` | `structure` | `ConnectionName`, `HostArn`, `ProviderType`, `Tags` | - |
| `CreateConnectionOutput` | `structure` | `ConnectionArn`, `Tags` | - |
| `CreateHostInput` | `structure` | `Name`, `ProviderEndpoint`, `ProviderType`, `Tags`, `VpcConfiguration` | - |
| `CreateHostOutput` | `structure` | `HostArn`, `Tags` | - |
| `CreateRepositoryLinkInput` | `structure` | `ConnectionArn`, `EncryptionKeyArn`, `OwnerId`, `RepositoryName`, `Tags` | - |
| `CreateRepositoryLinkOutput` | `structure` | `RepositoryLinkInfo` | - |
| `CreateSyncConfigurationInput` | `structure` | `Branch`, `ConfigFile`, `PublishDeploymentStatus`, `PullRequestComment`, `RepositoryLinkId`, `ResourceName`, `RoleArn`, `SyncType`, `TriggerResourceUpdateOn` | - |
| `CreateSyncConfigurationOutput` | `structure` | `SyncConfiguration` | - |
| `DeleteConnectionInput` | `structure` | `ConnectionArn` | - |
| `DeleteConnectionOutput` | `structure` | - | - |
| `DeleteHostInput` | `structure` | `HostArn` | - |
| `DeleteHostOutput` | `structure` | - | - |
| `DeleteRepositoryLinkInput` | `structure` | `RepositoryLinkId` | - |
| `DeleteRepositoryLinkOutput` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
