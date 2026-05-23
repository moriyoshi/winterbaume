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

### List

- Operations: `ListAccessTokens`, `ListDevEnvironmentSessions`, `ListDevEnvironments`, `ListEventLogs`, `ListProjects`, `ListSourceRepositories`, `ListSourceRepositoryBranches`, `ListSpaces`, `ListWorkflowRuns`, `ListWorkflows`
- Traits: `paginated` (10), `readonly` (10)
- Common required input members in this group: `devEnvironmentId`, `endTime`, `projectName`, `sourceRepositoryName`, `spaceName`, `startTime`

### Get

- Operations: `GetDevEnvironment`, `GetProject`, `GetSourceRepository`, `GetSourceRepositoryCloneUrls`, `GetSpace`, `GetSubscription`, `GetUserDetails`, `GetWorkflow`, `GetWorkflowRun`
- Traits: `readonly` (9)
- Common required input members in this group: `id`, `name`, `projectName`, `sourceRepositoryName`, `spaceName`

### Create

- Operations: `CreateAccessToken`, `CreateDevEnvironment`, `CreateProject`, `CreateSourceRepository`, `CreateSourceRepositoryBranch`
- Traits: `idempotent` (4)
- Common required input members in this group: `displayName`, `instanceType`, `name`, `persistentStorage`, `projectName`, `sourceRepositoryName`, `spaceName`

### Delete

- Operations: `DeleteAccessToken`, `DeleteDevEnvironment`, `DeleteProject`, `DeleteSourceRepository`, `DeleteSpace`
- Traits: `idempotent` (5)
- Common required input members in this group: `id`, `name`, `projectName`, `spaceName`

### Start

- Operations: `StartDevEnvironment`, `StartDevEnvironmentSession`, `StartWorkflowRun`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `id`, `projectName`, `sessionConfiguration`, `spaceName`, `workflowId`

### Update

- Operations: `UpdateDevEnvironment`, `UpdateProject`, `UpdateSpace`
- Traits: `idempotent` (3)
- Common required input members in this group: `id`, `name`, `projectName`, `spaceName`

### Stop

- Operations: `StopDevEnvironment`, `StopDevEnvironmentSession`
- Traits: `idempotent` (2)
- Common required input members in this group: `id`, `projectName`, `sessionId`, `spaceName`

### Verify

