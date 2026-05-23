# AWS Cloud9

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Cloud9 Cloud9 is a collection of tools that you can use to code, build, run, test, debug, and release software in the cloud. For more information about Cloud9, see the Cloud9 User Guide. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" Cloud9 supports these operations: `CreateEnvironmentEC2`: Creates an Cloud9 development environment, launches an Amazon EC2 instance, and then connects from the instance to the environment.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Cloud9 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Cloud9 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Cloud9 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Delete`, `List`, `Update` operation families, including `DescribeEnvironmentMemberships`, `DescribeEnvironmentStatus`, `DescribeEnvironments`, `CreateEnvironmentEC2`, `CreateEnvironmentMembership`, `DeleteEnvironment`.

## Service Identity and Protocol

- AWS model slug: `cloud9`
- AWS SDK for Rust slug: `cloud9`
- Model version: `2017-09-23`
- Model file: `vendor/api-models-aws/models/cloud9/service/2017-09-23/cloud9-2017-09-23.json`
- SDK ID: `Cloud9`
- Endpoint prefix: `cloud9`
- ARN namespace: `cloud9`
- CloudFormation name: `Cloud9`
- CloudTrail event source: `cloud9.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (3), `Create` (2), `Delete` (2), `List` (2), `Update` (2), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateEnvironmentEC2`, `CreateEnvironmentMembership`, `DeleteEnvironment`, `DeleteEnvironmentMembership`, `TagResource`, `UntagResource`, `UpdateEnvironment`, `UpdateEnvironmentMembership`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeEnvironmentMemberships`, `DescribeEnvironmentStatus`, `DescribeEnvironments`, `ListEnvironments`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cloud9/latest/user-guide/welcome.html
- https://docs.aws.amazon.com/cloud9/latest/user-guide/create-environment-ssh.html
- https://docs.aws.amazon.com/cloud9/latest/user-guide/troubleshooting.html

Research outcomes:
- Cloud9 provides browser-based IDE environments backed by either managed EC2 instances or existing SSH-accessible compute.
- EC2 environments can be created and managed by Cloud9, while SSH environments connect to external hosts with public key authentication.
- Environments support coding, building, testing, running, debugging, terminal access, and collaboration.
- SSH environments require prerequisites such as reachable host, SSH user, Node.js path, and authorised public key setup.
- Environment connection and IDE performance depend on backing compute/network health.

Parity implications:
- Model environments, environment type, EC2 backing instance, SSH connection metadata, membership/permissions, and connection status separately.
- CreateEnvironment should branch between EC2-managed provisioning and SSH metadata registration.
- IDE session availability should depend on backing environment reachability.

## Operation Groups

### Describe

- Operations: `DescribeEnvironmentMemberships`, `DescribeEnvironments`, `DescribeEnvironmentStatus`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateEnvironmentEC2`, `CreateEnvironmentMembership`
- Traits: `idempotent` (2)
- Common required input members in this group: -

### Delete

- Operations: `DeleteEnvironment`, `DeleteEnvironmentMembership`
- Traits: `idempotent` (2)
- Common required input members in this group: `environmentId`

### List

