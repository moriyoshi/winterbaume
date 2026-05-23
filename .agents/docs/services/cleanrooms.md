# AWS Clean Rooms Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Clean Rooms API Reference . Clean Rooms is an Amazon Web Services service that helps multiple parties to join their data together in a secure collaboration workspace. In the collaboration, members who can run queries and jobs and receive results can get insights into the collective datasets without either party getting access to the other party's raw data. To learn more about Clean Rooms concepts, procedures, and best practices, see the Clean Rooms User Guide. To learn more about SQL commands, functions, and conditions supported in Clean Rooms, see the Clean Rooms SQL Reference.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Clean Rooms Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Clean Rooms Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Clean Rooms Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Clean Rooms Service workflows in the local mock. Key resources include `AnalysisTemplateResource`, `CollaborationResource`, `ConfiguredAudienceModelAssociationResource`, `ConfiguredTableAssociationResource`, `ConfiguredTableResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Create`, `Delete` operation families, including `ListAnalysisTemplates`, `ListCollaborationAnalysisTemplates`, `ListCollaborationChangeRequests`, `ListCollaborationConfiguredAudienceModelAssociations`, `GetAnalysisTemplate`, `GetCollaboration`.

## Service Identity and Protocol

- AWS model slug: `cleanrooms`
- AWS SDK for Rust slug: `cleanrooms`
- Model version: `2022-02-17`
- Model file: `vendor/api-models-aws/models/cleanrooms/service/2022-02-17/cleanrooms-2022-02-17.json`
- SDK ID: `CleanRooms`
- Endpoint prefix: `-`
- ARN namespace: `cleanrooms`
- CloudFormation name: `-`
- CloudTrail event source: `cleanrooms.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (21), `Get` (20), `Update` (14), `Create` (12), `Delete` (12), `Batch` (3), `Start` (2), `Populate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetCollaborationAnalysisTemplate`, `BatchGetSchema`, `BatchGetSchemaAnalysisRule`, `CreateAnalysisTemplate`, `CreateCollaboration`, `CreateCollaborationChangeRequest`, `CreateConfiguredAudienceModelAssociation`, `CreateConfiguredTable`, `CreateConfiguredTableAnalysisRule`, `CreateConfiguredTableAssociation`, `CreateConfiguredTableAssociationAnalysisRule`, `CreateIdMappingTable`, `CreateIdNamespaceAssociation`, `CreateMembership`, `CreatePrivacyBudgetTemplate`, `DeleteAnalysisTemplate`, `DeleteCollaboration`, `DeleteConfiguredAudienceModelAssociation`, `DeleteConfiguredTable`, `DeleteConfiguredTableAnalysisRule`, ... (+25).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetCollaborationAnalysisTemplate`, `BatchGetSchema`, `BatchGetSchemaAnalysisRule`, `GetAnalysisTemplate`, `GetCollaboration`, `GetCollaborationAnalysisTemplate`, `GetCollaborationChangeRequest`, `GetCollaborationConfiguredAudienceModelAssociation`, `GetCollaborationIdNamespaceAssociation`, `GetCollaborationPrivacyBudgetTemplate`, `GetConfiguredAudienceModelAssociation`, `GetConfiguredTable`, `GetConfiguredTableAnalysisRule`, `GetConfiguredTableAssociation`, `GetConfiguredTableAssociationAnalysisRule`, `GetIdMappingTable`, `GetIdNamespaceAssociation`, `GetMembership`, `GetPrivacyBudgetTemplate`, `GetProtectedJob`, ... (+24).
- Pagination is modelled for 20 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 17 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchGetCollaborationAnalysisTemplate`, `BatchGetSchemaAnalysisRule`, `CreateAnalysisTemplate`, `CreateConfiguredTableAnalysisRule`, `CreateConfiguredTableAssociationAnalysisRule`, `DeleteAnalysisTemplate`, `DeleteConfiguredTableAnalysisRule`, `DeleteConfiguredTableAssociationAnalysisRule`, `GetAnalysisTemplate`, `GetCollaborationAnalysisTemplate`, `GetConfiguredTableAnalysisRule`, `GetConfiguredTableAssociationAnalysisRule`, `GetProtectedJob`, `GetSchemaAnalysisRule`, `ListAnalysisTemplates`, `ListCollaborationAnalysisTemplates`, `ListProtectedJobs`, `StartProtectedJob`, `StartProtectedQuery`, `UpdateAnalysisTemplate`, ... (+3).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 88 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Glue`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AnalysisTemplateResource` | `AnalysisTemplateId`, `MembershipId` | create: `CreateAnalysisTemplate`; read: `GetAnalysisTemplate`; update: `UpdateAnalysisTemplate`; delete: `DeleteAnalysisTemplate`; list: `ListAnalysisTemplates` | - | Represents a query that can be stored within a collaboration |
| `CollaborationResource` | `CollaborationId` | create: `CreateCollaboration`; read: `GetCollaboration`; update: `UpdateCollaboration`; delete: `DeleteCollaboration`; list: `ListCollaborations` | `BatchGetCollaborationAnalysisTemplate`, `BatchGetSchema`, `BatchGetSchemaAnalysisRule`, `CreateCollaborationChangeRequest`, `DeleteMember`, `GetCollaborationAnalysisTemplate`, `GetCollaborationChangeRequest`, `GetCollaborationConfiguredAudienceModelAssociation`, `GetCollaborationIdNamespaceAssociation`, `GetCollaborationPrivacyBudgetTemplate`, ... (+11) | Represents a collaboration between AWS accounts that allows for secure data collaboration |
| `ConfiguredAudienceModelAssociationResource` | `ConfiguredAudienceModelAssociationId`, `MembershipId` | create: `CreateConfiguredAudienceModelAssociation`; read: `GetConfiguredAudienceModelAssociation`; update: `UpdateConfiguredAudienceModelAssociation`; delete: `DeleteConfiguredAudienceModelAssociation`; list: `ListConfiguredAudienceModelAssociations` | - | Represents a configured audience that has been associated with the collaboration |
| `ConfiguredTableAssociationResource` | `ConfiguredTableAssociationId`, `MembershipId` | create: `CreateConfiguredTableAssociation`; read: `GetConfiguredTableAssociation`; update: `UpdateConfiguredTableAssociation`; delete: `DeleteConfiguredTableAssociation`; list: `ListConfiguredTableAssociations` | `CreateConfiguredTableAssociationAnalysisRule`, `DeleteConfiguredTableAssociationAnalysisRule`, `GetConfiguredTableAssociationAnalysisRule`, `UpdateConfiguredTableAssociationAnalysisRule` | Represents a table that can be queried within a collaboration |
| `ConfiguredTableResource` | `ConfiguredTableId` | create: `CreateConfiguredTable`; read: `GetConfiguredTable`; update: `UpdateConfiguredTable`; delete: `DeleteConfiguredTable`; list: `ListConfiguredTables` | `CreateConfiguredTableAnalysisRule`, `DeleteConfiguredTableAnalysisRule`, `GetConfiguredTableAnalysisRule`, `UpdateConfiguredTableAnalysisRule` | Represents a table that can be associated with collaborations |
| `IdMappingTableResource` | `IdMappingTableId`, `MembershipId` | create: `CreateIdMappingTable`; read: `GetIdMappingTable`; update: `UpdateIdMappingTable`; delete: `DeleteIdMappingTable`; list: `ListIdMappingTables` | `PopulateIdMappingTable` | Represents an Id Mapping Workflow associate with a collaboration |
| `IdNamespaceAssociationResource` | `IdNamespaceAssociationId`, `MembershipId` | create: `CreateIdNamespaceAssociation`; read: `GetIdNamespaceAssociation`; update: `UpdateIdNamespaceAssociation`; delete: `DeleteIdNamespaceAssociation`; list: `ListIdNamespaceAssociations` | - | Represents an AWS Entity Resolution Id Namespace resource that has been associated with the collaboration for use in a entity resolution workflow as a data input |
| `MembershipResource` | `MembershipId` | create: `CreateMembership`; read: `GetMembership`; update: `UpdateMembership`; delete: `DeleteMembership`; list: `ListMemberships` | `GetProtectedJob`, `GetProtectedQuery`, `ListPrivacyBudgets`, `ListProtectedJobs`, `ListProtectedQueries`, `PreviewPrivacyImpact`, `StartProtectedJob`, `StartProtectedQuery`, `UpdateProtectedJob`, `UpdateProtectedQuery` | Represents an AWS account that is a part of a collaboration |
| `PrivacyBudgetTemplateResource` | `MembershipId`, `PrivacyBudgetTemplateId` | create: `CreatePrivacyBudgetTemplate`; read: `GetPrivacyBudgetTemplate`; update: `UpdatePrivacyBudgetTemplate`; delete: `DeletePrivacyBudgetTemplate`; list: `ListPrivacyBudgetTemplates` | - | Represents a privacy budget template |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
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
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Lists all of the tags that have been added to a resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Tags a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Removes a tag or list of tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, reason | Caller does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType, reason | Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | message | Unexpected error during processing of request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | Request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, quotaName, quotaValue | Request denied because service quota has been exceeded. |
| `ThrottlingException` | `structure` | message | Request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the specified constraints. |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `AccessBudgetType` | `enum` | CALENDAR_DAY, CALENDAR_MONTH, CALENDAR_WEEK, LIFETIME | - |
| `AdditionalAnalyses` | `enum` | ALLOWED, REQUIRED, NOT_ALLOWED | - |
| `AnalysisFormat` | `enum` | SQL, PYSPARK_1_0 | - |
| `AnalysisMethod` | `enum` | DIRECT_QUERY, DIRECT_JOB, MULTIPLE | - |
| `AnalysisRuleType` | `enum` | AGGREGATION, LIST, CUSTOM, ID_MAPPING_TABLE | - |
| `AnalysisTemplateValidationStatus` | `enum` | VALID, INVALID, UNABLE_TO_VALIDATE | - |
| `AnalysisTemplateValidationType` | `enum` | DIFFERENTIAL_PRIVACY | - |
| `AnalysisType` | `enum` | DIRECT_ANALYSIS, ADDITIONAL_ANALYSIS | - |
| `AnalyticsEngine` | `enum` | SPARK, CLEAN_ROOMS_SQL | - |
| `ApprovalStatus` | `enum` | APPROVED, DENIED, PENDING | - |
| `AutoApprovedChangeType` | `enum` | ADD_MEMBER, GRANT_RECEIVE_RESULTS_ABILITY, REVOKE_RECEIVE_RESULTS_ABILITY | - |
| `AutoRefreshMode` | `enum` | ENABLED, DISABLED | - |
| `ChangeRequestAction` | `enum` | APPROVE, DENY, CANCEL, COMMIT | - |
| `ChangeRequestStatus` | `enum` | PENDING, APPROVED, CANCELLED, DENIED, COMMITTED | - |
| `ChangeSpecificationType` | `enum` | MEMBER, COLLABORATION | - |
| `ChangeType` | `enum` | ADD_MEMBER, GRANT_RECEIVE_RESULTS_ABILITY, REVOKE_RECEIVE_RESULTS_ABILITY, EDIT_AUTO_APPROVED_CHANGE_TYPES | - |
| `CollaborationJobLogStatus` | `enum` | ENABLED, DISABLED | - |
| `CollaborationQueryLogStatus` | `enum` | ENABLED, DISABLED | - |
| `CommercialRegion` | `enum` | US_WEST_1, US_WEST_2, US_EAST_1, US_EAST_2, AF_SOUTH_1, AP_EAST_1, AP_SOUTH_2, AP_SOUTHEAST_1, AP_SOUTHEAST_2, AP_SOUTHEAST_3, AP_SOUTHEAST_5, AP_SOUTHEAST_4, ... (+21) | - |
| `ConfiguredTableAnalysisRuleType` | `enum` | AGGREGATION, LIST, CUSTOM | - |
| `ConfiguredTableAssociationAnalysisRuleType` | `enum` | AGGREGATION, LIST, CUSTOM | - |
| `CustomMLMemberAbility` | `enum` | CAN_RECEIVE_MODEL_OUTPUT, CAN_RECEIVE_INFERENCE_OUTPUT | - |
| `DifferentialPrivacyAggregationType` | `enum` | AVG, COUNT, COUNT_DISTINCT, SUM, STDDEV | - |
| `ErrorMessageType` | `enum` | DETAILED | - |
| `IdNamespaceType` | `enum` | SOURCE, TARGET | - |
| `JobType` | `enum` | BATCH, INCREMENTAL, DELETE_ONLY | - |
| `MemberAbility` | `enum` | CAN_QUERY, CAN_RECEIVE_RESULTS, CAN_RUN_JOB | - |
| `MembershipJobLogStatus` | `enum` | ENABLED, DISABLED | - |
| `MembershipQueryLogStatus` | `enum` | ENABLED, DISABLED | - |
| `ParameterType` | `enum` | SMALLINT, INTEGER, BIGINT, DECIMAL, REAL, DOUBLE_PRECISION, BOOLEAN, CHAR, VARCHAR, DATE, TIMESTAMP, TIMESTAMPTZ, ... (+16) | - |
| `PrivacyBudgetTemplateAutoRefresh` | `enum` | CALENDAR_MONTH, NONE | - |
| `PrivacyBudgetType` | `enum` | DIFFERENTIAL_PRIVACY, ACCESS_BUDGET | - |
| `ProtectedJobAnalysisType` | `enum` | DIRECT_ANALYSIS | - |
| `ProtectedJobStatus` | `enum` | SUBMITTED, STARTED, CANCELLED, CANCELLING, FAILED, SUCCESS | - |
| `ProtectedJobType` | `enum` | PYSPARK | - |
| `ProtectedJobWorkerComputeType` | `enum` | CR1X, CR4X | - |
| `ResultFormat` | `enum` | CSV, PARQUET | File format of the returned data. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
