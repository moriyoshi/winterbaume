# AWS IoT Core Device Advisor

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services IoT Core Device Advisor is a cloud-based, fully managed test capability for validating IoT devices during device software development. Device Advisor provides pre-built tests that you can use to validate IoT devices for reliable and secure connectivity with Amazon Web Services IoT Core before deploying devices to production. By using Device Advisor, you can confirm that your devices can connect to Amazon Web Services IoT Core, follow security best practices and, if applicable, receive software updates from IoT Device Management. You can also download signed qualification reports to submit to the Amazon Web Services Partner Network to get your device qualified for the Amazon Web Services Partner Device Catalog without the need to send your device in and wait for it to be tested.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS IoT Core Device Advisor resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS IoT Core Device Advisor workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Start` operation families, including `GetEndpoint`, `GetSuiteDefinition`, `GetSuiteRun`, `GetSuiteRunReport`, `ListSuiteDefinitions`, `ListSuiteRuns`.

## Service Identity and Protocol

- AWS model slug: `iotdeviceadvisor`
- AWS SDK for Rust slug: `iotdeviceadvisor`
- Model version: `2020-09-18`
- Model file: `vendor/api-models-aws/models/iotdeviceadvisor/service/2020-09-18/iotdeviceadvisor-2020-09-18.json`
- SDK ID: `IotDeviceAdvisor`
- Endpoint prefix: `api.iotdeviceadvisor`
- ARN namespace: `iotdeviceadvisor`
- CloudFormation name: `IotDeviceAdvisor`
- CloudTrail event source: `iotdeviceadvisor.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (3), `Create` (1), `Delete` (1), `Start` (1), `Stop` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateSuiteDefinition`, `DeleteSuiteDefinition`, `StartSuiteRun`, `StopSuiteRun`, `TagResource`, `UntagResource`, `UpdateSuiteDefinition`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEndpoint`, `GetSuiteDefinition`, `GetSuiteRun`, `GetSuiteRunReport`, `ListSuiteDefinitions`, `ListSuiteRuns`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetSuiteRunReport`, `StartSuiteRun`, `StopSuiteRun`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.

## Operation Groups

### Get

- Operations: `GetEndpoint`, `GetSuiteDefinition`, `GetSuiteRun`, `GetSuiteRunReport`
- Common required input members in this group: `suiteDefinitionId`, `suiteRunId`

### List

- Operations: `ListSuiteDefinitions`, `ListSuiteRuns`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreateSuiteDefinition`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `suiteDefinitionConfiguration`

### Delete

- Operations: `DeleteSuiteDefinition`
- Common required input members in this group: `suiteDefinitionId`

### Start

- Operations: `StartSuiteRun`
- Common required input members in this group: `suiteDefinitionId`, `suiteRunConfiguration`

### Stop

