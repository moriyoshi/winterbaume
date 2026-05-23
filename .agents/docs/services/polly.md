# Amazon Polly

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Polly is a web service that makes it easy to synthesize speech from text. The Amazon Polly service provides API operations for synthesizing high-quality speech from plain text and Speech Synthesis Markup Language (SSML), along with managing pronunciations lexicons that enable you to get the best results for your application domain.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-polly/tests/scenario_test.rs`: create a custom lexicon, use it in speech synthesis, list/get it, and remove it.
- Backported from `scenario_test.rs`: start, inspect, and clean up an asynchronous synthesis task.
- Scenario insight from EC2: add full state-machine walks for Amazon Polly resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: model text-to-speech generation, voice and engine discovery, lexicon management, speech marks, asynchronous task state, and output storage references.

## Service Identity and Protocol

- AWS model slug: `polly`
- AWS SDK for Rust slug: `polly`
- Model version: `2016-06-10`
- Model file: `vendor/api-models-aws/models/polly/service/2016-06-10/polly-2016-06-10.json`
- SDK ID: `Polly`
- Endpoint prefix: `polly`
- ARN namespace: `polly`
- CloudFormation name: `Polly`
- CloudTrail event source: `polly.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (2), `List` (2), `Delete` (1), `Describe` (1), `Put` (1), `Start` (1), `Synthesize` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteLexicon`, `PutLexicon`, `StartSpeechSynthesisTask`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeVoices`, `GetLexicon`, `GetSpeechSynthesisTask`, `ListLexicons`, `ListSpeechSynthesisTasks`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetSpeechSynthesisTask`, `ListSpeechSynthesisTasks`, `StartSpeechSynthesisTask`.
- 9 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `SNS`, `ECS`.

## Operation Groups

### Get

- Operations: `GetLexicon`, `GetSpeechSynthesisTask`
- Common required input members in this group: -

### List

- Operations: `ListLexicons`, `ListSpeechSynthesisTasks`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartSpeechSynthesisStream`, `StartSpeechSynthesisTask`
- Common required input members in this group: `OutputFormat`, `VoiceId`

### Delete

- Operations: `DeleteLexicon`
- Common required input members in this group: -

### Describe

- Operations: `DescribeVoices`
- Common required input members in this group: -

### Put

- Operations: `PutLexicon`
- Common required input members in this group: -

### Synthesize

