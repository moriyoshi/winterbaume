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
- Common required input members in this group: `Name`, `TaskId`

### List

- Operations: `ListLexicons`, `ListSpeechSynthesisTasks`
- Traits: `paginated` (1)

### Delete

- Operations: `DeleteLexicon`
- Common required input members in this group: `Name`

### Describe

- Operations: `DescribeVoices`

### Put

- Operations: `PutLexicon`
- Common required input members in this group: `Content`, `Name`

### Start

- Operations: `StartSpeechSynthesisTask`
- Common required input members in this group: `OutputFormat`, `OutputS3BucketName`, `Text`, `VoiceId`

### Synthesize

- Operations: `SynthesizeSpeech`
- Common required input members in this group: `OutputFormat`, `Text`, `VoiceId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteLexicon` | `DELETE /v1/lexicons/{Name}` | - | `Name` | - | `DeleteLexiconOutput` | `LexiconNotFoundException`, `ServiceFailureException` | Deletes the specified pronunciation lexicon stored in an Amazon Web Services Region. A lexicon which has been deleted is not available for speech synthesis, nor is it possible to retrieve it using either the `GetLexicon` or `ListLexicon` APIs. |
| `DescribeVoices` | `GET /v1/voices` | - | - | - | `DescribeVoicesOutput` | `InvalidNextTokenException`, `ServiceFailureException` | Returns the list of voices that are available for use when requesting speech synthesis. Each voice speaks a specified language, is either male or female, and is identified by an ID, which is the ASCII version of the voice name. |
| `GetLexicon` | `GET /v1/lexicons/{Name}` | - | `Name` | - | `GetLexiconOutput` | `LexiconNotFoundException`, `ServiceFailureException` | Returns the content of the specified pronunciation lexicon stored in an Amazon Web Services Region. For more information, see Managing Lexicons. |
| `GetSpeechSynthesisTask` | `GET /v1/synthesisTasks/{TaskId}` | - | `TaskId` | - | `GetSpeechSynthesisTaskOutput` | `InvalidTaskIdException`, `ServiceFailureException`, `SynthesisTaskNotFoundException` | Retrieves a specific SpeechSynthesisTask object based on its TaskID. This object contains information about the given speech synthesis task, including the status of the task, and a link to the S3 bucket containing the output of the task. |
| `ListLexicons` | `GET /v1/lexicons` | - | - | - | `ListLexiconsOutput` | `InvalidNextTokenException`, `ServiceFailureException` | Returns a list of pronunciation lexicons stored in an Amazon Web Services Region. For more information, see Managing Lexicons. |
| `ListSpeechSynthesisTasks` | `GET /v1/synthesisTasks` | `paginated` | - | - | `ListSpeechSynthesisTasksOutput` | `InvalidNextTokenException`, `ServiceFailureException` | Returns a list of SpeechSynthesisTask objects ordered by their creation date. This operation can filter the tasks by their status, for example, allowing users to list only tasks that are completed. |
| `PutLexicon` | `PUT /v1/lexicons/{Name}` | - | `Content`, `Name` | - | `PutLexiconOutput` | `InvalidLexiconException`, `LexiconSizeExceededException`, `MaxLexemeLengthExceededException`, `MaxLexiconsNumberExceededException`, `ServiceFailureException`, `UnsupportedPlsAlphabetException`, `UnsupportedPlsLanguageException` | Stores a pronunciation lexicon in an Amazon Web Services Region. If a lexicon with the same name already exists in the region, it is overwritten by the new lexicon. |
| `StartSpeechSynthesisTask` | `POST /v1/synthesisTasks` | - | `OutputFormat`, `OutputS3BucketName`, `Text`, `VoiceId` | - | `StartSpeechSynthesisTaskOutput` | `EngineNotSupportedException`, `InvalidS3BucketException`, `InvalidS3KeyException`, `InvalidSampleRateException`, `InvalidSnsTopicArnException`, `InvalidSsmlException`, `LanguageNotSupportedException`, `LexiconNotFoundException`, ... (+4) | Allows the creation of an asynchronous synthesis task, by starting a new `SpeechSynthesisTask`. This operation requires all the standard information needed for speech synthesis, plus the name of an Amazon S3 bucket for the service to store the output of the... |
| `SynthesizeSpeech` | `POST /v1/speech` | - | `OutputFormat`, `Text`, `VoiceId` | - | `SynthesizeSpeechOutput` | `EngineNotSupportedException`, `InvalidSampleRateException`, `InvalidSsmlException`, `LanguageNotSupportedException`, `LexiconNotFoundException`, `MarksNotSupportedForFormatException`, `ServiceFailureException`, `SsmlMarksNotSupportedForTextTypeException`, ... (+1) | Synthesizes UTF-8 input, plain text or SSML, to a stream of bytes. SSML input must be valid, well-formed SSML. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ServiceFailureException` | `structure` | `message` | An unknown condition has caused a service failure. |
| `LexiconNotFoundException` | `structure` | `message` | Amazon Polly can't find the specified lexicon. |
| `InvalidNextTokenException` | `structure` | `message` | The NextToken is invalid. |
| `EngineNotSupportedException` | `structure` | `message` | This engine is not compatible with the voice that you have designated. |
| `InvalidSampleRateException` | `structure` | `message` | The specified sample rate is not valid. |
| `InvalidSsmlException` | `structure` | `message` | The SSML you provided is invalid. |
| `LanguageNotSupportedException` | `structure` | `message` | The language specified is not currently supported by Amazon Polly in this capacity. |
| `MarksNotSupportedForFormatException` | `structure` | `message` | Speech marks are not supported for the `OutputFormat` selected. |
| `SsmlMarksNotSupportedForTextTypeException` | `structure` | `message` | SSML speech marks are not supported for plain text-type input. |
| `TextLengthExceededException` | `structure` | `message` | The value of the "Text" parameter is longer than the accepted limits. |
| `DeleteLexiconInput` | `structure` | `Name` | - |
| `DeleteLexiconOutput` | `structure` | - | - |
| `DescribeVoicesInput` | `structure` | `Engine`, `IncludeAdditionalLanguageCodes`, `LanguageCode`, `NextToken` | - |
| `DescribeVoicesOutput` | `structure` | `NextToken`, `Voices` | - |
| `GetLexiconInput` | `structure` | `Name` | - |
| `GetLexiconOutput` | `structure` | `Lexicon`, `LexiconAttributes` | - |
| `GetSpeechSynthesisTaskInput` | `structure` | `TaskId` | - |
| `GetSpeechSynthesisTaskOutput` | `structure` | `SynthesisTask` | - |
| `InvalidTaskIdException` | `structure` | `message` | The provided Task ID is not valid. |
| `SynthesisTaskNotFoundException` | `structure` | `message` | The Speech Synthesis task with requested Task ID cannot be found. |
| `ListLexiconsInput` | `structure` | `NextToken` | - |
| `ListLexiconsOutput` | `structure` | `Lexicons`, `NextToken` | - |
| `ListSpeechSynthesisTasksInput` | `structure` | `MaxResults`, `NextToken`, `Status` | - |
| `ListSpeechSynthesisTasksOutput` | `structure` | `NextToken`, `SynthesisTasks` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
