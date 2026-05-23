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

- Operations: `GetBatchImportJobs`, `GetBatchPredictionJobs`, `GetDeleteEventsByEventTypeStatus`, `GetDetectors`, `GetDetectorVersion`, `GetEntityTypes`, `GetEvent`, `GetEventPrediction`, `GetEventPredictionMetadata`, `GetEventTypes`, `GetExternalModels`, `GetKMSEncryptionKey`, `GetLabels`, `GetListElements`, `GetListsMetadata`, `GetModels`, `GetModelVersion`, `GetOutcomes`, `GetRules`, `GetVariables`
- Traits: `paginated` (13)
- Common required input members in this group: `eventTypeName`, `detectorId`, `detectorVersionId`, `eventId`

### Delete

- Operations: `DeleteBatchImportJob`, `DeleteBatchPredictionJob`, `DeleteDetector`, `DeleteDetectorVersion`, `DeleteEntityType`, `DeleteEvent`, `DeleteEventsByEventType`, `DeleteEventType`, `DeleteExternalModel`, `DeleteLabel`, `DeleteList`, `DeleteModel`, `DeleteModelVersion`, `DeleteOutcome`, `DeleteRule`, `DeleteVariable`
- Common required input members in this group: `jobId`, `detectorId`, `name`, `eventTypeName`, `modelId`, `modelType`

### Update

- Operations: `UpdateDetectorVersion`, `UpdateDetectorVersionMetadata`, `UpdateDetectorVersionStatus`, `UpdateEventLabel`, `UpdateList`, `UpdateModel`, `UpdateModelVersion`, `UpdateModelVersionStatus`, `UpdateRuleMetadata`, `UpdateRuleVersion`, `UpdateVariable`
- Common required input members in this group: `detectorId`, `detectorVersionId`, `description`, `status`, `name`, `modelId`, `modelType`, `rule`

### Create

- Operations: `CreateBatchImportJob`, `CreateBatchPredictionJob`, `CreateDetectorVersion`, `CreateList`, `CreateModel`, `CreateModelVersion`, `CreateRule`, `CreateVariable`
- Common required input members in this group: `jobId`, `inputPath`, `outputPath`, `eventTypeName`, `iamRoleArn`, `detectorId`, `name`, `modelId`, `modelType`

### Put

- Operations: `PutDetector`, `PutEntityType`, `PutEventType`, `PutExternalModel`, `PutKMSEncryptionKey`, `PutLabel`, `PutOutcome`
- Common required input members in this group: `name`

### Batch

- Operations: `BatchCreateVariable`, `BatchGetVariable`
- Common required input members in this group: -

### Cancel

- Operations: `CancelBatchImportJob`, `CancelBatchPredictionJob`
- Common required input members in this group: `jobId`

### Describe

- Operations: `DescribeDetector`, `DescribeModelVersions`
- Traits: `paginated` (1)
- Common required input members in this group: -

### List

- Operations: `ListEventPredictions`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Send

