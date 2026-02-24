# Amazon Fraud Detector

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Amazon Fraud Detector API Reference. This guide is for developers who need detailed information about Amazon Fraud Detector API actions, data types, and errors. For more information about Amazon Fraud Detector features, see the Amazon Fraud Detector User Guide. We provide the Query API as well as AWS software development kits (SDK) for Amazon Fraud Detector in Java and Python programming languages. The Amazon Fraud Detector Query API provides HTTPS requests that use the HTTP verb GET or POST and a Query parameter `Action`.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Fraud Detector resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Fraud Detector workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Delete`, `Update`, `Create`, `Put` operation families, including `GetBatchImportJobs`, `GetBatchPredictionJobs`, `GetDeleteEventsByEventTypeStatus`, `GetDetectorVersion`, `DeleteBatchImportJob`, `DeleteBatchPredictionJob`.

## Service Identity and Protocol

- AWS model slug: `frauddetector`
- AWS SDK for Rust slug: `frauddetector`
- Model version: `2019-11-15`
- Model file: `vendor/api-models-aws/models/frauddetector/service/2019-11-15/frauddetector-2019-11-15.json`
- SDK ID: `FraudDetector`
- Endpoint prefix: `frauddetector`
- ARN namespace: `frauddetector`
- CloudFormation name: `FraudDetector`
- CloudTrail event source: `frauddetector.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (20), `Delete` (16), `Update` (11), `Create` (8), `Put` (7), `Batch` (2), `Cancel` (2), `Describe` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCreateVariable`, `BatchGetVariable`, `CancelBatchImportJob`, `CancelBatchPredictionJob`, `CreateBatchImportJob`, `CreateBatchPredictionJob`, `CreateDetectorVersion`, `CreateList`, `CreateModel`, `CreateModelVersion`, `CreateRule`, `CreateVariable`, `DeleteBatchImportJob`, `DeleteBatchPredictionJob`, `DeleteDetector`, `DeleteDetectorVersion`, `DeleteEntityType`, `DeleteEvent`, `DeleteEventType`, `DeleteEventsByEventType`, ... (+28).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDetector`, `DescribeModelVersions`, `GetBatchImportJobs`, `GetBatchPredictionJobs`, `GetDeleteEventsByEventTypeStatus`, `GetDetectorVersion`, `GetDetectors`, `GetEntityTypes`, `GetEvent`, `GetEventPrediction`, `GetEventPredictionMetadata`, `GetEventTypes`, `GetExternalModels`, `GetKMSEncryptionKey`, `GetLabels`, `GetListElements`, `GetListsMetadata`, `GetModelVersion`, `GetModels`, `GetOutcomes`, ... (+4).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelBatchImportJob`, `CancelBatchPredictionJob`, `CreateBatchImportJob`, `CreateBatchPredictionJob`, `DeleteBatchImportJob`, `DeleteBatchPredictionJob`, `GetBatchImportJobs`, `GetBatchPredictionJobs`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 73 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetBatchImportJobs`, `GetBatchPredictionJobs`, `GetDeleteEventsByEventTypeStatus`, `GetDetectorVersion`, `GetDetectors`, `GetEntityTypes`, `GetEvent`, `GetEventPrediction`, `GetEventPredictionMetadata`, `GetEventTypes`, `GetExternalModels`, `GetKMSEncryptionKey`, `GetLabels`, `GetListElements`, `GetListsMetadata`, `GetModelVersion`, `GetModels`, `GetOutcomes`, `GetRules`, `GetVariables`
- Traits: `paginated` (13)
- Common required input members in this group: `detectorId`, `detectorVersionId`, `entities`, `eventId`, `eventTimestamp`, `eventTypeName`, `eventVariables`, `modelId`, `modelType`, `modelVersionNumber`, `name`, `predictionTimestamp`

### Delete

- Operations: `DeleteBatchImportJob`, `DeleteBatchPredictionJob`, `DeleteDetector`, `DeleteDetectorVersion`, `DeleteEntityType`, `DeleteEvent`, `DeleteEventType`, `DeleteEventsByEventType`, `DeleteExternalModel`, `DeleteLabel`, `DeleteList`, `DeleteModel`, `DeleteModelVersion`, `DeleteOutcome`, `DeleteRule`, `DeleteVariable`
- Common required input members in this group: `detectorId`, `detectorVersionId`, `eventId`, `eventTypeName`, `jobId`, `modelEndpoint`, `modelId`, `modelType`, `modelVersionNumber`, `name`, `rule`

