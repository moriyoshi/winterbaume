# AWS Amplify

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amplify enables developers to develop and deploy cloud-powered mobile and web apps. Amplify Hosting provides a continuous delivery and hosting service for web applications. For more information, see the Amplify Hosting User Guide. The Amplify Framework is a comprehensive set of SDKs, libraries, tools, and documentation for client app development. For more information, see the Amplify Framework.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Amplify where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Amplify by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Amplify resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: create and manage Amplify apps, branches, webhooks, backend environments, jobs, domains, and deployment artifacts.
- From the operation surface: model hosting workflows for connected repositories, manual deployments, preview branches, domain association, and tag-based app administration.

## Service Identity and Protocol

- AWS model slug: `amplify`
- AWS SDK for Rust slug: `amplify`
- Model version: `2017-07-25`
- Model file: `vendor/api-models-aws/models/amplify/service/2017-07-25/amplify-2017-07-25.json`
- SDK ID: `Amplify`
- Endpoint prefix: `amplify`
- ARN namespace: `amplify`
- CloudFormation name: `Amplify`
- CloudTrail event source: `amplify.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Get` (7), `Create` (6), `Delete` (6), `Update` (4), `Start` (2), `Generate` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApp`, `CreateBackendEnvironment`, `CreateBranch`, `CreateDeployment`, `CreateDomainAssociation`, `CreateWebhook`, `DeleteApp`, `DeleteBackendEnvironment`, `DeleteBranch`, `DeleteDomainAssociation`, `DeleteJob`, `DeleteWebhook`, `StartDeployment`, `StartJob`, `StopJob`, `TagResource`, `UntagResource`, `UpdateApp`, `UpdateBranch`, `UpdateDomainAssociation`, ... (+1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApp`, `GetArtifactUrl`, `GetBackendEnvironment`, `GetBranch`, `GetDomainAssociation`, `GetJob`, `GetWebhook`, `ListApps`, `ListArtifacts`, `ListBackendEnvironments`, `ListBranches`, `ListDomainAssociations`, `ListJobs`, `ListTagsForResource`, `ListWebhooks`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteJob`, `GetJob`, `ListJobs`, `StartDeployment`, `StartJob`, `StopJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 37 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`, `ECR`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/amplify/latest/userguide/advanced-deploy-features-chapter.html
- https://docs.aws.amazon.com/amplify/latest/userguide/build-settings.html
- https://docs.aws.amazon.com/amplify/latest/userguide/environment-variables.html

Research outcomes:
- Amplify Hosting organises hosted applications as apps with branches, builds, deployments, domains, redirects, environment variables, and optional backend linkage.
- Git-connected apps use repository branches and build settings from the console or `amplify.yml`.
- Environment variables can configure frontend frameworks, build behaviour, secrets, diff deployments, and backend-related identifiers.
- Amplify supports branch protections, pull request previews, direct deployments, and end-to-end test integration.
- Server-side rendered apps require framework-specific support and can use IAM roles for CloudWatch Logs.
- Build and deployment state is asynchronous and branch-specific.

Parity implications:
- Model apps, branches, jobs, deployments, domains, webhooks, environment variables, build specs, and backend association separately.
- StartJob-style deployment APIs should create branch-scoped asynchronous jobs with status transitions.
- Domain and branch mapping should be separate from build artefact state.

## Operation Groups

### List

- Operations: `ListApps`, `ListArtifacts`, `ListBackendEnvironments`, `ListBranches`, `ListDomainAssociations`, `ListJobs`, `ListTagsForResource`, `ListWebhooks`
- Traits: `paginated` (4)
- Common required input members in this group: `appId`, `branchName`, `jobId`, `resourceArn`

### Get

- Operations: `GetApp`, `GetArtifactUrl`, `GetBackendEnvironment`, `GetBranch`, `GetDomainAssociation`, `GetJob`, `GetWebhook`
- Common required input members in this group: `appId`, `artifactId`, `branchName`, `domainName`, `environmentName`, `jobId`, `webhookId`

### Create

- Operations: `CreateApp`, `CreateBackendEnvironment`, `CreateBranch`, `CreateDeployment`, `CreateDomainAssociation`, `CreateWebhook`
- Common required input members in this group: `appId`, `branchName`, `domainName`, `environmentName`, `name`, `subDomainSettings`

### Delete

- Operations: `DeleteApp`, `DeleteBackendEnvironment`, `DeleteBranch`, `DeleteDomainAssociation`, `DeleteJob`, `DeleteWebhook`
- Common required input members in this group: `appId`, `branchName`, `domainName`, `environmentName`, `jobId`, `webhookId`

### Update

- Operations: `UpdateApp`, `UpdateBranch`, `UpdateDomainAssociation`, `UpdateWebhook`
- Common required input members in this group: `appId`, `branchName`, `domainName`, `webhookId`

### Start

- Operations: `StartDeployment`, `StartJob`
- Common required input members in this group: `appId`, `branchName`, `jobType`

### Generate

- Operations: `GenerateAccessLogs`
- Common required input members in this group: `appId`, `domainName`

### Stop

- Operations: `StopJob`
- Common required input members in this group: `appId`, `branchName`, `jobId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApp` | `POST /apps` | - | `name` | - | `CreateAppResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `LimitExceededException`, `UnauthorizedException` | Creates a new Amplify app. |
| `CreateBackendEnvironment` | `POST /apps/{appId}/backendenvironments` | - | `appId`, `environmentName` | - | `CreateBackendEnvironmentResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Creates a new backend environment for an Amplify app. This API is available only to Amplify Gen 1 applications where the backend is created using Amplify Studio or the Amplify command line interface (CLI). |
| `CreateBranch` | `POST /apps/{appId}/branches` | - | `appId`, `branchName` | - | `CreateBranchResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Creates a new branch for an Amplify app. |
| `CreateDeployment` | `POST /apps/{appId}/branches/{branchName}/deployments` | - | `appId`, `branchName` | - | `CreateDeploymentResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `UnauthorizedException` | Creates a deployment for a manually deployed Amplify app. Manually deployed apps are not connected to a Git repository. |
| `CreateDomainAssociation` | `POST /apps/{appId}/domains` | - | `appId`, `domainName`, `subDomainSettings` | - | `CreateDomainAssociationResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Creates a new domain association for an Amplify app. This action associates a custom domain with the Amplify app |
| `CreateWebhook` | `POST /apps/{appId}/webhooks` | - | `appId`, `branchName` | - | `CreateWebhookResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Creates a new webhook on an Amplify app. |
| `DeleteApp` | `DELETE /apps/{appId}` | - | `appId` | - | `DeleteAppResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes an existing Amplify app specified by an app ID. |
| `DeleteBackendEnvironment` | `DELETE /apps/{appId}/backendenvironments/{environmentName}` | - | `appId`, `environmentName` | - | `DeleteBackendEnvironmentResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a backend environment for an Amplify app. This API is available only to Amplify Gen 1 applications where the backend is created using Amplify Studio or the Amplify command line interface (CLI). |
| `DeleteBranch` | `DELETE /apps/{appId}/branches/{branchName}` | - | `appId`, `branchName` | - | `DeleteBranchResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a branch for an Amplify app. |
| `DeleteDomainAssociation` | `DELETE /apps/{appId}/domains/{domainName}` | - | `appId`, `domainName` | - | `DeleteDomainAssociationResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a domain association for an Amplify app. |
| `DeleteJob` | `DELETE /apps/{appId}/branches/{branchName}/jobs/{jobId}` | - | `appId`, `branchName`, `jobId` | - | `DeleteJobResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Deletes a job for a branch of an Amplify app. |
| `DeleteWebhook` | `DELETE /webhooks/{webhookId}` | - | `webhookId` | - | `DeleteWebhookResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Deletes a webhook. |
| `GenerateAccessLogs` | `POST /apps/{appId}/accesslogs` | - | `appId`, `domainName` | - | `GenerateAccessLogsResult` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Returns the website access logs for a specific time range using a presigned URL. |
| `GetApp` | `GET /apps/{appId}` | - | `appId` | - | `GetAppResult` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Returns an existing Amplify app specified by an app ID. |
| `GetArtifactUrl` | `GET /artifacts/{artifactId}` | - | `artifactId` | - | `GetArtifactUrlResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Returns the artifact info that corresponds to an artifact id. |
| `GetBackendEnvironment` | `GET /apps/{appId}/backendenvironments/{environmentName}` | - | `appId`, `environmentName` | - | `GetBackendEnvironmentResult` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Returns a backend environment for an Amplify app. This API is available only to Amplify Gen 1 applications where the backend is created using Amplify Studio or the Amplify command line interface (CLI). |
| `GetBranch` | `GET /apps/{appId}/branches/{branchName}` | - | `appId`, `branchName` | - | `GetBranchResult` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Returns a branch for an Amplify app. |
| `GetDomainAssociation` | `GET /apps/{appId}/domains/{domainName}` | - | `appId`, `domainName` | - | `GetDomainAssociationResult` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Returns the domain information for an Amplify app. |
| `GetJob` | `GET /apps/{appId}/branches/{branchName}/jobs/{jobId}` | - | `appId`, `branchName`, `jobId` | - | `GetJobResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Returns a job for a branch of an Amplify app. |
| `GetWebhook` | `GET /webhooks/{webhookId}` | - | `webhookId` | - | `GetWebhookResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Returns the webhook information that corresponds to a specified webhook ID. |
| `ListApps` | `GET /apps` | `paginated` | - | - | `ListAppsResult` | `BadRequestException`, `InternalFailureException`, `UnauthorizedException` | Returns a list of the existing Amplify apps. |
| `ListArtifacts` | `GET /apps/{appId}/branches/{branchName}/jobs/{jobId}/artifacts` | - | `appId`, `branchName`, `jobId` | - | `ListArtifactsResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `UnauthorizedException` | Returns a list of end-to-end testing artifacts for a specified app, branch, and job. To return the build artifacts, use the GetJob API. |
| `ListBackendEnvironments` | `GET /apps/{appId}/backendenvironments` | - | `appId` | - | `ListBackendEnvironmentsResult` | `BadRequestException`, `InternalFailureException`, `UnauthorizedException` | Lists the backend environments for an Amplify app. This API is available only to Amplify Gen 1 applications where the backend is created using Amplify Studio or the Amplify command line interface (CLI). |
| `ListBranches` | `GET /apps/{appId}/branches` | `paginated` | `appId` | - | `ListBranchesResult` | `BadRequestException`, `InternalFailureException`, `UnauthorizedException` | Lists the branches of an Amplify app. |
| `ListDomainAssociations` | `GET /apps/{appId}/domains` | `paginated` | `appId` | - | `ListDomainAssociationsResult` | `BadRequestException`, `InternalFailureException`, `UnauthorizedException` | Returns the domain associations for an Amplify app. |
| `ListJobs` | `GET /apps/{appId}/branches/{branchName}/jobs` | `paginated` | `appId`, `branchName` | - | `ListJobsResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `UnauthorizedException` | Lists the jobs for a branch of an Amplify app. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalFailureException`, `ResourceNotFoundException` | Returns a list of tags for a specified Amazon Resource Name (ARN). |
| `ListWebhooks` | `GET /apps/{appId}/webhooks` | - | `appId` | - | `ListWebhooksResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `UnauthorizedException` | Returns a list of webhooks for an Amplify app. |
| `StartDeployment` | `POST /apps/{appId}/branches/{branchName}/deployments/start` | - | `appId`, `branchName` | - | `StartDeploymentResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Starts a deployment for a manually deployed app. Manually deployed apps are not connected to a Git repository. |
| `StartJob` | `POST /apps/{appId}/branches/{branchName}/jobs` | - | `appId`, `branchName`, `jobType` | - | `StartJobResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Starts a new job for a branch of an Amplify app. |
| `StopJob` | `DELETE /apps/{appId}/branches/{branchName}/jobs/{jobId}/stop` | - | `appId`, `branchName`, `jobId` | - | `StopJobResult` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Stops a job that is in progress for a branch of an Amplify app. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `InternalFailureException`, `ResourceNotFoundException` | Tags the resource with a tag key and value. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `InternalFailureException`, `ResourceNotFoundException` | Untags a resource with a specified Amazon Resource Name (ARN). |
| `UpdateApp` | `POST /apps/{appId}` | - | `appId` | - | `UpdateAppResult` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates an existing Amplify app. |
| `UpdateBranch` | `POST /apps/{appId}/branches/{branchName}` | - | `appId`, `branchName` | - | `UpdateBranchResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a branch for an Amplify app. |
| `UpdateDomainAssociation` | `POST /apps/{appId}/domains/{domainName}` | - | `appId`, `domainName` | - | `UpdateDomainAssociationResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a new domain association for an Amplify app. |
| `UpdateWebhook` | `POST /webhooks/{webhookId}` | - | `webhookId` | - | `UpdateWebhookResult` | `BadRequestException`, `DependentServiceFailureException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a webhook. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListApps` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListArtifacts` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListBackendEnvironments` | - | `environmentName -> environmentName`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListBranches` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDomainAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListJobs` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListWebhooks` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `message` | A request contains unexpected data. |
| `InternalFailureException` | `structure` | `message` | The service failed to perform an operation due to an internal issue. |
| `UnauthorizedException` | `structure` | `message` | An operation failed due to a lack of access. |
| `NotFoundException` | `structure` | `message` | An entity was not found during an operation. |
| `LimitExceededException` | `structure` | `message` | A resource could not be created because service quotas were exceeded. |
| `DependentServiceFailureException` | `structure` | `message` | An operation failed because a dependent service threw an exception. |
| `ResourceNotFoundException` | `structure` | `code`, `message` | An operation failed due to a non-existent resource. |
| `CreateAppRequest` | `structure` | `accessToken`, `autoBranchCreationConfig`, `autoBranchCreationPatterns`, `basicAuthCredentials`, `buildSpec`, `cacheConfig`, `computeRoleArn`, `customHeaders`, `customRules`, `description`, `enableAutoBranchCreation`, `enableBasicAuth`, ... (+10) | The request structure used to create apps in Amplify. |
| `CreateAppResult` | `structure` | `app` | - |
| `CreateBackendEnvironmentRequest` | `structure` | `appId`, `deploymentArtifacts`, `environmentName`, `stackName` | The request structure for the backend environment create request. |
| `CreateBackendEnvironmentResult` | `structure` | `backendEnvironment` | The result structure for the create backend environment request. |
| `CreateBranchRequest` | `structure` | `appId`, `backend`, `backendEnvironmentArn`, `basicAuthCredentials`, `branchName`, `buildSpec`, `computeRoleArn`, `description`, `displayName`, `enableAutoBuild`, `enableBasicAuth`, `enableNotification`, ... (+9) | The request structure for the create branch request. |
| `CreateBranchResult` | `structure` | `branch` | The result structure for create branch request. |
| `CreateDeploymentRequest` | `structure` | `appId`, `branchName`, `fileMap` | The request structure for the create a new deployment request. |
| `CreateDeploymentResult` | `structure` | `fileUploadUrls`, `jobId`, `zipUploadUrl` | The result structure for the create a new deployment request. |
| `CreateDomainAssociationRequest` | `structure` | `appId`, `autoSubDomainCreationPatterns`, `autoSubDomainIAMRole`, `certificateSettings`, `domainName`, `enableAutoSubDomain`, `subDomainSettings` | The request structure for the create domain association request. |
| `CreateDomainAssociationResult` | `structure` | `domainAssociation` | The result structure for the create domain association request. |
| `CreateWebhookRequest` | `structure` | `appId`, `branchName`, `description` | The request structure for the create webhook request. |
| `CreateWebhookResult` | `structure` | `webhook` | The result structure for the create webhook request. |
| `DeleteAppRequest` | `structure` | `appId` | Describes the request structure for the delete app request. |
| `DeleteAppResult` | `structure` | `app` | The result structure for the delete app request. |
| `DeleteBackendEnvironmentRequest` | `structure` | `appId`, `environmentName` | The request structure for the delete backend environment request. |
| `DeleteBackendEnvironmentResult` | `structure` | `backendEnvironment` | The result structure of the delete backend environment result. |
| `DeleteBranchRequest` | `structure` | `appId`, `branchName` | The request structure for the delete branch request. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
