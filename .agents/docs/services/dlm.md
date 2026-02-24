# Amazon Data Lifecycle Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Data Lifecycle Manager With Amazon Data Lifecycle Manager, you can manage the lifecycle of your Amazon Web Services resources. You create lifecycle policies, which are used to automate operations on the specified resources. Amazon Data Lifecycle Manager supports Amazon EBS volumes and snapshots. For information about using Amazon Data Lifecycle Manager with Amazon EBS, see Amazon Data Lifecycle Manager in the Amazon EC2 User Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Data Lifecycle Manager workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Create`, `Delete`, `List`, `Tag` operation families, including `GetLifecyclePolicies`, `GetLifecyclePolicy`, `CreateLifecyclePolicy`, `DeleteLifecyclePolicy`, `ListTagsForResource`, `TagResource`.

## Service Identity and Protocol

- AWS model slug: `dlm`
- AWS SDK for Rust slug: `dlm`
- Model version: `2018-01-12`
- Model file: `vendor/api-models-aws/models/dlm/service/2018-01-12/dlm-2018-01-12.json`
- SDK ID: `DLM`
- Endpoint prefix: `dlm`
- ARN namespace: `dlm`
- CloudFormation name: `DLM`
- CloudTrail event source: `dlm.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (2), `Create` (1), `Delete` (1), `List` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateLifecyclePolicy`, `DeleteLifecyclePolicy`, `TagResource`, `UntagResource`, `UpdateLifecyclePolicy`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetLifecyclePolicies`, `GetLifecyclePolicy`, `ListTagsForResource`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 8 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/ebs/latest/userguide/snapshot-lifecycle.html
- https://docs.aws.amazon.com/ebs/latest/userguide/dlm-elements.html
- https://docs.aws.amazon.com/ebs/latest/userguide/snapshot-ami-policy.html

Research outcomes:
- Amazon Data Lifecycle Manager automates creation, retention, copy, archive, sharing, and deletion of EBS snapshots and EBS-backed AMIs.
- Policies can be default policies or custom policies.
- Custom policies use target resource tags and schedules to select volumes or instances and create snapshots or AMIs.
- Schedules define frequency, start times, retention, copy rules, archive rules, and fast snapshot restore settings.
- DLM applies its own tags to created snapshots and AMIs for tracking.
- Cross-Region and cross-account copy behaviour depends on policy configuration and encryption/KMS constraints.

Parity implications:
- Model lifecycle policies, schedules, target tags, created snapshots/AMIs, retention rules, copy rules, archive rules, and DLM tags separately.
- Policy execution should be scheduled and asynchronous, deriving targets from current tags.
- Retention deletion should affect only policy-managed artefacts according to retention rules.

## Operation Groups

### Get

- Operations: `GetLifecyclePolicies`, `GetLifecyclePolicy`
- Common required input members in this group: `PolicyId`

### Create

- Operations: `CreateLifecyclePolicy`
- Common required input members in this group: `Description`, `ExecutionRoleArn`, `State`

### Delete

- Operations: `DeleteLifecyclePolicy`
- Common required input members in this group: `PolicyId`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `ResourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateLifecyclePolicy`
- Common required input members in this group: `PolicyId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateLifecyclePolicy` | `POST /policies` | - | `Description`, `ExecutionRoleArn`, `State` | - | `CreateLifecyclePolicyResponse` | `InternalServerException`, `InvalidRequestException`, `LimitExceededException` | Creates an Amazon Data Lifecycle Manager lifecycle policy. Amazon Data Lifecycle Manager supports the following policy types: Custom EBS snapshot policy Custom EBS-backed AMI policy Cross-account copy event policy Default policy for EBS snapshots Default... |
| `DeleteLifecyclePolicy` | `DELETE /policies/{PolicyId}` | - | `PolicyId` | - | `DeleteLifecyclePolicyResponse` | `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException` | Deletes the specified lifecycle policy and halts the automated operations that the policy specified. For more information about deleting a policy, see Delete lifecycle policies. |
| `GetLifecyclePolicies` | `GET /policies` | - | - | - | `GetLifecyclePoliciesResponse` | `InternalServerException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException` | Gets summary information about all or the specified data lifecycle policies. To get complete information about a policy, use GetLifecyclePolicy. |
| `GetLifecyclePolicy` | `GET /policies/{PolicyId}` | - | `PolicyId` | - | `GetLifecyclePolicyResponse` | `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException` | Gets detailed information about the specified lifecycle policy. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Lists the tags for the specified resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Removes the specified tags from the specified resource. |
| `UpdateLifecyclePolicy` | `PATCH /policies/{PolicyId}` | - | `PolicyId` | - | `UpdateLifecyclePolicyResponse` | `InternalServerException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException` | Updates the specified lifecycle policy. For more information about updating a policy, see Modify lifecycle policies. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Code`, `Message` | The service failed in an unexpected way. |
| `ResourceNotFoundException` | `structure` | `Code`, `Message`, `ResourceIds`, `ResourceType` | A requested resource was not found. |
| `InvalidRequestException` | `structure` | `Code`, `Message`, `MutuallyExclusiveParameters`, `RequiredParameters` | Bad request. |
| `LimitExceededException` | `structure` | `Code`, `Message`, `ResourceType` | The request failed because a limit was exceeded. |
| `CreateLifecyclePolicyRequest` | `structure` | `CopyTags`, `CreateInterval`, `CrossRegionCopyTargets`, `DefaultPolicy`, `Description`, `Exclusions`, `ExecutionRoleArn`, `ExtendDeletion`, `PolicyDetails`, `RetainInterval`, `State`, `Tags` | - |
| `CreateLifecyclePolicyResponse` | `structure` | `PolicyId` | - |
| `DeleteLifecyclePolicyRequest` | `structure` | `PolicyId` | - |
| `DeleteLifecyclePolicyResponse` | `structure` | - | - |
| `GetLifecyclePoliciesRequest` | `structure` | `DefaultPolicyType`, `PolicyIds`, `ResourceTypes`, `State`, `TagsToAdd`, `TargetTags` | - |
| `GetLifecyclePoliciesResponse` | `structure` | `Policies` | - |
| `GetLifecyclePolicyRequest` | `structure` | `PolicyId` | - |
| `GetLifecyclePolicyResponse` | `structure` | `Policy` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `Tags` | - |
| `TagResourceRequest` | `structure` | `ResourceArn`, `Tags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UntagResourceRequest` | `structure` | `ResourceArn`, `TagKeys` | - |
| `UntagResourceResponse` | `structure` | - | - |
| `UpdateLifecyclePolicyRequest` | `structure` | `CopyTags`, `CreateInterval`, `CrossRegionCopyTargets`, `Description`, `Exclusions`, `ExecutionRoleArn`, `ExtendDeletion`, `PolicyDetails`, `PolicyId`, `RetainInterval`, `State` | - |
| `UpdateLifecyclePolicyResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
