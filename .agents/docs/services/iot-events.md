# AWS IoT Events

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS IoT Events monitors your equipment or device fleets for failures or changes in operation, and triggers actions when such events occur. You can use AWS IoT Events API operations to create, read, update, and delete inputs and detector models, and to list their versions.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS IoT Events by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AWS IoT Events workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Update` operation families, including `ListAlarmModelVersions`, `ListAlarmModels`, `ListDetectorModelVersions`, `ListDetectorModels`, `DescribeAlarmModel`, `DescribeDetectorModel`.

## Service Identity and Protocol

- AWS model slug: `iot-events`
- AWS SDK for Rust slug: `iotevents`
- Model version: `2018-07-27`
- Model file: `vendor/api-models-aws/models/iot-events/service/2018-07-27/iot-events-2018-07-27.json`
- SDK ID: `IoT Events`
- Endpoint prefix: `iotevents`
- ARN namespace: `iotevents`
- CloudFormation name: `IoTEvents`
- CloudTrail event source: `iotevents.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Describe` (5), `Create` (3), `Delete` (3), `Update` (3), `Get` (1), `Put` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAlarmModel`, `CreateDetectorModel`, `CreateInput`, `DeleteAlarmModel`, `DeleteDetectorModel`, `DeleteInput`, `PutLoggingOptions`, `StartDetectorModelAnalysis`, `TagResource`, `UntagResource`, `UpdateAlarmModel`, `UpdateDetectorModel`, `UpdateInput`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAlarmModel`, `DescribeDetectorModel`, `DescribeDetectorModelAnalysis`, `DescribeInput`, `DescribeLoggingOptions`, `GetDetectorModelAnalysisResults`, `ListAlarmModelVersions`, `ListAlarmModels`, `ListDetectorModelVersions`, `ListDetectorModels`, `ListInputRoutings`, `ListInputs`, `ListTagsForResource`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeDetectorModelAnalysis`, `GetDetectorModelAnalysisResults`, `StartDetectorModelAnalysis`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `CloudWatch`, `SNS`, `SQS`, `Lambda`, `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListAlarmModels`, `ListAlarmModelVersions`, `ListDetectorModels`, `ListDetectorModelVersions`, `ListInputRoutings`, `ListInputs`, `ListTagsForResource`
- Common required input members in this group: -

### Describe

- Operations: `DescribeAlarmModel`, `DescribeDetectorModel`, `DescribeDetectorModelAnalysis`, `DescribeInput`, `DescribeLoggingOptions`
- Common required input members in this group: -

### Create

- Operations: `CreateAlarmModel`, `CreateDetectorModel`, `CreateInput`
- Common required input members in this group: `roleArn`

### Delete

- Operations: `DeleteAlarmModel`, `DeleteDetectorModel`, `DeleteInput`
- Common required input members in this group: -

### Update

- Operations: `UpdateAlarmModel`, `UpdateDetectorModel`, `UpdateInput`
- Common required input members in this group: `roleArn`

### Get

- Operations: `GetDetectorModelAnalysisResults`
- Common required input members in this group: -

### Put

- Operations: `PutLoggingOptions`
- Common required input members in this group: -

### Start