- Operations: `StopSuiteRun`
- Common required input members in this group: `suiteDefinitionId`, `suiteRunId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateSuiteDefinition`
- Common required input members in this group: `suiteDefinitionConfiguration`, `suiteDefinitionId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateSuiteDefinition` | `POST /suiteDefinitions` | `idempotency-token` | `suiteDefinitionConfiguration` | `clientToken` | `CreateSuiteDefinitionResponse` | `InternalServerException`, `ValidationException` | Creates a Device Advisor test suite. Requires permission to access the CreateSuiteDefinition action. |
| `DeleteSuiteDefinition` | `DELETE /suiteDefinitions/{suiteDefinitionId}` | - | `suiteDefinitionId` | - | `DeleteSuiteDefinitionResponse` | `InternalServerException`, `ValidationException` | Deletes a Device Advisor test suite. Requires permission to access the DeleteSuiteDefinition action. |
| `GetEndpoint` | `GET /endpoint` | - | - | - | `GetEndpointResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about an Device Advisor endpoint. |
| `GetSuiteDefinition` | `GET /suiteDefinitions/{suiteDefinitionId}` | - | `suiteDefinitionId` | - | `GetSuiteDefinitionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a Device Advisor test suite. Requires permission to access the GetSuiteDefinition action. |
| `GetSuiteRun` | `GET /suiteDefinitions/{suiteDefinitionId}/suiteRuns/{suiteRunId}` | - | `suiteDefinitionId`, `suiteRunId` | - | `GetSuiteRunResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a Device Advisor test suite run. Requires permission to access the GetSuiteRun action. |
| `GetSuiteRunReport` | `GET /suiteDefinitions/{suiteDefinitionId}/suiteRuns/{suiteRunId}/report` | - | `suiteDefinitionId`, `suiteRunId` | - | `GetSuiteRunReportResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets a report download link for a successful Device Advisor qualifying test suite run. Requires permission to access the GetSuiteRunReport action. |
| `ListSuiteDefinitions` | `GET /suiteDefinitions` | `paginated` | - | - | `ListSuiteDefinitionsResponse` | `InternalServerException`, `ValidationException` | Lists the Device Advisor test suites you have created. Requires permission to access the ListSuiteDefinitions action. |
| `ListSuiteRuns` | `GET /suiteRuns` | `paginated` | - | - | `ListSuiteRunsResponse` | `InternalServerException`, `ValidationException` | Lists runs of the specified Device Advisor test suite. You can list all runs of the test suite, or the runs of a specific version of the test suite. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags attached to an IoT Device Advisor resource. Requires permission to access the ListTagsForResource action. |
| `StartSuiteRun` | `POST /suiteDefinitions/{suiteDefinitionId}/suiteRuns` | - | `suiteDefinitionId`, `suiteRunConfiguration` | - | `StartSuiteRunResponse` | `ConflictException`, `InternalServerException`, `ValidationException` | Starts a Device Advisor test suite run. Requires permission to access the StartSuiteRun action. |
| `StopSuiteRun` | `POST /suiteDefinitions/{suiteDefinitionId}/suiteRuns/{suiteRunId}/stop` | - | `suiteDefinitionId`, `suiteRunId` | - | `StopSuiteRunResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Stops a Device Advisor test suite run that is currently running. Requires permission to access the StopSuiteRun action. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds to and modifies existing tags of an IoT Device Advisor resource. Requires permission to access the TagResource action. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from an IoT Device Advisor resource. Requires permission to access the UntagResource action. |
| `UpdateSuiteDefinition` | `PATCH /suiteDefinitions/{suiteDefinitionId}` | - | `suiteDefinitionConfiguration`, `suiteDefinitionId` | - | `UpdateSuiteDefinitionResponse` | `InternalServerException`, `ValidationException` | Updates a Device Advisor test suite. Requires permission to access the UpdateSuiteDefinition action. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | Sends an Internal Failure exception. |
| `ValidationException` | `structure` | `message` | Sends a validation exception. |
| `ResourceNotFoundException` | `structure` | `message` | Sends a Resource Not Found exception. |
| `CreateSuiteDefinitionRequest` | `structure` | `clientToken`, `suiteDefinitionConfiguration`, `tags` | - |
| `CreateSuiteDefinitionResponse` | `structure` | `createdAt`, `suiteDefinitionArn`, `suiteDefinitionId`, `suiteDefinitionName` | - |
| `DeleteSuiteDefinitionRequest` | `structure` | `suiteDefinitionId` | - |
| `DeleteSuiteDefinitionResponse` | `structure` | - | - |
| `GetEndpointRequest` | `structure` | `authenticationMethod`, `certificateArn`, `deviceRoleArn`, `thingArn` | - |
| `GetEndpointResponse` | `structure` | `endpoint` | - |
| `GetSuiteDefinitionRequest` | `structure` | `suiteDefinitionId`, `suiteDefinitionVersion` | - |
| `GetSuiteDefinitionResponse` | `structure` | `createdAt`, `lastModifiedAt`, `latestVersion`, `suiteDefinitionArn`, `suiteDefinitionConfiguration`, `suiteDefinitionId`, `suiteDefinitionVersion`, `tags` | - |
| `GetSuiteRunRequest` | `structure` | `suiteDefinitionId`, `suiteRunId` | - |
| `GetSuiteRunResponse` | `structure` | `endTime`, `errorReason`, `startTime`, `status`, `suiteDefinitionId`, `suiteDefinitionVersion`, `suiteRunArn`, `suiteRunConfiguration`, `suiteRunId`, `tags`, `testResult` | - |
| `GetSuiteRunReportRequest` | `structure` | `suiteDefinitionId`, `suiteRunId` | - |
| `GetSuiteRunReportResponse` | `structure` | `qualificationReportDownloadUrl` | - |
| `ListSuiteDefinitionsRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListSuiteDefinitionsResponse` | `structure` | `nextToken`, `suiteDefinitionInformationList` | - |
| `ListSuiteRunsRequest` | `structure` | `maxResults`, `nextToken`, `suiteDefinitionId`, `suiteDefinitionVersion` | - |
| `ListSuiteRunsResponse` | `structure` | `nextToken`, `suiteRunsList` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `StartSuiteRunRequest` | `structure` | `suiteDefinitionId`, `suiteDefinitionVersion`, `suiteRunConfiguration`, `tags` | - |
| `StartSuiteRunResponse` | `structure` | `createdAt`, `endpoint`, `suiteRunArn`, `suiteRunId` | - |
| `ConflictException` | `structure` | `message` | Sends a Conflict Exception. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
