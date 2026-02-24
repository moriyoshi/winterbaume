# Amazon Connect Customer Profiles

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Connect Customer Profiles Customer Profiles actions Customer Profiles data types Amazon Connect Customer Profiles is a unified customer profile for your contact center that has pre-built connectors powered by AppFlow that make it easy to combine customer information from third party applications, such as Salesforce (CRM), ServiceNow (ITSM), and your enterprise resource planning (ERP), with contact history from your Amazon Connect contact center. For more information about the Amazon Connect Customer Profiles feature, see Use Customer Profiles in the Amazon Connect Administrator's Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Connect Customer Profiles resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Connect Customer Profiles workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Create`, `Update` operation families, including `GetAutoMergingPreview`, `GetCalculatedAttributeDefinition`, `GetCalculatedAttributeForProfile`, `GetDomain`, `ListAccountIntegrations`, `ListCalculatedAttributeDefinitions`.

## Service Identity and Protocol

- AWS model slug: `customer-profiles`
- AWS SDK for Rust slug: `customerprofiles`
- Model version: `2020-08-15`
- Model file: `vendor/api-models-aws/models/customer-profiles/service/2020-08-15/customer-profiles-2020-08-15.json`
- SDK ID: `Customer Profiles`
- Endpoint prefix: `profile`
- ARN namespace: `profile`
- CloudFormation name: `CustomerProfiles`
- CloudTrail event source: `profile.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (27), `List` (25), `Delete` (15), `Create` (13), `Update` (6), `Put` (4), `Batch` (2), `Start` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddProfileKey`, `BatchGetCalculatedAttributeForProfile`, `BatchGetProfile`, `CreateCalculatedAttributeDefinition`, `CreateDomain`, `CreateDomainLayout`, `CreateEventStream`, `CreateEventTrigger`, `CreateIntegrationWorkflow`, `CreateProfile`, `CreateRecommender`, `CreateRecommenderFilter`, `CreateSegmentDefinition`, `CreateSegmentEstimate`, `CreateSegmentSnapshot`, `CreateUploadJob`, `DeleteCalculatedAttributeDefinition`, `DeleteDomain`, `DeleteDomainLayout`, `DeleteDomainObjectType`, ... (+27).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAutoMergingPreview`, `GetCalculatedAttributeDefinition`, `GetCalculatedAttributeForProfile`, `GetDomain`, `GetDomainLayout`, `GetDomainObjectType`, `GetEventStream`, `GetEventTrigger`, `GetIdentityResolutionJob`, `GetIntegration`, `GetMatches`, `GetObjectTypeAttributeStatistics`, `GetProfileHistoryRecord`, `GetProfileObjectType`, `GetProfileObjectTypeTemplate`, `GetProfileRecommendations`, `GetRecommender`, `GetRecommenderFilter`, `GetSegmentDefinition`, `GetSegmentEstimate`, ... (+33).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateUploadJob`, `GetIdentityResolutionJob`, `GetUploadJob`, `GetUploadJobPath`, `ListIdentityResolutionJobs`, `ListUploadJobs`, `StartRecommender`, `StartUploadJob`, `StopRecommender`, `StopUploadJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 102 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SQS`, `Lambda`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/customerprofiles/latest/APIReference/index.html
- https://docs.aws.amazon.com/connect/latest/adminguide/customer-profile-access.html
- https://docs.aws.amazon.com/connect/latest/adminguide/integrate-customer-profiles-shopify.html

Research outcomes:
- Amazon Connect Customer Profiles stores customer profile data in domains and links profile objects from integrations.
- Domains configure storage, encryption, matching, and integration scope.
- Profiles aggregate standard and custom objects such as customer records, cases, orders, and third-party application data.
- Object types define field structure, keys, and how objects can be searched or associated.
- Integrations such as Shopify can use AppIntegrations and EventBridge/webhooks to ingest profile objects.
- Agents access Customer Profiles through the Connect agent workspace with appropriate security permissions.

Parity implications:
- Model domains, profiles, object types, profile objects, keys, integrations, event streams, matching configuration, and permissions separately.
- SearchProfiles should evaluate profile keys and object mappings, not only profile id.
- Integration ingestion should create profile objects and associations asynchronously.

## Operation Groups

### Get

