# AWS CodeBuild

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

CodeBuild CodeBuild is a fully managed build service in the cloud. CodeBuild compiles your source code, runs unit tests, and produces artifacts that are ready to deploy. CodeBuild eliminates the need to provision, manage, and scale your own build servers. It provides prepackaged build environments for the most popular programming languages and build tools, such as Apache Maven, Gradle, and more. You can also fully customize build environments in CodeBuild to use your own build tools.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS CodeBuild resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS CodeBuild workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Batch`, `Delete`, `Start`, `Update` operation families, including `ListBuildBatches`, `ListBuildBatchesForProject`, `ListBuilds`, `ListBuildsForProject`, `BatchDeleteBuilds`, `BatchGetBuildBatches`.

## Service Identity and Protocol

- AWS model slug: `codebuild`
- AWS SDK for Rust slug: `codebuild`
- Model version: `2016-10-06`
- Model file: `vendor/api-models-aws/models/codebuild/service/2016-10-06/codebuild-2016-10-06.json`
- SDK ID: `CodeBuild`
- Endpoint prefix: `codebuild`
- ARN namespace: `codebuild`
- CloudFormation name: `CodeBuild`
- CloudTrail event source: `codebuild.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (16), `Batch` (9), `Delete` (8), `Start` (5), `Update` (5), `Create` (4), `Stop` (3), `Describe` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchDeleteBuilds`, `BatchGetBuildBatches`, `BatchGetBuilds`, `BatchGetCommandExecutions`, `BatchGetFleets`, `BatchGetProjects`, `BatchGetReportGroups`, `BatchGetReports`, `BatchGetSandboxes`, `CreateFleet`, `CreateProject`, `CreateReportGroup`, `CreateWebhook`, `DeleteBuildBatch`, `DeleteFleet`, `DeleteProject`, `DeleteReport`, `DeleteReportGroup`, `DeleteResourcePolicy`, `DeleteSourceCredentials`, ... (+16).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCodeCoverages`, `DescribeTestCases`, `GetReportGroupTrend`, `GetResourcePolicy`, `ListBuildBatches`, `ListBuildBatchesForProject`, `ListBuilds`, `ListBuildsForProject`, `ListCommandExecutionsForSandbox`, `ListCuratedEnvironmentImages`, `ListFleets`, `ListProjects`, `ListReportGroups`, `ListReports`, `ListReportsForReportGroup`, `ListSandboxes`, `ListSandboxesForProject`, `ListSharedProjects`, `ListSharedReportGroups`, `ListSourceCredentials`.
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchGetCommandExecutions`, `BatchGetReportGroups`, `BatchGetReports`, `CreateReportGroup`, `DeleteReport`, `DeleteReportGroup`, `GetReportGroupTrend`, `ImportSourceCredentials`, `ListCommandExecutionsForSandbox`, `ListReportGroups`, `ListReports`, `ListReportsForReportGroup`, `ListSharedReportGroups`, `StartBuild`, `StartBuildBatch`, `StartCommandExecution`, `StartSandbox`, `StartSandboxConnection`, `StopBuild`, `StopBuildBatch`, ... (+2).
- 58 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECR`, `Secrets Manager`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/codebuild/latest/userguide/view-build-details-phases.html
- https://docs.aws.amazon.com/codebuild/latest/userguide/build-spec-ref.html
- https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-env-vars.html