- Operations: `ListEnvironments`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateEnvironment`, `UpdateEnvironmentMembership`
- Traits: `idempotent` (2)
- Common required input members in this group: `environmentId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEnvironmentEC2` | `-` | `idempotent` | `name`, `instanceType`, `imageId` | - | `CreateEnvironmentEC2Result` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Creates an Cloud9 development environment, launches an Amazon Elastic Compute Cloud (Amazon EC2) instance, and then connects from the instance to the environment. Cloud9 is no longer available to new customers. Exist ... |
| `CreateEnvironmentMembership` | `-` | `idempotent` | `environmentId`, `userArn`, `permissions` | - | `CreateEnvironmentMembershipResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Adds an environment member to an Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `DeleteEnvironment` | `-` | `idempotent` | `environmentId` | - | `DeleteEnvironmentResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Deletes an Cloud9 development environment. If an Amazon EC2 instance is connected to the environment, also terminates the instance. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can con ... |
| `DeleteEnvironmentMembership` | `-` | `idempotent` | `environmentId`, `userArn` | - | `DeleteEnvironmentMembershipResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Deletes an environment member from a development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `DescribeEnvironmentMemberships` | `-` | `paginated` | - | - | `DescribeEnvironmentMembershipsResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets information about environment members for an Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `DescribeEnvironments` | `-` | - | `environmentIds` | - | `DescribeEnvironmentsResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets information about Cloud9 development environments. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `DescribeEnvironmentStatus` | `-` | - | `environmentId` | - | `DescribeEnvironmentStatusResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets status information for an Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `ListEnvironments` | `-` | `paginated` | - | - | `ListEnvironmentsResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets a list of Cloud9 development environment identifiers. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" Cloud9 is no longer avail ... |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Gets a list of the tags associated with an Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ConcurrentAccessException`, `InternalServerErrorException`, `NotFoundException` | Adds tags to an Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" Tags that you add to an Cloud9 envir ... |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConcurrentAccessException`, `InternalServerErrorException`, `NotFoundException` | Removes tags from an Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `UpdateEnvironment` | `-` | `idempotent` | `environmentId` | - | `UpdateEnvironmentResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Changes the settings of an existing Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Learn more" |
| `UpdateEnvironmentMembership` | `-` | `idempotent` | `environmentId`, `userArn`, `permissions` | - | `UpdateEnvironmentMembershipResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Changes the settings of an existing environment member for an Cloud9 development environment. Cloud9 is no longer available to new customers. Existing customers of Cloud9 can continue to use the service as normal. Le ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | message, className, code | The target request is invalid. |
| `ConcurrentAccessException` | `structure` | message, className, code | A concurrent access issue occurred. |
| `ConflictException` | `structure` | message, className, code | A conflict occurred. |
| `ForbiddenException` | `structure` | message, className, code | An access permissions issue occurred. |
| `InternalServerErrorException` | `structure` | message, className, code | An internal server error occurred. |
| `LimitExceededException` | `structure` | message, className, code | A service limit was exceeded. |
| `NotFoundException` | `structure` | message, className, code | The target resource cannot be found. |
| `TooManyRequestsException` | `structure` | message, className, code | Too many service requests were made over the given time period. |
| `CreateEnvironmentEC2Request` | `structure` | name, description, clientRequestToken, instanceType, subnetId, imageId, automaticStopTimeMinutes, ownerArn, tags, connectionType, dryRun | - |
| `CreateEnvironmentEC2Result` | `structure` | environmentId | - |
| `CreateEnvironmentMembershipRequest` | `structure` | environmentId, userArn, permissions | - |
| `CreateEnvironmentMembershipResult` | `structure` | membership | - |
| `DeleteEnvironmentRequest` | `structure` | environmentId | - |
| `DeleteEnvironmentResult` | `structure` | **empty (no members)** | - |
| `DeleteEnvironmentMembershipRequest` | `structure` | environmentId, userArn | - |
| `DeleteEnvironmentMembershipResult` | `structure` | **empty (no members)** | - |
| `DescribeEnvironmentMembershipsRequest` | `structure` | userArn, environmentId, permissions, nextToken, maxResults | - |
| `DescribeEnvironmentMembershipsResult` | `structure` | memberships, nextToken | - |
| `DescribeEnvironmentsRequest` | `structure` | environmentIds | - |
| `DescribeEnvironmentsResult` | `structure` | environments | - |
| `DescribeEnvironmentStatusRequest` | `structure` | environmentId | - |
| `DescribeEnvironmentStatusResult` | `structure` | status, message | - |
| `ListEnvironmentsRequest` | `structure` | nextToken, maxResults | - |
| `ListEnvironmentsResult` | `structure` | nextToken, environmentIds | - |
| `ListTagsForResourceRequest` | `structure` | ResourceARN | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceARN, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceARN, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateEnvironmentRequest` | `structure` | environmentId, name, description, managedCredentialsAction | - |
| `UpdateEnvironmentResult` | `structure` | **empty (no members)** | - |
| `UpdateEnvironmentMembershipRequest` | `structure` | environmentId, userArn, permissions | - |
| `UpdateEnvironmentMembershipResult` | `structure` | membership | - |
| `ConnectionType` | `enum` | CONNECT_SSH, CONNECT_SSM | - |
| `EnvironmentLifecycleStatus` | `enum` | CREATING, CREATED, CREATE_FAILED, DELETING, DELETE_FAILED | - |
| `EnvironmentStatus` | `enum` | ERROR, CREATING, CONNECTING, READY, STOPPING, STOPPED, DELETING | - |
| `EnvironmentType` | `enum` | SSH, EC2 | - |
| `ManagedCredentialsAction` | `enum` | ENABLE, DISABLE | - |
| `ManagedCredentialsStatus` | `enum` | ENABLED_ON_CREATE, ENABLED_BY_OWNER, DISABLED_BY_DEFAULT, DISABLED_BY_OWNER, DISABLED_BY_COLLABORATOR, PENDING_REMOVAL_BY_COLLABORATOR, PENDING_START_REMOVAL_BY_COLLABORATOR, PENDING_REMOVAL_BY_OWNER, PENDING_START_REMOVAL_BY_OWNER, FAILED_REMOVAL_BY_COLLABORATOR, FAILED_REMOVAL_BY_OWNER | - |
| `MemberPermissions` | `enum` | READ_WRITE, READ_ONLY | - |
| `Permissions` | `enum` | OWNER, READ_WRITE, READ_ONLY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
