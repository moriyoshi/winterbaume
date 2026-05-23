# Amazon Lex Model Building V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Lex Model Building Service provides APIs for creating, managing, and deploying conversational bots and their components.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Lex Model Building V2 resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Lex Model Building V2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Delete`, `Create`, `Update` operation families, including `ListAggregatedUtterances`, `ListBotAliasReplicas`, `ListBotAliases`, `ListBotAnalyzerHistory`, `DescribeBot`, `DescribeBotAlias`.

## Service Identity and Protocol

- AWS model slug: `lex-models-v2`
- AWS SDK for Rust slug: `lexmodelsv2`
- Model version: `2020-08-07`
- Model file: `vendor/api-models-aws/models/lex-models-v2/service/2020-08-07/lex-models-v2-2020-08-07.json`
- SDK ID: `Lex Models V2`
- Endpoint prefix: `models-v2-lex`
- ARN namespace: `lex`
- CloudFormation name: `LexModelsV2`
- CloudTrail event source: `lexmodelsv2.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (32), `Describe` (19), `Delete` (16), `Create` (13), `Update` (10), `Start` (6), `Batch` (3), `Stop` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCreateCustomVocabularyItem`, `BatchDeleteCustomVocabularyItem`, `BatchUpdateCustomVocabularyItem`, `CreateBot`, `CreateBotAlias`, `CreateBotLocale`, `CreateBotReplica`, `CreateBotVersion`, `CreateExport`, `CreateIntent`, `CreateResourcePolicy`, `CreateResourcePolicyStatement`, `CreateSlot`, `CreateSlotType`, `CreateTestSetDiscrepancyReport`, `CreateUploadUrl`, `DeleteBot`, `DeleteBotAlias`, `DeleteBotAnalyzerRecommendation`, `DeleteBotLocale`, ... (+32).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBot`, `DescribeBotAlias`, `DescribeBotAnalyzerRecommendation`, `DescribeBotLocale`, `DescribeBotRecommendation`, `DescribeBotReplica`, `DescribeBotResourceGeneration`, `DescribeBotVersion`, `DescribeCustomVocabularyMetadata`, `DescribeExport`, `DescribeImport`, `DescribeIntent`, `DescribeResourcePolicy`, `DescribeSlot`, `DescribeSlotType`, `DescribeTestExecution`, `DescribeTestSet`, `DescribeTestSetDiscrepancyReport`, `DescribeTestSetGeneration`, `GetTestExecutionArtifactsUrl`, ... (+33).
- Pagination is modelled for 30 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateExport`, `CreateTestSetDiscrepancyReport`, `DeleteExport`, `DeleteImport`, `DescribeExport`, `DescribeImport`, `DescribeTestExecution`, `DescribeTestSetDiscrepancyReport`, `GetTestExecutionArtifactsUrl`, `ListExports`, `ListImports`, `ListTestExecutionResultItems`, `ListTestExecutions`, `StartBotAnalyzer`, `StartBotRecommendation`, `StartBotResourceGeneration`, `StartImport`, `StartTestExecution`, `StartTestSetGeneration`, `StopBotAnalyzer`, ... (+2).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 107 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `Lambda`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAggregatedUtterances`, `ListBotAliases`, `ListBotAliasReplicas`, `ListBotAnalyzerHistory`, `ListBotLocales`, `ListBotRecommendations`, `ListBotReplicas`, `ListBotResourceGenerations`, `ListBots`, `ListBotVersionReplicas`, `ListBotVersions`, `ListBuiltInIntents`, `ListBuiltInSlotTypes`, `ListCustomVocabularyItems`, `ListExports`, `ListImports`, `ListIntentMetrics`, `ListIntentPaths`, `ListIntents`, `ListIntentStageMetrics`, `ListRecommendedIntents`, `ListSessionAnalyticsData`, `ListSessionMetrics`, `ListSlots`, `ListSlotTypes`, `ListTagsForResource`, `ListTestExecutionResultItems`, `ListTestExecutions`, `ListTestSetRecords`, `ListTestSets`, `ListUtteranceAnalyticsData`, `ListUtteranceMetrics`
- Traits: `paginated` (29)
- Common required input members in this group: `botId`, `localeId`, `replicaRegion`, `botVersion`, `startDateTime`, `endDateTime`, `metrics`

### Describe

- Operations: `DescribeBot`, `DescribeBotAlias`, `DescribeBotAnalyzerRecommendation`, `DescribeBotLocale`, `DescribeBotRecommendation`, `DescribeBotReplica`, `DescribeBotResourceGeneration`, `DescribeBotVersion`, `DescribeCustomVocabularyMetadata`, `DescribeExport`, `DescribeImport`, `DescribeIntent`, `DescribeResourcePolicy`, `DescribeSlot`, `DescribeSlotType`, `DescribeTestExecution`, `DescribeTestSet`, `DescribeTestSetDiscrepancyReport`, `DescribeTestSetGeneration`
- Traits: `paginated` (1)
- Common required input members in this group: `botId`, `botVersion`, `localeId`, `intentId`

### Delete

- Operations: `DeleteBot`, `DeleteBotAlias`, `DeleteBotAnalyzerRecommendation`, `DeleteBotLocale`, `DeleteBotReplica`, `DeleteBotVersion`, `DeleteCustomVocabulary`, `DeleteExport`, `DeleteImport`, `DeleteIntent`, `DeleteResourcePolicy`, `DeleteResourcePolicyStatement`, `DeleteSlot`, `DeleteSlotType`, `DeleteTestSet`, `DeleteUtterances`
- Traits: `idempotent` (1)
- Common required input members in this group: `botId`, `botVersion`, `localeId`, `intentId`, `resourceArn`

### Create

- Operations: `CreateBot`, `CreateBotAlias`, `CreateBotLocale`, `CreateBotReplica`, `CreateBotVersion`, `CreateExport`, `CreateIntent`, `CreateResourcePolicy`, `CreateResourcePolicyStatement`, `CreateSlot`, `CreateSlotType`, `CreateTestSetDiscrepancyReport`, `CreateUploadUrl`
- Common required input members in this group: `botId`, `botVersion`, `localeId`, `resourceArn`

### Update

- Operations: `UpdateBot`, `UpdateBotAlias`, `UpdateBotLocale`, `UpdateBotRecommendation`, `UpdateExport`, `UpdateIntent`, `UpdateResourcePolicy`, `UpdateSlot`, `UpdateSlotType`, `UpdateTestSet`
- Traits: `idempotent` (1)
- Common required input members in this group: `botId`, `botVersion`, `localeId`, `intentId`

### Start

- Operations: `StartBotAnalyzer`, `StartBotRecommendation`, `StartBotResourceGeneration`, `StartImport`, `StartTestExecution`, `StartTestSetGeneration`
- Traits: `idempotent` (2)
- Common required input members in this group: `botId`, `botVersion`, `localeId`

### Batch

- Operations: `BatchCreateCustomVocabularyItem`, `BatchDeleteCustomVocabularyItem`, `BatchUpdateCustomVocabularyItem`
- Common required input members in this group: `botId`, `botVersion`, `localeId`, `customVocabularyItemList`

### Stop

- Operations: `StopBotAnalyzer`, `StopBotRecommendation`
- Common required input members in this group: `botId`

### Build

- Operations: `BuildBotLocale`
- Common required input members in this group: -

### Generate

- Operations: `GenerateBotElement`
- Common required input members in this group: -

### Get

- Operations: `GetTestExecutionArtifactsUrl`
- Common required input members in this group: -

### Search

- Operations: `SearchAssociatedTranscripts`
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
| `BatchCreateCustomVocabularyItem` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/batchcreate` | - | `botId`, `botVersion`, `localeId`, `customVocabularyItemList` | - | `BatchCreateCustomVocabularyItemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a batch of custom vocabulary items for a given bot locale's custom vocabulary. |
| `BatchDeleteCustomVocabularyItem` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/batchdelete` | - | `botId`, `botVersion`, `localeId`, `customVocabularyItemList` | - | `BatchDeleteCustomVocabularyItemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Delete a batch of custom vocabulary items for a given bot locale's custom vocabulary. |
| `BatchUpdateCustomVocabularyItem` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/batchupdate` | - | `botId`, `botVersion`, `localeId`, `customVocabularyItemList` | - | `BatchUpdateCustomVocabularyItemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update a batch of custom vocabulary items for a given bot locale's custom vocabulary. |
| `BuildBotLocale` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}` | - | `botId`, `botVersion`, `localeId` | - | `BuildBotLocaleResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Builds a bot, its intents, and its slot types into a specific locale. A bot can be built into multiple locales. At runtime the locale is used to choose a specific build of the bot. |
| `CreateBot` | `PUT /bots` | - | `botName`, `roleArn`, `dataPrivacy`, `idleSessionTTLInSeconds` | - | `CreateBotResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Lex conversational bot. |
| `CreateBotAlias` | `PUT /bots/{botId}/botaliases` | - | `botAliasName`, `botId` | - | `CreateBotAliasResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an alias for the specified version of a bot. Use an alias to enable you to change the version of a bot without updating applications that use the bot. For example, you can create an alias called "PROD" that y ... |
| `CreateBotLocale` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales` | - | `botId`, `botVersion`, `localeId`, `nluIntentConfidenceThreshold` | - | `CreateBotLocaleResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a locale in the bot. The locale contains the intents and slot types that the bot uses in conversations with users in the specified language and locale. You must add a locale to a bot before you can add intent ... |
| `CreateBotReplica` | `PUT /bots/{botId}/replicas` | - | `botId`, `replicaRegion` | - | `CreateBotReplicaResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Action to create a replication of the source bot in the secondary region. |
| `CreateBotVersion` | `PUT /bots/{botId}/botversions` | - | `botId`, `botVersionLocaleSpecification` | - | `CreateBotVersionResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an immutable version of the bot. When you create the first version of a bot, Amazon Lex sets the version number to 1. Subsequent bot versions increase in an increment of 1. The version number will always repr ... |
| `CreateExport` | `PUT /exports` | - | `resourceSpecification`, `fileFormat` | - | `CreateExportResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a zip archive containing the contents of a bot or a bot locale. The archive contains a directory structure that contains JSON files that define the bot. You can create an archive that contains the complete de ... |
| `CreateIntent` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents` | - | `intentName`, `botId`, `botVersion`, `localeId` | - | `CreateIntentResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an intent. To define the interaction between the user and your bot, you define one or more intents. For example, for a pizza ordering bot you would create an OrderPizza intent. When you create an intent, you ... |
| `CreateResourcePolicy` | `POST /policy/{resourceArn}` | - | `resourceArn`, `policy` | - | `CreateResourcePolicyResponse` | `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new resource policy with the specified policy statements. |
| `CreateResourcePolicyStatement` | `POST /policy/{resourceArn}/statements` | - | `resourceArn`, `statementId`, `effect`, `principal`, `action` | - | `CreateResourcePolicyStatementResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a new resource policy statement to a bot or bot alias. If a resource policy exists, the statement is added to the current resource policy. If a policy doesn't exist, a new policy is created. You can't create a r ... |
| `CreateSlot` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots` | - | `slotName`, `valueElicitationSetting`, `botId`, `botVersion`, `localeId`, `intentId` | - | `CreateSlotResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a slot in an intent. A slot is a variable needed to fulfill an intent. For example, an OrderPizza intent might need slots for size, crust, and number of pizzas. For each slot, you define one or more utterance ... |
| `CreateSlotType` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes` | - | `slotTypeName`, `botId`, `botVersion`, `localeId` | - | `CreateSlotTypeResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a custom slot type To create a custom slot type, specify a name for the slot type and a set of enumeration values, the values that a slot of this type can assume. |
| `CreateTestSetDiscrepancyReport` | `POST /testsets/{testSetId}/testsetdiscrepancy` | - | `testSetId`, `target` | - | `CreateTestSetDiscrepancyReportResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a report that describes the differences between the bot and the test set. |
| `CreateUploadUrl` | `POST /createuploadurl` | - | - | - | `CreateUploadUrlResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a pre-signed S3 write URL that you use to upload the zip archive when importing a bot or a bot locale. |
| `DeleteBot` | `DELETE /bots/{botId}` | - | `botId` | - | `DeleteBotResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes all versions of a bot, including the Draft version. To delete a specific version, use the DeleteBotVersion operation. When you delete a bot, all of the resources contained in the bot are also deleted. Deletin ... |
| `DeleteBotAlias` | `DELETE /bots/{botId}/botaliases/{botAliasId}` | - | `botAliasId`, `botId` | - | `DeleteBotAliasResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes the specified bot alias. |
| `DeleteBotAnalyzerRecommendation` | `DELETE /bots/{botId}/botanalyzer/{botAnalyzerRequestId}` | - | `botId`, `botAnalyzerRequestId` | - | `DeleteBotAnalyzerRecommendationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently deletes the recommendations and analysis results for a specific bot analysis request. This operation is provided for GDPR compliance and cannot be undone. After deletion, the analysis results cannot be re ... |
| `DeleteBotLocale` | `DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}` | - | `botId`, `botVersion`, `localeId` | - | `DeleteBotLocaleResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes a locale from a bot. When you delete a locale, all intents, slots, and slot types defined for the locale are also deleted. |
| `DeleteBotReplica` | `DELETE /bots/{botId}/replicas/{replicaRegion}` | - | `botId`, `replicaRegion` | - | `DeleteBotReplicaResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The action to delete the replicated bot in the secondary region. |
| `DeleteBotVersion` | `DELETE /bots/{botId}/botversions/{botVersion}` | - | `botId`, `botVersion` | - | `DeleteBotVersionResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a specific version of a bot. To delete all versions of a bot, use the DeleteBot operation. |
| `DeleteCustomVocabulary` | `DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary` | - | `botId`, `botVersion`, `localeId` | - | `DeleteCustomVocabularyResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes a custom vocabulary from the specified locale in the specified bot. |
| `DeleteExport` | `DELETE /exports/{exportId}` | - | `exportId` | - | `DeleteExportResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes a previous export and the associated files stored in an S3 bucket. |
| `DeleteImport` | `DELETE /imports/{importId}` | - | `importId` | - | `DeleteImportResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes a previous import and the associated file stored in an S3 bucket. |
| `DeleteIntent` | `DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}` | - | `intentId`, `botId`, `botVersion`, `localeId` | - | `Unit` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes the specified intent. Deleting an intent also deletes the slots associated with the intent. |
| `DeleteResourcePolicy` | `DELETE /policy/{resourceArn}` | - | `resourceArn` | - | `DeleteResourcePolicyResponse` | `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ThrottlingException` | Removes an existing policy from a bot or bot alias. If the resource doesn't have a policy attached, Amazon Lex returns an exception. |
| `DeleteResourcePolicyStatement` | `DELETE /policy/{resourceArn}/statements/{statementId}` | - | `resourceArn`, `statementId` | - | `DeleteResourcePolicyStatementResponse` | `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a policy statement from a resource policy. If you delete the last statement from a policy, the policy is deleted. If you specify a statement ID that doesn't exist in the policy, or if the bot or bot alias doe ... |
| `DeleteSlot` | `DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots/{slotId}` | - | `slotId`, `botId`, `botVersion`, `localeId`, `intentId` | - | `Unit` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes the specified slot from an intent. |
| `DeleteSlotType` | `DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes/{slotTypeId}` | - | `slotTypeId`, `botId`, `botVersion`, `localeId` | - | `Unit` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a slot type from a bot locale. If a slot is using the slot type, Amazon Lex throws a ResourceInUseException exception. To avoid the exception, set the skipResourceInUseCheck parameter to true . |
| `DeleteTestSet` | `DELETE /testsets/{testSetId}` | `idempotent` | `testSetId` | - | `Unit` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The action to delete the selected test set. |
| `DeleteUtterances` | `DELETE /bots/{botId}/utterances` | - | `botId` | - | `DeleteUtterancesResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes stored utterances. Amazon Lex stores the utterances that users send to your bot. Utterances are stored for 15 days for use with the ListAggregatedUtterances operation, and then stored indefinitely for use in ... |
| `DescribeBot` | `GET /bots/{botId}` | - | `botId` | - | `DescribeBotResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Provides metadata information about a bot. |
| `DescribeBotAlias` | `GET /bots/{botId}/botaliases/{botAliasId}` | - | `botAliasId`, `botId` | - | `DescribeBotAliasResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Get information about a specific bot alias. |
| `DescribeBotAnalyzerRecommendation` | `POST /bots/{botId}/botanalyzer/describe/{botAnalyzerRequestId}` | `paginated` | `botId`, `botAnalyzerRequestId` | - | `DescribeBotAnalyzerRecommendationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the analysis results and recommendations for bot optimization. The analysis must be in Available status before recommendations can be retrieved. Recommendations are returned with pagination support. Each re ... |
| `DescribeBotLocale` | `GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}` | - | `botId`, `botVersion`, `localeId` | - | `DescribeBotLocaleResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Describes the settings that a bot has for a specific locale. |
| `DescribeBotRecommendation` | `GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/botrecommendations/{botRecommendationId}` | - | `botId`, `botVersion`, `localeId`, `botRecommendationId` | - | `DescribeBotRecommendationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides metadata information about a bot recommendation. This information will enable you to get a description on the request inputs, to download associated transcripts after processing is complete, and to download ... |
| `DescribeBotReplica` | `GET /bots/{botId}/replicas/{replicaRegion}` | - | `botId`, `replicaRegion` | - | `DescribeBotReplicaResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Monitors the bot replication status through the UI console. |
| `DescribeBotResourceGeneration` | `GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/generations/{generationId}` | - | `botId`, `botVersion`, `localeId`, `generationId` | - | `DescribeBotResourceGenerationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a request to generate a bot through natural language description, made through the StartBotResource API. Use the generatedBotLocaleUrl to retrieve the Amazon S3 object containing the bot loc ... |
| `DescribeBotVersion` | `GET /bots/{botId}/botversions/{botVersion}` | - | `botId`, `botVersion` | - | `DescribeBotVersionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Provides metadata about a version of a bot. |
| `DescribeCustomVocabularyMetadata` | `GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/metadata` | - | `botId`, `botVersion`, `localeId` | - | `DescribeCustomVocabularyMetadataResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Provides metadata information about a custom vocabulary. |
| `DescribeExport` | `GET /exports/{exportId}` | - | `exportId` | - | `DescribeExportResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specific export. |
| `DescribeImport` | `GET /imports/{importId}` | - | `importId` | - | `DescribeImportResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specific import. |
| `DescribeIntent` | `GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}` | - | `intentId`, `botId`, `botVersion`, `localeId` | - | `DescribeIntentResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Returns metadata about an intent. |
| `DescribeResourcePolicy` | `GET /policy/{resourceArn}` | - | `resourceArn` | - | `DescribeResourcePolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the resource policy and policy revision for a bot or bot alias. |
| `DescribeSlot` | `GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots/{slotId}` | - | `slotId`, `botId`, `botVersion`, `localeId`, `intentId` | - | `DescribeSlotResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets metadata information about a slot. |
| `DescribeSlotType` | `GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes/{slotTypeId}` | - | `slotTypeId`, `botId`, `botVersion`, `localeId` | - | `DescribeSlotTypeResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets metadata information about a slot type. |
| `DescribeTestExecution` | `GET /testexecutions/{testExecutionId}` | - | `testExecutionId` | - | `DescribeTestExecutionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets metadata information about the test execution. |
| `DescribeTestSet` | `GET /testsets/{testSetId}` | - | `testSetId` | - | `DescribeTestSetResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets metadata information about the test set. |
| `DescribeTestSetDiscrepancyReport` | `GET /testsetdiscrepancy/{testSetDiscrepancyReportId}` | - | `testSetDiscrepancyReportId` | - | `DescribeTestSetDiscrepancyReportResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets metadata information about the test set discrepancy report. |
| `DescribeTestSetGeneration` | `GET /testsetgenerations/{testSetGenerationId}` | - | `testSetGenerationId` | - | `DescribeTestSetGenerationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets metadata information about the test set generation. |
| `GenerateBotElement` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/generate` | - | `intentId`, `botId`, `botVersion`, `localeId` | - | `GenerateBotElementResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Generates sample utterances for an intent. |
| `GetTestExecutionArtifactsUrl` | `GET /testexecutions/{testExecutionId}/artifacturl` | - | `testExecutionId` | - | `GetTestExecutionArtifactsUrlResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The pre-signed Amazon S3 URL to download the test execution result artifacts. |
| `ListAggregatedUtterances` | `POST /bots/{botId}/aggregatedutterances` | `paginated` | `botId`, `localeId`, `aggregationDuration` | - | `ListAggregatedUtterancesResponse` | `InternalServerException`, `PreconditionFailedException`, `ThrottlingException`, `ValidationException` | Provides a list of utterances that users have sent to the bot. Utterances are aggregated by the text of the utterance. For example, all instances where customers used the phrase "I want to order pizza" are aggregated ... |
| `ListBotAliases` | `POST /bots/{botId}/botaliases` | `paginated` | `botId` | - | `ListBotAliasesResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of aliases for the specified bot. |
| `ListBotAliasReplicas` | `POST /bots/{botId}/replicas/{replicaRegion}/botaliases` | `paginated` | `botId`, `replicaRegion` | - | `ListBotAliasReplicasResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The action to list the replicated bots created from the source bot alias. |
| `ListBotAnalyzerHistory` | `POST /bots/{botId}/botanalyzer/history` | `paginated` | `botId` | - | `ListBotAnalyzerHistoryResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of historical bot analysis executions for a specific bot. You can filter the results by locale and bot version. The history includes all analysis executions regardless of their status, allowing you t ... |
| `ListBotLocales` | `POST /bots/{botId}/botversions/{botVersion}/botlocales` | `paginated` | `botId`, `botVersion` | - | `ListBotLocalesResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of locales for the specified bot. |
| `ListBotRecommendations` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/botrecommendations` | `paginated` | `botId`, `botVersion`, `localeId` | - | `ListBotRecommendationsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a list of bot recommendations that meet the specified criteria. |
| `ListBotReplicas` | `POST /bots/{botId}/replicas` | - | `botId` | - | `ListBotReplicasResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The action to list the replicated bots. |
| `ListBotResourceGenerations` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/generations` | `paginated` | `botId`, `botVersion`, `localeId` | - | `ListBotResourceGenerationsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the generation requests made for a bot locale. |
| `ListBots` | `POST /bots` | `paginated` | - | - | `ListBotsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of available bots. |
| `ListBotVersionReplicas` | `POST /bots/{botId}/replicas/{replicaRegion}/botversions` | `paginated` | `botId`, `replicaRegion` | - | `ListBotVersionReplicasResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Contains information about all the versions replication statuses applicable for Global Resiliency. |
| `ListBotVersions` | `POST /bots/{botId}/botversions` | `paginated` | `botId` | - | `ListBotVersionsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets information about all of the versions of a bot. The ListBotVersions operation returns a summary of each version of a bot. For example, if a bot has three numbered versions, the ListBotVersions operation returns ... |
| `ListBuiltInIntents` | `POST /builtins/locales/{localeId}/intents` | `paginated` | `localeId` | - | `ListBuiltInIntentsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of built-in intents provided by Amazon Lex that you can use in your bot. To use a built-in intent as a the base for your own intent, include the built-in intent signature in the parentIntentSignature para ... |
| `ListBuiltInSlotTypes` | `POST /builtins/locales/{localeId}/slottypes` | `paginated` | `localeId` | - | `ListBuiltInSlotTypesResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of built-in slot types that meet the specified criteria. |
| `ListCustomVocabularyItems` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/list` | `paginated` | `botId`, `botVersion`, `localeId` | - | `ListCustomVocabularyItemsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Paginated list of custom vocabulary items for a given bot locale's custom vocabulary. |
| `ListExports` | `POST /exports` | `paginated` | - | - | `ListExportsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the exports for a bot, bot locale, or custom vocabulary. Exports are kept in the list for 7 days. |
| `ListImports` | `POST /imports` | `paginated` | - | - | `ListImportsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the imports for a bot, bot locale, or custom vocabulary. Imports are kept in the list for 7 days. |
| `ListIntentMetrics` | `POST /bots/{botId}/analytics/intentmetrics` | `paginated` | `botId`, `startDateTime`, `endDateTime`, `metrics` | - | `ListIntentMetricsResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves summary metrics for the intents in your bot. The following fields are required: metrics – A list of AnalyticsIntentMetric objects. In each object, use the name field to specify the metric to calculate, the ... |
| `ListIntentPaths` | `POST /bots/{botId}/analytics/intentpaths` | - | `botId`, `startDateTime`, `endDateTime`, `intentPath` | - | `ListIntentPathsResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves summary statistics for a path of intents that users take over sessions with your bot. The following fields are required: startDateTime and endDateTime – Define a time range for which you want to retrieve re ... |
| `ListIntents` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents` | `paginated` | `botId`, `botVersion`, `localeId` | - | `ListIntentsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Get a list of intents that meet the specified criteria. |
| `ListIntentStageMetrics` | `POST /bots/{botId}/analytics/intentstagemetrics` | `paginated` | `botId`, `startDateTime`, `endDateTime`, `metrics` | - | `ListIntentStageMetricsResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves summary metrics for the stages within intents in your bot. The following fields are required: metrics – A list of AnalyticsIntentStageMetric objects. In each object, use the name field to specify the metric ... |
| `ListRecommendedIntents` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/botrecommendations/{botRecommendationId}/intents` | `paginated` | `botId`, `botVersion`, `localeId`, `botRecommendationId` | - | `ListRecommendedIntentsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of recommended intents provided by the bot recommendation that you can use in your bot. Intents in the response are ordered by relevance. |
| `ListSessionAnalyticsData` | `POST /bots/{botId}/analytics/sessions` | `paginated` | `botId`, `startDateTime`, `endDateTime` | - | `ListSessionAnalyticsDataResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves a list of metadata for individual user sessions with your bot. The startDateTime and endDateTime fields are required. These fields define a time range for which you want to retrieve results. Of the optional ... |
| `ListSessionMetrics` | `POST /bots/{botId}/analytics/sessionmetrics` | `paginated` | `botId`, `startDateTime`, `endDateTime`, `metrics` | - | `ListSessionMetricsResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves summary metrics for the user sessions with your bot. The following fields are required: metrics – A list of AnalyticsSessionMetric objects. In each object, use the name field to specify the metric to calcul ... |
| `ListSlots` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots` | `paginated` | `botId`, `botVersion`, `localeId`, `intentId` | - | `ListSlotsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of slots that match the specified criteria. |
| `ListSlotTypes` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes` | `paginated` | `botId`, `botVersion`, `localeId` | - | `ListSlotTypesResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of slot types that match the specified criteria. |
| `ListTagsForResource` | `GET /tags/{resourceARN}` | - | `resourceARN` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a list of tags associated with a resource. Only bots, bot aliases, and bot channels can have tags associated with them. |
| `ListTestExecutionResultItems` | `POST /testexecutions/{testExecutionId}/results` | `paginated` | `testExecutionId`, `resultFilterBy` | - | `ListTestExecutionResultItemsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets a list of test execution result items. |
| `ListTestExecutions` | `POST /testexecutions` | `paginated` | - | - | `ListTestExecutionsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The list of test set executions. |
| `ListTestSetRecords` | `POST /testsets/{testSetId}/records` | `paginated` | `testSetId` | - | `ListTestSetRecordsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The list of test set records. |
| `ListTestSets` | `POST /testsets` | `paginated` | - | - | `ListTestSetsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The list of the test sets |
| `ListUtteranceAnalyticsData` | `POST /bots/{botId}/analytics/utterances` | `paginated` | `botId`, `startDateTime`, `endDateTime` | - | `ListUtteranceAnalyticsDataResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | To use this API operation, your IAM role must have permissions to perform the ListAggregatedUtterances operation, which provides access to utterance-related analytics. See Viewing utterance statistics for the IAM pol ... |
| `ListUtteranceMetrics` | `POST /bots/{botId}/analytics/utterancemetrics` | `paginated` | `botId`, `startDateTime`, `endDateTime`, `metrics` | - | `ListUtteranceMetricsResponse` | `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | To use this API operation, your IAM role must have permissions to perform the ListAggregatedUtterances operation, which provides access to utterance-related analytics. See Viewing utterance statistics for the IAM pol ... |
| `SearchAssociatedTranscripts` | `POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/botrecommendations/{botRecommendationId}/associatedtranscripts` | - | `botId`, `botVersion`, `localeId`, `botRecommendationId`, `filters` | - | `SearchAssociatedTranscriptsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Search for associated transcripts that meet the specified criteria. |
| `StartBotAnalyzer` | `POST /bots/{botId}/botanalyzer` | - | `botId`, `analysisScope` | - | `StartBotAnalyzerResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Initiates an asynchronous analysis of your bot configuration using AI-powered analysis to identify potential issues and recommend improvements based on AWS best practices. The analysis examines your bot's configurati ... |
| `StartBotRecommendation` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/botrecommendations` | - | `botId`, `botVersion`, `localeId`, `transcriptSourceSetting` | - | `StartBotRecommendationResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Use this to provide your transcript data, and to start the bot recommendation process. |
| `StartBotResourceGeneration` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/startgeneration` | `idempotent` | `generationInputPrompt`, `botId`, `botVersion`, `localeId` | - | `StartBotResourceGenerationResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a request for the descriptive bot builder to generate a bot locale configuration based on the prompt you provide it. After you make this call, use the DescribeBotResourceGeneration operation to check on the st ... |
| `StartImport` | `PUT /imports` | - | `importId`, `resourceSpecification`, `mergeStrategy` | - | `StartImportResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts importing a bot, bot locale, or custom vocabulary from a zip archive that you uploaded to an S3 bucket. |
| `StartTestExecution` | `POST /testsets/{testSetId}/testexecutions` | - | `testSetId`, `target`, `apiMode` | - | `StartTestExecutionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The action to start test set execution. |
| `StartTestSetGeneration` | `PUT /testsetgenerations` | `idempotent` | `testSetName`, `storageLocation`, `generationDataSource`, `roleArn` | - | `StartTestSetGenerationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The action to start the generation of test set. |
| `StopBotAnalyzer` | `PUT /bots/{botId}/botanalyzer/{botAnalyzerRequestId}/stop` | - | `botId`, `botAnalyzerRequestId` | - | `StopBotAnalyzerResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an ongoing bot analysis execution. Once stopped, the analysis cannot be resumed and no recommendations will be generated. |
| `StopBotRecommendation` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/botrecommendations/{botRecommendationId}/stopbotrecommendation` | - | `botId`, `botVersion`, `localeId`, `botRecommendationId` | - | `StopBotRecommendationResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Stop an already running Bot Recommendation request. |
| `TagResource` | `POST /tags/{resourceARN}` | - | `resourceARN`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds the specified tags to the specified resource. If a tag key already exists, the existing value is replaced with the new value. |
| `UntagResource` | `DELETE /tags/{resourceARN}` | - | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a bot, bot alias, or bot channel. |
| `UpdateBot` | `PUT /bots/{botId}` | - | `botId`, `botName`, `roleArn`, `dataPrivacy`, `idleSessionTTLInSeconds` | - | `UpdateBotResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an existing bot. |
| `UpdateBotAlias` | `PUT /bots/{botId}/botaliases/{botAliasId}` | - | `botAliasId`, `botAliasName`, `botId` | - | `UpdateBotAliasResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an existing bot alias. |
| `UpdateBotLocale` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}` | - | `botId`, `botVersion`, `localeId`, `nluIntentConfidenceThreshold` | - | `UpdateBotLocaleResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the settings that a bot has for a specific locale. |
| `UpdateBotRecommendation` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/botrecommendations/{botRecommendationId}` | - | `botId`, `botVersion`, `localeId`, `botRecommendationId`, `encryptionSetting` | - | `UpdateBotRecommendationResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing bot recommendation request. |
| `UpdateExport` | `PUT /exports/{exportId}` | - | `exportId` | - | `UpdateExportResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the password used to protect an export zip archive. The password is not required. If you don't supply a password, Amazon Lex generates a zip file that is not protected by a password. This is the archive that ... |
| `UpdateIntent` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}` | - | `intentId`, `intentName`, `botId`, `botVersion`, `localeId` | - | `UpdateIntentResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the settings for an intent. |
| `UpdateResourcePolicy` | `PUT /policy/{resourceArn}` | - | `resourceArn`, `policy` | - | `UpdateResourcePolicyResponse` | `InternalServerException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Replaces the existing resource policy for a bot or bot alias with a new one. If the policy doesn't exist, Amazon Lex returns an exception. |
| `UpdateSlot` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots/{slotId}` | - | `slotId`, `slotName`, `valueElicitationSetting`, `botId`, `botVersion`, `localeId`, `intentId` | - | `UpdateSlotResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the settings for a slot. |
| `UpdateSlotType` | `PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes/{slotTypeId}` | - | `slotTypeId`, `slotTypeName`, `botId`, `botVersion`, `localeId` | - | `UpdateSlotTypeResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an existing slot type. |
| `UpdateTestSet` | `PUT /testsets/{testSetId}` | `idempotent` | `testSetId`, `testSetName` | - | `UpdateTestSetResponse` | `ConflictException`, `InternalServerException`, `PreconditionFailedException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The action to update the test set. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateResourcePolicyStatement` | - | `expectedRevisionId -> expectedRevisionId` | - | - |
| `DeleteBot` | - | `skipResourceInUseCheck -> skipResourceInUseCheck` | - | - |
| `DeleteBotAlias` | - | `skipResourceInUseCheck -> skipResourceInUseCheck` | - | - |
| `DeleteBotVersion` | - | `skipResourceInUseCheck -> skipResourceInUseCheck` | - | - |
| `DeleteResourcePolicy` | - | `expectedRevisionId -> expectedRevisionId` | - | - |
| `DeleteResourcePolicyStatement` | - | `expectedRevisionId -> expectedRevisionId` | - | - |
| `DeleteSlotType` | - | `skipResourceInUseCheck -> skipResourceInUseCheck` | - | - |
| `DeleteUtterances` | - | `localeId -> localeId`, `sessionId -> sessionId` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |
| `UpdateResourcePolicy` | - | `expectedRevisionId -> expectedRevisionId` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | message | The action that you tried to perform couldn't be completed because the resource is in a conflicting state. For example, deleting a bot that is in the CREATI ... |
| `InternalServerException` | `structure` | message | The service encountered an unexpected condition. Try your request again. |
| `PreconditionFailedException` | `structure` | message | Your request couldn't be completed because one or more request fields aren't valid. Check the fields in your request and try again. |
| `ResourceNotFoundException` | `structure` | message | You asked to describe a resource that doesn't exist. Check the resource that you are requesting and try again. |
| `ServiceQuotaExceededException` | `structure` | message | You have reached a quota for your bot. |
| `ThrottlingException` | `structure` | retryAfterSeconds, message | Your request rate is too high. Reduce the frequency of requests. |
| `ValidationException` | `structure` | message | One of the input parameters in your request isn't valid. Check the parameters and try your request again. |
| `BatchCreateCustomVocabularyItemRequest` | `structure` | botId, botVersion, localeId, customVocabularyItemList | - |
| `BatchCreateCustomVocabularyItemResponse` | `structure` | botId, botVersion, localeId, errors, resources | - |
| `BatchDeleteCustomVocabularyItemRequest` | `structure` | botId, botVersion, localeId, customVocabularyItemList | - |
| `BatchDeleteCustomVocabularyItemResponse` | `structure` | botId, botVersion, localeId, errors, resources | - |
| `BatchUpdateCustomVocabularyItemRequest` | `structure` | botId, botVersion, localeId, customVocabularyItemList | - |
| `BatchUpdateCustomVocabularyItemResponse` | `structure` | botId, botVersion, localeId, errors, resources | - |
| `BuildBotLocaleRequest` | `structure` | botId, botVersion, localeId | - |
| `BuildBotLocaleResponse` | `structure` | botId, botVersion, localeId, botLocaleStatus, lastBuildSubmittedDateTime | - |
| `CreateBotRequest` | `structure` | botName, description, roleArn, dataPrivacy, idleSessionTTLInSeconds, botTags, testBotAliasTags, botType, botMembers, errorLogSettings | - |
| `CreateBotResponse` | `structure` | botId, botName, description, roleArn, dataPrivacy, idleSessionTTLInSeconds, botStatus, creationDateTime, botTags, testBotAliasTags, botType, botMembers, ... (+1) | - |
| `CreateBotAliasRequest` | `structure` | botAliasName, description, botVersion, botAliasLocaleSettings, conversationLogSettings, sentimentAnalysisSettings, botId, tags | - |
| `CreateBotAliasResponse` | `structure` | botAliasId, botAliasName, description, botVersion, botAliasLocaleSettings, conversationLogSettings, sentimentAnalysisSettings, botAliasStatus, botId, creationDateTime, tags | - |
| `CreateBotLocaleRequest` | `structure` | botId, botVersion, localeId, description, nluIntentConfidenceThreshold, voiceSettings, unifiedSpeechSettings, speechRecognitionSettings, generativeAISettings, speechDetectionSensitivity | - |
| `CreateBotLocaleResponse` | `structure` | botId, botVersion, localeName, localeId, description, nluIntentConfidenceThreshold, voiceSettings, unifiedSpeechSettings, speechRecognitionSettings, botLocaleStatus, creationDateTime, generativeAISettings, ... (+1) | - |
| `CreateBotReplicaRequest` | `structure` | botId, replicaRegion | - |
| `CreateBotReplicaResponse` | `structure` | botId, replicaRegion, sourceRegion, creationDateTime, botReplicaStatus | - |
| `CreateBotVersionRequest` | `structure` | botId, description, botVersionLocaleSpecification | - |
| `CreateBotVersionResponse` | `structure` | botId, description, botVersion, botVersionLocaleSpecification, botStatus, creationDateTime | - |
| `CreateExportRequest` | `structure` | resourceSpecification, fileFormat, filePassword | - |
| `CreateExportResponse` | `structure` | exportId, resourceSpecification, fileFormat, exportStatus, creationDateTime | - |
| `CreateIntentRequest` | `structure` | intentName, intentDisplayName, description, parentIntentSignature, sampleUtterances, dialogCodeHook, fulfillmentCodeHook, intentConfirmationSetting, intentClosingSetting, inputContexts, outputContexts, kendraConfiguration, ... (+6) | - |
| `CreateIntentResponse` | `structure` | intentId, intentName, intentDisplayName, description, parentIntentSignature, sampleUtterances, dialogCodeHook, fulfillmentCodeHook, intentConfirmationSetting, intentClosingSetting, inputContexts, outputContexts, ... (+8) | - |
| `CreateResourcePolicyRequest` | `structure` | resourceArn, policy | - |
| `CreateResourcePolicyResponse` | `structure` | resourceArn, revisionId | - |
| `CreateResourcePolicyStatementRequest` | `structure` | resourceArn, statementId, effect, principal, action, condition, expectedRevisionId | - |
| `CreateResourcePolicyStatementResponse` | `structure` | resourceArn, revisionId | - |
| `CreateSlotRequest` | `structure` | slotName, description, slotTypeId, valueElicitationSetting, obfuscationSetting, botId, botVersion, localeId, intentId, multipleValuesSetting, subSlotSetting | - |
| `CreateSlotResponse` | `structure` | slotId, slotName, description, slotTypeId, valueElicitationSetting, obfuscationSetting, botId, botVersion, localeId, intentId, creationDateTime, multipleValuesSetting, ... (+1) | - |
| `CreateSlotTypeRequest` | `structure` | slotTypeName, description, slotTypeValues, valueSelectionSetting, parentSlotTypeSignature, botId, botVersion, localeId, externalSourceSetting, compositeSlotTypeSetting | - |
| `CreateSlotTypeResponse` | `structure` | slotTypeId, slotTypeName, description, slotTypeValues, valueSelectionSetting, parentSlotTypeSignature, botId, botVersion, localeId, creationDateTime, externalSourceSetting, compositeSlotTypeSetting | - |
| `CreateTestSetDiscrepancyReportRequest` | `structure` | testSetId, target | - |
| `CreateTestSetDiscrepancyReportResponse` | `structure` | testSetDiscrepancyReportId, creationDateTime, testSetId, target | - |
| `CreateUploadUrlRequest` | `structure` | **empty (no members)** | - |
| `AggregatedUtterancesFilterName` | `enum` | Utterance | - |
| `AggregatedUtterancesFilterOperator` | `enum` | Contains, Equals | - |
| `AggregatedUtterancesSortAttribute` | `enum` | HitCount, MissedCount | - |
| `AnalysisScope` | `enum` | BotLocale | The scope of analysis to perform on the bot. Valid values include: BotLocale |
| `AnalyticsBinByName` | `enum` | ConversationStartTime, UtteranceTimestamp | - |
| `AnalyticsCommonFilterName` | `enum` | BotAliasId, BotVersion, LocaleId, Modality, Channel | - |
| `AnalyticsFilterOperator` | `enum` | Equals, GreaterThan, LessThan | - |
| `AnalyticsIntentField` | `enum` | IntentName, IntentEndState, IntentLevel | - |
| `AnalyticsIntentFilterName` | `enum` | BotAliasId, BotVersion, LocaleId, Modality, Channel, SessionId, OriginatingRequestId, IntentName, IntentEndState | - |
| `AnalyticsIntentMetricName` | `enum` | Count, Success, Failure, Switched, Dropped | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
