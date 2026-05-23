# Amazon CodeCatalyst

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Amazon CodeCatalyst API reference. This reference provides descriptions of operations and data types for Amazon CodeCatalyst. You can use the Amazon CodeCatalyst API to work with the following objects. Spaces, by calling the following: DeleteSpace, which deletes a space. GetSpace, which returns information about a space.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon CodeCatalyst where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: add full state-machine walks for Amazon CodeCatalyst resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon CodeCatalyst workflows in the local mock. Key resources include `AccessToken`, `DevEnvironment`, `EventLogResource`, `Project`, `SourceRepository`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Start` operation families, including `ListAccessTokens`, `ListDevEnvironmentSessions`, `ListDevEnvironments`, `ListEventLogs`, `GetDevEnvironment`, `GetProject`.

## Service Identity and Protocol

- AWS model slug: `codecatalyst`
- AWS SDK for Rust slug: `codecatalyst`
- Model version: `2022-09-28`
- Model file: `vendor/api-models-aws/models/codecatalyst/service/2022-09-28/codecatalyst-2022-09-28.json`
- SDK ID: `CodeCatalyst`
- Endpoint prefix: `codecatalyst`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: -
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (9), `Create` (5), `Delete` (5), `Start` (3), `Update` (3), `Stop` (2), `Verify` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAccessToken`, `CreateDevEnvironment`, `CreateProject`, `CreateSourceRepository`, `CreateSourceRepositoryBranch`, `DeleteAccessToken`, `DeleteDevEnvironment`, `DeleteProject`, `DeleteSourceRepository`, `DeleteSpace`, `StartDevEnvironment`, `StartDevEnvironmentSession`, `StartWorkflowRun`, `StopDevEnvironment`, `StopDevEnvironmentSession`, `UpdateDevEnvironment`, `UpdateProject`, `UpdateSpace`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDevEnvironment`, `GetProject`, `GetSourceRepository`, `GetSourceRepositoryCloneUrls`, `GetSpace`, `GetSubscription`, `GetUserDetails`, `GetWorkflow`, `GetWorkflowRun`, `ListAccessTokens`, `ListDevEnvironmentSessions`, `ListDevEnvironments`, `ListEventLogs`, `ListProjects`, `ListSourceRepositories`, `ListSourceRepositoryBranches`, `ListSpaces`, `ListWorkflowRuns`, `ListWorkflows`, `VerifySession`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 16 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDevEnvironment`, `StartDevEnvironmentSession`, `StartWorkflowRun`, `StopDevEnvironment`, `StopDevEnvironmentSession`.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `ECR`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccessToken` | `accessTokenId` | create: `CreateAccessToken`; delete: `DeleteAccessToken`; list: `ListAccessTokens` | - | - |
| `DevEnvironment` | `devEnvironmentId`, `projectName`, `spaceName` | create: `CreateDevEnvironment`; read: `GetDevEnvironment`; update: `UpdateDevEnvironment`; delete: `DeleteDevEnvironment` | `ListDevEnvironmentSessions`, `StartDevEnvironment`, `StartDevEnvironmentSession`, `StopDevEnvironment`, `StopDevEnvironmentSession` | - |
| `EventLogResource` | `eventLogId`, `spaceName` | list: `ListEventLogs` | - | - |
| `Project` | `projectName`, `spaceName` | create: `CreateProject`; read: `GetProject`; update: `UpdateProject`; delete: `DeleteProject`; list: `ListProjects` | - | - |
| `SourceRepository` | `projectName`, `sourceRepositoryName`, `spaceName` | put: `CreateSourceRepository`; read: `GetSourceRepository`; delete: `DeleteSourceRepository`; list: `ListSourceRepositories` | `GetSourceRepositoryCloneUrls` | - |
| `SourceRepositoryBranch` | `projectName`, `sourceRepositoryBranchName`, `sourceRepositoryName`, `spaceName` | put: `CreateSourceRepositoryBranch`; list: `ListSourceRepositoryBranches` | - | - |
| `Space` | `spaceName` | read: `GetSpace`; update: `UpdateSpace`; delete: `DeleteSpace`; list: `ListSpaces` | `ListDevEnvironments` | - |
| `Subscription` | `spaceName` | read: `GetSubscription` | - | - |
| `Workflow` | `projectName`, `spaceName`, `workflowId` | read: `GetWorkflow`; list: `ListWorkflows` | - | - |
| `WorkflowRun` | `projectName`, `spaceName`, `workflowRunId` | create: `StartWorkflowRun`; read: `GetWorkflowRun`; list: `ListWorkflowRuns` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/codecatalyst/latest/userguide/extensions-link.html
- https://docs.aws.amazon.com/codecatalyst/latest/userguide/projects-troubleshooting.html

Research outcomes:
- CodeCatalyst organises software delivery work into spaces and projects with source repositories, issues, workflows, dev environments, and extensions.
- Projects can link third-party GitHub, Bitbucket, GitLab repositories, and Jira projects.
- Linked repositories can be used by workflows, dev environments, and pull request integrations.
- Blueprints can scaffold projects and can fail with project-specific troubleshooting conditions.
- CodeCatalyst resources exist outside standard regional AWS service control planes in several workflows, but still integrate with AWS IAM and connections.

Parity implications:
- Model spaces, projects, repositories, linked repositories, workflows, dev environments, issues, blueprints, and extension links separately.
- Linked third-party repository state should be distinct from native source repository state.
- Workflow execution should depend on repository linkage and trigger configuration.

## Operation Groups

### Get

- Operations: `GetUserDetails`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Verify

- Operations: `VerifySession`
- Traits: `readonly` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetUserDetails` | `GET /userDetails` | `readonly` | - | - | `GetUserDetailsResponse` | - | Returns information about a user. |
| `VerifySession` | `GET /session` | `readonly` | - | - | `VerifySessionResponse` | - | Verifies whether the calling user has a valid Amazon CodeCatalyst login and session. If successful, this returns the ID of the user in Amazon CodeCatalyst. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetUserDetails` | - | `id -> id`, `userName -> userName` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The request was denied because you don't have sufficient access to perform this action. Verify that you are a member of a role that allows this action. |
| `ConflictException` | `structure` | message | The request was denied because the requested operation would cause a conflict with the current state of a service resource associated with the request. Anot ... |
| `ResourceNotFoundException` | `structure` | message | The request was denied because the specified resource was not found. Verify that the spelling is correct and that you have access to the resource. |
| `ServiceQuotaExceededException` | `structure` | message | The request was denied because one or more resources has reached its limits for the tier the space belongs to. Either reduce the number of resources, or cha ... |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The request was denied because an input failed to satisfy the constraints specified by the service. Check the spelling and input requirements, and then try ... |
| `GetUserDetailsRequest` | `structure` | id, userName | - |
| `GetUserDetailsResponse` | `structure` | userId, userName, displayName, primaryEmail, version | - |
| `VerifySessionResponse` | `structure` | identity | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
