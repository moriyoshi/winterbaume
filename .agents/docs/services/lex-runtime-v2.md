# Amazon Lex Runtime V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This section contains documentation for the Amazon Lex V2 Runtime V2 API operations.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Lex Runtime V2 resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Lex Runtime V2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Recognize`, `Delete`, `Get`, `Put`, `Start` operation families, including `RecognizeText`, `RecognizeUtterance`, `DeleteSession`, `GetSession`, `PutSession`, `StartConversation`.

## Service Identity and Protocol

- AWS model slug: `lex-runtime-v2`
- AWS SDK for Rust slug: `lexruntimev2`
- Model version: `2020-08-07`
- Model file: `vendor/api-models-aws/models/lex-runtime-v2/service/2020-08-07/lex-runtime-v2-2020-08-07.json`
- SDK ID: `Lex Runtime V2`
- Endpoint prefix: `runtime-v2-lex`
- ARN namespace: `lex`
- CloudFormation name: `LexRuntimeV2`
- CloudTrail event source: `lexruntimev2.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Recognize` (2), `Delete` (1), `Get` (1), `Put` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteSession`, `PutSession`, `StartConversation`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetSession`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartConversation`.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.

## Operation Groups

### Recognize

- Operations: `RecognizeText`, `RecognizeUtterance`
- Common required input members in this group: `botId`, `botAliasId`, `localeId`, `sessionId`

### Delete

- Operations: `DeleteSession`
- Common required input members in this group: -

### Get

- Operations: `GetSession`
- Common required input members in this group: -

### Put

- Operations: `PutSession`
- Common required input members in this group: -

### Start

- Operations: `StartConversation`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteSession` | `DELETE /bots/{botId}/botAliases/{botAliasId}/botLocales/{localeId}/sessions/{sessionId}` | - | `botId`, `botAliasId`, `localeId`, `sessionId` | - | `DeleteSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes session information for a specified bot, alias, and user ID. You can use this operation to restart a conversation with a bot. When you remove a session, the entire history of the session is removed so that yo ... |
| `GetSession` | `GET /bots/{botId}/botAliases/{botAliasId}/botLocales/{localeId}/sessions/{sessionId}` | - | `botId`, `botAliasId`, `localeId`, `sessionId` | - | `GetSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns session information for a specified bot, alias, and user. For example, you can use this operation to retrieve session information for a user that has left a long-running session in use. If the bot, alias, or ... |
| `PutSession` | `POST /bots/{botId}/botAliases/{botAliasId}/botLocales/{localeId}/sessions/{sessionId}` | - | `botId`, `botAliasId`, `localeId`, `sessionId`, `sessionState` | - | `PutSessionResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new session or modifies an existing session with an Amazon Lex V2 bot. Use this operation to enable your application to set the state of the bot. |
| `RecognizeText` | `POST /bots/{botId}/botAliases/{botAliasId}/botLocales/{localeId}/sessions/{sessionId}/text` | - | `botId`, `botAliasId`, `localeId`, `sessionId`, `text` | - | `RecognizeTextResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends user input to Amazon Lex V2. Client applications use this API to send requests to Amazon Lex V2 at runtime. Amazon Lex V2 then interprets the user input using the machine learning model that it build for the bo ... |
| `RecognizeUtterance` | `POST /bots/{botId}/botAliases/{botAliasId}/botLocales/{localeId}/sessions/{sessionId}/utterance` | - | `botId`, `botAliasId`, `localeId`, `sessionId`, `requestContentType` | - | `RecognizeUtteranceResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends user input to Amazon Lex V2. You can send text or speech. Clients use this API to send text and audio requests to Amazon Lex V2 at runtime. Amazon Lex V2 interprets the user input using the machine learning mod ... |
| `StartConversation` | `POST /bots/{botId}/botAliases/{botAliasId}/botLocales/{localeId}/sessions/{sessionId}/conversation` | - | `botId`, `botAliasId`, `localeId`, `sessionId`, `requestEventStream` | - | `StartConversationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Starts an HTTP/2 bidirectional event stream that enables you to send audio, text, or DTMF input in real time. After your application starts a conversation, users send input to Amazon Lex V2 as a stream of events. Ama ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `PutSession` | `responseContentType -> ResponseContentType` | - | - | - |
| `RecognizeUtterance` | `sessionState -> x-amz-lex-session-state`, `requestAttributes -> x-amz-lex-request-attributes`, `requestContentType -> Content-Type`, `responseContentType -> Response-Content-Type` | - | - | `inputStream` |
| `StartConversation` | `conversationMode -> x-amz-lex-conversation-mode` | - | - | `requestEventStream` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | - |
| `BadGatewayException` | `structure` | message | - |
| `ConflictException` | `structure` | message | - |
| `DependencyFailedException` | `structure` | message | - |
| `InternalServerException` | `structure` | message | - |
| `ResourceNotFoundException` | `structure` | message | - |
| `ThrottlingException` | `structure` | message | - |
| `ValidationException` | `structure` | message | - |
| `DeleteSessionRequest` | `structure` | botId, botAliasId, localeId, sessionId | - |
| `DeleteSessionResponse` | `structure` | botId, botAliasId, localeId, sessionId | - |
| `GetSessionRequest` | `structure` | botId, botAliasId, localeId, sessionId | - |
| `GetSessionResponse` | `structure` | sessionId, messages, interpretations, sessionState | - |
| `PutSessionRequest` | `structure` | botId, botAliasId, localeId, sessionId, messages, sessionState, requestAttributes, responseContentType | - |
| `PutSessionResponse` | `structure` | contentType, messages, sessionState, requestAttributes, sessionId, audioStream | - |
| `RecognizeTextRequest` | `structure` | botId, botAliasId, localeId, sessionId, text, sessionState, requestAttributes | - |
| `RecognizeTextResponse` | `structure` | messages, sessionState, interpretations, requestAttributes, sessionId, recognizedBotMember | - |
| `RecognizeUtteranceRequest` | `structure` | botId, botAliasId, localeId, sessionId, sessionState, requestAttributes, requestContentType, responseContentType, inputStream | - |
| `RecognizeUtteranceResponse` | `structure` | inputMode, contentType, messages, interpretations, sessionState, requestAttributes, sessionId, inputTranscript, audioStream, recognizedBotMember | - |
| `StartConversationRequest` | `structure` | botId, botAliasId, localeId, sessionId, conversationMode, requestEventStream | - |
| `StartConversationResponse` | `structure` | responseEventStream | - |
| `ConfirmationState` | `enum` | CONFIRMED, DENIED, NONE | - |
| `ConversationMode` | `enum` | AUDIO, TEXT | - |
| `DialogActionType` | `enum` | CLOSE, CONFIRM_INTENT, DELEGATE, ELICIT_INTENT, ELICIT_SLOT, NONE | - |
| `InputMode` | `enum` | TEXT, SPEECH, DTMF | - |
| `IntentState` | `enum` | FAILED, FULFILLED, IN_PROGRESS, READY_FOR_FULFILLMENT, WAITING, FULFILLMENT_IN_PROGRESS | - |
| `InterpretationSource` | `enum` | BEDROCK, LEX | - |
| `MessageContentType` | `enum` | CUSTOM_PAYLOAD, IMAGE_RESPONSE_CARD, PLAIN_TEXT, SSML | - |
| `PlaybackInterruptionReason` | `enum` | DTMF_START_DETECTED, TEXT_DETECTED, VOICE_START_DETECTED | - |
| `SentimentType` | `enum` | MIXED, NEGATIVE, NEUTRAL, POSITIVE | - |
| `Shape` | `enum` | SCALAR, LIST, COMPOSITE | - |
| `StyleType` | `enum` | DEFAULT, SPELL_BY_LETTER, SPELL_BY_WORD | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
