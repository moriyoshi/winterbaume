# Amazon SageMaker Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provides APIs for creating and managing SageMaker resources. Other Resources: SageMaker Developer Guide Amazon Augmented AI Runtime API Reference

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon SageMaker Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon SageMaker Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon SageMaker Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon SageMaker Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Update` operation families, including `ListActions`, `ListAlgorithms`, `ListAliases`, `ListAppImageConfigs`, `DescribeAction`, `DescribeAlgorithm`.

## Service Identity and Protocol

- AWS model slug: `sagemaker`
- AWS SDK for Rust slug: `sagemaker`
- Model version: `2017-07-24`
- Model file: `vendor/api-models-aws/models/sagemaker/service/2017-07-24/sagemaker-2017-07-24.json`
- SDK ID: `SageMaker`
- Endpoint prefix: `api.sagemaker`
- ARN namespace: `sagemaker`
- CloudFormation name: `SageMaker`
- CloudTrail event source: `sagemaker.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (85), `Describe` (71), `Create` (68), `Delete` (58), `Update` (45), `Stop` (16), `Start` (7), `Get` (6).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddAssociation`, `AddTags`, `AssociateTrialComponent`, `AttachClusterNodeVolume`, `BatchAddClusterNodes`, `BatchDeleteClusterNodes`, `BatchDescribeModelPackage`, `BatchRebootClusterNodes`, `BatchReplaceClusterNodes`, `CreateAction`, `CreateAlgorithm`, `CreateApp`, `CreateAppImageConfig`, `CreateArtifact`, `CreateAutoMLJob`, `CreateAutoMLJobV2`, `CreateCluster`, `CreateClusterSchedulerConfig`, `CreateCodeRepository`, `CreateCompilationJob`, ... (+191).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAction`, `DescribeAlgorithm`, `DescribeApp`, `DescribeAppImageConfig`, `DescribeArtifact`, `DescribeAutoMLJob`, `DescribeAutoMLJobV2`, `DescribeCluster`, `DescribeClusterEvent`, `DescribeClusterNode`, `DescribeClusterSchedulerConfig`, `DescribeCodeRepository`, `DescribeCompilationJob`, `DescribeComputeQuota`, `DescribeContext`, `DescribeDataQualityJobDefinition`, `DescribeDevice`, `DescribeDeviceFleet`, `DescribeDomain`, `DescribeEdgeDeploymentPlan`, ... (+144).
- Pagination is modelled for 86 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateAutoMLJob`, `CreateAutoMLJobV2`, `CreateCompilationJob`, `CreateDataQualityJobDefinition`, `CreateEdgePackagingJob`, `CreateHumanTaskUi`, `CreateHyperParameterTuningJob`, `CreateInferenceRecommendationsJob`, `CreateLabelingJob`, `CreateModelBiasJobDefinition`, `CreateModelCardExportJob`, `CreateModelExplainabilityJobDefinition`, `CreateModelQualityJobDefinition`, `CreateOptimizationJob`, `CreateProcessingJob`, `CreateTrainingJob`, `CreateTransformJob`, `DeleteCompilationJob`, `DeleteDataQualityJobDefinition`, `DeleteHumanTaskUi`, ... (+80).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 276 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `SQS`, `Lambda`, `Glue`, `EC2/VPC`, `ECR`, `ECS`, `EKS`, `Redshift`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Control-Plane / Data-Plane Coherence

- **Paired with `sagemakerruntime` ( synchronous and async invocation ) and `sagemakermetrics` ( telemetry write-only ).** All three crates use the same SDK slug `sagemaker` in the AWS SDK index; in real AWS, `sagemakerruntime`'s `InvokeEndpoint` targets endpoints created by this control plane via `CreateEndpoint`, and `sagemakermetrics`'s `BatchPutMetrics` writes telemetry against trial components created by this control plane.
- **Current Winterbaume status: divergent for the runtime, benign for metrics.**
  - `winterbaume-sagemakerruntime` does not depend on `winterbaume-sagemaker`. Its `state.rs` carries the comment "*In production, endpoints are created via the SageMaker API ( not Runtime ). For mock purposes, we auto-create endpoints on first invocation.*" — so `InvokeEndpoint` always succeeds, even for endpoint names that were never created here.
  - `winterbaume-sagemakermetrics` is fire-and-forget telemetry; AWS itself does not validate trial component names against the control plane on `BatchPutMetrics`. Independent state is fine.
- **What needs to change:** `winterbaume-sagemakerruntime` should observe this crate's `endpoints` and reject `InvokeEndpoint` for unknown endpoints with `ValidationError` ( "Endpoint X of account Y not found" ), matching real AWS. The endpoint-config / model / variant graph should also resolve through here so the invocation record can capture the targeted variant. The `sagemakermetrics` separation needs no change.

## Current Network Resource Stub Semantics

SageMaker currently leaves most VPC-aware model fields unpopulated in implemented state.

- Model records include a `vpc_config` slot, but current `create_model` initialises it to `None`.
- Domain records include VPC ID, subnet IDs, app network access type, and security group IDs, but current `create_domain` initialises VPC ID to `None` and the subnet/security group lists to empty.
- Notebook, job, app, and workforce VPC shapes exist in the generated model, but the implemented state does not provision ENIs or validate networking inputs.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListActions`, `ListAlgorithms`, `ListAliases`, `ListAppImageConfigs`, `ListApps`, `ListArtifacts`, `ListAssociations`, `ListAutoMLJobs`, `ListCandidatesForAutoMLJob`, `ListClusterEvents`, `ListClusterNodes`, `ListClusterSchedulerConfigs`, `ListClusters`, `ListCodeRepositories`, `ListCompilationJobs`, `ListComputeQuotas`, `ListContexts`, `ListDataQualityJobDefinitions`, `ListDeviceFleets`, `ListDevices`, `ListDomains`, `ListEdgeDeploymentPlans`, `ListEdgePackagingJobs`, `ListEndpointConfigs`, `ListEndpoints`, `ListExperiments`, `ListFeatureGroups`, `ListFlowDefinitions`, `ListHubContentVersions`, `ListHubContents`, `ListHubs`, `ListHumanTaskUis`, `ListHyperParameterTuningJobs`, `ListImageVersions`, `ListImages`, `ListInferenceComponents`, `ListInferenceExperiments`, `ListInferenceRecommendationsJobSteps`, `ListInferenceRecommendationsJobs`, `ListLabelingJobs`, ... (+45)
- Traits: `paginated` (82)
- Common required input members in this group: `AutoMLJobName`, `ClusterName`, `EdgeDeploymentPlanName`, `HubContentName`, `HubContentType`, `HubName`, `HyperParameterTuningJobName`, `ImageName`, `JobName`, `ModelCardName`, `MonitoringScheduleName`, `PipelineExecutionArn`, `PipelineName`, `ReservedCapacityArn`, `ResourceArn`, `StageName`, `WorkteamArn`

### Describe

- Operations: `DescribeAction`, `DescribeAlgorithm`, `DescribeApp`, `DescribeAppImageConfig`, `DescribeArtifact`, `DescribeAutoMLJob`, `DescribeAutoMLJobV2`, `DescribeCluster`, `DescribeClusterEvent`, `DescribeClusterNode`, `DescribeClusterSchedulerConfig`, `DescribeCodeRepository`, `DescribeCompilationJob`, `DescribeComputeQuota`, `DescribeContext`, `DescribeDataQualityJobDefinition`, `DescribeDevice`, `DescribeDeviceFleet`, `DescribeDomain`, `DescribeEdgeDeploymentPlan`, `DescribeEdgePackagingJob`, `DescribeEndpoint`, `DescribeEndpointConfig`, `DescribeExperiment`, `DescribeFeatureGroup`, `DescribeFeatureMetadata`, `DescribeFlowDefinition`, `DescribeHub`, `DescribeHubContent`, `DescribeHumanTaskUi`, `DescribeHyperParameterTuningJob`, `DescribeImage`, `DescribeImageVersion`, `DescribeInferenceComponent`, `DescribeInferenceExperiment`, `DescribeInferenceRecommendationsJob`, `DescribeLabelingJob`, `DescribeLineageGroup`, `DescribeMlflowApp`, `DescribeMlflowTrackingServer`, ... (+31)
- Traits: `paginated` (1)
- Common required input members in this group: `ActionName`, `AlgorithmName`, `AppImageConfigName`, `AppName`, `AppType`, `Arn`, `ArtifactArn`, `AutoMLJobName`, `ClusterName`, `ClusterSchedulerConfigId`, `CodeRepositoryName`, `CompilationJobName`, `ComputeQuotaId`, `ContextName`, `DeviceFleetName`, `DeviceName`, `DomainId`, `EdgeDeploymentPlanName`, `EdgePackagingJobName`, `EndpointConfigName`, `EndpointName`, `EventId`, `ExperimentName`, `FeatureGroupName`, ... (+41)

### Create

- Operations: `CreateAction`, `CreateAlgorithm`, `CreateApp`, `CreateAppImageConfig`, `CreateArtifact`, `CreateAutoMLJob`, `CreateAutoMLJobV2`, `CreateCluster`, `CreateClusterSchedulerConfig`, `CreateCodeRepository`, `CreateCompilationJob`, `CreateComputeQuota`, `CreateContext`, `CreateDataQualityJobDefinition`, `CreateDeviceFleet`, `CreateDomain`, `CreateEdgeDeploymentPlan`, `CreateEdgeDeploymentStage`, `CreateEdgePackagingJob`, `CreateEndpoint`, `CreateEndpointConfig`, `CreateExperiment`, `CreateFeatureGroup`, `CreateFlowDefinition`, `CreateHub`, `CreateHubContentPresignedUrls`, `CreateHubContentReference`, `CreateHumanTaskUi`, `CreateHyperParameterTuningJob`, `CreateImage`, `CreateImageVersion`, `CreateInferenceComponent`, `CreateInferenceExperiment`, `CreateInferenceRecommendationsJob`, `CreateLabelingJob`, `CreateMlflowApp`, `CreateMlflowTrackingServer`, `CreateModel`, `CreateModelBiasJobDefinition`, `CreateModelCard`, ... (+28)
- Traits: `idempotency-token` (4), `paginated` (1)
- Common required input members in this group: `ActionName`, `ActionType`, `AlgorithmName`, `AppImageConfigName`, `AppName`, `AppSpecification`, `AppType`, `Arn`, `ArtifactStoreUri`, `ArtifactType`, `AuthMode`, `AuthType`, `AutoMLJobInputDataConfig`, `AutoMLJobName`, `AutoMLProblemTypeConfig`, `BaseImage`, `ClientRequestToken`, `ClientToken`, `ClusterArn`, `ClusterName`, `CodeRepositoryName`, `CompilationJobName`, `ComputeQuotaConfig`, `ComputeQuotaTarget`, ... (+105)

### Delete

- Operations: `DeleteAction`, `DeleteAlgorithm`, `DeleteApp`, `DeleteAppImageConfig`, `DeleteArtifact`, `DeleteAssociation`, `DeleteCluster`, `DeleteClusterSchedulerConfig`, `DeleteCodeRepository`, `DeleteCompilationJob`, `DeleteComputeQuota`, `DeleteContext`, `DeleteDataQualityJobDefinition`, `DeleteDeviceFleet`, `DeleteDomain`, `DeleteEdgeDeploymentPlan`, `DeleteEdgeDeploymentStage`, `DeleteEndpoint`, `DeleteEndpointConfig`, `DeleteExperiment`, `DeleteFeatureGroup`, `DeleteFlowDefinition`, `DeleteHub`, `DeleteHubContent`, `DeleteHubContentReference`, `DeleteHumanTaskUi`, `DeleteHyperParameterTuningJob`, `DeleteImage`, `DeleteImageVersion`, `DeleteInferenceComponent`, `DeleteInferenceExperiment`, `DeleteMlflowApp`, `DeleteMlflowTrackingServer`, `DeleteModel`, `DeleteModelBiasJobDefinition`, `DeleteModelCard`, `DeleteModelExplainabilityJobDefinition`, `DeleteModelPackage`, `DeleteModelPackageGroup`, `DeleteModelPackageGroupPolicy`, ... (+18)
- Traits: `idempotency-token` (2)
- Common required input members in this group: `ActionName`, `AlgorithmName`, `AppImageConfigName`, `AppName`, `AppType`, `Arn`, `ClientRequestToken`, `ClusterName`, `ClusterSchedulerConfigId`, `CodeRepositoryName`, `CompilationJobName`, `ComputeQuotaId`, `ContextName`, `DestinationArn`, `DeviceFleetName`, `DomainId`, `EdgeDeploymentPlanName`, `EndpointConfigName`, `EndpointName`, `ExperimentName`, `FeatureGroupName`, `FlowDefinitionName`, `HubContentName`, `HubContentType`, ... (+32)

### Update

