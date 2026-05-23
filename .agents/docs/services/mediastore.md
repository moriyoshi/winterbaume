# AWS Elemental MediaStore

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

An AWS Elemental MediaStore container is a namespace that holds folders and objects. You use a container endpoint to create, read, and delete objects.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-mediastore/tests/scenario_test.rs`: manage a container policy lifecycle.
- Backported from `scenario_test.rs`: attach metric and lifecycle policies to the same container and verify independent policy state.
- Backported from `scenario_test.rs`: tag containers and clean up tagged resources.
- From the AWS documentation and model: represent media container provisioning, policy documents, CORS/metric/lifecycle policies, endpoint discovery, tagging, and deletion state.

## Service Identity and Protocol

- AWS model slug: `mediastore`
- AWS SDK for Rust slug: `mediastore`
- Model version: `2017-09-01`
- Model file: `vendor/api-models-aws/models/mediastore/service/2017-09-01/mediastore-2017-09-01.json`
- SDK ID: `MediaStore`
- Endpoint prefix: `mediastore`
- ARN namespace: `mediastore`
- CloudFormation name: `MediaStore`
- CloudTrail event source: `mediastore.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (5), `Get` (4), `Put` (4), `List` (2), `Create` (1), `Describe` (1), `Start` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateContainer`, `DeleteContainer`, `DeleteContainerPolicy`, `DeleteCorsPolicy`, `DeleteLifecyclePolicy`, `DeleteMetricPolicy`, `PutContainerPolicy`, `PutCorsPolicy`, `PutLifecyclePolicy`, `PutMetricPolicy`, `StartAccessLogging`, `StopAccessLogging`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeContainer`, `GetContainerPolicy`, `GetCorsPolicy`, `GetLifecyclePolicy`, `GetMetricPolicy`, `ListContainers`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartAccessLogging`, `StopAccessLogging`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 21 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Control-Plane / Data-Plane Coherence

- **Paired with `mediastoredata`.** MediaStore Data ( `winterbaume-mediastoredata` ) reads and writes objects that live **inside containers** created by this control plane via `CreateContainer`. In real AWS each container has its own data-plane endpoint URL ( `<container-name>.data.mediastore.<region>.amazonaws.com` ); `PutObject` / `GetObject` calls are scoped to a single container by virtue of the endpoint they target.
- **Current Winterbaume status: divergent and the model is wrong.** `winterbaume-mediastoredata` stores objects in a single global `objects: HashMap<String, MediaStoreObject>` without any container concept, so an object PUT into one container is visible to GET requests targeting any other container, and `DeleteContainer` here does not affect the data plane's stored objects.
- **What needs to change:** `winterbaume-mediastoredata` must key its `objects` map by `(container_name, path)` ( derived from the request's host header / endpoint ) and observe this crate's `containers` state so that operations against an unknown or `DELETING` container fail with the right error. `DeleteContainer` here should also reject when the container still has objects, matching the real-AWS `ContainerNotEmptyException`.

## Operation Groups

### Delete

- Operations: `DeleteContainer`, `DeleteContainerPolicy`, `DeleteCorsPolicy`, `DeleteLifecyclePolicy`, `DeleteMetricPolicy`
- Common required input members in this group: `ContainerName`

### Get

- Operations: `GetContainerPolicy`, `GetCorsPolicy`, `GetLifecyclePolicy`, `GetMetricPolicy`
- Common required input members in this group: `ContainerName`

### Put

- Operations: `PutContainerPolicy`, `PutCorsPolicy`, `PutLifecyclePolicy`, `PutMetricPolicy`
- Common required input members in this group: `ContainerName`

### List

- Operations: `ListContainers`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateContainer`
- Common required input members in this group: -

### Describe

- Operations: `DescribeContainer`
- Common required input members in this group: -

### Start

- Operations: `StartAccessLogging`
- Common required input members in this group: -

### Stop

