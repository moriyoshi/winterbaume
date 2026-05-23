# AmazonMWAA

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Workflows for Apache Airflow This section contains the Amazon Managed Workflows for Apache Airflow (MWAA) API reference documentation. For more information, see What is Amazon MWAA?. Endpoints `api.airflow.{region}.amazonaws.com` - This endpoint is used for environment management. CreateEnvironment DeleteEnvironment GetEnvironment ListEnvironments ListTagsForResource TagResource UntagResource UpdateEnvironment `env.airflow.{region}.amazonaws.com` - This endpoint is used to operate the Airflow environment. CreateCliToken CreateWebLoginToken InvokeRestApi Regions For a list of supported regions, see Amazon MWAA endpoints and quotas in the Amazon Web Services General Reference .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AmazonMWAA workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `List`, `Delete`, `Get`, `Invoke` operation families, including `CreateCliToken`, `CreateEnvironment`, `CreateWebLoginToken`, `ListEnvironments`, `ListTagsForResource`, `DeleteEnvironment`.

## Service Identity and Protocol

- AWS model slug: `mwaa`
- AWS SDK for Rust slug: `mwaa`
- Model version: `2020-07-01`
- Model file: `vendor/api-models-aws/models/mwaa/service/2020-07-01/mwaa-2020-07-01.json`
- SDK ID: `MWAA`
- Endpoint prefix: `-`
- ARN namespace: `airflow`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (3), `List` (2), `Delete` (1), `Get` (1), `Invoke` (1), `Publish` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCliToken`, `CreateEnvironment`, `CreateWebLoginToken`, `DeleteEnvironment`, `TagResource`, `UntagResource`, `UpdateEnvironment`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEnvironment`, `ListEnvironments`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/mwaa/latest/userguide/what-is-mwaa.html
- https://docs.aws.amazon.com/mwaa/latest/userguide/using-mwaa.html
- https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-folder.html

Research outcomes:
- Amazon MWAA creates managed Apache Airflow environments with schedulers, workers, web server, metadata database, and supporting infrastructure.
- Environments reference an S3 bucket and DAG folder path. DAG updates are uploaded to S3 and synchronised into the Airflow environment.
- MWAA can auto-scale workers and configure webserver and worker capacity.
- Environment settings include Airflow version, environment class, network configuration, logging, Airflow configuration options, requirements, plugins, and startup scripts.
- DAG folder changes are periodically synchronised to scheduler containers.
- CloudWatch integrates with MWAA logs and metrics.
- Airflow UI access is controlled through IAM and MWAA web login mechanisms.

Parity implications:
- Model environments, source bucket/path versions, requirements/plugins/startup scripts, scheduler/worker/webserver configuration, logs, and update state separately.
- Environment updates should be asynchronous and should distinguish S3 content changes from control-plane configuration updates.
- DAG visibility should lag S3 updates according to sync behaviour.

## Operation Groups

### Create

- Operations: `CreateCliToken`, `CreateEnvironment`, `CreateWebLoginToken`
- Traits: `endpoint-bound` (3), `idempotent` (2)
- Common required input members in this group: `DagS3Path`, `ExecutionRoleArn`, `Name`, `NetworkConfiguration`, `SourceBucketArn`

### List

- Operations: `ListEnvironments`, `ListTagsForResource`
- Traits: `endpoint-bound` (2), `paginated` (1), `readonly` (2)
- Common required input members in this group: `ResourceArn`

### Delete

- Operations: `DeleteEnvironment`
- Traits: `endpoint-bound` (1), `idempotent` (1)
- Common required input members in this group: `Name`

### Get

- Operations: `GetEnvironment`
- Traits: `endpoint-bound` (1), `readonly` (1)
- Common required input members in this group: `Name`

### Invoke

- Operations: `InvokeRestApi`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `Method`, `Name`, `Path`

### Publish

- Operations: `PublishMetrics`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `EnvironmentName`, `MetricData`

### Tag