- Operations: `GetAutoMergingPreview`, `GetCalculatedAttributeDefinition`, `GetCalculatedAttributeForProfile`, `GetDomain`, `GetDomainLayout`, `GetDomainObjectType`, `GetEventStream`, `GetEventTrigger`, `GetIdentityResolutionJob`, `GetIntegration`, `GetMatches`, `GetObjectTypeAttributeStatistics`, `GetProfileHistoryRecord`, `GetProfileObjectType`, `GetProfileObjectTypeTemplate`, `GetProfileRecommendations`, `GetRecommender`, `GetRecommenderFilter`, `GetSegmentDefinition`, `GetSegmentEstimate`, `GetSegmentMembership`, `GetSegmentSnapshot`, `GetSimilarProfiles`, `GetUploadJob`, `GetUploadJobPath`, `GetWorkflow`, `GetWorkflowSteps`
- Traits: `idempotent` (1), `paginated` (1), `readonly` (5)
- Common required input members in this group: `AttributeName`, `CalculatedAttributeName`, `ConflictResolution`, `Consolidation`, `DomainName`, `EstimateId`, `EventStreamName`, `EventTriggerName`, `Id`, `JobId`, `LayoutDefinitionName`, `MatchType`, `ObjectTypeName`, `ProfileId`, `ProfileIds`, `RecommenderFilterName`, `RecommenderName`, `SearchKey`, `SearchValue`, `SegmentDefinitionName`, `SnapshotId`, `TemplateId`, `Uri`, `WorkflowId`

### List

- Operations: `ListAccountIntegrations`, `ListCalculatedAttributeDefinitions`, `ListCalculatedAttributesForProfile`, `ListDomainLayouts`, `ListDomainObjectTypes`, `ListDomains`, `ListEventStreams`, `ListEventTriggers`, `ListIdentityResolutionJobs`, `ListIntegrations`, `ListObjectTypeAttributeValues`, `ListObjectTypeAttributes`, `ListProfileAttributeValues`, `ListProfileHistoryRecords`, `ListProfileObjectTypeTemplates`, `ListProfileObjectTypes`, `ListProfileObjects`, `ListRecommenderFilters`, `ListRecommenderRecipes`, `ListRecommenders`, `ListRuleBasedMatches`, `ListSegmentDefinitions`, `ListTagsForResource`, `ListUploadJobs`, `ListWorkflows`
- Traits: `paginated` (11), `readonly` (5)
- Common required input members in this group: `AttributeName`, `DomainName`, `ObjectTypeName`, `ProfileId`, `Uri`, `resourceArn`

### Delete

- Operations: `DeleteCalculatedAttributeDefinition`, `DeleteDomain`, `DeleteDomainLayout`, `DeleteDomainObjectType`, `DeleteEventStream`, `DeleteEventTrigger`, `DeleteIntegration`, `DeleteProfile`, `DeleteProfileKey`, `DeleteProfileObject`, `DeleteProfileObjectType`, `DeleteRecommender`, `DeleteRecommenderFilter`, `DeleteSegmentDefinition`, `DeleteWorkflow`
- Traits: `idempotent` (2)
- Common required input members in this group: `CalculatedAttributeName`, `DomainName`, `EventStreamName`, `EventTriggerName`, `KeyName`, `LayoutDefinitionName`, `ObjectTypeName`, `ProfileId`, `ProfileObjectUniqueKey`, `RecommenderFilterName`, `RecommenderName`, `SegmentDefinitionName`, `Uri`, `Values`, `WorkflowId`

### Create

