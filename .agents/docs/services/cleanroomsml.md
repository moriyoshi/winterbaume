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

- Operations: `ListAudienceExportJobs`, `ListAudienceGenerationJobs`, `ListAudienceModels`, `ListCollaborationConfiguredModelAlgorithmAssociations`, `ListCollaborationMLInputChannels`, `ListCollaborationTrainedModelExportJobs`, `ListCollaborationTrainedModelInferenceJobs`, `ListCollaborationTrainedModels`, `ListConfiguredAudienceModels`, `ListConfiguredModelAlgorithmAssociations`, `ListConfiguredModelAlgorithms`, `ListMLInputChannels`, `ListTagsForResource`, `ListTrainedModelInferenceJobs`, `ListTrainedModelVersions`, `ListTrainedModels`, `ListTrainingDatasets`
- Traits: `paginated` (16), `readonly` (17)
- Common required input members in this group: `collaborationIdentifier`, `membershipIdentifier`, `resourceArn`, `trainedModelArn`

### Get

- Operations: `GetAudienceGenerationJob`, `GetAudienceModel`, `GetCollaborationConfiguredModelAlgorithmAssociation`, `GetCollaborationMLInputChannel`, `GetCollaborationTrainedModel`, `GetConfiguredAudienceModel`, `GetConfiguredAudienceModelPolicy`, `GetConfiguredModelAlgorithm`, `GetConfiguredModelAlgorithmAssociation`, `GetMLConfiguration`, `GetMLInputChannel`, `GetTrainedModel`, `GetTrainedModelInferenceJob`, `GetTrainingDataset`
- Traits: `readonly` (14)
- Common required input members in this group: `audienceGenerationJobArn`, `audienceModelArn`, `collaborationIdentifier`, `configuredAudienceModelArn`, `configuredModelAlgorithmArn`, `configuredModelAlgorithmAssociationArn`, `membershipIdentifier`, `mlInputChannelArn`, `trainedModelArn`, `trainedModelInferenceJobArn`, `trainingDatasetArn`

### Delete

- Operations: `DeleteAudienceGenerationJob`, `DeleteAudienceModel`, `DeleteConfiguredAudienceModel`, `DeleteConfiguredAudienceModelPolicy`, `DeleteConfiguredModelAlgorithm`, `DeleteConfiguredModelAlgorithmAssociation`, `DeleteMLConfiguration`, `DeleteMLInputChannelData`, `DeleteTrainedModelOutput`, `DeleteTrainingDataset`
- Traits: `idempotent` (10)
- Common required input members in this group: `audienceGenerationJobArn`, `audienceModelArn`, `configuredAudienceModelArn`, `configuredModelAlgorithmArn`, `configuredModelAlgorithmAssociationArn`, `membershipIdentifier`, `mlInputChannelArn`, `trainedModelArn`, `trainingDatasetArn`

### Create

- Operations: `CreateAudienceModel`, `CreateConfiguredAudienceModel`, `CreateConfiguredModelAlgorithm`, `CreateConfiguredModelAlgorithmAssociation`, `CreateMLInputChannel`, `CreateTrainedModel`, `CreateTrainingDataset`
- Traits: `idempotent` (7)
- Common required input members in this group: `audienceModelArn`, `configuredModelAlgorithmArn`, `configuredModelAlgorithmAssociationArn`, `configuredModelAlgorithmAssociations`, `dataChannels`, `inputChannel`, `membershipIdentifier`, `name`, `outputConfig`, `resourceConfig`, `retentionInDays`, `roleArn`, `sharedAudienceMetrics`, `trainingData`, `trainingDatasetArn`

### Start

- Operations: `StartAudienceExportJob`, `StartAudienceGenerationJob`, `StartTrainedModelExportJob`, `StartTrainedModelInferenceJob`
- Traits: `idempotent` (4)
- Common required input members in this group: `audienceGenerationJobArn`, `audienceSize`, `configuredAudienceModelArn`, `dataSource`, `membershipIdentifier`, `name`, `outputConfiguration`, `resourceConfig`, `seedAudience`, `trainedModelArn`

### Cancel

- Operations: `CancelTrainedModel`, `CancelTrainedModelInferenceJob`
- Traits: `idempotent` (2)
- Common required input members in this group: `membershipIdentifier`, `trainedModelArn`, `trainedModelInferenceJobArn`

### Put