- Operations: `UpdateAction`, `UpdateAppImageConfig`, `UpdateArtifact`, `UpdateCluster`, `UpdateClusterSchedulerConfig`, `UpdateClusterSoftware`, `UpdateCodeRepository`, `UpdateComputeQuota`, `UpdateContext`, `UpdateDeviceFleet`, `UpdateDevices`, `UpdateDomain`, `UpdateEndpoint`, `UpdateEndpointWeightsAndCapacities`, `UpdateExperiment`, `UpdateFeatureGroup`, `UpdateFeatureMetadata`, `UpdateHub`, `UpdateHubContent`, `UpdateHubContentReference`, `UpdateImage`, `UpdateImageVersion`, `UpdateInferenceComponent`, `UpdateInferenceComponentRuntimeConfig`, `UpdateInferenceExperiment`, `UpdateMlflowApp`, `UpdateMlflowTrackingServer`, `UpdateModelCard`, `UpdateModelPackage`, `UpdateMonitoringAlert`, `UpdateMonitoringSchedule`, `UpdateNotebookInstance`, `UpdateNotebookInstanceLifecycleConfig`, `UpdatePartnerApp`, `UpdatePipeline`, `UpdatePipelineExecution`, `UpdatePipelineVersion`, `UpdateProject`, `UpdateSpace`, `UpdateTrainingJob`, ... (+5)
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ActionName`, `AppImageConfigName`, `Arn`, `ArtifactArn`, `ClusterName`, `ClusterSchedulerConfigId`, `CodeRepositoryName`, `ComputeQuotaId`, `ContextName`, `DatapointsToAlert`, `DesiredRuntimeConfig`, `DesiredWeightsAndCapacities`, `DeviceFleetName`, `Devices`, `DomainId`, `EndpointConfigName`, `EndpointName`, `EvaluationPeriod`, `ExperimentName`, `FeatureGroupName`, `FeatureName`, `HubContentName`, `HubContentType`, `HubContentVersion`, ... (+26)

### Stop

- Operations: `StopAutoMLJob`, `StopCompilationJob`, `StopEdgeDeploymentStage`, `StopEdgePackagingJob`, `StopHyperParameterTuningJob`, `StopInferenceExperiment`, `StopInferenceRecommendationsJob`, `StopLabelingJob`, `StopMlflowTrackingServer`, `StopMonitoringSchedule`, `StopNotebookInstance`, `StopOptimizationJob`, `StopPipelineExecution`, `StopProcessingJob`, `StopTrainingJob`, `StopTransformJob`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AutoMLJobName`, `ClientRequestToken`, `CompilationJobName`, `EdgeDeploymentPlanName`, `EdgePackagingJobName`, `HyperParameterTuningJobName`, `JobName`, `LabelingJobName`, `ModelVariantActions`, `MonitoringScheduleName`, `Name`, `NotebookInstanceName`, `OptimizationJobName`, `PipelineExecutionArn`, `ProcessingJobName`, `StageName`, `TrackingServerName`, `TrainingJobName`, `TransformJobName`

### Start

- Operations: `StartEdgeDeploymentStage`, `StartInferenceExperiment`, `StartMlflowTrackingServer`, `StartMonitoringSchedule`, `StartNotebookInstance`, `StartPipelineExecution`, `StartSession`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientRequestToken`, `EdgeDeploymentPlanName`, `MonitoringScheduleName`, `Name`, `NotebookInstanceName`, `PipelineName`, `ResourceIdentifier`, `StageName`, `TrackingServerName`

### Get

- Operations: `GetDeviceFleetReport`, `GetLineageGroupPolicy`, `GetModelPackageGroupPolicy`, `GetSagemakerServicecatalogPortfolioStatus`, `GetScalingConfigurationRecommendation`, `GetSearchSuggestions`
- Common required input members in this group: `DeviceFleetName`, `InferenceRecommendationsJobName`, `LineageGroupName`, `ModelPackageGroupName`, `Resource`

### Batch

- Operations: `BatchAddClusterNodes`, `BatchDeleteClusterNodes`, `BatchDescribeModelPackage`, `BatchRebootClusterNodes`, `BatchReplaceClusterNodes`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClusterName`, `ModelPackageArnList`, `NodesToAdd`

### Add

- Operations: `AddAssociation`, `AddTags`
- Common required input members in this group: `DestinationArn`, `ResourceArn`, `SourceArn`, `Tags`

### Search

- Operations: `Search`, `SearchTrainingPlanOfferings`
- Traits: `paginated` (1)
- Common required input members in this group: `Resource`

### Send

- Operations: `SendPipelineExecutionStepFailure`, `SendPipelineExecutionStepSuccess`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `CallbackToken`

### Associate

- Operations: `AssociateTrialComponent`
- Common required input members in this group: `TrialComponentName`, `TrialName`

### Attach

- Operations: `AttachClusterNodeVolume`
- Common required input members in this group: `ClusterArn`, `NodeId`, `VolumeId`

### Deregister

- Operations: `DeregisterDevices`
- Common required input members in this group: `DeviceFleetName`, `DeviceNames`

### Detach

- Operations: `DetachClusterNodeVolume`
- Common required input members in this group: `ClusterArn`, `NodeId`, `VolumeId`

### Disable

- Operations: `DisableSagemakerServicecatalogPortfolio`

### Disassociate

- Operations: `DisassociateTrialComponent`
- Common required input members in this group: `TrialComponentName`, `TrialName`

### Enable

- Operations: `EnableSagemakerServicecatalogPortfolio`

### Extend

- Operations: `ExtendTrainingPlan`
- Common required input members in this group: `TrainingPlanExtensionOfferingId`

### Import

- Operations: `ImportHubContent`
- Common required input members in this group: `DocumentSchemaVersion`, `HubContentDocument`, `HubContentName`, `HubContentType`, `HubName`

### Put

- Operations: `PutModelPackageGroupPolicy`
- Common required input members in this group: `ModelPackageGroupName`, `ResourcePolicy`

### Query

- Operations: `QueryLineage`
- Traits: `paginated` (1)

### Register

- Operations: `RegisterDevices`
- Common required input members in this group: `DeviceFleetName`, `Devices`

### Render

- Operations: `RenderUiTemplate`
- Common required input members in this group: `RoleArn`, `Task`

### Retry