- Operations: `SendEvent`
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
| `BatchCreateVariable` | `-` | - | `variableEntries` | - | `BatchCreateVariableResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a batch of variables. |
| `BatchGetVariable` | `-` | - | `names` | - | `BatchGetVariableResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets a batch of variables. |
| `CancelBatchImportJob` | `-` | - | `jobId` | - | `CancelBatchImportJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an in-progress batch import job. |
| `CancelBatchPredictionJob` | `-` | - | `jobId` | - | `CancelBatchPredictionJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels the specified batch prediction job. |
| `CreateBatchImportJob` | `-` | - | `jobId`, `inputPath`, `outputPath`, `eventTypeName`, `iamRoleArn` | - | `CreateBatchImportJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a batch import job. |
| `CreateBatchPredictionJob` | `-` | - | `jobId`, `inputPath`, `outputPath`, `eventTypeName`, `detectorName`, `iamRoleArn` | - | `CreateBatchPredictionJobResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a batch prediction job. |
| `CreateDetectorVersion` | `-` | - | `detectorId`, `rules` | - | `CreateDetectorVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a detector version. The detector version starts in a DRAFT status. |
| `CreateList` | `-` | - | `name` | - | `CreateListResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a list. List is a set of input data for a variable in your event dataset. You use the input data in a rule that's associated with your detector. For more information, see Lists . |
| `CreateModel` | `-` | - | `modelId`, `modelType`, `eventTypeName` | - | `CreateModelResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a model using the specified model type. |
| `CreateModelVersion` | `-` | - | `modelId`, `modelType`, `trainingDataSource`, `trainingDataSchema` | - | `CreateModelVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a version of the model using the specified model type and model id. |
| `CreateRule` | `-` | - | `ruleId`, `detectorId`, `expression`, `language`, `outcomes` | - | `CreateRuleResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a rule for use with the specified detector. |
| `CreateVariable` | `-` | - | `name`, `dataType`, `dataSource`, `defaultValue` | - | `CreateVariableResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a variable. |
| `DeleteBatchImportJob` | `-` | - | `jobId` | - | `DeleteBatchImportJobResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the specified batch import job ID record. This action does not delete the data that was batch imported. |
| `DeleteBatchPredictionJob` | `-` | - | `jobId` | - | `DeleteBatchPredictionJobResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a batch prediction job. |
| `DeleteDetector` | `-` | - | `detectorId` | - | `DeleteDetectorResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the detector. Before deleting a detector, you must first delete all detector versions and rule versions associated with the detector. When you delete a detector, Amazon Fraud Detector permanently deletes the ... |
| `DeleteDetectorVersion` | `-` | - | `detectorId`, `detectorVersionId` | - | `DeleteDetectorVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the detector version. You cannot delete detector versions that are in ACTIVE status. When you delete a detector version, Amazon Fraud Detector permanently deletes the detector and the data is no longer stored ... |
| `DeleteEntityType` | `-` | - | `name` | - | `DeleteEntityTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an entity type. You cannot delete an entity type that is included in an event type. When you delete an entity type, Amazon Fraud Detector permanently deletes that entity type and the data is no longer stored ... |
| `DeleteEvent` | `-` | - | `eventId`, `eventTypeName` | - | `DeleteEventResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the specified event. When you delete an event, Amazon Fraud Detector permanently deletes that event and the event data is no longer stored in Amazon Fraud Detector. If deleteAuditHistory is True , event data ... |
| `DeleteEventsByEventType` | `-` | - | `eventTypeName` | - | `DeleteEventsByEventTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes all events of a particular event type. |
| `DeleteEventType` | `-` | - | `name` | - | `DeleteEventTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an event type. You cannot delete an event type that is used in a detector or a model. When you delete an event type, Amazon Fraud Detector permanently deletes that event type and the data is no longer stored ... |
| `DeleteExternalModel` | `-` | - | `modelEndpoint` | - | `DeleteExternalModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Removes a SageMaker model from Amazon Fraud Detector. You can remove an Amazon SageMaker model if it is not associated with a detector version. Removing a SageMaker model disconnects it from Amazon Fraud Detector, bu ... |
| `DeleteLabel` | `-` | - | `name` | - | `DeleteLabelResult` | `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a label. You cannot delete labels that are included in an event type in Amazon Fraud Detector. You cannot delete a label assigned to an event ID. You must first delete the relevant event ID. When you delete a ... |
| `DeleteList` | `-` | - | `name` | - | `DeleteListResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the list, provided it is not used in a rule. When you delete a list, Amazon Fraud Detector permanently deletes that list and the elements in the list. |
| `DeleteModel` | `-` | - | `modelId`, `modelType` | - | `DeleteModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a model. You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version. When you delete a model, Amazon Fraud Detector permanently deletes th ... |
| `DeleteModelVersion` | `-` | - | `modelId`, `modelType`, `modelVersionNumber` | - | `DeleteModelVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a model version. You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version. When you delete a model version, Amazon Fraud Detector perman ... |
| `DeleteOutcome` | `-` | - | `name` | - | `DeleteOutcomeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an outcome. You cannot delete an outcome that is used in a rule version. When you delete an outcome, Amazon Fraud Detector permanently deletes that outcome and the data is no longer stored in Amazon Fraud Det ... |
| `DeleteRule` | `-` | - | `rule` | - | `DeleteRuleResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the rule. You cannot delete a rule if it is used by an ACTIVE or INACTIVE detector version. When you delete a rule, Amazon Fraud Detector permanently deletes that rule and the data is no longer stored in Amaz ... |
| `DeleteVariable` | `-` | - | `name` | - | `DeleteVariableResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a variable. You can't delete variables that are included in an event type in Amazon Fraud Detector. Amazon Fraud Detector automatically deletes model output variables and SageMaker model output variables when ... |
| `DescribeDetector` | `-` | - | `detectorId` | - | `DescribeDetectorResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all versions for a specified detector. |
| `DescribeModelVersions` | `-` | `paginated` | - | - | `DescribeModelVersionsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all of the model versions for the specified model type or for the specified model type and model ID. You can also get details for a single, specified model version. |
| `GetBatchImportJobs` | `-` | `paginated` | - | - | `GetBatchImportJobsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all batch import jobs or a specific job of the specified ID. This is a paginated API. If you provide a null maxResults , this action retrieves a maximum of 50 records per page. If you provide a maxResults , the ... |
| `GetBatchPredictionJobs` | `-` | `paginated` | - | - | `GetBatchPredictionJobsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all batch prediction jobs or a specific job if you specify a job ID. This is a paginated API. If you provide a null maxResults, this action retrieves a maximum of 50 records per page. If you provide a maxResults ... |
| `GetDeleteEventsByEventTypeStatus` | `-` | - | `eventTypeName` | - | `GetDeleteEventsByEventTypeStatusResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the status of a DeleteEventsByEventType action. |
| `GetDetectors` | `-` | `paginated` | - | - | `GetDetectorsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all detectors or a single detector if a detectorId is specified. This is a paginated API. If you provide a null maxResults , this action retrieves a maximum of 10 records per page. If you provide a maxResults , ... |
| `GetDetectorVersion` | `-` | - | `detectorId`, `detectorVersionId` | - | `GetDetectorVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a particular detector version. |
| `GetEntityTypes` | `-` | `paginated` | - | - | `GetEntityTypesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all entity types or a specific entity type if a name is specified. This is a paginated API. If you provide a null maxResults , this action retrieves a maximum of 10 records per page. If you provide a maxResults ... |
| `GetEvent` | `-` | - | `eventId`, `eventTypeName` | - | `GetEventResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details of events stored with Amazon Fraud Detector. This action does not retrieve prediction results. |
| `GetEventPrediction` | `-` | - | `detectorId`, `eventId`, `eventTypeName`, `entities`, `eventTimestamp`, `eventVariables` | - | `GetEventPredictionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Evaluates an event against a detector version. If a version ID is not provided, the detector’s ( ACTIVE ) version is used. |
| `GetEventPredictionMetadata` | `-` | - | `eventId`, `eventTypeName`, `detectorId`, `detectorVersionId`, `predictionTimestamp` | - | `GetEventPredictionMetadataResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets details of the past fraud predictions for the specified event ID, event type, detector ID, and detector version ID that was generated in the specified time period. |
| `GetEventTypes` | `-` | `paginated` | - | - | `GetEventTypesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all event types or a specific event type if name is provided. This is a paginated API. If you provide a null maxResults , this action retrieves a maximum of 10 records per page. If you provide a maxResults , the ... |
| `GetExternalModels` | `-` | `paginated` | - | - | `GetExternalModelsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details for one or more Amazon SageMaker models that have been imported into the service. This is a paginated API. If you provide a null maxResults , this actions retrieves a maximum of 10 records per page. ... |
| `GetKMSEncryptionKey` | `-` | - | - | - | `GetKMSEncryptionKeyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the encryption key if a KMS key has been specified to be used to encrypt content in Amazon Fraud Detector. |
| `GetLabels` | `-` | `paginated` | - | - | `GetLabelsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all labels or a specific label if name is provided. This is a paginated API. If you provide a null maxResults , this action retrieves a maximum of 50 records per page. If you provide a maxResults , the value mus ... |
| `GetListElements` | `-` | `paginated` | `name` | - | `GetListElementsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all the elements in the specified list. |
| `GetListsMetadata` | `-` | `paginated` | - | - | `GetListsMetadataResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the metadata of either all the lists under the account or the specified list. |
| `GetModels` | `-` | `paginated` | - | - | `GetModelsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets one or more models. Gets all models for the Amazon Web Services account if no model type and no model id provided. Gets all models for the Amazon Web Services account and model type, if the model type is specifi ... |
| `GetModelVersion` | `-` | - | `modelId`, `modelType`, `modelVersionNumber` | - | `GetModelVersionResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of the specified model version. |
| `GetOutcomes` | `-` | `paginated` | - | - | `GetOutcomesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets one or more outcomes. This is a paginated API. If you provide a null maxResults , this actions retrieves a maximum of 100 records per page. If you provide a maxResults , the value must be between 50 and 100. To ... |
| `GetRules` | `-` | `paginated` | `detectorId` | - | `GetRulesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get all rules for a detector (paginated) if ruleId and ruleVersion are not specified. Gets all rules for the detector and the ruleId if present (paginated). Gets a specific rule if both the ruleId and the ruleVersion ... |
| `GetVariables` | `-` | `paginated` | - | - | `GetVariablesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets all of the variables or the specific variable. This is a paginated API. Providing null maxSizePerPage results in retrieving maximum of 100 records per page. If you provide maxSizePerPage the value must be betwee ... |
| `ListEventPredictions` | `-` | `paginated` | - | - | `ListEventPredictionsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets a list of past predictions. The list can be filtered by detector ID, detector version ID, event ID, event type, or by specifying a time period. If filter is not specified, the most recent prediction is returned. ... |
| `ListTagsForResource` | `-` | `paginated` | `resourceARN` | - | `ListTagsForResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags associated with the resource. This is a paginated API. To get the next page results, provide the pagination token from the response as part of your request. A null pagination token fetches the records ... |
| `PutDetector` | `-` | - | `detectorId`, `eventTypeName` | - | `PutDetectorResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates a detector. |
| `PutEntityType` | `-` | - | `name` | - | `PutEntityTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an entity type. An entity represents who is performing the event. As part of a fraud prediction, you pass the entity ID to indicate the specific entity who performed the event. An entity type class ... |
| `PutEventType` | `-` | - | `name`, `eventVariables`, `entityTypes` | - | `PutEventTypeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an event type. An event is a business activity that is evaluated for fraud risk. With Amazon Fraud Detector, you generate fraud predictions for events. An event type defines the structure for an ev ... |
| `PutExternalModel` | `-` | - | `modelEndpoint`, `modelSource`, `invokeModelEndpointRoleArn`, `inputConfiguration`, `outputConfiguration`, `modelEndpointStatus` | - | `PutExternalModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an Amazon SageMaker model endpoint. You can also use this action to update the configuration of the model endpoint, including the IAM role and/or the mapped variables. |
| `PutKMSEncryptionKey` | `-` | - | `kmsEncryptionKeyArn` | - | `PutKMSEncryptionKeyResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Specifies the KMS key to be used to encrypt content in Amazon Fraud Detector. |
| `PutLabel` | `-` | - | `name` | - | `PutLabelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates label. A label classifies an event as fraudulent or legitimate. Labels are associated with event types and used to train supervised machine learning models in Amazon Fraud Detector. |
| `PutOutcome` | `-` | - | `name` | - | `PutOutcomeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates or updates an outcome. |
| `SendEvent` | `-` | - | `eventId`, `eventTypeName`, `eventTimestamp`, `eventVariables`, `entities` | - | `SendEventResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stores events in Amazon Fraud Detector without generating fraud predictions for those events. For example, you can use SendEvent to upload a historical dataset, which you can then later use to train a model. |
| `TagResource` | `-` | - | `resourceARN`, `tags` | - | `TagResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns tags to a resource. |
| `UntagResource` | `-` | - | `resourceARN`, `tagKeys` | - | `UntagResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a resource. |
| `UpdateDetectorVersion` | `-` | - | `detectorId`, `detectorVersionId`, `externalModelEndpoints`, `rules` | - | `UpdateDetectorVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a detector version. The detector version attributes that you can update include models, external model endpoints, rules, rule execution mode, and description. You can only update a DRAFT detector version. |
| `UpdateDetectorVersionMetadata` | `-` | - | `detectorId`, `detectorVersionId`, `description` | - | `UpdateDetectorVersionMetadataResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the detector version's description. You can update the metadata for any detector version ( DRAFT, ACTIVE, or INACTIVE ). |
| `UpdateDetectorVersionStatus` | `-` | - | `detectorId`, `detectorVersionId`, `status` | - | `UpdateDetectorVersionStatusResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the detector version’s status. You can perform the following promotions or demotions using UpdateDetectorVersionStatus : DRAFT to ACTIVE , ACTIVE to INACTIVE , and INACTIVE to ACTIVE . |
| `UpdateEventLabel` | `-` | - | `eventId`, `eventTypeName`, `assignedLabel`, `labelTimestamp` | - | `UpdateEventLabelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified event with a new label. |
| `UpdateList` | `-` | - | `name` | - | `UpdateListResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a list. |
| `UpdateModel` | `-` | - | `modelId`, `modelType` | - | `UpdateModelResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates model description. |
| `UpdateModelVersion` | `-` | - | `modelId`, `modelType`, `majorVersionNumber` | - | `UpdateModelVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a model version. Updating a model version retrains an existing model version using updated training data and produces a new minor version of the model. You can update the training data set location and data a ... |
| `UpdateModelVersionStatus` | `-` | - | `modelId`, `modelType`, `modelVersionNumber`, `status` | - | `UpdateModelVersionStatusResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the status of a model version. You can perform the following status updates: Change the TRAINING_IN_PROGRESS status to TRAINING_CANCELLED . Change the TRAINING_COMPLETE status to ACTIVE . Change ACTIVE to INA ... |
| `UpdateRuleMetadata` | `-` | - | `rule`, `description` | - | `UpdateRuleMetadataResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a rule's metadata. The description attribute can be updated. |
| `UpdateRuleVersion` | `-` | - | `rule`, `expression`, `language`, `outcomes` | - | `UpdateRuleVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a rule version resulting in a new rule version. Updates a rule version resulting in a new rule version (version 1, 2, 3 ...). |
| `UpdateVariable` | `-` | - | `name` | - | `UpdateVariableResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a variable. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as PutExternalModel , that ... |
| `ConflictException` | `structure` | message | An exception indicating there was a conflict during a delete operation. |
| `InternalServerException` | `structure` | message | An exception indicating an internal server error. |
| `ResourceNotFoundException` | `structure` | message | An exception indicating the specified resource was not found. |
| `ResourceUnavailableException` | `structure` | message | An exception indicating that the attached customer-owned (external) model threw an exception when Amazon Fraud Detector invoked the model. |
| `ThrottlingException` | `structure` | message | An exception indicating a throttling error. |
| `ValidationException` | `structure` | message | An exception indicating a specified value is not allowed. |
| `BatchCreateVariableRequest` | `structure` | variableEntries, tags | - |
| `BatchCreateVariableResult` | `structure` | errors | - |
| `BatchGetVariableRequest` | `structure` | names | - |
| `BatchGetVariableResult` | `structure` | variables, errors | - |
| `CancelBatchImportJobRequest` | `structure` | jobId | - |
| `CancelBatchImportJobResult` | `structure` | **empty (no members)** | - |
| `CancelBatchPredictionJobRequest` | `structure` | jobId | - |
| `CancelBatchPredictionJobResult` | `structure` | **empty (no members)** | - |
| `CreateBatchImportJobRequest` | `structure` | jobId, inputPath, outputPath, eventTypeName, iamRoleArn, tags | - |
| `CreateBatchImportJobResult` | `structure` | **empty (no members)** | - |
| `CreateBatchPredictionJobRequest` | `structure` | jobId, inputPath, outputPath, eventTypeName, detectorName, detectorVersion, iamRoleArn, tags | - |
| `CreateBatchPredictionJobResult` | `structure` | **empty (no members)** | - |
| `CreateDetectorVersionRequest` | `structure` | detectorId, description, externalModelEndpoints, rules, modelVersions, ruleExecutionMode, tags | - |
| `CreateDetectorVersionResult` | `structure` | detectorId, detectorVersionId, status | - |
| `CreateListRequest` | `structure` | name, elements, variableType, description, tags | - |
| `CreateListResult` | `structure` | **empty (no members)** | - |
| `CreateModelRequest` | `structure` | modelId, modelType, description, eventTypeName, tags | - |
| `CreateModelResult` | `structure` | **empty (no members)** | - |
| `CreateModelVersionRequest` | `structure` | modelId, modelType, trainingDataSource, trainingDataSchema, externalEventsDetail, ingestedEventsDetail, tags | - |
| `CreateModelVersionResult` | `structure` | modelId, modelType, modelVersionNumber, status | - |
| `CreateRuleRequest` | `structure` | ruleId, detectorId, description, expression, language, outcomes, tags | - |
| `CreateRuleResult` | `structure` | rule | - |
| `CreateVariableRequest` | `structure` | name, dataType, dataSource, defaultValue, description, variableType, tags | - |
| `CreateVariableResult` | `structure` | **empty (no members)** | - |
| `DeleteBatchImportJobRequest` | `structure` | jobId | - |
| `DeleteBatchImportJobResult` | `structure` | **empty (no members)** | - |
| `DeleteBatchPredictionJobRequest` | `structure` | jobId | - |
| `DeleteBatchPredictionJobResult` | `structure` | **empty (no members)** | - |
| `DeleteDetectorRequest` | `structure` | detectorId | - |
| `DeleteDetectorResult` | `structure` | **empty (no members)** | - |
| `DeleteDetectorVersionRequest` | `structure` | detectorId, detectorVersionId | - |
| `DeleteDetectorVersionResult` | `structure` | **empty (no members)** | - |
| `DeleteEntityTypeRequest` | `structure` | name | - |
| `AsyncJobStatus` | `enum` | IN_PROGRESS_INITIALIZING, IN_PROGRESS, CANCEL_IN_PROGRESS, CANCELED, COMPLETE, FAILED | - |
| `DataSource` | `enum` | EVENT, MODEL_SCORE, EXTERNAL_MODEL_SCORE | - |
| `DataType` | `enum` | STRING, INTEGER, FLOAT, BOOLEAN, DATETIME | - |
| `DetectorVersionStatus` | `enum` | DRAFT, ACTIVE, INACTIVE | - |
| `EventIngestion` | `enum` | ENABLED, DISABLED | - |
| `Language` | `enum` | DETECTORPL | - |
| `ListUpdateMode` | `enum` | REPLACE, APPEND, REMOVE | - |
| `ModelEndpointStatus` | `enum` | ASSOCIATED, DISSOCIATED | - |
| `ModelInputDataFormat` | `enum` | CSV, JSON | - |
| `ModelOutputDataFormat` | `enum` | CSV, JSONLINES | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
