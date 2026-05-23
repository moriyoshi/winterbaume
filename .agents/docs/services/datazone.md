# Amazon DataZone

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon DataZone is a data management service that enables you to catalog, discover, govern, share, and analyze your data. With Amazon DataZone, you can share and access your data across accounts and supported regions. Amazon DataZone simplifies your experience across Amazon Web Services services, including, but not limited to, Amazon Redshift, Amazon Athena, Amazon Web Services Glue, and Amazon Web Services Lake Formation.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon DataZone where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon DataZone by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon DataZone workflows in the local mock. Key resources include `Asset`, `AssetType`, `DataProduct`, `DataSource`, `DataSourceRun`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetAccountPool`, `GetAsset`, `GetAssetFilter`, `GetAssetType`, `ListAccountPools`, `ListAccountsInAccountPool`.

## Service Identity and Protocol

- AWS model slug: `datazone`
- AWS SDK for Rust slug: `datazone`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/datazone/service/2018-05-10/datazone-2018-05-10.json`
- SDK ID: `DataZone`
- Endpoint prefix: `-`
- ARN namespace: `datazone`
- CloudFormation name: `-`
- CloudTrail event source: `datazone.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (36), `List` (33), `Create` (28), `Delete` (27), `Update` (21), `Search` (5), `Accept` (2), `Add` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptPredictions`, `AcceptSubscriptionRequest`, `AddEntityOwner`, `AddPolicyGrant`, `AssociateEnvironmentRole`, `AssociateGovernedTerms`, `BatchGetAttributesMetadata`, `BatchPutAttributesMetadata`, `CancelMetadataGenerationRun`, `CancelSubscription`, `CreateAccountPool`, `CreateAsset`, `CreateAssetFilter`, `CreateAssetRevision`, `CreateAssetType`, `CreateConnection`, `CreateDataProduct`, `CreateDataProductRevision`, `CreateDataSource`, `CreateDomain`, ... (+79).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetAttributesMetadata`, `GetAccountPool`, `GetAsset`, `GetAssetFilter`, `GetAssetType`, `GetConnection`, `GetDataExportConfiguration`, `GetDataProduct`, `GetDataSource`, `GetDataSourceRun`, `GetDomain`, `GetDomainUnit`, `GetEnvironment`, `GetEnvironmentAction`, `GetEnvironmentBlueprint`, `GetEnvironmentBlueprintConfiguration`, `GetEnvironmentCredentials`, `GetEnvironmentProfile`, `GetFormType`, `GetGlossary`, ... (+55).
- Pagination is modelled for 38 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 74 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelMetadataGenerationRun`, `CancelSubscription`, `DeleteDataExportConfiguration`, `GetDataExportConfiguration`, `GetJobRun`, `ListJobRuns`, `PutDataExportConfiguration`, `StartDataSourceRun`, `StartMetadataGenerationRun`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 176 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `Lambda`, `Glue`, `EC2/VPC`, `ECR`, `ECS`, `EKS`, `Redshift`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Asset` | `identifier` | create: `CreateAsset`; read: `GetAsset`; delete: `DeleteAsset` | `CreateAssetRevision` | - |
| `AssetType` | `domainIdentifier`, `identifier`, `revision` | create: `CreateAssetType` | `DeleteAssetType`, `GetAssetType` | - |
| `DataProduct` | `domainIdentifier`, `identifier` | create: `CreateDataProduct`; read: `GetDataProduct`; delete: `DeleteDataProduct` | `CreateDataProductRevision` | - |
| `DataSource` | `domainIdentifier`, `identifier` | create: `CreateDataSource`; read: `GetDataSource`; update: `UpdateDataSource`; delete: `DeleteDataSource`; list: `ListDataSources` | - | - |
| `DataSourceRun` | `domainIdentifier`, `identifier` | create: `StartDataSourceRun`; read: `GetDataSourceRun`; list: `ListDataSourceRuns` | - | - |
| `Domain` | `identifier` | create: `CreateDomain`; read: `GetDomain`; update: `UpdateDomain`; delete: `DeleteDomain`; list: `ListDomains` | - | - |
| `DomainUnit` | `identifier` | create: `CreateDomainUnit`; read: `GetDomainUnit`; update: `UpdateDomainUnit`; delete: `DeleteDomainUnit`; list: `ListDomainUnitsForParent` | - | - |
| `EnvironmentBlueprintConfiguration` | `domainIdentifier`, `environmentBlueprintIdentifier` | put: `PutEnvironmentBlueprintConfiguration`; read: `GetEnvironmentBlueprintConfiguration`; delete: `DeleteEnvironmentBlueprintConfiguration`; list: `ListEnvironmentBlueprintConfigurations` | - | - |
| `FormType` | `domainIdentifier`, `formTypeIdentifier`, `revision` | create: `CreateFormType` | `DeleteFormType`, `GetFormType` | - |
| `Glossary` | `domainIdentifier`, `identifier` | create: `CreateGlossary`; read: `GetGlossary`; update: `UpdateGlossary`; delete: `DeleteGlossary` | - | - |
| `GlossaryTerm` | `domainIdentifier`, `identifier` | create: `CreateGlossaryTerm`; read: `GetGlossaryTerm`; update: `UpdateGlossaryTerm`; delete: `DeleteGlossaryTerm` | - | - |
| `Listing` | `identifier` | read: `GetListing`; delete: `DeleteListing` | - | - |
| `MetadataGenerationRun` | `domainIdentifier`, `identifier` | create: `StartMetadataGenerationRun`; read: `GetMetadataGenerationRun`; update: `CancelMetadataGenerationRun`; list: `ListMetadataGenerationRuns` | - | - |
| `Rule` | `identifier` | create: `CreateRule`; read: `GetRule`; update: `UpdateRule`; delete: `DeleteRule`; list: `ListRules` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/datazone/latest/userguide/datazone-concepts.html
- https://docs.aws.amazon.com/datazone/latest/userguide/discover-subscribe-consume-data.html
- https://docs.aws.amazon.com/datazone/latest/userguide/create-maintain-business-glossary.html

Research outcomes:
- Amazon DataZone organises data governance around domains, projects, environments, blueprints, inventories, published assets, subscriptions, and fulfilment workflows.
- Domains group assets, users, projects, metadata forms, glossaries, and governance configuration.
- Projects are collaboration boundaries where users publish, discover, subscribe to, and consume data assets.
- Assets are published into the catalogue and can be discovered, requested, approved, and fulfilled through subscription workflows.
- Business glossaries define terms and definitions that can be associated with catalogue assets.
- Subscription fulfilment can grant access through services such as Glue, Redshift, JDBC sources, Lake Formation, and EventBridge-supported workflows.

Parity implications:
- Model domains, projects, environments, blueprints, assets, listings, subscriptions, glossary terms, metadata forms, and fulfilment grants separately.
- Subscription request, approval, revocation, and fulfilment should be stateful.
- Catalogue metadata should be distinct from underlying data-plane storage.

## Operation Groups

### List

- Operations: `ListAccountPools`, `ListAccountsInAccountPool`, `ListAssetFilters`, `ListAssetRevisions`, `ListConnections`, `ListDataProductRevisions`, `ListDataSourceRunActivities`, `ListEntityOwners`, `ListEnvironmentActions`, `ListEnvironmentBlueprints`, `ListEnvironmentProfiles`, `ListEnvironments`, `ListJobRuns`, `ListLineageEvents`, `ListLineageNodeHistory`, `ListNotifications`, `ListPolicyGrants`, `ListProjectMemberships`, `ListProjectProfiles`, `ListProjects`, `ListSubscriptionGrants`, `ListSubscriptionRequests`, `ListSubscriptions`, `ListSubscriptionTargets`, `ListTagsForResource`, `ListTimeSeriesDataPoints`
- Traits: `readonly` (26), `paginated` (25)
- Common required input members in this group: `domainIdentifier`, `identifier`, `entityType`, `entityIdentifier`, `environmentIdentifier`, `projectIdentifier`

### Get

- Operations: `GetAccountPool`, `GetAssetFilter`, `GetConnection`, `GetDataExportConfiguration`, `GetEnvironment`, `GetEnvironmentAction`, `GetEnvironmentBlueprint`, `GetEnvironmentCredentials`, `GetEnvironmentProfile`, `GetGroupProfile`, `GetIamPortalLoginUrl`, `GetJobRun`, `GetLineageEvent`, `GetLineageNode`, `GetProject`, `GetProjectProfile`, `GetSubscription`, `GetSubscriptionGrant`, `GetSubscriptionRequestDetails`, `GetSubscriptionTarget`, `GetTimeSeriesDataPoint`, `GetUserProfile`
- Traits: `readonly` (21)
- Common required input members in this group: `domainIdentifier`, `identifier`, `environmentIdentifier`

### Create

- Operations: `CreateAccountPool`, `CreateAssetFilter`, `CreateConnection`, `CreateEnvironment`, `CreateEnvironmentAction`, `CreateEnvironmentBlueprint`, `CreateEnvironmentProfile`, `CreateGroupProfile`, `CreateListingChangeSet`, `CreateProject`, `CreateProjectMembership`, `CreateProjectProfile`, `CreateSubscriptionGrant`, `CreateSubscriptionRequest`, `CreateSubscriptionTarget`, `CreateUserProfile`
- Traits: `idempotent` (4), `idempotency-token` (8)
- Common required input members in this group: `domainIdentifier`, `name`, `projectIdentifier`, `environmentIdentifier`

### Delete

- Operations: `DeleteAccountPool`, `DeleteAssetFilter`, `DeleteConnection`, `DeleteDataExportConfiguration`, `DeleteEnvironment`, `DeleteEnvironmentAction`, `DeleteEnvironmentBlueprint`, `DeleteEnvironmentProfile`, `DeleteProject`, `DeleteProjectMembership`, `DeleteProjectProfile`, `DeleteSubscriptionGrant`, `DeleteSubscriptionRequest`, `DeleteSubscriptionTarget`, `DeleteTimeSeriesDataPoints`
- Traits: `idempotent` (12), `idempotency-token` (1)
- Common required input members in this group: `domainIdentifier`, `identifier`, `environmentIdentifier`

### Update

- Operations: `UpdateAccountPool`, `UpdateAssetFilter`, `UpdateConnection`, `UpdateEnvironment`, `UpdateEnvironmentAction`, `UpdateEnvironmentBlueprint`, `UpdateEnvironmentProfile`, `UpdateGroupProfile`, `UpdateProject`, `UpdateProjectProfile`, `UpdateRootDomainUnitOwner`, `UpdateSubscriptionGrantStatus`, `UpdateSubscriptionRequest`, `UpdateSubscriptionTarget`, `UpdateUserProfile`
- Traits: `idempotent` (10), `idempotency-token` (1)
- Common required input members in this group: `domainIdentifier`, `identifier`, `assetIdentifier`, `environmentIdentifier`, `status`

### Search

- Operations: `Search`, `SearchGroupProfiles`, `SearchListings`, `SearchTypes`, `SearchUserProfiles`
- Traits: `paginated` (5)
- Common required input members in this group: `domainIdentifier`, `searchScope`

### Accept

- Operations: `AcceptPredictions`, `AcceptSubscriptionRequest`
- Traits: `idempotent` (2), `idempotency-token` (1)
- Common required input members in this group: `domainIdentifier`, `identifier`

### Add

- Operations: `AddEntityOwner`, `AddPolicyGrant`
- Traits: `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: `domainIdentifier`, `entityType`, `entityIdentifier`

