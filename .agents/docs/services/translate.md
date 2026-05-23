# Amazon Translate

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provides translation of the input content from the source language to the target language.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Translate resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Translate workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Get`, `Translate`, `Create` operation families, including `ListLanguages`, `ListParallelData`, `ListTagsForResource`, `ListTerminologies`, `DeleteParallelData`, `DeleteTerminology`.

## Service Identity and Protocol

- AWS model slug: `translate`
- AWS SDK for Rust slug: `translate`
- Model version: `2017-07-01`
- Model file: `vendor/api-models-aws/models/translate/service/2017-07-01/translate-2017-07-01.json`
- SDK ID: `Translate`
- Endpoint prefix: `translate`
- ARN namespace: `translate`
- CloudFormation name: `Translate`
- CloudTrail event source: `translate.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Delete` (2), `Get` (2), `Translate` (2), `Create` (1), `Describe` (1), `Import` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateParallelData`, `DeleteParallelData`, `DeleteTerminology`, `ImportTerminology`, `StartTextTranslationJob`, `StopTextTranslationJob`, `TagResource`, `UntagResource`, `UpdateParallelData`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeTextTranslationJob`, `GetParallelData`, `GetTerminology`, `ListLanguages`, `ListParallelData`, `ListTagsForResource`, `ListTerminologies`, `ListTextTranslationJobs`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeTextTranslationJob`, `ImportTerminology`, `ListTextTranslationJobs`, `StartTextTranslationJob`, `StopTextTranslationJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`.

## Operation Groups

### List

- Operations: `ListLanguages`, `ListParallelData`, `ListTagsForResource`, `ListTerminologies`, `ListTextTranslationJobs`
- Traits: `paginated` (4)
- Common required input members in this group: -

### Delete

- Operations: `DeleteParallelData`, `DeleteTerminology`
- Common required input members in this group: `Name`

### Get

- Operations: `GetParallelData`, `GetTerminology`
- Common required input members in this group: `Name`

### Translate

- Operations: `TranslateDocument`, `TranslateText`
- Common required input members in this group: `SourceLanguageCode`, `TargetLanguageCode`

### Create

- Operations: `CreateParallelData`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Describe

- Operations: `DescribeTextTranslationJob`
- Common required input members in this group: -

### Import

- Operations: `ImportTerminology`
- Common required input members in this group: -

### Start

- Operations: `StartTextTranslationJob`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Stop