- Operations: `SynthesizeSpeech`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteLexicon` | `DELETE /v1/lexicons/{Name}` | - | `Name` | - | `DeleteLexiconOutput` | `LexiconNotFoundException`, `ServiceFailureException` | Deletes the specified pronunciation lexicon stored in an Amazon Web Services Region. A lexicon which has been deleted is not available for speech synthesis, nor is it possible to retrieve it using either the GetLexic ... |
| `DescribeVoices` | `GET /v1/voices` | - | - | - | `DescribeVoicesOutput` | `InvalidNextTokenException`, `ServiceFailureException` | Returns the list of voices that are available for use when requesting speech synthesis. Each voice speaks a specified language, is either male or female, and is identified by an ID, which is the ASCII version of the ... |
| `GetLexicon` | `GET /v1/lexicons/{Name}` | - | `Name` | - | `GetLexiconOutput` | `LexiconNotFoundException`, `ServiceFailureException` | Returns the content of the specified pronunciation lexicon stored in an Amazon Web Services Region. For more information, see Managing Lexicons . |
| `GetSpeechSynthesisTask` | `GET /v1/synthesisTasks/{TaskId}` | - | `TaskId` | - | `GetSpeechSynthesisTaskOutput` | `InvalidTaskIdException`, `ServiceFailureException`, `SynthesisTaskNotFoundException` | Retrieves a specific SpeechSynthesisTask object based on its TaskID. This object contains information about the given speech synthesis task, including the status of the task, and a link to the S3 bucket containing th ... |
| `ListLexicons` | `GET /v1/lexicons` | - | - | - | `ListLexiconsOutput` | `InvalidNextTokenException`, `ServiceFailureException` | Returns a list of pronunciation lexicons stored in an Amazon Web Services Region. For more information, see Managing Lexicons . |
| `ListSpeechSynthesisTasks` | `GET /v1/synthesisTasks` | `paginated` | - | - | `ListSpeechSynthesisTasksOutput` | `InvalidNextTokenException`, `ServiceFailureException` | Returns a list of SpeechSynthesisTask objects ordered by their creation date. This operation can filter the tasks by their status, for example, allowing users to list only tasks that are completed. |
| `PutLexicon` | `PUT /v1/lexicons/{Name}` | - | `Name`, `Content` | - | `PutLexiconOutput` | `InvalidLexiconException`, `LexiconSizeExceededException`, `MaxLexemeLengthExceededException`, `MaxLexiconsNumberExceededException`, `ServiceFailureException`, `UnsupportedPlsAlphabetException`, `UnsupportedPlsLanguageException` | Stores a pronunciation lexicon in an Amazon Web Services Region. If a lexicon with the same name already exists in the region, it is overwritten by the new lexicon. Lexicon operations have eventual consistency, there ... |
| `StartSpeechSynthesisStream` | `POST /v1/synthesisStream` | - | `Engine`, `OutputFormat`, `VoiceId` | - | `StartSpeechSynthesisStreamOutput` | `ServiceFailureException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Synthesizes UTF-8 input, plain text, or SSML over a bidirectional streaming connection. Specify synthesis parameters in HTTP/2 headers, send text incrementally as events on the input stream, and receive synthesized a ... |
| `StartSpeechSynthesisTask` | `POST /v1/synthesisTasks` | - | `OutputFormat`, `OutputS3BucketName`, `Text`, `VoiceId` | - | `StartSpeechSynthesisTaskOutput` | `EngineNotSupportedException`, `InvalidS3BucketException`, `InvalidS3KeyException`, `InvalidSampleRateException`, `InvalidSnsTopicArnException`, `InvalidSsmlException`, `LanguageNotSupportedException`, `LexiconNotFoundException`, `MarksNotSupportedForFormatException`, `ServiceFailureException`, `SsmlMarksNotSupportedForTextTypeException`, `TextLengthExceededException` | Allows the creation of an asynchronous synthesis task, by starting a new SpeechSynthesisTask . This operation requires all the standard information needed for speech synthesis, plus the name of an Amazon S3 bucket fo ... |
| `SynthesizeSpeech` | `POST /v1/speech` | - | `OutputFormat`, `Text`, `VoiceId` | - | `SynthesizeSpeechOutput` | `EngineNotSupportedException`, `InvalidSampleRateException`, `InvalidSsmlException`, `LanguageNotSupportedException`, `LexiconNotFoundException`, `MarksNotSupportedForFormatException`, `ServiceFailureException`, `SsmlMarksNotSupportedForTextTypeException`, `TextLengthExceededException` | Synthesizes UTF-8 input, plain text or SSML, to a stream of bytes. SSML input must be valid, well-formed SSML. Some alphabets might not be available with all the voices (for example, Cyrillic might not be read at all ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DescribeVoices` | - | `Engine -> Engine`, `LanguageCode -> LanguageCode`, `IncludeAdditionalLanguageCodes -> IncludeAdditionalLanguageCodes`, `NextToken -> NextToken` | - | - |
| `ListLexicons` | - | `NextToken -> NextToken` | - | - |
| `ListSpeechSynthesisTasks` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken`, `Status -> Status` | - | - |
| `StartSpeechSynthesisStream` | `Engine -> x-amzn-Engine`, `LanguageCode -> x-amzn-LanguageCode`, `LexiconNames -> x-amzn-LexiconNames`, `OutputFormat -> x-amzn-OutputFormat`, `SampleRate -> x-amzn-SampleRate`, `VoiceId -> x-amzn-VoiceId` | - | - | `ActionStream` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `EngineNotSupportedException` | `structure` | message | This engine is not compatible with the voice that you have designated. Choose a new voice that is compatible with the engine or change the engine and restar ... |
| `InvalidLexiconException` | `structure` | message | Amazon Polly can't find the specified lexicon. Verify that the lexicon's name is spelled correctly, and then try again. |
| `InvalidNextTokenException` | `structure` | message | The NextToken is invalid. Verify that it's spelled correctly, and then try again. |
| `InvalidS3BucketException` | `structure` | message | The provided Amazon S3 bucket name is invalid. Please check your input with S3 bucket naming requirements and try again. |
| `InvalidS3KeyException` | `structure` | message | The provided Amazon S3 key prefix is invalid. Please provide a valid S3 object key name. |
| `InvalidSampleRateException` | `structure` | message | The specified sample rate is not valid. |
| `InvalidSnsTopicArnException` | `structure` | message | The provided SNS topic ARN is invalid. Please provide a valid SNS topic ARN and try again. |
| `InvalidSsmlException` | `structure` | message | The SSML you provided is invalid. Verify the SSML syntax, spelling of tags and values, and then try again. |
| `InvalidTaskIdException` | `structure` | message | The provided Task ID is not valid. Please provide a valid Task ID and try again. |
| `LanguageNotSupportedException` | `structure` | message | The language specified is not currently supported by Amazon Polly in this capacity. |
| `LexiconNotFoundException` | `structure` | message | Amazon Polly can't find the specified lexicon. This could be caused by a lexicon that is missing, its name is misspelled or specifying a lexicon that is in ... |
| `LexiconSizeExceededException` | `structure` | message | The maximum size of the specified lexicon would be exceeded by this operation. |
| `MarksNotSupportedForFormatException` | `structure` | message | Speech marks are not supported for the OutputFormat selected. Speech marks are only available for content in json format. |
| `MaxLexemeLengthExceededException` | `structure` | message | The maximum size of the lexeme would be exceeded by this operation. |
| `MaxLexiconsNumberExceededException` | `structure` | message | The maximum number of lexicons would be exceeded by this operation. |
| `ServiceFailureException` | `structure` | message | An unknown condition has caused a service failure. |
| `ServiceQuotaExceededException` | `structure` | message, quotaCode, serviceCode | The request would cause a service quota to be exceeded. |
| `SsmlMarksNotSupportedForTextTypeException` | `structure` | message | SSML speech marks are not supported for plain text-type input. |
| `SynthesisTaskNotFoundException` | `structure` | message | The Speech Synthesis task with requested Task ID cannot be found. |
| `TextLengthExceededException` | `structure` | message | The value of the "Text" parameter is longer than the accepted limits. For the SynthesizeSpeech API, the limit for input text is a maximum of 6000 characters ... |
| `ThrottlingException` | `structure` | message, throttlingReasons | The request was denied because of request throttling. |
| `UnsupportedPlsAlphabetException` | `structure` | message | The alphabet specified by the lexicon is not a supported alphabet. Valid values are x-sampa and ipa . |
| `UnsupportedPlsLanguageException` | `structure` | message | The language specified in the lexicon is unsupported. For a list of supported languages, see Lexicon Attributes . |
| `ValidationException` | `structure` | message, reason, fields | The input fails to satisfy the constraints specified by the service. |
| `DeleteLexiconInput` | `structure` | Name | - |
| `DeleteLexiconOutput` | `structure` | **empty (no members)** | - |
| `DescribeVoicesInput` | `structure` | Engine, LanguageCode, IncludeAdditionalLanguageCodes, NextToken | - |
| `DescribeVoicesOutput` | `structure` | Voices, NextToken | - |
| `GetLexiconInput` | `structure` | Name | - |
| `GetLexiconOutput` | `structure` | Lexicon, LexiconAttributes | - |
| `GetSpeechSynthesisTaskInput` | `structure` | TaskId | - |
| `GetSpeechSynthesisTaskOutput` | `structure` | SynthesisTask | - |
| `ListLexiconsInput` | `structure` | NextToken | - |
| `ListLexiconsOutput` | `structure` | Lexicons, NextToken | - |
| `ListSpeechSynthesisTasksInput` | `structure` | MaxResults, NextToken, Status | - |
| `ListSpeechSynthesisTasksOutput` | `structure` | NextToken, SynthesisTasks | - |
| `PutLexiconInput` | `structure` | Name, Content | - |
| `PutLexiconOutput` | `structure` | **empty (no members)** | - |
| `StartSpeechSynthesisStreamInput` | `structure` | Engine, LanguageCode, LexiconNames, OutputFormat, SampleRate, VoiceId, ActionStream | - |
| `StartSpeechSynthesisStreamOutput` | `structure` | EventStream | - |
| `Engine` | `enum` | STANDARD, NEURAL, LONG_FORM, GENERATIVE | - |
| `Gender` | `enum` | Female, Male | - |
| `LanguageCode` | `enum` | arb, cmn_CN, cy_GB, da_DK, de_DE, en_AU, en_GB, en_GB_WLS, en_IN, en_US, es_ES, es_MX, ... (+30) | - |
| `OutputFormat` | `enum` | JSON, MP3, OGG_OPUS, OGG_VORBIS, PCM, MULAW, ALAW | - |
| `QuotaCode` | `enum` | INPUT_STREAM_INBOUND_EVENT_TIMEOUT, INPUT_STREAM_TIMEOUT | - |
| `ServiceCode` | `enum` | POLLY | - |
| `SpeechMarkType` | `enum` | SENTENCE, SSML, VISEME, WORD | - |
| `TaskStatus` | `enum` | SCHEDULED, IN_PROGRESS, COMPLETED, FAILED | - |
| `TextType` | `enum` | SSML, TEXT | - |
| `ValidationExceptionReason` | `enum` | UNSUPPORTED_OPERATION, FIELD_VALIDATION_FAILED, OTHER, INVALID_INBOUND_EVENT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
