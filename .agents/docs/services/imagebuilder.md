# EC2 Image Builder

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

EC2 Image Builder is a fully managed Amazon Web Services service that makes it easier to automate the creation, management, and deployment of customized, secure, and up-to-date "golden" server images that are pre-installed and pre-configured with software and settings to meet specific IT standards.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for EC2 Image Builder resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented EC2 Image Builder workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Put` operation families, including `ListComponentBuildVersions`, `ListComponents`, `ListContainerRecipes`, `ListDistributionConfigurations`, `GetComponent`, `GetComponentPolicy`.

## Service Identity and Protocol

- AWS model slug: `imagebuilder`
- AWS SDK for Rust slug: `imagebuilder`
- Model version: `2019-12-02`
- Model file: `vendor/api-models-aws/models/imagebuilder/service/2019-12-02/imagebuilder-2019-12-02.json`
- SDK ID: `imagebuilder`
- Endpoint prefix: `imagebuilder`
- ARN namespace: `imagebuilder`
- CloudFormation name: `Imagebuilder`
- CloudTrail event source: `imagebuilder.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (22), `Get` (17), `Create` (9), `Delete` (9), `Put` (4), `Update` (4), `Import` (3), `Cancel` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelImageCreation`, `CancelLifecycleExecution`, `CreateComponent`, `CreateContainerRecipe`, `CreateDistributionConfiguration`, `CreateImage`, `CreateImagePipeline`, `CreateImageRecipe`, `CreateInfrastructureConfiguration`, `CreateLifecyclePolicy`, `CreateWorkflow`, `DeleteComponent`, `DeleteContainerRecipe`, `DeleteDistributionConfiguration`, `DeleteImage`, `DeleteImagePipeline`, `DeleteImageRecipe`, `DeleteInfrastructureConfiguration`, `DeleteLifecyclePolicy`, `DeleteWorkflow`, ... (+15).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetComponent`, `GetComponentPolicy`, `GetContainerRecipe`, `GetContainerRecipePolicy`, `GetDistributionConfiguration`, `GetImage`, `GetImagePipeline`, `GetImagePolicy`, `GetImageRecipe`, `GetImageRecipePolicy`, `GetInfrastructureConfiguration`, `GetLifecycleExecution`, `GetLifecyclePolicy`, `GetMarketplaceResource`, `GetWorkflow`, `GetWorkflowExecution`, `GetWorkflowStepExecution`, `ListComponentBuildVersions`, `ListComponents`, `ListContainerRecipes`, ... (+19).
- Pagination is modelled for 21 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 23 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelImageCreation`, `CancelLifecycleExecution`, `GetLifecycleExecution`, `GetWorkflowExecution`, `GetWorkflowStepExecution`, `ImportComponent`, `ImportDiskImage`, `ImportVmImage`, `ListImageScanFindingAggregations`, `ListImageScanFindings`, `ListLifecycleExecutionResources`, `ListLifecycleExecutions`, `ListWorkflowExecutions`, `ListWorkflowStepExecutions`, `StartImagePipelineExecution`, `StartResourceStateUpdate`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 77 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECR`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListComponentBuildVersions`, `ListComponents`, `ListContainerRecipes`, `ListDistributionConfigurations`, `ListImageBuildVersions`, `ListImagePackages`, `ListImagePipelineImages`, `ListImagePipelines`, `ListImageRecipes`, `ListImageScanFindingAggregations`, `ListImageScanFindings`, `ListImages`, `ListInfrastructureConfigurations`, `ListLifecycleExecutionResources`, `ListLifecycleExecutions`, `ListLifecyclePolicies`, `ListTagsForResource`, `ListWaitingWorkflowSteps`, `ListWorkflowBuildVersions`, `ListWorkflowExecutions`, `ListWorkflowStepExecutions`, `ListWorkflows`
- Traits: `paginated` (21)
- Common required input members in this group: `imageBuildVersionArn`, `imagePipelineArn`, `lifecycleExecutionId`, `resourceArn`, `workflowExecutionId`

### Get

- Operations: `GetComponent`, `GetComponentPolicy`, `GetContainerRecipe`, `GetContainerRecipePolicy`, `GetDistributionConfiguration`, `GetImage`, `GetImagePipeline`, `GetImagePolicy`, `GetImageRecipe`, `GetImageRecipePolicy`, `GetInfrastructureConfiguration`, `GetLifecycleExecution`, `GetLifecyclePolicy`, `GetMarketplaceResource`, `GetWorkflow`, `GetWorkflowExecution`, `GetWorkflowStepExecution`
- Common required input members in this group: `componentArn`, `componentBuildVersionArn`, `containerRecipeArn`, `distributionConfigurationArn`, `imageArn`, `imageBuildVersionArn`, `imagePipelineArn`, `imageRecipeArn`, `infrastructureConfigurationArn`, `lifecycleExecutionId`, `lifecyclePolicyArn`, `resourceArn`, `resourceType`, `stepExecutionId`, `workflowBuildVersionArn`, `workflowExecutionId`

### Create

- Operations: `CreateComponent`, `CreateContainerRecipe`, `CreateDistributionConfiguration`, `CreateImage`, `CreateImagePipeline`, `CreateImageRecipe`, `CreateInfrastructureConfiguration`, `CreateLifecyclePolicy`, `CreateWorkflow`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `clientToken`, `containerType`, `distributions`, `executionRole`, `infrastructureConfigurationArn`, `instanceProfileName`, `name`, `parentImage`, `platform`, `policyDetails`, `resourceSelection`, `resourceType`, `semanticVersion`, `targetRepository`, `type`

### Delete

- Operations: `DeleteComponent`, `DeleteContainerRecipe`, `DeleteDistributionConfiguration`, `DeleteImage`, `DeleteImagePipeline`, `DeleteImageRecipe`, `DeleteInfrastructureConfiguration`, `DeleteLifecyclePolicy`, `DeleteWorkflow`
- Common required input members in this group: `componentBuildVersionArn`, `containerRecipeArn`, `distributionConfigurationArn`, `imageBuildVersionArn`, `imagePipelineArn`, `imageRecipeArn`, `infrastructureConfigurationArn`, `lifecyclePolicyArn`, `workflowBuildVersionArn`

### Put

- Operations: `PutComponentPolicy`, `PutContainerRecipePolicy`, `PutImagePolicy`, `PutImageRecipePolicy`
- Common required input members in this group: `componentArn`, `containerRecipeArn`, `imageArn`, `imageRecipeArn`, `policy`

### Update

- Operations: `UpdateDistributionConfiguration`, `UpdateImagePipeline`, `UpdateInfrastructureConfiguration`, `UpdateLifecyclePolicy`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `clientToken`, `distributionConfigurationArn`, `distributions`, `executionRole`, `imagePipelineArn`, `infrastructureConfigurationArn`, `instanceProfileName`, `lifecyclePolicyArn`, `policyDetails`, `resourceSelection`, `resourceType`

### Import

- Operations: `ImportComponent`, `ImportDiskImage`, `ImportVmImage`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `clientToken`, `format`, `infrastructureConfigurationArn`, `name`, `osVersion`, `platform`, `semanticVersion`, `type`, `uri`, `vmImportTaskId`

### Cancel

- Operations: `CancelImageCreation`, `CancelLifecycleExecution`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `clientToken`, `imageBuildVersionArn`, `lifecycleExecutionId`

### Start

- Operations: `StartImagePipelineExecution`, `StartResourceStateUpdate`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `clientToken`, `imagePipelineArn`, `resourceArn`, `state`

### Distribute

- Operations: `DistributeImage`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `clientToken`, `distributionConfigurationArn`, `executionRole`, `sourceImage`

### Retry

- Operations: `RetryImage`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `clientToken`, `imageBuildVersionArn`

### Send

- Operations: `SendWorkflowStepAction`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `action`, `clientToken`, `imageBuildVersionArn`, `stepExecutionId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelImageCreation` | `PUT /CancelImageCreation` | `idempotency-token` | `clientToken`, `imageBuildVersionArn` | `clientToken` | `CancelImageCreationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | CancelImageCreation cancels the creation of Image. This operation can only be used on images in a non-terminal state. |
| `CancelLifecycleExecution` | `PUT /CancelLifecycleExecution` | `idempotency-token` | `clientToken`, `lifecycleExecutionId` | `clientToken` | `CancelLifecycleExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Cancel a specific image lifecycle policy runtime instance. |
| `CreateComponent` | `PUT /CreateComponent` | `idempotency-token` | `clientToken`, `name`, `platform`, `semanticVersion` | `clientToken` | `CreateComponentResponse` | `CallRateLimitExceededException`, `ClientException`, `DryRunOperationException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `InvalidVersionNumberException`, ... (+4) | Creates a new component that can be used to build, validate, test, and assess your image. The component is based on a YAML document that you specify using exactly one of the following methods: Inline, using the `data` property in the request body. |
| `CreateContainerRecipe` | `PUT /CreateContainerRecipe` | `idempotency-token` | `clientToken`, `containerType`, `name`, `parentImage`, `semanticVersion`, `targetRepository` | `clientToken` | `CreateContainerRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, ... (+3) | Creates a new container recipe. Container recipes define how images are configured, tested, and assessed. |
| `CreateDistributionConfiguration` | `PUT /CreateDistributionConfiguration` | `idempotency-token` | `clientToken`, `distributions`, `name` | `clientToken` | `CreateDistributionConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, ... (+3) | Creates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. |
| `CreateImage` | `PUT /CreateImage` | `idempotency-token` | `clientToken`, `infrastructureConfigurationArn` | `clientToken` | `CreateImageResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, ... (+1) | Creates a new image. This request will create a new image along with all of the configured output resources defined in the distribution configuration. |
| `CreateImagePipeline` | `PUT /CreateImagePipeline` | `idempotency-token` | `clientToken`, `infrastructureConfigurationArn`, `name` | `clientToken` | `CreateImagePipelineResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, ... (+2) | Creates a new image pipeline. Image pipelines enable you to automate the creation and distribution of images. |
| `CreateImageRecipe` | `PUT /CreateImageRecipe` | `idempotency-token` | `clientToken`, `name`, `parentImage`, `semanticVersion` | `clientToken` | `CreateImageRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, ... (+3) | Creates a new image recipe. Image recipes define how images are configured, tested, and assessed. |
| `CreateInfrastructureConfiguration` | `PUT /CreateInfrastructureConfiguration` | `idempotency-token` | `clientToken`, `instanceProfileName`, `name` | `clientToken` | `CreateInfrastructureConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, ... (+2) | Creates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. |
| `CreateLifecyclePolicy` | `PUT /CreateLifecyclePolicy` | `idempotency-token` | `clientToken`, `executionRole`, `name`, `policyDetails`, `resourceSelection`, `resourceType` | `clientToken` | `CreateLifecyclePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, ... (+2) | Create a lifecycle policy resource. |
| `CreateWorkflow` | `PUT /CreateWorkflow` | `idempotency-token` | `clientToken`, `name`, `semanticVersion`, `type` | `clientToken` | `CreateWorkflowResponse` | `CallRateLimitExceededException`, `ClientException`, `DryRunOperationException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `InvalidVersionNumberException`, ... (+4) | Create a new workflow or a new version of an existing workflow. |
| `DeleteComponent` | `DELETE /DeleteComponent` | - | `componentBuildVersionArn` | - | `DeleteComponentResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a component build version. |
| `DeleteContainerRecipe` | `DELETE /DeleteContainerRecipe` | - | `containerRecipeArn` | - | `DeleteContainerRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a container recipe. |
| `DeleteDistributionConfiguration` | `DELETE /DeleteDistributionConfiguration` | - | `distributionConfigurationArn` | - | `DeleteDistributionConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a distribution configuration. |
| `DeleteImage` | `DELETE /DeleteImage` | - | `imageBuildVersionArn` | - | `DeleteImageResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an Image Builder image resource. This does not delete any EC2 AMIs or ECR container images that are created during the image build process. |
| `DeleteImagePipeline` | `DELETE /DeleteImagePipeline` | - | `imagePipelineArn` | - | `DeleteImagePipelineResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an image pipeline. |
| `DeleteImageRecipe` | `DELETE /DeleteImageRecipe` | - | `imageRecipeArn` | - | `DeleteImageRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an image recipe. |
| `DeleteInfrastructureConfiguration` | `DELETE /DeleteInfrastructureConfiguration` | - | `infrastructureConfigurationArn` | - | `DeleteInfrastructureConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an infrastructure configuration. |
| `DeleteLifecyclePolicy` | `DELETE /DeleteLifecyclePolicy` | - | `lifecyclePolicyArn` | - | `DeleteLifecyclePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Delete the specified lifecycle policy resource. |
| `DeleteWorkflow` | `DELETE /DeleteWorkflow` | - | `workflowBuildVersionArn` | - | `DeleteWorkflowResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a specific workflow resource. |
| `DistributeImage` | `PUT /DistributeImage` | `idempotency-token` | `clientToken`, `distributionConfigurationArn`, `executionRole`, `sourceImage` | `clientToken` | `DistributeImageResponse` | `AccessDeniedException`, `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, ... (+4) | DistributeImage distributes existing AMIs to additional regions and accounts without rebuilding the image. |
| `GetComponent` | `GET /GetComponent` | - | `componentBuildVersionArn` | - | `GetComponentResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Gets a component object. |
| `GetComponentPolicy` | `GET /GetComponentPolicy` | - | `componentArn` | - | `GetComponentPolicyResponse` | `CallRateLimitExceededException`, `ForbiddenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Gets a component policy. |
| `GetContainerRecipe` | `GET /GetContainerRecipe` | - | `containerRecipeArn` | - | `GetContainerRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Retrieves a container recipe. |
| `GetContainerRecipePolicy` | `GET /GetContainerRecipePolicy` | - | `containerRecipeArn` | - | `GetContainerRecipePolicyResponse` | `CallRateLimitExceededException`, `ForbiddenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Retrieves the policy for a container recipe. |
| `GetDistributionConfiguration` | `GET /GetDistributionConfiguration` | - | `distributionConfigurationArn` | - | `GetDistributionConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Gets a distribution configuration. |
| `GetImage` | `GET /GetImage` | - | `imageBuildVersionArn` | - | `GetImageResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Gets an image. |
| `GetImagePipeline` | `GET /GetImagePipeline` | - | `imagePipelineArn` | - | `GetImagePipelineResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Gets an image pipeline. |
| `GetImagePolicy` | `GET /GetImagePolicy` | - | `imageArn` | - | `GetImagePolicyResponse` | `CallRateLimitExceededException`, `ForbiddenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Gets an image policy. |
| `GetImageRecipe` | `GET /GetImageRecipe` | - | `imageRecipeArn` | - | `GetImageRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Gets an image recipe. |
| `GetImageRecipePolicy` | `GET /GetImageRecipePolicy` | - | `imageRecipeArn` | - | `GetImageRecipePolicyResponse` | `CallRateLimitExceededException`, `ForbiddenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Gets an image recipe policy. |
| `GetInfrastructureConfiguration` | `GET /GetInfrastructureConfiguration` | - | `infrastructureConfigurationArn` | - | `GetInfrastructureConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Gets an infrastructure configuration. |
| `GetLifecycleExecution` | `GET /GetLifecycleExecution` | - | `lifecycleExecutionId` | - | `GetLifecycleExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get the runtime information that was logged for a specific runtime instance of the lifecycle policy. |
| `GetLifecyclePolicy` | `GET /GetLifecyclePolicy` | - | `lifecyclePolicyArn` | - | `GetLifecyclePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get details for the specified image lifecycle policy. |
| `GetMarketplaceResource` | `POST /GetMarketplaceResource` | - | `resourceArn`, `resourceType` | - | `GetMarketplaceResourceResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Verify the subscription and perform resource dependency checks on the requested Amazon Web Services Marketplace resource. For Amazon Web Services Marketplace components, the response contains fields to download the components and their artifacts. |
| `GetWorkflow` | `GET /GetWorkflow` | - | `workflowBuildVersionArn` | - | `GetWorkflowResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get a workflow resource object. |
| `GetWorkflowExecution` | `GET /GetWorkflowExecution` | - | `workflowExecutionId` | - | `GetWorkflowExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get the runtime information that was logged for a specific runtime instance of the workflow. |
| `GetWorkflowStepExecution` | `GET /GetWorkflowStepExecution` | - | `stepExecutionId` | - | `GetWorkflowStepExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get the runtime information that was logged for a specific runtime instance of the workflow step. |
| `ImportComponent` | `PUT /ImportComponent` | `idempotency-token` | `clientToken`, `format`, `name`, `platform`, `semanticVersion`, `type` | `clientToken` | `ImportComponentResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceInUseException`, ... (+2) | Imports a component and transforms its data into a component document. |
| `ImportDiskImage` | `PUT /ImportDiskImage` | `idempotency-token` | `clientToken`, `infrastructureConfigurationArn`, `name`, `osVersion`, `platform`, `semanticVersion`, `uri` | `clientToken` | `ImportDiskImageResponse` | `ClientException`, `ServiceException`, `ServiceUnavailableException` | Import a Windows operating system image from a verified Microsoft ISO disk file. The following disk images are supported: Windows 11 Enterprise |
| `ImportVmImage` | `PUT /ImportVmImage` | `idempotency-token` | `clientToken`, `name`, `platform`, `semanticVersion`, `vmImportTaskId` | `clientToken` | `ImportVmImageResponse` | `ClientException`, `ServiceException`, `ServiceUnavailableException` | When you export your virtual machine (VM) from its virtualization environment, that process creates a set of one or more disk container files that act as snapshots of your VM’s environment, settings, and data. The Amazon EC2 API ImportImage action uses those... |
| `ListComponentBuildVersions` | `POST /ListComponentBuildVersions` | `paginated` | - | - | `ListComponentBuildVersionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns the list of component build versions for the specified component version Amazon Resource Name (ARN). |
| `ListComponents` | `POST /ListComponents` | `paginated` | - | - | `ListComponentsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns the list of components that can be filtered by name, or by using the listed `filters` to streamline results. Newly created components can take up to two minutes to appear in the ListComponents API Results. |
| `ListContainerRecipes` | `POST /ListContainerRecipes` | `paginated` | - | - | `ListContainerRecipesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of container recipes. |
| `ListDistributionConfigurations` | `POST /ListDistributionConfigurations` | `paginated` | - | - | `ListDistributionConfigurationsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of distribution configurations. |
| `ListImageBuildVersions` | `POST /ListImageBuildVersions` | `paginated` | - | - | `ListImageBuildVersionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image build versions. |
| `ListImagePackages` | `POST /ListImagePackages` | `paginated` | `imageBuildVersionArn` | - | `ListImagePackagesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | List the Packages that are associated with an Image Build Version, as determined by Amazon Web Services Systems Manager Inventory at build time. |
| `ListImagePipelineImages` | `POST /ListImagePipelineImages` | `paginated` | `imagePipelineArn` | - | `ListImagePipelineImagesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of images created by the specified pipeline. |
| `ListImagePipelines` | `POST /ListImagePipelines` | `paginated` | - | - | `ListImagePipelinesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image pipelines. |
| `ListImageRecipes` | `POST /ListImageRecipes` | `paginated` | - | - | `ListImageRecipesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image recipes. |
| `ListImageScanFindingAggregations` | `POST /ListImageScanFindingAggregations` | `paginated` | - | - | `ListImageScanFindingAggregationsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image scan aggregations for your account. You can filter by the type of key that Image Builder uses to group results. |
| `ListImageScanFindings` | `POST /ListImageScanFindings` | `paginated` | - | - | `ListImageScanFindingsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image scan findings for your account. |
| `ListImages` | `POST /ListImages` | `paginated` | - | - | `ListImagesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns the list of images that you have access to. Newly created images can take up to two minutes to appear in the ListImages API Results. |
| `ListInfrastructureConfigurations` | `POST /ListInfrastructureConfigurations` | `paginated` | - | - | `ListInfrastructureConfigurationsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of infrastructure configurations. |
| `ListLifecycleExecutionResources` | `POST /ListLifecycleExecutionResources` | `paginated` | `lifecycleExecutionId` | - | `ListLifecycleExecutionResourcesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | List resources that the runtime instance of the image lifecycle identified for lifecycle actions. |
| `ListLifecycleExecutions` | `POST /ListLifecycleExecutions` | `paginated` | `resourceArn` | - | `ListLifecycleExecutionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get the lifecycle runtime history for the specified resource. |
| `ListLifecyclePolicies` | `POST /ListLifecyclePolicies` | `paginated` | - | - | `ListLifecyclePoliciesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get a list of lifecycle policies in your Amazon Web Services account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceException` | Returns the list of tags for the specified resource. |
| `ListWaitingWorkflowSteps` | `POST /ListWaitingWorkflowSteps` | `paginated` | - | - | `ListWaitingWorkflowStepsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get a list of workflow steps that are waiting for action for workflows in your Amazon Web Services account. |
| `ListWorkflowBuildVersions` | `POST /ListWorkflowBuildVersions` | `paginated` | - | - | `ListWorkflowBuildVersionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of build versions for a specific workflow resource. |
| `ListWorkflowExecutions` | `POST /ListWorkflowExecutions` | `paginated` | `imageBuildVersionArn` | - | `ListWorkflowExecutionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of workflow runtime instance metadata objects for a specific image build version. |
| `ListWorkflowStepExecutions` | `POST /ListWorkflowStepExecutions` | `paginated` | `workflowExecutionId` | - | `ListWorkflowStepExecutionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns runtime data for each step in a runtime instance of the workflow that you specify in the request. |
| `ListWorkflows` | `POST /ListWorkflows` | `paginated` | - | - | `ListWorkflowsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Lists workflow build versions based on filtering parameters. |
| `PutComponentPolicy` | `PUT /PutComponentPolicy` | - | `componentArn`, `policy` | - | `PutComponentPolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to a component. We recommend that you call the RAM API CreateResourceShare to share resources. |
| `PutContainerRecipePolicy` | `PUT /PutContainerRecipePolicy` | - | `containerRecipeArn`, `policy` | - | `PutContainerRecipePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to a container image. We recommend that you call the RAM API CreateResourceShare (https://docs.aws.amazon.com//ram/latest/APIReference/API_CreateResourceShare.html) to share resources. |
| `PutImagePolicy` | `PUT /PutImagePolicy` | - | `imageArn`, `policy` | - | `PutImagePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to an image. We recommend that you call the RAM API CreateResourceShare to share resources. |
| `PutImageRecipePolicy` | `PUT /PutImageRecipePolicy` | - | `imageRecipeArn`, `policy` | - | `PutImageRecipePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to an image recipe. We recommend that you call the RAM API CreateResourceShare to share resources. |
| `RetryImage` | `PUT /RetryImage` | `idempotency-token` | `clientToken`, `imageBuildVersionArn` | `clientToken` | `RetryImageResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | RetryImage retries an image distribution without rebuilding the image. |
| `SendWorkflowStepAction` | `PUT /SendWorkflowStepAction` | `idempotency-token` | `action`, `clientToken`, `imageBuildVersionArn`, `stepExecutionId` | `clientToken` | `SendWorkflowStepActionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, ... (+2) | Pauses or resumes image creation when the associated workflow runs a `WaitForAction` step. |
| `StartImagePipelineExecution` | `PUT /StartImagePipelineExecution` | `idempotency-token` | `clientToken`, `imagePipelineArn` | `clientToken` | `StartImagePipelineExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, ... (+1) | Manually triggers a pipeline to create an image. |
| `StartResourceStateUpdate` | `PUT /StartResourceStateUpdate` | `idempotency-token` | `clientToken`, `resourceArn`, `state` | `clientToken` | `StartResourceStateUpdateResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, ... (+1) | Begin asynchronous resource state update for lifecycle changes to the specified image resources. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceException` | Adds a tag to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceException` | Removes a tag from a resource. |
| `UpdateDistributionConfiguration` | `PUT /UpdateDistributionConfiguration` | `idempotency-token` | `clientToken`, `distributionConfigurationArn`, `distributions` | `clientToken` | `UpdateDistributionConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, ... (+1) | Updates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. |
| `UpdateImagePipeline` | `PUT /UpdateImagePipeline` | `idempotency-token` | `clientToken`, `imagePipelineArn`, `infrastructureConfigurationArn` | `clientToken` | `UpdateImagePipelineResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Updates an image pipeline. Image pipelines enable you to automate the creation and distribution of images. |
| `UpdateInfrastructureConfiguration` | `PUT /UpdateInfrastructureConfiguration` | `idempotency-token` | `clientToken`, `infrastructureConfigurationArn`, `instanceProfileName` | `clientToken` | `UpdateInfrastructureConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Updates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. |
| `UpdateLifecyclePolicy` | `PUT /UpdateLifecyclePolicy` | `idempotency-token` | `clientToken`, `executionRole`, `lifecyclePolicyArn`, `policyDetails`, `resourceSelection`, `resourceType` | `clientToken` | `UpdateLifecyclePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, ... (+1) | Update the specified lifecycle policy. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteComponent` | - | `componentBuildVersionArn -> componentBuildVersionArn` | - | - |
| `DeleteContainerRecipe` | - | `containerRecipeArn -> containerRecipeArn` | - | - |
| `DeleteDistributionConfiguration` | - | `distributionConfigurationArn -> distributionConfigurationArn` | - | - |
| `DeleteImage` | - | `imageBuildVersionArn -> imageBuildVersionArn` | - | - |
| `DeleteImagePipeline` | - | `imagePipelineArn -> imagePipelineArn` | - | - |
| `DeleteImageRecipe` | - | `imageRecipeArn -> imageRecipeArn` | - | - |
| `DeleteInfrastructureConfiguration` | - | `infrastructureConfigurationArn -> infrastructureConfigurationArn` | - | - |
| `DeleteLifecyclePolicy` | - | `lifecyclePolicyArn -> lifecyclePolicyArn` | - | - |
| `DeleteWorkflow` | - | `workflowBuildVersionArn -> workflowBuildVersionArn` | - | - |
| `GetComponent` | - | `componentBuildVersionArn -> componentBuildVersionArn` | - | - |
| `GetComponentPolicy` | - | `componentArn -> componentArn` | - | - |
| `GetContainerRecipe` | - | `containerRecipeArn -> containerRecipeArn` | - | - |
| `GetContainerRecipePolicy` | - | `containerRecipeArn -> containerRecipeArn` | - | - |
| `GetDistributionConfiguration` | - | `distributionConfigurationArn -> distributionConfigurationArn` | - | - |
| `GetImage` | - | `imageBuildVersionArn -> imageBuildVersionArn` | - | - |
| `GetImagePipeline` | - | `imagePipelineArn -> imagePipelineArn` | - | - |
| `GetImagePolicy` | - | `imageArn -> imageArn` | - | - |
| `GetImageRecipe` | - | `imageRecipeArn -> imageRecipeArn` | - | - |
| `GetImageRecipePolicy` | - | `imageRecipeArn -> imageRecipeArn` | - | - |
| `GetInfrastructureConfiguration` | - | `infrastructureConfigurationArn -> infrastructureConfigurationArn` | - | - |
| `GetLifecycleExecution` | - | `lifecycleExecutionId -> lifecycleExecutionId` | - | - |
| `GetLifecyclePolicy` | - | `lifecyclePolicyArn -> lifecyclePolicyArn` | - | - |
| `GetWorkflow` | - | `workflowBuildVersionArn -> workflowBuildVersionArn` | - | - |
| `GetWorkflowExecution` | - | `workflowExecutionId -> workflowExecutionId` | - | - |
| `GetWorkflowStepExecution` | - | `stepExecutionId -> stepExecutionId` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ServiceException` | `structure` | `message` | This exception is thrown when the service encounters an unrecoverable exception. |
| `ServiceUnavailableException` | `structure` | `message` | The service is unable to process your request at this time. |
| `CallRateLimitExceededException` | `structure` | `message` | You have exceeded the permitted request rate for the specific operation. |
| `ForbiddenException` | `structure` | `message` | You are not authorized to perform the requested operation. |
| `InvalidRequestException` | `structure` | `message` | You have requested an action that that the service doesn't support. |
| `ClientException` | `structure` | `message` | These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or... |
| `IdempotentParameterMismatchException` | `structure` | `message` | You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token. |
| `ResourceInUseException` | `structure` | `message` | The resource that you are trying to operate on is currently in use. |
| `InvalidPaginationTokenException` | `structure` | `message` | You have provided an invalid pagination token in your request. |
| `ResourceNotFoundException` | `structure` | `message` | At least one of the resources referenced by your request does not exist. |
| `ServiceQuotaExceededException` | `structure` | `message` | You have exceeded the number of permitted resources or operations for this service. |
| `ResourceDependencyException` | `structure` | `message` | You have attempted to mutate or delete a resource with a dependency that prohibits this action. |
| `InvalidParameterCombinationException` | `structure` | `message` | You have specified two or more mutually exclusive parameters. |
| `ResourceAlreadyExistsException` | `structure` | `message` | The resource that you are trying to create already exists. |
| `InvalidVersionNumberException` | `structure` | `message` | Your version number is out of bounds or does not follow the required syntax. |
| `InvalidParameterValueException` | `structure` | `message` | The value that you provided for the specified parameter is invalid. |
| `InvalidParameterException` | `structure` | `message` | The specified parameter is invalid. |
| `DryRunOperationException` | `structure` | `message` | The dry run operation of the resource was successful, and no resources or mutations were actually performed due to the dry run flag in the request. |
| `CancelImageCreationRequest` | `structure` | `clientToken`, `imageBuildVersionArn` | - |
| `CancelImageCreationResponse` | `structure` | `clientToken`, `imageBuildVersionArn`, `requestId` | - |
| `CancelLifecycleExecutionRequest` | `structure` | `clientToken`, `lifecycleExecutionId` | - |
| `CancelLifecycleExecutionResponse` | `structure` | `lifecycleExecutionId` | - |
| `CreateComponentRequest` | `structure` | `changeDescription`, `clientToken`, `data`, `description`, `dryRun`, `kmsKeyId`, `name`, `platform`, `semanticVersion`, `supportedOsVersions`, `tags`, `uri` | - |
| `CreateComponentResponse` | `structure` | `clientToken`, `componentBuildVersionArn`, `latestVersionReferences`, `requestId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
