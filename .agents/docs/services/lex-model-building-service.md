# Amazon Lex Model Building Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Lex Build-Time Actions Amazon Lex is an AWS service for building conversational voice and text interfaces. Use these actions to create, update, and delete conversational bots for new and existing client applications.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Lex Model Building Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: add full state-machine walks for Amazon Lex Model Building Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Lex Model Building Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Delete`, `Put`, `Create`, `Start` operation families, including `GetBot`, `GetBotAlias`, `GetBotAliases`, `GetBotChannelAssociation`, `DeleteBot`, `DeleteBotAlias`.

## Service Identity and Protocol

- AWS model slug: `lex-model-building-service`
- AWS SDK for Rust slug: `lexmodelbuildingservice`
- Model version: `2017-04-19`
- Model file: `vendor/api-models-aws/models/lex-model-building-service/service/2017-04-19/lex-model-building-service-2017-04-19.json`
- SDK ID: `Lex Model Building Service`
- Endpoint prefix: `models.lex`
- ARN namespace: `lex`
- CloudFormation name: `LexModelBuildingService`
- CloudTrail event source: `lexmodelbuildingservice.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (21), `Delete` (9), `Put` (4), `Create` (3), `Start` (2), `List` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBotVersion`, `CreateIntentVersion`, `CreateSlotTypeVersion`, `DeleteBot`, `DeleteBotAlias`, `DeleteBotChannelAssociation`, `DeleteBotVersion`, `DeleteIntent`, `DeleteIntentVersion`, `DeleteSlotType`, `DeleteSlotTypeVersion`, `DeleteUtterances`, `PutBot`, `PutBotAlias`, `PutIntent`, `PutSlotType`, `StartImport`, `StartMigration`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBot`, `GetBotAlias`, `GetBotAliases`, `GetBotChannelAssociation`, `GetBotChannelAssociations`, `GetBotVersions`, `GetBots`, `GetBuiltinIntent`, `GetBuiltinIntents`, `GetBuiltinSlotTypes`, `GetExport`, `GetImport`, `GetIntent`, `GetIntentVersions`, `GetIntents`, `GetMigration`, `GetMigrations`, `GetSlotType`, `GetSlotTypeVersions`, `GetSlotTypes`, ... (+2).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetExport`, `GetImport`, `StartImport`, `StartMigration`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 42 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Lambda`.

## Operation Groups

### Get

- Operations: `GetBot`, `GetBotAlias`, `GetBotAliases`, `GetBotChannelAssociation`, `GetBotChannelAssociations`, `GetBotVersions`, `GetBots`, `GetBuiltinIntent`, `GetBuiltinIntents`, `GetBuiltinSlotTypes`, `GetExport`, `GetImport`, `GetIntent`, `GetIntentVersions`, `GetIntents`, `GetMigration`, `GetMigrations`, `GetSlotType`, `GetSlotTypeVersions`, `GetSlotTypes`, `GetUtterancesView`
- Traits: `paginated` (11)
- Common required input members in this group: `botAlias`, `botName`, `botVersions`, `exportType`, `importId`, `migrationId`, `name`, `resourceType`, `signature`, `statusType`, `version`, `versionOrAlias`

### Delete

- Operations: `DeleteBot`, `DeleteBotAlias`, `DeleteBotChannelAssociation`, `DeleteBotVersion`, `DeleteIntent`, `DeleteIntentVersion`, `DeleteSlotType`, `DeleteSlotTypeVersion`, `DeleteUtterances`
- Common required input members in this group: `botAlias`, `botName`, `name`, `userId`, `version`

### Put

- Operations: `PutBot`, `PutBotAlias`, `PutIntent`, `PutSlotType`
- Common required input members in this group: `botName`, `botVersion`, `childDirected`, `locale`, `name`

### Create

- Operations: `CreateBotVersion`, `CreateIntentVersion`, `CreateSlotTypeVersion`
- Common required input members in this group: `name`

### Start

- Operations: `StartImport`, `StartMigration`
- Common required input members in this group: `mergeStrategy`, `migrationStrategy`, `payload`, `resourceType`, `v1BotName`, `v1BotVersion`, `v2BotName`, `v2BotRole`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `resourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateBotVersion` | `POST /bots/{name}/versions` | - | `name` | - | `CreateBotVersionResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `PreconditionFailedException` | Creates a new version of the bot based on the `$LATEST` version. If the `$LATEST` version of this resource hasn't changed since you created the last version, Amazon Lex doesn't create a new version. |
| `CreateIntentVersion` | `POST /intents/{name}/versions` | - | `name` | - | `CreateIntentVersionResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `PreconditionFailedException` | Creates a new version of an intent based on the `$LATEST` version of the intent. If the `$LATEST` version of this intent hasn't changed since you last updated it, Amazon Lex doesn't create a new version. |
| `CreateSlotTypeVersion` | `POST /slottypes/{name}/versions` | - | `name` | - | `CreateSlotTypeVersionResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `PreconditionFailedException` | Creates a new version of a slot type based on the `$LATEST` version of the specified slot type. If the `$LATEST` version of this resource has not changed since the last version that you created, Amazon Lex doesn't create a new version. |
| `DeleteBot` | `DELETE /bots/{name}` | - | `name` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `ResourceInUseException` | Deletes all versions of the bot, including the `$LATEST` version. To delete a specific version of the bot, use the DeleteBotVersion operation. |
| `DeleteBotAlias` | `DELETE /bots/{botName}/aliases/{name}` | - | `botName`, `name` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `ResourceInUseException` | Deletes an alias for the specified bot. You can't delete an alias that is used in the association between a bot and a messaging channel. |
| `DeleteBotChannelAssociation` | `DELETE /bots/{botName}/aliases/{botAlias}/channels/{name}` | - | `botAlias`, `botName`, `name` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Deletes the association between an Amazon Lex bot and a messaging platform. This operation requires permission for the `lex:DeleteBotChannelAssociation` action. |
| `DeleteBotVersion` | `DELETE /bots/{name}/versions/{version}` | - | `name`, `version` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `ResourceInUseException` | Deletes a specific version of a bot. To delete all versions of a bot, use the DeleteBot operation. |
| `DeleteIntent` | `DELETE /intents/{name}` | - | `name` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `ResourceInUseException` | Deletes all versions of the intent, including the `$LATEST` version. To delete a specific version of the intent, use the DeleteIntentVersion operation. |
| `DeleteIntentVersion` | `DELETE /intents/{name}/versions/{version}` | - | `name`, `version` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `ResourceInUseException` | Deletes a specific version of an intent. To delete all versions of a intent, use the DeleteIntent operation. |
| `DeleteSlotType` | `DELETE /slottypes/{name}` | - | `name` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `ResourceInUseException` | Deletes all versions of the slot type, including the `$LATEST` version. To delete a specific version of the slot type, use the DeleteSlotTypeVersion operation. |
| `DeleteSlotTypeVersion` | `DELETE /slottypes/{name}/version/{version}` | - | `name`, `version` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `ResourceInUseException` | Deletes a specific version of a slot type. To delete all versions of a slot type, use the DeleteSlotType operation. |
| `DeleteUtterances` | `DELETE /bots/{botName}/utterances/{userId}` | - | `botName`, `userId` | - | `Unit` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Deletes stored utterances. Amazon Lex stores the utterances that users send to your bot. |
| `GetBot` | `GET /bots/{name}/versions/{versionOrAlias}` | - | `name`, `versionOrAlias` | - | `GetBotResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns metadata information for a specific bot. You must provide the bot name and the bot version or alias. |
| `GetBotAlias` | `GET /bots/{botName}/aliases/{name}` | - | `botName`, `name` | - | `GetBotAliasResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns information about an Amazon Lex bot alias. For more information about aliases, see versioning-aliases. |
| `GetBotAliases` | `GET /bots/{botName}/aliases` | `paginated` | `botName` | - | `GetBotAliasesResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Returns a list of aliases for a specified Amazon Lex bot. This operation requires permissions for the `lex:GetBotAliases` action. |
| `GetBotChannelAssociation` | `GET /bots/{botName}/aliases/{botAlias}/channels/{name}` | - | `botAlias`, `botName`, `name` | - | `GetBotChannelAssociationResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns information about the association between an Amazon Lex bot and a messaging platform. This operation requires permissions for the `lex:GetBotChannelAssociation` action. |
| `GetBotChannelAssociations` | `GET /bots/{botName}/aliases/{botAlias}/channels` | `paginated` | `botAlias`, `botName` | - | `GetBotChannelAssociationsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Returns a list of all of the channels associated with the specified bot. The `GetBotChannelAssociations` operation requires permissions for the `lex:GetBotChannelAssociations` action. |
| `GetBotVersions` | `GET /bots/{name}/versions` | `paginated` | `name` | - | `GetBotVersionsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Gets information about all of the versions of a bot. The `GetBotVersions` operation returns a `BotMetadata` object for each version of a bot. |
| `GetBots` | `GET /bots` | `paginated` | - | - | `GetBotsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns bot information as follows: If you provide the `nameContains` field, the response includes information for the `$LATEST` version of all bots whose name contains the specified string. If you don't specify the `nameContains` field, the operation returns... |
| `GetBuiltinIntent` | `GET /builtins/intents/{signature}` | - | `signature` | - | `GetBuiltinIntentResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns information about a built-in intent. This operation requires permission for the `lex:GetBuiltinIntent` action. |
| `GetBuiltinIntents` | `GET /builtins/intents` | `paginated` | - | - | `GetBuiltinIntentsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Gets a list of built-in intents that meet the specified criteria. This operation requires permission for the `lex:GetBuiltinIntents` action. |
| `GetBuiltinSlotTypes` | `GET /builtins/slottypes` | `paginated` | - | - | `GetBuiltinSlotTypesResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Gets a list of built-in slot types that meet the specified criteria. For a list of built-in slot types, see Slot Type Reference in the Alexa Skills Kit . |
| `GetExport` | `GET /exports` | - | `exportType`, `name`, `resourceType`, `version` | - | `GetExportResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Exports the contents of a Amazon Lex resource in a specified format. |
| `GetImport` | `GET /imports/{importId}` | - | `importId` | - | `GetImportResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Gets information about an import job started with the `StartImport` operation. |
| `GetIntent` | `GET /intents/{name}/versions/{version}` | - | `name`, `version` | - | `GetIntentResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns information about an intent. In addition to the intent name, you must specify the intent version. |
| `GetIntentVersions` | `GET /intents/{name}/versions` | `paginated` | `name` | - | `GetIntentVersionsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Gets information about all of the versions of an intent. The `GetIntentVersions` operation returns an `IntentMetadata` object for each version of an intent. |
| `GetIntents` | `GET /intents` | `paginated` | - | - | `GetIntentsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns intent information as follows: If you specify the `nameContains` field, returns the `$LATEST` version of all intents that contain the specified string. If you don't specify the `nameContains` field, returns information about the `$LATEST` version of... |
| `GetMigration` | `GET /migrations/{migrationId}` | - | `migrationId` | - | `GetMigrationResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Provides details about an ongoing or complete migration from an Amazon Lex V1 bot to an Amazon Lex V2 bot. Use this operation to view the migration alerts and warnings related to the migration. |
| `GetMigrations` | `GET /migrations` | `paginated` | - | - | `GetMigrationsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Gets a list of migrations between Amazon Lex V1 and Amazon Lex V2. |
| `GetSlotType` | `GET /slottypes/{name}/versions/{version}` | - | `name`, `version` | - | `GetSlotTypeResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns information about a specific version of a slot type. In addition to specifying the slot type name, you must specify the slot type version. |
| `GetSlotTypeVersions` | `GET /slottypes/{name}/versions` | `paginated` | `name` | - | `GetSlotTypeVersionsResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Gets information about all versions of a slot type. The `GetSlotTypeVersions` operation returns a `SlotTypeMetadata` object for each version of a slot type. |
| `GetSlotTypes` | `GET /slottypes` | `paginated` | - | - | `GetSlotTypesResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Returns slot type information as follows: If you specify the `nameContains` field, returns the `$LATEST` version of all slot types that contain the specified string. If you don't specify the `nameContains` field, returns information about the `$LATEST`... |
| `GetUtterancesView` | `GET /bots/{botName}/utterances?view=aggregation` | - | `botName`, `botVersions`, `statusType` | - | `GetUtterancesViewResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Use the `GetUtterancesView` operation to get information about the utterances that your users have made to your bot. You can use this list to tune the utterances that your bot responds to. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Gets a list of tags associated with the specified resource. Only bots, bot aliases, and bot channels can have tags associated with them. |
| `PutBot` | `PUT /bots/{name}/versions/$LATEST` | - | `childDirected`, `locale`, `name` | - | `PutBotResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `PreconditionFailedException` | Creates an Amazon Lex conversational bot or replaces an existing bot. When you create or update a bot you are only required to specify a name, a locale, and whether the bot is directed toward children under age 13. |
| `PutBotAlias` | `PUT /bots/{botName}/aliases/{name}` | - | `botName`, `botVersion`, `name` | - | `PutBotAliasResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `PreconditionFailedException` | Creates an alias for the specified version of the bot or replaces an alias for the specified bot. To change the version of the bot that the alias points to, replace the alias. |
| `PutIntent` | `PUT /intents/{name}/versions/$LATEST` | - | `name` | - | `PutIntentResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `PreconditionFailedException` | Creates an intent or replaces an existing intent. To define the interaction between the user and your bot, you use one or more intents. |
| `PutSlotType` | `PUT /slottypes/{name}/versions/$LATEST` | - | `name` | - | `PutSlotTypeResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `PreconditionFailedException` | Creates a custom slot type or replaces an existing custom slot type. To create a custom slot type, specify a name for the slot type and a set of enumeration values, which are the values that a slot of this type can assume. |
| `StartImport` | `POST /imports` | - | `mergeStrategy`, `payload`, `resourceType` | - | `StartImportResponse` | `BadRequestException`, `InternalFailureException`, `LimitExceededException` | Starts a job to import a resource to Amazon Lex. |
| `StartMigration` | `POST /migrations` | - | `migrationStrategy`, `v1BotName`, `v1BotVersion`, `v2BotName`, `v2BotRole` | - | `StartMigrationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Starts migrating a bot from Amazon Lex V1 to Amazon Lex V2. Migrate your bot when you want to take advantage of the new features of Amazon Lex V2. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Adds the specified tags to the specified resource. If a tag key already exists, the existing value is replaced with the new value. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConflictException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException` | Removes tags from a bot, bot alias or bot channel. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `message` | The request is not well formed. |
| `InternalFailureException` | `structure` | `message` | An internal Amazon Lex error occurred. |
| `LimitExceededException` | `structure` | `message`, `retryAfterSeconds` | The request exceeded a limit. |
| `NotFoundException` | `structure` | `message` | The resource specified in the request was not found. |
| `ConflictException` | `structure` | `message` | There was a conflict processing the request. |
| `PreconditionFailedException` | `structure` | `message` | The checksum of the resource that you are trying to change does not match the checksum in the request. |
| `ResourceInUseException` | `structure` | `exampleReference`, `referenceType` | The resource that you are attempting to delete is referred to by another resource. |
| `CreateBotVersionRequest` | `structure` | `checksum`, `name` | - |
| `CreateBotVersionResponse` | `structure` | `abortStatement`, `checksum`, `childDirected`, `clarificationPrompt`, `createdDate`, `description`, `detectSentiment`, `enableModelImprovements`, `failureReason`, `idleSessionTTLInSeconds`, `intents`, `lastUpdatedDate`, ... (+5) | - |
| `CreateIntentVersionRequest` | `structure` | `checksum`, `name` | - |
| `CreateIntentVersionResponse` | `structure` | `checksum`, `conclusionStatement`, `confirmationPrompt`, `createdDate`, `description`, `dialogCodeHook`, `followUpPrompt`, `fulfillmentActivity`, `inputContexts`, `kendraConfiguration`, `lastUpdatedDate`, `name`, ... (+6) | - |
| `CreateSlotTypeVersionRequest` | `structure` | `checksum`, `name` | - |
| `CreateSlotTypeVersionResponse` | `structure` | `checksum`, `createdDate`, `description`, `enumerationValues`, `lastUpdatedDate`, `name`, `parentSlotTypeSignature`, `slotTypeConfigurations`, `valueSelectionStrategy`, `version` | - |
| `DeleteBotRequest` | `structure` | `name` | - |
| `DeleteBotAliasRequest` | `structure` | `botName`, `name` | - |
| `DeleteBotChannelAssociationRequest` | `structure` | `botAlias`, `botName`, `name` | - |
| `DeleteBotVersionRequest` | `structure` | `name`, `version` | - |
| `DeleteIntentRequest` | `structure` | `name` | - |
| `DeleteIntentVersionRequest` | `structure` | `name`, `version` | - |
| `DeleteSlotTypeRequest` | `structure` | `name` | - |
| `DeleteSlotTypeVersionRequest` | `structure` | `name`, `version` | - |
| `DeleteUtterancesRequest` | `structure` | `botName`, `userId` | - |
| `GetBotRequest` | `structure` | `name`, `versionOrAlias` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
