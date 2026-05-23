# AWS IoT Events Data

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IoT Events monitors your equipment or device fleets for failures or changes in operation, and triggers actions when such events occur. You can use IoT Events Data API commands to send inputs to detectors, list detectors, and view or update a detector's status. For more information, see What is IoT Events? in the IoT Events Developer Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS IoT Events Data workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Batch`, `Describe`, `List` operation families, including `BatchAcknowledgeAlarm`, `BatchDeleteDetector`, `BatchDisableAlarm`, `BatchEnableAlarm`, `DescribeAlarm`, `DescribeDetector`.

## Service Identity and Protocol

- AWS model slug: `iot-events-data`
- AWS SDK for Rust slug: `ioteventsdata`
- Model version: `2018-10-23`
- Model file: `vendor/api-models-aws/models/iot-events-data/service/2018-10-23/iot-events-data-2018-10-23.json`
- SDK ID: `IoT Events Data`
- Endpoint prefix: `data.iotevents`
- ARN namespace: `ioteventsdata`
- CloudFormation name: `IoTEventsData`
- CloudTrail event source: `ioteventsdata.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Batch` (8), `Describe` (2), `List` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchAcknowledgeAlarm`, `BatchDeleteDetector`, `BatchDisableAlarm`, `BatchEnableAlarm`, `BatchPutMessage`, `BatchResetAlarm`, `BatchSnoozeAlarm`, `BatchUpdateDetector`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAlarm`, `DescribeDetector`, `ListAlarms`, `ListDetectors`.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`, `EC2/VPC`.

## Operation Groups

### Batch

- Operations: `BatchAcknowledgeAlarm`, `BatchDeleteDetector`, `BatchDisableAlarm`, `BatchEnableAlarm`, `BatchPutMessage`, `BatchResetAlarm`, `BatchSnoozeAlarm`, `BatchUpdateDetector`
- Common required input members in this group: `detectors`

### Describe

- Operations: `DescribeAlarm`, `DescribeDetector`
- Common required input members in this group: -

### List

- Operations: `ListAlarms`, `ListDetectors`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchAcknowledgeAlarm` | `POST /alarms/acknowledge` | - | `acknowledgeActionRequests` | - | `BatchAcknowledgeAlarmResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Acknowledges one or more alarms. The alarms change to the ACKNOWLEDGED state after you acknowledge them. |
| `BatchDeleteDetector` | `POST /detectors/delete` | - | `detectors` | - | `BatchDeleteDetectorResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes one or more detectors that were created. When a detector is deleted, its state will be cleared and the detector will be removed from the list of detectors. The deleted detector will no longer appear if refere ... |
| `BatchDisableAlarm` | `POST /alarms/disable` | - | `disableActionRequests` | - | `BatchDisableAlarmResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Disables one or more alarms. The alarms change to the DISABLED state after you disable them. |
| `BatchEnableAlarm` | `POST /alarms/enable` | - | `enableActionRequests` | - | `BatchEnableAlarmResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Enables one or more alarms. The alarms change to the NORMAL state after you enable them. |
| `BatchPutMessage` | `POST /inputs/messages` | - | `messages` | - | `BatchPutMessageResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Sends a set of messages to the IoT Events system. Each message payload is transformed into the input you specify ( "inputName" ) and ingested into any detectors that monitor that input. If multiple messages are sent, ... |
| `BatchResetAlarm` | `POST /alarms/reset` | - | `resetActionRequests` | - | `BatchResetAlarmResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Resets one or more alarms. The alarms return to the NORMAL state after you reset them. |
| `BatchSnoozeAlarm` | `POST /alarms/snooze` | - | `snoozeActionRequests` | - | `BatchSnoozeAlarmResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Changes one or more alarms to the snooze mode. The alarms change to the SNOOZE_DISABLED state after you set them to the snooze mode. |
| `BatchUpdateDetector` | `POST /detectors` | - | `detectors` | - | `BatchUpdateDetectorResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Updates the state, variable values, and timer settings of one or more detectors (instances) of a specified detector model. |
| `DescribeAlarm` | `GET /alarms/{alarmModelName}/keyValues` | - | `alarmModelName` | - | `DescribeAlarmResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves information about an alarm. |
| `DescribeDetector` | `GET /detectors/{detectorModelName}/keyValues` | - | `detectorModelName` | - | `DescribeDetectorResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns information about the specified detector (instance). |
| `ListAlarms` | `GET /alarms/{alarmModelName}` | - | `alarmModelName` | - | `ListAlarmsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists one or more alarms. The operation returns only the metadata associated with each alarm. |
| `ListDetectors` | `GET /detectors/{detectorModelName}` | - | `detectorModelName` | - | `ListDetectorsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists detectors (the instances of a detector model). |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DescribeAlarm` | - | `keyValue -> keyValue` | - | - |
| `DescribeDetector` | - | `keyValue -> keyValue` | - | - |
| `ListAlarms` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDetectors` | - | `stateName -> stateName`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalFailureException` | `structure` | message | An internal failure occurred. |
| `InvalidRequestException` | `structure` | message | The request was invalid. |
| `ResourceNotFoundException` | `structure` | message | The resource was not found. |
| `ServiceUnavailableException` | `structure` | message | The service is currently unavailable. |
| `ThrottlingException` | `structure` | message | The request could not be completed due to throttling. |
| `BatchAcknowledgeAlarmRequest` | `structure` | acknowledgeActionRequests | - |
| `BatchAcknowledgeAlarmResponse` | `structure` | errorEntries | - |
| `BatchDeleteDetectorRequest` | `structure` | detectors | - |
| `BatchDeleteDetectorResponse` | `structure` | batchDeleteDetectorErrorEntries | - |
| `BatchDisableAlarmRequest` | `structure` | disableActionRequests | - |
| `BatchDisableAlarmResponse` | `structure` | errorEntries | - |
| `BatchEnableAlarmRequest` | `structure` | enableActionRequests | - |
| `BatchEnableAlarmResponse` | `structure` | errorEntries | - |
| `BatchPutMessageRequest` | `structure` | messages | - |
| `BatchPutMessageResponse` | `structure` | BatchPutMessageErrorEntries | - |
| `BatchResetAlarmRequest` | `structure` | resetActionRequests | - |
| `BatchResetAlarmResponse` | `structure` | errorEntries | - |
| `BatchSnoozeAlarmRequest` | `structure` | snoozeActionRequests | - |
| `BatchSnoozeAlarmResponse` | `structure` | errorEntries | - |
| `BatchUpdateDetectorRequest` | `structure` | detectors | - |
| `BatchUpdateDetectorResponse` | `structure` | batchUpdateDetectorErrorEntries | - |
| `DescribeAlarmRequest` | `structure` | alarmModelName, keyValue | - |
| `DescribeAlarmResponse` | `structure` | alarm | - |
| `DescribeDetectorRequest` | `structure` | detectorModelName, keyValue | - |
| `DescribeDetectorResponse` | `structure` | detector | - |
| `ListAlarmsRequest` | `structure` | alarmModelName, nextToken, maxResults | - |
| `ListAlarmsResponse` | `structure` | alarmSummaries, nextToken | - |
| `ListDetectorsRequest` | `structure` | detectorModelName, stateName, nextToken, maxResults | - |
| `ListDetectorsResponse` | `structure` | detectorSummaries, nextToken | - |
| `AlarmStateName` | `enum` | DISABLED, NORMAL, ACTIVE, ACKNOWLEDGED, SNOOZE_DISABLED, LATCHED | - |
| `ComparisonOperator` | `enum` | GREATER, GREATER_OR_EQUAL, LESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL | - |
| `CustomerActionName` | `enum` | SNOOZE, ENABLE, DISABLE, ACKNOWLEDGE, RESET | - |
| `ErrorCode` | `enum` | ResourceNotFoundException, InvalidRequestException, InternalFailureException, ServiceUnavailableException, ThrottlingException | - |
| `EventType` | `enum` | STATE_CHANGE | - |
| `TriggerType` | `enum` | SNOOZE_TIMEOUT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
