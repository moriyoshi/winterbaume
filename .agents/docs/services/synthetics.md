# Synthetics

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudWatch Synthetics You can use Amazon CloudWatch Synthetics to continually monitor your services. You can create and manage canaries , which are modular, lightweight scripts that monitor your endpoints and APIs from the outside-in. You can set up your canaries to run 24 hours a day, once per minute. The canaries help you check the availability and latency of your web services and troubleshoot anomalies by investigating load time data, screenshots of the UI, logs, and metrics. The canaries seamlessly integrate with CloudWatch ServiceLens to help you trace the causes of impacted nodes in your applications.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Synthetics where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Synthetics by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Synthetics workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Get`, `Create`, `Delete` operation families, including `ListAssociatedGroups`, `ListGroupResources`, `ListGroups`, `ListTagsForResource`, `DescribeCanaries`, `DescribeCanariesLastRun`.

## Service Identity and Protocol

- AWS model slug: `synthetics`
- AWS SDK for Rust slug: `synthetics`
- Model version: `2017-10-11`
- Model file: `vendor/api-models-aws/models/synthetics/service/2017-10-11/synthetics-2017-10-11.json`
- SDK ID: `synthetics`
- Endpoint prefix: `synthetics`
- ARN namespace: `synthetics`
- CloudFormation name: `Synthetics`
- CloudTrail event source: `synthetics.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Describe` (3), `Get` (3), `Create` (2), `Delete` (2), `Start` (2), `Associate` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateResource`, `CreateCanary`, `CreateGroup`, `DeleteCanary`, `DeleteGroup`, `DisassociateResource`, `StartCanary`, `StartCanaryDryRun`, `StopCanary`, `TagResource`, `UntagResource`, `UpdateCanary`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCanaries`, `DescribeCanariesLastRun`, `DescribeRuntimeVersions`, `GetCanary`, `GetCanaryRuns`, `GetGroup`, `ListAssociatedGroups`, `ListGroupResources`, `ListGroups`, `ListTagsForResource`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartCanary`, `StartCanaryDryRun`, `StopCanary`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 22 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Lambda`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Current Network Resource Stub Semantics

Synthetics currently has a placeholder for canary VPC configuration.

- Canary views include an optional `vpc_config` JSON slot, and current snapshot construction sets it to `None`.
- Canary create/update/read paths do not allocate ENIs or retain subnet and security group metadata in the implemented state.
- Run status is independent of VPC reachability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListAssociatedGroups`, `ListGroupResources`, `ListGroups`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: `ResourceArn`

### Describe

- Operations: `DescribeCanaries`, `DescribeCanariesLastRun`, `DescribeRuntimeVersions`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Get

- Operations: `GetCanary`, `GetCanaryRuns`, `GetGroup`
- Traits: `paginated` (1)
- Common required input members in this group: `Name`

### Create

- Operations: `CreateCanary`, `CreateGroup`
- Common required input members in this group: `Name`

### Delete

- Operations: `DeleteCanary`, `DeleteGroup`
- Common required input members in this group: -

### Start

- Operations: `StartCanary`, `StartCanaryDryRun`
- Common required input members in this group: `Name`

### Associate

