# AWS Migration Hub

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The AWS Migration Hub API methods help to obtain server and application migration status and integrate your resource-specific migration tool by providing a programmatic interface to Migration Hub. Remember that you must set your AWS Migration Hub home region before you call any of these APIs, or a `HomeRegionNotSetException` error will be returned. Also, you must make the API calls while in your home region.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Migration Hub where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Migration Hub by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Migration Hub workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Associate`, `Disassociate`, `Describe`, `Notify` operation families, including `ListApplicationStates`, `ListCreatedArtifacts`, `ListDiscoveredResources`, `ListMigrationTaskUpdates`, `AssociateCreatedArtifact`, `AssociateDiscoveredResource`.

## Service Identity and Protocol

- AWS model slug: `migration-hub`
- AWS SDK for Rust slug: `migrationhub`
- Model version: `2017-05-31`
- Model file: `vendor/api-models-aws/models/migration-hub/service/2017-05-31/migration-hub-2017-05-31.json`
- SDK ID: `Migration Hub`
- Endpoint prefix: `mgh`
- ARN namespace: `mgh`
- CloudFormation name: `MigrationHub`
- CloudTrail event source: `migrationhub.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Associate` (3), `Disassociate` (3), `Describe` (2), `Notify` (2), `Create` (1), `Delete` (1), `Import` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateCreatedArtifact`, `AssociateDiscoveredResource`, `AssociateSourceResource`, `CreateProgressUpdateStream`, `DeleteProgressUpdateStream`, `DisassociateCreatedArtifact`, `DisassociateDiscoveredResource`, `DisassociateSourceResource`, `ImportMigrationTask`, `PutResourceAttributes`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApplicationState`, `DescribeMigrationTask`, `ListApplicationStates`, `ListCreatedArtifacts`, `ListDiscoveredResources`, `ListMigrationTaskUpdates`, `ListMigrationTasks`, `ListProgressUpdateStreams`, `ListSourceResources`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeMigrationTask`, `ImportMigrationTask`, `ListMigrationTaskUpdates`, `ListMigrationTasks`, `NotifyMigrationTaskState`.
- 21 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECR`, `ECS`, `RDS`.

## Operation Groups

### List

- Operations: `ListApplicationStates`, `ListCreatedArtifacts`, `ListDiscoveredResources`, `ListMigrationTasks`, `ListMigrationTaskUpdates`, `ListProgressUpdateStreams`, `ListSourceResources`
- Traits: `paginated` (7)
- Common required input members in this group: `ProgressUpdateStream`, `MigrationTaskName`

### Associate

- Operations: `AssociateCreatedArtifact`, `AssociateDiscoveredResource`, `AssociateSourceResource`
- Common required input members in this group: `ProgressUpdateStream`, `MigrationTaskName`

### Disassociate

- Operations: `DisassociateCreatedArtifact`, `DisassociateDiscoveredResource`, `DisassociateSourceResource`
- Common required input members in this group: `ProgressUpdateStream`, `MigrationTaskName`

### Describe

- Operations: `DescribeApplicationState`, `DescribeMigrationTask`
- Common required input members in this group: -

### Notify

- Operations: `NotifyApplicationState`, `NotifyMigrationTaskState`
- Common required input members in this group: -

### Create

- Operations: `CreateProgressUpdateStream`
- Common required input members in this group: -

### Delete

- Operations: `DeleteProgressUpdateStream`
- Common required input members in this group: -

### Import

- Operations: `ImportMigrationTask`
- Common required input members in this group: -

### Put

- Operations: `PutResourceAttributes`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateCreatedArtifact` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `CreatedArtifact` | - | `AssociateCreatedArtifactResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Associates a created artifact of an AWS cloud resource, the target receiving the migration, with the migration task performed by a migration tool. This API has the following traits: Migration tools can call the Assoc ... |
| `AssociateDiscoveredResource` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `DiscoveredResource` | - | `AssociateDiscoveredResourceResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `PolicyErrorException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Associates a discovered resource ID from Application Discovery Service with a migration task. |
| `AssociateSourceResource` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `SourceResource` | - | `AssociateSourceResourceResult` | `AccessDeniedException`, `DryRunOperation`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Associates a source resource with a migration task. For example, the source resource can be a source server, an application, or a migration wave. |
| `CreateProgressUpdateStream` | `-` | - | `ProgressUpdateStreamName` | - | `CreateProgressUpdateStreamResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Creates a progress update stream which is an AWS resource used for access control as well as a namespace for migration task names that is implicitly linked to your AWS account. It must uniquely identify the migration ... |
| `DeleteProgressUpdateStream` | `-` | - | `ProgressUpdateStreamName` | - | `DeleteProgressUpdateStreamResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Deletes a progress update stream, including all of its tasks, which was previously created as an AWS resource used for access control. This API has the following traits: The only parameter needed for DeleteProgressUp ... |
| `DescribeApplicationState` | `-` | - | `ApplicationId` | - | `DescribeApplicationStateResult` | `AccessDeniedException`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `PolicyErrorException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Gets the migration status of an application. |
| `DescribeMigrationTask` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName` | - | `DescribeMigrationTaskResult` | `AccessDeniedException`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves a list of all attributes associated with a specific migration task. |
| `DisassociateCreatedArtifact` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `CreatedArtifactName` | - | `DisassociateCreatedArtifactResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Disassociates a created artifact of an AWS resource with a migration task performed by a migration tool that was previously associated. This API has the following traits: A migration user can call the DisassociateCre ... |
| `DisassociateDiscoveredResource` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `ConfigurationId` | - | `DisassociateDiscoveredResourceResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Disassociate an Application Discovery Service discovered resource from a migration task. |
| `DisassociateSourceResource` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `SourceResourceName` | - | `DisassociateSourceResourceResult` | `AccessDeniedException`, `DryRunOperation`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Removes the association between a source resource and a migration task. |
| `ImportMigrationTask` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName` | - | `ImportMigrationTaskResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Registers a new migration task which represents a server, database, etc., being migrated to AWS by a migration tool. This API is a prerequisite to calling the NotifyMigrationTaskState API as the migration tool must f ... |
| `ListApplicationStates` | `-` | `paginated` | - | - | `ListApplicationStatesResult` | `AccessDeniedException`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ServiceUnavailableException`, `ThrottlingException` | Lists all the migration statuses for your applications. If you use the optional ApplicationIds parameter, only the migration statuses for those applications will be returned. |
| `ListCreatedArtifacts` | `-` | `paginated` | `ProgressUpdateStream`, `MigrationTaskName` | - | `ListCreatedArtifactsResult` | `AccessDeniedException`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the created artifacts attached to a given migration task in an update stream. This API has the following traits: Gets the list of the created artifacts while migration is taking place. Shows the artifacts creat ... |
| `ListDiscoveredResources` | `-` | `paginated` | `ProgressUpdateStream`, `MigrationTaskName` | - | `ListDiscoveredResourcesResult` | `AccessDeniedException`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists discovered resources associated with the given MigrationTask . |
| `ListMigrationTasks` | `-` | `paginated` | - | - | `ListMigrationTasksResult` | `AccessDeniedException`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `PolicyErrorException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists all, or filtered by resource name, migration tasks associated with the user account making this call. This API has the following traits: Can show a summary list of the most recent migration tasks. Can show a su ... |
| `ListMigrationTaskUpdates` | `-` | `paginated` | `ProgressUpdateStream`, `MigrationTaskName` | - | `ListMigrationTaskUpdatesResult` | `AccessDeniedException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | This is a paginated API that returns all the migration-task states for the specified MigrationTaskName and ProgressUpdateStream . |
| `ListProgressUpdateStreams` | `-` | `paginated` | - | - | `ListProgressUpdateStreamsResult` | `AccessDeniedException`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ServiceUnavailableException`, `ThrottlingException` | Lists progress update streams associated with the user account making this call. |
| `ListSourceResources` | `-` | `paginated` | `ProgressUpdateStream`, `MigrationTaskName` | - | `ListSourceResourcesResult` | `AccessDeniedException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists all the source resource that are associated with the specified MigrationTaskName and ProgressUpdateStream . |
| `NotifyApplicationState` | `-` | - | `ApplicationId`, `Status` | - | `NotifyApplicationStateResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `PolicyErrorException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Sets the migration state of an application. For a given application identified by the value passed to ApplicationId , its status is set or updated by passing one of three values to Status : NOT_STARTED | IN_PROGRESS ... |
| `NotifyMigrationTaskState` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `Task`, `UpdateDateTime`, `NextUpdateSeconds` | - | `NotifyMigrationTaskStateResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Notifies Migration Hub of the current status, progress, or other detail regarding a migration task. This API has the following traits: Migration tools will call the NotifyMigrationTaskState API to share the latest pr ... |
| `PutResourceAttributes` | `-` | - | `ProgressUpdateStream`, `MigrationTaskName`, `ResourceAttributeList` | - | `PutResourceAttributesResult` | `AccessDeniedException`, `DryRunOperation`, `HomeRegionNotSetException`, `InternalServerError`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedOperation` | Provides identifying details of the resource being migrated so that it can be associated in the Application Discovery Service repository. This association occurs asynchronously after PutResourceAttributes returns. Ke ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `DryRunOperation` | `structure` | Message | Exception raised to indicate a successfully authorized action when the DryRun flag is set to "true". |
| `HomeRegionNotSetException` | `structure` | Message | The home region is not set. Set the home region to continue. |
| `InternalServerError` | `structure` | Message | Exception raised when an internal, configuration, or dependency error is encountered. |
| `InvalidInputException` | `structure` | Message | Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type. |
| `PolicyErrorException` | `structure` | Message | Exception raised when there are problems accessing Application Discovery Service (Application Discovery Service); most likely due to a misconfigured policy ... |
| `ResourceNotFoundException` | `structure` | Message | Exception raised when the request references a resource (Application Discovery Service configuration, update stream, migration task, etc.) that does not exi ... |
| `ServiceUnavailableException` | `structure` | Message | Exception raised when there is an internal, configuration, or dependency error encountered. |
| `ThrottlingException` | `structure` | Message, RetryAfterSeconds | The request was denied due to request throttling. |
| `UnauthorizedOperation` | `structure` | Message | Exception raised to indicate a request was not authorized when the DryRun flag is set to "true". |
| `AssociateCreatedArtifactRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, CreatedArtifact, DryRun | - |
| `AssociateCreatedArtifactResult` | `structure` | **empty (no members)** | - |
| `AssociateDiscoveredResourceRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, DiscoveredResource, DryRun | - |
| `AssociateDiscoveredResourceResult` | `structure` | **empty (no members)** | - |
| `AssociateSourceResourceRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, SourceResource, DryRun | - |
| `AssociateSourceResourceResult` | `structure` | **empty (no members)** | - |
| `CreateProgressUpdateStreamRequest` | `structure` | ProgressUpdateStreamName, DryRun | - |
| `CreateProgressUpdateStreamResult` | `structure` | **empty (no members)** | - |
| `DeleteProgressUpdateStreamRequest` | `structure` | ProgressUpdateStreamName, DryRun | - |
| `DeleteProgressUpdateStreamResult` | `structure` | **empty (no members)** | - |
| `DescribeApplicationStateRequest` | `structure` | ApplicationId | - |
| `DescribeApplicationStateResult` | `structure` | ApplicationStatus, LastUpdatedTime | - |
| `DescribeMigrationTaskRequest` | `structure` | ProgressUpdateStream, MigrationTaskName | - |
| `DescribeMigrationTaskResult` | `structure` | MigrationTask | - |
| `DisassociateCreatedArtifactRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, CreatedArtifactName, DryRun | - |
| `DisassociateCreatedArtifactResult` | `structure` | **empty (no members)** | - |
| `DisassociateDiscoveredResourceRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, ConfigurationId, DryRun | - |
| `DisassociateDiscoveredResourceResult` | `structure` | **empty (no members)** | - |
| `DisassociateSourceResourceRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, SourceResourceName, DryRun | - |
| `DisassociateSourceResourceResult` | `structure` | **empty (no members)** | - |
| `ImportMigrationTaskRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, DryRun | - |
| `ImportMigrationTaskResult` | `structure` | **empty (no members)** | - |
| `ListApplicationStatesRequest` | `structure` | ApplicationIds, NextToken, MaxResults | - |
| `ListApplicationStatesResult` | `structure` | ApplicationStateList, NextToken | - |
| `ListCreatedArtifactsRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, NextToken, MaxResults | - |
| `ListCreatedArtifactsResult` | `structure` | NextToken, CreatedArtifactList | - |
| `ListDiscoveredResourcesRequest` | `structure` | ProgressUpdateStream, MigrationTaskName, NextToken, MaxResults | - |
| `ListDiscoveredResourcesResult` | `structure` | NextToken, DiscoveredResourceList | - |
| `ListMigrationTasksRequest` | `structure` | NextToken, MaxResults, ResourceName | - |
| `ListMigrationTasksResult` | `structure` | NextToken, MigrationTaskSummaryList | - |
| `ApplicationStatus` | `enum` | NOT_STARTED, IN_PROGRESS, COMPLETED | - |
| `ResourceAttributeType` | `enum` | IPV4_ADDRESS, IPV6_ADDRESS, MAC_ADDRESS, FQDN, VM_MANAGER_ID, VM_MANAGED_OBJECT_REFERENCE, VM_NAME, VM_PATH, BIOS_ID, MOTHERBOARD_SERIAL_NUMBER | - |
| `Status` | `enum` | NOT_STARTED, IN_PROGRESS, FAILED, COMPLETED | - |
| `UpdateType` | `enum` | MigrationTaskStateUpdated | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
