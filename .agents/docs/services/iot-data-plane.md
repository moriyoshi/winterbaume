# AWS IoT Data Plane

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IoT data IoT data enables secure, bi-directional communication between Internet-connected things (such as sensors, actuators, embedded devices, or smart appliances) and the Amazon Web Services cloud. It implements a broker for applications and things to publish messages over HTTP (Publish) and retrieve, update, and delete shadows. A shadow is a persistent representation of your things and their state in the Amazon Web Services cloud. Find the endpoint address for actions in IoT data by running this CLI command: `aws iot describe-endpoint --endpoint-type iot:Data-ATS` The service name used by Amazon Web ServicesSignature Version 4 to sign requests is: iotdevicegateway .

## Possible Usage Scenarios
- Backported from `crates/winterbaume-iotdataplane/tests/scenario_test.rs`: create, update, read, and delete a device shadow.
- Backported from `scenario_test.rs`: keep classic and named shadows independent for the same device.
- Backported from `scenario_test.rs`: publish and retrieve retained messages.
- From the AWS documentation and model: model MQTT-style data-plane operations for thing shadows, named shadows, retained messages, payload versioning, and request/response error semantics.

## Service Identity and Protocol

- AWS model slug: `iot-data-plane`
- AWS SDK for Rust slug: `iotdataplane`
- Model version: `2015-05-28`
- Model file: `vendor/api-models-aws/models/iot-data-plane/service/2015-05-28/iot-data-plane-2015-05-28.json`
- SDK ID: `IoT Data Plane`
- Endpoint prefix: `data-ats.iot`
- ARN namespace: `iotdata`
- CloudFormation name: `IoTDataPlane`
- CloudTrail event source: `iotdataplane.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (2), `Get` (2), `List` (2), `Publish` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteConnection`, `DeleteThingShadow`, `UpdateThingShadow`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRetainedMessage`, `GetThingShadow`, `ListNamedShadowsForThing`, `ListRetainedMessages`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- 8 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Delete

- Operations: `DeleteConnection`, `DeleteThingShadow`
- Common required input members in this group: `clientId`, `thingName`

### Get

- Operations: `GetRetainedMessage`, `GetThingShadow`
- Common required input members in this group: `thingName`, `topic`

### List

- Operations: `ListNamedShadowsForThing`, `ListRetainedMessages`
- Traits: `paginated` (1)
- Common required input members in this group: `thingName`

### Publish

- Operations: `Publish`
- Common required input members in this group: `topic`

### Update

- Operations: `UpdateThingShadow`
- Common required input members in this group: `payload`, `thingName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteConnection` | `DELETE /connections/{clientId}` | - | `clientId` | - | `Unit` | `ForbiddenException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Disconnects a connected MQTT client from Amazon Web Services IoT Core. When you disconnect a client, Amazon Web Services IoT Core closes the client's network connection and optionally cleans the session state. |
| `DeleteThingShadow` | `DELETE /things/{thingName}/shadow` | - | `thingName` | - | `DeleteThingShadowResponse` | `InternalFailureException`, `InvalidRequestException`, `MethodNotAllowedException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `UnsupportedDocumentEncodingException` | Deletes the shadow for the specified thing. Requires permission to access the DeleteThingShadow action. |
| `GetRetainedMessage` | `GET /retainedMessage/{topic}` | - | `topic` | - | `GetRetainedMessageResponse` | `InternalFailureException`, `InvalidRequestException`, `MethodNotAllowedException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Gets the details of a single retained message for the specified topic. This action returns the message payload of the retained message, which can incur messaging costs. |
| `GetThingShadow` | `GET /things/{thingName}/shadow` | - | `thingName` | - | `GetThingShadowResponse` | `InternalFailureException`, `InvalidRequestException`, `MethodNotAllowedException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `UnsupportedDocumentEncodingException` | Gets the shadow for the specified thing. Requires permission to access the GetThingShadow action. |
| `ListNamedShadowsForThing` | `GET /api/things/shadow/ListNamedShadowsForThing/{thingName}` | - | `thingName` | - | `ListNamedShadowsForThingResponse` | `InternalFailureException`, `InvalidRequestException`, `MethodNotAllowedException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Lists the shadows for the specified thing. Requires permission to access the ListNamedShadowsForThing action. |
| `ListRetainedMessages` | `GET /retainedMessage` | `paginated` | - | - | `ListRetainedMessagesResponse` | `InternalFailureException`, `InvalidRequestException`, `MethodNotAllowedException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Lists summary information about the retained messages stored for the account. This action returns only the topic names of the retained messages. |
| `Publish` | `POST /topics/{topic}` | - | `topic` | - | `Unit` | `InternalFailureException`, `InvalidRequestException`, `MethodNotAllowedException`, `ThrottlingException`, `UnauthorizedException` | Publishes an MQTT message. Requires permission to access the Publish action. |
| `UpdateThingShadow` | `POST /things/{thingName}/shadow` | - | `payload`, `thingName` | - | `UpdateThingShadowResponse` | `ConflictException`, `InternalFailureException`, `InvalidRequestException`, `MethodNotAllowedException`, `RequestEntityTooLargeException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, ... (+1) | Updates the shadow for the specified thing. Requires permission to access the UpdateThingShadow action. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteConnection` | - | `cleanSession -> cleanSession`, `preventWillMessage -> preventWillMessage` | - | - |
| `DeleteThingShadow` | - | `shadowName -> name` | - | - |
| `GetThingShadow` | - | `shadowName -> name` | - | - |
| `ListNamedShadowsForThing` | - | `nextToken -> nextToken`, `pageSize -> pageSize` | - | - |
| `ListRetainedMessages` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `Publish` | `userProperties -> x-amz-mqtt5-user-properties`, `payloadFormatIndicator -> x-amz-mqtt5-payload-format-indicator`, `correlationData -> x-amz-mqtt5-correlation-data` | `qos -> qos`, `retain -> retain`, `contentType -> contentType`, `responseTopic -> responseTopic`, `messageExpiry -> messageExpiry` | - | `payload` |
| `UpdateThingShadow` | - | `shadowName -> name` | - | `payload` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalFailureException` | `structure` | `message` | An unexpected error has occurred. |
| `InvalidRequestException` | `structure` | `message` | The request is not valid. |
| `ThrottlingException` | `structure` | `message` | The rate exceeds the limit. |
| `MethodNotAllowedException` | `structure` | `message` | The specified combination of HTTP verb and URI is not supported. |
| `UnauthorizedException` | `structure` | `message` | You are not authorized to perform this operation. |
| `ServiceUnavailableException` | `structure` | `message` | The service is temporarily unavailable. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource does not exist. |
| `UnsupportedDocumentEncodingException` | `structure` | `message` | The document encoding is not supported. |
| `DeleteConnectionRequest` | `structure` | `cleanSession`, `clientId`, `preventWillMessage` | - |
| `ForbiddenException` | `structure` | `message` | The caller isn't authorized to make the request. |
| `DeleteThingShadowRequest` | `structure` | `shadowName`, `thingName` | The input for the DeleteThingShadow operation. |
| `DeleteThingShadowResponse` | `structure` | `payload` | The output from the DeleteThingShadow operation. |
| `GetRetainedMessageRequest` | `structure` | `topic` | The input for the GetRetainedMessage operation. |
| `GetRetainedMessageResponse` | `structure` | `lastModifiedTime`, `payload`, `qos`, `topic`, `userProperties` | The output from the GetRetainedMessage operation. |
| `GetThingShadowRequest` | `structure` | `shadowName`, `thingName` | The input for the GetThingShadow operation. |
| `GetThingShadowResponse` | `structure` | `payload` | The output from the GetThingShadow operation. |
| `ListNamedShadowsForThingRequest` | `structure` | `nextToken`, `pageSize`, `thingName` | - |
| `ListNamedShadowsForThingResponse` | `structure` | `nextToken`, `results`, `timestamp` | - |
| `ListRetainedMessagesRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListRetainedMessagesResponse` | `structure` | `nextToken`, `retainedTopics` | - |
| `PublishRequest` | `structure` | `contentType`, `correlationData`, `messageExpiry`, `payload`, `payloadFormatIndicator`, `qos`, `responseTopic`, `retain`, `topic`, `userProperties` | The input for the Publish operation. |
| `UpdateThingShadowRequest` | `structure` | `payload`, `shadowName`, `thingName` | The input for the UpdateThingShadow operation. |
| `UpdateThingShadowResponse` | `structure` | `payload` | The output from the UpdateThingShadow operation. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
