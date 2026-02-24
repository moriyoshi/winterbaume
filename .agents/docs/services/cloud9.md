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

- Operations: `DescribeEnvironmentMemberships`, `DescribeEnvironmentStatus`, `DescribeEnvironments`
- Traits: `paginated` (1)
- Common required input members in this group: `environmentId`, `environmentIds`

### Create

- Operations: `CreateEnvironmentEC2`, `CreateEnvironmentMembership`
- Traits: `idempotent` (2)
- Common required input members in this group: `environmentId`, `imageId`, `instanceType`, `name`, `permissions`, `userArn`

### Delete

- Operations: `DeleteEnvironment`, `DeleteEnvironmentMembership`
- Traits: `idempotent` (2)
- Common required input members in this group: `environmentId`, `userArn`

### List

- Operations: `ListEnvironments`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: `ResourceARN`

### Update

- Operations: `UpdateEnvironment`, `UpdateEnvironmentMembership`
- Traits: `idempotent` (2)
- Common required input members in this group: `environmentId`, `permissions`, `userArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEnvironmentEC2` | - | `idempotent` | `imageId`, `instanceType`, `name` | - | `CreateEnvironmentEC2Result` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Creates an Cloud9 development environment, launches an Amazon Elastic Compute Cloud (Amazon EC2) instance, and then connects from the instance to the environment. Cloud9 is no longer available to new customers. |
| `CreateEnvironmentMembership` | - | `idempotent` | `environmentId`, `permissions`, `userArn` | - | `CreateEnvironmentMembershipResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Adds an environment member to an Cloud9 development environment. Cloud9 is no longer available to new customers. |
| `DeleteEnvironment` | - | `idempotent` | `environmentId` | - | `DeleteEnvironmentResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Deletes an Cloud9 development environment. If an Amazon EC2 instance is connected to the environment, also terminates the instance. |
| `DeleteEnvironmentMembership` | - | `idempotent` | `environmentId`, `userArn` | - | `DeleteEnvironmentMembershipResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Deletes an environment member from a development environment. Cloud9 is no longer available to new customers. |
| `DescribeEnvironmentMemberships` | - | `paginated` | - | - | `DescribeEnvironmentMembershipsResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets information about environment members for an Cloud9 development environment. Cloud9 is no longer available to new customers. |
| `DescribeEnvironmentStatus` | - | - | `environmentId` | - | `DescribeEnvironmentStatusResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets status information for an Cloud9 development environment. Cloud9 is no longer available to new customers. |
| `DescribeEnvironments` | - | - | `environmentIds` | - | `DescribeEnvironmentsResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets information about Cloud9 development environments. Cloud9 is no longer available to new customers. |
| `ListEnvironments` | - | `paginated` | - | - | `ListEnvironmentsResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Gets a list of Cloud9 development environment identifiers. Cloud9 is no longer available to new customers. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Gets a list of the tags associated with an Cloud9 development environment. Cloud9 is no longer available to new customers. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ConcurrentAccessException`, `InternalServerErrorException`, `NotFoundException` | Adds tags to an Cloud9 development environment. Cloud9 is no longer available to new customers. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConcurrentAccessException`, `InternalServerErrorException`, `NotFoundException` | Removes tags from an Cloud9 development environment. Cloud9 is no longer available to new customers. |
| `UpdateEnvironment` | - | `idempotent` | `environmentId` | - | `UpdateEnvironmentResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Changes the settings of an existing Cloud9 development environment. Cloud9 is no longer available to new customers. |
| `UpdateEnvironmentMembership` | - | `idempotent` | `environmentId`, `permissions`, `userArn` | - | `UpdateEnvironmentMembershipResult` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Changes the settings of an existing environment member for an Cloud9 development environment. Cloud9 is no longer available to new customers. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `className`, `code`, `message` | The target request is invalid. |
| `InternalServerErrorException` | `structure` | `className`, `code`, `message` | An internal server error occurred. |
| `NotFoundException` | `structure` | `className`, `code`, `message` | The target resource cannot be found. |
| `ConflictException` | `structure` | `className`, `code`, `message` | A conflict occurred. |
| `ForbiddenException` | `structure` | `className`, `code`, `message` | An access permissions issue occurred. |
| `LimitExceededException` | `structure` | `className`, `code`, `message` | A service limit was exceeded. |
| `TooManyRequestsException` | `structure` | `className`, `code`, `message` | Too many service requests were made over the given time period. |
| `ConcurrentAccessException` | `structure` | `className`, `code`, `message` | A concurrent access issue occurred. |
| `CreateEnvironmentEC2Request` | `structure` | `automaticStopTimeMinutes`, `clientRequestToken`, `connectionType`, `description`, `dryRun`, `imageId`, `instanceType`, `name`, `ownerArn`, `subnetId`, `tags` | - |
| `CreateEnvironmentEC2Result` | `structure` | `environmentId` | - |
| `CreateEnvironmentMembershipRequest` | `structure` | `environmentId`, `permissions`, `userArn` | - |
| `CreateEnvironmentMembershipResult` | `structure` | `membership` | - |
| `DeleteEnvironmentRequest` | `structure` | `environmentId` | - |
| `DeleteEnvironmentResult` | `structure` | - | - |
| `DeleteEnvironmentMembershipRequest` | `structure` | `environmentId`, `userArn` | - |
| `DeleteEnvironmentMembershipResult` | `structure` | - | - |
| `DescribeEnvironmentMembershipsRequest` | `structure` | `environmentId`, `maxResults`, `nextToken`, `permissions`, `userArn` | - |
| `DescribeEnvironmentMembershipsResult` | `structure` | `memberships`, `nextToken` | - |
| `DescribeEnvironmentStatusRequest` | `structure` | `environmentId` | - |
| `DescribeEnvironmentStatusResult` | `structure` | `message`, `status` | - |
| `DescribeEnvironmentsRequest` | `structure` | `environmentIds` | - |
| `DescribeEnvironmentsResult` | `structure` | `environments` | - |
| `ListEnvironmentsRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListEnvironmentsResult` | `structure` | `environmentIds`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
