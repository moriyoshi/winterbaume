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

- Operations: `GetAutoMergingPreview`, `GetCalculatedAttributeDefinition`, `GetCalculatedAttributeForProfile`, `GetDomain`, `GetDomainLayout`, `GetDomainObjectType`, `GetEventStream`, `GetEventTrigger`, `GetIdentityResolutionJob`, `GetIntegration`, `GetMatches`, `GetObjectTypeAttributeStatistics`, `GetProfileHistoryRecord`, `GetProfileObjectType`, `GetProfileObjectTypeTemplate`, `GetProfileRecommendations`, `GetRecommender`, `GetRecommenderFilter`, `GetRecommenderSchema`, `GetSegmentDefinition`, `GetSegmentEstimate`, `GetSegmentMembership`, `GetSegmentSnapshot`, `GetSimilarProfiles`, `GetUploadJob`, `GetUploadJobPath`, `GetWorkflow`, `GetWorkflowSteps`
- Traits: `readonly` (6), `idempotent` (1), `paginated` (1)
- Common required input members in this group: `DomainName`, `CalculatedAttributeName`, `ProfileId`, `ObjectTypeName`, `JobId`, `RecommenderName`, `SegmentDefinitionName`, `WorkflowId`

### List

- Operations: `ListAccountIntegrations`, `ListCalculatedAttributeDefinitions`, `ListCalculatedAttributesForProfile`, `ListDomainLayouts`, `ListDomainObjectTypes`, `ListDomains`, `ListEventStreams`, `ListEventTriggers`, `ListIdentityResolutionJobs`, `ListIntegrations`, `ListObjectTypeAttributes`, `ListObjectTypeAttributeValues`, `ListProfileAttributeValues`, `ListProfileHistoryRecords`, `ListProfileObjects`, `ListProfileObjectTypes`, `ListProfileObjectTypeTemplates`, `ListRecommenderFilters`, `ListRecommenderRecipes`, `ListRecommenders`, `ListRecommenderSchemas`, `ListRuleBasedMatches`, `ListSegmentDefinitions`, `ListTagsForResource`, `ListUploadJobs`, `ListWorkflows`
- Traits: `paginated` (12), `readonly` (6)
- Common required input members in this group: `DomainName`, `ProfileId`, `ObjectTypeName`, `AttributeName`

### Delete

- Operations: `DeleteCalculatedAttributeDefinition`, `DeleteDomain`, `DeleteDomainLayout`, `DeleteDomainObjectType`, `DeleteEventStream`, `DeleteEventTrigger`, `DeleteIntegration`, `DeleteProfile`, `DeleteProfileKey`, `DeleteProfileObject`, `DeleteProfileObjectType`, `DeleteRecommender`, `DeleteRecommenderFilter`, `DeleteRecommenderSchema`, `DeleteSegmentDefinition`, `DeleteWorkflow`
- Traits: `idempotent` (2)
- Common required input members in this group: `DomainName`, `ObjectTypeName`, `ProfileId`

### Create

