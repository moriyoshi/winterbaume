# AWS Clean Rooms ML

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Amazon Web Services Clean Rooms ML API Reference . Amazon Web Services Clean Rooms ML provides a privacy-enhancing method for two parties to identify similar users in their data without the need to share their data with each other. The first party brings the training data to Clean Rooms so that they can create and configure an audience model (lookalike model) and associate it with a collaboration. The second party then brings their seed data to Clean Rooms and generates an audience (lookalike segment) that resembles the training data. To learn more about Amazon Web Services Clean Rooms ML concepts, procedures, and best practices, see the Clean Rooms User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Clean Rooms ML where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Clean Rooms ML by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Clean Rooms ML resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Clean Rooms ML workflows in the local mock. Key resources include `AudienceExportJob`, `AudienceGenerationJob`, `AudienceModel`, `ConfiguredAudienceModel`, `ConfiguredAudienceModelPolicy`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Delete`, `Create`, `Start` operation families, including `ListAudienceExportJobs`, `ListAudienceGenerationJobs`, `ListAudienceModels`, `ListCollaborationConfiguredModelAlgorithmAssociations`, `GetAudienceGenerationJob`, `GetAudienceModel`.

## Service Identity and Protocol

- AWS model slug: `cleanroomsml`
- AWS SDK for Rust slug: `cleanroomsml`
- Model version: `2023-09-06`
- Model file: `vendor/api-models-aws/models/cleanroomsml/service/2023-09-06/cleanroomsml-2023-09-06.json`
- SDK ID: `CleanRoomsML`
- Endpoint prefix: `-`
- ARN namespace: `cleanrooms-ml`
- CloudFormation name: `-`
- CloudTrail event source: `cleanrooms-ml.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (17), `Get` (14), `Delete` (10), `Create` (7), `Start` (4), `Cancel` (2), `Put` (2), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelTrainedModel`, `CancelTrainedModelInferenceJob`, `CreateAudienceModel`, `CreateConfiguredAudienceModel`, `CreateConfiguredModelAlgorithm`, `CreateConfiguredModelAlgorithmAssociation`, `CreateMLInputChannel`, `CreateTrainedModel`, `CreateTrainingDataset`, `DeleteAudienceGenerationJob`, `DeleteAudienceModel`, `DeleteConfiguredAudienceModel`, `DeleteConfiguredAudienceModelPolicy`, `DeleteConfiguredModelAlgorithm`, `DeleteConfiguredModelAlgorithmAssociation`, `DeleteMLConfiguration`, `DeleteMLInputChannelData`, `DeleteTrainedModelOutput`, `DeleteTrainingDataset`, `PutConfiguredAudienceModelPolicy`, ... (+8).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAudienceGenerationJob`, `GetAudienceModel`, `GetCollaborationConfiguredModelAlgorithmAssociation`, `GetCollaborationMLInputChannel`, `GetCollaborationTrainedModel`, `GetConfiguredAudienceModel`, `GetConfiguredAudienceModelPolicy`, `GetConfiguredModelAlgorithm`, `GetConfiguredModelAlgorithmAssociation`, `GetMLConfiguration`, `GetMLInputChannel`, `GetTrainedModel`, `GetTrainedModelInferenceJob`, `GetTrainingDataset`, `ListAudienceExportJobs`, `ListAudienceGenerationJobs`, `ListAudienceModels`, `ListCollaborationConfiguredModelAlgorithmAssociations`, `ListCollaborationMLInputChannels`, `ListCollaborationTrainedModelExportJobs`, ... (+11).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 27 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelTrainedModel`, `CancelTrainedModelInferenceJob`, `DeleteAudienceGenerationJob`, `GetAudienceGenerationJob`, `GetTrainedModelInferenceJob`, `ListAudienceExportJobs`, `ListAudienceGenerationJobs`, `ListCollaborationTrainedModelExportJobs`, `ListCollaborationTrainedModelInferenceJobs`, `ListTrainedModelInferenceJobs`, `StartAudienceExportJob`, `StartAudienceGenerationJob`, `StartTrainedModelExportJob`, `StartTrainedModelInferenceJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 59 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Glue`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AudienceExportJob` | `audienceExportJobArn` | create: `StartAudienceExportJob`; list: `ListAudienceExportJobs` | - | - |
| `AudienceGenerationJob` | `audienceGenerationJobArn` | create: `StartAudienceGenerationJob`; read: `GetAudienceGenerationJob`; delete: `DeleteAudienceGenerationJob`; list: `ListAudienceGenerationJobs` | - | - |
| `AudienceModel` | `audienceModelArn` | create: `CreateAudienceModel`; read: `GetAudienceModel`; delete: `DeleteAudienceModel`; list: `ListAudienceModels` | - | - |
| `ConfiguredAudienceModel` | `configuredAudienceModelArn` | create: `CreateConfiguredAudienceModel`; read: `GetConfiguredAudienceModel`; update: `UpdateConfiguredAudienceModel`; delete: `DeleteConfiguredAudienceModel`; list: `ListConfiguredAudienceModels` | - | - |
| `ConfiguredAudienceModelPolicy` | `configuredAudienceModelArn` | put: `PutConfiguredAudienceModelPolicy`; read: `GetConfiguredAudienceModelPolicy`; delete: `DeleteConfiguredAudienceModelPolicy` | - | - |
| `ConfiguredModelAlgorithm` | `configuredModelAlgorithmArn` | create: `CreateConfiguredModelAlgorithm`; read: `GetConfiguredModelAlgorithm`; delete: `DeleteConfiguredModelAlgorithm`; list: `ListConfiguredModelAlgorithms` | - | - |
| `ConfiguredModelAlgorithmAssociation` | `configuredModelAlgorithmAssociationArn` | create: `CreateConfiguredModelAlgorithmAssociation`; read: `GetConfiguredModelAlgorithmAssociation`; delete: `DeleteConfiguredModelAlgorithmAssociation`; list: `ListConfiguredModelAlgorithmAssociations` | `GetCollaborationConfiguredModelAlgorithmAssociation` | - |
| `MLConfiguration` | `membershipIdentifier` | put: `PutMLConfiguration`; read: `GetMLConfiguration`; delete: `DeleteMLConfiguration` | - | - |
| `MLInputChannel` | `mlInputChannelArn` | create: `CreateMLInputChannel`; read: `GetMLInputChannel`; delete: `DeleteMLInputChannelData`; list: `ListMLInputChannels` | `GetCollaborationMLInputChannel` | - |
| `TrainedModel` | `trainedModelArn` | create: `CreateTrainedModel`; read: `GetTrainedModel`; delete: `DeleteTrainedModelOutput`; list: `ListTrainedModels` | `CancelTrainedModel`, `GetCollaborationTrainedModel`, `ListTrainedModelVersions` | - |
| `TrainedModelExportJob` | `trainedModelExportJobArn` | create: `StartTrainedModelExportJob` | - | - |
| `TrainedModelInferenceJob` | `trainedModelInferenceJobArn` | create: `StartTrainedModelInferenceJob`; read: `GetTrainedModelInferenceJob`; list: `ListTrainedModelInferenceJobs` | `CancelTrainedModelInferenceJob` | - |
| `TrainingDataset` | `trainingDatasetArn` | create: `CreateTrainingDataset`; read: `GetTrainingDataset`; delete: `DeleteTrainingDataset`; list: `ListTrainingDatasets` | - | - |
## Operation Groups