- Operations: `StopAccessLogging`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateContainer` | `-` | - | `ContainerName` | - | `CreateContainerOutput` | `ContainerInUseException`, `InternalServerError`, `LimitExceededException` | Creates a storage container to hold objects. A container is similar to a bucket in the Amazon S3 service. |
| `DeleteContainer` | `-` | - | `ContainerName` | - | `DeleteContainerOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Deletes the specified container. Before you make a DeleteContainer request, delete any objects in the container or in any folders in the container. You can delete only empty containers. |
| `DeleteContainerPolicy` | `-` | - | `ContainerName` | - | `DeleteContainerPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError`, `PolicyNotFoundException` | Deletes the access policy that is associated with the specified container. |
| `DeleteCorsPolicy` | `-` | - | `ContainerName` | - | `DeleteCorsPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `CorsPolicyNotFoundException`, `InternalServerError` | Deletes the cross-origin resource sharing (CORS) configuration information that is set for the container. To use this operation, you must have permission to perform the MediaStore:DeleteCorsPolicy action. The contain ... |
| `DeleteLifecyclePolicy` | `-` | - | `ContainerName` | - | `DeleteLifecyclePolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError`, `PolicyNotFoundException` | Removes an object lifecycle policy from a container. It takes up to 20 minutes for the change to take effect. |
| `DeleteMetricPolicy` | `-` | - | `ContainerName` | - | `DeleteMetricPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError`, `PolicyNotFoundException` | Deletes the metric policy that is associated with the specified container. If there is no metric policy associated with the container, MediaStore doesn't send metrics to CloudWatch. |
| `DescribeContainer` | `-` | - | - | - | `DescribeContainerOutput` | `ContainerNotFoundException`, `InternalServerError` | Retrieves the properties of the requested container. This request is commonly used to retrieve the endpoint of a container. An endpoint is a value assigned by the service when a new container is created. A container' ... |
| `GetContainerPolicy` | `-` | - | `ContainerName` | - | `GetContainerPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError`, `PolicyNotFoundException` | Retrieves the access policy for the specified container. For information about the data that is included in an access policy, see the AWS Identity and Access Management User Guide . |
| `GetCorsPolicy` | `-` | - | `ContainerName` | - | `GetCorsPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `CorsPolicyNotFoundException`, `InternalServerError` | Returns the cross-origin resource sharing (CORS) configuration information that is set for the container. To use this operation, you must have permission to perform the MediaStore:GetCorsPolicy action. By default, th ... |
| `GetLifecyclePolicy` | `-` | - | `ContainerName` | - | `GetLifecyclePolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError`, `PolicyNotFoundException` | Retrieves the object lifecycle policy that is assigned to a container. |
| `GetMetricPolicy` | `-` | - | `ContainerName` | - | `GetMetricPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError`, `PolicyNotFoundException` | Returns the metric policy for the specified container. |
| `ListContainers` | `-` | `paginated` | - | - | `ListContainersOutput` | `InternalServerError` | Lists the properties of all containers in AWS Elemental MediaStore. You can query to receive all the containers in one response. Or you can include the MaxResults parameter to receive a limited number of containers i ... |
| `ListTagsForResource` | `-` | - | `Resource` | - | `ListTagsForResourceOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Returns a list of the tags assigned to the specified container. |
| `PutContainerPolicy` | `-` | - | `ContainerName`, `Policy` | - | `PutContainerPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Creates an access policy for the specified container to restrict the users and clients that can access it. For information about the data that is included in an access policy, see the AWS Identity and Access Manageme ... |
| `PutCorsPolicy` | `-` | - | `ContainerName`, `CorsPolicy` | - | `PutCorsPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Sets the cross-origin resource sharing (CORS) configuration on a container so that the container can service cross-origin requests. For example, you might want to enable a request whose origin is http://www.example.c ... |
| `PutLifecyclePolicy` | `-` | - | `ContainerName`, `LifecyclePolicy` | - | `PutLifecyclePolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Writes an object lifecycle policy to a container. If the container already has an object lifecycle policy, the service replaces the existing policy with the new policy. It takes up to 20 minutes for the change to tak ... |
| `PutMetricPolicy` | `-` | - | `ContainerName`, `MetricPolicy` | - | `PutMetricPolicyOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | The metric policy that you want to add to the container. A metric policy allows AWS Elemental MediaStore to send metrics to Amazon CloudWatch. It takes up to 20 minutes for the new policy to take effect. |
| `StartAccessLogging` | `-` | - | `ContainerName` | - | `StartAccessLoggingOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Starts access logging on the specified container. When you enable access logging on a container, MediaStore delivers access logs for objects stored in that container to Amazon CloudWatch Logs. |
| `StopAccessLogging` | `-` | - | `ContainerName` | - | `StopAccessLoggingOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Stops access logging on the specified container. When you stop access logging on a container, MediaStore stops sending access logs to Amazon CloudWatch Logs. These access logs are not saved and are not retrievable. |
| `TagResource` | `-` | - | `Resource`, `Tags` | - | `TagResourceOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Adds tags to the specified AWS Elemental MediaStore container. Tags are key:value pairs that you can associate with AWS resources. For example, the tag key might be "customer" and the tag value might be "companyA." Y ... |
| `UntagResource` | `-` | - | `Resource`, `TagKeys` | - | `UntagResourceOutput` | `ContainerInUseException`, `ContainerNotFoundException`, `InternalServerError` | Removes tags from the specified container. You can specify one or more tags to remove. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ContainerInUseException` | `structure` | Message | The container that you specified in the request already exists or is being updated. |
| `ContainerNotFoundException` | `structure` | Message | The container that you specified in the request does not exist. |
| `CorsPolicyNotFoundException` | `structure` | Message | The CORS policy that you specified in the request does not exist. |
| `InternalServerError` | `structure` | Message | The service is temporarily unavailable. |
| `LimitExceededException` | `structure` | Message | A service limit has been exceeded. |
| `PolicyNotFoundException` | `structure` | Message | The policy that you specified in the request does not exist. |
| `CreateContainerInput` | `structure` | ContainerName, Tags | - |
| `CreateContainerOutput` | `structure` | Container | - |
| `DeleteContainerInput` | `structure` | ContainerName | - |
| `DeleteContainerOutput` | `structure` | **empty (no members)** | - |
| `DeleteContainerPolicyInput` | `structure` | ContainerName | - |
| `DeleteContainerPolicyOutput` | `structure` | **empty (no members)** | - |
| `DeleteCorsPolicyInput` | `structure` | ContainerName | - |
| `DeleteCorsPolicyOutput` | `structure` | **empty (no members)** | - |
| `DeleteLifecyclePolicyInput` | `structure` | ContainerName | - |
| `DeleteLifecyclePolicyOutput` | `structure` | **empty (no members)** | - |
| `DeleteMetricPolicyInput` | `structure` | ContainerName | - |
| `DeleteMetricPolicyOutput` | `structure` | **empty (no members)** | - |
| `DescribeContainerInput` | `structure` | ContainerName | - |
| `DescribeContainerOutput` | `structure` | Container | - |
| `GetContainerPolicyInput` | `structure` | ContainerName | - |
| `GetContainerPolicyOutput` | `structure` | Policy | - |
| `GetCorsPolicyInput` | `structure` | ContainerName | - |
| `GetCorsPolicyOutput` | `structure` | CorsPolicy | - |
| `GetLifecyclePolicyInput` | `structure` | ContainerName | - |
| `GetLifecyclePolicyOutput` | `structure` | LifecyclePolicy | - |
| `GetMetricPolicyInput` | `structure` | ContainerName | - |
| `GetMetricPolicyOutput` | `structure` | MetricPolicy | - |
| `ListContainersInput` | `structure` | NextToken, MaxResults | - |
| `ListContainersOutput` | `structure` | Containers, NextToken | - |
| `ListTagsForResourceInput` | `structure` | Resource | - |
| `ListTagsForResourceOutput` | `structure` | Tags | - |
| `PutContainerPolicyInput` | `structure` | ContainerName, Policy | - |
| `PutContainerPolicyOutput` | `structure` | **empty (no members)** | - |
| `PutCorsPolicyInput` | `structure` | ContainerName, CorsPolicy | - |
| `PutCorsPolicyOutput` | `structure` | **empty (no members)** | - |
| `PutLifecyclePolicyInput` | `structure` | ContainerName, LifecyclePolicy | - |
| `PutLifecyclePolicyOutput` | `structure` | **empty (no members)** | - |
| `PutMetricPolicyInput` | `structure` | ContainerName, MetricPolicy | - |
| `PutMetricPolicyOutput` | `structure` | **empty (no members)** | - |
| `ContainerLevelMetrics` | `enum` | ENABLED, DISABLED | - |
| `ContainerStatus` | `enum` | ACTIVE, CREATING, DELETING | - |
| `MethodName` | `enum` | PUT, GET, DELETE, HEAD | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