Research outcomes:
- CodeBuild builds proceed through ordered phases, and UPLOAD_ARTIFACTS is always attempted even if BUILD fails.
- By default, a source buildspec file is named `buildspec.yml` and is located at the source root, but projects and StartBuild overrides can point to another relative path or same-Region S3 ARN.
- Buildspec version 0.2 runs build commands in the same shell instance; version 0.1 runs each command in a separate shell instance.
- Buildspec phase on-failure actions include ABORT, CONTINUE, RETRY, retry counts, retry regex, and retry count plus regex. This option is not supported for Lambda compute or reserved capacity.
- `finally` commands run after phase commands even when a command in the commands block fails; the phase succeeds only if both commands and finally commands succeed.
- Environment variable precedence is StartBuild overrides first, project definition second, buildspec declarations last.
- Parameter Store and Secrets Manager variables replace existing environment variables and require service-role permissions. CodeBuild hides those values and AWS access key IDs in logs.
- Exported variables can change from install through post_build; after post_build ends, exported variable values cannot change.

Parity implications:
- Model projects, builds, phases, environment resolution, sources, buildspec parsing, artifacts, reports, cache, logs, and exported variables.
- Build execution should preserve phase ordering, shell-version behaviour, on-failure rules, finally blocks, artifact upload attempts, and environment precedence.
- Integration with SSM, Secrets Manager, S3, CloudWatch Logs, and CodePipeline-exported variables should be explicit.

## Current Network Resource Stub Semantics

CodeBuild currently has generated and snapshot fields for VPC configuration, but the implemented state does not create network resources.

