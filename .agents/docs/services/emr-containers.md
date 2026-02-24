# Amazon EMR Containers

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EMR on EKS provides a deployment option for Amazon EMR that allows you to run open-source big data frameworks on Amazon Elastic Kubernetes Service (Amazon EKS). With this deployment option, you can focus on running analytics workloads while Amazon EMR on EKS builds, configures, and manages containers for open-source applications. For more information about Amazon EMR on EKS concepts and tasks, see What is Amazon EMR on EKS. Amazon EMR containers is the API name for Amazon EMR on EKS. The `emr-containers` prefix is used in the following scenarios: It is the prefix in the CLI commands for Amazon EMR on EKS.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon EMR Containers resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon EMR Containers workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Cancel` operation families, including `ListJobRuns`, `ListJobTemplates`, `ListManagedEndpoints`, `ListSecurityConfigurations`, `DescribeJobRun`, `DescribeJobTemplate`.

## Service Identity and Protocol

- AWS model slug: `emr-containers`
- AWS SDK for Rust slug: `emrcontainers`
- Model version: `2020-10-01`
- Model file: `vendor/api-models-aws/models/emr-containers/service/2020-10-01/emr-containers-2020-10-01.json`
- SDK ID: `EMR containers`
- Endpoint prefix: `emr-containers`
- ARN namespace: `emr-containers`
- CloudFormation name: `EMRcontainers`
- CloudTrail event source: `emrcontainers.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Describe` (5), `Create` (4), `Delete` (3), `Cancel` (1), `Get` (1), `Start` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelJobRun`, `CreateJobTemplate`, `CreateManagedEndpoint`, `CreateSecurityConfiguration`, `CreateVirtualCluster`, `DeleteJobTemplate`, `DeleteManagedEndpoint`, `DeleteVirtualCluster`, `StartJobRun`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeJobRun`, `DescribeJobTemplate`, `DescribeManagedEndpoint`, `DescribeSecurityConfiguration`, `DescribeVirtualCluster`, `GetManagedEndpointSessionCredentials`, `ListJobRuns`, `ListJobTemplates`, `ListManagedEndpoints`, `ListSecurityConfigurations`, `ListTagsForResource`, `ListVirtualClusters`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelJobRun`, `CreateJobTemplate`, `DeleteJobTemplate`, `DescribeJobRun`, `DescribeJobTemplate`, `ListJobRuns`, `ListJobTemplates`, `StartJobRun`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `ECS`, `EKS`, `STS`.

## Operation Groups

### List

- Operations: `ListJobRuns`, `ListJobTemplates`, `ListManagedEndpoints`, `ListSecurityConfigurations`, `ListTagsForResource`, `ListVirtualClusters`
- Traits: `paginated` (5)
- Common required input members in this group: `resourceArn`, `virtualClusterId`

### Describe

- Operations: `DescribeJobRun`, `DescribeJobTemplate`, `DescribeManagedEndpoint`, `DescribeSecurityConfiguration`, `DescribeVirtualCluster`
- Common required input members in this group: `id`, `virtualClusterId`

### Create

- Operations: `CreateJobTemplate`, `CreateManagedEndpoint`, `CreateSecurityConfiguration`, `CreateVirtualCluster`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `clientToken`, `containerProvider`, `executionRoleArn`, `jobTemplateData`, `name`, `releaseLabel`, `securityConfigurationData`, `type`, `virtualClusterId`

### Delete

- Operations: `DeleteJobTemplate`, `DeleteManagedEndpoint`, `DeleteVirtualCluster`
- Common required input members in this group: `id`, `virtualClusterId`

### Cancel

- Operations: `CancelJobRun`
- Common required input members in this group: `id`, `virtualClusterId`

### Get

- Operations: `GetManagedEndpointSessionCredentials`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `credentialType`, `endpointIdentifier`, `executionRoleArn`, `virtualClusterIdentifier`

### Start

