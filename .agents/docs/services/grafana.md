# Amazon Managed Grafana

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Grafana is a fully managed and secure data visualization service that you can use to instantly query, correlate, and visualize operational metrics, logs, and traces from multiple sources. Amazon Managed Grafana makes it easy to deploy, operate, and scale Grafana, a widely deployed data visualization tool that is popular for its extensible data support. With Amazon Managed Grafana, you create logically isolated Grafana servers called workspaces . In a workspace, you can create Grafana dashboards and visualizations to analyze your metrics, logs, and traces without having to build, package, or deploy any hardware to run Grafana servers.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Managed Grafana where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Managed Grafana by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Managed Grafana workflows in the local mock. Key resources include `ApiKey`, `Authentication`, `Configuration`, `License`, `Permission`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Update`, `Describe` operation families, including `ListPermissions`, `ListTagsForResource`, `ListVersions`, `ListWorkspaceServiceAccountTokens`, `CreateWorkspace`, `CreateWorkspaceApiKey`.

## Service Identity and Protocol

- AWS model slug: `grafana`
- AWS SDK for Rust slug: `grafana`
- Model version: `2020-08-18`
- Model file: `vendor/api-models-aws/models/grafana/service/2020-08-18/grafana-2020-08-18.json`
- SDK ID: `grafana`
- Endpoint prefix: `-`
- ARN namespace: `grafana`
- CloudFormation name: `Grafana`
- CloudTrail event source: `grafana.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Create` (4), `Delete` (4), `Update` (4), `Describe` (3), `Associate` (1), `Disassociate` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateLicense`, `CreateWorkspace`, `CreateWorkspaceApiKey`, `CreateWorkspaceServiceAccount`, `CreateWorkspaceServiceAccountToken`, `DeleteWorkspace`, `DeleteWorkspaceApiKey`, `DeleteWorkspaceServiceAccount`, `DeleteWorkspaceServiceAccountToken`, `DisassociateLicense`, `TagResource`, `UntagResource`, `UpdatePermissions`, `UpdateWorkspace`, `UpdateWorkspaceAuthentication`, `UpdateWorkspaceConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeWorkspace`, `DescribeWorkspaceAuthentication`, `DescribeWorkspaceConfiguration`, `ListPermissions`, `ListTagsForResource`, `ListVersions`, `ListWorkspaceServiceAccountTokens`, `ListWorkspaceServiceAccounts`, `ListWorkspaces`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 25 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApiKey` | `workspaceId` | - | `CreateWorkspaceApiKey`, `DeleteWorkspaceApiKey` | - |
| `Authentication` | `workspaceId` | read: `DescribeWorkspaceAuthentication`; update: `UpdateWorkspaceAuthentication` | - | - |
| `Configuration` | `workspaceId` | - | `DescribeWorkspaceConfiguration`, `UpdateWorkspaceConfiguration` | - |
| `License` | `licenseType`, `workspaceId` | - | `AssociateLicense`, `DisassociateLicense` | - |
| `Permission` | `workspaceId` | read: `ListPermissions`; update: `UpdatePermissions` | - | - |
| `ServiceAccount` | `workspaceId` | - | `CreateWorkspaceServiceAccount`, `DeleteWorkspaceServiceAccount`, `ListWorkspaceServiceAccounts` | - |
| `ServiceAccountToken` | `serviceAccountId`, `workspaceId` | - | `CreateWorkspaceServiceAccountToken`, `DeleteWorkspaceServiceAccountToken`, `ListWorkspaceServiceAccountTokens` | - |
| `Workspace` | `workspaceId` | create: `CreateWorkspace`; read: `DescribeWorkspace`; update: `UpdateWorkspace`; delete: `DeleteWorkspace`; list: `ListWorkspaces` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`, `ListVersions`
- Traits: `readonly` (2), `paginated` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The ListTagsForResource operation returns the tags that are associated with the Amazon Managed Service for Grafana resource specified by the resourceArn . Currently, the only resource that can be tagged is a workspace. |
| `ListVersions` | `GET /versions` | `readonly`, `paginated` | - | - | `ListVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists available versions of Grafana. These are available when calling CreateWorkspace . Optionally, include a workspace to list the versions to which it can be upgraded. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The TagResource operation associates tags with an Amazon Managed Grafana resource. Currently, the only resource that can be tagged is workspaces. If you specify a new tag key for the resource, this tag is appended to ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The UntagResource operation removes the association of the tag with the Amazon Managed Grafana resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListVersions` | - | `maxResults -> maxResults`, `nextToken -> nextToken`, `workspaceId -> workspace-id` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient permissions to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | A resource was in an inconsistent state during an update or a deletion. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | Unexpected error while processing the request. Retry the request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The request references a resource that does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was denied because of request throttling. Retry the request. |
| `ValidationException` | `structure` | message, reason, fieldList | The value of a parameter in the request caused an error. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `ListVersionsRequest` | `structure` | maxResults, nextToken, workspaceId | - |
| `ListVersionsResponse` | `structure` | nextToken, grafanaVersions | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
