# Amazon AppIntegrations Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon AppIntegrations actions Amazon AppIntegrations data types The Amazon AppIntegrations service enables you to configure and reuse connections to external applications. For information about how you can use external applications with Amazon Connect, see the following topics in the Amazon Connect Administrator Guide : Third-party applications (3p apps) in the agent workspace Use Amazon Q in Connect for generative AI–powered agent assistance in real-time

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon AppIntegrations Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon AppIntegrations Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: manage application integrations, data integrations, event integrations, and associations used by Amazon Connect and related workflows.
- From the operation surface: support external application connection setup, data integration metadata, event routing configuration, and tag-based integration inventory.

## Service Identity and Protocol

- AWS model slug: `appintegrations`
- AWS SDK for Rust slug: `appintegrations`
- Model version: `2020-07-29`
- Model file: `vendor/api-models-aws/models/appintegrations/service/2020-07-29/appintegrations-2020-07-29.json`
- SDK ID: `AppIntegrations`
- Endpoint prefix: `app-integrations`
- ARN namespace: `app-integrations`
- CloudFormation name: `AppIntegrations`
- CloudTrail event source: `app-integrations.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Create` (4), `Update` (4), `Delete` (3), `Get` (3), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApplication`, `CreateDataIntegration`, `CreateDataIntegrationAssociation`, `CreateEventIntegration`, `DeleteApplication`, `DeleteDataIntegration`, `DeleteEventIntegration`, `TagResource`, `UntagResource`, `UpdateApplication`, `UpdateDataIntegration`, `UpdateDataIntegrationAssociation`, `UpdateEventIntegration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetDataIntegration`, `GetEventIntegration`, `ListApplicationAssociations`, `ListApplications`, `ListDataIntegrationAssociations`, `ListDataIntegrations`, `ListEventIntegrationAssociations`, `ListEventIntegrations`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EventBridge`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/connect/latest/APIReference/API_Types_Amazon_AppIntegrations_Service.html
- https://docs.aws.amazon.com/eventbridge/latest/ref/events-ref-app-integrations.html
- https://docs.aws.amazon.com/config/latest/developerguide/appintegrations-event-integration-description.html

Research outcomes:
- Amazon AppIntegrations provides application, data integration, and event integration resources commonly used with Amazon Connect.
- Event integrations connect external event sources to destinations and emit service events through CloudTrail/EventBridge.
- Data integration resources describe how external application data can be made available to consuming services.
- AppIntegrations resources can have descriptions, tags, object configuration, and KMS encryption depending on resource type.
- Amazon Connect exposes AppIntegrations resource types in its API model because Connect consumes these integrations.

Parity implications:
- Model applications, data integrations, event integrations, associations, tags, KMS keys, and external URI/source metadata separately.
- Event integration lifecycle should be independent from downstream Connect consumption.
- CloudTrail/EventBridge service events should be distinct from the integrated application event payloads.

## Operation Groups

### List

- Operations: `ListApplicationAssociations`, `ListApplications`, `ListDataIntegrationAssociations`, `ListDataIntegrations`, `ListEventIntegrationAssociations`, `ListEventIntegrations`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: -

### Create

- Operations: `CreateApplication`, `CreateDataIntegration`, `CreateDataIntegrationAssociation`, `CreateEventIntegration`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `Name`

### Update

- Operations: `UpdateApplication`, `UpdateDataIntegration`, `UpdateDataIntegrationAssociation`, `UpdateEventIntegration`
- Common required input members in this group: -

### Delete

- Operations: `DeleteApplication`, `DeleteDataIntegration`, `DeleteEventIntegration`
- Common required input members in this group: -

### Get