### List

- Operations: `ListCollaborationConfiguredModelAlgorithmAssociations`, `ListCollaborationMLInputChannels`, `ListCollaborationTrainedModelExportJobs`, `ListCollaborationTrainedModelInferenceJobs`, `ListCollaborationTrainedModels`, `ListTagsForResource`
- Traits: `readonly` (6), `paginated` (5)
- Common required input members in this group: `collaborationIdentifier`

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
| `ListCollaborationConfiguredModelAlgorithmAssociations` | `GET /collaborations/{collaborationIdentifier}/configured-model-algorithm-associations` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationConfiguredModelAlgorithmAssociationsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the configured model algorithm associations in a collaboration. |
| `ListCollaborationMLInputChannels` | `GET /collaborations/{collaborationIdentifier}/ml-input-channels` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationMLInputChannelsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the ML input channels in a collaboration. |
| `ListCollaborationTrainedModelExportJobs` | `GET /collaborations/{collaborationIdentifier}/trained-models/{trainedModelArn}/export-jobs` | `readonly`, `paginated` | `collaborationIdentifier`, `trainedModelArn` | - | `ListCollaborationTrainedModelExportJobsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the export jobs for a trained model in a collaboration. |
| `ListCollaborationTrainedModelInferenceJobs` | `GET /collaborations/{collaborationIdentifier}/trained-model-inference-jobs` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationTrainedModelInferenceJobsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of trained model inference jobs in a specified collaboration. |
| `ListCollaborationTrainedModels` | `GET /collaborations/{collaborationIdentifier}/trained-models` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationTrainedModelsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the trained models in a collaboration. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of tags for a provided resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Adds metadata tags to a specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes metadata tags from a specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListCollaborationConfiguredModelAlgorithmAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListCollaborationMLInputChannels` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListCollaborationTrainedModelExportJobs` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `trainedModelVersionIdentifier -> trainedModelVersionIdentifier` | - | - |
| `ListCollaborationTrainedModelInferenceJobs` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `trainedModelArn -> trainedModelArn`, `trainedModelVersionIdentifier -> trainedModelVersionIdentifier` | - | - |
| `ListCollaborationTrainedModels` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | You can't complete this action because another resource depends on this resource. |
| `InternalServiceException` | `structure` | message | An internal service error occurred. Retry your request. If the problem persists, contact AWS Support. |
| `ResourceNotFoundException` | `structure` | message | The resource you are requesting does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, quotaName, quotaValue | You have exceeded your service quota. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The request parameters for this request are incorrect. |
| `ListCollaborationConfiguredModelAlgorithmAssociationsRequest` | `structure` | nextToken, maxResults, collaborationIdentifier | - |
| `ListCollaborationConfiguredModelAlgorithmAssociationsResponse` | `structure` | nextToken, collaborationConfiguredModelAlgorithmAssociations | - |
| `ListCollaborationMLInputChannelsRequest` | `structure` | nextToken, maxResults, collaborationIdentifier | - |
| `ListCollaborationMLInputChannelsResponse` | `structure` | nextToken, collaborationMLInputChannelsList | - |
| `ListCollaborationTrainedModelExportJobsRequest` | `structure` | nextToken, maxResults, collaborationIdentifier, trainedModelArn, trainedModelVersionIdentifier | - |
| `ListCollaborationTrainedModelExportJobsResponse` | `structure` | nextToken, collaborationTrainedModelExportJobs | - |
| `ListCollaborationTrainedModelInferenceJobsRequest` | `structure` | nextToken, maxResults, collaborationIdentifier, trainedModelArn, trainedModelVersionIdentifier | - |
| `ListCollaborationTrainedModelInferenceJobsResponse` | `structure` | nextToken, collaborationTrainedModelInferenceJobs | - |
| `ListCollaborationTrainedModelsRequest` | `structure` | nextToken, maxResults, collaborationIdentifier | - |
| `ListCollaborationTrainedModelsResponse` | `structure` | nextToken, collaborationTrainedModels | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AccessBudgetType` | `enum` | CALENDAR_DAY, CALENDAR_MONTH, CALENDAR_WEEK, LIFETIME | - |
| `AudienceExportJobStatus` | `enum` | CREATE_PENDING, CREATE_IN_PROGRESS, CREATE_FAILED, ACTIVE | - |
| `AudienceGenerationJobStatus` | `enum` | CREATE_PENDING, CREATE_IN_PROGRESS, CREATE_FAILED, ACTIVE, DELETE_PENDING, DELETE_IN_PROGRESS, DELETE_FAILED | - |
| `AudienceModelStatus` | `enum` | CREATE_PENDING, CREATE_IN_PROGRESS, CREATE_FAILED, ACTIVE, DELETE_PENDING, DELETE_IN_PROGRESS, DELETE_FAILED | - |
| `AudienceSizeType` | `enum` | ABSOLUTE, PERCENTAGE | - |
| `AutoRefreshMode` | `enum` | ENABLED, DISABLED | - |
| `ColumnType` | `enum` | USER_ID, ITEM_ID, TIMESTAMP, CATEGORICAL_FEATURE, NUMERICAL_FEATURE | - |
| `ConfiguredAudienceModelStatus` | `enum` | ACTIVE | - |
| `DatasetType` | `enum` | INTERACTIONS | - |
| `EntityType` | `enum` | ALL_PERSONALLY_IDENTIFIABLE_INFORMATION, NUMBERS, CUSTOM | - |
| `InferenceInstanceType` | `enum` | ML_R7I_48XLARGE, ML_R6I_16XLARGE, ML_M6I_XLARGE, ML_M5_4XLARGE, ML_P2_XLARGE, ML_M4_16XLARGE, ML_R7I_16XLARGE, ML_M7I_XLARGE, ML_M6I_12XLARGE, ML_R7I_8XLARGE, ML_R7I_LARGE, ML_M7I_12XLARGE, ... (+82) | - |
| `InstanceType` | `enum` | ML_M4_XLARGE, ML_M4_2XLARGE, ML_M4_4XLARGE, ML_M4_10XLARGE, ML_M4_16XLARGE, ML_G4DN_XLARGE, ML_G4DN_2XLARGE, ML_G4DN_4XLARGE, ML_G4DN_8XLARGE, ML_G4DN_12XLARGE, ML_G4DN_16XLARGE, ML_M5_LARGE, ... (+121) | - |
| `LogType` | `enum` | ALL, ERROR_SUMMARY | - |
| `LogsStatus` | `enum` | PUBLISH_SUCCEEDED, PUBLISH_FAILED | - |
| `MLInputChannelStatus` | `enum` | CREATE_PENDING, CREATE_IN_PROGRESS, CREATE_FAILED, ACTIVE, DELETE_PENDING, DELETE_IN_PROGRESS, DELETE_FAILED, INACTIVE | - |
| `MembershipInferenceAttackVersion` | `enum` | DISTANCE_TO_CLOSEST_RECORD_V1 | - |
| `MetricsStatus` | `enum` | PUBLISH_SUCCEEDED, PUBLISH_FAILED | - |
| `NoiseLevelType` | `enum` | HIGH, MEDIUM, LOW, NONE | - |
| `PolicyExistenceCondition` | `enum` | POLICY_MUST_EXIST, POLICY_MUST_NOT_EXIST | - |
| `ResultFormat` | `enum` | CSV, PARQUET | File format of the returned data. |
| `S3DataDistributionType` | `enum` | FULLY_REPLICATED, SHARDED_BY_S3_KEY | - |
| `SharedAudienceMetrics` | `enum` | ALL, NONE | - |
| `SyntheticDataColumnType` | `enum` | CATEGORICAL, NUMERICAL | - |
| `TagOnCreatePolicy` | `enum` | FROM_PARENT_RESOURCE, NONE | - |
| `TrainedModelArtifactMaxSizeUnitType` | `enum` | GB | - |
| `TrainedModelExportFileType` | `enum` | MODEL, OUTPUT | - |
| `TrainedModelExportJobStatus` | `enum` | CREATE_PENDING, CREATE_IN_PROGRESS, CREATE_FAILED, ACTIVE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