### Update

- Operations: `UpdateDetectorVersion`, `UpdateDetectorVersionMetadata`, `UpdateDetectorVersionStatus`, `UpdateEventLabel`, `UpdateList`, `UpdateModel`, `UpdateModelVersion`, `UpdateModelVersionStatus`, `UpdateRuleMetadata`, `UpdateRuleVersion`, `UpdateVariable`
- Common required input members in this group: `assignedLabel`, `description`, `detectorId`, `detectorVersionId`, `eventId`, `eventTypeName`, `expression`, `externalModelEndpoints`, `labelTimestamp`, `language`, `majorVersionNumber`, `modelId`, `modelType`, `modelVersionNumber`, `name`, `outcomes`, `rule`, `rules`, `status`

### Create

- Operations: `CreateBatchImportJob`, `CreateBatchPredictionJob`, `CreateDetectorVersion`, `CreateList`, `CreateModel`, `CreateModelVersion`, `CreateRule`, `CreateVariable`
- Common required input members in this group: `dataSource`, `dataType`, `defaultValue`, `detectorId`, `detectorName`, `eventTypeName`, `expression`, `iamRoleArn`, `inputPath`, `jobId`, `language`, `modelId`, `modelType`, `name`, `outcomes`, `outputPath`, `ruleId`, `rules`, `trainingDataSchema`, `trainingDataSource`

### Put

- Operations: `PutDetector`, `PutEntityType`, `PutEventType`, `PutExternalModel`, `PutKMSEncryptionKey`, `PutLabel`, `PutOutcome`
- Common required input members in this group: `detectorId`, `entityTypes`, `eventTypeName`, `eventVariables`, `inputConfiguration`, `invokeModelEndpointRoleArn`, `kmsEncryptionKeyArn`, `modelEndpoint`, `modelEndpointStatus`, `modelSource`, `name`, `outputConfiguration`

### Batch

- Operations: `BatchCreateVariable`, `BatchGetVariable`
- Common required input members in this group: `names`, `variableEntries`

### Cancel

- Operations: `CancelBatchImportJob`, `CancelBatchPredictionJob`
- Common required input members in this group: `jobId`

### Describe

- Operations: `DescribeDetector`, `DescribeModelVersions`
- Traits: `paginated` (1)
- Common required input members in this group: `detectorId`

### List

- Operations: `ListEventPredictions`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: `resourceARN`

### Send