- Operations: `CreateCalculatedAttributeDefinition`, `CreateDomain`, `CreateDomainLayout`, `CreateEventStream`, `CreateEventTrigger`, `CreateIntegrationWorkflow`, `CreateProfile`, `CreateRecommender`, `CreateRecommenderFilter`, `CreateRecommenderSchema`, `CreateSegmentDefinition`, `CreateSegmentEstimate`, `CreateSegmentSnapshot`, `CreateUploadJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `DomainName`, `DisplayName`, `ObjectTypeName`, `Fields`, `SegmentDefinitionName`

### Update

- Operations: `UpdateCalculatedAttributeDefinition`, `UpdateDomain`, `UpdateDomainLayout`, `UpdateEventTrigger`, `UpdateProfile`, `UpdateRecommender`
- Common required input members in this group: `DomainName`

### Put

- Operations: `PutDomainObjectType`, `PutIntegration`, `PutProfileObject`, `PutProfileObjectType`
- Common required input members in this group: `DomainName`, `ObjectTypeName`

### Batch

- Operations: `BatchGetCalculatedAttributeForProfile`, `BatchGetProfile`
- Common required input members in this group: `DomainName`, `ProfileIds`

### Start

- Operations: `StartRecommender`, `StartUploadJob`
- Common required input members in this group: `DomainName`

### Stop

- Operations: `StopRecommender`, `StopUploadJob`
- Common required input members in this group: `DomainName`

### Add

- Operations: `AddProfileKey`
- Common required input members in this group: -

### Detect

- Operations: `DetectProfileObjectType`
- Common required input members in this group: -

### Merge

- Operations: `MergeProfiles`
- Common required input members in this group: -

### Search

- Operations: `SearchProfiles`
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
| `AddProfileKey` | `POST /domains/{DomainName}/profiles/keys` | - | `ProfileId`, `KeyName`, `Values`, `DomainName` | - | `AddProfileKeyResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Associates a new key value with a specific profile, such as a Contact Record ContactId. A profile object can have a single unique key and any number of additional keys that can be used to identify the profile that it ... |
| `BatchGetCalculatedAttributeForProfile` | `POST /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}/batch-get-for-profiles` | - | `CalculatedAttributeName`, `DomainName`, `ProfileIds` | - | `BatchGetCalculatedAttributeForProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetch the possible attribute values given the attribute name. |
| `BatchGetProfile` | `POST /domains/{DomainName}/batch-get-profiles` | - | `DomainName`, `ProfileIds` | - | `BatchGetProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get a batch of profiles. |
| `CreateCalculatedAttributeDefinition` | `POST /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `DomainName`, `CalculatedAttributeName`, `AttributeDetails`, `Statistic` | - | `CreateCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a new calculated attribute definition. After creation, new object data ingested into Customer Profiles will be included in the calculated attribute, which can be retrieved for a profile using the GetCalculate ... |
| `CreateDomain` | `POST /domains/{DomainName}` | - | `DomainName`, `DefaultExpirationDays` | - | `CreateDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a domain, which is a container for all customer data, such as customer profile attributes, object types, profile keys, and encryption keys. You can create multiple domains, and each domain can have multiple t ... |
| `CreateDomainLayout` | `POST /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `DomainName`, `LayoutDefinitionName`, `Description`, `DisplayName`, `LayoutType`, `Layout` | - | `CreateDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates the layout to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `CreateEventStream` | `POST /domains/{DomainName}/event-streams/{EventStreamName}` | - | `DomainName`, `Uri`, `EventStreamName` | - | `CreateEventStreamResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an event stream, which is a subscription to real-time events, such as when profiles are created and updated through Amazon Connect Customer Profiles. Each event stream can be associated with only one Kinesis ... |
| `CreateEventTrigger` | `POST /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerName`, `ObjectTypeName`, `EventTriggerConditions` | - | `CreateEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an event trigger, which specifies the rules when to perform action based on customer's ingested data. Each event stream can be associated with only one integration in the same region and AWS account as the ev ... |
| `CreateIntegrationWorkflow` | `POST /domains/{DomainName}/workflows/integrations` | - | `DomainName`, `WorkflowType`, `IntegrationConfig`, `ObjectTypeName`, `RoleArn` | - | `CreateIntegrationWorkflowResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an integration workflow. An integration workflow is an async process which ingests historic data and sets up an integration for ongoing updates. The supported Amazon AppFlow sources are Salesforce, ServiceNow ... |
| `CreateProfile` | `POST /domains/{DomainName}/profiles` | - | `DomainName` | - | `CreateProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a standard profile. A standard profile represents the following attributes for a customer profile in a domain. |
| `CreateRecommender` | `POST /domains/{DomainName}/recommenders/{RecommenderName}` | - | `DomainName`, `RecommenderName`, `RecommenderRecipeName` | - | `CreateRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a recommender |
| `CreateRecommenderFilter` | `POST /domains/{DomainName}/recommender-filters/{RecommenderFilterName}` | - | `DomainName`, `RecommenderFilterName`, `RecommenderFilterExpression` | - | `CreateRecommenderFilterResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a recommender filter. A recommender filter specifies which items to include or exclude from recommendations. |
| `CreateRecommenderSchema` | `POST /domains/{DomainName}/recommender-schemas/{RecommenderSchemaName}` | - | `DomainName`, `RecommenderSchemaName`, `Fields` | - | `CreateRecommenderSchemaResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a recommender schema. A recommender schema defines the set of data columns available for training recommenders and filters under a domain. |
| `CreateSegmentDefinition` | `POST /domains/{DomainName}/segment-definitions/{SegmentDefinitionName}` | `idempotent` | `DomainName`, `SegmentDefinitionName`, `DisplayName` | - | `CreateSegmentDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a segment definition associated to the given domain. |
| `CreateSegmentEstimate` | `POST /domains/{DomainName}/segment-estimates` | - | `DomainName` | - | `CreateSegmentEstimateResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a segment estimate query. |
| `CreateSegmentSnapshot` | `POST /domains/{DomainName}/segments/{SegmentDefinitionName}/snapshots` | - | `DomainName`, `SegmentDefinitionName`, `DataFormat` | - | `CreateSegmentSnapshotResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Triggers a job to export a segment to a specified destination. |
| `CreateUploadJob` | `POST /domains/{DomainName}/upload-jobs` | - | `DomainName`, `DisplayName`, `Fields`, `UniqueKey` | - | `CreateUploadJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an Upload job to ingest data for segment imports. The metadata is created for the job with the provided field mapping and unique key. |
| `DeleteCalculatedAttributeDefinition` | `DELETE /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `DomainName`, `CalculatedAttributeName` | - | `DeleteCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an existing calculated attribute definition. Note that deleting a default calculated attribute is possible, however once deleted, you will be unable to undo that action and will need to recreate it on your ow ... |
| `DeleteDomain` | `DELETE /domains/{DomainName}` | - | `DomainName` | - | `DeleteDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a specific domain and all of its customer data, such as customer profile attributes and their related objects. |
| `DeleteDomainLayout` | `DELETE /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `DomainName`, `LayoutDefinitionName` | - | `DeleteDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the layout used to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `DeleteDomainObjectType` | `DELETE /domains/{DomainName}/domain-object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `DeleteDomainObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Delete a DomainObjectType for the given Domain and ObjectType name. |
| `DeleteEventStream` | `DELETE /domains/{DomainName}/event-streams/{EventStreamName}` | `idempotent` | `DomainName`, `EventStreamName` | - | `DeleteEventStreamResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Disables and deletes the specified event stream. |
| `DeleteEventTrigger` | `DELETE /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerName` | - | `DeleteEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Disable and deletes the Event Trigger. You cannot delete an Event Trigger with an active Integration associated. |
| `DeleteIntegration` | `POST /domains/{DomainName}/integrations/delete` | - | `DomainName`, `Uri` | - | `DeleteIntegrationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes an integration from a specific domain. |
| `DeleteProfile` | `POST /domains/{DomainName}/profiles/delete` | - | `ProfileId`, `DomainName` | - | `DeleteProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the standard customer profile and all data pertaining to the profile. |
| `DeleteProfileKey` | `POST /domains/{DomainName}/profiles/keys/delete` | - | `ProfileId`, `KeyName`, `Values`, `DomainName` | - | `DeleteProfileKeyResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes a searchable key from a customer profile. |
| `DeleteProfileObject` | `POST /domains/{DomainName}/profiles/objects/delete` | - | `ProfileId`, `ProfileObjectUniqueKey`, `ObjectTypeName`, `DomainName` | - | `DeleteProfileObjectResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes an object associated with a profile of a given ProfileObjectType. |
| `DeleteProfileObjectType` | `DELETE /domains/{DomainName}/object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `DeleteProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes a ProfileObjectType from a specific domain as well as removes all the ProfileObjects of that type. It also disables integrations from this specific ProfileObjectType. In addition, it scrubs all of the fields ... |
| `DeleteRecommender` | `DELETE /domains/{DomainName}/recommenders/{RecommenderName}` | - | `DomainName`, `RecommenderName` | - | `DeleteRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a recommender. |
| `DeleteRecommenderFilter` | `DELETE /domains/{DomainName}/recommender-filters/{RecommenderFilterName}` | - | `DomainName`, `RecommenderFilterName` | - | `DeleteRecommenderFilterResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a recommender filter from a domain. |
| `DeleteRecommenderSchema` | `DELETE /domains/{DomainName}/recommender-schemas/{RecommenderSchemaName}` | - | `DomainName`, `RecommenderSchemaName` | - | `DeleteRecommenderSchemaResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a recommender schema from a domain. |
| `DeleteSegmentDefinition` | `DELETE /domains/{DomainName}/segment-definitions/{SegmentDefinitionName}` | `idempotent` | `DomainName`, `SegmentDefinitionName` | - | `DeleteSegmentDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a segment definition from the domain. |
| `DeleteWorkflow` | `DELETE /domains/{DomainName}/workflows/{WorkflowId}` | - | `DomainName`, `WorkflowId` | - | `DeleteWorkflowResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified workflow and all its corresponding resources. This is an async process. |
| `DetectProfileObjectType` | `POST /domains/{DomainName}/detect/object-types` | - | `Objects`, `DomainName` | - | `DetectProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | The process of detecting profile object type mapping by using given objects. |
| `GetAutoMergingPreview` | `POST /domains/{DomainName}/identity-resolution-jobs/auto-merging-preview` | - | `DomainName`, `Consolidation`, `ConflictResolution` | - | `GetAutoMergingPreviewResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Tests the auto-merging settings of your Identity Resolution Job without merging your data. It randomly selects a sample of matching groups from the existing matching results, and applies the automerging settings that ... |
| `GetCalculatedAttributeDefinition` | `GET /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `DomainName`, `CalculatedAttributeName` | - | `GetCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Provides more information on a calculated attribute definition for Customer Profiles. |
| `GetCalculatedAttributeForProfile` | `GET /domains/{DomainName}/profile/{ProfileId}/calculated-attributes/{CalculatedAttributeName}` | - | `DomainName`, `ProfileId`, `CalculatedAttributeName` | - | `GetCalculatedAttributeForProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieve a calculated attribute for a customer profile. |
| `GetDomain` | `GET /domains/{DomainName}` | - | `DomainName` | - | `GetDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about a specific domain. |
| `GetDomainLayout` | `GET /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `DomainName`, `LayoutDefinitionName` | - | `GetDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the layout to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `GetDomainObjectType` | `GET /domains/{DomainName}/domain-object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `GetDomainObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Return a DomainObjectType for the input Domain and ObjectType names. |
| `GetEventStream` | `GET /domains/{DomainName}/event-streams/{EventStreamName}` | - | `DomainName`, `EventStreamName` | - | `GetEventStreamResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about the specified event stream in a specific domain. |
| `GetEventTrigger` | `GET /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerName` | - | `GetEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get a specific Event Trigger from the domain. |
| `GetIdentityResolutionJob` | `GET /domains/{DomainName}/identity-resolution-jobs/{JobId}` | - | `DomainName`, `JobId` | - | `GetIdentityResolutionJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about an Identity Resolution Job in a specific domain. Identity Resolution Jobs are set up using the Amazon Connect admin console. For more information, see Use Identity Resolution to consolidate ... |
| `GetIntegration` | `POST /domains/{DomainName}/integrations` | - | `DomainName`, `Uri` | - | `GetIntegrationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns an integration for a domain. |
| `GetMatches` | `GET /domains/{DomainName}/matches` | - | `DomainName` | - | `GetMatchesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Before calling this API, use CreateDomain or UpdateDomain to enable identity resolution: set Matching to true. GetMatches returns potentially matching profiles, based on the results of the latest run of a machine lea ... |
| `GetObjectTypeAttributeStatistics` | `POST /domains/{DomainName}/object-types/{ObjectTypeName}/attributes/{AttributeName}/statistics` | - | `DomainName`, `ObjectTypeName`, `AttributeName` | - | `GetObjectTypeAttributeStatisticsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | The GetObjectTypeAttributeValues API delivers statistical insights about attributes within a specific object type, but is exclusively available for domains with data store enabled. This API performs daily calculation ... |
| `GetProfileHistoryRecord` | `GET /domains/{DomainName}/profiles/{ProfileId}/history-records/{Id}` | - | `DomainName`, `ProfileId`, `Id` | - | `GetProfileHistoryRecordResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a history record for a specific profile, for a specific domain. |
| `GetProfileObjectType` | `GET /domains/{DomainName}/object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName` | - | `GetProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the object types for a specific domain. |
| `GetProfileObjectTypeTemplate` | `GET /templates/{TemplateId}` | - | `TemplateId` | - | `GetProfileObjectTypeTemplateResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the template information for a specific object type. A template is a predefined ProfileObjectType, such as “Salesforce-Account” or “Salesforce-Contact.” When a user sends a ProfileObject, using the PutProfile ... |
| `GetProfileRecommendations` | `POST /domains/{DomainName}/profiles/{ProfileId}/recommendations` | - | `DomainName`, `ProfileId`, `RecommenderName` | - | `GetProfileRecommendationsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetches the recommendations for a profile in the input Customer Profiles domain. Fetches all the profile recommendations |
| `GetRecommender` | `GET /domains/{DomainName}/recommenders/{RecommenderName}` | `readonly` | `DomainName`, `RecommenderName` | - | `GetRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a recommender. |
| `GetRecommenderFilter` | `GET /domains/{DomainName}/recommender-filters/{RecommenderFilterName}` | `readonly` | `DomainName`, `RecommenderFilterName` | - | `GetRecommenderFilterResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a specific recommender filter in a domain. |
| `GetRecommenderSchema` | `GET /domains/{DomainName}/recommender-schemas/{RecommenderSchemaName}` | `readonly` | `DomainName`, `RecommenderSchemaName` | - | `GetRecommenderSchemaResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a specific recommender schema in a domain. |
| `GetSegmentDefinition` | `GET /domains/{DomainName}/segment-definitions/{SegmentDefinitionName}` | `readonly` | `DomainName`, `SegmentDefinitionName` | - | `GetSegmentDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets a segment definition from the domain. |
| `GetSegmentEstimate` | `GET /domains/{DomainName}/segment-estimates/{EstimateId}` | - | `DomainName`, `EstimateId` | - | `GetSegmentEstimateResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the result of a segment estimate query. |
| `GetSegmentMembership` | `POST /domains/{DomainName}/segments/{SegmentDefinitionName}/membership` | `idempotent` | `DomainName`, `SegmentDefinitionName`, `ProfileIds` | - | `GetSegmentMembershipResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Determines if the given profiles are within a segment. |
| `GetSegmentSnapshot` | `GET /domains/{DomainName}/segments/{SegmentDefinitionName}/snapshots/{SnapshotId}` | - | `DomainName`, `SegmentDefinitionName`, `SnapshotId` | - | `GetSegmentSnapshotResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieve the latest status of a segment snapshot. |
| `GetSimilarProfiles` | `POST /domains/{DomainName}/matches` | `paginated` | `DomainName`, `MatchType`, `SearchKey`, `SearchValue` | - | `GetSimilarProfilesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a set of profiles that belong to the same matching group using the matchId or profileId . You can also specify the type of matching that you want for finding similar profiles using either RULE_BASED_MATCHING ... |
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
| `ListIdentityResolutionJobs` | `GET /domains/{DomainName}/identity-resolution-jobs` | - | `DomainName` | - | `ListIdentityResolutionJobsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the Identity Resolution Jobs in your domain. The response sorts the list by JobStartTime . |
| `ListIntegrations` | `GET /domains/{DomainName}/integrations` | - | `DomainName` | - | `ListIntegrationsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the integrations in your domain. |
| `ListObjectTypeAttributes` | `GET /domains/{DomainName}/object-types/{ObjectTypeName}/attributes` | `paginated` | `DomainName`, `ObjectTypeName` | - | `ListObjectTypeAttributesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetch the possible attribute values given the attribute name. |
| `ListObjectTypeAttributeValues` | `GET /domains/{DomainName}/object-types/{ObjectTypeName}/attributes/{AttributeName}/values` | - | `DomainName`, `ObjectTypeName`, `AttributeName` | - | `ListObjectTypeAttributeValuesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | The ListObjectTypeAttributeValues API provides access to the most recent distinct values for any specified attribute, making it valuable for real-time data validation and consistency checks within your object types. ... |
| `ListProfileAttributeValues` | `GET /domains/{DomainName}/profile-attributes/{AttributeName}/values` | - | `DomainName`, `AttributeName` | - | `ProfileAttributeValuesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Fetch the possible attribute values given the attribute name. |
| `ListProfileHistoryRecords` | `POST /domains/{DomainName}/profiles/history-records` | - | `DomainName`, `ProfileId` | - | `ListProfileHistoryRecordsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of history records for a specific profile, for a specific domain. |
| `ListProfileObjects` | `POST /domains/{DomainName}/profiles/objects` | - | `DomainName`, `ObjectTypeName`, `ProfileId` | - | `ListProfileObjectsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of objects associated with a profile of a given ProfileObjectType. |
| `ListProfileObjectTypes` | `GET /domains/{DomainName}/object-types` | - | `DomainName` | - | `ListProfileObjectTypesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the templates available within the service. |
| `ListProfileObjectTypeTemplates` | `GET /templates` | - | - | - | `ListProfileObjectTypeTemplatesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all of the template information for object types. |
| `ListRecommenderFilters` | `GET /domains/{DomainName}/recommender-filters` | `readonly`, `paginated` | `DomainName` | - | `ListRecommenderFiltersResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of recommender filters in the specified domain. |
| `ListRecommenderRecipes` | `GET /recommender-recipes` | `readonly`, `paginated` | - | - | `ListRecommenderRecipesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ThrottlingException` | Returns a list of available recommender recipes that can be used to create recommenders. |
| `ListRecommenders` | `GET /domains/{DomainName}/recommenders` | `readonly`, `paginated` | `DomainName` | - | `ListRecommendersResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of recommenders in the specified domain. |
| `ListRecommenderSchemas` | `GET /domains/{DomainName}/recommender-schemas` | `readonly`, `paginated` | `DomainName` | - | `ListRecommenderSchemasResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of recommender schemas in the specified domain. |
| `ListRuleBasedMatches` | `GET /domains/{DomainName}/profiles/ruleBasedMatches` | `paginated` | `DomainName` | - | `ListRuleBasedMatchesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a set of MatchIds that belong to the given domain. |
| `ListSegmentDefinitions` | `GET /domains/{DomainName}/segment-definitions` | `readonly`, `paginated` | `DomainName` | - | `ListSegmentDefinitionsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all segment definitions under a domain. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Displays the tags associated with an Amazon Connect Customer Profiles resource. In Connect Customer Profiles, domains, profile object types, and integrations can be tagged. |
| `ListUploadJobs` | `GET /domains/{DomainName}/upload-jobs` | `readonly`, `paginated` | `DomainName` | - | `ListUploadJobsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API retrieves a list of upload jobs for the specified domain. |
| `ListWorkflows` | `POST /domains/{DomainName}/workflows` | - | `DomainName` | - | `ListWorkflowsResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Query to list all workflows. |
| `MergeProfiles` | `POST /domains/{DomainName}/profiles/objects/merge` | - | `DomainName`, `MainProfileId`, `ProfileIdsToBeMerged` | - | `MergeProfilesResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Runs an AWS Lambda job that does the following: All the profileKeys in the ProfileToBeMerged will be moved to the main profile. All the objects in the ProfileToBeMerged will be moved to the main profile. All the Prof ... |
| `PutDomainObjectType` | `PUT /domains/{DomainName}/domain-object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName`, `Fields` | - | `PutDomainObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Create/Update a DomainObjectType in a Customer Profiles domain. To create a new DomainObjectType, Data Store needs to be enabled on the Domain. |
| `PutIntegration` | `PUT /domains/{DomainName}/integrations` | - | `DomainName` | - | `PutIntegrationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds an integration between the service and a third-party service, which includes Amazon AppFlow and Amazon Connect. An integration can belong to only one domain. To add or remove tags on an existing Integration, see ... |
| `PutProfileObject` | `PUT /domains/{DomainName}/profiles/objects` | - | `ObjectTypeName`, `Object`, `DomainName` | - | `PutProfileObjectResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds additional objects to customer profiles of a given ObjectType. When adding a specific profile object, like a Contact Record, an inferred profile can get created if it is not mapped to an existing profile. The re ... |
| `PutProfileObjectType` | `PUT /domains/{DomainName}/object-types/{ObjectTypeName}` | - | `DomainName`, `ObjectTypeName`, `Description` | - | `PutProfileObjectTypeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Defines a ProfileObjectType. To add or remove tags on an existing ObjectType, see TagResource / UntagResource . |
| `SearchProfiles` | `POST /domains/{DomainName}/profiles/search` | - | `DomainName`, `KeyName`, `Values` | - | `SearchProfilesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Searches for profiles within a specific domain using one or more predefined search keys (e.g., _fullName, _phone, _email, _account, etc.) and/or custom-defined search keys. A search key is a data type pair that consi ... |
| `StartRecommender` | `PUT /domains/{DomainName}/recommenders/{RecommenderName}/start` | - | `DomainName`, `RecommenderName` | - | `StartRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Starts a recommender that was previously stopped. Starting a recommender resumes its ability to generate recommendations. |
| `StartUploadJob` | `PUT /domains/{DomainName}/upload-jobs/{JobId}` | - | `DomainName`, `JobId` | - | `StartUploadJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API starts the processing of an upload job to ingest profile data. |
| `StopRecommender` | `PUT /domains/{DomainName}/recommenders/{RecommenderName}/stop` | - | `DomainName`, `RecommenderName` | - | `StopRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Stops a recommender, suspending its ability to generate recommendations. The recommender can be restarted later using StartRecommender. |
| `StopUploadJob` | `PUT /domains/{DomainName}/upload-jobs/{JobId}/stop` | - | `DomainName`, `JobId` | - | `StopUploadJobResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | This API stops the processing of an upload job. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Assigns one or more tags (key-value pairs) to the specified Amazon Connect Customer Profiles resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by gran ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Removes one or more tags from the specified Amazon Connect Customer Profiles resource. In Connect Customer Profiles, domains, profile object types, and integrations can be tagged. |
| `UpdateCalculatedAttributeDefinition` | `PUT /domains/{DomainName}/calculated-attributes/{CalculatedAttributeName}` | - | `DomainName`, `CalculatedAttributeName` | - | `UpdateCalculatedAttributeDefinitionResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an existing calculated attribute definition. When updating the Conditions, note that increasing the date range of a calculated attribute will not trigger inclusion of historical data greater than the current ... |
| `UpdateDomain` | `PUT /domains/{DomainName}` | - | `DomainName` | - | `UpdateDomainResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of a domain, including creating or selecting a dead letter queue or an encryption key. After a domain is created, the name can’t be changed. Use this API or CreateDomain to enable identity reso ... |
| `UpdateDomainLayout` | `PUT /domains/{DomainName}/layouts/{LayoutDefinitionName}` | - | `DomainName`, `LayoutDefinitionName` | - | `UpdateDomainLayoutResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the layout used to view data for a specific domain. This API can only be invoked from the Amazon Connect admin website. |
| `UpdateEventTrigger` | `PUT /domains/{DomainName}/event-triggers/{EventTriggerName}` | - | `DomainName`, `EventTriggerName` | - | `UpdateEventTriggerResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Update the properties of an Event Trigger. |
| `UpdateProfile` | `PUT /domains/{DomainName}/profiles` | - | `DomainName`, `ProfileId` | - | `UpdateProfileResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of a profile. The ProfileId is required for updating a customer profile. When calling the UpdateProfile API, specifying an empty string value means that any existing value will be removed. Not ... |
| `UpdateRecommender` | `PATCH /domains/{DomainName}/recommenders/{RecommenderName}` | - | `DomainName`, `RecommenderName` | - | `UpdateRecommenderResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of an existing recommender, allowing you to modify its configuration and description. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetMatches` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `GetRecommender` | - | `TrainingMetricsCount -> training-metrics-count` | - | - |
| `GetSimilarProfiles` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `GetWorkflowSteps` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListAccountIntegrations` | - | `NextToken -> next-token`, `MaxResults -> max-results`, `IncludeHidden -> include-hidden` | - | - |
| `ListCalculatedAttributeDefinitions` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListCalculatedAttributesForProfile` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListDomainLayouts` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListDomainObjectTypes` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListDomains` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListEventStreams` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListEventTriggers` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListIdentityResolutionJobs` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListIntegrations` | - | `NextToken -> next-token`, `MaxResults -> max-results`, `IncludeHidden -> include-hidden` | - | - |
| `ListObjectTypeAttributes` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListObjectTypeAttributeValues` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListProfileHistoryRecords` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListProfileObjects` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListProfileObjectTypes` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListProfileObjectTypeTemplates` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListRecommenderFilters` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListRecommenderRecipes` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListRecommenders` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListRecommenderSchemas` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListRuleBasedMatches` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListSegmentDefinitions` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListUploadJobs` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListWorkflows` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `SearchProfiles` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `BadRequestException` | `structure` | Message | The input you provided is invalid. |
| `InternalServerException` | `structure` | Message | An internal service error occurred. |
| `ResourceNotFoundException` | `structure` | Message | The requested resource does not exist, or access was denied. |
| `ThrottlingException` | `structure` | Message | You exceeded the maximum number of requests. |
| `AddProfileKeyRequest` | `structure` | ProfileId, KeyName, Values, DomainName | - |
| `AddProfileKeyResponse` | `structure` | KeyName, Values | - |
| `BatchGetCalculatedAttributeForProfileRequest` | `structure` | CalculatedAttributeName, DomainName, ProfileIds, ConditionOverrides | - |
| `BatchGetCalculatedAttributeForProfileResponse` | `structure` | Errors, CalculatedAttributeValues, ConditionOverrides | - |
| `BatchGetProfileRequest` | `structure` | DomainName, ProfileIds | - |
| `BatchGetProfileResponse` | `structure` | Errors, Profiles | - |
| `CreateCalculatedAttributeDefinitionRequest` | `structure` | DomainName, CalculatedAttributeName, DisplayName, Description, AttributeDetails, Conditions, Filter, Statistic, UseHistoricalData, Tags | - |
| `CreateCalculatedAttributeDefinitionResponse` | `structure` | CalculatedAttributeName, DisplayName, Description, AttributeDetails, Conditions, Filter, Statistic, CreatedAt, LastUpdatedAt, UseHistoricalData, Status, Readiness, ... (+1) | - |
| `CreateDomainRequest` | `structure` | DomainName, DefaultExpirationDays, DefaultEncryptionKey, DeadLetterQueueUrl, Matching, RuleBasedMatching, DataStore, Tags | - |
| `CreateDomainResponse` | `structure` | DomainName, DefaultExpirationDays, DefaultEncryptionKey, DeadLetterQueueUrl, Matching, RuleBasedMatching, DataStore, CreatedAt, LastUpdatedAt, Tags | - |
| `CreateDomainLayoutRequest` | `structure` | DomainName, LayoutDefinitionName, Description, DisplayName, IsDefault, LayoutType, Layout, Tags | - |
| `CreateDomainLayoutResponse` | `structure` | LayoutDefinitionName, Description, DisplayName, IsDefault, LayoutType, Layout, Version, Tags, CreatedAt, LastUpdatedAt | - |
| `CreateEventStreamRequest` | `structure` | DomainName, Uri, EventStreamName, Tags | - |
| `CreateEventStreamResponse` | `structure` | EventStreamArn, Tags | - |
| `CreateEventTriggerRequest` | `structure` | DomainName, EventTriggerName, ObjectTypeName, Description, EventTriggerConditions, SegmentFilter, EventTriggerLimits, Tags | - |
| `CreateEventTriggerResponse` | `structure` | EventTriggerName, ObjectTypeName, Description, EventTriggerConditions, SegmentFilter, EventTriggerLimits, CreatedAt, LastUpdatedAt, Tags | - |
| `CreateIntegrationWorkflowRequest` | `structure` | DomainName, WorkflowType, IntegrationConfig, ObjectTypeName, RoleArn, Tags | - |
| `CreateIntegrationWorkflowResponse` | `structure` | WorkflowId, Message | - |
| `CreateProfileRequest` | `structure` | DomainName, AccountNumber, AdditionalInformation, PartyType, BusinessName, FirstName, MiddleName, LastName, BirthDate, Gender, PhoneNumber, MobilePhoneNumber, ... (+14) | - |
| `CreateProfileResponse` | `structure` | ProfileId | - |
| `CreateRecommenderRequest` | `structure` | DomainName, RecommenderName, RecommenderRecipeName, RecommenderConfig, Description, RecommenderSchemaName, Tags | - |
| `CreateRecommenderResponse` | `structure` | RecommenderArn, Tags | - |
| `CreateRecommenderFilterRequest` | `structure` | DomainName, RecommenderFilterName, RecommenderFilterExpression, RecommenderSchemaName, Description, Tags | - |
| `CreateRecommenderFilterResponse` | `structure` | RecommenderFilterArn, Tags | - |
| `CreateRecommenderSchemaRequest` | `structure` | DomainName, RecommenderSchemaName, Fields, Tags | - |
| `CreateRecommenderSchemaResponse` | `structure` | RecommenderSchemaArn, RecommenderSchemaName, Fields, CreatedAt, Status, Tags | - |
| `CreateSegmentDefinitionRequest` | `structure` | DomainName, SegmentDefinitionName, DisplayName, Description, SegmentGroups, SegmentSqlQuery, SegmentSort, Tags | - |
| `CreateSegmentDefinitionResponse` | `structure` | SegmentDefinitionName, DisplayName, Description, CreatedAt, SegmentDefinitionArn, Tags | - |
| `CreateSegmentEstimateRequest` | `structure` | DomainName, SegmentQuery, SegmentSqlQuery | - |
| `CreateSegmentEstimateResponse` | `structure` | DomainName, EstimateId, StatusCode | - |
| `CreateSegmentSnapshotRequest` | `structure` | DomainName, SegmentDefinitionName, DataFormat, EncryptionKey, RoleArn, DestinationUri | - |
| `CreateSegmentSnapshotResponse` | `structure` | SnapshotId | - |
| `CreateUploadJobRequest` | `structure` | DomainName, DisplayName, Fields, UniqueKey, DataExpiry | - |
| `CreateUploadJobResponse` | `structure` | JobId | - |
| `DeleteCalculatedAttributeDefinitionRequest` | `structure` | DomainName, CalculatedAttributeName | - |
| `ActionType` | `enum` | ADDED_PROFILE_KEY, DELETED_PROFILE_KEY, CREATED, UPDATED, INGESTED, DELETED_BY_CUSTOMER, EXPIRED, MERGED, DELETED_BY_MERGE | - |
| `AttributeDimensionType` | `enum` | INCLUSIVE, EXCLUSIVE, CONTAINS, BEGINS_WITH, ENDS_WITH, BEFORE, AFTER, BETWEEN, NOT_BETWEEN, ON, GREATER_THAN, LESS_THAN, ... (+3) | - |
| `AttributeMatchingModel` | `enum` | ONE_TO_ONE, MANY_TO_MANY | - |
| `ComparisonOperator` | `enum` | INCLUSIVE, EXCLUSIVE, CONTAINS, BEGINS_WITH, ENDS_WITH, GREATER_THAN, LESS_THAN, GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL, EQUAL, BEFORE, AFTER, ... (+3) | - |
| `ConflictResolvingModel` | `enum` | RECENCY, SOURCE | - |
| `ContactType` | `enum` | PHONE_NUMBER, MOBILE_PHONE_NUMBER, HOME_PHONE_NUMBER, BUSINESS_PHONE_NUMBER, EMAIL_ADDRESS, PERSONAL_EMAIL_ADDRESS, BUSINESS_EMAIL_ADDRESS | - |
| `ContentType` | `enum` | STRING, NUMBER | - |
| `DataFormat` | `enum` | CSV, JSONL, ORC | - |
| `DataPullMode` | `enum` | INCREMENTAL, COMPLETE | - |
| `DateDimensionType` | `enum` | BEFORE, AFTER, BETWEEN, NOT_BETWEEN, ON | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
