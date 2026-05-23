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

- Operations: `ListComponentBuildVersions`, `ListComponents`, `ListContainerRecipes`, `ListDistributionConfigurations`, `ListImageBuildVersions`, `ListImagePackages`, `ListImagePipelineImages`, `ListImagePipelines`, `ListImageRecipes`, `ListImages`, `ListImageScanFindingAggregations`, `ListImageScanFindings`, `ListInfrastructureConfigurations`, `ListLifecycleExecutionResources`, `ListLifecycleExecutions`, `ListLifecyclePolicies`, `ListTagsForResource`, `ListWaitingWorkflowSteps`, `ListWorkflowBuildVersions`, `ListWorkflowExecutions`, `ListWorkflows`, `ListWorkflowStepExecutions`
- Traits: `paginated` (21)
- Common required input members in this group: `imageBuildVersionArn`, `resourceArn`

### Get

- Operations: `GetComponent`, `GetComponentPolicy`, `GetContainerRecipe`, `GetContainerRecipePolicy`, `GetDistributionConfiguration`, `GetImage`, `GetImagePipeline`, `GetImagePolicy`, `GetImageRecipe`, `GetImageRecipePolicy`, `GetInfrastructureConfiguration`, `GetLifecycleExecution`, `GetLifecyclePolicy`, `GetMarketplaceResource`, `GetWorkflow`, `GetWorkflowExecution`, `GetWorkflowStepExecution`
- Common required input members in this group: `containerRecipeArn`, `imageRecipeArn`

### Create

- Operations: `CreateComponent`, `CreateContainerRecipe`, `CreateDistributionConfiguration`, `CreateImage`, `CreateImagePipeline`, `CreateImageRecipe`, `CreateInfrastructureConfiguration`, `CreateLifecyclePolicy`, `CreateWorkflow`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `name`, `semanticVersion`, `clientToken`, `parentImage`, `infrastructureConfigurationArn`

### Delete

- Operations: `DeleteComponent`, `DeleteContainerRecipe`, `DeleteDistributionConfiguration`, `DeleteImage`, `DeleteImagePipeline`, `DeleteImageRecipe`, `DeleteInfrastructureConfiguration`, `DeleteLifecyclePolicy`, `DeleteWorkflow`
- Common required input members in this group: -

### Put

- Operations: `PutComponentPolicy`, `PutContainerRecipePolicy`, `PutImagePolicy`, `PutImageRecipePolicy`
- Common required input members in this group: `policy`

### Update

- Operations: `UpdateDistributionConfiguration`, `UpdateImagePipeline`, `UpdateInfrastructureConfiguration`, `UpdateLifecyclePolicy`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `clientToken`, `infrastructureConfigurationArn`

### Import

- Operations: `ImportComponent`, `ImportDiskImage`, `ImportVmImage`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `name`, `semanticVersion`, `platform`, `clientToken`

### Cancel

- Operations: `CancelImageCreation`, `CancelLifecycleExecution`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `clientToken`

### Start

- Operations: `StartImagePipelineExecution`, `StartResourceStateUpdate`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `clientToken`

### Distribute

- Operations: `DistributeImage`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Retry

- Operations: `RetryImage`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Send