- Operations: `StopTextTranslationJob`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateParallelData`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateParallelData` | `-` | `idempotency-token` | `Name`, `ParallelDataConfig`, `ClientToken` | `ClientToken` | `CreateParallelDataResponse` | `ConcurrentModificationException`, `ConflictException`, `InternalServerException`, `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `TooManyRequestsException`, `TooManyTagsException` | Creates a parallel data resource in Amazon Translate by importing an input file from Amazon S3. Parallel data files contain examples that show how you want segments of text to be translated. By adding parallel data, ... |
| `DeleteParallelData` | `-` | - | `Name` | - | `DeleteParallelDataResponse` | `ConcurrentModificationException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException` | Deletes a parallel data resource in Amazon Translate. |
| `DeleteTerminology` | `-` | - | `Name` | - | `Unit` | `InternalServerException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `TooManyRequestsException` | A synchronous action that deletes a custom terminology. |
| `DescribeTextTranslationJob` | `-` | - | `JobId` | - | `DescribeTextTranslationJobResponse` | `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with an asynchronous batch translation job including name, ID, status, source and target languages, input/output S3 buckets, and so on. |
| `GetParallelData` | `-` | - | `Name` | - | `GetParallelDataResponse` | `InternalServerException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `TooManyRequestsException` | Provides information about a parallel data resource. |
| `GetTerminology` | `-` | - | `Name` | - | `GetTerminologyResponse` | `InternalServerException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `TooManyRequestsException` | Retrieves a custom terminology. |
| `ImportTerminology` | `-` | - | `Name`, `MergeStrategy`, `TerminologyData` | - | `ImportTerminologyResponse` | `ConcurrentModificationException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `TooManyRequestsException`, `TooManyTagsException` | Creates or updates a custom terminology, depending on whether one already exists for the given terminology name. Importing a terminology with the same name as an existing one will merge the terminologies based on the ... |
| `ListLanguages` | `-` | `paginated` | - | - | `ListLanguagesResponse` | `InternalServerException`, `InvalidParameterValueException`, `TooManyRequestsException`, `UnsupportedDisplayLanguageCodeException` | Provides a list of languages (RFC-5646 codes and names) that Amazon Translate supports. |
| `ListParallelData` | `-` | `paginated` | - | - | `ListParallelDataResponse` | `InternalServerException`, `InvalidParameterValueException`, `TooManyRequestsException` | Provides a list of your parallel data resources in Amazon Translate. |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `InvalidParameterValueException`, `ResourceNotFoundException` | Lists all tags associated with a given Amazon Translate resource. For more information, see Tagging your resources . |
| `ListTerminologies` | `-` | `paginated` | - | - | `ListTerminologiesResponse` | `InternalServerException`, `InvalidParameterValueException`, `TooManyRequestsException` | Provides a list of custom terminologies associated with your account. |
| `ListTextTranslationJobs` | `-` | `paginated` | - | - | `ListTextTranslationJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the batch translation jobs that you have submitted. |
| `StartTextTranslationJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn`, `SourceLanguageCode`, `TargetLanguageCodes`, `ClientToken` | `ClientToken` | `StartTextTranslationJobResponse` | `InternalServerException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException`, `UnsupportedLanguagePairException` | Starts an asynchronous batch translation job. Use batch translation jobs to translate large volumes of text across multiple documents at once. For batch translation, you can input documents with different source lang ... |
| `StopTextTranslationJob` | `-` | - | `JobId` | - | `StopTextTranslationJobResponse` | `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException` | Stops an asynchronous batch translation job that is in progress. If the job's state is IN_PROGRESS , the job will be marked for termination and put into the STOP_REQUESTED state. If the job completes before it can be ... |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ConcurrentModificationException`, `InternalServerException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `TooManyTagsException` | Associates a specific tag with a resource. A tag is a key-value pair that adds as a metadata to a resource. For more information, see Tagging your resources . |
| `TranslateDocument` | `-` | - | `Document`, `SourceLanguageCode`, `TargetLanguageCode` | - | `TranslateDocumentResponse` | `InternalServerException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnsupportedLanguagePairException` | Translates the input document from the source language to the target language. This synchronous operation supports text, HTML, or Word documents as the input document. TranslateDocument supports translations from Eng ... |
| `TranslateText` | `-` | - | `Text`, `SourceLanguageCode`, `TargetLanguageCode` | - | `TranslateTextResponse` | `DetectedLanguageLowConfidenceException`, `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `TextSizeLimitExceededException`, `TooManyRequestsException`, `UnsupportedLanguagePairException` | Translates input text from the source language to the target language. For a list of available languages and language codes, see Supported languages . |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ConcurrentModificationException`, `InternalServerException`, `InvalidParameterValueException`, `ResourceNotFoundException` | Removes a specific tag associated with an Amazon Translate resource. For more information, see Tagging your resources . |
| `UpdateParallelData` | `-` | `idempotency-token` | `Name`, `ParallelDataConfig`, `ClientToken` | `ClientToken` | `UpdateParallelDataResponse` | `ConcurrentModificationException`, `ConflictException`, `InternalServerException`, `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `TooManyRequestsException` | Updates a previously created parallel data resource by importing a new input file from Amazon S3. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConcurrentModificationException` | `structure` | Message | Another modification is being made. That modification must complete before you can make your change. |
| `ConflictException` | `structure` | Message | There was a conflict processing the request. Try your request again. |
| `DetectedLanguageLowConfidenceException` | `structure` | Message, DetectedLanguageCode | The confidence that Amazon Comprehend accurately detected the source language is low. If a low confidence level is acceptable for your application, you can ... |
| `InternalServerException` | `structure` | Message | An internal server error occurred. Retry your request. |
| `InvalidFilterException` | `structure` | Message | The filter specified for the operation is not valid. Specify a different filter. |
| `InvalidParameterValueException` | `structure` | Message | The value of the parameter is not valid. Review the value of the parameter you are using to correct it, and then retry your operation. |
| `InvalidRequestException` | `structure` | Message | The request that you made is not valid. Check your request to determine why it's not valid and then retry the request. |
| `LimitExceededException` | `structure` | Message | The specified limit has been exceeded. Review your request and retry it with a quantity below the stated limit. |
| `ResourceNotFoundException` | `structure` | Message | The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs befor ... |
| `ServiceUnavailableException` | `structure` | Message | The Amazon Translate service is temporarily unavailable. Wait a bit and then retry your request. |
| `TextSizeLimitExceededException` | `structure` | Message | The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. |
| `TooManyRequestsException` | `structure` | Message | You have made too many requests within a short period of time. Wait for a short time and then try your request again. |
| `TooManyTagsException` | `structure` | message, ResourceArn | You have added too many tags to this resource. The maximum is 50 tags. |
| `UnsupportedDisplayLanguageCodeException` | `structure` | Message, DisplayLanguageCode | Requested display language code is not supported. |
| `UnsupportedLanguagePairException` | `structure` | Message, SourceLanguageCode, TargetLanguageCode | Amazon Translate does not support translation from the language of the source text into the requested target language. For more information, see Supported l ... |
| `CreateParallelDataRequest` | `structure` | Name, Description, ParallelDataConfig, EncryptionKey, ClientToken, Tags | - |
| `CreateParallelDataResponse` | `structure` | Name, Status | - |
| `DeleteParallelDataRequest` | `structure` | Name | - |
| `DeleteParallelDataResponse` | `structure` | Name, Status | - |
| `DeleteTerminologyRequest` | `structure` | Name | - |
| `DescribeTextTranslationJobRequest` | `structure` | JobId | - |
| `DescribeTextTranslationJobResponse` | `structure` | TextTranslationJobProperties | - |
| `GetParallelDataRequest` | `structure` | Name | - |
| `GetParallelDataResponse` | `structure` | ParallelDataProperties, DataLocation, AuxiliaryDataLocation, LatestUpdateAttemptAuxiliaryDataLocation | - |
| `GetTerminologyRequest` | `structure` | Name, TerminologyDataFormat | - |
| `GetTerminologyResponse` | `structure` | TerminologyProperties, TerminologyDataLocation, AuxiliaryDataLocation | - |
| `ImportTerminologyRequest` | `structure` | Name, MergeStrategy, Description, TerminologyData, EncryptionKey, Tags | - |
| `ImportTerminologyResponse` | `structure` | TerminologyProperties, AuxiliaryDataLocation | - |
| `ListLanguagesRequest` | `structure` | DisplayLanguageCode, NextToken, MaxResults | - |
| `ListLanguagesResponse` | `structure` | Languages, DisplayLanguageCode, NextToken | - |
| `ListParallelDataRequest` | `structure` | NextToken, MaxResults | - |
| `ListParallelDataResponse` | `structure` | ParallelDataPropertiesList, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `ListTerminologiesRequest` | `structure` | NextToken, MaxResults | - |
| `ListTerminologiesResponse` | `structure` | TerminologyPropertiesList, NextToken | - |
| `ListTextTranslationJobsRequest` | `structure` | Filter, NextToken, MaxResults | - |
| `ListTextTranslationJobsResponse` | `structure` | TextTranslationJobPropertiesList, NextToken | - |
| `StartTextTranslationJobRequest` | `structure` | JobName, InputDataConfig, OutputDataConfig, DataAccessRoleArn, SourceLanguageCode, TargetLanguageCodes, TerminologyNames, ParallelDataNames, ClientToken, Settings | - |
| `StartTextTranslationJobResponse` | `structure` | JobId, JobStatus | - |
| `Brevity` | `enum` | ON | - |
| `Directionality` | `enum` | UNI, MULTI | - |
| `DisplayLanguageCode` | `enum` | DE, EN, ES, FR, IT, JA, KO, PT, ZH, ZH_TW | - |
| `EncryptionKeyType` | `enum` | KMS | - |
| `Formality` | `enum` | FORMAL, INFORMAL | - |
| `JobStatus` | `enum` | SUBMITTED, IN_PROGRESS, COMPLETED, COMPLETED_WITH_ERROR, FAILED, STOP_REQUESTED, STOPPED | - |
| `MergeStrategy` | `enum` | OVERWRITE | - |
| `ParallelDataFormat` | `enum` | TSV, CSV, TMX | - |
| `ParallelDataStatus` | `enum` | CREATING, UPDATING, ACTIVE, DELETING, FAILED | - |
| `Profanity` | `enum` | MASK | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