- Operations: `GetApplication`, `GetDataIntegration`, `GetEventIntegration`
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
| `CreateApplication` | `POST /applications` | `idempotency-token` | `Name`, `Namespace`, `ApplicationSourceConfig` | `ClientToken` | `CreateApplicationResponse` | `AccessDeniedException`, `DuplicateResourceException`, `InternalServiceError`, `InvalidRequestException`, `ResourceQuotaExceededException`, `ThrottlingException`, `UnsupportedOperationException` | Creates and persists an Application resource. |
| `CreateDataIntegration` | `POST /dataIntegrations` | `idempotency-token` | `Name`, `KmsKey` | `ClientToken` | `CreateDataIntegrationResponse` | `AccessDeniedException`, `DuplicateResourceException`, `InternalServiceError`, `InvalidRequestException`, `ResourceQuotaExceededException`, `ThrottlingException` | Creates and persists a DataIntegration resource. You cannot create a DataIntegration association for a DataIntegration that has been previously associated. Use a different DataIntegration, or recreate the DataIntegra ... |
| `CreateDataIntegrationAssociation` | `POST /dataIntegrations/{DataIntegrationIdentifier}/associations` | `idempotency-token` | `DataIntegrationIdentifier` | `ClientToken` | `CreateDataIntegrationAssociationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceQuotaExceededException`, `ThrottlingException` | Creates and persists a DataIntegrationAssociation resource. |
| `CreateEventIntegration` | `POST /eventIntegrations` | `idempotency-token` | `Name`, `EventFilter`, `EventBridgeBus` | `ClientToken` | `CreateEventIntegrationResponse` | `AccessDeniedException`, `DuplicateResourceException`, `InternalServiceError`, `InvalidRequestException`, `ResourceQuotaExceededException`, `ThrottlingException` | Creates an EventIntegration, given a specified name, description, and a reference to an Amazon EventBridge bus in your account and a partner event source that pushes events to that bus. No objects are created in the ... |
| `DeleteApplication` | `DELETE /applications/{Arn}` | - | `Arn` | - | `DeleteApplicationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the Application. Only Applications that don't have any Application Associations can be deleted. |
| `DeleteDataIntegration` | `DELETE /dataIntegrations/{DataIntegrationIdentifier}` | - | `DataIntegrationIdentifier` | - | `DeleteDataIntegrationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the DataIntegration. Only DataIntegrations that don't have any DataIntegrationAssociations can be deleted. Deleting a DataIntegration also deletes the underlying Amazon AppFlow flow and service linked role. Y ... |
| `DeleteEventIntegration` | `DELETE /eventIntegrations/{Name}` | - | `Name` | - | `DeleteEventIntegrationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified existing event integration. If the event integration is associated with clients, the request is rejected. |
| `GetApplication` | `GET /applications/{Arn}` | - | `Arn` | - | `GetApplicationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Get an Application resource. |
| `GetDataIntegration` | `GET /dataIntegrations/{Identifier}` | - | `Identifier` | - | `GetDataIntegrationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about the DataIntegration. You cannot create a DataIntegration association for a DataIntegration that has been previously associated. Use a different DataIntegration, or recreate the DataIntegrati ... |
| `GetEventIntegration` | `GET /eventIntegrations/{Name}` | - | `Name` | - | `GetEventIntegrationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about the event integration. |
| `ListApplicationAssociations` | `GET /applications/{ApplicationId}/associations` | `paginated` | `ApplicationId` | - | `ListApplicationAssociationsResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a paginated list of application associations for an application. |
| `ListApplications` | `GET /applications` | `paginated` | - | - | `ListApplicationsResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ThrottlingException` | Lists applications in the account. |
| `ListDataIntegrationAssociations` | `GET /dataIntegrations/{DataIntegrationIdentifier}/associations` | `paginated` | `DataIntegrationIdentifier` | - | `ListDataIntegrationAssociationsResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a paginated list of DataIntegration associations in the account. You cannot create a DataIntegration association for a DataIntegration that has been previously associated. Use a different DataIntegration, or ... |
| `ListDataIntegrations` | `GET /dataIntegrations` | `paginated` | - | - | `ListDataIntegrationsResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ThrottlingException` | Returns a paginated list of DataIntegrations in the account. You cannot create a DataIntegration association for a DataIntegration that has been previously associated. Use a different DataIntegration, or recreate the ... |
| `ListEventIntegrationAssociations` | `GET /eventIntegrations/{EventIntegrationName}/associations` | `paginated` | `EventIntegrationName` | - | `ListEventIntegrationAssociationsResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a paginated list of event integration associations in the account. |
| `ListEventIntegrations` | `GET /eventIntegrations` | `paginated` | - | - | `ListEventIntegrationsResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ThrottlingException` | Returns a paginated list of event integrations in the account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the tags for the specified resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Removes the specified tags from the specified resource. |
| `UpdateApplication` | `PATCH /applications/{Arn}` | - | `Arn` | - | `UpdateApplicationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnsupportedOperationException` | Updates and persists an Application resource. |
| `UpdateDataIntegration` | `PATCH /dataIntegrations/{Identifier}` | - | `Identifier` | - | `UpdateDataIntegrationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the description of a DataIntegration. You cannot create a DataIntegration association for a DataIntegration that has been previously associated. Use a different DataIntegration, or recreate the DataIntegratio ... |
| `UpdateDataIntegrationAssociation` | `PATCH /dataIntegrations/{DataIntegrationIdentifier}/associations/{DataIntegrationAssociationIdentifier}` | - | `DataIntegrationIdentifier`, `DataIntegrationAssociationIdentifier`, `ExecutionConfiguration` | - | `UpdateDataIntegrationAssociationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates and persists a DataIntegrationAssociation resource. Updating a DataIntegrationAssociation with ExecutionConfiguration will rerun the on-demand job. |
| `UpdateEventIntegration` | `PATCH /eventIntegrations/{Name}` | - | `Name` | - | `UpdateEventIntegrationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the description of an event integration. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListApplicationAssociations` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListApplications` | - | `NextToken -> nextToken`, `MaxResults -> maxResults`, `ApplicationType -> applicationType` | - | - |
| `ListDataIntegrationAssociations` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListDataIntegrations` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListEventIntegrationAssociations` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListEventIntegrations` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `DuplicateResourceException` | `structure` | Message | A resource with the specified name already exists. |
| `InternalServiceError` | `structure` | Message | Request processing failed due to an error or failure with the service. |
| `InvalidRequestException` | `structure` | Message | The request is not valid. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource was not found. |
| `ResourceQuotaExceededException` | `structure` | Message | The allowed quota for the resource has been exceeded. |
| `ThrottlingException` | `structure` | Message | The throttling limit has been exceeded. |
| `UnsupportedOperationException` | `structure` | Message | The operation is not supported. |
| `CreateApplicationRequest` | `structure` | Name, Namespace, Description, ApplicationSourceConfig, Subscriptions, Publications, ClientToken, Tags, Permissions, IsService, InitializationTimeout, ApplicationConfig, ... (+2) | - |
| `CreateApplicationResponse` | `structure` | Arn, Id | - |
| `CreateDataIntegrationRequest` | `structure` | Name, Description, KmsKey, SourceURI, ScheduleConfig, Tags, ClientToken, FileConfiguration, ObjectConfiguration | - |
| `CreateDataIntegrationResponse` | `structure` | Arn, Id, Name, Description, KmsKey, SourceURI, ScheduleConfiguration, Tags, ClientToken, FileConfiguration, ObjectConfiguration | - |
| `CreateDataIntegrationAssociationRequest` | `structure` | DataIntegrationIdentifier, ClientId, ObjectConfiguration, DestinationURI, ClientAssociationMetadata, ClientToken, ExecutionConfiguration | - |
| `CreateDataIntegrationAssociationResponse` | `structure` | DataIntegrationAssociationId, DataIntegrationArn | - |
| `CreateEventIntegrationRequest` | `structure` | Name, Description, EventFilter, EventBridgeBus, ClientToken, Tags | - |
| `CreateEventIntegrationResponse` | `structure` | EventIntegrationArn | - |
| `DeleteApplicationRequest` | `structure` | Arn | - |
| `DeleteApplicationResponse` | `structure` | **empty (no members)** | - |
| `DeleteDataIntegrationRequest` | `structure` | DataIntegrationIdentifier | - |
| `DeleteDataIntegrationResponse` | `structure` | **empty (no members)** | - |
| `DeleteEventIntegrationRequest` | `structure` | Name | - |
| `DeleteEventIntegrationResponse` | `structure` | **empty (no members)** | - |
| `GetApplicationRequest` | `structure` | Arn | - |
| `GetApplicationResponse` | `structure` | Arn, Id, Name, Namespace, Description, ApplicationSourceConfig, Subscriptions, Publications, CreatedTime, LastModifiedTime, Tags, Permissions, ... (+5) | - |
| `GetDataIntegrationRequest` | `structure` | Identifier | - |
| `GetDataIntegrationResponse` | `structure` | Arn, Id, Name, Description, KmsKey, SourceURI, ScheduleConfiguration, Tags, FileConfiguration, ObjectConfiguration | - |
| `GetEventIntegrationRequest` | `structure` | Name | - |
| `GetEventIntegrationResponse` | `structure` | Name, Description, EventIntegrationArn, EventBridgeBus, EventFilter, Tags | - |
| `ListApplicationAssociationsRequest` | `structure` | ApplicationId, NextToken, MaxResults | - |
| `ListApplicationAssociationsResponse` | `structure` | ApplicationAssociations, NextToken | - |
| `ListApplicationsRequest` | `structure` | NextToken, MaxResults, ApplicationType | - |
| `ListApplicationsResponse` | `structure` | Applications, NextToken | - |
| `ListDataIntegrationAssociationsRequest` | `structure` | DataIntegrationIdentifier, NextToken, MaxResults | - |
| `ListDataIntegrationAssociationsResponse` | `structure` | DataIntegrationAssociations, NextToken | - |
| `ListDataIntegrationsRequest` | `structure` | NextToken, MaxResults | - |
| `ListDataIntegrationsResponse` | `structure` | DataIntegrations, NextToken | - |
| `ListEventIntegrationAssociationsRequest` | `structure` | EventIntegrationName, NextToken, MaxResults | - |
| `ListEventIntegrationAssociationsResponse` | `structure` | EventIntegrationAssociations, NextToken | - |
| `ListEventIntegrationsRequest` | `structure` | NextToken, MaxResults | - |
| `ListEventIntegrationsResponse` | `structure` | EventIntegrations, NextToken | - |
| `ApplicationType` | `enum` | STANDARD, SERVICE, MCP_SERVER | The type of application |
| `ContactHandlingScope` | `enum` | CROSS_CONTACTS, PER_CONTACT | - |
| `ExecutionMode` | `enum` | ON_DEMAND, SCHEDULED | - |
| `ExecutionStatus` | `enum` | COMPLETED, IN_PROGRESS, FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