- Operations: `PutConfiguredAudienceModelPolicy`, `PutMLConfiguration`
- Traits: `idempotent` (2)
- Common required input members in this group: `configuredAudienceModelArn`, `configuredAudienceModelPolicy`, `defaultOutputLocation`, `membershipIdentifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateConfiguredAudienceModel`
- Traits: `idempotent` (1)
- Common required input members in this group: `configuredAudienceModelArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelTrainedModel` | `PATCH /memberships/{membershipIdentifier}/trained-models/{trainedModelArn}` | `idempotent` | `membershipIdentifier`, `trainedModelArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Submits a request to cancel the trained model job. |
| `CancelTrainedModelInferenceJob` | `PATCH /memberships/{membershipIdentifier}/trained-model-inference-jobs/{trainedModelInferenceJobArn}` | `idempotent` | `membershipIdentifier`, `trainedModelInferenceJobArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Submits a request to cancel a trained model inference job. |
| `CreateAudienceModel` | `POST /audience-model` | `idempotent` | `name`, `trainingDatasetArn` | - | `CreateAudienceModelResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Defines the information necessary to create an audience model. An audience model is a machine learning model that Clean Rooms ML trains to measure similarity between users. |
| `CreateConfiguredAudienceModel` | `POST /configured-audience-model` | `idempotent` | `audienceModelArn`, `name`, `outputConfig`, `sharedAudienceMetrics` | - | `CreateConfiguredAudienceModelResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Defines the information necessary to create a configured audience model. |
| `CreateConfiguredModelAlgorithm` | `POST /configured-model-algorithms` | `idempotent` | `name`, `roleArn` | - | `CreateConfiguredModelAlgorithmResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a configured model algorithm using a container image stored in an ECR repository. |
| `CreateConfiguredModelAlgorithmAssociation` | `POST /memberships/{membershipIdentifier}/configured-model-algorithm-associations` | `idempotent` | `configuredModelAlgorithmArn`, `membershipIdentifier`, `name` | - | `CreateConfiguredModelAlgorithmAssociationResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a configured model algorithm to a collaboration for use by any member of the collaboration. |
| `CreateMLInputChannel` | `POST /memberships/{membershipIdentifier}/ml-input-channels` | `idempotent` | `configuredModelAlgorithmAssociations`, `inputChannel`, `membershipIdentifier`, `name`, `retentionInDays` | - | `CreateMLInputChannelResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Provides the information to create an ML input channel. An ML input channel is the result of a query that can be used for ML modeling. |
| `CreateTrainedModel` | `POST /memberships/{membershipIdentifier}/trained-models` | `idempotent` | `configuredModelAlgorithmAssociationArn`, `dataChannels`, `membershipIdentifier`, `name`, `resourceConfig` | - | `CreateTrainedModelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a trained model from an associated configured model algorithm using data from any member of the collaboration. |
| `CreateTrainingDataset` | `POST /training-dataset` | `idempotent` | `name`, `roleArn`, `trainingData` | - | `CreateTrainingDatasetResponse` | `AccessDeniedException`, `ConflictException`, `ValidationException` | Defines the information necessary to create a training dataset. In Clean Rooms ML, the `TrainingDataset` is metadata that points to a Glue table, which is read only during `AudienceModel` creation. |
| `DeleteAudienceGenerationJob` | `DELETE /audience-generation-job/{audienceGenerationJobArn}` | `idempotent` | `audienceGenerationJobArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified audience generation job, and removes all data associated with the job. |
| `DeleteAudienceModel` | `DELETE /audience-model/{audienceModelArn}` | `idempotent` | `audienceModelArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Specifies an audience model that you want to delete. You can't delete an audience model if there are any configured audience models that depend on the audience model. |
| `DeleteConfiguredAudienceModel` | `DELETE /configured-audience-model/{configuredAudienceModelArn}` | `idempotent` | `configuredAudienceModelArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified configured audience model. You can't delete a configured audience model if there are any lookalike models that use the configured audience model. |
| `DeleteConfiguredAudienceModelPolicy` | `DELETE /configured-audience-model/{configuredAudienceModelArn}/policy` | `idempotent` | `configuredAudienceModelArn` | - | `Unit` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified configured audience model policy. |
| `DeleteConfiguredModelAlgorithm` | `DELETE /configured-model-algorithms/{configuredModelAlgorithmArn}` | `idempotent` | `configuredModelAlgorithmArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes a configured model algorithm. |
| `DeleteConfiguredModelAlgorithmAssociation` | `DELETE /memberships/{membershipIdentifier}/configured-model-algorithm-associations/{configuredModelAlgorithmAssociationArn}` | `idempotent` | `configuredModelAlgorithmAssociationArn`, `membershipIdentifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a configured model algorithm association. |
| `DeleteMLConfiguration` | `DELETE /memberships/{membershipIdentifier}/ml-configurations` | `idempotent` | `membershipIdentifier` | - | `Unit` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a ML modeling configuration. |
| `DeleteMLInputChannelData` | `DELETE /memberships/{membershipIdentifier}/ml-input-channels/{mlInputChannelArn}` | `idempotent` | `membershipIdentifier`, `mlInputChannelArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides the information necessary to delete an ML input channel. |
| `DeleteTrainedModelOutput` | `DELETE /memberships/{membershipIdentifier}/trained-models/{trainedModelArn}` | `idempotent` | `membershipIdentifier`, `trainedModelArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the model artifacts stored by the service. |
| `DeleteTrainingDataset` | `DELETE /training-dataset/{trainingDatasetArn}` | `idempotent` | `trainingDatasetArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Specifies a training dataset that you want to delete. You can't delete a training dataset if there are any audience models that depend on the training dataset. |
| `GetAudienceGenerationJob` | `GET /audience-generation-job/{audienceGenerationJobArn}` | `readonly` | `audienceGenerationJobArn` | - | `GetAudienceGenerationJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns information about an audience generation job. |
| `GetAudienceModel` | `GET /audience-model/{audienceModelArn}` | `readonly` | `audienceModelArn` | - | `GetAudienceModelResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns information about an audience model |
| `GetCollaborationConfiguredModelAlgorithmAssociation` | `GET /collaborations/{collaborationIdentifier}/configured-model-algorithm-associations/{configuredModelAlgorithmAssociationArn}` | `readonly` | `collaborationIdentifier`, `configuredModelAlgorithmAssociationArn` | - | `GetCollaborationConfiguredModelAlgorithmAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the configured model algorithm association in a collaboration. |
| `GetCollaborationMLInputChannel` | `GET /collaborations/{collaborationIdentifier}/ml-input-channels/{mlInputChannelArn}` | `readonly` | `collaborationIdentifier`, `mlInputChannelArn` | - | `GetCollaborationMLInputChannelResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific ML input channel in a collaboration. |
| `GetCollaborationTrainedModel` | `GET /collaborations/{collaborationIdentifier}/trained-models/{trainedModelArn}` | `readonly` | `collaborationIdentifier`, `trainedModelArn` | - | `GetCollaborationTrainedModelResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a trained model in a collaboration. |
| `GetConfiguredAudienceModel` | `GET /configured-audience-model/{configuredAudienceModelArn}` | `readonly` | `configuredAudienceModelArn` | - | `GetConfiguredAudienceModelResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a specified configured audience model. |
| `GetConfiguredAudienceModelPolicy` | `GET /configured-audience-model/{configuredAudienceModelArn}/policy` | `readonly` | `configuredAudienceModelArn` | - | `GetConfiguredAudienceModelPolicyResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a configured audience model policy. |
| `GetConfiguredModelAlgorithm` | `GET /configured-model-algorithms/{configuredModelAlgorithmArn}` | `readonly` | `configuredModelAlgorithmArn` | - | `GetConfiguredModelAlgorithmResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a configured model algorithm. |
| `GetConfiguredModelAlgorithmAssociation` | `GET /memberships/{membershipIdentifier}/configured-model-algorithm-associations/{configuredModelAlgorithmAssociationArn}` | `readonly` | `configuredModelAlgorithmAssociationArn`, `membershipIdentifier` | - | `GetConfiguredModelAlgorithmAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a configured model algorithm association. |
| `GetMLConfiguration` | `GET /memberships/{membershipIdentifier}/ml-configurations` | `readonly` | `membershipIdentifier` | - | `GetMLConfigurationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific ML configuration. |
| `GetMLInputChannel` | `GET /memberships/{membershipIdentifier}/ml-input-channels/{mlInputChannelArn}` | `readonly` | `membershipIdentifier`, `mlInputChannelArn` | - | `GetMLInputChannelResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about an ML input channel. |
| `GetTrainedModel` | `GET /memberships/{membershipIdentifier}/trained-models/{trainedModelArn}` | `readonly` | `membershipIdentifier`, `trainedModelArn` | - | `GetTrainedModelResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a trained model. |
| `GetTrainedModelInferenceJob` | `GET /memberships/{membershipIdentifier}/trained-model-inference-jobs/{trainedModelInferenceJobArn}` | `readonly` | `membershipIdentifier`, `trainedModelInferenceJobArn` | - | `GetTrainedModelInferenceJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a trained model inference job. |
| `GetTrainingDataset` | `GET /training-dataset/{trainingDatasetArn}` | `readonly` | `trainingDatasetArn` | - | `GetTrainingDatasetResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a training dataset. |
| `ListAudienceExportJobs` | `GET /audience-export-job` | `readonly`, `paginated` | - | - | `ListAudienceExportJobsResponse` | `AccessDeniedException`, `ValidationException` | Returns a list of the audience export jobs. |
| `ListAudienceGenerationJobs` | `GET /audience-generation-job` | `readonly`, `paginated` | - | - | `ListAudienceGenerationJobsResponse` | `AccessDeniedException`, `ValidationException` | Returns a list of audience generation jobs. |
| `ListAudienceModels` | `GET /audience-model` | `readonly`, `paginated` | - | - | `ListAudienceModelsResponse` | `AccessDeniedException`, `ValidationException` | Returns a list of audience models. |
| `ListCollaborationConfiguredModelAlgorithmAssociations` | `GET /collaborations/{collaborationIdentifier}/configured-model-algorithm-associations` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationConfiguredModelAlgorithmAssociationsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the configured model algorithm associations in a collaboration. |
| `ListCollaborationMLInputChannels` | `GET /collaborations/{collaborationIdentifier}/ml-input-channels` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationMLInputChannelsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the ML input channels in a collaboration. |
| `ListCollaborationTrainedModelExportJobs` | `GET /collaborations/{collaborationIdentifier}/trained-models/{trainedModelArn}/export-jobs` | `readonly`, `paginated` | `collaborationIdentifier`, `trainedModelArn` | - | `ListCollaborationTrainedModelExportJobsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the export jobs for a trained model in a collaboration. |
| `ListCollaborationTrainedModelInferenceJobs` | `GET /collaborations/{collaborationIdentifier}/trained-model-inference-jobs` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationTrainedModelInferenceJobsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of trained model inference jobs in a specified collaboration. |
| `ListCollaborationTrainedModels` | `GET /collaborations/{collaborationIdentifier}/trained-models` | `readonly`, `paginated` | `collaborationIdentifier` | - | `ListCollaborationTrainedModelsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of the trained models in a collaboration. |
| `ListConfiguredAudienceModels` | `GET /configured-audience-model` | `readonly`, `paginated` | - | - | `ListConfiguredAudienceModelsResponse` | `AccessDeniedException`, `ValidationException` | Returns a list of the configured audience models. |
| `ListConfiguredModelAlgorithmAssociations` | `GET /memberships/{membershipIdentifier}/configured-model-algorithm-associations` | `readonly`, `paginated` | `membershipIdentifier` | - | `ListConfiguredModelAlgorithmAssociationsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of configured model algorithm associations. |
| `ListConfiguredModelAlgorithms` | `GET /configured-model-algorithms` | `readonly`, `paginated` | - | - | `ListConfiguredModelAlgorithmsResponse` | `AccessDeniedException`, `ValidationException` | Returns a list of configured model algorithms. |
| `ListMLInputChannels` | `GET /memberships/{membershipIdentifier}/ml-input-channels` | `readonly`, `paginated` | `membershipIdentifier` | - | `ListMLInputChannelsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of ML input channels. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of tags for a provided resource. |
| `ListTrainedModelInferenceJobs` | `GET /memberships/{membershipIdentifier}/trained-model-inference-jobs` | `readonly`, `paginated` | `membershipIdentifier` | - | `ListTrainedModelInferenceJobsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of trained model inference jobs that match the request parameters. |
| `ListTrainedModelVersions` | `GET /memberships/{membershipIdentifier}/trained-models/{trainedModelArn}/versions` | `readonly`, `paginated` | `membershipIdentifier`, `trainedModelArn` | - | `ListTrainedModelVersionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of trained model versions for a specified trained model. This operation allows you to view all versions of a trained model, including information about their status and creation details. |
| `ListTrainedModels` | `GET /memberships/{membershipIdentifier}/trained-models` | `readonly`, `paginated` | `membershipIdentifier` | - | `ListTrainedModelsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of trained models. |
| `ListTrainingDatasets` | `GET /training-dataset` | `readonly`, `paginated` | - | - | `ListTrainingDatasetsResponse` | `AccessDeniedException`, `ValidationException` | Returns a list of training datasets. |
| `PutConfiguredAudienceModelPolicy` | `PUT /configured-audience-model/{configuredAudienceModelArn}/policy` | `idempotent` | `configuredAudienceModelArn`, `configuredAudienceModelPolicy` | - | `PutConfiguredAudienceModelPolicyResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Create or update the resource policy for a configured audience model. |
| `PutMLConfiguration` | `PUT /memberships/{membershipIdentifier}/ml-configurations` | `idempotent` | `defaultOutputLocation`, `membershipIdentifier` | - | `Unit` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Assigns information about an ML configuration. |
| `StartAudienceExportJob` | `POST /audience-export-job` | `idempotent` | `audienceGenerationJobArn`, `audienceSize`, `name` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Export an audience of a specified size after you have generated an audience. |
| `StartAudienceGenerationJob` | `POST /audience-generation-job` | `idempotent` | `configuredAudienceModelArn`, `name`, `seedAudience` | - | `StartAudienceGenerationJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Information necessary to start the audience generation job. |
| `StartTrainedModelExportJob` | `POST /memberships/{membershipIdentifier}/trained-models/{trainedModelArn}/export-jobs` | `idempotent` | `membershipIdentifier`, `name`, `outputConfiguration`, `trainedModelArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides the information necessary to start a trained model export job. |
| `StartTrainedModelInferenceJob` | `POST /memberships/{membershipIdentifier}/trained-model-inference-jobs` | `idempotent` | `dataSource`, `membershipIdentifier`, `name`, `outputConfiguration`, `resourceConfig`, `trainedModelArn` | - | `StartTrainedModelInferenceJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Defines the information necessary to begin a trained model inference job. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Adds metadata tags to a specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes metadata tags from a specified resource. |
| `UpdateConfiguredAudienceModel` | `PATCH /configured-audience-model/{configuredAudienceModelArn}` | `idempotent` | `configuredAudienceModelArn` | - | `UpdateConfiguredAudienceModelResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Provides the information necessary to update a configured audience model. Updates that impact audience generation jobs take effect when a new job starts, but do not impact currently running jobs. |

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
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `ValidationException` | `structure` | `message` | The request parameters for this request are incorrect. |
| `ResourceNotFoundException` | `structure` | `message` | The resource you are requesting does not exist. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ConflictException` | `structure` | `message` | You can't complete this action because another resource depends on this resource. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaName`, `quotaValue` | You have exceeded your service quota. |
| `CancelTrainedModelRequest` | `structure` | `membershipIdentifier`, `trainedModelArn`, `versionIdentifier` | - |
| `CancelTrainedModelInferenceJobRequest` | `structure` | `membershipIdentifier`, `trainedModelInferenceJobArn` | - |
| `CreateAudienceModelRequest` | `structure` | `description`, `kmsKeyArn`, `name`, `tags`, `trainingDataEndTime`, `trainingDataStartTime`, `trainingDatasetArn` | - |
| `CreateAudienceModelResponse` | `structure` | `audienceModelArn` | - |
| `CreateConfiguredAudienceModelRequest` | `structure` | `audienceModelArn`, `audienceSizeConfig`, `childResourceTagOnCreatePolicy`, `description`, `minMatchingSeedSize`, `name`, `outputConfig`, `sharedAudienceMetrics`, `tags` | - |
| `CreateConfiguredAudienceModelResponse` | `structure` | `configuredAudienceModelArn` | - |
| `CreateConfiguredModelAlgorithmRequest` | `structure` | `description`, `inferenceContainerConfig`, `kmsKeyArn`, `name`, `roleArn`, `tags`, `trainingContainerConfig` | - |
| `CreateConfiguredModelAlgorithmResponse` | `structure` | `configuredModelAlgorithmArn` | - |
| `CreateConfiguredModelAlgorithmAssociationRequest` | `structure` | `configuredModelAlgorithmArn`, `description`, `membershipIdentifier`, `name`, `privacyConfiguration`, `tags` | - |
| `CreateConfiguredModelAlgorithmAssociationResponse` | `structure` | `configuredModelAlgorithmAssociationArn` | - |
| `CreateMLInputChannelRequest` | `structure` | `configuredModelAlgorithmAssociations`, `description`, `inputChannel`, `kmsKeyArn`, `membershipIdentifier`, `name`, `retentionInDays`, `tags` | - |
| `CreateMLInputChannelResponse` | `structure` | `mlInputChannelArn` | - |
| `CreateTrainedModelRequest` | `structure` | `configuredModelAlgorithmAssociationArn`, `dataChannels`, `description`, `environment`, `hyperparameters`, `incrementalTrainingDataChannels`, `kmsKeyArn`, `membershipIdentifier`, `name`, `resourceConfig`, `stoppingCondition`, `tags`, ... (+1) | - |
| `CreateTrainedModelResponse` | `structure` | `trainedModelArn`, `versionIdentifier` | - |
| `InternalServiceException` | `structure` | `message` | An internal service error occurred. |
| `CreateTrainingDatasetRequest` | `structure` | `description`, `name`, `roleArn`, `tags`, `trainingData` | - |
| `CreateTrainingDatasetResponse` | `structure` | `trainingDatasetArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