- Operations: `VerifySession`
- Traits: `readonly` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAccessToken` | `PUT /v1/accessTokens` | - | `name` | - | `CreateAccessTokenResponse` | - | Creates a personal access token (PAT) for the current user. A personal access token (PAT) is similar to a password. |
| `CreateDevEnvironment` | `PUT /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments` | `idempotent` | `instanceType`, `persistentStorage`, `projectName`, `spaceName` | - | `CreateDevEnvironmentResponse` | - | Creates a Dev Environment in Amazon CodeCatalyst, a cloud-based development environment that you can use to quickly work on the code stored in the source repositories of your project. When created in the Amazon CodeCatalyst console, by default a Dev... |
| `CreateProject` | `PUT /v1/spaces/{spaceName}/projects` | `idempotent` | `displayName`, `spaceName` | - | `CreateProjectResponse` | - | Creates a project in a specified space. |
| `CreateSourceRepository` | `PUT /v1/spaces/{spaceName}/projects/{projectName}/sourceRepositories/{name}` | `idempotent` | `name`, `projectName`, `spaceName` | - | `CreateSourceRepositoryResponse` | - | Creates an empty Git-based source repository in a specified project. The repository is created with an initial empty commit with a default branch named `main`. |
| `CreateSourceRepositoryBranch` | `PUT /v1/spaces/{spaceName}/projects/{projectName}/sourceRepositories/{sourceRepositoryName}/branches/{name}` | `idempotent` | `name`, `projectName`, `sourceRepositoryName`, `spaceName` | - | `CreateSourceRepositoryBranchResponse` | - | Creates a branch in a specified source repository in Amazon CodeCatalyst. This API only creates a branch in a source repository hosted in Amazon CodeCatalyst. |
| `DeleteAccessToken` | `DELETE /v1/accessTokens/{id}` | `idempotent` | `id` | - | `DeleteAccessTokenResponse` | - | Deletes a specified personal access token (PAT). A personal access token can only be deleted by the user who created it. |
| `DeleteDevEnvironment` | `DELETE /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{id}` | `idempotent` | `id`, `projectName`, `spaceName` | - | `DeleteDevEnvironmentResponse` | - | Deletes a Dev Environment. |
| `DeleteProject` | `DELETE /v1/spaces/{spaceName}/projects/{name}` | `idempotent` | `name`, `spaceName` | - | `DeleteProjectResponse` | - | Deletes a project in a space. |
| `DeleteSourceRepository` | `DELETE /v1/spaces/{spaceName}/projects/{projectName}/sourceRepositories/{name}` | `idempotent` | `name`, `projectName`, `spaceName` | - | `DeleteSourceRepositoryResponse` | - | Deletes a source repository in Amazon CodeCatalyst. You cannot use this API to delete a linked repository. |
| `DeleteSpace` | `DELETE /v1/spaces/{name}` | `idempotent` | `name` | - | `DeleteSpaceResponse` | - | Deletes a space. Deleting a space cannot be undone. |
| `GetDevEnvironment` | `GET /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{id}` | `readonly` | `id`, `projectName`, `spaceName` | - | `GetDevEnvironmentResponse` | - | Returns information about a Dev Environment for a source repository in a project. Dev Environments are specific to the user who creates them. |
| `GetProject` | `GET /v1/spaces/{spaceName}/projects/{name}` | `readonly` | `name`, `spaceName` | - | `GetProjectResponse` | - | Returns information about a project. |
| `GetSourceRepository` | `GET /v1/spaces/{spaceName}/projects/{projectName}/sourceRepositories/{name}` | `readonly` | `name`, `projectName`, `spaceName` | - | `GetSourceRepositoryResponse` | - | Returns information about a source repository. |
| `GetSourceRepositoryCloneUrls` | `GET /v1/spaces/{spaceName}/projects/{projectName}/sourceRepositories/{sourceRepositoryName}/cloneUrls` | `readonly` | `projectName`, `sourceRepositoryName`, `spaceName` | - | `GetSourceRepositoryCloneUrlsResponse` | - | Returns information about the URLs that can be used with a Git client to clone a source repository. |
| `GetSpace` | `GET /v1/spaces/{name}` | `readonly` | `name` | - | `GetSpaceResponse` | - | Returns information about an space. |
| `GetSubscription` | `GET /v1/spaces/{spaceName}/subscription` | `readonly` | `spaceName` | - | `GetSubscriptionResponse` | - | Returns information about the Amazon Web Services account used for billing purposes and the billing plan for the space. |
| `GetUserDetails` | `GET /userDetails` | `readonly` | - | - | `GetUserDetailsResponse` | - | Returns information about a user. |
| `GetWorkflow` | `GET /v1/spaces/{spaceName}/projects/{projectName}/workflows/{id}` | `readonly` | `id`, `projectName`, `spaceName` | - | `GetWorkflowResponse` | - | Returns information about a workflow. |
| `GetWorkflowRun` | `GET /v1/spaces/{spaceName}/projects/{projectName}/workflowRuns/{id}` | `readonly` | `id`, `projectName`, `spaceName` | - | `GetWorkflowRunResponse` | - | Returns information about a specified run of a workflow. |
| `ListAccessTokens` | `POST /v1/accessTokens` | `readonly`, `paginated` | - | - | `ListAccessTokensResponse` | - | Lists all personal access tokens (PATs) associated with the user who calls the API. You can only list PATs associated with your Amazon Web Services Builder ID. |
| `ListDevEnvironmentSessions` | `POST /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{devEnvironmentId}/sessions` | `readonly`, `paginated` | `devEnvironmentId`, `projectName`, `spaceName` | - | `ListDevEnvironmentSessionsResponse` | - | Retrieves a list of active sessions for a Dev Environment in a project. |
| `ListDevEnvironments` | `POST /v1/spaces/{spaceName}/devEnvironments` | `readonly`, `paginated` | `spaceName` | - | `ListDevEnvironmentsResponse` | - | Retrieves a list of Dev Environments in a project. |
| `ListEventLogs` | `POST /v1/spaces/{spaceName}/eventLogs` | `readonly`, `paginated` | `endTime`, `spaceName`, `startTime` | - | `ListEventLogsResponse` | - | Retrieves a list of events that occurred during a specific time in a space. You can use these events to audit user and system activity in a space. |
| `ListProjects` | `POST /v1/spaces/{spaceName}/projects` | `readonly`, `paginated` | `spaceName` | - | `ListProjectsResponse` | - | Retrieves a list of projects. |
| `ListSourceRepositories` | `POST /v1/spaces/{spaceName}/projects/{projectName}/sourceRepositories` | `readonly`, `paginated` | `projectName`, `spaceName` | - | `ListSourceRepositoriesResponse` | - | Retrieves a list of source repositories in a project. |
| `ListSourceRepositoryBranches` | `POST /v1/spaces/{spaceName}/projects/{projectName}/sourceRepositories/{sourceRepositoryName}/branches` | `readonly`, `paginated` | `projectName`, `sourceRepositoryName`, `spaceName` | - | `ListSourceRepositoryBranchesResponse` | - | Retrieves a list of branches in a specified source repository. |
| `ListSpaces` | `POST /v1/spaces` | `readonly`, `paginated` | - | - | `ListSpacesResponse` | - | Retrieves a list of spaces. |
| `ListWorkflowRuns` | `POST /v1/spaces/{spaceName}/projects/{projectName}/workflowRuns` | `readonly`, `paginated` | `projectName`, `spaceName` | - | `ListWorkflowRunsResponse` | - | Retrieves a list of workflow runs of a specified workflow. |
| `ListWorkflows` | `POST /v1/spaces/{spaceName}/projects/{projectName}/workflows` | `readonly`, `paginated` | `projectName`, `spaceName` | - | `ListWorkflowsResponse` | - | Retrieves a list of workflows in a specified project. |
| `StartDevEnvironment` | `PUT /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{id}/start` | `idempotent` | `id`, `projectName`, `spaceName` | - | `StartDevEnvironmentResponse` | - | Starts a specified Dev Environment and puts it into an active state. |
| `StartDevEnvironmentSession` | `PUT /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{id}/session` | - | `id`, `projectName`, `sessionConfiguration`, `spaceName` | - | `StartDevEnvironmentSessionResponse` | - | Starts a session for a specified Dev Environment. |
| `StartWorkflowRun` | `PUT /v1/spaces/{spaceName}/projects/{projectName}/workflowRuns` | `idempotent`, `idempotency-token` | `projectName`, `spaceName`, `workflowId` | `clientToken` | `StartWorkflowRunResponse` | - | Begins a run of a specified workflow. |
| `StopDevEnvironment` | `PUT /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{id}/stop` | `idempotent` | `id`, `projectName`, `spaceName` | - | `StopDevEnvironmentResponse` | - | Pauses a specified Dev Environment and places it in a non-running state. Stopped Dev Environments do not consume compute minutes. |
| `StopDevEnvironmentSession` | `DELETE /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{id}/session/{sessionId}` | `idempotent` | `id`, `projectName`, `sessionId`, `spaceName` | - | `StopDevEnvironmentSessionResponse` | - | Stops a session for a specified Dev Environment. |
| `UpdateDevEnvironment` | `PATCH /v1/spaces/{spaceName}/projects/{projectName}/devEnvironments/{id}` | `idempotent` | `id`, `projectName`, `spaceName` | - | `UpdateDevEnvironmentResponse` | - | Changes one or more values for a Dev Environment. Updating certain values of the Dev Environment will cause a restart. |
| `UpdateProject` | `PATCH /v1/spaces/{spaceName}/projects/{name}` | `idempotent` | `name`, `spaceName` | - | `UpdateProjectResponse` | - | Changes one or more values for a project. |
| `UpdateSpace` | `PATCH /v1/spaces/{name}` | `idempotent` | `name` | - | `UpdateSpaceResponse` | - | Changes one or more values for a space. |
| `VerifySession` | `GET /session` | `readonly` | - | - | `VerifySessionResponse` | - | Verifies whether the calling user has a valid Amazon CodeCatalyst login and session. If successful, this returns the ID of the user in Amazon CodeCatalyst. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetUserDetails` | - | `id -> id`, `userName -> userName` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CreateAccessTokenRequest` | `structure` | `expiresTime`, `name` | - |
| `CreateAccessTokenResponse` | `structure` | `accessTokenId`, `expiresTime`, `name`, `secret` | - |
| `CreateDevEnvironmentRequest` | `structure` | `alias`, `clientToken`, `ides`, `inactivityTimeoutMinutes`, `instanceType`, `persistentStorage`, `projectName`, `repositories`, `spaceName`, `vpcConnectionName` | - |
| `CreateDevEnvironmentResponse` | `structure` | `id`, `projectName`, `spaceName`, `vpcConnectionName` | - |
| `CreateProjectRequest` | `structure` | `description`, `displayName`, `spaceName` | - |
| `CreateProjectResponse` | `structure` | `description`, `displayName`, `name`, `spaceName` | - |
| `CreateSourceRepositoryRequest` | `structure` | `description`, `name`, `projectName`, `spaceName` | - |
| `CreateSourceRepositoryResponse` | `structure` | `description`, `name`, `projectName`, `spaceName` | - |
| `CreateSourceRepositoryBranchRequest` | `structure` | `headCommitId`, `name`, `projectName`, `sourceRepositoryName`, `spaceName` | - |
| `CreateSourceRepositoryBranchResponse` | `structure` | `headCommitId`, `lastUpdatedTime`, `name`, `ref` | - |
| `DeleteAccessTokenRequest` | `structure` | `id` | - |
| `DeleteAccessTokenResponse` | `structure` | - | - |
| `DeleteDevEnvironmentRequest` | `structure` | `id`, `projectName`, `spaceName` | - |
| `DeleteDevEnvironmentResponse` | `structure` | `id`, `projectName`, `spaceName` | - |
| `DeleteProjectRequest` | `structure` | `name`, `spaceName` | - |
| `DeleteProjectResponse` | `structure` | `displayName`, `name`, `spaceName` | - |
| `DeleteSourceRepositoryRequest` | `structure` | `name`, `projectName`, `spaceName` | - |
| `DeleteSourceRepositoryResponse` | `structure` | `name`, `projectName`, `spaceName` | - |
| `DeleteSpaceRequest` | `structure` | `name` | - |
| `DeleteSpaceResponse` | `structure` | `displayName`, `name` | - |
| `GetDevEnvironmentRequest` | `structure` | `id`, `projectName`, `spaceName` | - |
| `GetDevEnvironmentResponse` | `structure` | `alias`, `creatorId`, `id`, `ides`, `inactivityTimeoutMinutes`, `instanceType`, `lastUpdatedTime`, `persistentStorage`, `projectName`, `repositories`, `spaceName`, `status`, ... (+2) | - |
| `GetProjectRequest` | `structure` | `name`, `spaceName` | - |
| `GetProjectResponse` | `structure` | `description`, `displayName`, `name`, `spaceName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
