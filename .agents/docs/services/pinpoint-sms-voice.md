# Amazon Pinpoint SMS and Voice Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Pinpoint SMS and Voice Messaging public facing APIs

## Possible Usage Scenarios
- Backported from `crates/winterbaume-pinpointsmsvoice/tests/scenario_test.rs`: configure a voice campaign pipeline with a configuration set and event destination.
- Backported from `scenario_test.rs`: manage multiple configuration sets and verify that they coexist in list operations.
- From the AWS documentation and model: support SMS/voice messaging configuration, event destinations, opt-out and sender configuration surfaces, and campaign-oriented messaging setup.

## Service Identity and Protocol

- AWS model slug: `pinpoint-sms-voice`
- AWS SDK for Rust slug: `pinpointsmsvoice`
- Model version: `2018-09-05`
- Model file: `vendor/api-models-aws/models/pinpoint-sms-voice/service/2018-09-05/pinpoint-sms-voice-2018-09-05.json`
- SDK ID: `Pinpoint SMS Voice`
- Endpoint prefix: `sms-voice.pinpoint`
- ARN namespace: `sms-voice`
- CloudFormation name: `PinpointSMSVoice`
- CloudTrail event source: `pinpointsmsvoice.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (2), `Delete` (2), `Get` (1), `List` (1), `Send` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateConfigurationSet`, `CreateConfigurationSetEventDestination`, `DeleteConfigurationSet`, `DeleteConfigurationSetEventDestination`, `UpdateConfigurationSetEventDestination`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetConfigurationSetEventDestinations`, `ListConfigurationSets`.
- 8 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`, `SNS`.

## Operation Groups

### Create

- Operations: `CreateConfigurationSet`, `CreateConfigurationSetEventDestination`
- Common required input members in this group: -

### Delete

- Operations: `DeleteConfigurationSet`, `DeleteConfigurationSetEventDestination`
- Common required input members in this group: `ConfigurationSetName`

### Get

- Operations: `GetConfigurationSetEventDestinations`
- Common required input members in this group: -

### List

- Operations: `ListConfigurationSets`
- Common required input members in this group: -

### Send

- Operations: `SendVoiceMessage`
- Common required input members in this group: -

### Update