- Operations: `SendEvent`
- Common required input members in this group: `entities`, `eventId`, `eventTimestamp`, `eventTypeName`, `eventVariables`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceARN`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceARN`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCreateVariable` | - | - | `variableEntries` | - | `BatchCreateVariableResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a batch of variables. |
| `BatchGetVariable` | - | - | `names` | - | `BatchGetVariableResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets a batch of variables. |
| `CancelBatchImportJob` | - | - | `jobId` | - | `CancelBatchImportJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an in-progress batch import job. |
| `CancelBatchPredictionJob` | - | - | `jobId` | - | `CancelBatchPredictionJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels the specified batch prediction job. |
| `CreateBatchImportJob` | - | - | `eventTypeName`, `iamRoleArn`, `inputPath`, `jobId`, `outputPath` | - | `CreateBatchImportJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a batch import job. |
| `CreateBatchPredictionJob` | - | - | `detectorName`, `eventTypeName`, `iamRoleArn`, `inputPath`, `jobId`, `outputPath` | - | `CreateBatchPredictionJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a batch prediction job. |
| `CreateDetectorVersion` | - | - | `detectorId`, `rules` | - | `CreateDetectorVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a detector version. The detector version starts in a `DRAFT` status. |
| `CreateList` | - | - | `name` | - | `CreateListResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a list. List is a set of input data for a variable in your event dataset. |
| `CreateModel` | - | - | `eventTypeName`, `modelId`, `modelType` | - | `CreateModelResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a model using the specified model type. |
| `CreateModelVersion` | - | - | `modelId`, `modelType`, `trainingDataSchema`, `trainingDataSource` | - | `CreateModelVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a version of the model using the specified model type and model id. |
| `CreateRule` | - | - | `detectorId`, `expression`, `language`, `outcomes`, `ruleId` | - | `CreateRuleResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a rule for use with the specified detector. |
| `CreateVariable` | - | - | `dataSource`, `dataType`, `defaultValue`, `name` | - | `CreateVariableResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a variable. |
| `DeleteBatchImportJob` | - | - | `jobId` | - | `DeleteBatchImportJobResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the specified batch import job ID record. This action does not delete the data that was batch imported. |
| `DeleteBatchPredictionJob` | - | - | `jobId` | - | `DeleteBatchPredictionJobResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a batch prediction job. |
| `DeleteDetector` | - | - | `detectorId` | - | `DeleteDetectorResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the detector. Before deleting a detector, you must first delete all detector versions and rule versions associated with the detector. |
| `DeleteDetectorVersion` | - | - | `detectorId`, `detectorVersionId` | - | `DeleteDetectorVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the detector version. You cannot delete detector versions that are in `ACTIVE` status. |
| `DeleteEntityType` | - | - | `name` | - | `DeleteEntityTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an entity type. You cannot delete an entity type that is included in an event type. |
| `DeleteEvent` | - | - | `eventId`, `eventTypeName` | - | `DeleteEventResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the specified event. When you delete an event, Amazon Fraud Detector permanently deletes that event and the event data is no longer stored in Amazon Fraud Detector. |
| `DeleteEventType` | - | - | `name` | - | `DeleteEventTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an event type. You cannot delete an event type that is used in a detector or a model. |
| `DeleteEventsByEventType` | - | - | `eventTypeName` | - | `DeleteEventsByEventTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes all events of a particular event type. |
| `DeleteExternalModel` | - | - | `modelEndpoint` | - | `DeleteExternalModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Removes a SageMaker model from Amazon Fraud Detector. You can remove an Amazon SageMaker model if it is not associated with a detector version. |
| `DeleteLabel` | - | - | `name` | - | `DeleteLabelResult` | `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a label. You cannot delete labels that are included in an event type in Amazon Fraud Detector. |
| `DeleteList` | - | - | `name` | - | `DeleteListResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the list, provided it is not used in a rule. When you delete a list, Amazon Fraud Detector permanently deletes that list and the elements in the list. |
| `DeleteModel` | - | - | `modelId`, `modelType` | - | `DeleteModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a model. You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version. |
| `DeleteModelVersion` | - | - | `modelId`, `modelType`, `modelVersionNumber` | - | `DeleteModelVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a model version. You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version. |
| `DeleteOutcome` | - | - | `name` | - | `DeleteOutcomeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an outcome. You cannot delete an outcome that is used in a rule version. |
| `DeleteRule` | - | - | `rule` | - | `DeleteRuleResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the rule. You cannot delete a rule if it is used by an `ACTIVE` or `INACTIVE` detector version. |
| `DeleteVariable` | - | - | `name` | - | `DeleteVariableResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a variable. You can't delete variables that are included in an event type in Amazon Fraud Detector. |
| `DescribeDetector` | - | - | `detectorId` | - | `DescribeDetectorResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all versions for a specified detector. |
| `DescribeModelVersions` | - | `paginated` | - | - | `DescribeModelVersionsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all of the model versions for the specified model type or for the specified model type and model ID. You can also get details for a single, specified model version. |
| `GetBatchImportJobs` | - | `paginated` | - | - | `GetBatchImportJobsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all batch import jobs or a specific job of the specified ID. This is a paginated API. |
| `GetBatchPredictionJobs` | - | `paginated` | - | - | `GetBatchPredictionJobsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all batch prediction jobs or a specific job if you specify a job ID. This is a paginated API. |
| `GetDeleteEventsByEventTypeStatus` | - | - | `eventTypeName` | - | `GetDeleteEventsByEventTypeStatusResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the status of a `DeleteEventsByEventType` action. |
| `GetDetectorVersion` | - | - | `detectorId`, `detectorVersionId` | - | `GetDetectorVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a particular detector version. |
| `GetDetectors` | - | `paginated` | - | - | `GetDetectorsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all detectors or a single detector if a `detectorId` is specified. This is a paginated API. |
| `GetEntityTypes` | - | `paginated` | - | - | `GetEntityTypesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all entity types or a specific entity type if a name is specified. This is a paginated API. |
| `GetEvent` | - | - | `eventId`, `eventTypeName` | - | `GetEventResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details of events stored with Amazon Fraud Detector. This action does not retrieve prediction results. |
| `GetEventPrediction` | - | - | `detectorId`, `entities`, `eventId`, `eventTimestamp`, `eventTypeName`, `eventVariables` | - | `GetEventPredictionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Evaluates an event against a detector version. If a version ID is not provided, the detector’s (`ACTIVE`) version is used. |
| `GetEventPredictionMetadata` | - | - | `detectorId`, `detectorVersionId`, `eventId`, `eventTypeName`, `predictionTimestamp` | - | `GetEventPredictionMetadataResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets details of the past fraud predictions for the specified event ID, event type, detector ID, and detector version ID that was generated in the specified time period. |
| `GetEventTypes` | - | `paginated` | - | - | `GetEventTypesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all event types or a specific event type if name is provided. This is a paginated API. |
| `GetExternalModels` | - | `paginated` | - | - | `GetExternalModelsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details for one or more Amazon SageMaker models that have been imported into the service. This is a paginated API. |
| `GetKMSEncryptionKey` | - | - | - | - | `GetKMSEncryptionKeyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the encryption key if a KMS key has been specified to be used to encrypt content in Amazon Fraud Detector. |
| `GetLabels` | - | `paginated` | - | - | `GetLabelsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all labels or a specific label if name is provided. This is a paginated API. |
| `GetListElements` | - | `paginated` | `name` | - | `GetListElementsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all the elements in the specified list. |
| `GetListsMetadata` | - | `paginated` | - | - | `GetListsMetadataResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the metadata of either all the lists under the account or the specified list. |
| `GetModelVersion` | - | - | `modelId`, `modelType`, `modelVersionNumber` | - | `GetModelVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of the specified model version. |
| `GetModels` | - | `paginated` | - | - | `GetModelsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets one or more models. Gets all models for the Amazon Web Services account if no model type and no model id provided. |
| `GetOutcomes` | - | `paginated` | - | - | `GetOutcomesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets one or more outcomes. This is a paginated API. |
| `GetRules` | - | `paginated` | `detectorId` | - | `GetRulesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get all rules for a detector (paginated) if `ruleId` and `ruleVersion` are not specified. Gets all rules for the detector and the `ruleId` if present (paginated). |
| `GetVariables` | - | `paginated` | - | - | `GetVariablesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all of the variables or the specific variable. This is a paginated API. |
| `ListEventPredictions` | - | `paginated` | - | - | `ListEventPredictionsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets a list of past predictions. The list can be filtered by detector ID, detector version ID, event ID, event type, or by specifying a time period. |
| `ListTagsForResource` | - | `paginated` | `resourceARN` | - | `ListTagsForResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags associated with the resource. This is a paginated API. |
| `PutDetector` | - | - | `detectorId`, `eventTypeName` | - | `PutDetectorResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates a detector. |
| `PutEntityType` | - | - | `name` | - | `PutEntityTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an entity type. An entity represents who is performing the event. |
| `PutEventType` | - | - | `entityTypes`, `eventVariables`, `name` | - | `PutEventTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an event type. An event is a business activity that is evaluated for fraud risk. |
| `PutExternalModel` | - | - | `inputConfiguration`, `invokeModelEndpointRoleArn`, `modelEndpoint`, `modelEndpointStatus`, `modelSource`, `outputConfiguration` | - | `PutExternalModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an Amazon SageMaker model endpoint. You can also use this action to update the configuration of the model endpoint, including the IAM role and/or the mapped variables. |
| `PutKMSEncryptionKey` | - | - | `kmsEncryptionKeyArn` | - | `PutKMSEncryptionKeyResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Specifies the KMS key to be used to encrypt content in Amazon Fraud Detector. |
| `PutLabel` | - | - | `name` | - | `PutLabelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates label. A label classifies an event as fraudulent or legitimate. |
| `PutOutcome` | - | - | `name` | - | `PutOutcomeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an outcome. |
| `SendEvent` | - | - | `entities`, `eventId`, `eventTimestamp`, `eventTypeName`, `eventVariables` | - | `SendEventResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stores events in Amazon Fraud Detector without generating fraud predictions for those events. For example, you can use `SendEvent` to upload a historical dataset, which you can then later use to train a model. |
| `TagResource` | - | - | `resourceARN`, `tags` | - | `TagResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns tags to a resource. |
| `UntagResource` | - | - | `resourceARN`, `tagKeys` | - | `UntagResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a resource. |
| `UpdateDetectorVersion` | - | - | `detectorId`, `detectorVersionId`, `externalModelEndpoints`, `rules` | - | `UpdateDetectorVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a detector version. The detector version attributes that you can update include models, external model endpoints, rules, rule execution mode, and description. |
| `UpdateDetectorVersionMetadata` | - | - | `description`, `detectorId`, `detectorVersionId` | - | `UpdateDetectorVersionMetadataResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the detector version's description. You can update the metadata for any detector version (`DRAFT, ACTIVE,` or `INACTIVE`). |
| `UpdateDetectorVersionStatus` | - | - | `detectorId`, `detectorVersionId`, `status` | - | `UpdateDetectorVersionStatusResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the detector version’s status. You can perform the following promotions or demotions using `UpdateDetectorVersionStatus`: `DRAFT` to `ACTIVE`, `ACTIVE` to `INACTIVE`, and `INACTIVE` to `ACTIVE`. |
| `UpdateEventLabel` | - | - | `assignedLabel`, `eventId`, `eventTypeName`, `labelTimestamp` | - | `UpdateEventLabelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified event with a new label. |
| `UpdateList` | - | - | `name` | - | `UpdateListResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a list. |
| `UpdateModel` | - | - | `modelId`, `modelType` | - | `UpdateModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates model description. |
| `UpdateModelVersion` | - | - | `majorVersionNumber`, `modelId`, `modelType` | - | `UpdateModelVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a model version. Updating a model version retrains an existing model version using updated training data and produces a new minor version of the model. |
| `UpdateModelVersionStatus` | - | - | `modelId`, `modelType`, `modelVersionNumber`, `status` | - | `UpdateModelVersionStatusResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the status of a model version. You can perform the following status updates: Change the `TRAINING_IN_PROGRESS` status to `TRAINING_CANCELLED`. |
| `UpdateRuleMetadata` | - | - | `description`, `rule` | - | `UpdateRuleMetadataResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a rule's metadata. The description attribute can be updated. |
| `UpdateRuleVersion` | - | - | `expression`, `language`, `outcomes`, `rule` | - | `UpdateRuleVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a rule version resulting in a new rule version. Updates a rule version resulting in a new rule version (version 1, 2, 3 ...). |
| `UpdateVariable` | - | - | `name` | - | `UpdateVariableResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a variable. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ThrottlingException` | `structure` | `message` | An exception indicating a throttling error. |
| `AccessDeniedException` | `structure` | `message` | An exception indicating Amazon Fraud Detector does not have the needed permissions. |
| `ValidationException` | `structure` | `message` | An exception indicating a specified value is not allowed. |
| `InternalServerException` | `structure` | `message` | An exception indicating an internal server error. |
| `ResourceNotFoundException` | `structure` | `message` | An exception indicating the specified resource was not found. |
| `ConflictException` | `structure` | `message` | An exception indicating there was a conflict during a delete operation. |
| `BatchCreateVariableRequest` | `structure` | `tags`, `variableEntries` | - |
| `BatchCreateVariableResult` | `structure` | `errors` | - |
| `BatchGetVariableRequest` | `structure` | `names` | - |
| `BatchGetVariableResult` | `structure` | `errors`, `variables` | - |
| `CancelBatchImportJobRequest` | `structure` | `jobId` | - |
| `CancelBatchImportJobResult` | `structure` | - | - |
| `CancelBatchPredictionJobRequest` | `structure` | `jobId` | - |
| `CancelBatchPredictionJobResult` | `structure` | - | - |
| `CreateBatchImportJobRequest` | `structure` | `eventTypeName`, `iamRoleArn`, `inputPath`, `jobId`, `outputPath`, `tags` | - |
| `CreateBatchImportJobResult` | `structure` | - | - |
| `CreateBatchPredictionJobRequest` | `structure` | `detectorName`, `detectorVersion`, `eventTypeName`, `iamRoleArn`, `inputPath`, `jobId`, `outputPath`, `tags` | - |
| `CreateBatchPredictionJobResult` | `structure` | - | - |
| `CreateDetectorVersionRequest` | `structure` | `description`, `detectorId`, `externalModelEndpoints`, `modelVersions`, `ruleExecutionMode`, `rules`, `tags` | - |
| `CreateDetectorVersionResult` | `structure` | `detectorId`, `detectorVersionId`, `status` | - |
| `CreateListRequest` | `structure` | `description`, `elements`, `name`, `tags`, `variableType` | - |
| `CreateListResult` | `structure` | - | - |
| `CreateModelRequest` | `structure` | `description`, `eventTypeName`, `modelId`, `modelType`, `tags` | - |
| `CreateModelResult` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