- Operations: `TagResource`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `endpoint-bound` (1), `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `tagKeys`

### Update

- Operations: `UpdateEnvironment`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `Name`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCliToken` | `POST /clitoken/{Name}` | `endpoint-bound` | `Name` | - | `CreateCliTokenResponse` | `ResourceNotFoundException` | Creates a CLI token for the Airflow CLI. To learn more, see Creating an Apache Airflow CLI token. |
| `CreateEnvironment` | `PUT /environments/{Name}` | `idempotent`, `endpoint-bound` | `DagS3Path`, `ExecutionRoleArn`, `Name`, `NetworkConfiguration`, `SourceBucketArn` | - | `CreateEnvironmentOutput` | `InternalServerException`, `ValidationException` | Creates an Amazon Managed Workflows for Apache Airflow (Amazon MWAA) environment. |
| `CreateWebLoginToken` | `POST /webtoken/{Name}` | `idempotent`, `endpoint-bound` | `Name` | - | `CreateWebLoginTokenResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a web login token for the Airflow Web UI. To learn more, see Creating an Apache Airflow web login token. |
| `DeleteEnvironment` | `DELETE /environments/{Name}` | `idempotent`, `endpoint-bound` | `Name` | - | `DeleteEnvironmentOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an Amazon Managed Workflows for Apache Airflow (Amazon MWAA) environment. |
| `GetEnvironment` | `GET /environments/{Name}` | `readonly`, `endpoint-bound` | `Name` | - | `GetEnvironmentOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes an Amazon Managed Workflows for Apache Airflow (MWAA) environment. |
| `InvokeRestApi` | `POST /restapi/{Name}` | `endpoint-bound` | `Method`, `Name`, `Path` | - | `InvokeRestApiResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `RestApiClientException`, `RestApiServerException`, `ValidationException` | Invokes the Apache Airflow REST API on the webserver with the specified inputs. To learn more, see Using the Apache Airflow REST API |
| `ListEnvironments` | `GET /environments` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListEnvironmentsOutput` | `InternalServerException`, `ValidationException` | Lists the Amazon Managed Workflows for Apache Airflow (MWAA) environments. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly`, `endpoint-bound` | `ResourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the key-value tag pairs associated to the Amazon Managed Workflows for Apache Airflow (MWAA) environment. For example, `"Environment": "Staging"`. |
| `PublishMetrics` | `POST /metrics/environments/{EnvironmentName}` | `endpoint-bound` | `EnvironmentName`, `MetricData` | - | `PublishMetricsOutput` | `InternalServerException`, `ValidationException` | Internal only . Publishes environment health metrics to Amazon CloudWatch. |
| `TagResource` | `POST /tags/{ResourceArn}` | `endpoint-bound` | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Associates key-value tag pairs to your Amazon Managed Workflows for Apache Airflow (MWAA) environment. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent`, `endpoint-bound` | `ResourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes key-value tag pairs associated to your Amazon Managed Workflows for Apache Airflow (MWAA) environment. For example, `"Environment": "Staging"`. |
| `UpdateEnvironment` | `PATCH /environments/{Name}` | `endpoint-bound` | `Name` | - | `UpdateEnvironmentOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates an Amazon Managed Workflows for Apache Airflow (MWAA) environment. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListEnvironments` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | InternalServerException: An internal error has occurred. |
| `ValidationException` | `structure` | `message` | ValidationException: The provided input is not valid. |
| `ResourceNotFoundException` | `structure` | `message` | ResourceNotFoundException: The resource is not available. |
| `AccessDeniedException` | `structure` | `Message` | Access to the Apache Airflow Web UI or CLI has been denied due to insufficient permissions. |
| `CreateCliTokenRequest` | `structure` | `Name` | - |
| `CreateCliTokenResponse` | `structure` | `CliToken`, `WebServerHostname` | - |
| `CreateEnvironmentInput` | `structure` | `AirflowConfigurationOptions`, `AirflowVersion`, `DagS3Path`, `EndpointManagement`, `EnvironmentClass`, `ExecutionRoleArn`, `KmsKey`, `LoggingConfiguration`, `MaxWebservers`, `MaxWorkers`, `MinWebservers`, `MinWorkers`, ... (+13) | This section contains the Amazon Managed Workflows for Apache Airflow (Amazon MWAA) API reference documentation to create an environment. |
| `CreateEnvironmentOutput` | `structure` | `Arn` | - |
| `CreateWebLoginTokenRequest` | `structure` | `Name` | - |
| `CreateWebLoginTokenResponse` | `structure` | `AirflowIdentity`, `IamIdentity`, `WebServerHostname`, `WebToken` | - |
| `DeleteEnvironmentInput` | `structure` | `Name` | - |
| `DeleteEnvironmentOutput` | `structure` | - | - |
| `GetEnvironmentInput` | `structure` | `Name` | - |
| `GetEnvironmentOutput` | `structure` | `Environment` | - |
| `InvokeRestApiRequest` | `structure` | `Body`, `Method`, `Name`, `Path`, `QueryParameters` | - |
| `InvokeRestApiResponse` | `structure` | `RestApiResponse`, `RestApiStatusCode` | - |
| `RestApiClientException` | `structure` | `RestApiResponse`, `RestApiStatusCode` | An exception indicating that a client-side error occurred during the Apache Airflow REST API call. |
| `RestApiServerException` | `structure` | `RestApiResponse`, `RestApiStatusCode` | An exception indicating that a server-side error occurred during the Apache Airflow REST API call. |
| `ListEnvironmentsInput` | `structure` | `MaxResults`, `NextToken` | - |
| `ListEnvironmentsOutput` | `structure` | `Environments`, `NextToken` | - |
| `ListTagsForResourceInput` | `structure` | `ResourceArn` | - |
| `ListTagsForResourceOutput` | `structure` | `Tags` | - |
| `PublishMetricsInput` | `structure` | `EnvironmentName`, `MetricData` | - |
| `PublishMetricsOutput` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
