# Amazon Lex Runtime Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Lex provides both build and runtime endpoints. Each endpoint provides a set of operations (API). Your conversational bot uses the runtime API to understand user utterances (user input text or voice). For example, suppose a user says "I want pizza", your bot sends this input to Amazon Lex using the runtime API. Amazon Lex recognizes that the user request is for the OrderPizza intent (one of the intents defined in the bot).

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Lex Runtime Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Post`, `Delete`, `Get`, `Put` operation families, including `PostContent`, `PostText`, `DeleteSession`, `GetSession`, `PutSession`.

## Service Identity and Protocol

- AWS model slug: `lex-runtime-service`
- AWS SDK for Rust slug: `lexruntimeservice`
- Model version: `2016-11-28`
- Model file: `vendor/api-models-aws/models/lex-runtime-service/service/2016-11-28/lex-runtime-service-2016-11-28.json`
- SDK ID: `Lex Runtime Service`
- Endpoint prefix: `runtime.lex`
- ARN namespace: `lex`
- CloudFormation name: `LexRuntimeService`
- CloudTrail event source: `lexruntimeservice.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Post` (2), `Delete` (1), `Get` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteSession`, `PutSession`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetSession`.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `Lambda`.

## Operation Groups

### Post

- Operations: `PostContent`, `PostText`
- Common required input members in this group: `botName`, `botAlias`, `userId`

### Delete

- Operations: `DeleteSession`
- Common required input members in this group: -

### Get

- Operations: `GetSession`
- Common required input members in this group: -

### Put

- Operations: `PutSession`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteSession` | `DELETE /bot/{botName}/alias/{botAlias}/user/{userId}/session` | - | `botName`, `botAlias`, `userId` | - | `DeleteSessionResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Removes session information for a specified bot, alias, and user ID. |
| `GetSession` | `GET /bot/{botName}/alias/{botAlias}/user/{userId}/session` | - | `botName`, `botAlias`, `userId` | - | `GetSessionResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns session information for a specified bot, alias, and user ID. |
| `PostContent` | `POST /bot/{botName}/alias/{botAlias}/user/{userId}/content` | - | `botName`, `botAlias`, `userId`, `contentType`, `inputStream` | - | `PostContentResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `DependencyFailedException`, `InternalFailureException`, `LimitExceededException`, `LoopDetectedException`, `NotAcceptableException`, `NotFoundException`, `RequestTimeoutException`, `UnsupportedMediaTypeException` | Sends user input (text or speech) to Amazon Lex. Clients use this API to send text and audio requests to Amazon Lex at runtime. Amazon Lex interprets the user input using the machine learning model that it built for ... |
| `PostText` | `POST /bot/{botName}/alias/{botAlias}/user/{userId}/text` | - | `botName`, `botAlias`, `userId`, `inputText` | - | `PostTextResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `DependencyFailedException`, `InternalFailureException`, `LimitExceededException`, `LoopDetectedException`, `NotFoundException` | Sends user input to Amazon Lex. Client applications can use this API to send requests to Amazon Lex at runtime. Amazon Lex then interprets the user input using the machine learning model it built for the bot. In resp ... |
| `PutSession` | `POST /bot/{botName}/alias/{botAlias}/user/{userId}/session` | - | `botName`, `botAlias`, `userId` | - | `PutSessionResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `DependencyFailedException`, `InternalFailureException`, `LimitExceededException`, `NotAcceptableException`, `NotFoundException` | Creates a new session or modifies an existing session with an Amazon Lex bot. Use this operation to enable your application to set the state of the bot. For more information, see Managing Sessions . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetSession` | - | `checkpointLabelFilter -> checkpointLabelFilter` | - | - |
| `PostContent` | `sessionAttributes -> x-amz-lex-session-attributes`, `requestAttributes -> x-amz-lex-request-attributes`, `contentType -> Content-Type`, `accept -> Accept`, `activeContexts -> x-amz-lex-active-contexts` | - | - | `inputStream` |
| `PutSession` | `accept -> Accept` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadGatewayException` | `structure` | Message | Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error. |
| `BadRequestException` | `structure` | message | Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. |
| `ConflictException` | `structure` | message | Two clients are using the same AWS account, Amazon Lex bot, and user ID. |
| `DependencyFailedException` | `structure` | Message | One of the dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, If Amazon Lex does not have sufficient permissions to call a L ... |
| `InternalFailureException` | `structure` | message | Internal service error. Retry the call. |
| `LimitExceededException` | `structure` | retryAfterSeconds, message | Exceeded a limit. |
| `LoopDetectedException` | `structure` | Message | This exception is not used. |
| `NotAcceptableException` | `structure` | message | The accept header in the request does not have a valid value. |
| `NotFoundException` | `structure` | message | The resource (such as the Amazon Lex bot or an alias) that is referred to is not found. |
| `RequestTimeoutException` | `structure` | message | The input speech is too long. |
| `UnsupportedMediaTypeException` | `structure` | message | The Content-Type header ( PostContent API) has an invalid value. |
| `DeleteSessionRequest` | `structure` | botName, botAlias, userId | - |
| `DeleteSessionResponse` | `structure` | botName, botAlias, userId, sessionId | - |
| `GetSessionRequest` | `structure` | botName, botAlias, userId, checkpointLabelFilter | - |
| `GetSessionResponse` | `structure` | recentIntentSummaryView, sessionAttributes, sessionId, dialogAction, activeContexts | - |
| `PostContentRequest` | `structure` | botName, botAlias, userId, sessionAttributes, requestAttributes, contentType, accept, inputStream, activeContexts | - |
| `PostContentResponse` | `structure` | contentType, intentName, nluIntentConfidence, alternativeIntents, slots, sessionAttributes, sentimentResponse, message, encodedMessage, messageFormat, dialogState, slotToElicit, ... (+6) | - |
| `PostTextRequest` | `structure` | botName, botAlias, userId, sessionAttributes, requestAttributes, inputText, activeContexts | - |
| `PostTextResponse` | `structure` | intentName, nluIntentConfidence, alternativeIntents, slots, sessionAttributes, message, sentimentResponse, messageFormat, dialogState, slotToElicit, responseCard, sessionId, ... (+2) | - |
| `PutSessionRequest` | `structure` | botName, botAlias, userId, sessionAttributes, dialogAction, recentIntentSummaryView, accept, activeContexts | - |
| `PutSessionResponse` | `structure` | contentType, intentName, slots, sessionAttributes, message, encodedMessage, messageFormat, dialogState, slotToElicit, audioStream, sessionId, activeContexts | - |
| `ConfirmationStatus` | `enum` | NONE, CONFIRMED, DENIED | - |
| `ContentType` | `enum` | GENERIC | - |
| `DialogActionType` | `enum` | ELICIT_INTENT, CONFIRM_INTENT, ELICIT_SLOT, CLOSE, DELEGATE | - |
| `DialogState` | `enum` | ELICIT_INTENT, CONFIRM_INTENT, ELICIT_SLOT, FULFILLED, READY_FOR_FULFILLMENT, FAILED | - |
| `FulfillmentState` | `enum` | FULFILLED, FAILED, READY_FOR_FULFILLMENT | - |
| `MessageFormatType` | `enum` | PLAIN_TEXT, CUSTOM_PAYLOAD, SSML, COMPOSITE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
