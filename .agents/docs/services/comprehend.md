# Amazon Comprehend

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Comprehend is an Amazon Web Services service for gaining insight into the content of documents. Use these actions to determine the topics contained in your documents, the topics they discuss, the predominant sentiment expressed in them, the predominant language used, and more.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Comprehend resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Comprehend workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Start`, `Stop`, `Detect` operation families, including `ListDatasets`, `ListDocumentClassificationJobs`, `ListDocumentClassifierSummaries`, `ListDocumentClassifiers`, `DescribeDataset`, `DescribeDocumentClassificationJob`.

## Service Identity and Protocol

- AWS model slug: `comprehend`
- AWS SDK for Rust slug: `comprehend`
- Model version: `2017-11-27`
- Model file: `vendor/api-models-aws/models/comprehend/service/2017-11-27/comprehend-2017-11-27.json`
- SDK ID: `Comprehend`
- Endpoint prefix: `comprehend`
- ARN namespace: `comprehend`
- CloudFormation name: `Comprehend`
- CloudTrail event source: `comprehend.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (18), `Describe` (16), `Start` (10), `Stop` (9), `Detect` (8), `Batch` (6), `Create` (5), `Delete` (5).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchDetectDominantLanguage`, `BatchDetectEntities`, `BatchDetectKeyPhrases`, `BatchDetectSentiment`, `BatchDetectSyntax`, `BatchDetectTargetedSentiment`, `CreateDataset`, `CreateDocumentClassifier`, `CreateEndpoint`, `CreateEntityRecognizer`, `CreateFlywheel`, `DeleteDocumentClassifier`, `DeleteEndpoint`, `DeleteEntityRecognizer`, `DeleteFlywheel`, `DeleteResourcePolicy`, `ImportModel`, `PutResourcePolicy`, `StartDocumentClassificationJob`, `StartDominantLanguageDetectionJob`, ... (+21).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDataset`, `DescribeDocumentClassificationJob`, `DescribeDocumentClassifier`, `DescribeDominantLanguageDetectionJob`, `DescribeEndpoint`, `DescribeEntitiesDetectionJob`, `DescribeEntityRecognizer`, `DescribeEventsDetectionJob`, `DescribeFlywheel`, `DescribeFlywheelIteration`, `DescribeKeyPhrasesDetectionJob`, `DescribePiiEntitiesDetectionJob`, `DescribeResourcePolicy`, `DescribeSentimentDetectionJob`, `DescribeTargetedSentimentDetectionJob`, `DescribeTopicsDetectionJob`, `ListDatasets`, `ListDocumentClassificationJobs`, `ListDocumentClassifierSummaries`, `ListDocumentClassifiers`, ... (+14).
- Pagination is modelled for 17 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeDocumentClassificationJob`, `DescribeDominantLanguageDetectionJob`, `DescribeEntitiesDetectionJob`, `DescribeEventsDetectionJob`, `DescribeKeyPhrasesDetectionJob`, `DescribePiiEntitiesDetectionJob`, `DescribeSentimentDetectionJob`, `DescribeTargetedSentimentDetectionJob`, `DescribeTopicsDetectionJob`, `ImportModel`, `ListDocumentClassificationJobs`, `ListDominantLanguageDetectionJobs`, `ListEntitiesDetectionJobs`, `ListEventsDetectionJobs`, `ListKeyPhrasesDetectionJobs`, `ListPiiEntitiesDetectionJobs`, `ListSentimentDetectionJobs`, `ListTargetedSentimentDetectionJobs`, `ListTopicsDetectionJobs`, `StartDocumentClassificationJob`, ... (+18).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 85 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/comprehend/latest/dg/what-is.html
- https://docs.aws.amazon.com/comprehend/latest/dg/how-entities.html
- https://docs.aws.amazon.com/comprehend/latest/dg/how-document-classification.html

Research outcomes:
- Amazon Comprehend is an NLP service that detects entities, key phrases, sentiment, PII, language, syntax, topics, and custom insights.
- Synchronous APIs analyse single documents or small batches depending on operation.
- Asynchronous jobs process larger inputs from S3 and write outputs to S3.
- Entity detection returns detected real-world entities with types, offsets, and confidence scores.
- Custom classification trains custom models and can classify documents synchronously or asynchronously.
- Topic modelling clusters documents and is job-based.
- Flywheels manage iterative model improvement for custom models.

Parity implications:
- Model synchronous analyses, asynchronous jobs, input/output S3 configuration, custom classifiers, entity recognisers, document classifiers, endpoints, topics jobs, and flywheels separately.
- Job APIs should expose submitted/in-progress/completed/failed status and output locations.
- Custom-model APIs should validate model training state before inference.

## Operation Groups

### List

- Operations: `ListDatasets`, `ListDocumentClassificationJobs`, `ListDocumentClassifiers`, `ListDocumentClassifierSummaries`, `ListDominantLanguageDetectionJobs`, `ListEndpoints`, `ListEntitiesDetectionJobs`, `ListEntityRecognizers`, `ListEntityRecognizerSummaries`, `ListEventsDetectionJobs`, `ListFlywheelIterationHistory`, `ListFlywheels`, `ListKeyPhrasesDetectionJobs`, `ListPiiEntitiesDetectionJobs`, `ListSentimentDetectionJobs`, `ListTagsForResource`, `ListTargetedSentimentDetectionJobs`, `ListTopicsDetectionJobs`
- Traits: `paginated` (17)
- Common required input members in this group: -

### Describe

- Operations: `DescribeDataset`, `DescribeDocumentClassificationJob`, `DescribeDocumentClassifier`, `DescribeDominantLanguageDetectionJob`, `DescribeEndpoint`, `DescribeEntitiesDetectionJob`, `DescribeEntityRecognizer`, `DescribeEventsDetectionJob`, `DescribeFlywheel`, `DescribeFlywheelIteration`, `DescribeKeyPhrasesDetectionJob`, `DescribePiiEntitiesDetectionJob`, `DescribeResourcePolicy`, `DescribeSentimentDetectionJob`, `DescribeTargetedSentimentDetectionJob`, `DescribeTopicsDetectionJob`
- Common required input members in this group: `JobId`, `FlywheelArn`

### Start

- Operations: `StartDocumentClassificationJob`, `StartDominantLanguageDetectionJob`, `StartEntitiesDetectionJob`, `StartEventsDetectionJob`, `StartFlywheelIteration`, `StartKeyPhrasesDetectionJob`, `StartPiiEntitiesDetectionJob`, `StartSentimentDetectionJob`, `StartTargetedSentimentDetectionJob`, `StartTopicsDetectionJob`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn`, `LanguageCode`