- Operations: `SendWorkflowStepAction`
- Traits: `idempotency-token` (1)
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
| `CancelImageCreation` | `PUT /CancelImageCreation` | `idempotency-token` | `imageBuildVersionArn`, `clientToken` | `clientToken` | `CancelImageCreationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | CancelImageCreation cancels the creation of Image. This operation can only be used on images in a non-terminal state. |
| `CancelLifecycleExecution` | `PUT /CancelLifecycleExecution` | `idempotency-token` | `lifecycleExecutionId`, `clientToken` | `clientToken` | `CancelLifecycleExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Cancel a specific image lifecycle policy runtime instance. |
| `CreateComponent` | `PUT /CreateComponent` | `idempotency-token` | `name`, `semanticVersion`, `platform`, `clientToken` | `clientToken` | `CreateComponentResponse` | `CallRateLimitExceededException`, `ClientException`, `DryRunOperationException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a new component that can be used to build, validate, test, and assess your image. The component is based on a YAML document that you specify using exactly one of the following methods: Inline, using the data ... |
| `CreateContainerRecipe` | `PUT /CreateContainerRecipe` | `idempotency-token` | `containerType`, `name`, `semanticVersion`, `parentImage`, `targetRepository`, `clientToken` | `clientToken` | `CreateContainerRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a new container recipe. Container recipes define how images are configured, tested, and assessed. |
| `CreateDistributionConfiguration` | `PUT /CreateDistributionConfiguration` | `idempotency-token` | `name`, `distributions`, `clientToken` | `clientToken` | `CreateDistributionConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. |
| `CreateImage` | `PUT /CreateImage` | `idempotency-token` | `infrastructureConfigurationArn`, `clientToken` | `clientToken` | `CreateImageResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a new image. This request will create a new image along with all of the configured output resources defined in the distribution configuration. You must specify exactly one recipe for your image, using either ... |
| `CreateImagePipeline` | `PUT /CreateImagePipeline` | `idempotency-token` | `name`, `infrastructureConfigurationArn`, `clientToken` | `clientToken` | `CreateImagePipelineResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a new image pipeline. Image pipelines enable you to automate the creation and distribution of images. |
| `CreateImageRecipe` | `PUT /CreateImageRecipe` | `idempotency-token` | `name`, `semanticVersion`, `parentImage`, `clientToken` | `clientToken` | `CreateImageRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a new image recipe. Image recipes define how images are configured, tested, and assessed. |
| `CreateInfrastructureConfiguration` | `PUT /CreateInfrastructureConfiguration` | `idempotency-token` | `name`, `instanceProfileName`, `clientToken` | `clientToken` | `CreateInfrastructureConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. |
| `CreateLifecyclePolicy` | `PUT /CreateLifecyclePolicy` | `idempotency-token` | `name`, `executionRole`, `resourceType`, `policyDetails`, `resourceSelection`, `clientToken` | `clientToken` | `CreateLifecyclePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Create a lifecycle policy resource. |
| `CreateWorkflow` | `PUT /CreateWorkflow` | `idempotency-token` | `name`, `semanticVersion`, `clientToken`, `type` | `clientToken` | `CreateWorkflowResponse` | `CallRateLimitExceededException`, `ClientException`, `DryRunOperationException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceInUseException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Create a new workflow or a new version of an existing workflow. |
| `DeleteComponent` | `DELETE /DeleteComponent` | - | `componentBuildVersionArn` | - | `DeleteComponentResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a component build version. |
| `DeleteContainerRecipe` | `DELETE /DeleteContainerRecipe` | - | `containerRecipeArn` | - | `DeleteContainerRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a container recipe. |
| `DeleteDistributionConfiguration` | `DELETE /DeleteDistributionConfiguration` | - | `distributionConfigurationArn` | - | `DeleteDistributionConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a distribution configuration. |
| `DeleteImage` | `DELETE /DeleteImage` | - | `imageBuildVersionArn` | - | `DeleteImageResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an Image Builder image resource. This does not delete any EC2 AMIs or ECR container images that are created during the image build process. You must clean those up separately, using the appropriate Amazon EC2 ... |
| `DeleteImagePipeline` | `DELETE /DeleteImagePipeline` | - | `imagePipelineArn` | - | `DeleteImagePipelineResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an image pipeline. |
| `DeleteImageRecipe` | `DELETE /DeleteImageRecipe` | - | `imageRecipeArn` | - | `DeleteImageRecipeResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an image recipe. |
| `DeleteInfrastructureConfiguration` | `DELETE /DeleteInfrastructureConfiguration` | - | `infrastructureConfigurationArn` | - | `DeleteInfrastructureConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes an infrastructure configuration. |
| `DeleteLifecyclePolicy` | `DELETE /DeleteLifecyclePolicy` | - | `lifecyclePolicyArn` | - | `DeleteLifecyclePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Delete the specified lifecycle policy resource. |
| `DeleteWorkflow` | `DELETE /DeleteWorkflow` | - | `workflowBuildVersionArn` | - | `DeleteWorkflowResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ResourceDependencyException`, `ServiceException`, `ServiceUnavailableException` | Deletes a specific workflow resource. |
| `DistributeImage` | `PUT /DistributeImage` | `idempotency-token` | `sourceImage`, `distributionConfigurationArn`, `executionRole`, `clientToken` | `clientToken` | `DistributeImageResponse` | `AccessDeniedException`, `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `TooManyRequestsException` | DistributeImage distributes existing AMIs to additional regions and accounts without rebuilding the image. |
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
| `GetMarketplaceResource` | `POST /GetMarketplaceResource` | - | `resourceType`, `resourceArn` | - | `GetMarketplaceResourceResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Verify the subscription and perform resource dependency checks on the requested Amazon Web Services Marketplace resource. For Amazon Web Services Marketplace components, the response contains fields to download the c ... |
| `GetWorkflow` | `GET /GetWorkflow` | - | `workflowBuildVersionArn` | - | `GetWorkflowResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get a workflow resource object. |
| `GetWorkflowExecution` | `GET /GetWorkflowExecution` | - | `workflowExecutionId` | - | `GetWorkflowExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get the runtime information that was logged for a specific runtime instance of the workflow. |
| `GetWorkflowStepExecution` | `GET /GetWorkflowStepExecution` | - | `stepExecutionId` | - | `GetWorkflowStepExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get the runtime information that was logged for a specific runtime instance of the workflow step. |
| `ImportComponent` | `PUT /ImportComponent` | `idempotency-token` | `name`, `semanticVersion`, `type`, `format`, `platform`, `clientToken` | `clientToken` | `ImportComponentResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `InvalidVersionNumberException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Imports a component and transforms its data into a component document. |
| `ImportDiskImage` | `PUT /ImportDiskImage` | `idempotency-token` | `name`, `semanticVersion`, `platform`, `osVersion`, `infrastructureConfigurationArn`, `uri`, `clientToken` | `clientToken` | `ImportDiskImageResponse` | `ClientException`, `ServiceException`, `ServiceUnavailableException` | Import a Windows operating system image from a verified Microsoft ISO disk file. The following disk images are supported: Windows 11 Enterprise |
| `ImportVmImage` | `PUT /ImportVmImage` | `idempotency-token` | `name`, `semanticVersion`, `platform`, `vmImportTaskId`, `clientToken` | `clientToken` | `ImportVmImageResponse` | `ClientException`, `ServiceException`, `ServiceUnavailableException` | When you export your virtual machine (VM) from its virtualization environment, that process creates a set of one or more disk container files that act as snapshots of your VM’s environment, settings, and data. The Am ... |
| `ListComponentBuildVersions` | `POST /ListComponentBuildVersions` | `paginated` | - | - | `ListComponentBuildVersionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns the list of component build versions for the specified component version Amazon Resource Name (ARN). |
| `ListComponents` | `POST /ListComponents` | `paginated` | - | - | `ListComponentsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns the list of components that can be filtered by name, or by using the listed filters to streamline results. Newly created components can take up to two minutes to appear in the ListComponents API Results. The ... |
| `ListContainerRecipes` | `POST /ListContainerRecipes` | `paginated` | - | - | `ListContainerRecipesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of container recipes. |
| `ListDistributionConfigurations` | `POST /ListDistributionConfigurations` | `paginated` | - | - | `ListDistributionConfigurationsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of distribution configurations. |
| `ListImageBuildVersions` | `POST /ListImageBuildVersions` | `paginated` | - | - | `ListImageBuildVersionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image build versions. |
| `ListImagePackages` | `POST /ListImagePackages` | `paginated` | `imageBuildVersionArn` | - | `ListImagePackagesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | List the Packages that are associated with an Image Build Version, as determined by Amazon Web Services Systems Manager Inventory at build time. |
| `ListImagePipelineImages` | `POST /ListImagePipelineImages` | `paginated` | `imagePipelineArn` | - | `ListImagePipelineImagesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of images created by the specified pipeline. |
| `ListImagePipelines` | `POST /ListImagePipelines` | `paginated` | - | - | `ListImagePipelinesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image pipelines. |
| `ListImageRecipes` | `POST /ListImageRecipes` | `paginated` | - | - | `ListImageRecipesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image recipes. |
| `ListImages` | `POST /ListImages` | `paginated` | - | - | `ListImagesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns the list of images that you have access to. Newly created images can take up to two minutes to appear in the ListImages API Results. |
| `ListImageScanFindingAggregations` | `POST /ListImageScanFindingAggregations` | `paginated` | - | - | `ListImageScanFindingAggregationsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image scan aggregations for your account. You can filter by the type of key that Image Builder uses to group results. For example, if you want to get a list of findings by severity level for one of ... |
| `ListImageScanFindings` | `POST /ListImageScanFindings` | `paginated` | - | - | `ListImageScanFindingsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of image scan findings for your account. |
| `ListInfrastructureConfigurations` | `POST /ListInfrastructureConfigurations` | `paginated` | - | - | `ListInfrastructureConfigurationsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of infrastructure configurations. |
| `ListLifecycleExecutionResources` | `POST /ListLifecycleExecutionResources` | `paginated` | `lifecycleExecutionId` | - | `ListLifecycleExecutionResourcesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | List resources that the runtime instance of the image lifecycle identified for lifecycle actions. |
| `ListLifecycleExecutions` | `POST /ListLifecycleExecutions` | `paginated` | `resourceArn` | - | `ListLifecycleExecutionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get the lifecycle runtime history for the specified resource. |
| `ListLifecyclePolicies` | `POST /ListLifecyclePolicies` | `paginated` | - | - | `ListLifecyclePoliciesResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get a list of lifecycle policies in your Amazon Web Services account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceException` | Returns the list of tags for the specified resource. |
| `ListWaitingWorkflowSteps` | `POST /ListWaitingWorkflowSteps` | `paginated` | - | - | `ListWaitingWorkflowStepsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Get a list of workflow steps that are waiting for action for workflows in your Amazon Web Services account. |
| `ListWorkflowBuildVersions` | `POST /ListWorkflowBuildVersions` | `paginated` | - | - | `ListWorkflowBuildVersionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of build versions for a specific workflow resource. |
| `ListWorkflowExecutions` | `POST /ListWorkflowExecutions` | `paginated` | `imageBuildVersionArn` | - | `ListWorkflowExecutionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns a list of workflow runtime instance metadata objects for a specific image build version. |
| `ListWorkflows` | `POST /ListWorkflows` | `paginated` | - | - | `ListWorkflowsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Lists workflow build versions based on filtering parameters. |
| `ListWorkflowStepExecutions` | `POST /ListWorkflowStepExecutions` | `paginated` | `workflowExecutionId` | - | `ListWorkflowStepExecutionsResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidPaginationTokenException`, `InvalidRequestException`, `ServiceException`, `ServiceUnavailableException` | Returns runtime data for each step in a runtime instance of the workflow that you specify in the request. |
| `PutComponentPolicy` | `PUT /PutComponentPolicy` | - | `componentArn`, `policy` | - | `PutComponentPolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to a component. We recommend that you call the RAM API CreateResourceShare to share resources. If you call the Image Builder API PutComponentPolicy , you must also call the RAM API PromoteResourceSha ... |
| `PutContainerRecipePolicy` | `PUT /PutContainerRecipePolicy` | - | `containerRecipeArn`, `policy` | - | `PutContainerRecipePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to a container image. We recommend that you call the RAM API CreateResourceShare (https://docs.aws.amazon.com//ram/latest/APIReference/API_CreateResourceShare.html) to share resources. If you call th ... |
| `PutImagePolicy` | `PUT /PutImagePolicy` | - | `imageArn`, `policy` | - | `PutImagePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to an image. We recommend that you call the RAM API CreateResourceShare to share resources. If you call the Image Builder API PutImagePolicy , you must also call the RAM API PromoteResourceShareCreat ... |
| `PutImageRecipePolicy` | `PUT /PutImageRecipePolicy` | - | `imageRecipeArn`, `policy` | - | `PutImageRecipePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Applies a policy to an image recipe. We recommend that you call the RAM API CreateResourceShare to share resources. If you call the Image Builder API PutImageRecipePolicy , you must also call the RAM API PromoteResou ... |
| `RetryImage` | `PUT /RetryImage` | `idempotency-token` | `imageBuildVersionArn`, `clientToken` | `clientToken` | `RetryImageResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | RetryImage retries an image distribution without rebuilding the image. |
| `SendWorkflowStepAction` | `PUT /SendWorkflowStepAction` | `idempotency-token` | `stepExecutionId`, `imageBuildVersionArn`, `action`, `clientToken` | `clientToken` | `SendWorkflowStepActionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterValueException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Pauses or resumes image creation when the associated workflow runs a WaitForAction step. |
| `StartImagePipelineExecution` | `PUT /StartImagePipelineExecution` | `idempotency-token` | `imagePipelineArn`, `clientToken` | `clientToken` | `StartImagePipelineExecutionResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Manually triggers a pipeline to create an image. |
| `StartResourceStateUpdate` | `PUT /StartResourceStateUpdate` | `idempotency-token` | `resourceArn`, `state`, `clientToken` | `clientToken` | `StartResourceStateUpdateResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, `ServiceUnavailableException` | Begin asynchronous resource state update for lifecycle changes to the specified image resources. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceException` | Adds a tag to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceException` | Removes a tag from a resource. |
| `UpdateDistributionConfiguration` | `PUT /UpdateDistributionConfiguration` | `idempotency-token` | `distributionConfigurationArn`, `distributions`, `clientToken` | `clientToken` | `UpdateDistributionConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Updates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. |
| `UpdateImagePipeline` | `PUT /UpdateImagePipeline` | `idempotency-token` | `imagePipelineArn`, `infrastructureConfigurationArn`, `clientToken` | `clientToken` | `UpdateImagePipelineResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Updates an image pipeline. Image pipelines enable you to automate the creation and distribution of images. You must specify exactly one recipe for your image, using either a containerRecipeArn or an imageRecipeArn . ... |
| `UpdateInfrastructureConfiguration` | `PUT /UpdateInfrastructureConfiguration` | `idempotency-token` | `infrastructureConfigurationArn`, `instanceProfileName`, `clientToken` | `clientToken` | `UpdateInfrastructureConfigurationResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Updates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. |
| `UpdateLifecyclePolicy` | `PUT /UpdateLifecyclePolicy` | `idempotency-token` | `lifecyclePolicyArn`, `executionRole`, `resourceType`, `policyDetails`, `resourceSelection`, `clientToken` | `clientToken` | `UpdateLifecyclePolicyResponse` | `CallRateLimitExceededException`, `ClientException`, `ForbiddenException`, `IdempotentParameterMismatchException`, `InvalidParameterCombinationException`, `InvalidRequestException`, `ResourceInUseException`, `ServiceException`, `ServiceUnavailableException` | Update the specified lifecycle policy. |

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
| `AccessDeniedException` | `structure` | message | You do not have permissions to perform the requested operation. |
| `CallRateLimitExceededException` | `structure` | message | You have exceeded the permitted request rate for the specific operation. |
| `ClientException` | `structure` | message | These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action ... |
| `DryRunOperationException` | `structure` | message | The dry run operation of the resource was successful, and no resources or mutations were actually performed due to the dry run flag in the request. |
| `ForbiddenException` | `structure` | message | You are not authorized to perform the requested operation. |
| `IdempotentParameterMismatchException` | `structure` | message | You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token. |
| `InvalidPaginationTokenException` | `structure` | message | You have provided an invalid pagination token in your request. |
| `InvalidParameterCombinationException` | `structure` | message | You have specified two or more mutually exclusive parameters. Review the error message for details. |
| `InvalidParameterException` | `structure` | message | The specified parameter is invalid. Review the available parameters for the API request. |
| `InvalidParameterValueException` | `structure` | message | The value that you provided for the specified parameter is invalid. |
| `InvalidRequestException` | `structure` | message | You have requested an action that that the service doesn't support. |
| `InvalidVersionNumberException` | `structure` | message | Your version number is out of bounds or does not follow the required syntax. |
| `ResourceAlreadyExistsException` | `structure` | message | The resource that you are trying to create already exists. |
| `ResourceDependencyException` | `structure` | message | You have attempted to mutate or delete a resource with a dependency that prohibits this action. See the error message for more details. |
| `ResourceInUseException` | `structure` | message | The resource that you are trying to operate on is currently in use. Review the message details and retry later. |
| `ResourceNotFoundException` | `structure` | message | At least one of the resources referenced by your request does not exist. |
| `ServiceException` | `structure` | message | This exception is thrown when the service encounters an unrecoverable exception. |
| `ServiceQuotaExceededException` | `structure` | message | You have exceeded the number of permitted resources or operations for this service. For service quotas, see EC2 Image Builder endpoints and quotas . |
| `ServiceUnavailableException` | `structure` | message | The service is unable to process your request at this time. |
| `TooManyRequestsException` | `structure` | message | You have attempted too many requests for the specific operation. |
| `CancelImageCreationRequest` | `structure` | imageBuildVersionArn, clientToken | - |
| `CancelImageCreationResponse` | `structure` | requestId, clientToken, imageBuildVersionArn | - |
| `CancelLifecycleExecutionRequest` | `structure` | lifecycleExecutionId, clientToken | - |
| `CancelLifecycleExecutionResponse` | `structure` | lifecycleExecutionId | - |
| `CreateComponentRequest` | `structure` | name, semanticVersion, description, changeDescription, platform, supportedOsVersions, data, uri, kmsKeyId, tags, clientToken, dryRun | - |
| `CreateComponentResponse` | `structure` | requestId, clientToken, componentBuildVersionArn, latestVersionReferences | - |
| `CreateContainerRecipeRequest` | `structure` | containerType, name, description, semanticVersion, components, instanceConfiguration, dockerfileTemplateData, dockerfileTemplateUri, platformOverride, imageOsVersionOverride, parentImage, tags, ... (+4) | - |
| `CreateContainerRecipeResponse` | `structure` | requestId, clientToken, containerRecipeArn, latestVersionReferences | - |
| `CreateDistributionConfigurationRequest` | `structure` | name, description, distributions, tags, clientToken | - |
| `CreateDistributionConfigurationResponse` | `structure` | requestId, clientToken, distributionConfigurationArn | - |
| `CreateImageRequest` | `structure` | imageRecipeArn, containerRecipeArn, distributionConfigurationArn, infrastructureConfigurationArn, imageTestsConfiguration, enhancedImageMetadataEnabled, tags, clientToken, imageScanningConfiguration, workflows, executionRole, loggingConfiguration | - |
| `CreateImageResponse` | `structure` | requestId, clientToken, imageBuildVersionArn, latestVersionReferences | - |
| `CreateImagePipelineRequest` | `structure` | name, description, imageRecipeArn, containerRecipeArn, infrastructureConfigurationArn, distributionConfigurationArn, imageTestsConfiguration, enhancedImageMetadataEnabled, schedule, status, tags, imageTags, ... (+5) | - |
| `CreateImagePipelineResponse` | `structure` | requestId, clientToken, imagePipelineArn | - |
| `CreateImageRecipeRequest` | `structure` | name, description, semanticVersion, components, parentImage, blockDeviceMappings, tags, workingDirectory, additionalInstanceConfiguration, amiTags, clientToken | - |
| `CreateImageRecipeResponse` | `structure` | requestId, clientToken, imageRecipeArn, latestVersionReferences | - |
| `CreateInfrastructureConfigurationRequest` | `structure` | name, description, instanceTypes, instanceProfileName, securityGroupIds, subnetId, logging, keyPair, terminateInstanceOnFailure, snsTopicArn, resourceTags, instanceMetadataOptions, ... (+3) | - |
| `CreateInfrastructureConfigurationResponse` | `structure` | requestId, clientToken, infrastructureConfigurationArn | - |
| `CreateLifecyclePolicyRequest` | `structure` | name, description, status, executionRole, resourceType, policyDetails, resourceSelection, tags, clientToken | - |
| `CreateLifecyclePolicyResponse` | `structure` | clientToken, lifecyclePolicyArn | - |
| `BuildType` | `enum` | USER_INITIATED, SCHEDULED, IMPORT, IMPORT_ISO | - |
| `ComponentFormat` | `enum` | SHELL | - |
| `ComponentStatus` | `enum` | DEPRECATED, DISABLED, ACTIVE | - |
| `ComponentType` | `enum` | BUILD, TEST | - |
| `ContainerRepositoryService` | `enum` | ECR | - |
| `ContainerType` | `enum` | DOCKER | - |
| `DiskImageFormat` | `enum` | VMDK, RAW, VHD | - |
| `EbsVolumeType` | `enum` | STANDARD, IO1, IO2, GP2, GP3, SC1, ST1 | - |
| `ImageScanStatus` | `enum` | PENDING, SCANNING, COLLECTING, COMPLETED, ABANDONED, FAILED, TIMED_OUT | - |
| `ImageSource` | `enum` | AMAZON_MANAGED, AWS_MARKETPLACE, IMPORTED, CUSTOM | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