- Project snapshots expose a `vpc_config` JSON slot, and current snapshot construction sets it to `None`.
- Builds and projects do not allocate ENIs, attach to subnets, or track security group membership.
- Networking-related Smithy fields are therefore effectively ignored by the implemented service state unless a future handler explicitly stores them.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListBuildBatches`, `ListBuildBatchesForProject`, `ListBuilds`, `ListBuildsForProject`, `ListCommandExecutionsForSandbox`, `ListCuratedEnvironmentImages`, `ListFleets`, `ListProjects`, `ListReportGroups`, `ListReports`, `ListReportsForReportGroup`, `ListSandboxes`, `ListSandboxesForProject`, `ListSharedProjects`, `ListSharedReportGroups`, `ListSourceCredentials`
- Traits: `paginated` (14)
- Common required input members in this group: `projectName`, `reportGroupArn`, `sandboxId`

### Batch

- Operations: `BatchDeleteBuilds`, `BatchGetBuildBatches`, `BatchGetBuilds`, `BatchGetCommandExecutions`, `BatchGetFleets`, `BatchGetProjects`, `BatchGetReportGroups`, `BatchGetReports`, `BatchGetSandboxes`
- Common required input members in this group: `commandExecutionIds`, `ids`, `names`, `reportArns`, `reportGroupArns`, `sandboxId`

### Delete

- Operations: `DeleteBuildBatch`, `DeleteFleet`, `DeleteProject`, `DeleteReport`, `DeleteReportGroup`, `DeleteResourcePolicy`, `DeleteSourceCredentials`, `DeleteWebhook`
- Common required input members in this group: `arn`, `id`, `name`, `projectName`, `resourceArn`

### Start

- Operations: `StartBuild`, `StartBuildBatch`, `StartCommandExecution`, `StartSandbox`, `StartSandboxConnection`
- Common required input members in this group: `command`, `projectName`, `sandboxId`

### Update

- Operations: `UpdateFleet`, `UpdateProject`, `UpdateProjectVisibility`, `UpdateReportGroup`, `UpdateWebhook`
- Common required input members in this group: `arn`, `name`, `projectArn`, `projectName`, `projectVisibility`

### Create

- Operations: `CreateFleet`, `CreateProject`, `CreateReportGroup`, `CreateWebhook`
- Common required input members in this group: `artifacts`, `baseCapacity`, `computeType`, `environment`, `environmentType`, `exportConfig`, `name`, `projectName`, `serviceRole`, `source`, `type`

### Stop

- Operations: `StopBuild`, `StopBuildBatch`, `StopSandbox`
- Common required input members in this group: `id`

### Describe

- Operations: `DescribeCodeCoverages`, `DescribeTestCases`
- Traits: `paginated` (2)
- Common required input members in this group: `reportArn`

### Get

- Operations: `GetReportGroupTrend`, `GetResourcePolicy`
- Common required input members in this group: `reportGroupArn`, `resourceArn`, `trendField`

### Retry

- Operations: `RetryBuild`, `RetryBuildBatch`

### Import

- Operations: `ImportSourceCredentials`
- Common required input members in this group: `authType`, `serverType`, `token`

### Invalidate

- Operations: `InvalidateProjectCache`
- Common required input members in this group: `projectName`

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: `policy`, `resourceArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchDeleteBuilds` | - | - | `ids` | - | `BatchDeleteBuildsOutput` | `InvalidInputException` | Deletes one or more builds. |
| `BatchGetBuildBatches` | - | - | `ids` | - | `BatchGetBuildBatchesOutput` | `InvalidInputException` | Retrieves information about one or more batch builds. |
| `BatchGetBuilds` | - | - | `ids` | - | `BatchGetBuildsOutput` | `InvalidInputException` | Gets information about one or more builds. |
| `BatchGetCommandExecutions` | - | - | `commandExecutionIds`, `sandboxId` | - | `BatchGetCommandExecutionsOutput` | `InvalidInputException` | Gets information about the command executions. |
| `BatchGetFleets` | - | - | `names` | - | `BatchGetFleetsOutput` | `InvalidInputException` | Gets information about one or more compute fleets. |
| `BatchGetProjects` | - | - | `names` | - | `BatchGetProjectsOutput` | `InvalidInputException` | Gets information about one or more build projects. |
| `BatchGetReportGroups` | - | - | `reportGroupArns` | - | `BatchGetReportGroupsOutput` | `InvalidInputException` | Returns an array of report groups. |
| `BatchGetReports` | - | - | `reportArns` | - | `BatchGetReportsOutput` | `InvalidInputException` | Returns an array of reports. |
| `BatchGetSandboxes` | - | - | `ids` | - | `BatchGetSandboxesOutput` | `InvalidInputException` | Gets information about the sandbox status. |
| `CreateFleet` | - | - | `baseCapacity`, `computeType`, `environmentType`, `name` | - | `CreateFleetOutput` | `AccountLimitExceededException`, `InvalidInputException`, `ResourceAlreadyExistsException` | Creates a compute fleet. |
| `CreateProject` | - | - | `artifacts`, `environment`, `name`, `serviceRole`, `source` | - | `CreateProjectOutput` | `AccountLimitExceededException`, `InvalidInputException`, `ResourceAlreadyExistsException` | Creates a build project. |
| `CreateReportGroup` | - | - | `exportConfig`, `name`, `type` | - | `CreateReportGroupOutput` | `AccountLimitExceededException`, `InvalidInputException`, `ResourceAlreadyExistsException` | Creates a report group. A report group contains a collection of reports. |
| `CreateWebhook` | - | - | `projectName` | - | `CreateWebhookOutput` | `InvalidInputException`, `OAuthProviderException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | For an existing CodeBuild build project that has its source code stored in a GitHub or Bitbucket repository, enables CodeBuild to start rebuilding the source code every time a code change is pushed to the repository. If you enable webhooks for an CodeBuild... |
| `DeleteBuildBatch` | - | - | `id` | - | `DeleteBuildBatchOutput` | `InvalidInputException` | Deletes a batch build. |
| `DeleteFleet` | - | - | `arn` | - | `DeleteFleetOutput` | `InvalidInputException` | Deletes a compute fleet. When you delete a compute fleet, its builds are not deleted. |
| `DeleteProject` | - | - | `name` | - | `DeleteProjectOutput` | `InvalidInputException` | Deletes a build project. When you delete a project, its builds are not deleted. |
| `DeleteReport` | - | - | `arn` | - | `DeleteReportOutput` | `InvalidInputException` | Deletes a report. |
| `DeleteReportGroup` | - | - | `arn` | - | `DeleteReportGroupOutput` | `InvalidInputException` | Deletes a report group. Before you delete a report group, you must delete its reports. |
| `DeleteResourcePolicy` | - | - | `resourceArn` | - | `DeleteResourcePolicyOutput` | `InvalidInputException` | Deletes a resource policy that is identified by its resource ARN. |
| `DeleteSourceCredentials` | - | - | `arn` | - | `DeleteSourceCredentialsOutput` | `InvalidInputException`, `ResourceNotFoundException` | Deletes a set of GitHub, GitHub Enterprise, or Bitbucket source credentials. |
| `DeleteWebhook` | - | - | `projectName` | - | `DeleteWebhookOutput` | `InvalidInputException`, `OAuthProviderException`, `ResourceNotFoundException` | For an existing CodeBuild build project that has its source code stored in a GitHub or Bitbucket repository, stops CodeBuild from rebuilding the source code every time a code change is pushed to the repository. |
| `DescribeCodeCoverages` | - | `paginated` | `reportArn` | - | `DescribeCodeCoveragesOutput` | `InvalidInputException` | Retrieves one or more code coverage reports. |
| `DescribeTestCases` | - | `paginated` | `reportArn` | - | `DescribeTestCasesOutput` | `InvalidInputException`, `ResourceNotFoundException` | Returns a list of details about test cases for a report. |
| `GetReportGroupTrend` | - | - | `reportGroupArn`, `trendField` | - | `GetReportGroupTrendOutput` | `InvalidInputException`, `ResourceNotFoundException` | Analyzes and accumulates test report values for the specified test reports. |
| `GetResourcePolicy` | - | - | `resourceArn` | - | `GetResourcePolicyOutput` | `InvalidInputException`, `ResourceNotFoundException` | Gets a resource policy that is identified by its resource ARN. |
| `ImportSourceCredentials` | - | - | `authType`, `serverType`, `token` | - | `ImportSourceCredentialsOutput` | `AccountLimitExceededException`, `InvalidInputException`, `ResourceAlreadyExistsException` | Imports the source repository credentials for an CodeBuild project that has its source code stored in a GitHub, GitHub Enterprise, GitLab, GitLab Self Managed, or Bitbucket repository. |
| `InvalidateProjectCache` | - | - | `projectName` | - | `InvalidateProjectCacheOutput` | `InvalidInputException`, `ResourceNotFoundException` | Resets the cache for a project. |
| `ListBuildBatches` | - | `paginated` | - | - | `ListBuildBatchesOutput` | `InvalidInputException` | Retrieves the identifiers of your build batches in the current region. |
| `ListBuildBatchesForProject` | - | `paginated` | - | - | `ListBuildBatchesForProjectOutput` | `InvalidInputException`, `ResourceNotFoundException` | Retrieves the identifiers of the build batches for a specific project. |
| `ListBuilds` | - | `paginated` | - | - | `ListBuildsOutput` | `InvalidInputException` | Gets a list of build IDs, with each build ID representing a single build. |
| `ListBuildsForProject` | - | `paginated` | `projectName` | - | `ListBuildsForProjectOutput` | `InvalidInputException`, `ResourceNotFoundException` | Gets a list of build identifiers for the specified build project, with each build identifier representing a single build. |
| `ListCommandExecutionsForSandbox` | - | `paginated` | `sandboxId` | - | `ListCommandExecutionsForSandboxOutput` | `InvalidInputException`, `ResourceNotFoundException` | Gets a list of command executions for a sandbox. |
| `ListCuratedEnvironmentImages` | - | - | - | - | `ListCuratedEnvironmentImagesOutput` | - | Gets information about Docker images that are managed by CodeBuild. |
| `ListFleets` | - | `paginated` | - | - | `ListFleetsOutput` | `InvalidInputException` | Gets a list of compute fleet names with each compute fleet name representing a single compute fleet. |
| `ListProjects` | - | `paginated` | - | - | `ListProjectsOutput` | `InvalidInputException` | Gets a list of build project names, with each build project name representing a single build project. |
| `ListReportGroups` | - | `paginated` | - | - | `ListReportGroupsOutput` | `InvalidInputException` | Gets a list ARNs for the report groups in the current Amazon Web Services account. |
| `ListReports` | - | `paginated` | - | - | `ListReportsOutput` | `InvalidInputException` | Returns a list of ARNs for the reports in the current Amazon Web Services account. |
| `ListReportsForReportGroup` | - | `paginated` | `reportGroupArn` | - | `ListReportsForReportGroupOutput` | `InvalidInputException`, `ResourceNotFoundException` | Returns a list of ARNs for the reports that belong to a `ReportGroup`. |
| `ListSandboxes` | - | `paginated` | - | - | `ListSandboxesOutput` | `InvalidInputException` | Gets a list of sandboxes. |
| `ListSandboxesForProject` | - | `paginated` | `projectName` | - | `ListSandboxesForProjectOutput` | `InvalidInputException`, `ResourceNotFoundException` | Gets a list of sandboxes for a given project. |
| `ListSharedProjects` | - | `paginated` | - | - | `ListSharedProjectsOutput` | `InvalidInputException` | Gets a list of projects that are shared with other Amazon Web Services accounts or users. |
| `ListSharedReportGroups` | - | `paginated` | - | - | `ListSharedReportGroupsOutput` | `InvalidInputException` | Gets a list of report groups that are shared with other Amazon Web Services accounts or users. |
| `ListSourceCredentials` | - | - | - | - | `ListSourceCredentialsOutput` | `InvalidInputException` | Returns a list of `SourceCredentialsInfo` objects. |
| `PutResourcePolicy` | - | - | `policy`, `resourceArn` | - | `PutResourcePolicyOutput` | `InvalidInputException`, `ResourceNotFoundException` | Stores a resource policy for the ARN of a `Project` or `ReportGroup` object. |
| `RetryBuild` | - | - | - | - | `RetryBuildOutput` | `AccountLimitExceededException`, `InvalidInputException`, `ResourceNotFoundException` | Restarts a build. |
| `RetryBuildBatch` | - | - | - | - | `RetryBuildBatchOutput` | `InvalidInputException`, `ResourceNotFoundException` | Restarts a failed batch build. Only batch builds that have failed can be retried. |
| `StartBuild` | - | - | `projectName` | - | `StartBuildOutput` | `AccountLimitExceededException`, `InvalidInputException`, `ResourceNotFoundException` | Starts running a build with the settings defined in the project. These setting include: how to run a build, where to get the source code, which build environment to use, which build commands to run, and where to store the build output. |
| `StartBuildBatch` | - | - | `projectName` | - | `StartBuildBatchOutput` | `InvalidInputException`, `ResourceNotFoundException` | Starts a batch build for a project. |
| `StartCommandExecution` | - | - | `command`, `sandboxId` | - | `StartCommandExecutionOutput` | `InvalidInputException`, `ResourceNotFoundException` | Starts a command execution. |
| `StartSandbox` | - | - | - | - | `StartSandboxOutput` | `AccountSuspendedException`, `InvalidInputException`, `ResourceNotFoundException` | Starts a sandbox. |
| `StartSandboxConnection` | - | - | `sandboxId` | - | `StartSandboxConnectionOutput` | `InvalidInputException`, `ResourceNotFoundException` | Starts a sandbox connection. |
| `StopBuild` | - | - | `id` | - | `StopBuildOutput` | `InvalidInputException`, `ResourceNotFoundException` | Attempts to stop running a build. |
| `StopBuildBatch` | - | - | `id` | - | `StopBuildBatchOutput` | `InvalidInputException`, `ResourceNotFoundException` | Stops a running batch build. |
| `StopSandbox` | - | - | `id` | - | `StopSandboxOutput` | `InvalidInputException`, `ResourceNotFoundException` | Stops a sandbox. |
| `UpdateFleet` | - | - | `arn` | - | `UpdateFleetOutput` | `AccountLimitExceededException`, `InvalidInputException`, `ResourceNotFoundException` | Updates a compute fleet. |
| `UpdateProject` | - | - | `name` | - | `UpdateProjectOutput` | `InvalidInputException`, `ResourceNotFoundException` | Changes the settings of a build project. |
| `UpdateProjectVisibility` | - | - | `projectArn`, `projectVisibility` | - | `UpdateProjectVisibilityOutput` | `InvalidInputException`, `ResourceNotFoundException` | Changes the public visibility for a project. The project's build results, logs, and artifacts are available to the general public. |
| `UpdateReportGroup` | - | - | `arn` | - | `UpdateReportGroupOutput` | `InvalidInputException`, `ResourceNotFoundException` | Updates a report group. |
| `UpdateWebhook` | - | - | `projectName` | - | `UpdateWebhookOutput` | `InvalidInputException`, `OAuthProviderException`, `ResourceNotFoundException` | Updates the webhook associated with an CodeBuild build project. If you use Bitbucket for your repository, `rotateSecret` is ignored. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInputException` | `structure` | `message` | The input value that was provided is not valid. |
| `ResourceNotFoundException` | `structure` | `message` | The specified Amazon Web Services resource cannot be found. |
| `AccountLimitExceededException` | `structure` | `message` | An Amazon Web Services service limit was exceeded for the calling Amazon Web Services account. |
| `ResourceAlreadyExistsException` | `structure` | `message` | The specified Amazon Web Services resource cannot be created, because an Amazon Web Services resource with the same settings already exists. |
| `OAuthProviderException` | `structure` | `message` | There was a problem with the underlying OAuth provider. |
| `BatchDeleteBuildsInput` | `structure` | `ids` | - |
| `BatchDeleteBuildsOutput` | `structure` | `buildsDeleted`, `buildsNotDeleted` | - |
| `BatchGetBuildBatchesInput` | `structure` | `ids` | - |
| `BatchGetBuildBatchesOutput` | `structure` | `buildBatches`, `buildBatchesNotFound` | - |
| `BatchGetBuildsInput` | `structure` | `ids` | - |
| `BatchGetBuildsOutput` | `structure` | `builds`, `buildsNotFound` | - |
| `BatchGetCommandExecutionsInput` | `structure` | `commandExecutionIds`, `sandboxId` | - |
| `BatchGetCommandExecutionsOutput` | `structure` | `commandExecutions`, `commandExecutionsNotFound` | - |
| `BatchGetFleetsInput` | `structure` | `names` | - |
| `BatchGetFleetsOutput` | `structure` | `fleets`, `fleetsNotFound` | - |
| `BatchGetProjectsInput` | `structure` | `names` | - |
| `BatchGetProjectsOutput` | `structure` | `projects`, `projectsNotFound` | - |
| `BatchGetReportGroupsInput` | `structure` | `reportGroupArns` | - |
| `BatchGetReportGroupsOutput` | `structure` | `reportGroups`, `reportGroupsNotFound` | - |
| `BatchGetReportsInput` | `structure` | `reportArns` | - |
| `BatchGetReportsOutput` | `structure` | `reports`, `reportsNotFound` | - |
| `BatchGetSandboxesInput` | `structure` | `ids` | - |
| `BatchGetSandboxesOutput` | `structure` | `sandboxes`, `sandboxesNotFound` | - |
| `CreateFleetInput` | `structure` | `baseCapacity`, `computeConfiguration`, `computeType`, `environmentType`, `fleetServiceRole`, `imageId`, `name`, `overflowBehavior`, `proxyConfiguration`, `scalingConfiguration`, `tags`, `vpcConfig` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