### Associate

- Operations: `AssociateEnvironmentRole`, `AssociateGovernedTerms`
- Common required input members in this group: `domainIdentifier`

### Batch

- Operations: `BatchGetAttributesMetadata`, `BatchPutAttributesMetadata`
- Traits: `readonly` (1), `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: `domainIdentifier`, `entityType`, `entityIdentifier`

### Disassociate

- Operations: `DisassociateEnvironmentRole`, `DisassociateGovernedTerms`
- Common required input members in this group: `domainIdentifier`

### Post

- Operations: `PostLineageEvent`, `PostTimeSeriesDataPoints`
- Traits: `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: `domainIdentifier`

### Reject

- Operations: `RejectPredictions`, `RejectSubscriptionRequest`
- Traits: `idempotent` (2), `idempotency-token` (1)
- Common required input members in this group: `domainIdentifier`, `identifier`

### Remove

- Operations: `RemoveEntityOwner`, `RemovePolicyGrant`
- Traits: `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: `domainIdentifier`, `entityType`, `entityIdentifier`

### Cancel

- Operations: `CancelSubscription`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Put

- Operations: `PutDataExportConfiguration`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Query

- Operations: `QueryGraph`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Revoke

- Operations: `RevokeSubscription`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptPredictions` | `PUT /v2/domains/{domainIdentifier}/assets/{identifier}/accept-predictions` | `idempotent`, `idempotency-token` | `domainIdentifier`, `identifier` | `clientToken` | `AcceptPredictionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Accepts automatically generated business-friendly metadata for your Amazon DataZone assets. |
| `AcceptSubscriptionRequest` | `PUT /v2/domains/{domainIdentifier}/subscription-requests/{identifier}/accept` | `idempotent` | `domainIdentifier`, `identifier` | - | `AcceptSubscriptionRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Accepts a subscription request to a specific asset. |
| `AddEntityOwner` | `POST /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/addOwner` | `idempotent`, `idempotency-token` | `domainIdentifier`, `entityType`, `entityIdentifier`, `owner` | `clientToken` | `AddEntityOwnerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds the owner of an entity (a domain unit). |
| `AddPolicyGrant` | `POST /v2/domains/{domainIdentifier}/policies/managed/{entityType}/{entityIdentifier}/addGrant` | `idempotent`, `idempotency-token` | `domainIdentifier`, `entityType`, `entityIdentifier`, `policyType`, `principal`, `detail` | `clientToken` | `AddPolicyGrantOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a policy grant (an authorization policy) to a specified entity, including domain units, environment blueprint configurations, or environment profiles. |
| `AssociateEnvironmentRole` | `PUT /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/roles/{environmentRoleArn}` | - | `domainIdentifier`, `environmentIdentifier`, `environmentRoleArn` | - | `AssociateEnvironmentRoleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates the environment role in Amazon DataZone. |
| `AssociateGovernedTerms` | `PATCH /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/associate-governed-terms` | - | `domainIdentifier`, `entityIdentifier`, `entityType`, `governedGlossaryTerms` | - | `AssociateGovernedTermsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates governed terms with an asset. |
| `BatchGetAttributesMetadata` | `GET /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/attributes-metadata` | `readonly` | `domainIdentifier`, `entityType`, `entityIdentifier`, `attributeIdentifiers` | - | `BatchGetAttributesMetadataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the attribute metadata. |
| `BatchPutAttributesMetadata` | `PUT /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/attributes-metadata` | `idempotent`, `idempotency-token` | `domainIdentifier`, `entityType`, `entityIdentifier`, `attributes` | `clientToken` | `BatchPutAttributesMetadataOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Writes the attribute metadata. |
| `CancelSubscription` | `PUT /v2/domains/{domainIdentifier}/subscriptions/{identifier}/cancel` | `idempotent` | `domainIdentifier`, `identifier` | - | `CancelSubscriptionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels the subscription to the specified asset. |
| `CreateAccountPool` | `POST /v2/domains/{domainIdentifier}/account-pools` | - | `domainIdentifier`, `name`, `resolutionStrategy`, `accountSource` | - | `CreateAccountPoolOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an account pool. |
| `CreateAssetFilter` | `POST /v2/domains/{domainIdentifier}/assets/{assetIdentifier}/filters` | `idempotent`, `idempotency-token` | `domainIdentifier`, `assetIdentifier`, `name`, `configuration` | `clientToken` | `CreateAssetFilterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a data asset filter. Asset filters provide a sophisticated way to create controlled views of data assets by selecting specific columns or applying row-level filters. This capability is crucial for organizatio ... |
| `CreateConnection` | `POST /v2/domains/{domainIdentifier}/connections` | `idempotent`, `idempotency-token` | `domainIdentifier`, `name` | `clientToken` | `CreateConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new connection. In Amazon DataZone, a connection enables you to connect your resources (domains, projects, and environments) to external resources and services. |
| `CreateEnvironment` | `POST /v2/domains/{domainIdentifier}/environments` | - | `projectIdentifier`, `domainIdentifier`, `name` | - | `CreateEnvironmentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Create an Amazon DataZone environment. |
| `CreateEnvironmentAction` | `POST /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/actions` | - | `domainIdentifier`, `environmentIdentifier`, `name`, `parameters` | - | `CreateEnvironmentActionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an action for the environment, for example, creates a console link for an analytics tool that is available in this environment. |
| `CreateEnvironmentBlueprint` | `POST /v2/domains/{domainIdentifier}/environment-blueprints` | - | `domainIdentifier`, `name`, `provisioningProperties` | - | `CreateEnvironmentBlueprintOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a Amazon DataZone blueprint. |
| `CreateEnvironmentProfile` | `POST /v2/domains/{domainIdentifier}/environment-profiles` | - | `domainIdentifier`, `name`, `environmentBlueprintIdentifier`, `projectIdentifier` | - | `CreateEnvironmentProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon DataZone environment profile. |
| `CreateGroupProfile` | `POST /v2/domains/{domainIdentifier}/group-profiles` | `idempotent`, `idempotency-token` | `domainIdentifier` | `clientToken` | `CreateGroupProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a group profile in Amazon DataZone. |
| `CreateListingChangeSet` | `POST /v2/domains/{domainIdentifier}/listings/change-set` | `idempotency-token` | `domainIdentifier`, `entityIdentifier`, `entityType`, `action` | `clientToken` | `CreateListingChangeSetOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Publishes a listing (a record of an asset at a given time) or removes a listing from the catalog. |
| `CreateProject` | `POST /v2/domains/{domainIdentifier}/projects` | - | `domainIdentifier`, `name` | - | `CreateProjectOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon DataZone project. |
| `CreateProjectMembership` | `POST /v2/domains/{domainIdentifier}/projects/{projectIdentifier}/createMembership` | - | `domainIdentifier`, `projectIdentifier`, `member`, `designation` | - | `CreateProjectMembershipOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a project membership in Amazon DataZone. |
| `CreateProjectProfile` | `POST /v2/domains/{domainIdentifier}/project-profiles` | - | `domainIdentifier`, `name` | - | `CreateProjectProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a project profile. |
| `CreateSubscriptionGrant` | `POST /v2/domains/{domainIdentifier}/subscription-grants` | `idempotency-token` | `domainIdentifier`, `environmentIdentifier`, `grantedEntity` | `clientToken` | `CreateSubscriptionGrantOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a subsscription grant in Amazon DataZone. |
| `CreateSubscriptionRequest` | `POST /v2/domains/{domainIdentifier}/subscription-requests` | `idempotency-token` | `domainIdentifier`, `subscribedPrincipals`, `subscribedListings`, `requestReason` | `clientToken` | `CreateSubscriptionRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a subscription request in Amazon DataZone. |
| `CreateSubscriptionTarget` | `POST /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/subscription-targets` | `idempotency-token` | `domainIdentifier`, `environmentIdentifier`, `name`, `type`, `subscriptionTargetConfig`, `authorizedPrincipals`, `manageAccessRole`, `applicableAssetTypes` | `clientToken` | `CreateSubscriptionTargetOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a subscription target in Amazon DataZone. |
| `CreateUserProfile` | `POST /v2/domains/{domainIdentifier}/user-profiles` | `idempotent`, `idempotency-token` | `domainIdentifier`, `userIdentifier` | `clientToken` | `CreateUserProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a user profile in Amazon DataZone. |
| `DeleteAccountPool` | `DELETE /v2/domains/{domainIdentifier}/account-pools/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `DeleteAccountPoolOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an account pool. |
| `DeleteAssetFilter` | `DELETE /v2/domains/{domainIdentifier}/assets/{assetIdentifier}/filters/{identifier}` | `idempotent` | `domainIdentifier`, `assetIdentifier`, `identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an asset filter. Prerequisites: The asset filter must exist. The domain and asset must not have been deleted. Ensure the --identifier refers to a valid filter ID. |
| `DeleteConnection` | `DELETE /v2/domains/{domainIdentifier}/connections/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `DeleteConnectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes and connection. In Amazon DataZone, a connection enables you to connect your resources (domains, projects, and environments) to external resources and services. |
| `DeleteDataExportConfiguration` | `DELETE /v2/domains/{domainIdentifier}/data-export-configuration` | `idempotent` | `domainIdentifier` | - | `DeleteDataExportConfigurationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes data export configuration for a domain. This operation does not delete the S3 table created by the PutDataExportConfiguration operation. To temporarily disable export without deleting the configuration, use t ... |
| `DeleteEnvironment` | `DELETE /v2/domains/{domainIdentifier}/environments/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an environment in Amazon DataZone. |
| `DeleteEnvironmentAction` | `DELETE /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/actions/{identifier}` | `idempotent` | `domainIdentifier`, `environmentIdentifier`, `identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an action for the environment, for example, deletes a console link for an analytics tool that is available in this environment. |
| `DeleteEnvironmentBlueprint` | `DELETE /v2/domains/{domainIdentifier}/environment-blueprints/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a blueprint in Amazon DataZone. |
| `DeleteEnvironmentProfile` | `DELETE /v2/domains/{domainIdentifier}/environment-profiles/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an environment profile in Amazon DataZone. |
| `DeleteProject` | `DELETE /v2/domains/{domainIdentifier}/projects/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `DeleteProjectOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a project in Amazon DataZone. |
| `DeleteProjectMembership` | `POST /v2/domains/{domainIdentifier}/projects/{projectIdentifier}/deleteMembership` | `idempotent` | `domainIdentifier`, `projectIdentifier`, `member` | - | `DeleteProjectMembershipOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes project membership in Amazon DataZone. |
| `DeleteProjectProfile` | `DELETE /v2/domains/{domainIdentifier}/project-profiles/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `DeleteProjectProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a project profile. |
| `DeleteSubscriptionGrant` | `DELETE /v2/domains/{domainIdentifier}/subscription-grants/{identifier}` | - | `domainIdentifier`, `identifier` | - | `DeleteSubscriptionGrantOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes and subscription grant in Amazon DataZone. |
| `DeleteSubscriptionRequest` | `DELETE /v2/domains/{domainIdentifier}/subscription-requests/{identifier}` | - | `domainIdentifier`, `identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a subscription request in Amazon DataZone. |
| `DeleteSubscriptionTarget` | `DELETE /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/subscription-targets/{identifier}` | - | `domainIdentifier`, `environmentIdentifier`, `identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a subscription target in Amazon DataZone. |
| `DeleteTimeSeriesDataPoints` | `DELETE /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/time-series-data-points` | `idempotent`, `idempotency-token` | `domainIdentifier`, `entityIdentifier`, `entityType`, `formName` | `clientToken` | `DeleteTimeSeriesDataPointsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified time series form for the specified asset. |
| `DisassociateEnvironmentRole` | `DELETE /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/roles/{environmentRoleArn}` | - | `domainIdentifier`, `environmentIdentifier`, `environmentRoleArn` | - | `DisassociateEnvironmentRoleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates the environment role in Amazon DataZone. |
| `DisassociateGovernedTerms` | `PATCH /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/disassociate-governed-terms` | - | `domainIdentifier`, `entityIdentifier`, `entityType`, `governedGlossaryTerms` | - | `DisassociateGovernedTermsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates restricted terms from an asset. |
| `GetAccountPool` | `GET /v2/domains/{domainIdentifier}/account-pools/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetAccountPoolOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of the account pool. |
| `GetAssetFilter` | `GET /v2/domains/{domainIdentifier}/assets/{assetIdentifier}/filters/{identifier}` | `readonly` | `domainIdentifier`, `assetIdentifier`, `identifier` | - | `GetAssetFilterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an asset filter. Prerequisites: Domain ( --domain-identifier ), asset ( --asset-identifier ), and filter ( --identifier ) must all exist. The asset filter should not have been deleted. The asset must still exist ... |
| `GetConnection` | `GET /v2/domains/{domainIdentifier}/connections/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetConnectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a connection. In Amazon DataZone, a connection enables you to connect your resources (domains, projects, and environments) to external resources and services. |
| `GetDataExportConfiguration` | `GET /v2/domains/{domainIdentifier}/data-export-configuration` | `readonly` | `domainIdentifier` | - | `GetDataExportConfigurationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets data export configuration details. |
| `GetEnvironment` | `GET /v2/domains/{domainIdentifier}/environments/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetEnvironmentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon DataZone environment. |
| `GetEnvironmentAction` | `GET /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/actions/{identifier}` | `readonly` | `domainIdentifier`, `environmentIdentifier`, `identifier` | - | `GetEnvironmentActionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the specified environment action. |
| `GetEnvironmentBlueprint` | `GET /v2/domains/{domainIdentifier}/environment-blueprints/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetEnvironmentBlueprintOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon DataZone blueprint. |
| `GetEnvironmentCredentials` | `GET /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/credentials` | `readonly` | `domainIdentifier`, `environmentIdentifier` | - | `GetEnvironmentCredentialsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the credentials of an environment in Amazon DataZone. |
| `GetEnvironmentProfile` | `GET /v2/domains/{domainIdentifier}/environment-profiles/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetEnvironmentProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an evinronment profile in Amazon DataZone. |
| `GetGroupProfile` | `GET /v2/domains/{domainIdentifier}/group-profiles/{groupIdentifier}` | `readonly` | `domainIdentifier`, `groupIdentifier` | - | `GetGroupProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets a group profile in Amazon DataZone. |
| `GetIamPortalLoginUrl` | `POST /v2/domains/{domainIdentifier}/get-portal-login-url` | - | `domainIdentifier` | - | `GetIamPortalLoginUrlOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the data portal URL for the specified Amazon DataZone domain. |
| `GetJobRun` | `GET /v2/domains/{domainIdentifier}/jobRuns/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetJobRunOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The details of the job run. |
| `GetLineageEvent` | `GET /v2/domains/{domainIdentifier}/lineage/events/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetLineageEventOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the lineage event. |
| `GetLineageNode` | `GET /v2/domains/{domainIdentifier}/lineage/nodes/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetLineageNodeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the data lineage node. |
| `GetProject` | `GET /v2/domains/{domainIdentifier}/projects/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetProjectOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a project in Amazon DataZone. |
| `GetProjectProfile` | `GET /v2/domains/{domainIdentifier}/project-profiles/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetProjectProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The details of the project profile. |
| `GetSubscription` | `GET /v2/domains/{domainIdentifier}/subscriptions/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetSubscriptionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a subscription in Amazon DataZone. |
| `GetSubscriptionGrant` | `GET /v2/domains/{domainIdentifier}/subscription-grants/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetSubscriptionGrantOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the subscription grant in Amazon DataZone. |
| `GetSubscriptionRequestDetails` | `GET /v2/domains/{domainIdentifier}/subscription-requests/{identifier}` | `readonly` | `domainIdentifier`, `identifier` | - | `GetSubscriptionRequestDetailsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of the specified subscription request. |
| `GetSubscriptionTarget` | `GET /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/subscription-targets/{identifier}` | `readonly` | `domainIdentifier`, `environmentIdentifier`, `identifier` | - | `GetSubscriptionTargetOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the subscription target in Amazon DataZone. |
| `GetTimeSeriesDataPoint` | `GET /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/time-series-data-points/{identifier}` | `readonly` | `domainIdentifier`, `entityIdentifier`, `entityType`, `identifier`, `formName` | - | `GetTimeSeriesDataPointOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the existing data point for the asset. |
| `GetUserProfile` | `GET /v2/domains/{domainIdentifier}/user-profiles/{userIdentifier}` | `readonly` | `domainIdentifier`, `userIdentifier` | - | `GetUserProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets a user profile in Amazon DataZone. |
| `ListAccountPools` | `GET /v2/domains/{domainIdentifier}/account-pools` | `readonly`, `paginated` | `domainIdentifier` | - | `ListAccountPoolsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists existing account pools. |
| `ListAccountsInAccountPool` | `GET /v2/domains/{domainIdentifier}/account-pools/{identifier}/accounts` | `readonly`, `paginated` | `domainIdentifier`, `identifier` | - | `ListAccountsInAccountPoolOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the accounts in the specified account pool. |
| `ListAssetFilters` | `GET /v2/domains/{domainIdentifier}/assets/{assetIdentifier}/filters` | `readonly`, `paginated` | `domainIdentifier`, `assetIdentifier` | - | `ListAssetFiltersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists asset filters. Prerequisites: A valid domain and asset must exist. The asset must have at least one filter created to return results. |
| `ListAssetRevisions` | `GET /v2/domains/{domainIdentifier}/assets/{identifier}/revisions` | `readonly`, `paginated` | `domainIdentifier`, `identifier` | - | `ListAssetRevisionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the revisions for the asset. Prerequisites: The asset must exist in the domain. There must be at least one revision of the asset (which happens automatically after creation). The domain must be valid and active ... |
| `ListConnections` | `GET /v2/domains/{domainIdentifier}/connections` | `readonly`, `paginated` | `domainIdentifier` | - | `ListConnectionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists connections. In Amazon DataZone, a connection enables you to connect your resources (domains, projects, and environments) to external resources and services. |
| `ListDataProductRevisions` | `GET /v2/domains/{domainIdentifier}/data-products/{identifier}/revisions` | `readonly`, `paginated` | `domainIdentifier`, `identifier` | - | `ListDataProductRevisionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists data product revisions. Prerequisites: The data product ID must exist within the domain. User must have view permissions on the data product. The domain must be in a valid and accessible state. |
| `ListDataSourceRunActivities` | `GET /v2/domains/{domainIdentifier}/data-source-runs/{identifier}/activities` | `readonly`, `paginated` | `domainIdentifier`, `identifier` | - | `ListDataSourceRunActivitiesOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists data source run activities. |
| `ListEntityOwners` | `GET /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/owners` | `readonly`, `paginated` | `domainIdentifier`, `entityType`, `entityIdentifier` | - | `ListEntityOwnersOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the entity (domain units) owners. |
| `ListEnvironmentActions` | `GET /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/actions` | `readonly`, `paginated` | `domainIdentifier`, `environmentIdentifier` | - | `ListEnvironmentActionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists existing environment actions. |
| `ListEnvironmentBlueprints` | `GET /v2/domains/{domainIdentifier}/environment-blueprints` | `readonly`, `paginated` | `domainIdentifier` | - | `ListEnvironmentBlueprintsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists blueprints in an Amazon DataZone environment. |
| `ListEnvironmentProfiles` | `GET /v2/domains/{domainIdentifier}/environment-profiles` | `readonly`, `paginated` | `domainIdentifier` | - | `ListEnvironmentProfilesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists Amazon DataZone environment profiles. |
| `ListEnvironments` | `GET /v2/domains/{domainIdentifier}/environments` | `readonly`, `paginated` | `domainIdentifier`, `projectIdentifier` | - | `ListEnvironmentsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists Amazon DataZone environments. |
| `ListJobRuns` | `GET /v2/domains/{domainIdentifier}/jobs/{jobIdentifier}/runs` | `readonly`, `paginated` | `domainIdentifier`, `jobIdentifier` | - | `ListJobRunsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists job runs. |
| `ListLineageEvents` | `GET /v2/domains/{domainIdentifier}/lineage/events` | `readonly`, `paginated` | `domainIdentifier` | - | `ListLineageEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists lineage events. |
| `ListLineageNodeHistory` | `GET /v2/domains/{domainIdentifier}/lineage/nodes/{identifier}/history` | `readonly`, `paginated` | `domainIdentifier`, `identifier` | - | `ListLineageNodeHistoryOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the history of the specified data lineage node. |
| `ListNotifications` | `GET /v2/domains/{domainIdentifier}/notifications` | `readonly`, `paginated` | `domainIdentifier`, `type` | - | `ListNotificationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all Amazon DataZone notifications. |
| `ListPolicyGrants` | `GET /v2/domains/{domainIdentifier}/policies/managed/{entityType}/{entityIdentifier}/grants` | `readonly`, `paginated` | `domainIdentifier`, `entityType`, `entityIdentifier`, `policyType` | - | `ListPolicyGrantsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists policy grants. |
| `ListProjectMemberships` | `GET /v2/domains/{domainIdentifier}/projects/{projectIdentifier}/memberships` | `readonly`, `paginated` | `domainIdentifier`, `projectIdentifier` | - | `ListProjectMembershipsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all members of the specified project. |
| `ListProjectProfiles` | `GET /v2/domains/{domainIdentifier}/project-profiles` | `readonly`, `paginated` | `domainIdentifier` | - | `ListProjectProfilesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists project profiles. |
| `ListProjects` | `GET /v2/domains/{domainIdentifier}/projects` | `readonly`, `paginated` | `domainIdentifier` | - | `ListProjectsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists Amazon DataZone projects. |
| `ListSubscriptionGrants` | `GET /v2/domains/{domainIdentifier}/subscription-grants` | `readonly`, `paginated` | `domainIdentifier` | - | `ListSubscriptionGrantsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists subscription grants. |
| `ListSubscriptionRequests` | `GET /v2/domains/{domainIdentifier}/subscription-requests` | `readonly`, `paginated` | `domainIdentifier` | - | `ListSubscriptionRequestsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists Amazon DataZone subscription requests. |
| `ListSubscriptions` | `GET /v2/domains/{domainIdentifier}/subscriptions` | `readonly`, `paginated` | `domainIdentifier` | - | `ListSubscriptionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists subscriptions in Amazon DataZone. |
| `ListSubscriptionTargets` | `GET /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/subscription-targets` | `readonly`, `paginated` | `domainIdentifier`, `environmentIdentifier` | - | `ListSubscriptionTargetsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists subscription targets in Amazon DataZone. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists tags for the specified resource in Amazon DataZone. |
| `ListTimeSeriesDataPoints` | `GET /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/time-series-data-points` | `readonly`, `paginated` | `domainIdentifier`, `entityIdentifier`, `entityType`, `formName` | - | `ListTimeSeriesDataPointsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists time series data points. |
| `PostLineageEvent` | `POST /v2/domains/{domainIdentifier}/lineage/events` | `idempotent`, `idempotency-token` | `domainIdentifier`, `event` | `clientToken` | `PostLineageEventOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Posts a data lineage event. |
| `PostTimeSeriesDataPoints` | `POST /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/time-series-data-points` | `idempotent`, `idempotency-token` | `domainIdentifier`, `entityIdentifier`, `entityType`, `forms` | `clientToken` | `PostTimeSeriesDataPointsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Posts time series data points to Amazon DataZone for the specified asset. |
| `PutDataExportConfiguration` | `PUT /v2/domains/{domainIdentifier}/data-export-configuration` | `idempotent`, `idempotency-token` | `domainIdentifier`, `enableExport` | `clientToken` | `PutDataExportConfigurationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates data export configuration details. If you want to temporarily disable export and later re-enable it for the same domain, use the --no-enable-export flag to disable and the --enable-export flag to re-enable. T ... |
| `QueryGraph` | `POST /v2/domains/{domainIdentifier}/graph/query` | `paginated` | `domainIdentifier`, `match` | - | `QueryGraphOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Queries entities in the graph store. |
| `RejectPredictions` | `PUT /v2/domains/{domainIdentifier}/assets/{identifier}/reject-predictions` | `idempotent`, `idempotency-token` | `domainIdentifier`, `identifier` | `clientToken` | `RejectPredictionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rejects automatically generated business-friendly metadata for your Amazon DataZone assets. |
| `RejectSubscriptionRequest` | `PUT /v2/domains/{domainIdentifier}/subscription-requests/{identifier}/reject` | `idempotent` | `domainIdentifier`, `identifier` | - | `RejectSubscriptionRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rejects the specified subscription request. |
| `RemoveEntityOwner` | `POST /v2/domains/{domainIdentifier}/entities/{entityType}/{entityIdentifier}/removeOwner` | `idempotent`, `idempotency-token` | `domainIdentifier`, `entityType`, `entityIdentifier`, `owner` | `clientToken` | `RemoveEntityOwnerOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes an owner from an entity. |
| `RemovePolicyGrant` | `POST /v2/domains/{domainIdentifier}/policies/managed/{entityType}/{entityIdentifier}/removeGrant` | `idempotent`, `idempotency-token` | `domainIdentifier`, `entityType`, `entityIdentifier`, `policyType`, `principal` | `clientToken` | `RemovePolicyGrantOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Removes a policy grant. |
| `RevokeSubscription` | `PUT /v2/domains/{domainIdentifier}/subscriptions/{identifier}/revoke` | `idempotent` | `domainIdentifier`, `identifier` | - | `RevokeSubscriptionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Revokes a specified subscription in Amazon DataZone. |
| `Search` | `POST /v2/domains/{domainIdentifier}/search` | `paginated` | `domainIdentifier`, `searchScope` | - | `SearchOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Searches for assets in Amazon DataZone. Search in Amazon DataZone is a powerful capability that enables users to discover and explore data assets, glossary terms, and data products across their organization. It provi ... |
| `SearchGroupProfiles` | `POST /v2/domains/{domainIdentifier}/search-group-profiles` | `paginated` | `domainIdentifier`, `groupType` | - | `SearchGroupProfilesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Searches group profiles in Amazon DataZone. |
| `SearchListings` | `POST /v2/domains/{domainIdentifier}/listings/search` | `paginated` | `domainIdentifier` | - | `SearchListingsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Searches listings in Amazon DataZone. SearchListings is a powerful capability that enables users to discover and explore published assets and data products across their organization. It provides both basic and advanc ... |
| `SearchTypes` | `POST /v2/domains/{domainIdentifier}/types-search` | `paginated` | `domainIdentifier`, `searchScope`, `managed` | - | `SearchTypesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Searches for types in Amazon DataZone. Prerequisites: The --domain-identifier must refer to an existing Amazon DataZone domain. --search-scope must be one of the valid values including: ASSET_TYPE, GLOSSARY_TERM_TYPE ... |
| `SearchUserProfiles` | `POST /v2/domains/{domainIdentifier}/search-user-profiles` | `paginated` | `domainIdentifier`, `userType` | - | `SearchUserProfilesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Searches user profiles in Amazon DataZone. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Tags a resource in Amazon DataZone. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException` | Untags a resource in Amazon DataZone. |
| `UpdateAccountPool` | `PATCH /v2/domains/{domainIdentifier}/account-pools/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `UpdateAccountPoolOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the account pool. |
| `UpdateAssetFilter` | `PATCH /v2/domains/{domainIdentifier}/assets/{assetIdentifier}/filters/{identifier}` | `idempotent` | `domainIdentifier`, `assetIdentifier`, `identifier` | - | `UpdateAssetFilterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an asset filter. Prerequisites: The domain, asset, and asset filter identifier must all exist. The asset must contain the columns being referenced in the update. If applying a row filter, ensure the column re ... |
| `UpdateConnection` | `PATCH /v2/domains/{domainIdentifier}/connections/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `UpdateConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a connection. In Amazon DataZone, a connection enables you to connect your resources (domains, projects, and environments) to external resources and services. |
| `UpdateEnvironment` | `PATCH /v2/domains/{domainIdentifier}/environments/{identifier}` | - | `domainIdentifier`, `identifier` | - | `UpdateEnvironmentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the specified environment in Amazon DataZone. |
| `UpdateEnvironmentAction` | `PATCH /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/actions/{identifier}` | - | `domainIdentifier`, `environmentIdentifier`, `identifier` | - | `UpdateEnvironmentActionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an environment action. |
| `UpdateEnvironmentBlueprint` | `PATCH /v2/domains/{domainIdentifier}/environment-blueprints/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `UpdateEnvironmentBlueprintOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an environment blueprint in Amazon DataZone. |
| `UpdateEnvironmentProfile` | `PATCH /v2/domains/{domainIdentifier}/environment-profiles/{identifier}` | - | `domainIdentifier`, `identifier` | - | `UpdateEnvironmentProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the specified environment profile in Amazon DataZone. |
| `UpdateGroupProfile` | `PUT /v2/domains/{domainIdentifier}/group-profiles/{groupIdentifier}` | - | `domainIdentifier`, `groupIdentifier`, `status` | - | `UpdateGroupProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the specified group profile in Amazon DataZone. |
| `UpdateProject` | `PATCH /v2/domains/{domainIdentifier}/projects/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `UpdateProjectOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the specified project in Amazon DataZone. |
| `UpdateProjectProfile` | `PATCH /v2/domains/{domainIdentifier}/project-profiles/{identifier}` | `idempotent` | `domainIdentifier`, `identifier` | - | `UpdateProjectProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a project profile. |
| `UpdateRootDomainUnitOwner` | `PATCH /v2/domains/{domainIdentifier}/root-domain-unit-owner` | `idempotent`, `idempotency-token` | `domainIdentifier`, `currentOwner`, `newOwner` | `clientToken` | `UpdateRootDomainUnitOwnerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the owner of the root domain unit. |
| `UpdateSubscriptionGrantStatus` | `PATCH /v2/domains/{domainIdentifier}/subscription-grants/{identifier}/status/{assetIdentifier}` | `idempotent` | `domainIdentifier`, `identifier`, `assetIdentifier`, `status` | - | `UpdateSubscriptionGrantStatusOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the status of the specified subscription grant status in Amazon DataZone. |
| `UpdateSubscriptionRequest` | `PATCH /v2/domains/{domainIdentifier}/subscription-requests/{identifier}` | `idempotent` | `domainIdentifier`, `identifier`, `requestReason` | - | `UpdateSubscriptionRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a specified subscription request in Amazon DataZone. |
| `UpdateSubscriptionTarget` | `PATCH /v2/domains/{domainIdentifier}/environments/{environmentIdentifier}/subscription-targets/{identifier}` | `idempotent` | `domainIdentifier`, `environmentIdentifier`, `identifier` | - | `UpdateSubscriptionTargetOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified subscription target in Amazon DataZone. |
| `UpdateUserProfile` | `PUT /v2/domains/{domainIdentifier}/user-profiles/{userIdentifier}` | - | `domainIdentifier`, `userIdentifier`, `status` | - | `UpdateUserProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the specified user profile in Amazon DataZone. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AcceptPredictions` | - | `revision -> revision` | - | - |
| `BatchGetAttributesMetadata` | - | `entityRevision -> entityRevision`, `attributeIdentifiers -> attributeIdentifier` | - | - |
| `DeleteProject` | - | `skipDeletionCheck -> skipDeletionCheck` | - | - |
| `DeleteTimeSeriesDataPoints` | - | `formName -> formName`, `clientToken -> clientToken` | - | - |
| `GetConnection` | - | `withSecret -> withSecret` | - | - |
| `GetLineageNode` | - | `eventTimestamp -> timestamp` | - | - |
| `GetTimeSeriesDataPoint` | - | `formName -> formName` | - | - |
| `GetUserProfile` | - | `type -> type`, `sessionName -> sessionName` | - | - |
| `ListAccountPools` | - | `name -> name`, `sortBy -> sortBy`, `sortOrder -> sortOrder`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAccountsInAccountPool` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssetFilters` | - | `status -> status`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssetRevisions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListConnections` | - | `maxResults -> maxResults`, `nextToken -> nextToken`, `sortBy -> sortBy`, `sortOrder -> sortOrder`, `name -> name`, `environmentIdentifier -> environmentIdentifier`, `projectIdentifier -> projectIdentifier`, `type -> type`, `scope -> scope` | - | - |
| `ListDataProductRevisions` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListDataSourceRunActivities` | - | `status -> status`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListEntityOwners` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListEnvironmentActions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListEnvironmentBlueprints` | - | `maxResults -> maxResults`, `nextToken -> nextToken`, `name -> name`, `managed -> managed` | - | - |
| `ListEnvironmentProfiles` | - | `awsAccountId -> awsAccountId`, `awsAccountRegion -> awsAccountRegion`, `environmentBlueprintIdentifier -> environmentBlueprintIdentifier`, `projectIdentifier -> projectIdentifier`, `name -> name`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListEnvironments` | - | `awsAccountId -> awsAccountId`, `status -> status`, `awsAccountRegion -> awsAccountRegion`, `projectIdentifier -> projectIdentifier`, `environmentProfileIdentifier -> environmentProfileIdentifier`, `environmentBlueprintIdentifier -> environmentBlueprintIdentifier`, `provider -> provider`, `name -> name`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListJobRuns` | - | `status -> status`, `sortOrder -> sortOrder`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListLineageEvents` | - | `maxResults -> maxResults`, `timestampAfter -> timestampAfter`, `timestampBefore -> timestampBefore`, `processingStatus -> processingStatus`, `sortOrder -> sortOrder`, `nextToken -> nextToken` | - | - |
| `ListLineageNodeHistory` | - | `maxResults -> maxResults`, `nextToken -> nextToken`, `direction -> direction`, `eventTimestampGTE -> timestampGTE`, `eventTimestampLTE -> timestampLTE`, `sortOrder -> sortOrder` | - | - |
| `ListNotifications` | - | `type -> type`, `afterTimestamp -> afterTimestamp`, `beforeTimestamp -> beforeTimestamp`, `subjects -> subjects`, `taskStatus -> taskStatus`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListPolicyGrants` | - | `policyType -> policyType`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListProjectMemberships` | - | `sortBy -> sortBy`, `sortOrder -> sortOrder`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListProjectProfiles` | - | `name -> name`, `sortBy -> sortBy`, `sortOrder -> sortOrder`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListProjects` | - | `userIdentifier -> userIdentifier`, `groupIdentifier -> groupIdentifier`, `name -> name`, `projectCategory -> projectCategory`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListSubscriptionGrants` | - | `environmentId -> environmentId`, `subscriptionTargetId -> subscriptionTargetId`, `subscribedListingId -> subscribedListingId`, `subscriptionId -> subscriptionId`, `owningProjectId -> owningProjectId`, `owningIamPrincipalArn -> owningIamPrincipalArn`, `owningUserId -> owningUserId`, `owningGroupId -> owningGroupId`, `sortBy -> sortBy`, `sortOrder -> sortOrder`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListSubscriptionRequests` | - | `status -> status`, `subscribedListingId -> subscribedListingId`, `owningProjectId -> owningProjectId`, `owningIamPrincipalArn -> owningIamPrincipalArn`, `approverProjectId -> approverProjectId`, `owningUserId -> owningUserId`, `owningGroupId -> owningGroupId`, `sortBy -> sortBy`, `sortOrder -> sortOrder`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListSubscriptions` | - | `subscriptionRequestIdentifier -> subscriptionRequestIdentifier`, `status -> status`, `subscribedListingId -> subscribedListingId`, `owningProjectId -> owningProjectId`, `owningIamPrincipalArn -> owningIamPrincipalArn`, `owningUserId -> owningUserId`, `owningGroupId -> owningGroupId`, `approverProjectId -> approverProjectId`, `sortBy -> sortBy`, `sortOrder -> sortOrder`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListSubscriptionTargets` | - | `sortBy -> sortBy`, `sortOrder -> sortOrder`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListTimeSeriesDataPoints` | - | `formName -> formName`, `startedAt -> startedAt`, `endedAt -> endedAt`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `PostLineageEvent` | `clientToken -> Client-Token` | - | - | `event` |
| `QueryGraph` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `RejectPredictions` | - | `revision -> revision` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | There is a conflict while performing this action. |
| `InternalServerException` | `structure` | message | The request has failed because of an unknown error, exception or failure. |
| `ResourceNotFoundException` | `structure` | message | The specified resource cannot be found. |
| `ServiceQuotaExceededException` | `structure` | message | The request has exceeded the specified service quota. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `UnauthorizedException` | `structure` | message | You do not have permission to perform this action. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by the Amazon Web Services service. |
| `AcceptPredictionsInput` | `structure` | domainIdentifier, identifier, revision, acceptRule, acceptChoices, clientToken | - |
| `AcceptPredictionsOutput` | `structure` | domainId, assetId, revision | - |
| `AcceptSubscriptionRequestInput` | `structure` | domainIdentifier, identifier, decisionComment, assetScopes, assetPermissions | - |
| `AcceptSubscriptionRequestOutput` | `structure` | id, createdBy, updatedBy, domainId, status, createdAt, updatedAt, requestReason, subscribedPrincipals, subscribedListings, reviewerId, decisionComment, ... (+2) | - |
| `AddEntityOwnerInput` | `structure` | domainIdentifier, entityType, entityIdentifier, owner, clientToken | - |
| `AddEntityOwnerOutput` | `structure` | **empty (no members)** | - |
| `AddPolicyGrantInput` | `structure` | domainIdentifier, entityType, entityIdentifier, policyType, principal, detail, clientToken | - |
| `AddPolicyGrantOutput` | `structure` | grantId | - |
| `AssociateEnvironmentRoleInput` | `structure` | domainIdentifier, environmentIdentifier, environmentRoleArn | - |
| `AssociateEnvironmentRoleOutput` | `structure` | **empty (no members)** | - |
| `AssociateGovernedTermsInput` | `structure` | domainIdentifier, entityIdentifier, entityType, governedGlossaryTerms | - |
| `AssociateGovernedTermsOutput` | `structure` | **empty (no members)** | - |
| `BatchGetAttributesMetadataInput` | `structure` | domainIdentifier, entityType, entityIdentifier, entityRevision, attributeIdentifiers | - |
| `BatchGetAttributesMetadataOutput` | `structure` | attributes, errors | - |
| `BatchPutAttributesMetadataInput` | `structure` | domainIdentifier, entityType, entityIdentifier, clientToken, attributes | - |
| `BatchPutAttributesMetadataOutput` | `structure` | errors, attributes | - |
| `CancelSubscriptionInput` | `structure` | domainIdentifier, identifier | - |
| `CancelSubscriptionOutput` | `structure` | id, createdBy, updatedBy, domainId, status, createdAt, updatedAt, subscribedPrincipal, subscribedListing, subscriptionRequestId, retainPermissions | - |
| `CreateAccountPoolInput` | `structure` | domainIdentifier, name, description, resolutionStrategy, accountSource | - |
| `CreateAccountPoolOutput` | `structure` | domainId, name, id, description, resolutionStrategy, accountSource, createdBy, createdAt, lastUpdatedAt, updatedBy, domainUnitId | - |
| `CreateAssetFilterInput` | `structure` | domainIdentifier, assetIdentifier, name, description, configuration, clientToken | - |
| `CreateAssetFilterOutput` | `structure` | id, domainId, assetId, name, description, status, configuration, createdAt, errorMessage, effectiveColumnNames, effectiveRowFilter | - |
| `CreateConnectionInput` | `structure` | awsLocation, clientToken, configurations, description, domainIdentifier, environmentIdentifier, name, props, enableTrustedIdentityPropagation, scope | - |
| `CreateConnectionOutput` | `structure` | connectionId, configurations, description, domainId, domainUnitId, environmentId, name, physicalEndpoints, projectId, props, type, scope | - |
| `CreateEnvironmentInput` | `structure` | projectIdentifier, domainIdentifier, description, name, environmentProfileIdentifier, userParameters, glossaryTerms, environmentAccountIdentifier, environmentAccountRegion, environmentBlueprintIdentifier, deploymentOrder, environmentConfigurationId, ... (+1) | - |
| `CreateEnvironmentOutput` | `structure` | projectId, id, domainId, createdBy, createdAt, updatedAt, name, description, environmentProfileId, awsAccountId, awsAccountRegion, provider, ... (+11) | - |
| `CreateEnvironmentActionInput` | `structure` | domainIdentifier, environmentIdentifier, name, parameters, description | - |
| `CreateEnvironmentActionOutput` | `structure` | domainId, environmentId, id, name, parameters, description | - |
| `CreateEnvironmentBlueprintInput` | `structure` | domainIdentifier, name, description, provisioningProperties, userParameters | - |
| `CreateEnvironmentBlueprintOutput` | `structure` | id, name, description, provider, provisioningProperties, deploymentProperties, userParameters, glossaryTerms, createdAt, updatedAt | - |
| `CreateEnvironmentProfileInput` | `structure` | domainIdentifier, name, description, environmentBlueprintIdentifier, projectIdentifier, userParameters, awsAccountId, awsAccountRegion | - |
| `CreateEnvironmentProfileOutput` | `structure` | id, domainId, awsAccountId, awsAccountRegion, createdBy, createdAt, updatedAt, name, description, environmentBlueprintId, projectId, userParameters | - |
| `AcceptRuleBehavior` | `enum` | ALL, NONE | - |
| `AttributeEntityType` | `enum` | ASSET, LISTING | - |
| `AuthType` | `enum` | IAM_IDC, DISABLED | - |
| `AuthenticationType` | `enum` | BASIC, OAUTH2, CUSTOM | - |
| `ChangeAction` | `enum` | PUBLISH, UNPUBLISH | - |
| `ComputeEnvironments` | `enum` | SPARK, ATHENA, PYTHON | - |
| `ConfigurableActionTypeAuthorization` | `enum` | IAM, HTTPS | - |
| `ConfigurationStatus` | `enum` | COMPLETED, FAILED | - |
| `ConnectionScope` | `enum` | DOMAIN, PROJECT | - |
| `ConnectionStatus` | `enum` | CREATING, CREATE_FAILED, DELETING, DELETE_FAILED, READY, UPDATING, UPDATE_FAILED, DELETED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