- Operations: `AssociateResource`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateResource`
- Common required input members in this group: -

### Stop

- Operations: `StopCanary`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateCanary`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateResource` | `PATCH /group/{GroupIdentifier}/associate` | - | `GroupIdentifier`, `ResourceArn` | - | `AssociateResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Associates a canary with a group. Using groups can help you with managing and automating your canaries, and you can also view aggregated run results and statistics for all canaries in a group. You must run this opera ... |
| `CreateCanary` | `POST /canary` | - | `Name`, `Code`, `ArtifactS3Location`, `ExecutionRoleArn`, `Schedule`, `RuntimeVersion` | - | `CreateCanaryResponse` | `InternalServerException`, `RequestEntityTooLargeException`, `ValidationException` | Creates a canary. Canaries are scripts that monitor your endpoints and APIs from the outside-in. Canaries help you check the availability and latency of your web services and troubleshoot anomalies by investigating l ... |
| `CreateGroup` | `POST /group` | - | `Name` | - | `CreateGroupResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a group which you can use to associate canaries with each other, including cross-Region canaries. Using groups can help you with managing and automating your canaries, and you can also view aggregated run res ... |
| `DeleteCanary` | `DELETE /canary/{Name}` | - | `Name` | - | `DeleteCanaryResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Permanently deletes the specified canary. If the canary's ProvisionedResourceCleanup field is set to AUTOMATIC or you specify DeleteLambda in this operation as true , CloudWatch Synthetics also deletes the Lambda fun ... |
| `DeleteGroup` | `DELETE /group/{GroupIdentifier}` | - | `GroupIdentifier` | - | `DeleteGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a group. The group doesn't need to be empty to be deleted. If there are canaries in the group, they are not deleted when you delete the group. Groups are a global resource that appear in all Regions, but the ... |
| `DescribeCanaries` | `POST /canaries` | `paginated` | - | - | `DescribeCanariesResponse` | `InternalServerException`, `ValidationException` | This operation returns a list of the canaries in your account, along with full details about each canary. This operation supports resource-level authorization using an IAM policy and the Names parameter. If you speci ... |
| `DescribeCanariesLastRun` | `POST /canaries/last-run` | `paginated` | - | - | `DescribeCanariesLastRunResponse` | `InternalServerException`, `ValidationException` | Use this operation to see information from the most recent run of each canary that you have created. This operation supports resource-level authorization using an IAM policy and the Names parameter. If you specify th ... |
| `DescribeRuntimeVersions` | `POST /runtime-versions` | `paginated` | - | - | `DescribeRuntimeVersionsResponse` | `InternalServerException`, `ValidationException` | Returns a list of Synthetics canary runtime versions. For more information, see Canary Runtime Versions . |
| `DisassociateResource` | `PATCH /group/{GroupIdentifier}/disassociate` | - | `GroupIdentifier`, `ResourceArn` | - | `DisassociateResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a canary from a group. You must run this operation in the Region where the canary exists. |
| `GetCanary` | `GET /canary/{Name}` | - | `Name` | - | `GetCanaryResponse` | `InternalServerException`, `ValidationException` | Retrieves complete information about one canary. You must specify the name of the canary that you want. To get a list of canaries and their names, use DescribeCanaries . |
| `GetCanaryRuns` | `POST /canary/{Name}/runs` | `paginated` | `Name` | - | `GetCanaryRunsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves a list of runs for a specified canary. |
| `GetGroup` | `GET /group/{GroupIdentifier}` | - | `GroupIdentifier` | - | `GetGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about one group. Groups are a global resource, so you can use this operation from any Region. |
| `ListAssociatedGroups` | `POST /resource/{ResourceArn}/groups` | `paginated` | `ResourceArn` | - | `ListAssociatedGroupsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of the groups that the specified canary is associated with. The canary that you specify must be in the current Region. |
| `ListGroupResources` | `POST /group/{GroupIdentifier}/resources` | `paginated` | `GroupIdentifier` | - | `ListGroupResourcesResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | This operation returns a list of the ARNs of the canaries that are associated with the specified group. |
| `ListGroups` | `POST /groups` | `paginated` | - | - | `ListGroupsResponse` | `InternalServerException`, `ValidationException` | Returns a list of all groups in the account, displaying their names, unique IDs, and ARNs. The groups from all Regions are returned. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `NotFoundException`, `TooManyRequestsException` | Displays the tags associated with a canary or group. |
| `StartCanary` | `POST /canary/{Name}/start` | - | `Name` | - | `StartCanaryResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Use this operation to run a canary that has already been created. The frequency of the canary runs is determined by the value of the canary's Schedule . To see a canary's schedule, use GetCanary . |
| `StartCanaryDryRun` | `POST /canary/{Name}/dry-run/start` | - | `Name` | - | `StartCanaryDryRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Use this operation to start a dry run for a canary that has already been created |
| `StopCanary` | `POST /canary/{Name}/stop` | - | `Name` | - | `StopCanaryResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Stops the canary to prevent all future runs. If the canary is currently running,the run that is in progress completes on its own, publishes metrics, and uploads artifacts, but it is not recorded in Synthetics as a co ... |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `NotFoundException`, `TooManyRequestsException` | Assigns one or more tags (key-value pairs) to the specified canary or group. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `NotFoundException`, `TooManyRequestsException` | Removes one or more tags from the specified resource. |
| `UpdateCanary` | `PATCH /canary/{Name}` | - | `Name` | - | `UpdateCanaryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `RequestEntityTooLargeException`, `ResourceNotFoundException`, `ValidationException` | Updates the configuration of a canary that has already been created. For multibrowser canaries, you can add or remove browsers by updating the browserConfig list in the update call. For example: To add Firefox to a c ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteCanary` | - | `DeleteLambda -> deleteLambda` | - | - |
| `GetCanary` | - | `DryRunId -> dryRunId` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You don't have permission to perform this operation on this resource. |
| `BadRequestException` | `structure` | Message | The request was not valid. |
| `ConflictException` | `structure` | Message | A conflicting operation is already in progress. |
| `InternalFailureException` | `structure` | Message | An internal failure occurred. Try the operation again. |
| `InternalServerException` | `structure` | Message | An unknown internal error occurred. |
| `NotFoundException` | `structure` | Message | The specified resource was not found. |
| `RequestEntityTooLargeException` | `structure` | Message | One of the input resources is larger than is allowed. |
| `ResourceNotFoundException` | `structure` | Message | One of the specified resources was not found. |
| `ServiceQuotaExceededException` | `structure` | Message | The request exceeded a service quota value. |
| `TooManyRequestsException` | `structure` | Message | There were too many simultaneous requests. Try the operation again. |
| `ValidationException` | `structure` | Message | A parameter could not be validated. |
| `AssociateResourceRequest` | `structure` | GroupIdentifier, ResourceArn | - |
| `AssociateResourceResponse` | `structure` | **empty (no members)** | - |
| `CreateCanaryRequest` | `structure` | Name, Code, ArtifactS3Location, ExecutionRoleArn, Schedule, RunConfig, SuccessRetentionPeriodInDays, FailureRetentionPeriodInDays, RuntimeVersion, VpcConfig, ResourcesToReplicateTags, ProvisionedResourceCleanup, ... (+3) | - |
| `CreateCanaryResponse` | `structure` | Canary | - |
| `CreateGroupRequest` | `structure` | Name, Tags | - |
| `CreateGroupResponse` | `structure` | Group | - |
| `DeleteCanaryRequest` | `structure` | Name, DeleteLambda | - |
| `DeleteCanaryResponse` | `structure` | **empty (no members)** | - |
| `DeleteGroupRequest` | `structure` | GroupIdentifier | - |
| `DeleteGroupResponse` | `structure` | **empty (no members)** | - |
| `DescribeCanariesRequest` | `structure` | NextToken, MaxResults, Names | - |
| `DescribeCanariesResponse` | `structure` | Canaries, NextToken | - |
| `DescribeCanariesLastRunRequest` | `structure` | NextToken, MaxResults, Names, BrowserType | - |
| `DescribeCanariesLastRunResponse` | `structure` | CanariesLastRun, NextToken | - |
| `DescribeRuntimeVersionsRequest` | `structure` | NextToken, MaxResults | - |
| `DescribeRuntimeVersionsResponse` | `structure` | RuntimeVersions, NextToken | - |
| `DisassociateResourceRequest` | `structure` | GroupIdentifier, ResourceArn | - |
| `DisassociateResourceResponse` | `structure` | **empty (no members)** | - |
| `GetCanaryRequest` | `structure` | Name, DryRunId | - |
| `GetCanaryResponse` | `structure` | Canary | - |
| `GetCanaryRunsRequest` | `structure` | Name, NextToken, MaxResults, DryRunId, RunType | - |
| `GetCanaryRunsResponse` | `structure` | CanaryRuns, NextToken | - |
| `GetGroupRequest` | `structure` | GroupIdentifier | - |
| `GetGroupResponse` | `structure` | Group | - |
| `ListAssociatedGroupsRequest` | `structure` | NextToken, MaxResults, ResourceArn | - |
| `ListAssociatedGroupsResponse` | `structure` | Groups, NextToken | - |
| `ListGroupResourcesRequest` | `structure` | NextToken, MaxResults, GroupIdentifier | - |
| `ListGroupResourcesResponse` | `structure` | Resources, NextToken | - |
| `ListGroupsRequest` | `structure` | NextToken, MaxResults | - |
| `BrowserType` | `enum` | CHROME, FIREFOX | - |
| `CanaryRunState` | `enum` | RUNNING, PASSED, FAILED | - |
| `CanaryRunStateReasonCode` | `enum` | CANARY_FAILURE, EXECUTION_FAILURE | - |
| `CanaryRunTestResult` | `enum` | PASSED, FAILED, UNKNOWN | - |
| `CanaryState` | `enum` | CREATING, READY, STARTING, RUNNING, UPDATING, STOPPING, STOPPED, ERROR, DELETING | - |
| `CanaryStateReasonCode` | `enum` | INVALID_PERMISSIONS, CREATE_PENDING, CREATE_IN_PROGRESS, CREATE_FAILED, UPDATE_PENDING, UPDATE_IN_PROGRESS, UPDATE_COMPLETE, ROLLBACK_COMPLETE, ROLLBACK_FAILED, DELETE_IN_PROGRESS, DELETE_FAILED, SYNC_DELETE_IN_PROGRESS | - |
| `DependencyType` | `enum` | LambdaLayer | - |
| `EncryptionMode` | `enum` | SSE_S3, SSE_KMS | - |
| `ProvisionedResourceCleanupSetting` | `enum` | AUTOMATIC, OFF | - |
| `ResourceToTag` | `enum` | LAMBDA_FUNCTION | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