- Operations: `UpdateConfigurationSetEventDestination`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateConfigurationSet` | `POST /v1/sms-voice/configuration-sets` | - | - | - | `CreateConfigurationSetResponse` | `AlreadyExistsException`, `BadRequestException`, `InternalServiceErrorException`, `LimitExceededException`, `TooManyRequestsException` | Create a new configuration set. After you create the configuration set, you can add one or more event destinations to it. |
| `CreateConfigurationSetEventDestination` | `POST /v1/sms-voice/configuration-sets/{ConfigurationSetName}/event-destinations` | - | `ConfigurationSetName` | - | `CreateConfigurationSetEventDestinationResponse` | `AlreadyExistsException`, `BadRequestException`, `InternalServiceErrorException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Create a new event destination in a configuration set. |
| `DeleteConfigurationSet` | `DELETE /v1/sms-voice/configuration-sets/{ConfigurationSetName}` | - | `ConfigurationSetName` | - | `DeleteConfigurationSetResponse` | `BadRequestException`, `InternalServiceErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes an existing configuration set. |
| `DeleteConfigurationSetEventDestination` | `DELETE /v1/sms-voice/configuration-sets/{ConfigurationSetName}/event-destinations/{EventDestinationName}` | - | `ConfigurationSetName`, `EventDestinationName` | - | `DeleteConfigurationSetEventDestinationResponse` | `BadRequestException`, `InternalServiceErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes an event destination in a configuration set. |
| `GetConfigurationSetEventDestinations` | `GET /v1/sms-voice/configuration-sets/{ConfigurationSetName}/event-destinations` | - | `ConfigurationSetName` | - | `GetConfigurationSetEventDestinationsResponse` | `BadRequestException`, `InternalServiceErrorException`, `NotFoundException`, `TooManyRequestsException` | Obtain information about an event destination, including the types of events it reports, the Amazon Resource Name (ARN) of the destination, and the name of the event destination. |
| `ListConfigurationSets` | `GET /v1/sms-voice/configuration-sets` | - | - | - | `ListConfigurationSetsResponse` | `BadRequestException`, `InternalServiceErrorException`, `TooManyRequestsException` | List all of the configuration sets associated with your Amazon Pinpoint account in the current region. |
| `SendVoiceMessage` | `POST /v1/sms-voice/voice/message` | - | - | - | `SendVoiceMessageResponse` | `BadRequestException`, `InternalServiceErrorException`, `TooManyRequestsException` | Create a new voice message and send it to a recipient's phone number. |
| `UpdateConfigurationSetEventDestination` | `PUT /v1/sms-voice/configuration-sets/{ConfigurationSetName}/event-destinations/{EventDestinationName}` | - | `ConfigurationSetName`, `EventDestinationName` | - | `UpdateConfigurationSetEventDestinationResponse` | `BadRequestException`, `InternalServiceErrorException`, `NotFoundException`, `TooManyRequestsException` | Update an event destination in a configuration set. An event destination is a location that you publish information about your voice calls to. For example, you can log an event to an Amazon CloudWatch destination whe ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListConfigurationSets` | - | `NextToken -> NextToken`, `PageSize -> PageSize` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AlreadyExistsException` | `structure` | Message | The resource specified in your request already exists. |
| `BadRequestException` | `structure` | Message | The input you provided is invalid. |
| `InternalServiceErrorException` | `structure` | Message | The API encountered an unexpected error and couldn't complete the request. You might be able to successfully issue the request again in the future. |
| `LimitExceededException` | `structure` | Message | There are too many instances of the specified resource type. |
| `NotFoundException` | `structure` | Message | The resource you attempted to access doesn't exist. |
| `TooManyRequestsException` | `structure` | Message | You've issued too many requests to the resource. Wait a few minutes, and then try again. |
| `CreateConfigurationSetRequest` | `structure` | ConfigurationSetName | A request to create a new configuration set. |
| `CreateConfigurationSetResponse` | `structure` | **empty (no members)** | An empty object that indicates that the configuration set was successfully created. |
| `CreateConfigurationSetEventDestinationRequest` | `structure` | ConfigurationSetName, EventDestination, EventDestinationName | Create a new event destination in a configuration set. |
| `CreateConfigurationSetEventDestinationResponse` | `structure` | **empty (no members)** | An empty object that indicates that the event destination was created successfully. |
| `DeleteConfigurationSetRequest` | `structure` | ConfigurationSetName | - |
| `DeleteConfigurationSetResponse` | `structure` | **empty (no members)** | An empty object that indicates that the configuration set was deleted successfully. |
| `DeleteConfigurationSetEventDestinationRequest` | `structure` | ConfigurationSetName, EventDestinationName | - |
| `DeleteConfigurationSetEventDestinationResponse` | `structure` | **empty (no members)** | An empty object that indicates that the event destination was deleted successfully. |
| `GetConfigurationSetEventDestinationsRequest` | `structure` | ConfigurationSetName | - |
| `GetConfigurationSetEventDestinationsResponse` | `structure` | EventDestinations | An object that contains information about an event destination. |
| `ListConfigurationSetsRequest` | `structure` | NextToken, PageSize | - |
| `ListConfigurationSetsResponse` | `structure` | ConfigurationSets, NextToken | An object that contains information about the configuration sets for your account in the current region. |
| `SendVoiceMessageRequest` | `structure` | CallerId, ConfigurationSetName, Content, DestinationPhoneNumber, OriginationPhoneNumber | SendVoiceMessageRequest |
| `SendVoiceMessageResponse` | `structure` | MessageId | An object that that contains the Message ID of a Voice message that was sent successfully. |
| `UpdateConfigurationSetEventDestinationRequest` | `structure` | ConfigurationSetName, EventDestination, EventDestinationName | UpdateConfigurationSetEventDestinationRequest |
| `UpdateConfigurationSetEventDestinationResponse` | `structure` | **empty (no members)** | An empty object that indicates that the event destination was updated successfully. |
| `EventType` | `enum` | INITIATED_CALL, RINGING, ANSWERED, COMPLETED_CALL, BUSY, FAILED, NO_ANSWER | The types of events that are sent to the event destination. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