- Operations: `StartDetectorModelAnalysis`
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
| `CreateAlarmModel` | `POST /alarm-models` | - | `alarmModelName`, `roleArn`, `alarmRule` | - | `CreateAlarmModelResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceUnavailableException`, `ThrottlingException` | Creates an alarm model to monitor an AWS IoT Events input attribute. You can use the alarm to get notified when the value is outside a specified range. For more information, see Create an alarm model in the AWS IoT E ... |
| `CreateDetectorModel` | `POST /detector-models` | - | `detectorModelName`, `detectorModelDefinition`, `roleArn` | - | `CreateDetectorModelResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceUnavailableException`, `ThrottlingException` | Creates a detector model. |
| `CreateInput` | `POST /inputs` | - | `inputName`, `inputDefinition` | - | `CreateInputResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ServiceUnavailableException`, `ThrottlingException` | Creates an input. |
| `DeleteAlarmModel` | `DELETE /alarm-models/{alarmModelName}` | - | `alarmModelName` | - | `DeleteAlarmModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes an alarm model. Any alarm instances that were created based on this alarm model are also deleted. This action can't be undone. |
| `DeleteDetectorModel` | `DELETE /detector-models/{detectorModelName}` | - | `detectorModelName` | - | `DeleteDetectorModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes a detector model. Any active instances of the detector model are also deleted. |
| `DeleteInput` | `DELETE /inputs/{inputName}` | - | `inputName` | - | `DeleteInputResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes an input. |
| `DescribeAlarmModel` | `GET /alarm-models/{alarmModelName}` | - | `alarmModelName` | - | `DescribeAlarmModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves information about an alarm model. If you don't specify a value for the alarmModelVersion parameter, the latest version is returned. |
| `DescribeDetectorModel` | `GET /detector-models/{detectorModelName}` | - | `detectorModelName` | - | `DescribeDetectorModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Describes a detector model. If the version parameter is not specified, information about the latest version is returned. |
| `DescribeDetectorModelAnalysis` | `GET /analysis/detector-models/{analysisId}` | - | `analysisId` | - | `DescribeDetectorModelAnalysisResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves runtime information about a detector model analysis. After AWS IoT Events starts analyzing your detector model, you have up to 24 hours to retrieve the analysis results. |
| `DescribeInput` | `GET /inputs/{inputName}` | - | `inputName` | - | `DescribeInputResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Describes an input. |
| `DescribeLoggingOptions` | `GET /logging` | - | - | - | `DescribeLoggingOptionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnsupportedOperationException` | Retrieves the current settings of the AWS IoT Events logging options. |
| `GetDetectorModelAnalysisResults` | `GET /analysis/detector-models/{analysisId}/results` | - | `analysisId` | - | `GetDetectorModelAnalysisResultsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves one or more analysis results of the detector model. After AWS IoT Events starts analyzing your detector model, you have up to 24 hours to retrieve the analysis results. |
| `ListAlarmModels` | `GET /alarm-models` | - | - | - | `ListAlarmModelsResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the alarm models that you created. The operation returns only the metadata associated with each alarm model. |
| `ListAlarmModelVersions` | `GET /alarm-models/{alarmModelName}/versions` | - | `alarmModelName` | - | `ListAlarmModelVersionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists all the versions of an alarm model. The operation returns only the metadata associated with each alarm model version. |
| `ListDetectorModels` | `GET /detector-models` | - | - | - | `ListDetectorModelsResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the detector models you have created. Only the metadata associated with each detector model is returned. |
| `ListDetectorModelVersions` | `GET /detector-models/{detectorModelName}/versions` | - | `detectorModelName` | - | `ListDetectorModelVersionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists all the versions of a detector model. Only the metadata associated with each detector model version is returned. |
| `ListInputRoutings` | `POST /input-routings` | - | `inputIdentifier` | - | `ListInputRoutingsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists one or more input routings. |
| `ListInputs` | `GET /inputs` | - | - | - | `ListInputsResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the inputs you have created. |
| `ListTagsForResource` | `GET /tags` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the tags (metadata) you have assigned to the resource. |
| `PutLoggingOptions` | `PUT /logging` | - | `loggingOptions` | - | `Unit` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceUnavailableException`, `ThrottlingException`, `UnsupportedOperationException` | Sets or updates the AWS IoT Events logging options. If you update the value of any loggingOptions field, it takes up to one minute for the change to take effect. If you change the policy attached to the role you spec ... |
| `StartDetectorModelAnalysis` | `POST /analysis/detector-models` | - | `detectorModelDefinition` | - | `StartDetectorModelAnalysisResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ServiceUnavailableException`, `ThrottlingException` | Performs an analysis of your detector model. For more information, see Troubleshooting a detector model in the AWS IoT Events Developer Guide . |
| `TagResource` | `POST /tags` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource. |
| `UntagResource` | `DELETE /tags` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Removes the given tags (metadata) from the resource. |
| `UpdateAlarmModel` | `POST /alarm-models/{alarmModelName}` | - | `alarmModelName`, `roleArn`, `alarmRule` | - | `UpdateAlarmModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates an alarm model. Any alarms that were created based on the previous version are deleted and then created again as new data arrives. |
| `UpdateDetectorModel` | `POST /detector-models/{detectorModelName}` | - | `detectorModelName`, `detectorModelDefinition`, `roleArn` | - | `UpdateDetectorModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates a detector model. Detectors (instances) spawned by the previous version are deleted and then re-created as new inputs arrive. |
| `UpdateInput` | `PUT /inputs/{inputName}` | - | `inputName`, `inputDefinition` | - | `UpdateInputResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates an input. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DescribeAlarmModel` | - | `alarmModelVersion -> version` | - | - |
| `DescribeDetectorModel` | - | `detectorModelVersion -> version` | - | - |
| `GetDetectorModelAnalysisResults` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAlarmModels` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAlarmModelVersions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDetectorModels` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDetectorModelVersions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListInputs` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListTagsForResource` | - | `resourceArn -> resourceArn` | - | - |
| `TagResource` | - | `resourceArn -> resourceArn` | - | - |
| `UntagResource` | - | `resourceArn -> resourceArn`, `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalFailureException` | `structure` | message | An internal failure occurred. |
| `InvalidRequestException` | `structure` | message | The request was invalid. |
| `LimitExceededException` | `structure` | message | A limit was exceeded. |
| `ResourceAlreadyExistsException` | `structure` | message, resourceId, resourceArn | The resource already exists. |
| `ResourceInUseException` | `structure` | message | The resource is in use. |
| `ResourceNotFoundException` | `structure` | message | The resource was not found. |
| `ServiceUnavailableException` | `structure` | message | The service is currently unavailable. |
| `ThrottlingException` | `structure` | message | The request could not be completed due to throttling. |
| `UnsupportedOperationException` | `structure` | message | The requested operation is not supported. |
| `CreateAlarmModelRequest` | `structure` | alarmModelName, alarmModelDescription, roleArn, tags, key, severity, alarmRule, alarmNotification, alarmEventActions, alarmCapabilities | - |
| `CreateAlarmModelResponse` | `structure` | creationTime, alarmModelArn, alarmModelVersion, lastUpdateTime, status | - |
| `CreateDetectorModelRequest` | `structure` | detectorModelName, detectorModelDefinition, detectorModelDescription, key, roleArn, tags, evaluationMethod | - |
| `CreateDetectorModelResponse` | `structure` | detectorModelConfiguration | - |
| `CreateInputRequest` | `structure` | inputName, inputDescription, inputDefinition, tags | - |
| `CreateInputResponse` | `structure` | inputConfiguration | - |
| `DeleteAlarmModelRequest` | `structure` | alarmModelName | - |
| `DeleteAlarmModelResponse` | `structure` | **empty (no members)** | - |
| `DeleteDetectorModelRequest` | `structure` | detectorModelName | - |
| `DeleteDetectorModelResponse` | `structure` | **empty (no members)** | - |
| `DeleteInputRequest` | `structure` | inputName | - |
| `DeleteInputResponse` | `structure` | **empty (no members)** | - |
| `DescribeAlarmModelRequest` | `structure` | alarmModelName, alarmModelVersion | - |
| `DescribeAlarmModelResponse` | `structure` | creationTime, alarmModelArn, alarmModelVersion, lastUpdateTime, status, statusMessage, alarmModelName, alarmModelDescription, roleArn, key, severity, alarmRule, ... (+3) | - |
| `DescribeDetectorModelRequest` | `structure` | detectorModelName, detectorModelVersion | - |
| `DescribeDetectorModelResponse` | `structure` | detectorModel | - |
| `DescribeDetectorModelAnalysisRequest` | `structure` | analysisId | - |
| `DescribeDetectorModelAnalysisResponse` | `structure` | status | - |
| `DescribeInputRequest` | `structure` | inputName | - |
| `DescribeInputResponse` | `structure` | input | - |
| `DescribeLoggingOptionsRequest` | `structure` | **empty (no members)** | - |
| `DescribeLoggingOptionsResponse` | `structure` | loggingOptions | - |
| `GetDetectorModelAnalysisResultsRequest` | `structure` | analysisId, nextToken, maxResults | - |
| `GetDetectorModelAnalysisResultsResponse` | `structure` | analysisResults, nextToken | - |
| `ListAlarmModelsRequest` | `structure` | nextToken, maxResults | - |
| `ListAlarmModelsResponse` | `structure` | alarmModelSummaries, nextToken | - |
| `ListAlarmModelVersionsRequest` | `structure` | alarmModelName, nextToken, maxResults | - |
| `ListAlarmModelVersionsResponse` | `structure` | alarmModelVersionSummaries, nextToken | - |
| `ListDetectorModelsRequest` | `structure` | nextToken, maxResults | - |
| `ListDetectorModelsResponse` | `structure` | detectorModelSummaries, nextToken | - |
| `ListDetectorModelVersionsRequest` | `structure` | detectorModelName, nextToken, maxResults | - |
| `AlarmModelVersionStatus` | `enum` | ACTIVE, ACTIVATING, INACTIVE, FAILED | - |
| `AnalysisResultLevel` | `enum` | INFO, WARNING, ERROR | - |
| `AnalysisStatus` | `enum` | RUNNING, COMPLETE, FAILED | - |
| `ComparisonOperator` | `enum` | GREATER, GREATER_OR_EQUAL, LESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL | - |
| `DetectorModelVersionStatus` | `enum` | ACTIVE, ACTIVATING, INACTIVE, DEPRECATED, DRAFT, PAUSED, FAILED | - |
| `EvaluationMethod` | `enum` | BATCH, SERIAL | - |
| `InputStatus` | `enum` | CREATING, UPDATING, ACTIVE, DELETING | - |
| `LoggingLevel` | `enum` | ERROR, INFO, DEBUG | - |
| `PayloadType` | `enum` | STRING, JSON | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
