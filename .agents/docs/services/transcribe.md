# Amazon Transcribe Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Transcribe offers three main types of batch transcription: Standard , Medical , and Call Analytics . Standard transcriptions are the most common option. Refer to for details. Medical transcriptions are tailored to medical professionals and incorporate medical terms. A common use case for this service is transcribing doctor-patient dialogue into after-visit notes.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Transcribe Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Transcribe Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Get`, `Create`, `Start` operation families, including `ListCallAnalyticsCategories`, `ListCallAnalyticsJobs`, `ListLanguageModels`, `ListMedicalScribeJobs`, `DeleteCallAnalyticsCategory`, `DeleteCallAnalyticsJob`.

## Service Identity and Protocol

- AWS model slug: `transcribe`
- AWS SDK for Rust slug: `transcribe`
- Model version: `2017-10-26`
- Model file: `vendor/api-models-aws/models/transcribe/service/2017-10-26/transcribe-2017-10-26.json`
- SDK ID: `Transcribe`
- Endpoint prefix: `transcribe`
- ARN namespace: `transcribe`
- CloudFormation name: `Transcribe`
- CloudTrail event source: `transcribe.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Delete` (9), `Get` (8), `Create` (5), `Start` (4), `Update` (4), `Describe` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCallAnalyticsCategory`, `CreateLanguageModel`, `CreateMedicalVocabulary`, `CreateVocabulary`, `CreateVocabularyFilter`, `DeleteCallAnalyticsCategory`, `DeleteCallAnalyticsJob`, `DeleteLanguageModel`, `DeleteMedicalScribeJob`, `DeleteMedicalTranscriptionJob`, `DeleteMedicalVocabulary`, `DeleteTranscriptionJob`, `DeleteVocabulary`, `DeleteVocabularyFilter`, `StartCallAnalyticsJob`, `StartMedicalScribeJob`, `StartMedicalTranscriptionJob`, `StartTranscriptionJob`, `TagResource`, `UntagResource`, ... (+4).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeLanguageModel`, `GetCallAnalyticsCategory`, `GetCallAnalyticsJob`, `GetMedicalScribeJob`, `GetMedicalTranscriptionJob`, `GetMedicalVocabulary`, `GetTranscriptionJob`, `GetVocabulary`, `GetVocabularyFilter`, `ListCallAnalyticsCategories`, `ListCallAnalyticsJobs`, `ListLanguageModels`, `ListMedicalScribeJobs`, `ListMedicalTranscriptionJobs`, `ListMedicalVocabularies`, `ListTagsForResource`, `ListTranscriptionJobs`, `ListVocabularies`, `ListVocabularyFilters`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteCallAnalyticsJob`, `DeleteMedicalScribeJob`, `DeleteMedicalTranscriptionJob`, `DeleteTranscriptionJob`, `GetCallAnalyticsJob`, `GetMedicalScribeJob`, `GetMedicalTranscriptionJob`, `GetTranscriptionJob`, `ListCallAnalyticsJobs`, `ListMedicalScribeJobs`, `ListMedicalTranscriptionJobs`, `ListTranscriptionJobs`, `StartCallAnalyticsJob`, `StartMedicalScribeJob`, `StartMedicalTranscriptionJob`, `StartTranscriptionJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 43 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListCallAnalyticsCategories`, `ListCallAnalyticsJobs`, `ListLanguageModels`, `ListMedicalScribeJobs`, `ListMedicalTranscriptionJobs`, `ListMedicalVocabularies`, `ListTagsForResource`, `ListTranscriptionJobs`, `ListVocabularies`, `ListVocabularyFilters`
- Traits: `paginated` (9)
- Common required input members in this group: `ResourceArn`

### Delete

- Operations: `DeleteCallAnalyticsCategory`, `DeleteCallAnalyticsJob`, `DeleteLanguageModel`, `DeleteMedicalScribeJob`, `DeleteMedicalTranscriptionJob`, `DeleteMedicalVocabulary`, `DeleteTranscriptionJob`, `DeleteVocabulary`, `DeleteVocabularyFilter`
- Common required input members in this group: `CallAnalyticsJobName`, `CategoryName`, `MedicalScribeJobName`, `MedicalTranscriptionJobName`, `ModelName`, `TranscriptionJobName`, `VocabularyFilterName`, `VocabularyName`

### Get

- Operations: `GetCallAnalyticsCategory`, `GetCallAnalyticsJob`, `GetMedicalScribeJob`, `GetMedicalTranscriptionJob`, `GetMedicalVocabulary`, `GetTranscriptionJob`, `GetVocabulary`, `GetVocabularyFilter`
- Common required input members in this group: `CallAnalyticsJobName`, `CategoryName`, `MedicalScribeJobName`, `MedicalTranscriptionJobName`, `TranscriptionJobName`, `VocabularyFilterName`, `VocabularyName`

### Create

- Operations: `CreateCallAnalyticsCategory`, `CreateLanguageModel`, `CreateMedicalVocabulary`, `CreateVocabulary`, `CreateVocabularyFilter`
- Common required input members in this group: `BaseModelName`, `CategoryName`, `InputDataConfig`, `LanguageCode`, `ModelName`, `Rules`, `VocabularyFileUri`, `VocabularyFilterName`, `VocabularyName`

### Start

- Operations: `StartCallAnalyticsJob`, `StartMedicalScribeJob`, `StartMedicalTranscriptionJob`, `StartTranscriptionJob`
- Common required input members in this group: `CallAnalyticsJobName`, `DataAccessRoleArn`, `LanguageCode`, `Media`, `MedicalScribeJobName`, `MedicalTranscriptionJobName`, `OutputBucketName`, `Settings`, `Specialty`, `TranscriptionJobName`, `Type`

### Update

- Operations: `UpdateCallAnalyticsCategory`, `UpdateMedicalVocabulary`, `UpdateVocabulary`, `UpdateVocabularyFilter`
- Common required input members in this group: `CategoryName`, `LanguageCode`, `Rules`, `VocabularyFileUri`, `VocabularyFilterName`, `VocabularyName`

### Describe

- Operations: `DescribeLanguageModel`
- Common required input members in this group: `ModelName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCallAnalyticsCategory` | `PUT /callanalyticscategories/{CategoryName}` | - | `CategoryName`, `Rules` | - | `CreateCallAnalyticsCategoryResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Creates a new Call Analytics category. All categories are automatically applied to your Call Analytics transcriptions. |
| `CreateLanguageModel` | `PUT /languagemodels/{ModelName}` | - | `BaseModelName`, `InputDataConfig`, `LanguageCode`, `ModelName` | - | `CreateLanguageModelResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Creates a new custom language model. When creating a new custom language model, you must specify: If you want a Wideband (audio sample rates over 16,000 Hz) or Narrowband (audio sample rates under 16,000 Hz) base model The location of your training and tuning... |
| `CreateMedicalVocabulary` | `PUT /medicalvocabularies/{VocabularyName}` | - | `LanguageCode`, `VocabularyFileUri`, `VocabularyName` | - | `CreateMedicalVocabularyResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Creates a new custom medical vocabulary. Before creating a new custom medical vocabulary, you must first upload a text file that contains your vocabulary table into an Amazon S3 bucket. |
| `CreateVocabulary` | `PUT /vocabularies/{VocabularyName}` | - | `LanguageCode`, `VocabularyName` | - | `CreateVocabularyResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Creates a new custom vocabulary. When creating a new custom vocabulary, you can either upload a text file that contains your new entries, phrases, and terms into an Amazon S3 bucket and include the URI in your request. |
| `CreateVocabularyFilter` | `POST /vocabularyFilters/{VocabularyFilterName}` | - | `LanguageCode`, `VocabularyFilterName` | - | `CreateVocabularyFilterResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Creates a new custom vocabulary filter. You can use custom vocabulary filters to mask, delete, or flag specific words from your transcript. |
| `DeleteCallAnalyticsCategory` | `DELETE /callanalyticscategories/{CategoryName}` | - | `CategoryName` | - | `DeleteCallAnalyticsCategoryResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Deletes a Call Analytics category. To use this operation, specify the name of the category you want to delete using `CategoryName`. |
| `DeleteCallAnalyticsJob` | `DELETE /callanalyticsjobs/{CallAnalyticsJobName}` | - | `CallAnalyticsJobName` | - | `DeleteCallAnalyticsJobResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Deletes a Call Analytics job. To use this operation, specify the name of the job you want to delete using `CallAnalyticsJobName`. |
| `DeleteLanguageModel` | `DELETE /languagemodels/{ModelName}` | - | `ModelName` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Deletes a custom language model. To use this operation, specify the name of the language model you want to delete using `ModelName`. |
| `DeleteMedicalScribeJob` | `DELETE /medicalscribejobs/{MedicalScribeJobName}` | - | `MedicalScribeJobName` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Deletes a Medical Scribe job. To use this operation, specify the name of the job you want to delete using `MedicalScribeJobName`. |
| `DeleteMedicalTranscriptionJob` | `DELETE /medicaltranscriptionjobs/{MedicalTranscriptionJobName}` | - | `MedicalTranscriptionJobName` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Deletes a medical transcription job. To use this operation, specify the name of the job you want to delete using `MedicalTranscriptionJobName`. |
| `DeleteMedicalVocabulary` | `DELETE /medicalvocabularies/{VocabularyName}` | - | `VocabularyName` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Deletes a custom medical vocabulary. To use this operation, specify the name of the custom vocabulary you want to delete using `VocabularyName`. |
| `DeleteTranscriptionJob` | `DELETE /transcriptionjobs/{TranscriptionJobName}` | - | `TranscriptionJobName` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Deletes a transcription job. To use this operation, specify the name of the job you want to delete using `TranscriptionJobName`. |
| `DeleteVocabulary` | `DELETE /vocabularies/{VocabularyName}` | - | `VocabularyName` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Deletes a custom vocabulary. To use this operation, specify the name of the custom vocabulary you want to delete using `VocabularyName`. |
| `DeleteVocabularyFilter` | `DELETE /vocabularyFilters/{VocabularyFilterName}` | - | `VocabularyFilterName` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Deletes a custom vocabulary filter. To use this operation, specify the name of the custom vocabulary filter you want to delete using `VocabularyFilterName`. |
| `DescribeLanguageModel` | `GET /languagemodels/{ModelName}` | - | `ModelName` | - | `DescribeLanguageModelResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified custom language model. This operation also shows if the base language model that you used to create your custom language model has been updated. |
| `GetCallAnalyticsCategory` | `GET /callanalyticscategories/{CategoryName}` | - | `CategoryName` | - | `GetCallAnalyticsCategoryResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified Call Analytics category. To get a list of your Call Analytics categories, use the operation. |
| `GetCallAnalyticsJob` | `GET /callanalyticsjobs/{CallAnalyticsJobName}` | - | `CallAnalyticsJobName` | - | `GetCallAnalyticsJobResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified Call Analytics job. To view the job's status, refer to `CallAnalyticsJobStatus`. |
| `GetMedicalScribeJob` | `GET /medicalscribejobs/{MedicalScribeJobName}` | - | `MedicalScribeJobName` | - | `GetMedicalScribeJobResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified Medical Scribe job. To view the status of the specified medical transcription job, check the `MedicalScribeJobStatus` field. |
| `GetMedicalTranscriptionJob` | `GET /medicaltranscriptionjobs/{MedicalTranscriptionJobName}` | - | `MedicalTranscriptionJobName` | - | `GetMedicalTranscriptionJobResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified medical transcription job. To view the status of the specified medical transcription job, check the `TranscriptionJobStatus` field. |
| `GetMedicalVocabulary` | `GET /medicalvocabularies/{VocabularyName}` | - | `VocabularyName` | - | `GetMedicalVocabularyResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified custom medical vocabulary. To view the status of the specified custom medical vocabulary, check the `VocabularyState` field. |
| `GetTranscriptionJob` | `GET /transcriptionjobs/{TranscriptionJobName}` | - | `TranscriptionJobName` | - | `GetTranscriptionJobResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified transcription job. To view the status of the specified transcription job, check the `TranscriptionJobStatus` field. |
| `GetVocabulary` | `GET /vocabularies/{VocabularyName}` | - | `VocabularyName` | - | `GetVocabularyResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified custom vocabulary. To view the status of the specified custom vocabulary, check the `VocabularyState` field. |
| `GetVocabularyFilter` | `GET /vocabularyFilters/{VocabularyFilterName}` | - | `VocabularyFilterName` | - | `GetVocabularyFilterResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides information about the specified custom vocabulary filter. To get a list of your custom vocabulary filters, use the operation. |
| `ListCallAnalyticsCategories` | `GET /callanalyticscategories` | `paginated` | - | - | `ListCallAnalyticsCategoriesResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of Call Analytics categories, including all rules that make up each category. To get detailed information about a specific Call Analytics category, use the operation. |
| `ListCallAnalyticsJobs` | `GET /callanalyticsjobs` | `paginated` | - | - | `ListCallAnalyticsJobsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of Call Analytics jobs that match the specified criteria. If no criteria are specified, all Call Analytics jobs are returned. |
| `ListLanguageModels` | `GET /languagemodels` | `paginated` | - | - | `ListLanguageModelsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of custom language models that match the specified criteria. If no criteria are specified, all custom language models are returned. |
| `ListMedicalScribeJobs` | `GET /medicalscribejobs` | `paginated` | - | - | `ListMedicalScribeJobsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of Medical Scribe jobs that match the specified criteria. If no criteria are specified, all Medical Scribe jobs are returned. |
| `ListMedicalTranscriptionJobs` | `GET /medicaltranscriptionjobs` | `paginated` | - | - | `ListMedicalTranscriptionJobsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of medical transcription jobs that match the specified criteria. If no criteria are specified, all medical transcription jobs are returned. |
| `ListMedicalVocabularies` | `GET /medicalvocabularies` | `paginated` | - | - | `ListMedicalVocabulariesResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of custom medical vocabularies that match the specified criteria. If no criteria are specified, all custom medical vocabularies are returned. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Lists all tags associated with the specified transcription job, vocabulary, model, or resource. To learn more about using tags with Amazon Transcribe, refer to Tagging resources. |
| `ListTranscriptionJobs` | `GET /transcriptionjobs` | `paginated` | - | - | `ListTranscriptionJobsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of transcription jobs that match the specified criteria. If no criteria are specified, all transcription jobs are returned. |
| `ListVocabularies` | `GET /vocabularies` | `paginated` | - | - | `ListVocabulariesResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of custom vocabularies that match the specified criteria. If no criteria are specified, all custom vocabularies are returned. |
| `ListVocabularyFilters` | `GET /vocabularyFilters` | `paginated` | - | - | `ListVocabularyFiltersResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Provides a list of custom vocabulary filters that match the specified criteria. If no criteria are specified, all custom vocabularies are returned. |
| `StartCallAnalyticsJob` | `PUT /callanalyticsjobs/{CallAnalyticsJobName}` | - | `CallAnalyticsJobName`, `Media` | - | `StartCallAnalyticsJobResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Transcribes the audio from a customer service call and applies any additional Request Parameters you choose to include in your request. In addition to many standard transcription features, Call Analytics provides you with call characteristics, call... |
| `StartMedicalScribeJob` | `PUT /medicalscribejobs/{MedicalScribeJobName}` | - | `DataAccessRoleArn`, `Media`, `MedicalScribeJobName`, `OutputBucketName`, `Settings` | - | `StartMedicalScribeJobResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Transcribes patient-clinician conversations and generates clinical notes. Amazon Web Services HealthScribe automatically provides rich conversation transcripts, identifies speaker roles, classifies dialogues, extracts medical terms, and generates preliminary... |
| `StartMedicalTranscriptionJob` | `PUT /medicaltranscriptionjobs/{MedicalTranscriptionJobName}` | - | `LanguageCode`, `Media`, `MedicalTranscriptionJobName`, `OutputBucketName`, `Specialty`, `Type` | - | `StartMedicalTranscriptionJobResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Transcribes the audio from a medical dictation or conversation and applies any additional Request Parameters you choose to include in your request. In addition to many standard transcription features, Amazon Transcribe Medical provides you with a robust... |
| `StartTranscriptionJob` | `PUT /transcriptionjobs/{TranscriptionJobName}` | - | `Media`, `TranscriptionJobName` | - | `StartTranscriptionJobResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException` | Transcribes the audio from a media file and applies any additional Request Parameters you choose to include in your request. To make a `StartTranscriptionJob` request, you must first upload your media file into an Amazon S3 bucket; you can then specify the... |
| `TagResource` | `PUT /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Adds one or more custom tags, each in the form of a key:value pair, to the specified resource. To learn more about using tags with Amazon Transcribe, refer to Tagging resources. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Removes the specified tags from the specified Amazon Transcribe resource. If you include `UntagResource` in your request, you must also include `ResourceArn` and `TagKeys`. |
| `UpdateCallAnalyticsCategory` | `PATCH /callanalyticscategories/{CategoryName}` | - | `CategoryName`, `Rules` | - | `UpdateCallAnalyticsCategoryResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Updates the specified Call Analytics category with new rules. Note that the `UpdateCallAnalyticsCategory` operation overwrites all existing rules contained in the specified category. |
| `UpdateMedicalVocabulary` | `PATCH /medicalvocabularies/{VocabularyName}` | - | `LanguageCode`, `VocabularyFileUri`, `VocabularyName` | - | `UpdateMedicalVocabularyResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Updates an existing custom medical vocabulary with new values. This operation overwrites all existing information with your new values; you cannot append new terms onto an existing custom vocabulary. |
| `UpdateVocabulary` | `PATCH /vocabularies/{VocabularyName}` | - | `LanguageCode`, `VocabularyName` | - | `UpdateVocabularyResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Updates an existing custom vocabulary with new values. This operation overwrites all existing information with your new values; you cannot append new terms onto an existing custom vocabulary. |
| `UpdateVocabularyFilter` | `PUT /vocabularyFilters/{VocabularyFilterName}` | - | `VocabularyFilterName` | - | `UpdateVocabularyFilterResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Updates an existing custom vocabulary filter with a new list of words. The new list you provide overwrites all previous entries; you cannot append new terms onto an existing custom vocabulary filter. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message` | Your request didn't pass one or more validation tests. |
| `InternalFailureException` | `structure` | `Message` | There was an internal error. |
| `LimitExceededException` | `structure` | `Message` | You've either sent too many requests or your input file is too long. |
| `NotFoundException` | `structure` | `Message` | We can't find the requested resource. |
| `ConflictException` | `structure` | `Message` | A resource already exists with this name. |
| `CreateCallAnalyticsCategoryRequest` | `structure` | `CategoryName`, `InputType`, `Rules`, `Tags` | - |
| `CreateCallAnalyticsCategoryResponse` | `structure` | `CategoryProperties` | - |
| `CreateLanguageModelRequest` | `structure` | `BaseModelName`, `InputDataConfig`, `LanguageCode`, `ModelName`, `Tags` | - |
| `CreateLanguageModelResponse` | `structure` | `BaseModelName`, `InputDataConfig`, `LanguageCode`, `ModelName`, `ModelStatus` | - |
| `CreateMedicalVocabularyRequest` | `structure` | `LanguageCode`, `Tags`, `VocabularyFileUri`, `VocabularyName` | - |
| `CreateMedicalVocabularyResponse` | `structure` | `FailureReason`, `LanguageCode`, `LastModifiedTime`, `VocabularyName`, `VocabularyState` | - |
| `CreateVocabularyRequest` | `structure` | `DataAccessRoleArn`, `LanguageCode`, `Phrases`, `Tags`, `VocabularyFileUri`, `VocabularyName` | - |
| `CreateVocabularyResponse` | `structure` | `FailureReason`, `LanguageCode`, `LastModifiedTime`, `VocabularyName`, `VocabularyState` | - |
| `CreateVocabularyFilterRequest` | `structure` | `DataAccessRoleArn`, `LanguageCode`, `Tags`, `VocabularyFilterFileUri`, `VocabularyFilterName`, `Words` | - |
| `CreateVocabularyFilterResponse` | `structure` | `LanguageCode`, `LastModifiedTime`, `VocabularyFilterName` | - |
| `DeleteCallAnalyticsCategoryRequest` | `structure` | `CategoryName` | - |
| `DeleteCallAnalyticsCategoryResponse` | `structure` | - | - |
| `DeleteCallAnalyticsJobRequest` | `structure` | `CallAnalyticsJobName` | - |
| `DeleteCallAnalyticsJobResponse` | `structure` | - | - |
| `DeleteLanguageModelRequest` | `structure` | `ModelName` | - |
| `DeleteMedicalScribeJobRequest` | `structure` | `MedicalScribeJobName` | - |
| `DeleteMedicalTranscriptionJobRequest` | `structure` | `MedicalTranscriptionJobName` | - |
| `DeleteMedicalVocabularyRequest` | `structure` | `VocabularyName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
