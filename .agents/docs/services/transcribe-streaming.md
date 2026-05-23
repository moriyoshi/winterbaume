# Amazon Transcribe Streaming Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Transcribe streaming offers four main types of real-time transcription: Standard , Medical , Call Analytics , and Health Scribe . Standard transcriptions are the most common option. Refer to for details. Medical transcriptions are tailored to medical professionals and incorporate medical terms. A common use case for this service is transcribing doctor-patient dialogue in real time, so doctors can focus on their patient instead of taking notes.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Transcribe Streaming Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Transcribe Streaming Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Start`, `Get` operation families, including `StartCallAnalyticsStreamTranscription`, `StartMedicalScribeStream`, `StartMedicalStreamTranscription`, `StartStreamTranscription`, `GetMedicalScribeStream`.

## Service Identity and Protocol

- AWS model slug: `transcribe-streaming`
- AWS SDK for Rust slug: `transcribe`
- Model version: `2017-10-26`
- Model file: `vendor/api-models-aws/models/transcribe-streaming/service/2017-10-26/transcribe-streaming-2017-10-26.json`
- SDK ID: `Transcribe Streaming`
- Endpoint prefix: `transcribestreaming`
- ARN namespace: `transcribe`
- CloudFormation name: `TranscribeStreaming`
- CloudTrail event source: `transcribestreaming.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Start` (4), `Get` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartCallAnalyticsStreamTranscription`, `StartMedicalScribeStream`, `StartMedicalStreamTranscription`, `StartStreamTranscription`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetMedicalScribeStream`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartCallAnalyticsStreamTranscription`, `StartMedicalScribeStream`, `StartMedicalStreamTranscription`, `StartStreamTranscription`.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Start

- Operations: `StartCallAnalyticsStreamTranscription`, `StartMedicalScribeStream`, `StartMedicalStreamTranscription`, `StartStreamTranscription`
- Common required input members in this group: `AudioStream`, `InputStream`, `LanguageCode`, `MediaEncoding`, `MediaSampleRateHertz`, `Specialty`, `Type`

### Get

- Operations: `GetMedicalScribeStream`
- Common required input members in this group: `SessionId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetMedicalScribeStream` | `GET /medical-scribe-stream/{SessionId}` | - | `SessionId` | - | `GetMedicalScribeStreamResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `ResourceNotFoundException` | Provides details about the specified Amazon Web Services HealthScribe streaming session. To view the status of the streaming session, check the `StreamStatus` field in the response. |
| `StartCallAnalyticsStreamTranscription` | `POST /call-analytics-stream-transcription` | - | `AudioStream`, `MediaEncoding`, `MediaSampleRateHertz` | - | `StartCallAnalyticsStreamTranscriptionResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `ServiceUnavailableException` | Starts a bidirectional HTTP/2 or WebSocket stream where audio is streamed to Amazon Transcribe and the transcription results are streamed to your application. Use this operation for Call Analytics transcriptions. |
| `StartMedicalScribeStream` | `POST /medical-scribe-stream` | - | `InputStream`, `LanguageCode`, `MediaEncoding`, `MediaSampleRateHertz` | - | `StartMedicalScribeStreamResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `ServiceUnavailableException` | Starts a bidirectional HTTP/2 stream, where audio is streamed to Amazon Web Services HealthScribe and the transcription results are streamed to your application. When you start a stream, you first specify the stream configuration in a... |
| `StartMedicalStreamTranscription` | `POST /medical-stream-transcription` | - | `AudioStream`, `LanguageCode`, `MediaEncoding`, `MediaSampleRateHertz`, `Specialty`, `Type` | - | `StartMedicalStreamTranscriptionResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `ServiceUnavailableException` | Starts a bidirectional HTTP/2 or WebSocket stream where audio is streamed to Amazon Transcribe Medical and the transcription results are streamed to your application. The following parameters are required: `language-code` `media-encoding` `sample-rate` For... |
| `StartStreamTranscription` | `POST /stream-transcription` | - | `AudioStream`, `MediaEncoding`, `MediaSampleRateHertz` | - | `StartStreamTranscriptionResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `ServiceUnavailableException` | Starts a bidirectional HTTP/2 or WebSocket stream where audio is streamed to Amazon Transcribe and the transcription results are streamed to your application. The following parameters are required: `language-code` or `identify-language` or... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `StartCallAnalyticsStreamTranscription` | `LanguageCode -> x-amzn-transcribe-language-code`, `MediaSampleRateHertz -> x-amzn-transcribe-sample-rate`, `MediaEncoding -> x-amzn-transcribe-media-encoding`, `VocabularyName -> x-amzn-transcribe-vocabulary-name`, `SessionId -> x-amzn-transcribe-session-id`, `VocabularyFilterName -> x-amzn-transcribe-vocabulary-filter-name`, `VocabularyFilterMethod -> x-amzn-transcribe-vocabulary-filter-method`, `LanguageModelName -> x-amzn-transcribe-language-model-name`, `IdentifyLanguage -> x-amzn-transcribe-identify-language`, `LanguageOptions -> x-amzn-transcribe-language-options`, `PreferredLanguage -> x-amzn-transcribe-preferred-language`, `VocabularyNames -> x-amzn-transcribe-vocabulary-names`, `VocabularyFilterNames -> x-amzn-transcribe-vocabulary-filter-names`, `EnablePartialResultsStabilization -> x-amzn-transcribe-enable-partial-results-stabilization`, `PartialResultsStability -> x-amzn-transcribe-partial-results-stability`, `ContentIdentificationType -> x-amzn-transcribe-content-identification-type`, `ContentRedactionType -> x-amzn-transcribe-content-redaction-type`, `PiiEntityTypes -> x-amzn-transcribe-pii-entity-types` | - | - | `AudioStream` |
| `StartMedicalScribeStream` | `SessionId -> x-amzn-transcribe-session-id`, `LanguageCode -> x-amzn-transcribe-language-code`, `MediaSampleRateHertz -> x-amzn-transcribe-sample-rate`, `MediaEncoding -> x-amzn-transcribe-media-encoding` | - | - | `InputStream` |
| `StartMedicalStreamTranscription` | `LanguageCode -> x-amzn-transcribe-language-code`, `MediaSampleRateHertz -> x-amzn-transcribe-sample-rate`, `MediaEncoding -> x-amzn-transcribe-media-encoding`, `VocabularyName -> x-amzn-transcribe-vocabulary-name`, `Specialty -> x-amzn-transcribe-specialty`, `Type -> x-amzn-transcribe-type`, `ShowSpeakerLabel -> x-amzn-transcribe-show-speaker-label`, `SessionId -> x-amzn-transcribe-session-id`, `EnableChannelIdentification -> x-amzn-transcribe-enable-channel-identification`, `NumberOfChannels -> x-amzn-transcribe-number-of-channels`, `ContentIdentificationType -> x-amzn-transcribe-content-identification-type` | - | - | `AudioStream` |
| `StartStreamTranscription` | `LanguageCode -> x-amzn-transcribe-language-code`, `MediaSampleRateHertz -> x-amzn-transcribe-sample-rate`, `MediaEncoding -> x-amzn-transcribe-media-encoding`, `VocabularyName -> x-amzn-transcribe-vocabulary-name`, `SessionId -> x-amzn-transcribe-session-id`, `VocabularyFilterName -> x-amzn-transcribe-vocabulary-filter-name`, `VocabularyFilterMethod -> x-amzn-transcribe-vocabulary-filter-method`, `ShowSpeakerLabel -> x-amzn-transcribe-show-speaker-label`, `EnableChannelIdentification -> x-amzn-transcribe-enable-channel-identification`, `NumberOfChannels -> x-amzn-transcribe-number-of-channels`, `EnablePartialResultsStabilization -> x-amzn-transcribe-enable-partial-results-stabilization`, `PartialResultsStability -> x-amzn-transcribe-partial-results-stability`, `ContentIdentificationType -> x-amzn-transcribe-content-identification-type`, `ContentRedactionType -> x-amzn-transcribe-content-redaction-type`, `PiiEntityTypes -> x-amzn-transcribe-pii-entity-types`, `LanguageModelName -> x-amzn-transcribe-language-model-name`, `IdentifyLanguage -> x-amzn-transcribe-identify-language`, `LanguageOptions -> x-amzn-transcribe-language-options`, `PreferredLanguage -> x-amzn-transcribe-preferred-language`, `IdentifyMultipleLanguages -> x-amzn-transcribe-identify-multiple-languages`, `VocabularyNames -> x-amzn-transcribe-vocabulary-names`, `VocabularyFilterNames -> x-amzn-transcribe-vocabulary-filter-names`, `SessionResumeWindow -> x-amzn-transcribe-session-resume-window` | - | - | `AudioStream` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message` | One or more arguments to the `StartStreamTranscription`, `StartMedicalStreamTranscription`, or `StartCallAnalyticsStreamTranscription` operation was not valid. |
| `InternalFailureException` | `structure` | `Message` | A problem occurred while processing the audio. |
| `LimitExceededException` | `structure` | `Message` | Your client has exceeded one of the Amazon Transcribe limits. |
| `ConflictException` | `structure` | `Message` | A new stream started with the same session ID. |
| `ServiceUnavailableException` | `structure` | `Message` | The service is currently unavailable. |
| `GetMedicalScribeStreamRequest` | `structure` | `SessionId` | - |
| `GetMedicalScribeStreamResponse` | `structure` | `MedicalScribeStreamDetails` | - |
| `ResourceNotFoundException` | `structure` | `Message` | The request references a resource which doesn't exist. |
| `StartCallAnalyticsStreamTranscriptionRequest` | `structure` | `AudioStream`, `ContentIdentificationType`, `ContentRedactionType`, `EnablePartialResultsStabilization`, `IdentifyLanguage`, `LanguageCode`, `LanguageModelName`, `LanguageOptions`, `MediaEncoding`, `MediaSampleRateHertz`, `PartialResultsStability`, `PiiEntityTypes`, ... (+7) | - |
| `StartCallAnalyticsStreamTranscriptionResponse` | `structure` | `CallAnalyticsTranscriptResultStream`, `ContentIdentificationType`, `ContentRedactionType`, `EnablePartialResultsStabilization`, `IdentifyLanguage`, `LanguageCode`, `LanguageModelName`, `LanguageOptions`, `MediaEncoding`, `MediaSampleRateHertz`, `PartialResultsStability`, `PiiEntityTypes`, ... (+8) | - |
| `StartMedicalScribeStreamRequest` | `structure` | `InputStream`, `LanguageCode`, `MediaEncoding`, `MediaSampleRateHertz`, `SessionId` | - |
| `StartMedicalScribeStreamResponse` | `structure` | `LanguageCode`, `MediaEncoding`, `MediaSampleRateHertz`, `RequestId`, `ResultStream`, `SessionId` | - |
| `StartMedicalStreamTranscriptionRequest` | `structure` | `AudioStream`, `ContentIdentificationType`, `EnableChannelIdentification`, `LanguageCode`, `MediaEncoding`, `MediaSampleRateHertz`, `NumberOfChannels`, `SessionId`, `ShowSpeakerLabel`, `Specialty`, `Type`, `VocabularyName` | - |
| `StartMedicalStreamTranscriptionResponse` | `structure` | `ContentIdentificationType`, `EnableChannelIdentification`, `LanguageCode`, `MediaEncoding`, `MediaSampleRateHertz`, `NumberOfChannels`, `RequestId`, `SessionId`, `ShowSpeakerLabel`, `Specialty`, `TranscriptResultStream`, `Type`, ... (+1) | - |
| `StartStreamTranscriptionRequest` | `structure` | `AudioStream`, `ContentIdentificationType`, `ContentRedactionType`, `EnableChannelIdentification`, `EnablePartialResultsStabilization`, `IdentifyLanguage`, `IdentifyMultipleLanguages`, `LanguageCode`, `LanguageModelName`, `LanguageOptions`, `MediaEncoding`, `MediaSampleRateHertz`, ... (+12) | - |
| `StartStreamTranscriptionResponse` | `structure` | `ContentIdentificationType`, `ContentRedactionType`, `EnableChannelIdentification`, `EnablePartialResultsStabilization`, `IdentifyLanguage`, `IdentifyMultipleLanguages`, `LanguageCode`, `LanguageModelName`, `LanguageOptions`, `MediaEncoding`, `MediaSampleRateHertz`, `NumberOfChannels`, ... (+13) | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