### Stop

- Operations: `StopDominantLanguageDetectionJob`, `StopEntitiesDetectionJob`, `StopEventsDetectionJob`, `StopKeyPhrasesDetectionJob`, `StopPiiEntitiesDetectionJob`, `StopSentimentDetectionJob`, `StopTargetedSentimentDetectionJob`, `StopTrainingDocumentClassifier`, `StopTrainingEntityRecognizer`
- Common required input members in this group: `JobId`

### Detect

- Operations: `DetectDominantLanguage`, `DetectEntities`, `DetectKeyPhrases`, `DetectPiiEntities`, `DetectSentiment`, `DetectSyntax`, `DetectTargetedSentiment`, `DetectToxicContent`
- Common required input members in this group: `Text`, `LanguageCode`

### Batch

- Operations: `BatchDetectDominantLanguage`, `BatchDetectEntities`, `BatchDetectKeyPhrases`, `BatchDetectSentiment`, `BatchDetectSyntax`, `BatchDetectTargetedSentiment`
- Common required input members in this group: `TextList`, `LanguageCode`

### Create

- Operations: `CreateDataset`, `CreateDocumentClassifier`, `CreateEndpoint`, `CreateEntityRecognizer`, `CreateFlywheel`
- Traits: `idempotency-token` (5)
- Common required input members in this group: `InputDataConfig`, `DataAccessRoleArn`, `LanguageCode`

### Delete

- Operations: `DeleteDocumentClassifier`, `DeleteEndpoint`, `DeleteEntityRecognizer`, `DeleteFlywheel`, `DeleteResourcePolicy`
- Common required input members in this group: -

### Update

- Operations: `UpdateEndpoint`, `UpdateFlywheel`
- Common required input members in this group: -

### Classify

- Operations: `ClassifyDocument`
- Common required input members in this group: -

### Contains

- Operations: `ContainsPiiEntities`
- Common required input members in this group: -

### Import