- Operations: `RetryPipelineExecution`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientRequestToken`, `PipelineExecutionArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddAssociation` | - | - | `DestinationArn`, `SourceArn` | - | `AddAssociationResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Creates an association between the source and the destination. A source can be associated with multiple destinations, and a destination can be associated with multiple sources. |
| `AddTags` | - | - | `ResourceArn`, `Tags` | - | `AddTagsOutput` | - | Adds or overwrites one or more tags for the specified SageMaker resource. You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints. |
| `AssociateTrialComponent` | - | - | `TrialComponentName`, `TrialName` | - | `AssociateTrialComponentResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Associates a trial component with a trial. A trial component can be associated with multiple trials. |
| `AttachClusterNodeVolume` | - | - | `ClusterArn`, `NodeId`, `VolumeId` | - | `AttachClusterNodeVolumeResponse` | `ResourceNotFound` | Attaches your Amazon Elastic Block Store (Amazon EBS) volume to a node in your EKS orchestrated HyperPod cluster. This API works with the Amazon Elastic Block Store (Amazon EBS) Container Storage Interface (CSI) driver to manage the lifecycle of persistent... |
| `BatchAddClusterNodes` | - | `idempotency-token` | `ClusterName`, `NodesToAdd` | `ClientToken` | `BatchAddClusterNodesResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Adds nodes to a HyperPod cluster by incrementing the target count for one or more instance groups. This operation returns a unique `NodeLogicalId` for each node being added, which can be used to track the provisioning status of the node. |
| `BatchDeleteClusterNodes` | - | - | `ClusterName` | - | `BatchDeleteClusterNodesResponse` | `ResourceNotFound` | Deletes specific nodes within a SageMaker HyperPod cluster. `BatchDeleteClusterNodes` accepts a cluster name and a list of node IDs. |
| `BatchDescribeModelPackage` | - | - | `ModelPackageArnList` | - | `BatchDescribeModelPackageOutput` | - | This action batch describes a list of versioned model packages |
| `BatchRebootClusterNodes` | - | - | `ClusterName` | - | `BatchRebootClusterNodesResponse` | `ResourceNotFound` | Reboots specific nodes within a SageMaker HyperPod cluster using a soft recovery mechanism. `BatchRebootClusterNodes` performs a graceful reboot of the specified nodes by calling the Amazon Elastic Compute Cloud `RebootInstances` API, which attempts to... |
| `BatchReplaceClusterNodes` | - | - | `ClusterName` | - | `BatchReplaceClusterNodesResponse` | `ResourceNotFound` | Replaces specific nodes within a SageMaker HyperPod cluster with new hardware. `BatchReplaceClusterNodes` terminates the specified instances and provisions new replacement instances with the same configuration but fresh hardware. |
| `CreateAction` | - | - | `ActionName`, `ActionType`, `Source` | - | `CreateActionResponse` | `ResourceLimitExceeded` | Creates an action . An action is a lineage tracking entity that represents an action or activity. |
| `CreateAlgorithm` | - | - | `AlgorithmName`, `TrainingSpecification` | - | `CreateAlgorithmOutput` | - | Create a machine learning algorithm that you can use in SageMaker and list in the Amazon Web Services Marketplace. |
| `CreateApp` | - | - | `AppName`, `AppType`, `DomainId` | - | `CreateAppResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a running app for the specified UserProfile. This operation is automatically invoked by Amazon SageMaker AI upon access to the associated Domain, and when new kernel configurations are selected by the user. |
| `CreateAppImageConfig` | - | - | `AppImageConfigName` | - | `CreateAppImageConfigResponse` | `ResourceInUse` | Creates a configuration for running a SageMaker AI image as a KernelGateway app. The configuration specifies the Amazon Elastic File System storage volume on the image, and a list of the kernels in the image. |
| `CreateArtifact` | - | - | `ArtifactType`, `Source` | - | `CreateArtifactResponse` | `ResourceLimitExceeded` | Creates an artifact . An artifact is a lineage tracking entity that represents a URI addressable object or data. |
| `CreateAutoMLJob` | - | - | `AutoMLJobName`, `InputDataConfig`, `OutputDataConfig`, `RoleArn` | - | `CreateAutoMLJobResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates an Autopilot job also referred to as Autopilot experiment or AutoML job. An AutoML job in SageMaker AI is a fully automated process that allows you to build machine learning models with minimal effort and machine learning expertise. |
| `CreateAutoMLJobV2` | - | - | `AutoMLJobInputDataConfig`, `AutoMLJobName`, `AutoMLProblemTypeConfig`, `OutputDataConfig`, `RoleArn` | - | `CreateAutoMLJobV2Response` | `ResourceInUse`, `ResourceLimitExceeded` | Creates an Autopilot job also referred to as Autopilot experiment or AutoML job V2. An AutoML job in SageMaker AI is a fully automated process that allows you to build machine learning models with minimal effort and machine learning expertise. |
| `CreateCluster` | - | - | `ClusterName` | - | `CreateClusterResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates an Amazon SageMaker HyperPod cluster. SageMaker HyperPod is a capability of SageMaker for creating and managing persistent clusters for developing large machine learning models, such as large language models (LLMs) and diffusion models. |
| `CreateClusterSchedulerConfig` | - | - | `ClusterArn`, `Name`, `SchedulerConfig` | - | `CreateClusterSchedulerConfigResponse` | `ConflictException`, `ResourceLimitExceeded` | Create cluster policy configuration. This policy is used for task prioritization and fair-share allocation of idle compute. |
| `CreateCodeRepository` | - | - | `CodeRepositoryName`, `GitConfig` | - | `CreateCodeRepositoryOutput` | - | Creates a Git repository as a resource in your SageMaker AI account. You can associate the repository with notebook instances so that you can use Git source control for the notebooks you create. |
| `CreateCompilationJob` | - | - | `CompilationJobName`, `OutputConfig`, `RoleArn`, `StoppingCondition` | - | `CreateCompilationJobResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Starts a model compilation job. After the model has been compiled, Amazon SageMaker AI saves the resulting model artifacts to an Amazon Simple Storage Service (Amazon S3) bucket that you specify. |
| `CreateComputeQuota` | - | - | `ClusterArn`, `ComputeQuotaConfig`, `ComputeQuotaTarget`, `Name` | - | `CreateComputeQuotaResponse` | `ConflictException`, `ResourceLimitExceeded` | Create compute allocation definition. This defines how compute is allocated, shared, and borrowed for specified entities. |
| `CreateContext` | - | - | `ContextName`, `ContextType`, `Source` | - | `CreateContextResponse` | `ResourceLimitExceeded` | Creates a context . A context is a lineage tracking entity that represents a logical grouping of other tracking or experiment entities. |
| `CreateDataQualityJobDefinition` | - | - | `DataQualityAppSpecification`, `DataQualityJobInput`, `DataQualityJobOutputConfig`, `JobDefinitionName`, `JobResources`, `RoleArn` | - | `CreateDataQualityJobDefinitionResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a definition for a job that monitors data quality and drift. For information about model monitor, see Amazon SageMaker AI Model Monitor. |
| `CreateDeviceFleet` | - | - | `DeviceFleetName`, `OutputConfig` | - | `Unit` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a device fleet. |
| `CreateDomain` | - | - | `AuthMode`, `DefaultUserSettings`, `DomainName` | - | `CreateDomainResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a `Domain`. A domain consists of an associated Amazon Elastic File System volume, a list of authorized users, and a variety of security, application, policy, and Amazon Virtual Private Cloud (VPC) configurations. |
| `CreateEdgeDeploymentPlan` | - | - | `DeviceFleetName`, `EdgeDeploymentPlanName`, `ModelConfigs` | - | `CreateEdgeDeploymentPlanResponse` | `ResourceLimitExceeded` | Creates an edge deployment plan, consisting of multiple stages. Each stage may have a different deployment configuration and devices. |
| `CreateEdgeDeploymentStage` | - | - | `EdgeDeploymentPlanName`, `Stages` | - | `Unit` | `ResourceLimitExceeded` | Creates a new stage in an existing edge deployment plan. |
| `CreateEdgePackagingJob` | - | - | `CompilationJobName`, `EdgePackagingJobName`, `ModelName`, `ModelVersion`, `OutputConfig`, `RoleArn` | - | `Unit` | `ResourceLimitExceeded` | Starts a SageMaker Edge Manager model packaging job. Edge Manager will use the model artifacts from the Amazon Simple Storage Service bucket that you specify. |
| `CreateEndpoint` | - | - | `EndpointConfigName`, `EndpointName` | - | `CreateEndpointOutput` | `ResourceLimitExceeded` | Creates an endpoint using the endpoint configuration specified in the request. SageMaker uses the endpoint to provision resources and deploy models. |
| `CreateEndpointConfig` | - | - | `EndpointConfigName`, `ProductionVariants` | - | `CreateEndpointConfigOutput` | `ResourceLimitExceeded` | Creates an endpoint configuration that SageMaker hosting services uses to deploy models. In the configuration, you identify one or more models, created using the `CreateModel` API, to deploy and the resources that you want SageMaker to provision. |
| `CreateExperiment` | - | - | `ExperimentName` | - | `CreateExperimentResponse` | `ResourceLimitExceeded` | Creates a SageMaker experiment . An experiment is a collection of trials that are observed, compared and evaluated as a group. |
| `CreateFeatureGroup` | - | - | `EventTimeFeatureName`, `FeatureDefinitions`, `FeatureGroupName`, `RecordIdentifierFeatureName` | - | `CreateFeatureGroupResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Create a new `FeatureGroup`. A `FeatureGroup` is a group of `Features` defined in the `FeatureStore` to describe a `Record`. |
| `CreateFlowDefinition` | - | - | `FlowDefinitionName`, `OutputConfig`, `RoleArn` | - | `CreateFlowDefinitionResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a flow definition. |
| `CreateHub` | - | - | `HubDescription`, `HubName` | - | `CreateHubResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Create a hub. |
| `CreateHubContentPresignedUrls` | - | `paginated` | `HubContentName`, `HubContentType`, `HubName` | - | `CreateHubContentPresignedUrlsResponse` | - | Creates presigned URLs for accessing hub content artifacts. This operation generates time-limited, secure URLs that allow direct download of model artifacts and associated files from Amazon SageMaker hub content, including gated models that require end-user... |
| `CreateHubContentReference` | - | - | `HubName`, `SageMakerPublicHubContentArn` | - | `CreateHubContentReferenceResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Create a hub content reference in order to add a model in the JumpStart public hub to a private hub. |
| `CreateHumanTaskUi` | - | - | `HumanTaskUiName`, `UiTemplate` | - | `CreateHumanTaskUiResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Defines the settings you will use for the human review workflow user interface. Reviewers will see a three-panel interface with an instruction area, the item to review, and an input area. |
| `CreateHyperParameterTuningJob` | - | - | `HyperParameterTuningJobConfig`, `HyperParameterTuningJobName` | - | `CreateHyperParameterTuningJobResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Starts a hyperparameter tuning job. A hyperparameter tuning job finds the best version of a model by running many training jobs on your dataset using the algorithm you choose and values for hyperparameters within ranges that you specify. |
| `CreateImage` | - | - | `ImageName`, `RoleArn` | - | `CreateImageResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a custom SageMaker AI image. A SageMaker AI image is a set of image versions. |
| `CreateImageVersion` | - | `idempotency-token` | `BaseImage`, `ClientToken`, `ImageName` | `ClientToken` | `CreateImageVersionResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Creates a version of the SageMaker AI image specified by `ImageName`. The version represents the Amazon ECR container image specified by `BaseImage`. |
| `CreateInferenceComponent` | - | - | `EndpointName`, `InferenceComponentName`, `Specification` | - | `CreateInferenceComponentOutput` | `ResourceLimitExceeded` | Creates an inference component, which is a SageMaker AI hosting object that you can use to deploy a model to an endpoint. In the inference component settings, you specify the model, the endpoint, and how the model utilizes the resources that the endpoint... |
| `CreateInferenceExperiment` | - | - | `EndpointName`, `ModelVariants`, `Name`, `RoleArn`, `ShadowModeConfig`, `Type` | - | `CreateInferenceExperimentResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates an inference experiment using the configurations specified in the request. Use this API to setup and schedule an experiment to compare model variants on a Amazon SageMaker inference endpoint. |
| `CreateInferenceRecommendationsJob` | - | - | `InputConfig`, `JobName`, `JobType`, `RoleArn` | - | `CreateInferenceRecommendationsJobResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Starts a recommendation job. You can create either an instance recommendation or load test job. |
| `CreateLabelingJob` | - | - | `HumanTaskConfig`, `InputConfig`, `LabelAttributeName`, `LabelingJobName`, `OutputConfig`, `RoleArn` | - | `CreateLabelingJobResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a job that uses workers to label the data objects in your input dataset. You can use the labeled data to train machine learning models. |
| `CreateMlflowApp` | - | - | `ArtifactStoreUri`, `Name`, `RoleArn` | - | `CreateMlflowAppResponse` | `ResourceLimitExceeded` | Creates an MLflow Tracking Server using a general purpose Amazon S3 bucket as the artifact store. |
| `CreateMlflowTrackingServer` | - | - | `ArtifactStoreUri`, `RoleArn`, `TrackingServerName` | - | `CreateMlflowTrackingServerResponse` | `ResourceLimitExceeded` | Creates an MLflow Tracking Server using a general purpose Amazon S3 bucket as the artifact store. For more information, see Create an MLflow Tracking Server. |
| `CreateModel` | - | - | `ModelName` | - | `CreateModelOutput` | `ResourceLimitExceeded` | Creates a model in SageMaker. In the request, you name the model and describe a primary container. |
| `CreateModelBiasJobDefinition` | - | - | `JobDefinitionName`, `JobResources`, `ModelBiasAppSpecification`, `ModelBiasJobInput`, `ModelBiasJobOutputConfig`, `RoleArn` | - | `CreateModelBiasJobDefinitionResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates the definition for a model bias job. |
| `CreateModelCard` | - | - | `Content`, `ModelCardName`, `ModelCardStatus` | - | `CreateModelCardResponse` | `ConflictException`, `ResourceLimitExceeded` | Creates an Amazon SageMaker Model Card. For information about how to use model cards, see Amazon SageMaker Model Card. |
| `CreateModelCardExportJob` | - | - | `ModelCardExportJobName`, `ModelCardName`, `OutputConfig` | - | `CreateModelCardExportJobResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Creates an Amazon SageMaker Model Card export job. |
| `CreateModelExplainabilityJobDefinition` | - | - | `JobDefinitionName`, `JobResources`, `ModelExplainabilityAppSpecification`, `ModelExplainabilityJobInput`, `ModelExplainabilityJobOutputConfig`, `RoleArn` | - | `CreateModelExplainabilityJobDefinitionResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates the definition for a model explainability job. |
| `CreateModelPackage` | - | `idempotency-token` | - | `ClientToken` | `CreateModelPackageOutput` | `ConflictException`, `ResourceLimitExceeded` | Creates a model package that you can use to create SageMaker models or list on Amazon Web Services Marketplace, or a versioned model that is part of a model group. Buyers can subscribe to model packages listed on Amazon Web Services Marketplace to create... |
| `CreateModelPackageGroup` | - | - | `ModelPackageGroupName` | - | `CreateModelPackageGroupOutput` | `ResourceLimitExceeded` | Creates a model group. A model group contains a group of model versions. |
| `CreateModelQualityJobDefinition` | - | - | `JobDefinitionName`, `JobResources`, `ModelQualityAppSpecification`, `ModelQualityJobInput`, `ModelQualityJobOutputConfig`, `RoleArn` | - | `CreateModelQualityJobDefinitionResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a definition for a job that monitors model quality and drift. For information about model monitor, see Amazon SageMaker AI Model Monitor. |
| `CreateMonitoringSchedule` | - | - | `MonitoringScheduleConfig`, `MonitoringScheduleName` | - | `CreateMonitoringScheduleResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a schedule that regularly starts Amazon SageMaker AI Processing Jobs to monitor the data captured for an Amazon SageMaker AI Endpoint. |
| `CreateNotebookInstance` | - | - | `InstanceType`, `NotebookInstanceName`, `RoleArn` | - | `CreateNotebookInstanceOutput` | `ResourceLimitExceeded` | Creates an SageMaker AI notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook. |
| `CreateNotebookInstanceLifecycleConfig` | - | - | `NotebookInstanceLifecycleConfigName` | - | `CreateNotebookInstanceLifecycleConfigOutput` | `ResourceLimitExceeded` | Creates a lifecycle configuration that you can associate with a notebook instance. A lifecycle configuration is a collection of shell scripts that run when you create or start a notebook instance. |
| `CreateOptimizationJob` | - | - | `DeploymentInstanceType`, `ModelSource`, `OptimizationConfigs`, `OptimizationJobName`, `OutputConfig`, `RoleArn`, `StoppingCondition` | - | `CreateOptimizationJobResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a job that optimizes a model for inference performance. To create the job, you provide the location of a source model, and you provide the settings for the optimization techniques that you want the job to apply. |
| `CreatePartnerApp` | - | `idempotency-token` | `AuthType`, `ExecutionRoleArn`, `Name`, `Tier`, `Type` | `ClientToken` | `CreatePartnerAppResponse` | `ConflictException`, `ResourceLimitExceeded` | Creates an Amazon SageMaker Partner AI App. |
| `CreatePartnerAppPresignedUrl` | - | - | `Arn` | - | `CreatePartnerAppPresignedUrlResponse` | `ResourceNotFound` | Creates a presigned URL to access an Amazon SageMaker Partner AI App. |
| `CreatePipeline` | - | `idempotency-token` | `ClientRequestToken`, `PipelineName`, `RoleArn` | `ClientRequestToken` | `CreatePipelineResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Creates a pipeline using a JSON pipeline definition. |
| `CreatePresignedDomainUrl` | - | - | `DomainId`, `UserProfileName` | - | `CreatePresignedDomainUrlResponse` | `ResourceNotFound` | Creates a URL for a specified UserProfile in a Domain. When accessed in a web browser, the user will be automatically signed in to the domain, and granted access to all of the Apps and files associated with the Domain's Amazon Elastic File System volume. |
| `CreatePresignedMlflowAppUrl` | - | - | `Arn` | - | `CreatePresignedMlflowAppUrlResponse` | `ResourceNotFound` | Returns a presigned URL that you can use to connect to the MLflow UI attached to your MLflow App. For more information, see Launch the MLflow UI using a presigned URL. |
| `CreatePresignedMlflowTrackingServerUrl` | - | - | `TrackingServerName` | - | `CreatePresignedMlflowTrackingServerUrlResponse` | `ResourceNotFound` | Returns a presigned URL that you can use to connect to the MLflow UI attached to your tracking server. For more information, see Launch the MLflow UI using a presigned URL. |
| `CreatePresignedNotebookInstanceUrl` | - | - | `NotebookInstanceName` | - | `CreatePresignedNotebookInstanceUrlOutput` | - | Returns a URL that you can use to connect to the Jupyter server from a notebook instance. In the SageMaker AI console, when you choose `Open` next to a notebook instance, SageMaker AI opens a new tab showing the Jupyter server home page from the notebook... |
| `CreateProcessingJob` | - | - | `AppSpecification`, `ProcessingJobName`, `ProcessingResources`, `RoleArn` | - | `CreateProcessingJobResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Creates a processing job. |
| `CreateProject` | - | - | `ProjectName` | - | `CreateProjectOutput` | `ResourceLimitExceeded` | Creates a machine learning (ML) project that can contain one or more templates that set up an ML pipeline from training to deploying an approved model. |
| `CreateSpace` | - | - | `DomainId`, `SpaceName` | - | `CreateSpaceResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a private space or a space used for real time collaboration in a domain. |
| `CreateStudioLifecycleConfig` | - | - | `StudioLifecycleConfigAppType`, `StudioLifecycleConfigContent`, `StudioLifecycleConfigName` | - | `CreateStudioLifecycleConfigResponse` | `ResourceInUse` | Creates a new Amazon SageMaker AI Studio Lifecycle Configuration. |
| `CreateTrainingJob` | - | - | `OutputDataConfig`, `RoleArn`, `TrainingJobName` | - | `CreateTrainingJobResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Starts a model training job. After training completes, SageMaker saves the resulting model artifacts to an Amazon S3 location that you specify. |
| `CreateTrainingPlan` | - | - | `TrainingPlanName`, `TrainingPlanOfferingId` | - | `CreateTrainingPlanResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Creates a new training plan in SageMaker to reserve compute capacity. Amazon SageMaker Training Plan is a capability within SageMaker that allows customers to reserve and manage GPU capacity for large-scale AI model training. |
| `CreateTransformJob` | - | - | `ModelName`, `TransformInput`, `TransformJobName`, `TransformOutput`, `TransformResources` | - | `CreateTransformJobResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Starts a transform job. A transform job uses a trained model to get inferences on a dataset and saves these results to an Amazon S3 location that you specify. |
| `CreateTrial` | - | - | `ExperimentName`, `TrialName` | - | `CreateTrialResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Creates an SageMaker trial . A trial is a set of steps called trial components that produce a machine learning model. |
| `CreateTrialComponent` | - | - | `TrialComponentName` | - | `CreateTrialComponentResponse` | `ResourceLimitExceeded` | Creates a trial component , which is a stage of a machine learning trial . A trial is composed of one or more trial components. |
| `CreateUserProfile` | - | - | `DomainId`, `UserProfileName` | - | `CreateUserProfileResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a user profile. A user profile represents a single user within a domain, and is the main way to reference a "person" for the purposes of sharing, reporting, and other user-oriented features. |
| `CreateWorkforce` | - | - | `WorkforceName` | - | `CreateWorkforceResponse` | - | Use this operation to create a workforce. This operation will return an error if a workforce already exists in the Amazon Web Services Region that you specify. |
| `CreateWorkteam` | - | - | `Description`, `MemberDefinitions`, `WorkteamName` | - | `CreateWorkteamResponse` | `ResourceInUse`, `ResourceLimitExceeded` | Creates a new work team for labeling your data. A work team is defined by one or more Amazon Cognito user pools. |
| `DeleteAction` | - | - | `ActionName` | - | `DeleteActionResponse` | `ResourceNotFound` | Deletes an action. |
| `DeleteAlgorithm` | - | - | `AlgorithmName` | - | `Unit` | `ConflictException` | Removes the specified algorithm from your account. |
| `DeleteApp` | - | - | `AppName`, `AppType`, `DomainId` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Used to stop and delete an app. |
| `DeleteAppImageConfig` | - | - | `AppImageConfigName` | - | `Unit` | `ResourceNotFound` | Deletes an AppImageConfig. |
| `DeleteArtifact` | - | - | - | - | `DeleteArtifactResponse` | `ResourceNotFound` | Deletes an artifact. Either `ArtifactArn` or `Source` must be specified. |
| `DeleteAssociation` | - | - | `DestinationArn`, `SourceArn` | - | `DeleteAssociationResponse` | `ResourceNotFound` | Deletes an association. |
| `DeleteCluster` | - | - | `ClusterName` | - | `DeleteClusterResponse` | `ConflictException`, `ResourceNotFound` | Delete a SageMaker HyperPod cluster. |
| `DeleteClusterSchedulerConfig` | - | - | `ClusterSchedulerConfigId` | - | `Unit` | `ResourceNotFound` | Deletes the cluster policy of the cluster. |
| `DeleteCodeRepository` | - | - | `CodeRepositoryName` | - | `Unit` | - | Deletes the specified Git repository from your account. |
| `DeleteCompilationJob` | - | - | `CompilationJobName` | - | `Unit` | `ResourceNotFound` | Deletes the specified compilation job. This action deletes only the compilation job resource in Amazon SageMaker AI. |
| `DeleteComputeQuota` | - | - | `ComputeQuotaId` | - | `Unit` | `ResourceNotFound` | Deletes the compute allocation from the cluster. |
| `DeleteContext` | - | - | `ContextName` | - | `DeleteContextResponse` | `ResourceNotFound` | Deletes an context. |
| `DeleteDataQualityJobDefinition` | - | - | `JobDefinitionName` | - | `Unit` | `ResourceNotFound` | Deletes a data quality monitoring job definition. |
| `DeleteDeviceFleet` | - | - | `DeviceFleetName` | - | `Unit` | `ResourceInUse` | Deletes a fleet. |
| `DeleteDomain` | - | - | `DomainId` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Used to delete a domain. If you onboarded with IAM mode, you will need to delete your domain to onboard again using IAM Identity Center. |
| `DeleteEdgeDeploymentPlan` | - | - | `EdgeDeploymentPlanName` | - | `Unit` | `ResourceInUse` | Deletes an edge deployment plan if (and only if) all the stages in the plan are inactive or there are no stages in the plan. |
| `DeleteEdgeDeploymentStage` | - | - | `EdgeDeploymentPlanName`, `StageName` | - | `Unit` | `ResourceInUse` | Delete a stage in an edge deployment plan if (and only if) the stage is inactive. |
| `DeleteEndpoint` | - | - | `EndpointName` | - | `Unit` | - | Deletes an endpoint. SageMaker frees up all of the resources that were deployed when the endpoint was created. |
| `DeleteEndpointConfig` | - | - | `EndpointConfigName` | - | `Unit` | - | Deletes an endpoint configuration. The `DeleteEndpointConfig` API deletes only the specified configuration. |
| `DeleteExperiment` | - | - | `ExperimentName` | - | `DeleteExperimentResponse` | `ResourceNotFound` | Deletes an SageMaker experiment. All trials associated with the experiment must be deleted first. |
| `DeleteFeatureGroup` | - | - | `FeatureGroupName` | - | `Unit` | `ResourceNotFound` | Delete the `FeatureGroup` and any data that was written to the `OnlineStore` of the `FeatureGroup`. Data cannot be accessed from the `OnlineStore` immediately after `DeleteFeatureGroup` is called. |
| `DeleteFlowDefinition` | - | - | `FlowDefinitionName` | - | `DeleteFlowDefinitionResponse` | `ResourceInUse`, `ResourceNotFound` | Deletes the specified flow definition. |
| `DeleteHub` | - | - | `HubName` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Delete a hub. |
| `DeleteHubContent` | - | - | `HubContentName`, `HubContentType`, `HubContentVersion`, `HubName` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Delete the contents of a hub. |
| `DeleteHubContentReference` | - | - | `HubContentName`, `HubContentType`, `HubName` | - | `Unit` | `ResourceNotFound` | Delete a hub content reference in order to remove a model from a private hub. |
| `DeleteHumanTaskUi` | - | - | `HumanTaskUiName` | - | `DeleteHumanTaskUiResponse` | `ResourceNotFound` | Use this operation to delete a human task user interface (worker task template). To see a list of human task user interfaces (work task templates) in your account, use ListHumanTaskUis. |
| `DeleteHyperParameterTuningJob` | - | - | `HyperParameterTuningJobName` | - | `Unit` | - | Deletes a hyperparameter tuning job. The `DeleteHyperParameterTuningJob` API deletes only the tuning job entry that was created in SageMaker when you called the `CreateHyperParameterTuningJob` API. |
| `DeleteImage` | - | - | `ImageName` | - | `DeleteImageResponse` | `ResourceInUse`, `ResourceNotFound` | Deletes a SageMaker AI image and all versions of the image. The container images aren't deleted. |
| `DeleteImageVersion` | - | - | `ImageName` | - | `DeleteImageVersionResponse` | `ResourceInUse`, `ResourceNotFound` | Deletes a version of a SageMaker AI image. The container image the version represents isn't deleted. |
| `DeleteInferenceComponent` | - | - | `InferenceComponentName` | - | `Unit` | - | Deletes an inference component. |
| `DeleteInferenceExperiment` | - | - | `Name` | - | `DeleteInferenceExperimentResponse` | `ConflictException`, `ResourceNotFound` | Deletes an inference experiment. This operation does not delete your endpoint, variants, or any underlying resources. |
| `DeleteMlflowApp` | - | - | `Arn` | - | `DeleteMlflowAppResponse` | `ResourceNotFound` | Deletes an MLflow App. |
| `DeleteMlflowTrackingServer` | - | - | `TrackingServerName` | - | `DeleteMlflowTrackingServerResponse` | `ResourceNotFound` | Deletes an MLflow Tracking Server. For more information, see Clean up MLflow resources. |
| `DeleteModel` | - | - | `ModelName` | - | `Unit` | - | Deletes a model. The `DeleteModel` API deletes only the model entry that was created in SageMaker when you called the `CreateModel` API. |
| `DeleteModelBiasJobDefinition` | - | - | `JobDefinitionName` | - | `Unit` | `ResourceNotFound` | Deletes an Amazon SageMaker AI model bias job definition. |
| `DeleteModelCard` | - | - | `ModelCardName` | - | `Unit` | `ConflictException`, `ResourceNotFound` | Deletes an Amazon SageMaker Model Card. |
| `DeleteModelExplainabilityJobDefinition` | - | - | `JobDefinitionName` | - | `Unit` | `ResourceNotFound` | Deletes an Amazon SageMaker AI model explainability job definition. |
| `DeleteModelPackage` | - | - | `ModelPackageName` | - | `Unit` | `ConflictException` | Deletes a model package. A model package is used to create SageMaker models or list on Amazon Web Services Marketplace. |
| `DeleteModelPackageGroup` | - | - | `ModelPackageGroupName` | - | `Unit` | `ConflictException` | Deletes the specified model group. |
| `DeleteModelPackageGroupPolicy` | - | - | `ModelPackageGroupName` | - | `Unit` | - | Deletes a model group resource policy. |
| `DeleteModelQualityJobDefinition` | - | - | `JobDefinitionName` | - | `Unit` | `ResourceNotFound` | Deletes the secified model quality monitoring job definition. |
| `DeleteMonitoringSchedule` | - | - | `MonitoringScheduleName` | - | `Unit` | `ResourceNotFound` | Deletes a monitoring schedule. Also stops the schedule had not already been stopped. |
| `DeleteNotebookInstance` | - | - | `NotebookInstanceName` | - | `Unit` | - | Deletes an SageMaker AI notebook instance. Before you can delete a notebook instance, you must call the `StopNotebookInstance` API. |
| `DeleteNotebookInstanceLifecycleConfig` | - | - | `NotebookInstanceLifecycleConfigName` | - | `Unit` | - | Deletes a notebook instance lifecycle configuration. |
| `DeleteOptimizationJob` | - | - | `OptimizationJobName` | - | `Unit` | `ResourceNotFound` | Deletes an optimization job. |
| `DeletePartnerApp` | - | `idempotency-token` | `Arn` | `ClientToken` | `DeletePartnerAppResponse` | `ConflictException`, `ResourceNotFound` | Deletes a SageMaker Partner AI App. |
| `DeletePipeline` | - | `idempotency-token` | `ClientRequestToken`, `PipelineName` | `ClientRequestToken` | `DeletePipelineResponse` | `ConflictException`, `ResourceNotFound` | Deletes a pipeline if there are no running instances of the pipeline. To delete a pipeline, you must stop all running instances of the pipeline using the `StopPipelineExecution` API. |
| `DeleteProcessingJob` | - | - | `ProcessingJobName` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Deletes a processing job. After Amazon SageMaker deletes a processing job, all of the metadata for the processing job is lost. |
| `DeleteProject` | - | - | `ProjectName` | - | `Unit` | `ConflictException` | Delete the specified project. |
| `DeleteSpace` | - | - | `DomainId`, `SpaceName` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Used to delete a space. |
| `DeleteStudioLifecycleConfig` | - | - | `StudioLifecycleConfigName` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Deletes the Amazon SageMaker AI Studio Lifecycle Configuration. In order to delete the Lifecycle Configuration, there must be no running apps using the Lifecycle Configuration. |
| `DeleteTags` | - | - | `ResourceArn`, `TagKeys` | - | `DeleteTagsOutput` | - | Deletes the specified tags from an SageMaker resource. To list a resource's tags, use the `ListTags` API. |
| `DeleteTrainingJob` | - | - | `TrainingJobName` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Deletes a training job. After SageMaker deletes a training job, all of the metadata for the training job is lost. |
| `DeleteTrial` | - | - | `TrialName` | - | `DeleteTrialResponse` | `ResourceNotFound` | Deletes the specified trial. All trial components that make up the trial must be deleted first. |
| `DeleteTrialComponent` | - | - | `TrialComponentName` | - | `DeleteTrialComponentResponse` | `ResourceNotFound` | Deletes the specified trial component. A trial component must be disassociated from all trials before the trial component can be deleted. |
| `DeleteUserProfile` | - | - | `DomainId`, `UserProfileName` | - | `Unit` | `ResourceInUse`, `ResourceNotFound` | Deletes a user profile. When a user profile is deleted, the user loses access to their EFS volume, including data, notebooks, and other artifacts. |
| `DeleteWorkforce` | - | - | `WorkforceName` | - | `DeleteWorkforceResponse` | - | Use this operation to delete a workforce. If you want to create a new workforce in an Amazon Web Services Region where a workforce already exists, use this operation to delete the existing workforce and then use CreateWorkforce to create a new workforce. |
| `DeleteWorkteam` | - | - | `WorkteamName` | - | `DeleteWorkteamResponse` | `ResourceLimitExceeded` | Deletes an existing work team. This operation can't be undone. |
| `DeregisterDevices` | - | - | `DeviceFleetName`, `DeviceNames` | - | `Unit` | - | Deregisters the specified devices. After you deregister a device, you will need to re-register the devices. |
| `DescribeAction` | - | - | `ActionName` | - | `DescribeActionResponse` | `ResourceNotFound` | Describes an action. |
| `DescribeAlgorithm` | - | - | `AlgorithmName` | - | `DescribeAlgorithmOutput` | - | Returns a description of the specified algorithm that is in your account. |
| `DescribeApp` | - | - | `AppName`, `AppType`, `DomainId` | - | `DescribeAppResponse` | `ResourceNotFound` | Describes the app. |
| `DescribeAppImageConfig` | - | - | `AppImageConfigName` | - | `DescribeAppImageConfigResponse` | `ResourceNotFound` | Describes an AppImageConfig. |
| `DescribeArtifact` | - | - | `ArtifactArn` | - | `DescribeArtifactResponse` | `ResourceNotFound` | Describes an artifact. |
| `DescribeAutoMLJob` | - | - | `AutoMLJobName` | - | `DescribeAutoMLJobResponse` | `ResourceNotFound` | Returns information about an AutoML job created by calling CreateAutoMLJob. AutoML jobs created by calling CreateAutoMLJobV2 cannot be described by `DescribeAutoMLJob`. |
| `DescribeAutoMLJobV2` | - | - | `AutoMLJobName` | - | `DescribeAutoMLJobV2Response` | `ResourceNotFound` | Returns information about an AutoML job created by calling CreateAutoMLJobV2 or CreateAutoMLJob. |
| `DescribeCluster` | - | - | `ClusterName` | - | `DescribeClusterResponse` | `ResourceNotFound` | Retrieves information of a SageMaker HyperPod cluster. |
| `DescribeClusterEvent` | - | - | `ClusterName`, `EventId` | - | `DescribeClusterEventResponse` | `ResourceNotFound` | Retrieves detailed information about a specific event for a given HyperPod cluster. This functionality is only supported when the `NodeProvisioningMode` is set to `Continuous`. |
| `DescribeClusterNode` | - | - | `ClusterName` | - | `DescribeClusterNodeResponse` | `ResourceNotFound` | Retrieves information of a node (also called a instance interchangeably) of a SageMaker HyperPod cluster. |
| `DescribeClusterSchedulerConfig` | - | - | `ClusterSchedulerConfigId` | - | `DescribeClusterSchedulerConfigResponse` | `ResourceNotFound` | Description of the cluster policy. This policy is used for task prioritization and fair-share allocation. |
| `DescribeCodeRepository` | - | - | `CodeRepositoryName` | - | `DescribeCodeRepositoryOutput` | - | Gets details about the specified Git repository. |
| `DescribeCompilationJob` | - | - | `CompilationJobName` | - | `DescribeCompilationJobResponse` | `ResourceNotFound` | Returns information about a model compilation job. To create a model compilation job, use CreateCompilationJob. |
| `DescribeComputeQuota` | - | - | `ComputeQuotaId` | - | `DescribeComputeQuotaResponse` | `ResourceNotFound` | Description of the compute allocation definition. |
| `DescribeContext` | - | - | `ContextName` | - | `DescribeContextResponse` | `ResourceNotFound` | Describes a context. |
| `DescribeDataQualityJobDefinition` | - | - | `JobDefinitionName` | - | `DescribeDataQualityJobDefinitionResponse` | `ResourceNotFound` | Gets the details of a data quality monitoring job definition. |
| `DescribeDevice` | - | - | `DeviceFleetName`, `DeviceName` | - | `DescribeDeviceResponse` | `ResourceNotFound` | Describes the device. |
| `DescribeDeviceFleet` | - | - | `DeviceFleetName` | - | `DescribeDeviceFleetResponse` | `ResourceNotFound` | A description of the fleet the device belongs to. |
| `DescribeDomain` | - | - | `DomainId` | - | `DescribeDomainResponse` | `ResourceNotFound` | The description of the domain. |
| `DescribeEdgeDeploymentPlan` | - | - | `EdgeDeploymentPlanName` | - | `DescribeEdgeDeploymentPlanResponse` | `ResourceNotFound` | Describes an edge deployment plan with deployment status per stage. |
| `DescribeEdgePackagingJob` | - | - | `EdgePackagingJobName` | - | `DescribeEdgePackagingJobResponse` | `ResourceNotFound` | A description of edge packaging jobs. |
| `DescribeEndpoint` | - | - | `EndpointName` | - | `DescribeEndpointOutput` | - | Returns the description of an endpoint. |
| `DescribeEndpointConfig` | - | - | `EndpointConfigName` | - | `DescribeEndpointConfigOutput` | - | Returns the description of an endpoint configuration created using the `CreateEndpointConfig` API. |
| `DescribeExperiment` | - | - | `ExperimentName` | - | `DescribeExperimentResponse` | `ResourceNotFound` | Provides a list of an experiment's properties. |
| `DescribeFeatureGroup` | - | - | `FeatureGroupName` | - | `DescribeFeatureGroupResponse` | `ResourceNotFound` | Use this operation to describe a `FeatureGroup`. The response includes information on the creation time, `FeatureGroup` name, the unique identifier for each `FeatureGroup`, and more. |
| `DescribeFeatureMetadata` | - | - | `FeatureGroupName`, `FeatureName` | - | `DescribeFeatureMetadataResponse` | `ResourceNotFound` | Shows the metadata for a feature within a feature group. |
| `DescribeFlowDefinition` | - | - | `FlowDefinitionName` | - | `DescribeFlowDefinitionResponse` | `ResourceNotFound` | Returns information about the specified flow definition. |
| `DescribeHub` | - | - | `HubName` | - | `DescribeHubResponse` | `ResourceNotFound` | Describes a hub. |
| `DescribeHubContent` | - | - | `HubContentName`, `HubContentType`, `HubName` | - | `DescribeHubContentResponse` | `ResourceNotFound` | Describe the content of a hub. |
| `DescribeHumanTaskUi` | - | - | `HumanTaskUiName` | - | `DescribeHumanTaskUiResponse` | `ResourceNotFound` | Returns information about the requested human task user interface (worker task template). |
| `DescribeHyperParameterTuningJob` | - | - | `HyperParameterTuningJobName` | - | `DescribeHyperParameterTuningJobResponse` | `ResourceNotFound` | Returns a description of a hyperparameter tuning job, depending on the fields selected. These fields can include the name, Amazon Resource Name (ARN), job status of your tuning job and more. |
| `DescribeImage` | - | - | `ImageName` | - | `DescribeImageResponse` | `ResourceNotFound` | Describes a SageMaker AI image. |
| `DescribeImageVersion` | - | - | `ImageName` | - | `DescribeImageVersionResponse` | `ResourceNotFound` | Describes a version of a SageMaker AI image. |
| `DescribeInferenceComponent` | - | - | `InferenceComponentName` | - | `DescribeInferenceComponentOutput` | - | Returns information about an inference component. |
| `DescribeInferenceExperiment` | - | - | `Name` | - | `DescribeInferenceExperimentResponse` | `ResourceNotFound` | Returns details about an inference experiment. |
| `DescribeInferenceRecommendationsJob` | - | - | `JobName` | - | `DescribeInferenceRecommendationsJobResponse` | `ResourceNotFound` | Provides the results of the Inference Recommender job. One or more recommendation jobs are returned. |
| `DescribeLabelingJob` | - | - | `LabelingJobName` | - | `DescribeLabelingJobResponse` | `ResourceNotFound` | Gets information about a labeling job. |
| `DescribeLineageGroup` | - | - | `LineageGroupName` | - | `DescribeLineageGroupResponse` | `ResourceNotFound` | Provides a list of properties for the requested lineage group. For more information, see Cross-Account Lineage Tracking in the Amazon SageMaker Developer Guide . |
| `DescribeMlflowApp` | - | - | `Arn` | - | `DescribeMlflowAppResponse` | `ResourceNotFound` | Returns information about an MLflow App. |
| `DescribeMlflowTrackingServer` | - | - | `TrackingServerName` | - | `DescribeMlflowTrackingServerResponse` | `ResourceNotFound` | Returns information about an MLflow Tracking Server. |
| `DescribeModel` | - | - | `ModelName` | - | `DescribeModelOutput` | - | Describes a model that you created using the `CreateModel` API. |
| `DescribeModelBiasJobDefinition` | - | - | `JobDefinitionName` | - | `DescribeModelBiasJobDefinitionResponse` | `ResourceNotFound` | Returns a description of a model bias job definition. |
| `DescribeModelCard` | - | - | `ModelCardName` | - | `DescribeModelCardResponse` | `ResourceNotFound` | Describes the content, creation time, and security configuration of an Amazon SageMaker Model Card. |
| `DescribeModelCardExportJob` | - | - | `ModelCardExportJobArn` | - | `DescribeModelCardExportJobResponse` | `ResourceNotFound` | Describes an Amazon SageMaker Model Card export job. |
| `DescribeModelExplainabilityJobDefinition` | - | - | `JobDefinitionName` | - | `DescribeModelExplainabilityJobDefinitionResponse` | `ResourceNotFound` | Returns a description of a model explainability job definition. |
| `DescribeModelPackage` | - | - | `ModelPackageName` | - | `DescribeModelPackageOutput` | - | Returns a description of the specified model package, which is used to create SageMaker models or list them on Amazon Web Services Marketplace. If you provided a KMS Key ID when you created your model package, you will see the KMS Decrypt API call in your... |
| `DescribeModelPackageGroup` | - | - | `ModelPackageGroupName` | - | `DescribeModelPackageGroupOutput` | - | Gets a description for the specified model group. |
| `DescribeModelQualityJobDefinition` | - | - | `JobDefinitionName` | - | `DescribeModelQualityJobDefinitionResponse` | `ResourceNotFound` | Returns a description of a model quality job definition. |
| `DescribeMonitoringSchedule` | - | - | `MonitoringScheduleName` | - | `DescribeMonitoringScheduleResponse` | `ResourceNotFound` | Describes the schedule for a monitoring job. |
| `DescribeNotebookInstance` | - | - | `NotebookInstanceName` | - | `DescribeNotebookInstanceOutput` | - | Returns information about a notebook instance. |
| `DescribeNotebookInstanceLifecycleConfig` | - | - | `NotebookInstanceLifecycleConfigName` | - | `DescribeNotebookInstanceLifecycleConfigOutput` | - | Returns a description of a notebook instance lifecycle configuration. For information about notebook instance lifestyle configurations, see Step 2.1: (Optional) Customize a Notebook Instance. |
| `DescribeOptimizationJob` | - | - | `OptimizationJobName` | - | `DescribeOptimizationJobResponse` | `ResourceNotFound` | Provides the properties of the specified optimization job. |
| `DescribePartnerApp` | - | - | `Arn` | - | `DescribePartnerAppResponse` | `ResourceNotFound` | Gets information about a SageMaker Partner AI App. |
| `DescribePipeline` | - | - | `PipelineName` | - | `DescribePipelineResponse` | `ResourceNotFound` | Describes the details of a pipeline. |
| `DescribePipelineDefinitionForExecution` | - | - | `PipelineExecutionArn` | - | `DescribePipelineDefinitionForExecutionResponse` | `ResourceNotFound` | Describes the details of an execution's pipeline definition. |
| `DescribePipelineExecution` | - | - | `PipelineExecutionArn` | - | `DescribePipelineExecutionResponse` | `ResourceNotFound` | Describes the details of a pipeline execution. |
| `DescribeProcessingJob` | - | - | `ProcessingJobName` | - | `DescribeProcessingJobResponse` | `ResourceNotFound` | Returns a description of a processing job. |
| `DescribeProject` | - | - | `ProjectName` | - | `DescribeProjectOutput` | - | Describes the details of a project. |
| `DescribeReservedCapacity` | - | - | `ReservedCapacityArn` | - | `DescribeReservedCapacityResponse` | `ResourceNotFound` | Retrieves details about a reserved capacity. |
| `DescribeSpace` | - | - | `DomainId`, `SpaceName` | - | `DescribeSpaceResponse` | `ResourceNotFound` | Describes the space. |
| `DescribeStudioLifecycleConfig` | - | - | `StudioLifecycleConfigName` | - | `DescribeStudioLifecycleConfigResponse` | `ResourceNotFound` | Describes the Amazon SageMaker AI Studio Lifecycle Configuration. |
| `DescribeSubscribedWorkteam` | - | - | `WorkteamArn` | - | `DescribeSubscribedWorkteamResponse` | - | Gets information about a work team provided by a vendor. It returns details about the subscription with a vendor in the Amazon Web Services Marketplace. |
| `DescribeTrainingJob` | - | - | `TrainingJobName` | - | `DescribeTrainingJobResponse` | `ResourceNotFound` | Returns information about a training job. Some of the attributes below only appear if the training job successfully starts. |
| `DescribeTrainingPlan` | - | - | `TrainingPlanName` | - | `DescribeTrainingPlanResponse` | `ResourceNotFound` | Retrieves detailed information about a specific training plan. |
| `DescribeTrainingPlanExtensionHistory` | - | `paginated` | `TrainingPlanArn` | - | `DescribeTrainingPlanExtensionHistoryResponse` | `ResourceNotFound` | Retrieves the extension history for a specified training plan. The response includes details about each extension, such as the offering ID, start and end dates, status, payment status, and cost information. |
| `DescribeTransformJob` | - | - | `TransformJobName` | - | `DescribeTransformJobResponse` | `ResourceNotFound` | Returns information about a transform job. |
| `DescribeTrial` | - | - | `TrialName` | - | `DescribeTrialResponse` | `ResourceNotFound` | Provides a list of a trial's properties. |
| `DescribeTrialComponent` | - | - | `TrialComponentName` | - | `DescribeTrialComponentResponse` | `ResourceNotFound` | Provides a list of a trials component's properties. |
| `DescribeUserProfile` | - | - | `DomainId`, `UserProfileName` | - | `DescribeUserProfileResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Describes a user profile. For more information, see `CreateUserProfile`. |
| `DescribeWorkforce` | - | - | `WorkforceName` | - | `DescribeWorkforceResponse` | - | Lists private workforce information, including workforce name, Amazon Resource Name (ARN), and, if applicable, allowed IP address ranges (CIDRs). Allowable IP address ranges are the IP addresses that workers can use to access tasks. |
| `DescribeWorkteam` | - | - | `WorkteamName` | - | `DescribeWorkteamResponse` | - | Gets information about a specific work team. You can see information such as the creation date, the last updated date, membership information, and the work team's Amazon Resource Name (ARN). |
| `DetachClusterNodeVolume` | - | - | `ClusterArn`, `NodeId`, `VolumeId` | - | `DetachClusterNodeVolumeResponse` | `ResourceNotFound` | Detaches your Amazon Elastic Block Store (Amazon EBS) volume from a node in your EKS orchestrated SageMaker HyperPod cluster. This API works with the Amazon Elastic Block Store (Amazon EBS) Container Storage Interface (CSI) driver to manage the lifecycle of... |
| `DisableSagemakerServicecatalogPortfolio` | - | - | - | - | `DisableSagemakerServicecatalogPortfolioOutput` | - | Disables using Service Catalog in SageMaker. Service Catalog is used to create SageMaker projects. |
| `DisassociateTrialComponent` | - | - | `TrialComponentName`, `TrialName` | - | `DisassociateTrialComponentResponse` | `ResourceNotFound` | Disassociates a trial component from a trial. This doesn't effect other trials the component is associated with. |
| `EnableSagemakerServicecatalogPortfolio` | - | - | - | - | `EnableSagemakerServicecatalogPortfolioOutput` | - | Enables using Service Catalog in SageMaker. Service Catalog is used to create SageMaker projects. |
| `ExtendTrainingPlan` | - | - | `TrainingPlanExtensionOfferingId` | - | `ExtendTrainingPlanResponse` | `ResourceNotFound` | Extends an existing training plan by purchasing an extension offering. This allows you to add additional compute capacity time to your training plan without creating a new plan or reconfiguring your workloads. |
| `GetDeviceFleetReport` | - | - | `DeviceFleetName` | - | `GetDeviceFleetReportResponse` | - | Describes a fleet. |
| `GetLineageGroupPolicy` | - | - | `LineageGroupName` | - | `GetLineageGroupPolicyResponse` | `ResourceNotFound` | The resource policy for the lineage group. |
| `GetModelPackageGroupPolicy` | - | - | `ModelPackageGroupName` | - | `GetModelPackageGroupPolicyOutput` | - | Gets a resource policy that manages access for a model group. For information about resource policies, see Identity-based policies and resource-based policies in the Amazon Web Services Identity and Access Management User Guide. |
| `GetSagemakerServicecatalogPortfolioStatus` | - | - | - | - | `GetSagemakerServicecatalogPortfolioStatusOutput` | - | Gets the status of Service Catalog in SageMaker. Service Catalog is used to create SageMaker projects. |
| `GetScalingConfigurationRecommendation` | - | - | `InferenceRecommendationsJobName` | - | `GetScalingConfigurationRecommendationResponse` | `ResourceNotFound` | Starts an Amazon SageMaker Inference Recommender autoscaling recommendation job. Returns recommendations for autoscaling policies that you can apply to your SageMaker endpoint. |
| `GetSearchSuggestions` | - | - | `Resource` | - | `GetSearchSuggestionsResponse` | - | An auto-complete API for the search functionality in the SageMaker console. It returns suggestions of possible matches for the property name to use in `Search` queries. |
| `ImportHubContent` | - | - | `DocumentSchemaVersion`, `HubContentDocument`, `HubContentName`, `HubContentType`, `HubName` | - | `ImportHubContentResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Import hub content. |
| `ListActions` | - | `paginated` | - | - | `ListActionsResponse` | `ResourceNotFound` | Lists the actions in your account and their properties. |
| `ListAlgorithms` | - | `paginated` | - | - | `ListAlgorithmsOutput` | - | Lists the machine learning algorithms that have been created. |
| `ListAliases` | - | `paginated` | `ImageName` | - | `ListAliasesResponse` | `ResourceNotFound` | Lists the aliases of a specified image or image version. |
| `ListAppImageConfigs` | - | `paginated` | - | - | `ListAppImageConfigsResponse` | - | Lists the AppImageConfigs in your account and their properties. The list can be filtered by creation time or modified time, and whether the AppImageConfig name contains a specified string. |
| `ListApps` | - | `paginated` | - | - | `ListAppsResponse` | - | Lists apps. |
| `ListArtifacts` | - | `paginated` | - | - | `ListArtifactsResponse` | `ResourceNotFound` | Lists the artifacts in your account and their properties. |
| `ListAssociations` | - | `paginated` | - | - | `ListAssociationsResponse` | `ResourceNotFound` | Lists the associations in your account and their properties. |
| `ListAutoMLJobs` | - | `paginated` | - | - | `ListAutoMLJobsResponse` | - | Request a list of jobs. |
| `ListCandidatesForAutoMLJob` | - | `paginated` | `AutoMLJobName` | - | `ListCandidatesForAutoMLJobResponse` | `ResourceNotFound` | List the candidates created for the job. |
| `ListClusterEvents` | - | `paginated` | `ClusterName` | - | `ListClusterEventsResponse` | `ResourceNotFound` | Retrieves a list of event summaries for a specified HyperPod cluster. The operation supports filtering, sorting, and pagination of results. |
| `ListClusterNodes` | - | `paginated` | `ClusterName` | - | `ListClusterNodesResponse` | `ResourceNotFound` | Retrieves the list of instances (also called nodes interchangeably) in a SageMaker HyperPod cluster. |
| `ListClusterSchedulerConfigs` | - | `paginated` | - | - | `ListClusterSchedulerConfigsResponse` | - | List the cluster policy configurations. |
| `ListClusters` | - | `paginated` | - | - | `ListClustersResponse` | - | Retrieves the list of SageMaker HyperPod clusters. |
| `ListCodeRepositories` | - | `paginated` | - | - | `ListCodeRepositoriesOutput` | - | Gets a list of the Git repositories in your account. |
| `ListCompilationJobs` | - | `paginated` | - | - | `ListCompilationJobsResponse` | - | Lists model compilation jobs that satisfy various filters. To create a model compilation job, use CreateCompilationJob. |
| `ListComputeQuotas` | - | `paginated` | - | - | `ListComputeQuotasResponse` | - | List the resource allocation definitions. |
| `ListContexts` | - | `paginated` | - | - | `ListContextsResponse` | `ResourceNotFound` | Lists the contexts in your account and their properties. |
| `ListDataQualityJobDefinitions` | - | `paginated` | - | - | `ListDataQualityJobDefinitionsResponse` | - | Lists the data quality job definitions in your account. |
| `ListDeviceFleets` | - | `paginated` | - | - | `ListDeviceFleetsResponse` | - | Returns a list of devices in the fleet. |
| `ListDevices` | - | `paginated` | - | - | `ListDevicesResponse` | - | A list of devices. |
| `ListDomains` | - | `paginated` | - | - | `ListDomainsResponse` | - | Lists the domains. |
| `ListEdgeDeploymentPlans` | - | `paginated` | - | - | `ListEdgeDeploymentPlansResponse` | - | Lists all edge deployment plans. |
| `ListEdgePackagingJobs` | - | `paginated` | - | - | `ListEdgePackagingJobsResponse` | - | Returns a list of edge packaging jobs. |
| `ListEndpointConfigs` | - | `paginated` | - | - | `ListEndpointConfigsOutput` | - | Lists endpoint configurations. |
| `ListEndpoints` | - | `paginated` | - | - | `ListEndpointsOutput` | - | Lists endpoints. |
| `ListExperiments` | - | `paginated` | - | - | `ListExperimentsResponse` | - | Lists all the experiments in your account. The list can be filtered to show only experiments that were created in a specific time range. |
| `ListFeatureGroups` | - | `paginated` | - | - | `ListFeatureGroupsResponse` | - | List `FeatureGroup`s based on given filter and order. |
| `ListFlowDefinitions` | - | `paginated` | - | - | `ListFlowDefinitionsResponse` | - | Returns information about the flow definitions in your account. |
| `ListHubContentVersions` | - | - | `HubContentName`, `HubContentType`, `HubName` | - | `ListHubContentVersionsResponse` | `ResourceNotFound` | List hub content versions. |
| `ListHubContents` | - | - | `HubContentType`, `HubName` | - | `ListHubContentsResponse` | `ResourceNotFound` | List the contents of a hub. |
| `ListHubs` | - | - | - | - | `ListHubsResponse` | - | List all existing hubs. |
| `ListHumanTaskUis` | - | `paginated` | - | - | `ListHumanTaskUisResponse` | - | Returns information about the human task user interfaces in your account. |
| `ListHyperParameterTuningJobs` | - | `paginated` | - | - | `ListHyperParameterTuningJobsResponse` | - | Gets a list of HyperParameterTuningJobSummary objects that describe the hyperparameter tuning jobs launched in your account. |
| `ListImageVersions` | - | `paginated` | `ImageName` | - | `ListImageVersionsResponse` | `ResourceNotFound` | Lists the versions of a specified image and their properties. The list can be filtered by creation time or modified time. |
| `ListImages` | - | `paginated` | - | - | `ListImagesResponse` | - | Lists the images in your account and their properties. The list can be filtered by creation time or modified time, and whether the image name contains a specified string. |
| `ListInferenceComponents` | - | `paginated` | - | - | `ListInferenceComponentsOutput` | - | Lists the inference components in your account and their properties. |
| `ListInferenceExperiments` | - | `paginated` | - | - | `ListInferenceExperimentsResponse` | - | Returns the list of all inference experiments. |
| `ListInferenceRecommendationsJobSteps` | - | `paginated` | `JobName` | - | `ListInferenceRecommendationsJobStepsResponse` | `ResourceNotFound` | Returns a list of the subtasks for an Inference Recommender job. The supported subtasks are benchmarks, which evaluate the performance of your model on different instance types. |
| `ListInferenceRecommendationsJobs` | - | `paginated` | - | - | `ListInferenceRecommendationsJobsResponse` | - | Lists recommendation jobs that satisfy various filters. |
| `ListLabelingJobs` | - | `paginated` | - | - | `ListLabelingJobsResponse` | - | Gets a list of labeling jobs. |
| `ListLabelingJobsForWorkteam` | - | `paginated` | `WorkteamArn` | - | `ListLabelingJobsForWorkteamResponse` | `ResourceNotFound` | Gets a list of labeling jobs assigned to a specified work team. |
| `ListLineageGroups` | - | `paginated` | - | - | `ListLineageGroupsResponse` | - | A list of lineage groups shared with your Amazon Web Services account. For more information, see Cross-Account Lineage Tracking in the Amazon SageMaker Developer Guide . |
| `ListMlflowApps` | - | `paginated` | - | - | `ListMlflowAppsResponse` | - | Lists all MLflow Apps |
| `ListMlflowTrackingServers` | - | `paginated` | - | - | `ListMlflowTrackingServersResponse` | - | Lists all MLflow Tracking Servers. |
| `ListModelBiasJobDefinitions` | - | `paginated` | - | - | `ListModelBiasJobDefinitionsResponse` | - | Lists model bias jobs definitions that satisfy various filters. |
| `ListModelCardExportJobs` | - | `paginated` | `ModelCardName` | - | `ListModelCardExportJobsResponse` | - | List the export jobs for the Amazon SageMaker Model Card. |
| `ListModelCardVersions` | - | `paginated` | `ModelCardName` | - | `ListModelCardVersionsResponse` | `ResourceNotFound` | List existing versions of an Amazon SageMaker Model Card. |
| `ListModelCards` | - | `paginated` | - | - | `ListModelCardsResponse` | - | List existing model cards. |
| `ListModelExplainabilityJobDefinitions` | - | `paginated` | - | - | `ListModelExplainabilityJobDefinitionsResponse` | - | Lists model explainability job definitions that satisfy various filters. |
| `ListModelMetadata` | - | `paginated` | - | - | `ListModelMetadataResponse` | - | Lists the domain, framework, task, and model name of standard machine learning models found in common model zoos. |
| `ListModelPackageGroups` | - | `paginated` | - | - | `ListModelPackageGroupsOutput` | - | Gets a list of the model groups in your Amazon Web Services account. |
| `ListModelPackages` | - | `paginated` | - | - | `ListModelPackagesOutput` | - | Lists the model packages that have been created. |
| `ListModelQualityJobDefinitions` | - | `paginated` | - | - | `ListModelQualityJobDefinitionsResponse` | - | Gets a list of model quality monitoring job definitions in your account. |
| `ListModels` | - | `paginated` | - | - | `ListModelsOutput` | - | Lists models created with the `CreateModel` API. |
| `ListMonitoringAlertHistory` | - | `paginated` | - | - | `ListMonitoringAlertHistoryResponse` | `ResourceNotFound` | Gets a list of past alerts in a model monitoring schedule. |
| `ListMonitoringAlerts` | - | `paginated` | `MonitoringScheduleName` | - | `ListMonitoringAlertsResponse` | `ResourceNotFound` | Gets the alerts for a single monitoring schedule. |
| `ListMonitoringExecutions` | - | `paginated` | - | - | `ListMonitoringExecutionsResponse` | - | Returns list of all monitoring job executions. |
| `ListMonitoringSchedules` | - | `paginated` | - | - | `ListMonitoringSchedulesResponse` | - | Returns list of all monitoring schedules. |
| `ListNotebookInstanceLifecycleConfigs` | - | `paginated` | - | - | `ListNotebookInstanceLifecycleConfigsOutput` | - | Lists notebook instance lifestyle configurations created with the CreateNotebookInstanceLifecycleConfig API. |
| `ListNotebookInstances` | - | `paginated` | - | - | `ListNotebookInstancesOutput` | - | Returns a list of the SageMaker AI notebook instances in the requester's account in an Amazon Web Services Region. |
| `ListOptimizationJobs` | - | `paginated` | - | - | `ListOptimizationJobsResponse` | - | Lists the optimization jobs in your account and their properties. |
| `ListPartnerApps` | - | `paginated` | - | - | `ListPartnerAppsResponse` | - | Lists all of the SageMaker Partner AI Apps in an account. |
| `ListPipelineExecutionSteps` | - | `paginated` | - | - | `ListPipelineExecutionStepsResponse` | `ResourceNotFound` | Gets a list of `PipeLineExecutionStep` objects. |
| `ListPipelineExecutions` | - | `paginated` | `PipelineName` | - | `ListPipelineExecutionsResponse` | `ResourceNotFound` | Gets a list of the pipeline executions. |
| `ListPipelineParametersForExecution` | - | `paginated` | `PipelineExecutionArn` | - | `ListPipelineParametersForExecutionResponse` | `ResourceNotFound` | Gets a list of parameters for a pipeline execution. |
| `ListPipelineVersions` | - | `paginated` | `PipelineName` | - | `ListPipelineVersionsResponse` | `ResourceNotFound` | Gets a list of all versions of the pipeline. |
| `ListPipelines` | - | `paginated` | - | - | `ListPipelinesResponse` | - | Gets a list of pipelines. |
| `ListProcessingJobs` | - | `paginated` | - | - | `ListProcessingJobsResponse` | - | Lists processing jobs that satisfy various filters. |
| `ListProjects` | - | `paginated` | - | - | `ListProjectsOutput` | - | Gets a list of the projects in an Amazon Web Services account. |
| `ListResourceCatalogs` | - | `paginated` | - | - | `ListResourceCatalogsResponse` | - | Lists Amazon SageMaker Catalogs based on given filters and orders. The maximum number of `ResourceCatalog`s viewable is 1000. |
| `ListSpaces` | - | `paginated` | - | - | `ListSpacesResponse` | - | Lists spaces. |
| `ListStageDevices` | - | `paginated` | `EdgeDeploymentPlanName`, `StageName` | - | `ListStageDevicesResponse` | - | Lists devices allocated to the stage, containing detailed device information and deployment status. |
| `ListStudioLifecycleConfigs` | - | `paginated` | - | - | `ListStudioLifecycleConfigsResponse` | `ResourceInUse` | Lists the Amazon SageMaker AI Studio Lifecycle Configurations in your Amazon Web Services Account. |
| `ListSubscribedWorkteams` | - | `paginated` | - | - | `ListSubscribedWorkteamsResponse` | - | Gets a list of the work teams that you are subscribed to in the Amazon Web Services Marketplace. The list may be empty if no work team satisfies the filter specified in the `NameContains` parameter. |
| `ListTags` | - | `paginated` | `ResourceArn` | - | `ListTagsOutput` | - | Returns the tags for the specified SageMaker resource. |
| `ListTrainingJobs` | - | `paginated` | - | - | `ListTrainingJobsResponse` | - | Lists training jobs. When `StatusEquals` and `MaxResults` are set at the same time, the `MaxResults` number of training jobs are first retrieved ignoring the `StatusEquals` parameter and then they are filtered by the `StatusEquals` parameter, which is... |
| `ListTrainingJobsForHyperParameterTuningJob` | - | `paginated` | `HyperParameterTuningJobName` | - | `ListTrainingJobsForHyperParameterTuningJobResponse` | `ResourceNotFound` | Gets a list of TrainingJobSummary objects that describe the training jobs that a hyperparameter tuning job launched. |
| `ListTrainingPlans` | - | `paginated` | - | - | `ListTrainingPlansResponse` | - | Retrieves a list of training plans for the current account. |
| `ListTransformJobs` | - | `paginated` | - | - | `ListTransformJobsResponse` | - | Lists transform jobs. |
| `ListTrialComponents` | - | `paginated` | - | - | `ListTrialComponentsResponse` | `ResourceNotFound` | Lists the trial components in your account. You can sort the list by trial component name or creation time. |
| `ListTrials` | - | `paginated` | - | - | `ListTrialsResponse` | `ResourceNotFound` | Lists the trials in your account. Specify an experiment name to limit the list to the trials that are part of that experiment. |
| `ListUltraServersByReservedCapacity` | - | `paginated` | `ReservedCapacityArn` | - | `ListUltraServersByReservedCapacityResponse` | `ResourceNotFound` | Lists all UltraServers that are part of a specified reserved capacity. |
| `ListUserProfiles` | - | `paginated` | - | - | `ListUserProfilesResponse` | - | Lists user profiles. |
| `ListWorkforces` | - | `paginated` | - | - | `ListWorkforcesResponse` | - | Use this operation to list all private and vendor workforces in an Amazon Web Services Region. Note that you can only have one private workforce per Amazon Web Services Region. |
| `ListWorkteams` | - | `paginated` | - | - | `ListWorkteamsResponse` | - | Gets a list of private work teams that you have defined in a region. The list may be empty if no work team satisfies the filter specified in the `NameContains` parameter. |
| `PutModelPackageGroupPolicy` | - | - | `ModelPackageGroupName`, `ResourcePolicy` | - | `PutModelPackageGroupPolicyOutput` | `ConflictException` | Adds a resouce policy to control access to a model group. For information about resoure policies, see Identity-based policies and resource-based policies in the Amazon Web Services Identity and Access Management User Guide. |
| `QueryLineage` | - | `paginated` | - | - | `QueryLineageResponse` | `ResourceNotFound` | Use this action to inspect your lineage and discover relationships between entities. For more information, see Querying Lineage Entities in the Amazon SageMaker Developer Guide . |
| `RegisterDevices` | - | - | `DeviceFleetName`, `Devices` | - | `Unit` | `ResourceLimitExceeded` | Register devices. |
| `RenderUiTemplate` | - | - | `RoleArn`, `Task` | - | `RenderUiTemplateResponse` | `ResourceNotFound` | Renders the UI template so that you can preview the worker's experience. |
| `RetryPipelineExecution` | - | `idempotency-token` | `ClientRequestToken`, `PipelineExecutionArn` | `ClientRequestToken` | `RetryPipelineExecutionResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Retry the execution of the pipeline. |
| `Search` | - | `paginated` | `Resource` | - | `SearchResponse` | - | Finds SageMaker resources that match a search query. Matching resources are returned as a list of `SearchRecord` objects in the response. |
| `SearchTrainingPlanOfferings` | - | - | - | - | `SearchTrainingPlanOfferingsResponse` | `ResourceLimitExceeded` | Searches for available training plan offerings based on specified criteria. Users search for available plan offerings based on their requirements (e.g., instance type, count, start time, duration). |
| `SendPipelineExecutionStepFailure` | - | `idempotency-token` | `CallbackToken` | `ClientRequestToken` | `SendPipelineExecutionStepFailureResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Notifies the pipeline that the execution of a callback step failed, along with a message describing why. When a callback step is run, the pipeline generates a callback token and includes the token in a message sent to Amazon Simple Queue Service (Amazon SQS). |
| `SendPipelineExecutionStepSuccess` | - | `idempotency-token` | `CallbackToken` | `ClientRequestToken` | `SendPipelineExecutionStepSuccessResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Notifies the pipeline that the execution of a callback step succeeded and provides a list of the step's output parameters. When a callback step is run, the pipeline generates a callback token and includes the token in a message sent to Amazon Simple Queue... |
| `StartEdgeDeploymentStage` | - | - | `EdgeDeploymentPlanName`, `StageName` | - | `Unit` | - | Starts a stage in an edge deployment plan. |
| `StartInferenceExperiment` | - | - | `Name` | - | `StartInferenceExperimentResponse` | `ConflictException`, `ResourceNotFound` | Starts an inference experiment. |
| `StartMlflowTrackingServer` | - | - | `TrackingServerName` | - | `StartMlflowTrackingServerResponse` | `ConflictException`, `ResourceNotFound` | Programmatically start an MLflow Tracking Server. |
| `StartMonitoringSchedule` | - | - | `MonitoringScheduleName` | - | `Unit` | `ResourceNotFound` | Starts a previously stopped monitoring schedule. By default, when you successfully create a new schedule, the status of a monitoring schedule is `scheduled`. |
| `StartNotebookInstance` | - | - | `NotebookInstanceName` | - | `Unit` | `ResourceLimitExceeded` | Launches an ML compute instance with the latest version of the libraries and attaches your ML storage volume. After configuring the notebook instance, SageMaker AI sets the notebook instance status to `InService`. |
| `StartPipelineExecution` | - | `idempotency-token` | `ClientRequestToken`, `PipelineName` | `ClientRequestToken` | `StartPipelineExecutionResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Starts a pipeline execution. |
| `StartSession` | - | - | `ResourceIdentifier` | - | `StartSessionResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Initiates a remote connection session between a local integrated development environments (IDEs) and a remote SageMaker space. |
| `StopAutoMLJob` | - | - | `AutoMLJobName` | - | `Unit` | `ResourceNotFound` | A method for forcing a running job to shut down. |
| `StopCompilationJob` | - | - | `CompilationJobName` | - | `Unit` | `ResourceNotFound` | Stops a model compilation job. To stop a job, Amazon SageMaker AI sends the algorithm the SIGTERM signal. |
| `StopEdgeDeploymentStage` | - | - | `EdgeDeploymentPlanName`, `StageName` | - | `Unit` | - | Stops a stage in an edge deployment plan. |
| `StopEdgePackagingJob` | - | - | `EdgePackagingJobName` | - | `Unit` | - | Request to stop an edge packaging job. |
| `StopHyperParameterTuningJob` | - | - | `HyperParameterTuningJobName` | - | `Unit` | `ResourceNotFound` | Stops a running hyperparameter tuning job and all running training jobs that the tuning job launched. All model artifacts output from the training jobs are stored in Amazon Simple Storage Service (Amazon S3). |
| `StopInferenceExperiment` | - | - | `ModelVariantActions`, `Name` | - | `StopInferenceExperimentResponse` | `ConflictException`, `ResourceNotFound` | Stops an inference experiment. |
| `StopInferenceRecommendationsJob` | - | - | `JobName` | - | `Unit` | `ResourceNotFound` | Stops an Inference Recommender job. |
| `StopLabelingJob` | - | - | `LabelingJobName` | - | `Unit` | `ResourceNotFound` | Stops a running labeling job. A job that is stopped cannot be restarted. |
| `StopMlflowTrackingServer` | - | - | `TrackingServerName` | - | `StopMlflowTrackingServerResponse` | `ConflictException`, `ResourceNotFound` | Programmatically stop an MLflow Tracking Server. |
| `StopMonitoringSchedule` | - | - | `MonitoringScheduleName` | - | `Unit` | `ResourceNotFound` | Stops a previously started monitoring schedule. |
| `StopNotebookInstance` | - | - | `NotebookInstanceName` | - | `Unit` | - | Terminates the ML compute instance. Before terminating the instance, SageMaker AI disconnects the ML storage volume from it. |
| `StopOptimizationJob` | - | - | `OptimizationJobName` | - | `Unit` | `ResourceNotFound` | Ends a running inference optimization job. |
| `StopPipelineExecution` | - | `idempotency-token` | `ClientRequestToken`, `PipelineExecutionArn` | `ClientRequestToken` | `StopPipelineExecutionResponse` | `ConflictException`, `ResourceNotFound` | Stops a pipeline execution. Callback Step A pipeline execution won't stop while a callback step is running. |
| `StopProcessingJob` | - | - | `ProcessingJobName` | - | `Unit` | `ResourceNotFound` | Stops a processing job. |
| `StopTrainingJob` | - | - | `TrainingJobName` | - | `Unit` | `ResourceNotFound` | Stops a training job. To stop a job, SageMaker sends the algorithm the `SIGTERM` signal, which delays job termination for 120 seconds. |
| `StopTransformJob` | - | - | `TransformJobName` | - | `Unit` | `ResourceNotFound` | Stops a batch transform job. When Amazon SageMaker receives a `StopTransformJob` request, the status of the job changes to `Stopping`. |
| `UpdateAction` | - | - | `ActionName` | - | `UpdateActionResponse` | `ConflictException`, `ResourceNotFound` | Updates an action. |
| `UpdateAppImageConfig` | - | - | `AppImageConfigName` | - | `UpdateAppImageConfigResponse` | `ResourceNotFound` | Updates the properties of an AppImageConfig. |
| `UpdateArtifact` | - | - | `ArtifactArn` | - | `UpdateArtifactResponse` | `ConflictException`, `ResourceNotFound` | Updates an artifact. |
| `UpdateCluster` | - | - | `ClusterName` | - | `UpdateClusterResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Updates a SageMaker HyperPod cluster. |
| `UpdateClusterSchedulerConfig` | - | - | `ClusterSchedulerConfigId`, `TargetVersion` | - | `UpdateClusterSchedulerConfigResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Update the cluster policy configuration. |
| `UpdateClusterSoftware` | - | - | `ClusterName` | - | `UpdateClusterSoftwareResponse` | `ConflictException`, `ResourceNotFound` | Updates the platform software of a SageMaker HyperPod cluster for security patching. To learn how to use this API, see Update the SageMaker HyperPod platform software of a cluster. |
| `UpdateCodeRepository` | - | - | `CodeRepositoryName` | - | `UpdateCodeRepositoryOutput` | `ConflictException` | Updates the specified Git repository with the specified values. |
| `UpdateComputeQuota` | - | - | `ComputeQuotaId`, `TargetVersion` | - | `UpdateComputeQuotaResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Update the compute allocation definition. |
| `UpdateContext` | - | - | `ContextName` | - | `UpdateContextResponse` | `ConflictException`, `ResourceNotFound` | Updates a context. |
| `UpdateDeviceFleet` | - | - | `DeviceFleetName`, `OutputConfig` | - | `Unit` | `ResourceInUse` | Updates a fleet of devices. |
| `UpdateDevices` | - | - | `DeviceFleetName`, `Devices` | - | `Unit` | - | Updates one or more devices in a fleet. |
| `UpdateDomain` | - | - | `DomainId` | - | `UpdateDomainResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Updates the default settings for new user profiles in the domain. |
| `UpdateEndpoint` | - | - | `EndpointConfigName`, `EndpointName` | - | `UpdateEndpointOutput` | `ResourceLimitExceeded` | Deploys the `EndpointConfig` specified in the request to a new fleet of instances. SageMaker shifts endpoint traffic to the new instances with the updated endpoint configuration and then deletes the old instances using the previous `EndpointConfig` (there is... |
| `UpdateEndpointWeightsAndCapacities` | - | - | `DesiredWeightsAndCapacities`, `EndpointName` | - | `UpdateEndpointWeightsAndCapacitiesOutput` | `ResourceLimitExceeded` | Updates variant weight of one or more variants associated with an existing endpoint, or capacity of one variant associated with an existing endpoint. When it receives the request, SageMaker sets the endpoint status to `Updating`. |
| `UpdateExperiment` | - | - | `ExperimentName` | - | `UpdateExperimentResponse` | `ConflictException`, `ResourceNotFound` | Adds, updates, or removes the description of an experiment. Updates the display name of an experiment. |
| `UpdateFeatureGroup` | - | - | `FeatureGroupName` | - | `UpdateFeatureGroupResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Updates the feature group by either adding features or updating the online store configuration. Use one of the following request parameters at a time while using the `UpdateFeatureGroup` API. |
| `UpdateFeatureMetadata` | - | - | `FeatureGroupName`, `FeatureName` | - | `Unit` | `ResourceNotFound` | Updates the description and parameters of the feature group. |
| `UpdateHub` | - | - | `HubName` | - | `UpdateHubResponse` | `ResourceNotFound` | Update a hub. |
| `UpdateHubContent` | - | - | `HubContentName`, `HubContentType`, `HubContentVersion`, `HubName` | - | `UpdateHubContentResponse` | `ResourceInUse`, `ResourceNotFound` | Updates SageMaker hub content (either a `Model` or `Notebook` resource). You can update the metadata that describes the resource. |
| `UpdateHubContentReference` | - | - | `HubContentName`, `HubContentType`, `HubName` | - | `UpdateHubContentReferenceResponse` | `ResourceInUse`, `ResourceNotFound` | Updates the contents of a SageMaker hub for a `ModelReference` resource. A `ModelReference` allows you to access public SageMaker JumpStart models from within your private hub. |
| `UpdateImage` | - | - | `ImageName` | - | `UpdateImageResponse` | `ResourceInUse`, `ResourceNotFound` | Updates the properties of a SageMaker AI image. To change the image's tags, use the AddTags and DeleteTags APIs. |
| `UpdateImageVersion` | - | - | `ImageName` | - | `UpdateImageVersionResponse` | `ResourceInUse`, `ResourceNotFound` | Updates the properties of a SageMaker AI image version. |
| `UpdateInferenceComponent` | - | - | `InferenceComponentName` | - | `UpdateInferenceComponentOutput` | `ResourceLimitExceeded` | Updates an inference component. |
| `UpdateInferenceComponentRuntimeConfig` | - | - | `DesiredRuntimeConfig`, `InferenceComponentName` | - | `UpdateInferenceComponentRuntimeConfigOutput` | `ResourceLimitExceeded` | Runtime settings for a model that is deployed with an inference component. |
| `UpdateInferenceExperiment` | - | - | `Name` | - | `UpdateInferenceExperimentResponse` | `ConflictException`, `ResourceNotFound` | Updates an inference experiment that you created. The status of the inference experiment has to be either `Created`, `Running`. |
| `UpdateMlflowApp` | - | - | `Arn` | - | `UpdateMlflowAppResponse` | `ConflictException`, `ResourceNotFound` | Updates an MLflow App. |
| `UpdateMlflowTrackingServer` | - | - | `TrackingServerName` | - | `UpdateMlflowTrackingServerResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Updates properties of an existing MLflow Tracking Server. |
| `UpdateModelCard` | - | - | `ModelCardName` | - | `UpdateModelCardResponse` | `ConflictException`, `ResourceLimitExceeded`, `ResourceNotFound` | Update an Amazon SageMaker Model Card. You cannot update both model card content and model card status in a single call. |
| `UpdateModelPackage` | - | - | `ModelPackageArn` | - | `UpdateModelPackageOutput` | `ConflictException` | Updates a versioned model. |
| `UpdateMonitoringAlert` | - | - | `DatapointsToAlert`, `EvaluationPeriod`, `MonitoringAlertName`, `MonitoringScheduleName` | - | `UpdateMonitoringAlertResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Update the parameters of a model monitor alert. |
| `UpdateMonitoringSchedule` | - | - | `MonitoringScheduleConfig`, `MonitoringScheduleName` | - | `UpdateMonitoringScheduleResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Updates a previously created schedule. |
| `UpdateNotebookInstance` | - | - | `NotebookInstanceName` | - | `UpdateNotebookInstanceOutput` | `ResourceLimitExceeded` | Updates a notebook instance. NotebookInstance updates include upgrading or downgrading the ML compute instance used for your notebook instance to accommodate changes in your workload requirements. |
| `UpdateNotebookInstanceLifecycleConfig` | - | - | `NotebookInstanceLifecycleConfigName` | - | `UpdateNotebookInstanceLifecycleConfigOutput` | `ResourceLimitExceeded` | Updates a notebook instance lifecycle configuration created with the CreateNotebookInstanceLifecycleConfig API. Updates to lifecycle configurations affect all notebook instances using that configuration upon their next start. |
| `UpdatePartnerApp` | - | `idempotency-token` | `Arn` | `ClientToken` | `UpdatePartnerAppResponse` | `ConflictException`, `ResourceNotFound` | Updates all of the SageMaker Partner AI Apps in an account. |
| `UpdatePipeline` | - | - | `PipelineName` | - | `UpdatePipelineResponse` | `ConflictException`, `ResourceNotFound` | Updates a pipeline. |
| `UpdatePipelineExecution` | - | - | `PipelineExecutionArn` | - | `UpdatePipelineExecutionResponse` | `ConflictException`, `ResourceNotFound` | Updates a pipeline execution. |
| `UpdatePipelineVersion` | - | - | `PipelineArn`, `PipelineVersionId` | - | `UpdatePipelineVersionResponse` | `ConflictException`, `ResourceNotFound` | Updates a pipeline version. |
| `UpdateProject` | - | - | `ProjectName` | - | `UpdateProjectOutput` | `ConflictException` | Updates a machine learning (ML) project that is created from a template that sets up an ML pipeline from training to deploying an approved model. You must not update a project that is in use. |
| `UpdateSpace` | - | - | `DomainId`, `SpaceName` | - | `UpdateSpaceResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Updates the settings of a space. You can't edit the app type of a space in the `SpaceSettings`. |
| `UpdateTrainingJob` | - | - | `TrainingJobName` | - | `UpdateTrainingJobResponse` | `ResourceLimitExceeded`, `ResourceNotFound` | Update a model training job to request a new Debugger profiling configuration or to change warm pool retention length. |
| `UpdateTrial` | - | - | `TrialName` | - | `UpdateTrialResponse` | `ConflictException`, `ResourceNotFound` | Updates the display name of a trial. |
| `UpdateTrialComponent` | - | - | `TrialComponentName` | - | `UpdateTrialComponentResponse` | `ConflictException`, `ResourceNotFound` | Updates one or more properties of a trial component. |
| `UpdateUserProfile` | - | - | `DomainId`, `UserProfileName` | - | `UpdateUserProfileResponse` | `ResourceInUse`, `ResourceLimitExceeded`, `ResourceNotFound` | Updates a user profile. |
| `UpdateWorkforce` | - | - | `WorkforceName` | - | `UpdateWorkforceResponse` | `ConflictException` | Use this operation to update your workforce. You can use this operation to require that workers use specific IP addresses to work on tasks and to update your OpenID Connect (OIDC) Identity Provider (IdP) workforce configuration. |
| `UpdateWorkteam` | - | - | `WorkteamName` | - | `UpdateWorkteamResponse` | `ResourceLimitExceeded` | Updates an existing work team with new member definitions or description. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFound` | `structure` | `Message` | Resource being access is not found. |
| `ResourceLimitExceeded` | `structure` | `Message` | You have exceeded an SageMaker resource limit. |
| `ResourceInUse` | `structure` | `Message` | Resource being accessed is in use. |
| `ConflictException` | `structure` | `Message` | There was a conflict when you attempted to modify a SageMaker entity such as an `Experiment` or `Artifact`. |
| `AddAssociationRequest` | `structure` | `AssociationType`, `DestinationArn`, `SourceArn` | - |
| `AddAssociationResponse` | `structure` | `DestinationArn`, `SourceArn` | - |
| `AddTagsInput` | `structure` | `ResourceArn`, `Tags` | - |
| `AddTagsOutput` | `structure` | `Tags` | - |
| `AssociateTrialComponentRequest` | `structure` | `TrialComponentName`, `TrialName` | - |
| `AssociateTrialComponentResponse` | `structure` | `TrialArn`, `TrialComponentArn` | - |
| `AttachClusterNodeVolumeRequest` | `structure` | `ClusterArn`, `NodeId`, `VolumeId` | - |
| `AttachClusterNodeVolumeResponse` | `structure` | `AttachTime`, `ClusterArn`, `DeviceName`, `NodeId`, `Status`, `VolumeId` | - |
| `BatchAddClusterNodesRequest` | `structure` | `ClientToken`, `ClusterName`, `NodesToAdd` | - |
| `BatchAddClusterNodesResponse` | `structure` | `Failed`, `Successful` | - |
| `BatchDeleteClusterNodesRequest` | `structure` | `ClusterName`, `NodeIds`, `NodeLogicalIds` | - |
| `BatchDeleteClusterNodesResponse` | `structure` | `Failed`, `FailedNodeLogicalIds`, `Successful`, `SuccessfulNodeLogicalIds` | - |
| `BatchDescribeModelPackageInput` | `structure` | `ModelPackageArnList` | - |
| `BatchDescribeModelPackageOutput` | `structure` | `BatchDescribeModelPackageErrorMap`, `ModelPackageSummaries` | - |
| `BatchRebootClusterNodesRequest` | `structure` | `ClusterName`, `NodeIds`, `NodeLogicalIds` | - |
| `BatchRebootClusterNodesResponse` | `structure` | `Failed`, `FailedNodeLogicalIds`, `Successful`, `SuccessfulNodeLogicalIds` | - |
| `BatchReplaceClusterNodesRequest` | `structure` | `ClusterName`, `NodeIds`, `NodeLogicalIds` | - |
| `BatchReplaceClusterNodesResponse` | `structure` | `Failed`, `FailedNodeLogicalIds`, `Successful`, `SuccessfulNodeLogicalIds` | - |
| `CreateActionRequest` | `structure` | `ActionName`, `ActionType`, `Description`, `MetadataProperties`, `Properties`, `Source`, `Status`, `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