- Operations: `StartJobRun`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `clientToken`, `virtualClusterId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelJobRun` | `DELETE /virtualclusters/{virtualClusterId}/jobruns/{id}` | - | `id`, `virtualClusterId` | - | `CancelJobRunResponse` | `InternalServerException`, `ValidationException` | Cancels a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS. |
| `CreateJobTemplate` | `POST /jobtemplates` | `idempotency-token` | `clientToken`, `jobTemplateData`, `name` | `clientToken` | `CreateJobTemplateResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a job template. Job template stores values of StartJobRun API request in a template and can be used to start a job run. |
| `CreateManagedEndpoint` | `POST /virtualclusters/{virtualClusterId}/endpoints` | `idempotency-token` | `clientToken`, `executionRoleArn`, `name`, `releaseLabel`, `type`, `virtualClusterId` | `clientToken` | `CreateManagedEndpointResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a managed endpoint. A managed endpoint is a gateway that connects Amazon EMR Studio to Amazon EMR on EKS so that Amazon EMR Studio can communicate with your virtual cluster. |
| `CreateSecurityConfiguration` | `POST /securityconfigurations` | `idempotency-token` | `clientToken`, `name`, `securityConfigurationData` | `clientToken` | `CreateSecurityConfigurationResponse` | `InternalServerException`, `ValidationException` | Creates a security configuration. Security configurations in Amazon EMR on EKS are templates for different security setups. |
| `CreateVirtualCluster` | `POST /virtualclusters` | `idempotency-token` | `clientToken`, `containerProvider`, `name` | `clientToken` | `CreateVirtualClusterResponse` | `EKSRequestThrottledException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. |
| `DeleteJobTemplate` | `DELETE /jobtemplates/{id}` | - | `id` | - | `DeleteJobTemplateResponse` | `InternalServerException`, `ValidationException` | Deletes a job template. Job template stores values of StartJobRun API request in a template and can be used to start a job run. |
| `DeleteManagedEndpoint` | `DELETE /virtualclusters/{virtualClusterId}/endpoints/{id}` | - | `id`, `virtualClusterId` | - | `DeleteManagedEndpointResponse` | `InternalServerException`, `ValidationException` | Deletes a managed endpoint. A managed endpoint is a gateway that connects Amazon EMR Studio to Amazon EMR on EKS so that Amazon EMR Studio can communicate with your virtual cluster. |
| `DeleteVirtualCluster` | `DELETE /virtualclusters/{id}` | - | `id` | - | `DeleteVirtualClusterResponse` | `InternalServerException`, `ValidationException` | Deletes a virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. |
| `DescribeJobRun` | `GET /virtualclusters/{virtualClusterId}/jobruns/{id}` | - | `id`, `virtualClusterId` | - | `DescribeJobRunResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays detailed information about a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS. |
| `DescribeJobTemplate` | `GET /jobtemplates/{id}` | - | `id` | - | `DescribeJobTemplateResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays detailed information about a specified job template. Job template stores values of StartJobRun API request in a template and can be used to start a job run. |
| `DescribeManagedEndpoint` | `GET /virtualclusters/{virtualClusterId}/endpoints/{id}` | - | `id`, `virtualClusterId` | - | `DescribeManagedEndpointResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays detailed information about a managed endpoint. A managed endpoint is a gateway that connects Amazon EMR Studio to Amazon EMR on EKS so that Amazon EMR Studio can communicate with your virtual cluster. |
| `DescribeSecurityConfiguration` | `GET /securityconfigurations/{id}` | - | `id` | - | `DescribeSecurityConfigurationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays detailed information about a specified security configuration. Security configurations in Amazon EMR on EKS are templates for different security setups. |
| `DescribeVirtualCluster` | `GET /virtualclusters/{id}` | - | `id` | - | `DescribeVirtualClusterResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays detailed information about a specified virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. |
| `GetManagedEndpointSessionCredentials` | `POST /virtualclusters/{virtualClusterIdentifier}/endpoints/{endpointIdentifier}/credentials` | `idempotency-token` | `credentialType`, `endpointIdentifier`, `executionRoleArn`, `virtualClusterIdentifier` | `clientToken` | `GetManagedEndpointSessionCredentialsResponse` | `InternalServerException`, `RequestThrottledException`, `ResourceNotFoundException`, `ValidationException` | Generate a session token to connect to a managed endpoint. |
| `ListJobRuns` | `GET /virtualclusters/{virtualClusterId}/jobruns` | `paginated` | `virtualClusterId` | - | `ListJobRunsResponse` | `InternalServerException`, `ValidationException` | Lists job runs based on a set of parameters. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS. |
| `ListJobTemplates` | `GET /jobtemplates` | `paginated` | - | - | `ListJobTemplatesResponse` | `InternalServerException`, `ValidationException` | Lists job templates based on a set of parameters. Job template stores values of StartJobRun API request in a template and can be used to start a job run. |
| `ListManagedEndpoints` | `GET /virtualclusters/{virtualClusterId}/endpoints` | `paginated` | `virtualClusterId` | - | `ListManagedEndpointsResponse` | `InternalServerException`, `ValidationException` | Lists managed endpoints based on a set of parameters. A managed endpoint is a gateway that connects Amazon EMR Studio to Amazon EMR on EKS so that Amazon EMR Studio can communicate with your virtual cluster. |
| `ListSecurityConfigurations` | `GET /securityconfigurations` | `paginated` | - | - | `ListSecurityConfigurationsResponse` | `InternalServerException`, `ValidationException` | Lists security configurations based on a set of parameters. Security configurations in Amazon EMR on EKS are templates for different security setups. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags assigned to the resources. |
| `ListVirtualClusters` | `GET /virtualclusters` | `paginated` | - | - | `ListVirtualClustersResponse` | `InternalServerException`, `ValidationException` | Lists information about the specified virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. |
| `StartJobRun` | `POST /virtualclusters/{virtualClusterId}/jobruns` | `idempotency-token` | `clientToken`, `virtualClusterId` | `clientToken` | `StartJobRunResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Starts a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Assigns tags to resources. A tag is a label that you assign to an Amazon Web Services resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from resources. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | This is an internal server exception. |
| `ValidationException` | `structure` | `message` | There are invalid parameters in the client request. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource was not found. |
| `CancelJobRunRequest` | `structure` | `id`, `virtualClusterId` | - |
| `CancelJobRunResponse` | `structure` | `id`, `virtualClusterId` | - |
| `CreateJobTemplateRequest` | `structure` | `clientToken`, `jobTemplateData`, `kmsKeyArn`, `name`, `tags` | - |
| `CreateJobTemplateResponse` | `structure` | `arn`, `createdAt`, `id`, `name` | - |
| `CreateManagedEndpointRequest` | `structure` | `certificateArn`, `clientToken`, `configurationOverrides`, `executionRoleArn`, `name`, `releaseLabel`, `tags`, `type`, `virtualClusterId` | - |
| `CreateManagedEndpointResponse` | `structure` | `arn`, `id`, `name`, `virtualClusterId` | - |
| `CreateSecurityConfigurationRequest` | `structure` | `clientToken`, `containerProvider`, `name`, `securityConfigurationData`, `tags` | - |
| `CreateSecurityConfigurationResponse` | `structure` | `arn`, `id`, `name` | - |
| `CreateVirtualClusterRequest` | `structure` | `clientToken`, `containerProvider`, `name`, `securityConfigurationId`, `tags` | - |
| `CreateVirtualClusterResponse` | `structure` | `arn`, `id`, `name` | - |
| `EKSRequestThrottledException` | `structure` | `message` | The request exceeded the Amazon EKS API operation limits. |
| `DeleteJobTemplateRequest` | `structure` | `id` | - |
| `DeleteJobTemplateResponse` | `structure` | `id` | - |
| `DeleteManagedEndpointRequest` | `structure` | `id`, `virtualClusterId` | - |
| `DeleteManagedEndpointResponse` | `structure` | `id`, `virtualClusterId` | - |
| `DeleteVirtualClusterRequest` | `structure` | `id` | - |
| `DeleteVirtualClusterResponse` | `structure` | `id` | - |
| `DescribeJobRunRequest` | `structure` | `id`, `virtualClusterId` | - |
| `DescribeJobRunResponse` | `structure` | `jobRun` | - |
| `DescribeJobTemplateRequest` | `structure` | `id` | - |
| `DescribeJobTemplateResponse` | `structure` | `jobTemplate` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