- Operations: `ImportModel`
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchDetectDominantLanguage` | `-` | - | `TextList` | - | `BatchDetectDominantLanguageResponse` | `BatchSizeLimitExceededException`, `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException` | Determines the dominant language of the input text for a batch of documents. For a list of languages that Amazon Comprehend can detect, see Amazon Comprehend Supported Languages . |
| `BatchDetectEntities` | `-` | - | `TextList`, `LanguageCode` | - | `BatchDetectEntitiesResponse` | `BatchSizeLimitExceededException`, `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects the text of a batch of documents for named entities and returns information about them. For more information about named entities, see Entities in the Comprehend Developer Guide. |
| `BatchDetectKeyPhrases` | `-` | - | `TextList`, `LanguageCode` | - | `BatchDetectKeyPhrasesResponse` | `BatchSizeLimitExceededException`, `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Detects the key noun phrases found in a batch of documents. |
| `BatchDetectSentiment` | `-` | - | `TextList`, `LanguageCode` | - | `BatchDetectSentimentResponse` | `BatchSizeLimitExceededException`, `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects a batch of documents and returns an inference of the prevailing sentiment, POSITIVE , NEUTRAL , MIXED , or NEGATIVE , in each one. |
| `BatchDetectSyntax` | `-` | - | `TextList`, `LanguageCode` | - | `BatchDetectSyntaxResponse` | `BatchSizeLimitExceededException`, `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects the text of a batch of documents for the syntax and part of speech of the words in the document and returns information about them. For more information, see Syntax in the Comprehend Developer Guide. |
| `BatchDetectTargetedSentiment` | `-` | - | `TextList`, `LanguageCode` | - | `BatchDetectTargetedSentimentResponse` | `BatchSizeLimitExceededException`, `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects a batch of documents and returns a sentiment analysis for each entity identified in the documents. For more information about targeted sentiment, see Targeted sentiment in the Amazon Comprehend Developer Guide . |
| `ClassifyDocument` | `-` | - | `EndpointArn` | - | `ClassifyDocumentResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceUnavailableException`, `TextSizeLimitExceededException` | Creates a classification request to analyze a single document in real-time. ClassifyDocument supports the following model types: Custom classifier - a custom model that you have created and trained. For input, you ca ... |
| `ContainsPiiEntities` | `-` | - | `Text`, `LanguageCode` | - | `ContainsPiiEntitiesResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Analyzes input text for the presence of personally identifiable information (PII) and returns the labels of identified PII entity types such as name, address, bank account number, or phone number. |
| `CreateDataset` | `-` | `idempotency-token` | `FlywheelArn`, `DatasetName`, `InputDataConfig` | `ClientRequestToken` | `CreateDatasetResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `TooManyRequestsException`, `TooManyTagsException` | Creates a dataset to upload training or test data for a model associated with a flywheel. For more information about datasets, see Flywheel overview in the Amazon Comprehend Developer Guide . |
| `CreateDocumentClassifier` | `-` | `idempotency-token` | `DocumentClassifierName`, `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode` | `ClientRequestToken` | `CreateDocumentClassifierResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `ResourceLimitExceededException`, `TooManyRequestsException`, `TooManyTagsException`, `UnsupportedLanguageException` | Creates a new document classifier that you can use to categorize documents. To create a classifier, you provide a set of training documents that are labeled with the categories that you want to use. For more informat ... |
| `CreateEndpoint` | `-` | `idempotency-token` | `EndpointName`, `DesiredInferenceUnits` | `ClientRequestToken` | `CreateEndpointResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException`, `TooManyTagsException` | Creates a model-specific endpoint for synchronous inference for a previously trained custom model For information about endpoints, see Managing endpoints . |
| `CreateEntityRecognizer` | `-` | `idempotency-token` | `RecognizerName`, `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode` | `ClientRequestToken` | `CreateEntityRecognizerResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `ResourceLimitExceededException`, `TooManyRequestsException`, `TooManyTagsException`, `UnsupportedLanguageException` | Creates an entity recognizer using submitted files. After your CreateEntityRecognizer request is submitted, you can check job status using the DescribeEntityRecognizer API. |
| `CreateFlywheel` | `-` | `idempotency-token` | `FlywheelName`, `DataAccessRoleArn`, `DataLakeS3Uri` | `ClientRequestToken` | `CreateFlywheelResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException`, `TooManyTagsException`, `UnsupportedLanguageException` | A flywheel is an Amazon Web Services resource that orchestrates the ongoing training of a model for custom classification or custom entity recognition. You can create a flywheel to start with an existing trained mode ... |
| `DeleteDocumentClassifier` | `-` | - | `DocumentClassifierArn` | - | `DeleteDocumentClassifierResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException` | Deletes a previously created document classifier Only those classifiers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a ResourceInUseException will ... |
| `DeleteEndpoint` | `-` | - | `EndpointArn` | - | `DeleteEndpointResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyRequestsException` | Deletes a model-specific endpoint for a previously-trained custom model. All endpoints must be deleted in order for the model to be deleted. For information about endpoints, see Managing endpoints . |
| `DeleteEntityRecognizer` | `-` | - | `EntityRecognizerArn` | - | `DeleteEntityRecognizerResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException` | Deletes an entity recognizer. Only those recognizers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a ResourceInUseException will be returned. This i ... |
| `DeleteFlywheel` | `-` | - | `FlywheelArn` | - | `DeleteFlywheelResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException` | Deletes a flywheel. When you delete the flywheel, Amazon Comprehend does not delete the data lake or the model associated with the flywheel. For more information about flywheels, see Flywheel overview in the Amazon C ... |
| `DeleteResourcePolicy` | `-` | - | `ResourceArn` | - | `DeleteResourcePolicyResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Deletes a resource-based policy that is attached to a custom model. |
| `DescribeDataset` | `-` | - | `DatasetArn` | - | `DescribeDatasetResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Returns information about the dataset that you specify. For more information about datasets, see Flywheel overview in the Amazon Comprehend Developer Guide . |
| `DescribeDocumentClassificationJob` | `-` | - | `JobId` | - | `DescribeDocumentClassificationJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a document classification job. Use this operation to get the status of a classification job. |
| `DescribeDocumentClassifier` | `-` | - | `DocumentClassifierArn` | - | `DescribeDocumentClassifierResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a document classifier. |
| `DescribeDominantLanguageDetectionJob` | `-` | - | `JobId` | - | `DescribeDominantLanguageDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a dominant language detection job. Use this operation to get the status of a detection job. |
| `DescribeEndpoint` | `-` | - | `EndpointArn` | - | `DescribeEndpointResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a specific endpoint. Use this operation to get the status of an endpoint. For information about endpoints, see Managing endpoints . |
| `DescribeEntitiesDetectionJob` | `-` | - | `JobId` | - | `DescribeEntitiesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with an entities detection job. Use this operation to get the status of a detection job. |
| `DescribeEntityRecognizer` | `-` | - | `EntityRecognizerArn` | - | `DescribeEntityRecognizerResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Provides details about an entity recognizer including status, S3 buckets containing training data, recognizer metadata, metrics, and so on. |
| `DescribeEventsDetectionJob` | `-` | - | `JobId` | - | `DescribeEventsDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the status and details of an events detection job. |
| `DescribeFlywheel` | `-` | - | `FlywheelArn` | - | `DescribeFlywheelResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Provides configuration information about the flywheel. For more information about flywheels, see Flywheel overview in the Amazon Comprehend Developer Guide . |
| `DescribeFlywheelIteration` | `-` | - | `FlywheelArn`, `FlywheelIterationId` | - | `DescribeFlywheelIterationResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Retrieve the configuration properties of a flywheel iteration. For more information about flywheels, see Flywheel overview in the Amazon Comprehend Developer Guide . |
| `DescribeKeyPhrasesDetectionJob` | `-` | - | `JobId` | - | `DescribeKeyPhrasesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a key phrases detection job. Use this operation to get the status of a detection job. |
| `DescribePiiEntitiesDetectionJob` | `-` | - | `JobId` | - | `DescribePiiEntitiesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a PII entities detection job. For example, you can use this operation to get the job status. |
| `DescribeResourcePolicy` | `-` | - | `ResourceArn` | - | `DescribeResourcePolicyResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Gets the details of a resource-based policy that is attached to a custom model, including the JSON body of the policy. |
| `DescribeSentimentDetectionJob` | `-` | - | `JobId` | - | `DescribeSentimentDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a sentiment detection job. Use this operation to get the status of a detection job. |
| `DescribeTargetedSentimentDetectionJob` | `-` | - | `JobId` | - | `DescribeTargetedSentimentDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a targeted sentiment detection job. Use this operation to get the status of the job. |
| `DescribeTopicsDetectionJob` | `-` | - | `JobId` | - | `DescribeTopicsDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a topic detection job. Use this operation to get the status of a detection job. |
| `DetectDominantLanguage` | `-` | - | `Text` | - | `DetectDominantLanguageResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException` | Determines the dominant language of the input text. For a list of languages that Amazon Comprehend can detect, see Amazon Comprehend Supported Languages . |
| `DetectEntities` | `-` | - | - | - | `DetectEntitiesResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceUnavailableException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Detects named entities in input text when you use the pre-trained model. Detects custom entities if you have a custom entity recognition model. When detecting named entities using the pre-trained model, use plain tex ... |
| `DetectKeyPhrases` | `-` | - | `Text`, `LanguageCode` | - | `DetectKeyPhrasesResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Detects the key noun phrases found in the text. |
| `DetectPiiEntities` | `-` | - | `Text`, `LanguageCode` | - | `DetectPiiEntitiesResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects the input text for entities that contain personally identifiable information (PII) and returns information about them. |
| `DetectSentiment` | `-` | - | `Text`, `LanguageCode` | - | `DetectSentimentResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects text and returns an inference of the prevailing sentiment ( POSITIVE , NEUTRAL , MIXED , or NEGATIVE ). |
| `DetectSyntax` | `-` | - | `Text`, `LanguageCode` | - | `DetectSyntaxResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects text for syntax and the part of speech of words in the document. For more information, see Syntax in the Comprehend Developer Guide. |
| `DetectTargetedSentiment` | `-` | - | `Text`, `LanguageCode` | - | `DetectTargetedSentimentResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Inspects the input text and returns a sentiment analysis for each entity identified in the text. For more information about targeted sentiment, see Targeted sentiment in the Amazon Comprehend Developer Guide . |
| `DetectToxicContent` | `-` | - | `TextSegments`, `LanguageCode` | - | `DetectToxicContentResponse` | `InternalServerException`, `InvalidRequestException`, `TextSizeLimitExceededException`, `UnsupportedLanguageException` | Performs toxicity analysis on the list of text strings that you provide as input. The API response contains a results list that matches the size of the input list. For more information about toxicity detection, see T ... |
| `ImportModel` | `-` | - | `SourceModelArn` | - | `ImportModelResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException`, `TooManyTagsException` | Creates a new custom model that replicates a source custom model that you import. The source model can be in your Amazon Web Services account or another one. If the source model is in another Amazon Web Services acco ... |
| `ListDatasets` | `-` | `paginated` | - | - | `ListDatasetsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | List the datasets that you have configured in this Region. For more information about datasets, see Flywheel overview in the Amazon Comprehend Developer Guide . |
| `ListDocumentClassificationJobs` | `-` | `paginated` | - | - | `ListDocumentClassificationJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the documentation classification jobs that you have submitted. |
| `ListDocumentClassifiers` | `-` | `paginated` | - | - | `ListDocumentClassifiersResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the document classifiers that you have created. |
| `ListDocumentClassifierSummaries` | `-` | `paginated` | - | - | `ListDocumentClassifierSummariesResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of summaries of the document classifiers that you have created |
| `ListDominantLanguageDetectionJobs` | `-` | `paginated` | - | - | `ListDominantLanguageDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the dominant language detection jobs that you have submitted. |
| `ListEndpoints` | `-` | `paginated` | - | - | `ListEndpointsResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of all existing endpoints that you've created. For information about endpoints, see Managing endpoints . |
| `ListEntitiesDetectionJobs` | `-` | `paginated` | - | - | `ListEntitiesDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the entity detection jobs that you have submitted. |
| `ListEntityRecognizers` | `-` | `paginated` | - | - | `ListEntityRecognizersResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the properties of all entity recognizers that you created, including recognizers currently in training. Allows you to filter the list of recognizers based on criteria such as status and submission time ... |
| `ListEntityRecognizerSummaries` | `-` | `paginated` | - | - | `ListEntityRecognizerSummariesResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of summaries for the entity recognizers that you have created. |
| `ListEventsDetectionJobs` | `-` | `paginated` | - | - | `ListEventsDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the events detection jobs that you have submitted. |
| `ListFlywheelIterationHistory` | `-` | `paginated` | `FlywheelArn` | - | `ListFlywheelIterationHistoryResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Information about the history of a flywheel iteration. For more information about flywheels, see Flywheel overview in the Amazon Comprehend Developer Guide . |
| `ListFlywheels` | `-` | `paginated` | - | - | `ListFlywheelsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the flywheels that you have created. |
| `ListKeyPhrasesDetectionJobs` | `-` | `paginated` | - | - | `ListKeyPhrasesDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Get a list of key phrase detection jobs that you have submitted. |
| `ListPiiEntitiesDetectionJobs` | `-` | `paginated` | - | - | `ListPiiEntitiesDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the PII entity detection jobs that you have submitted. |
| `ListSentimentDetectionJobs` | `-` | `paginated` | - | - | `ListSentimentDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of sentiment detection jobs that you have submitted. |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Lists all tags associated with a given Amazon Comprehend resource. |
| `ListTargetedSentimentDetectionJobs` | `-` | `paginated` | - | - | `ListTargetedSentimentDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of targeted sentiment detection jobs that you have submitted. |
| `ListTopicsDetectionJobs` | `-` | `paginated` | - | - | `ListTopicsDetectionJobsResponse` | `InternalServerException`, `InvalidFilterException`, `InvalidRequestException`, `TooManyRequestsException` | Gets a list of the topic detection jobs that you have submitted. |
| `PutResourcePolicy` | `-` | - | `ResourceArn`, `ResourcePolicy` | - | `PutResourcePolicyResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Attaches a resource-based policy to a custom model. You can use this policy to authorize an entity in another Amazon Web Services account to import the custom model, which replicates it in Amazon Comprehend in their ... |
| `StartDocumentClassificationJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn` | `ClientRequestToken` | `StartDocumentClassificationJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous document classification job using a custom classification model. Use the DescribeDocumentClassificationJob operation to track the progress of the job. |
| `StartDominantLanguageDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn` | `ClientRequestToken` | `StartDominantLanguageDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous dominant language detection job for a collection of documents. Use the operation to track the status of a job. |
| `StartEntitiesDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn`, `LanguageCode` | `ClientRequestToken` | `StartEntitiesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous entity detection job for a collection of documents. Use the operation to track the status of a job. This API can be used for either standard entity detection or custom entity recognition. In or ... |
| `StartEventsDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn`, `LanguageCode`, `TargetEventTypes` | `ClientRequestToken` | `StartEventsDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous event detection job for a collection of documents. |
| `StartFlywheelIteration` | `-` | - | `FlywheelArn` | - | `StartFlywheelIterationResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyRequestsException` | Start the flywheel iteration.This operation uses any new datasets to train a new model version. For more information about flywheels, see Flywheel overview in the Amazon Comprehend Developer Guide . |
| `StartKeyPhrasesDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn`, `LanguageCode` | `ClientRequestToken` | `StartKeyPhrasesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous key phrase detection job for a collection of documents. Use the operation to track the status of a job. |
| `StartPiiEntitiesDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `Mode`, `DataAccessRoleArn`, `LanguageCode` | `ClientRequestToken` | `StartPiiEntitiesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous PII entity detection job for a collection of documents. |
| `StartSentimentDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn`, `LanguageCode` | `ClientRequestToken` | `StartSentimentDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous sentiment detection job for a collection of documents. Use the operation to track the status of a job. |
| `StartTargetedSentimentDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn`, `LanguageCode` | `ClientRequestToken` | `StartTargetedSentimentDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous targeted sentiment detection job for a collection of documents. Use the DescribeTargetedSentimentDetectionJob operation to track the status of a job. |
| `StartTopicsDetectionJob` | `-` | `idempotency-token` | `InputDataConfig`, `OutputDataConfig`, `DataAccessRoleArn` | `ClientRequestToken` | `StartTopicsDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceInUseException`, `TooManyRequestsException`, `TooManyTagsException` | Starts an asynchronous topic detection job. Use the DescribeTopicDetectionJob operation to track the status of a job. |
| `StopDominantLanguageDetectionJob` | `-` | - | `JobId` | - | `StopDominantLanguageDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException` | Stops a dominant language detection job in progress. If the job state is IN_PROGRESS the job is marked for termination and put into the STOP_REQUESTED state. If the job completes before it can be stopped, it is put i ... |
| `StopEntitiesDetectionJob` | `-` | - | `JobId` | - | `StopEntitiesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException` | Stops an entities detection job in progress. If the job state is IN_PROGRESS the job is marked for termination and put into the STOP_REQUESTED state. If the job completes before it can be stopped, it is put into the ... |
| `StopEventsDetectionJob` | `-` | - | `JobId` | - | `StopEventsDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException` | Stops an events detection job in progress. |
| `StopKeyPhrasesDetectionJob` | `-` | - | `JobId` | - | `StopKeyPhrasesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException` | Stops a key phrases detection job in progress. If the job state is IN_PROGRESS the job is marked for termination and put into the STOP_REQUESTED state. If the job completes before it can be stopped, it is put into th ... |
| `StopPiiEntitiesDetectionJob` | `-` | - | `JobId` | - | `StopPiiEntitiesDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException` | Stops a PII entities detection job in progress. |
| `StopSentimentDetectionJob` | `-` | - | `JobId` | - | `StopSentimentDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException` | Stops a sentiment detection job in progress. If the job state is IN_PROGRESS , the job is marked for termination and put into the STOP_REQUESTED state. If the job completes before it can be stopped, it is put into th ... |
| `StopTargetedSentimentDetectionJob` | `-` | - | `JobId` | - | `StopTargetedSentimentDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `JobNotFoundException` | Stops a targeted sentiment detection job in progress. If the job state is IN_PROGRESS , the job is marked for termination and put into the STOP_REQUESTED state. If the job completes before it can be stopped, it is pu ... |
| `StopTrainingDocumentClassifier` | `-` | - | `DocumentClassifierArn` | - | `StopTrainingDocumentClassifierResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Stops a document classifier training job while in progress. If the training job state is TRAINING , the job is marked for termination and put into the STOP_REQUESTED state. If the training job completes before it can ... |
| `StopTrainingEntityRecognizer` | `-` | - | `EntityRecognizerArn` | - | `StopTrainingEntityRecognizerResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Stops an entity recognizer training job while in progress. If the training job state is TRAINING , the job is marked for termination and put into the STOP_REQUESTED state. If the training job completes before it can ... |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ConcurrentModificationException`, `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyTagsException` | Associates a specific tag with an Amazon Comprehend resource. A tag is a key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with "Sales" as the key might be added to a ... |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ConcurrentModificationException`, `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyTagKeysException` | Removes a specific tag associated with an Amazon Comprehend resource. |
| `UpdateEndpoint` | `-` | - | `EndpointArn` | - | `UpdateEndpointResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `TooManyRequestsException` | Updates information about the specified endpoint. For information about endpoints, see Managing endpoints . |
| `UpdateFlywheel` | `-` | - | `FlywheelArn` | - | `UpdateFlywheelResponse` | `InternalServerException`, `InvalidRequestException`, `KmsKeyValidationException`, `ResourceNotFoundException`, `TooManyRequestsException` | Update the configuration information for an existing flywheel. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BatchSizeLimitExceededException` | `structure` | Message | The number of documents in the request exceeds the limit of 25. Try your request again with fewer documents. |
| `ConcurrentModificationException` | `structure` | Message | Concurrent modification of the tags associated with an Amazon Comprehend resource is not supported. |
| `InternalServerException` | `structure` | Message | An internal server error occurred. Retry your request. |
| `InvalidFilterException` | `structure` | Message | The filter specified for the operation is invalid. Specify a different filter. |
| `InvalidRequestException` | `structure` | Message, Reason, Detail | The request is invalid. |
| `JobNotFoundException` | `structure` | Message | The specified job was not found. Check the job ID and try again. |
| `KmsKeyValidationException` | `structure` | Message | The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it. |
| `ResourceInUseException` | `structure` | Message | The specified resource name is already in use. Use a different name and try your request again. |
| `ResourceLimitExceededException` | `structure` | Message | The maximum number of resources per account has been exceeded. Review the resources, and then try your request again. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource ARN was not found. Check the ARN and try your request again. |
| `ResourceUnavailableException` | `structure` | Message | The specified resource is not available. Check the resource and try your request again. |
| `TextSizeLimitExceededException` | `structure` | Message | The size of the input text exceeds the limit. Use a smaller document. |
| `TooManyRequestsException` | `structure` | Message | The number of requests exceeds the limit. Resubmit your request later. |
| `TooManyTagKeysException` | `structure` | Message | The request contains more tag keys than can be associated with a resource (50 tag keys per resource). |
| `TooManyTagsException` | `structure` | Message | The request contains more tags than can be associated with a resource (50 tags per resource). The maximum number of tags includes both existing tags and tho ... |
| `UnsupportedLanguageException` | `structure` | Message | Amazon Comprehend can't process the language of the input text. For a list of supported languages, Supported languages in the Comprehend Developer Guide. |
| `BatchDetectDominantLanguageRequest` | `structure` | TextList | - |
| `BatchDetectDominantLanguageResponse` | `structure` | ResultList, ErrorList | - |
| `BatchDetectEntitiesRequest` | `structure` | TextList, LanguageCode | - |
| `BatchDetectEntitiesResponse` | `structure` | ResultList, ErrorList | - |
| `BatchDetectKeyPhrasesRequest` | `structure` | TextList, LanguageCode | - |
| `BatchDetectKeyPhrasesResponse` | `structure` | ResultList, ErrorList | - |
| `BatchDetectSentimentRequest` | `structure` | TextList, LanguageCode | - |
| `BatchDetectSentimentResponse` | `structure` | ResultList, ErrorList | - |
| `BatchDetectSyntaxRequest` | `structure` | TextList, LanguageCode | - |
| `BatchDetectSyntaxResponse` | `structure` | ResultList, ErrorList | - |
| `BatchDetectTargetedSentimentRequest` | `structure` | TextList, LanguageCode | - |
| `BatchDetectTargetedSentimentResponse` | `structure` | ResultList, ErrorList | - |
| `ClassifyDocumentRequest` | `structure` | Text, EndpointArn, Bytes, DocumentReaderConfig | - |
| `ClassifyDocumentResponse` | `structure` | Classes, Labels, DocumentMetadata, DocumentType, Errors, Warnings | - |
| `ContainsPiiEntitiesRequest` | `structure` | Text, LanguageCode | - |
| `ContainsPiiEntitiesResponse` | `structure` | Labels | - |
| `CreateDatasetRequest` | `structure` | FlywheelArn, DatasetName, DatasetType, Description, InputDataConfig, ClientRequestToken, Tags | - |
| `CreateDatasetResponse` | `structure` | DatasetArn | - |
| `CreateDocumentClassifierRequest` | `structure` | DocumentClassifierName, VersionName, DataAccessRoleArn, Tags, InputDataConfig, OutputDataConfig, ClientRequestToken, LanguageCode, VolumeKmsKeyId, VpcConfig, Mode, ModelKmsKeyId, ... (+1) | - |
| `CreateDocumentClassifierResponse` | `structure` | DocumentClassifierArn | - |
| `CreateEndpointRequest` | `structure` | EndpointName, ModelArn, DesiredInferenceUnits, ClientRequestToken, Tags, DataAccessRoleArn, FlywheelArn | - |
| `CreateEndpointResponse` | `structure` | EndpointArn, ModelArn | - |
| `CreateEntityRecognizerRequest` | `structure` | RecognizerName, VersionName, DataAccessRoleArn, Tags, InputDataConfig, ClientRequestToken, LanguageCode, VolumeKmsKeyId, VpcConfig, ModelKmsKeyId, ModelPolicy | - |
| `CreateEntityRecognizerResponse` | `structure` | EntityRecognizerArn | - |
| `AugmentedManifestsDocumentTypeFormat` | `enum` | PLAIN_TEXT_DOCUMENT, SEMI_STRUCTURED_DOCUMENT | - |
| `BlockType` | `enum` | LINE, WORD | - |
| `DatasetDataFormat` | `enum` | COMPREHEND_CSV, AUGMENTED_MANIFEST | - |
| `DatasetStatus` | `enum` | CREATING, COMPLETED, FAILED | - |
| `DatasetType` | `enum` | TRAIN, TEST | - |
| `DocumentClassifierDataFormat` | `enum` | COMPREHEND_CSV, AUGMENTED_MANIFEST | - |
| `DocumentClassifierDocumentTypeFormat` | `enum` | PLAIN_TEXT_DOCUMENT, SEMI_STRUCTURED_DOCUMENT | - |
| `DocumentClassifierMode` | `enum` | MULTI_CLASS, MULTI_LABEL | - |
| `DocumentReadAction` | `enum` | TEXTRACT_DETECT_DOCUMENT_TEXT, TEXTRACT_ANALYZE_DOCUMENT | - |
| `DocumentReadFeatureTypes` | `enum` | TABLES, FORMS | TABLES or FORMS |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