- Operations: `CreateCalculatedAttributeDefinition`, `CreateDomain`, `CreateDomainLayout`, `CreateEventStream`, `CreateEventTrigger`, `CreateIntegrationWorkflow`, `CreateProfile`, `CreateRecommender`, `CreateRecommenderFilter`, `CreateSegmentDefinition`, `CreateSegmentEstimate`, `CreateSegmentSnapshot`, `CreateUploadJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `AttributeDetails`, `CalculatedAttributeName`, `DataFormat`, `DefaultExpirationDays`, `Description`, `DisplayName`, `DomainName`, `EventStreamName`, `EventTriggerConditions`, `EventTriggerName`, `Fields`, `IntegrationConfig`, `Layout`, `LayoutDefinitionName`, `LayoutType`, `ObjectTypeName`, `RecommenderFilterExpression`, `RecommenderFilterName`, `RecommenderName`, `RecommenderRecipeName`, `RoleArn`, `SegmentDefinitionName`, `Statistic`, `UniqueKey`, ... (+2)

### Update

- Operations: `UpdateCalculatedAttributeDefinition`, `UpdateDomain`, `UpdateDomainLayout`, `UpdateEventTrigger`, `UpdateProfile`, `UpdateRecommender`
- Common required input members in this group: `CalculatedAttributeName`, `DomainName`, `EventTriggerName`, `LayoutDefinitionName`, `ProfileId`, `RecommenderName`

### Put

- Operations: `PutDomainObjectType`, `PutIntegration`, `PutProfileObject`, `PutProfileObjectType`
- Common required input members in this group: `Description`, `DomainName`, `Fields`, `Object`, `ObjectTypeName`

### Batch

- Operations: `BatchGetCalculatedAttributeForProfile`, `BatchGetProfile`
- Common required input members in this group: `CalculatedAttributeName`, `DomainName`, `ProfileIds`

### Start

- Operations: `StartRecommender`, `StartUploadJob`
- Common required input members in this group: `DomainName`, `JobId`, `RecommenderName`

### Stop

- Operations: `StopRecommender`, `StopUploadJob`
- Common required input members in this group: `DomainName`, `JobId`, `RecommenderName`

### Add

- Operations: `AddProfileKey`
- Common required input members in this group: `DomainName`, `KeyName`, `ProfileId`, `Values`

### Detect

- Operations: `DetectProfileObjectType`
- Common required input members in this group: `DomainName`, `Objects`

### Merge

- Operations: `MergeProfiles`
- Common required input members in this group: `DomainName`, `MainProfileId`, `ProfileIdsToBeMerged`

### Search

- Operations: `SearchProfiles`
- Common required input members in this group: `DomainName`, `KeyName`, `Values`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddProfileKey` | `POST /domains/{DomainName}/profiles/keys` | - | `DomainName`, `KeyName`, `ProfileId`, `Values` | - | `AddProfileKeyResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Associates a new key value with a specific profile, such as a Contact Record ContactId. A profile object can have a single unique key and any number of additional keys that can be used to identify the profile that it belongs to. |
| `BatchGetCalculatedAttributeForProfile` | `POST /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}/batch-get-for-profiles` | - | `CalculatedAttributeName`, `DomainName`, `ProfileIds` | - | `BatchGetCalculatedAttributeForProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetch the possible attribute values given the attribute name. |
| `BatchGetProfile` | `POST /domains/{DomainName}/batch-get-profiles` | - | `DomainName`, `ProfileIds` | - | `BatchGetProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get a batch of profiles. |
| `CreateCalculatedAttributeDefinition` | `POST /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `AttributeDetails`, `CalculatedAttributeName`, `DomainName`, `Statistic` | - | `CreateCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a new calculated attribute definition. After creation, new object data ingested into Customer Profiles will be included in the calculated attribute, which can be retrieved for a profile using the GetCalculatedAttributeForProfile API. |
| `CreateDomain` | `POST /domains/{DomainName}` | - | `DefaultExpirationDays`, `DomainName` | - | `CreateDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a domain, which is a container for all customer data, such as customer profile attributes, object types, profile keys, and encryption keys. You can create multiple domains, and each domain can have multiple third-party integrations. |
| `CreateDomainLayout` | `POST /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `Description`, `DisplayName`, `DomainName`, `Layout`, `LayoutDefinitionName`, `LayoutType` | - | `CreateDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates the layout to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `CreateEventStream` | `POST /domains/{DomainName}/event-streams/{EventStreamName}` | - | `DomainName`, `EventStreamName`, `Uri` | - | `CreateEventStreamResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an event stream, which is a subscription to real-time events, such as when profiles are created and updated through Amazon Connect Customer Profiles. Each event stream can be associated with only one Kinesis Data Stream destination in the same region... |
| `CreateEventTrigger` | `POST /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerConditions`, `EventTriggerName`, `ObjectTypeName` | - | `CreateEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an event trigger, which specifies the rules when to perform action based on customer's ingested data. Each event stream can be associated with only one integration in the same region and AWS account as the event stream. |
| `CreateIntegrationWorkflow` | `POST /domains/{DomainName}/workflows/integrations` | - | `DomainName`, `IntegrationConfig`, `ObjectTypeName`, `RoleArn`, `WorkflowType` | - | `CreateIntegrationWorkflowResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an integration workflow. An integration workflow is an async process which ingests historic data and sets up an integration for ongoing updates. |
| `CreateProfile` | `POST /domains/{DomainName}/profiles` | - | `DomainName` | - | `CreateProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a standard profile. A standard profile represents the following attributes for a customer profile in a domain. |
| `CreateRecommender` | `POST /domains/{DomainName}/recommenders/{RecommenderName}` | - | `DomainName`, `RecommenderName`, `RecommenderRecipeName` | - | `CreateRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a recommender |
| `CreateRecommenderFilter` | `POST /domains/{DomainName}/recommender-filters/{RecommenderFilterName}` | - | `DomainName`, `RecommenderFilterExpression`, `RecommenderFilterName` | - | `CreateRecommenderFilterResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a recommender filter. A recommender filter specifies which items to include or exclude from recommendations. |
| `CreateSegmentDefinition` | `POST /domains/{DomainName}/segment-definitions/{SegmentDefinitionName}` | `idempotent` | `DisplayName`, `DomainName`, `SegmentDefinitionName` | - | `CreateSegmentDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a segment definition associated to the given domain. |
| `CreateSegmentEstimate` | `POST /domains/{DomainName}/segment-estimates` | - | `DomainName` | - | `CreateSegmentEstimateResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a segment estimate query. |
| `CreateSegmentSnapshot` | `POST /domains/{DomainName}/segments/{SegmentDefinitionName}/snapshots` | - | `DataFormat`, `DomainName`, `SegmentDefinitionName` | - | `CreateSegmentSnapshotResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Triggers a job to export a segment to a specified destination. |
| `CreateUploadJob` | `POST /domains/{DomainName}/upload-jobs` | - | `DisplayName`, `DomainName`, `Fields`, `UniqueKey` | - | `CreateUploadJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an Upload job to ingest data for segment imports. The metadata is created for the job with the provided field mapping and unique key. |
| `DeleteCalculatedAttributeDefinition` | `DELETE /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `CalculatedAttributeName`, `DomainName` | - | `DeleteCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an existing calculated attribute definition. Note that deleting a default calculated attribute is possible, however once deleted, you will be unable to undo that action and will need to recreate it on your own using the... |
| `DeleteDomain` | `DELETE /domains/{DomainName}` | - | `DomainName` | - | `DeleteDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a specific domain and all of its customer data, such as customer profile attributes and their related objects. |
| `DeleteDomainLayout` | `DELETE /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `DomainName`, `LayoutDefinitionName` | - | `DeleteDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the layout used to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `DeleteDomainObjectType` | `DELETE /domains/{DomainName}/domain-object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `DeleteDomainObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Delete a DomainObjectType for the given Domain and ObjectType name. |
| `DeleteEventStream` | `DELETE /domains/{DomainName}/event-streams/{EventStreamName}` | `idempotent` | `DomainName`, `EventStreamName` | - | `DeleteEventStreamResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Disables and deletes the specified event stream. |
| `DeleteEventTrigger` | `DELETE /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerName` | - | `DeleteEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Disable and deletes the Event Trigger. You cannot delete an Event Trigger with an active Integration associated. |
| `DeleteIntegration` | `POST /domains/{DomainName}/integrations/delete` | - | `DomainName`, `Uri` | - | `DeleteIntegrationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes an integration from a specific domain. |
| `DeleteProfile` | `POST /domains/{DomainName}/profiles/delete` | - | `DomainName`, `ProfileId` | - | `DeleteProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the standard customer profile and all data pertaining to the profile. |
| `DeleteProfileKey` | `POST /domains/{DomainName}/profiles/keys/delete` | - | `DomainName`, `KeyName`, `ProfileId`, `Values` | - | `DeleteProfileKeyResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes a searchable key from a customer profile. |
| `DeleteProfileObject` | `POST /domains/{DomainName}/profiles/objects/delete` | - | `DomainName`, `ObjectTypeName`, `ProfileId`, `ProfileObjectUniqueKey` | - | `DeleteProfileObjectResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes an object associated with a profile of a given ProfileObjectType. |
| `DeleteProfileObjectType` | `DELETE /domains/{DomainName}/object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `DeleteProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes a ProfileObjectType from a specific domain as well as removes all the ProfileObjects of that type. It also disables integrations from this specific ProfileObjectType. |
| `DeleteRecommender` | `DELETE /domains/{DomainName}/recommenders/{RecommenderName}` | - | `DomainName`, `RecommenderName` | - | `DeleteRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a recommender. |
| `DeleteRecommenderFilter` | `DELETE /domains/{DomainName}/recommender-filters/{RecommenderFilterName}` | - | `DomainName`, `RecommenderFilterName` | - | `DeleteRecommenderFilterResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a recommender filter from a domain. |
| `DeleteSegmentDefinition` | `DELETE /domains/{DomainName}/segment-definitions/{SegmentDefinitionName}` | `idempotent` | `DomainName`, `SegmentDefinitionName` | - | `DeleteSegmentDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a segment definition from the domain. |
| `DeleteWorkflow` | `DELETE /domains/{DomainName}/workflows/{WorkflowId}` | - | `DomainName`, `WorkflowId` | - | `DeleteWorkflowResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified workflow and all its corresponding resources. This is an async process. |
| `DetectProfileObjectType` | `POST /domains/{DomainName}/detect/object-types` | - | `DomainName`, `Objects` | - | `DetectProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | The process of detecting profile object type mapping by using given objects. |
| `GetAutoMergingPreview` | `POST /domains/{DomainName}/identity-resolution-jobs/auto-merging-preview` | - | `ConflictResolution`, `Consolidation`, `DomainName` | - | `GetAutoMergingPreviewResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Tests the auto-merging settings of your Identity Resolution Job without merging your data. It randomly selects a sample of matching groups from the existing matching results, and applies the automerging settings that you provided. |
| `GetCalculatedAttributeDefinition` | `GET /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `CalculatedAttributeName`, `DomainName` | - | `GetCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Provides more information on a calculated attribute definition for Customer Profiles. |
| `GetCalculatedAttributeForProfile` | `GET /domains/{DomainName}/profile/{ProfileId}/calculated-attributes/{CalculatedAttributeName}` | - | `CalculatedAttributeName`, `DomainName`, `ProfileId` | - | `GetCalculatedAttributeForProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieve a calculated attribute for a customer profile. |
| `GetDomain` | `GET /domains/{DomainName}` | - | `DomainName` | - | `GetDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about a specific domain. |
| `GetDomainLayout` | `GET /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `DomainName`, `LayoutDefinitionName` | - | `GetDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the layout to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `GetDomainObjectType` | `GET /domains/{DomainName}/domain-object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `GetDomainObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Return a DomainObjectType for the input Domain and ObjectType names. |
| `GetEventStream` | `GET /domains/{DomainName}/event-streams/{EventStreamName}` | - | `DomainName`, `EventStreamName` | - | `GetEventStreamResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about the specified event stream in a specific domain. |
| `GetEventTrigger` | `GET /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerName` | - | `GetEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get a specific Event Trigger from the domain. |
| `GetIdentityResolutionJob` | `GET /domains/{DomainName}/identity-resolution-jobs/{JobId}` | - | `DomainName`, `JobId` | - | `GetIdentityResolutionJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about an Identity Resolution Job in a specific domain. Identity Resolution Jobs are set up using the Amazon Connect admin console. |
| `GetIntegration` | `POST /domains/{DomainName}/integrations` | - | `DomainName`, `Uri` | - | `GetIntegrationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns an integration for a domain. |
| `GetMatches` | `GET /domains/{DomainName}/matches` | - | `DomainName` | - | `GetMatchesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Before calling this API, use CreateDomain or UpdateDomain to enable identity resolution: set `Matching` to true. GetMatches returns potentially matching profiles, based on the results of the latest run of a machine learning process. |
| `GetObjectTypeAttributeStatistics` | `POST /domains/{DomainName}/object-types/{ObjectTypeName}/attributes/{AttributeName}/statistics` | - | `AttributeName`, `DomainName`, `ObjectTypeName` | - | `GetObjectTypeAttributeStatisticsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | The GetObjectTypeAttributeValues API delivers statistical insights about attributes within a specific object type, but is exclusively available for domains with data store enabled. This API performs daily calculations to provide statistical information about... |
| `GetProfileHistoryRecord` | `GET /domains/{DomainName}/profiles/{ProfileId}/history-records/{Id}` | - | `DomainName`, `Id`, `ProfileId` | - | `GetProfileHistoryRecordResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a history record for a specific profile, for a specific domain. |
| `GetProfileObjectType` | `GET /domains/{DomainName}/object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `GetProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the object types for a specific domain. |
| `GetProfileObjectTypeTemplate` | `GET /templates/{TemplateId}` | - | `TemplateId` | - | `GetProfileObjectTypeTemplateResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the template information for a specific object type. A template is a predefined ProfileObjectType, such as “Salesforce-Account” or “Salesforce-Contact.” When a user sends a ProfileObject, using the PutProfileObject API, with an ObjectTypeName that... |
| `GetProfileRecommendations` | `POST /domains/{DomainName}/profiles/{ProfileId}/recommendations` | - | `DomainName`, `ProfileId`, `RecommenderName` | - | `GetProfileRecommendationsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetches the recommendations for a profile in the input Customer Profiles domain. Fetches all the profile recommendations |
| `GetRecommender` | `GET /domains/{DomainName}/recommenders/{RecommenderName}` | `readonly` | `DomainName`, `RecommenderName` | - | `GetRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a recommender. |
| `GetRecommenderFilter` | `GET /domains/{DomainName}/recommender-filters/{RecommenderFilterName}` | `readonly` | `DomainName`, `RecommenderFilterName` | - | `GetRecommenderFilterResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a specific recommender filter in a domain. |
| `GetSegmentDefinition` | `GET /domains/{DomainName}/segment-definitions/{SegmentDefinitionName}` | `readonly` | `DomainName`, `SegmentDefinitionName` | - | `GetSegmentDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets a segment definition from the domain. |
| `GetSegmentEstimate` | `GET /domains/{DomainName}/segment-estimates/{EstimateId}` | - | `DomainName`, `EstimateId` | - | `GetSegmentEstimateResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the result of a segment estimate query. |
| `GetSegmentMembership` | `POST /domains/{DomainName}/segments/{SegmentDefinitionName}/membership` | `idempotent` | `DomainName`, `ProfileIds`, `SegmentDefinitionName` | - | `GetSegmentMembershipResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Determines if the given profiles are within a segment. |
| `GetSegmentSnapshot` | `GET /domains/{DomainName}/segments/{SegmentDefinitionName}/snapshots/{SnapshotId}` | - | `DomainName`, `SegmentDefinitionName`, `SnapshotId` | - | `GetSegmentSnapshotResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieve the latest status of a segment snapshot. |
| `GetSimilarProfiles` | `POST /domains/{DomainName}/matches` | `paginated` | `DomainName`, `MatchType`, `SearchKey`, `SearchValue` | - | `GetSimilarProfilesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a set of profiles that belong to the same matching group using the `matchId` or `profileId`. You can also specify the type of matching that you want for finding similar profiles using either `RULE_BASED_MATCHING` or `ML_BASED_MATCHING`. |
| `GetUploadJob` | `GET /domains/{DomainName}/upload-jobs/{JobId}` | `readonly` | `DomainName`, `JobId` | - | `GetUploadJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API retrieves the details of a specific upload job. |
| `GetUploadJobPath` | `GET /domains/{DomainName}/upload-jobs/{JobId}/path` | `readonly` | `DomainName`, `JobId` | - | `GetUploadJobPathResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API retrieves the pre-signed URL and client token for uploading the file associated with the upload job. |
| `GetWorkflow` | `GET /domains/{DomainName}/workflows/{WorkflowId}` | - | `DomainName`, `WorkflowId` | - | `GetWorkflowResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get details of specified workflow. |
| `GetWorkflowSteps` | `GET /domains/{DomainName}/workflows/{WorkflowId}/steps` | - | `DomainName`, `WorkflowId` | - | `GetWorkflowStepsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get granular list of steps in workflow. |
| `ListAccountIntegrations` | `POST /integrations` | - | `Uri` | - | `ListAccountIntegrationsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the integrations associated to a specific URI in the AWS account. |
| `ListCalculatedAttributeDefinitions` | `GET /domains/{DomainName}/calculated-attributes` | - | `DomainName` | - | `ListCalculatedAttributeDefinitionsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists calculated attribute definitions for Customer Profiles |
| `ListCalculatedAttributesForProfile` | `GET /domains/{DomainName}/profile/{ProfileId}/calculated-attributes` | - | `DomainName`, `ProfileId` | - | `ListCalculatedAttributesForProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieve a list of calculated attributes for a customer profile. |
| `ListDomainLayouts` | `GET /domains/{DomainName}/layouts` | `paginated` | `DomainName` | - | `ListDomainLayoutsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the existing layouts that can be used to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `ListDomainObjectTypes` | `GET /domains/{DomainName}/domain-object-types` | `paginated` | `DomainName` | - | `ListDomainObjectTypesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | List all DomainObjectType(s) in a Customer Profiles domain. |
| `ListDomains` | `GET /domains` | - | - | - | `ListDomainsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of all the domains for an AWS account that have been created. |
| `ListEventStreams` | `GET /domains/{DomainName}/event-streams` | `paginated` | `DomainName` | - | `ListEventStreamsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of all the event streams in a specific domain. |
| `ListEventTriggers` | `GET /domains/{DomainName}/event-triggers` | `paginated` | `DomainName` | - | `ListEventTriggersResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | List all Event Triggers under a domain. |
| `ListIdentityResolutionJobs` | `GET /domains/{DomainName}/identity-resolution-jobs` | - | `DomainName` | - | `ListIdentityResolutionJobsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the Identity Resolution Jobs in your domain. The response sorts the list by `JobStartTime`. |
| `ListIntegrations` | `GET /domains/{DomainName}/integrations` | - | `DomainName` | - | `ListIntegrationsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the integrations in your domain. |
| `ListObjectTypeAttributeValues` | `GET /domains/{DomainName}/object-types/{ObjectTypeName}/attributes/{AttributeName}/values` | - | `AttributeName`, `DomainName`, `ObjectTypeName` | - | `ListObjectTypeAttributeValuesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | The ListObjectTypeAttributeValues API provides access to the most recent distinct values for any specified attribute, making it valuable for real-time data validation and consistency checks within your object types. This API works across domain, supporting... |
| `ListObjectTypeAttributes` | `GET /domains/{DomainName}/object-types/{ObjectTypeName}/attributes` | `paginated` | `DomainName`, `ObjectTypeName` | - | `ListObjectTypeAttributesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetch the possible attribute values given the attribute name. |
| `ListProfileAttributeValues` | `GET /domains/{DomainName}/profile-attributes/{AttributeName}/values` | - | `AttributeName`, `DomainName` | - | `ProfileAttributeValuesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetch the possible attribute values given the attribute name. |
| `ListProfileHistoryRecords` | `POST /domains/{DomainName}/profiles/history-records` | - | `DomainName`, `ProfileId` | - | `ListProfileHistoryRecordsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of history records for a specific profile, for a specific domain. |
| `ListProfileObjectTypeTemplates` | `GET /templates` | - | - | - | `ListProfileObjectTypeTemplatesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the template information for object types. |
| `ListProfileObjectTypes` | `GET /domains/{DomainName}/object-types` | - | `DomainName` | - | `ListProfileObjectTypesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the templates available within the service. |
| `ListProfileObjects` | `POST /domains/{DomainName}/profiles/objects` | - | `DomainName`, `ObjectTypeName`, `ProfileId` | - | `ListProfileObjectsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of objects associated with a profile of a given ProfileObjectType. |
| `ListRecommenderFilters` | `GET /domains/{DomainName}/recommender-filters` | `readonly`, `paginated` | `DomainName` | - | `ListRecommenderFiltersResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of recommender filters in the specified domain. |
| `ListRecommenderRecipes` | `GET /recommender-recipes` | `readonly`, `paginated` | - | - | `ListRecommenderRecipesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ThrottlingException` | Returns a list of available recommender recipes that can be used to create recommenders. |
| `ListRecommenders` | `GET /domains/{DomainName}/recommenders` | `readonly`, `paginated` | `DomainName` | - | `ListRecommendersResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of recommenders in the specified domain. |
| `ListRuleBasedMatches` | `GET /domains/{DomainName}/profiles/ruleBasedMatches` | `paginated` | `DomainName` | - | `ListRuleBasedMatchesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a set of `MatchIds` that belong to the given domain. |
| `ListSegmentDefinitions` | `GET /domains/{DomainName}/segment-definitions` | `readonly`, `paginated` | `DomainName` | - | `ListSegmentDefinitionsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all segment definitions under a domain. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Displays the tags associated with an Amazon Connect Customer Profiles resource. In Connect Customer Profiles, domains, profile object types, and integrations can be tagged. |
| `ListUploadJobs` | `GET /domains/{DomainName}/upload-jobs` | `readonly`, `paginated` | `DomainName` | - | `ListUploadJobsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API retrieves a list of upload jobs for the specified domain. |
| `ListWorkflows` | `POST /domains/{DomainName}/workflows` | - | `DomainName` | - | `ListWorkflowsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Query to list all workflows. |
| `MergeProfiles` | `POST /domains/{DomainName}/profiles/objects/merge` | - | `DomainName`, `MainProfileId`, `ProfileIdsToBeMerged` | - | `MergeProfilesResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Runs an AWS Lambda job that does the following: All the profileKeys in the `ProfileToBeMerged` will be moved to the main profile. All the objects in the `ProfileToBeMerged` will be moved to the main profile. |
| `PutDomainObjectType` | `PUT /domains/{DomainName}/domain-object-types/{ObjectTypeName}` | - | `DomainName`, `Fields`, `ObjectTypeName` | - | `PutDomainObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Create/Update a DomainObjectType in a Customer Profiles domain. To create a new DomainObjectType, Data Store needs to be enabled on the Domain. |
| `PutIntegration` | `PUT /domains/{DomainName}/integrations` | - | `DomainName` | - | `PutIntegrationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds an integration between the service and a third-party service, which includes Amazon AppFlow and Amazon Connect. An integration can belong to only one domain. |
| `PutProfileObject` | `PUT /domains/{DomainName}/profiles/objects` | - | `DomainName`, `Object`, `ObjectTypeName` | - | `PutProfileObjectResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds additional objects to customer profiles of a given ObjectType. When adding a specific profile object, like a Contact Record, an inferred profile can get created if it is not mapped to an existing profile. |
| `PutProfileObjectType` | `PUT /domains/{DomainName}/object-types/{ObjectTypeName}` | - | `Description`, `DomainName`, `ObjectTypeName` | - | `PutProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Defines a ProfileObjectType. To add or remove tags on an existing ObjectType, see TagResource/UntagResource. |
| `SearchProfiles` | `POST /domains/{DomainName}/profiles/search` | - | `DomainName`, `KeyName`, `Values` | - | `SearchProfilesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Searches for profiles within a specific domain using one or more predefined search keys (e.g., _fullName, _phone, _email, _account, etc.) and/or custom-defined search keys. A search key is a data type pair that consists of a `KeyName` and `Values` list. |
| `StartRecommender` | `PUT /domains/{DomainName}/recommenders/{RecommenderName}/start` | - | `DomainName`, `RecommenderName` | - | `StartRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Starts a recommender that was previously stopped. Starting a recommender resumes its ability to generate recommendations. |
| `StartUploadJob` | `PUT /domains/{DomainName}/upload-jobs/{JobId}` | - | `DomainName`, `JobId` | - | `StartUploadJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API starts the processing of an upload job to ingest profile data. |
| `StopRecommender` | `PUT /domains/{DomainName}/recommenders/{RecommenderName}/stop` | - | `DomainName`, `RecommenderName` | - | `StopRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Stops a recommender, suspending its ability to generate recommendations. The recommender can be restarted later using StartRecommender. |
| `StopUploadJob` | `PUT /domains/{DomainName}/upload-jobs/{JobId}/stop` | - | `DomainName`, `JobId` | - | `StopUploadJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API stops the processing of an upload job. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Assigns one or more tags (key-value pairs) to the specified Amazon Connect Customer Profiles resource. Tags can help you organize and categorize your resources. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Removes one or more tags from the specified Amazon Connect Customer Profiles resource. In Connect Customer Profiles, domains, profile object types, and integrations can be tagged. |
| `UpdateCalculatedAttributeDefinition` | `PUT /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `CalculatedAttributeName`, `DomainName` | - | `UpdateCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an existing calculated attribute definition. When updating the Conditions, note that increasing the date range of a calculated attribute will not trigger inclusion of historical data greater than the current date range. |
| `UpdateDomain` | `PUT /domains/{DomainName}` | - | `DomainName` | - | `UpdateDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of a domain, including creating or selecting a dead letter queue or an encryption key. After a domain is created, the name can’t be changed. |
| `UpdateDomainLayout` | `PUT /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `DomainName`, `LayoutDefinitionName` | - | `UpdateDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the layout used to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `UpdateEventTrigger` | `PUT /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerName` | - | `UpdateEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Update the properties of an Event Trigger. |
| `UpdateProfile` | `PUT /domains/{DomainName}/profiles` | - | `DomainName`, `ProfileId` | - | `UpdateProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of a profile. The ProfileId is required for updating a customer profile. |
| `UpdateRecommender` | `PATCH /domains/{DomainName}/recommenders/{RecommenderName}` | - | `DomainName`, `RecommenderName` | - | `UpdateRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of an existing recommender, allowing you to modify its configuration and description. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message` | The input you provided is invalid. |
| `InternalServerException` | `structure` | `Message` | An internal service error occurred. |
| `ResourceNotFoundException` | `structure` | `Message` | The requested resource does not exist, or access was denied. |
| `ThrottlingException` | `structure` | `Message` | You exceeded the maximum number of requests. |
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `AddProfileKeyRequest` | `structure` | `DomainName`, `KeyName`, `ProfileId`, `Values` | - |
| `AddProfileKeyResponse` | `structure` | `KeyName`, `Values` | - |
| `BatchGetCalculatedAttributeForProfileRequest` | `structure` | `CalculatedAttributeName`, `ConditionOverrides`, `DomainName`, `ProfileIds` | - |
| `BatchGetCalculatedAttributeForProfileResponse` | `structure` | `CalculatedAttributeValues`, `ConditionOverrides`, `Errors` | - |
| `BatchGetProfileRequest` | `structure` | `DomainName`, `ProfileIds` | - |
| `BatchGetProfileResponse` | `structure` | `Errors`, `Profiles` | - |
| `CreateCalculatedAttributeDefinitionRequest` | `structure` | `AttributeDetails`, `CalculatedAttributeName`, `Conditions`, `Description`, `DisplayName`, `DomainName`, `Filter`, `Statistic`, `Tags`, `UseHistoricalData` | - |
| `CreateCalculatedAttributeDefinitionResponse` | `structure` | `AttributeDetails`, `CalculatedAttributeName`, `Conditions`, `CreatedAt`, `Description`, `DisplayName`, `Filter`, `LastUpdatedAt`, `Readiness`, `Statistic`, `Status`, `Tags`, ... (+1) | - |
| `CreateDomainRequest` | `structure` | `DataStore`, `DeadLetterQueueUrl`, `DefaultEncryptionKey`, `DefaultExpirationDays`, `DomainName`, `Matching`, `RuleBasedMatching`, `Tags` | - |
| `CreateDomainResponse` | `structure` | `CreatedAt`, `DataStore`, `DeadLetterQueueUrl`, `DefaultEncryptionKey`, `DefaultExpirationDays`, `DomainName`, `LastUpdatedAt`, `Matching`, `RuleBasedMatching`, `Tags` | - |
| `CreateDomainLayoutRequest` | `structure` | `Description`, `DisplayName`, `DomainName`, `IsDefault`, `Layout`, `LayoutDefinitionName`, `LayoutType`, `Tags` | - |
| `CreateDomainLayoutResponse` | `structure` | `CreatedAt`, `Description`, `DisplayName`, `IsDefault`, `LastUpdatedAt`, `Layout`, `LayoutDefinitionName`, `LayoutType`, `Tags`, `Version` | - |
| `CreateEventStreamRequest` | `structure` | `DomainName`, `EventStreamName`, `Tags`, `Uri` | - |
| `CreateEventStreamResponse` | `structure` | `EventStreamArn`, `Tags` | - |
| `CreateEventTriggerRequest` | `structure` | `Description`, `DomainName`, `EventTriggerConditions`, `EventTriggerLimits`, `EventTriggerName`, `ObjectTypeName`, `SegmentFilter`, `Tags` | - |
| `CreateEventTriggerResponse` | `structure` | `CreatedAt`, `Description`, `EventTriggerConditions`, `EventTriggerLimits`, `EventTriggerName`, `LastUpdatedAt`, `ObjectTypeName`, `SegmentFilter`, `Tags` | - |
| `CreateIntegrationWorkflowRequest` | `structure` | `DomainName`, `IntegrationConfig`, `ObjectTypeName`, `RoleArn`, `Tags`, `WorkflowType` | - |
| `CreateIntegrationWorkflowResponse` | `structure` | `Message`, `WorkflowId` | - |
| `CreateProfileRequest` | `structure` | `AccountNumber`, `AdditionalInformation`, `Address`, `Attributes`, `BillingAddress`, `BirthDate`, `BusinessEmailAddress`, `BusinessName`, `BusinessPhoneNumber`, `DomainName`, `EmailAddress`, `EngagementPreferences`, ... (+14) | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
