# Amazon Elastic Container Registry

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elastic Container Registry Amazon Elastic Container Registry (Amazon ECR) is a managed container image registry service. Customers can use the familiar Docker CLI, or their preferred client, to push, pull, and manage images. Amazon ECR provides a secure, scalable, and reliable registry for your Docker or Open Container Initiative (OCI) images. Amazon ECR supports private repositories with resource-based permissions using IAM so that specific users or Amazon EC2 instances can access repositories and images. Amazon ECR has service endpoints in each supported Region.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-ecr/tests/scenario_test.rs`: run a CI repository lifecycle with repository creation, image metadata, policies/settings, listing, and deletion.
- Backported from `scenario_test.rs`: create, describe, and delete a pull-through cache rule.
- From the AWS documentation and model: support container image registry operations, repository policies, lifecycle rules, scanning/encryption settings, pull-through cache configuration, authorisation tokens, and tag-based inventory.

## Service Identity and Protocol

- AWS model slug: `ecr`
- AWS SDK for Rust slug: `ecr`
- Model version: `2015-09-21`
- Model file: `vendor/api-models-aws/models/ecr/service/2015-09-21/ecr-2015-09-21.json`
- SDK ID: `ECR`
- Endpoint prefix: `api.ecr`
- ARN namespace: `ecr`
- CloudFormation name: `ECR`
- CloudTrail event source: `ecr.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (9), `Put` (9), `Describe` (8), `Delete` (7), `Batch` (4), `List` (4), `Create` (3), `Update` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCheckLayerAvailability`, `BatchDeleteImage`, `BatchGetImage`, `BatchGetRepositoryScanningConfiguration`, `CreatePullThroughCacheRule`, `CreateRepository`, `CreateRepositoryCreationTemplate`, `DeleteLifecyclePolicy`, `DeletePullThroughCacheRule`, `DeleteRegistryPolicy`, `DeleteRepository`, `DeleteRepositoryCreationTemplate`, `DeleteRepositoryPolicy`, `DeleteSigningConfiguration`, `DeregisterPullTimeUpdateExclusion`, `PutAccountSetting`, `PutImage`, `PutImageScanningConfiguration`, `PutImageTagMutability`, `PutLifecyclePolicy`, ... (+13).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeImageReplicationStatus`, `DescribeImageScanFindings`, `DescribeImageSigningStatus`, `DescribeImages`, `DescribePullThroughCacheRules`, `DescribeRegistry`, `DescribeRepositories`, `DescribeRepositoryCreationTemplates`, `GetAccountSetting`, `GetAuthorizationToken`, `GetDownloadUrlForLayer`, `GetLifecyclePolicy`, `GetLifecyclePolicyPreview`, `GetRegistryPolicy`, `GetRegistryScanningConfiguration`, `GetRepositoryPolicy`, `GetSigningConfiguration`, `ListImageReferrers`, `ListImages`, `ListPullTimeUpdateExclusions`, ... (+2).
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchGetRepositoryScanningConfiguration`, `DescribeImageScanFindings`, `GetRegistryScanningConfiguration`, `PutImageScanningConfiguration`, `PutRegistryScanningConfiguration`, `StartImageScan`, `StartLifecyclePolicyPreview`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 58 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `ECR`, `Secrets Manager`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonECR/latest/APIReference/API_GetAuthorizationToken.html
- https://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html
- https://docs.aws.amazon.com/AmazonECR/latest/APIReference/API_ImageScanningConfiguration.html

Research outcomes:
- GetAuthorizationToken returns a base64 token usable for Docker registry login. The token represents the caller's IAM credentials and is valid for 12 hours.
- The registryIds request parameter is deprecated; when omitted, the default registry is used.
- The authorization token size is not fixed, so clients must not assume a maximum token length.
- Lifecycle policies are JSON rule sets evaluated against repository images, then applied by rule priority. Lower priority numbers take precedence.
- All lifecycle policy rules are evaluated at the same time, but an image is expired or archived by at most one rule.
- Images referenced by manifest lists cannot be expired or archived until the manifest list is deleted or archived first.
- Lifecycle policy expiry happens within about 24 hours after images meet criteria and is recorded in CloudTrail.
- Repository image scanning has a scanOnPush setting. If unset, it defaults to false and scans occur only when manually started.

Parity implications:
- Model registries, repositories, image manifests, tags, manifest lists, scan state, auth tokens, and lifecycle policies separately.
- Lifecycle evaluation should apply all rules before priority resolution and preserve image-age, tag-pattern, tag-prefix, manifest-list, and untagged-image constraints.
- Auth tokens need expiry, account/registry scope, and Docker-compatible proxy endpoint data.

## Operation Groups

### Get

- Operations: `GetAccountSetting`, `GetAuthorizationToken`, `GetDownloadUrlForLayer`, `GetLifecyclePolicy`, `GetLifecyclePolicyPreview`, `GetRegistryPolicy`, `GetRegistryScanningConfiguration`, `GetRepositoryPolicy`, `GetSigningConfiguration`
- Traits: `paginated` (1)
- Common required input members in this group: `repositoryName`

### Put

- Operations: `PutAccountSetting`, `PutImage`, `PutImageScanningConfiguration`, `PutImageTagMutability`, `PutLifecyclePolicy`, `PutRegistryPolicy`, `PutRegistryScanningConfiguration`, `PutReplicationConfiguration`, `PutSigningConfiguration`
- Common required input members in this group: `repositoryName`

### Describe

- Operations: `DescribeImageReplicationStatus`, `DescribeImages`, `DescribeImageScanFindings`, `DescribeImageSigningStatus`, `DescribePullThroughCacheRules`, `DescribeRegistry`, `DescribeRepositories`, `DescribeRepositoryCreationTemplates`
- Traits: `paginated` (5)
- Common required input members in this group: `repositoryName`, `imageId`

### Delete

- Operations: `DeleteLifecyclePolicy`, `DeletePullThroughCacheRule`, `DeleteRegistryPolicy`, `DeleteRepository`, `DeleteRepositoryCreationTemplate`, `DeleteRepositoryPolicy`, `DeleteSigningConfiguration`
- Common required input members in this group: `repositoryName`

### Batch

- Operations: `BatchCheckLayerAvailability`, `BatchDeleteImage`, `BatchGetImage`, `BatchGetRepositoryScanningConfiguration`
- Common required input members in this group: `repositoryName`, `imageIds`

### List

- Operations: `ListImageReferrers`, `ListImages`, `ListPullTimeUpdateExclusions`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: `repositoryName`

### Create

- Operations: `CreatePullThroughCacheRule`, `CreateRepository`, `CreateRepositoryCreationTemplate`
- Common required input members in this group: -

### Update

- Operations: `UpdateImageStorageClass`, `UpdatePullThroughCacheRule`, `UpdateRepositoryCreationTemplate`
- Common required input members in this group: -

### Start

- Operations: `StartImageScan`, `StartLifecyclePolicyPreview`
- Common required input members in this group: `repositoryName`

### Complete

- Operations: `CompleteLayerUpload`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterPullTimeUpdateExclusion`
- Common required input members in this group: -

### Initiate

- Operations: `InitiateLayerUpload`
- Common required input members in this group: -

### Register

- Operations: `RegisterPullTimeUpdateExclusion`
- Common required input members in this group: -

### Set

- Operations: `SetRepositoryPolicy`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Upload

- Operations: `UploadLayerPart`
- Common required input members in this group: -

### Validate

- Operations: `ValidatePullThroughCacheRule`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCheckLayerAvailability` | `-` | - | `repositoryName`, `layerDigests` | - | `BatchCheckLayerAvailabilityResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | Checks the availability of one or more image layers in a repository. When an image is pushed to a repository, each image layer is checked to verify if it has been uploaded before. If it has been uploaded, then the im ... |
| `BatchDeleteImage` | `-` | - | `repositoryName`, `imageIds` | - | `BatchDeleteImageResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | Deletes a list of specified images within a repository. Images are specified with either an imageTag or imageDigest . You can remove a tag from an image by specifying the image's tag in your request. When you remove ... |
| `BatchGetImage` | `-` | - | `repositoryName`, `imageIds` | - | `BatchGetImageResponse` | `InvalidParameterException`, `LimitExceededException`, `RepositoryNotFoundException`, `ServerException`, `UnableToGetUpstreamImageException` | Gets detailed information for an image. Images are specified with either an imageTag or imageDigest . When an image is pulled, the BatchGetImage API is called once to retrieve the image manifest. |
| `BatchGetRepositoryScanningConfiguration` | `-` | - | `repositoryNames` | - | `BatchGetRepositoryScanningConfigurationResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Gets the scanning configuration for one or more repositories. |
| `CompleteLayerUpload` | `-` | - | `repositoryName`, `uploadId`, `layerDigests` | - | `CompleteLayerUploadResponse` | `EmptyUploadException`, `InvalidLayerException`, `InvalidParameterException`, `KmsException`, `LayerAlreadyExistsException`, `LayerPartTooSmallException`, `RepositoryNotFoundException`, `ServerException`, `UploadNotFoundException` | Informs Amazon ECR that the image layer upload has completed for a specified registry, repository name, and upload ID. You can optionally provide a sha256 digest of the image layer for data validation purposes. When ... |
| `CreatePullThroughCacheRule` | `-` | - | `ecrRepositoryPrefix`, `upstreamRegistryUrl` | - | `CreatePullThroughCacheRuleResponse` | `InvalidParameterException`, `LimitExceededException`, `PullThroughCacheRuleAlreadyExistsException`, `SecretNotFoundException`, `ServerException`, `UnableToAccessSecretException`, `UnableToDecryptSecretValueException`, `UnsupportedUpstreamRegistryException`, `ValidationException` | Creates a pull through cache rule. A pull through cache rule provides a way to cache images from an upstream registry source in your Amazon ECR private registry. For more information, see Using pull through cache rul ... |
| `CreateRepository` | `-` | - | `repositoryName` | - | `CreateRepositoryResponse` | `InvalidParameterException`, `InvalidTagParameterException`, `KmsException`, `LimitExceededException`, `RepositoryAlreadyExistsException`, `ServerException`, `TooManyTagsException` | Creates a repository. For more information, see Amazon ECR repositories in the Amazon Elastic Container Registry User Guide . |
| `CreateRepositoryCreationTemplate` | `-` | - | `prefix`, `appliedFor` | - | `CreateRepositoryCreationTemplateResponse` | `InvalidParameterException`, `LimitExceededException`, `ServerException`, `TemplateAlreadyExistsException`, `ValidationException` | Creates a repository creation template. This template is used to define the settings for repositories created by Amazon ECR on your behalf. For example, repositories created through pull through cache actions. For mo ... |
| `DeleteLifecyclePolicy` | `-` | - | `repositoryName` | - | `DeleteLifecyclePolicyResponse` | `InvalidParameterException`, `LifecyclePolicyNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Deletes the lifecycle policy associated with the specified repository. |
| `DeletePullThroughCacheRule` | `-` | - | `ecrRepositoryPrefix` | - | `DeletePullThroughCacheRuleResponse` | `InvalidParameterException`, `PullThroughCacheRuleNotFoundException`, `ServerException`, `ValidationException` | Deletes a pull through cache rule. |
| `DeleteRegistryPolicy` | `-` | - | - | - | `DeleteRegistryPolicyResponse` | `InvalidParameterException`, `RegistryPolicyNotFoundException`, `ServerException`, `ValidationException` | Deletes the registry permissions policy. |
| `DeleteRepository` | `-` | - | `repositoryName` | - | `DeleteRepositoryResponse` | `InvalidParameterException`, `KmsException`, `RepositoryNotEmptyException`, `RepositoryNotFoundException`, `ServerException` | Deletes a repository. If the repository isn't empty, you must either delete the contents of the repository or use the force option to delete the repository and have Amazon ECR delete all of its contents on your behalf. |
| `DeleteRepositoryCreationTemplate` | `-` | - | `prefix` | - | `DeleteRepositoryCreationTemplateResponse` | `InvalidParameterException`, `ServerException`, `TemplateNotFoundException`, `ValidationException` | Deletes a repository creation template. |
| `DeleteRepositoryPolicy` | `-` | - | `repositoryName` | - | `DeleteRepositoryPolicyResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `RepositoryPolicyNotFoundException`, `ServerException` | Deletes the repository policy associated with the specified repository. |
| `DeleteSigningConfiguration` | `-` | - | - | - | `DeleteSigningConfigurationResponse` | `ServerException`, `SigningConfigurationNotFoundException`, `ValidationException` | Deletes the registry's signing configuration. Images pushed after deletion of the signing configuration will no longer be automatically signed. For more information, see Managed signing in the Amazon Elastic Containe ... |
| `DeregisterPullTimeUpdateExclusion` | `-` | - | `principalArn` | - | `DeregisterPullTimeUpdateExclusionResponse` | `ExclusionNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `ServerException`, `ValidationException` | Removes a principal from the pull time update exclusion list for a registry. Once removed, Amazon ECR will resume updating the pull time if the specified principal pulls an image. |
| `DescribeImageReplicationStatus` | `-` | - | `repositoryName`, `imageId` | - | `DescribeImageReplicationStatusResponse` | `ImageNotFoundException`, `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Returns the replication status for a specified image. |
| `DescribeImages` | `-` | `paginated` | `repositoryName` | - | `DescribeImagesResponse` | `ImageNotFoundException`, `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | Returns metadata about the images in a repository. Starting with Docker version 1.9, the Docker client compresses image layers before pushing them to a V2 Docker registry. The output of the docker images command show ... |
| `DescribeImageScanFindings` | `-` | `paginated` | `repositoryName`, `imageId` | - | `DescribeImageScanFindingsResponse` | `ImageNotFoundException`, `InvalidParameterException`, `RepositoryNotFoundException`, `ScanNotFoundException`, `ServerException`, `ValidationException` | Returns the scan findings for the specified image. |
| `DescribeImageSigningStatus` | `-` | - | `repositoryName`, `imageId` | - | `DescribeImageSigningStatusResponse` | `ImageNotFoundException`, `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Returns the signing status for a specified image. If the image matched signing rules that reference different signing profiles, a status is returned for each profile. For more information, see Managed signing in the ... |
| `DescribePullThroughCacheRules` | `-` | `paginated` | - | - | `DescribePullThroughCacheRulesResponse` | `InvalidParameterException`, `PullThroughCacheRuleNotFoundException`, `ServerException`, `ValidationException` | Returns the pull through cache rules for a registry. |
| `DescribeRegistry` | `-` | - | - | - | `DescribeRegistryResponse` | `InvalidParameterException`, `ServerException`, `ValidationException` | Describes the settings for a registry. The replication configuration for a repository can be created or updated with the PutReplicationConfiguration API action. |
| `DescribeRepositories` | `-` | `paginated` | - | - | `DescribeRepositoriesResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | Describes image repositories in a registry. |
| `DescribeRepositoryCreationTemplates` | `-` | `paginated` | - | - | `DescribeRepositoryCreationTemplatesResponse` | `InvalidParameterException`, `ServerException`, `ValidationException` | Returns details about the repository creation templates in a registry. The prefixes request parameter can be used to return the details for a specific repository creation template. |
| `GetAccountSetting` | `-` | - | `name` | - | `GetAccountSettingResponse` | `InvalidParameterException`, `ServerException`, `ValidationException` | Retrieves the account setting value for the specified setting name. |
| `GetAuthorizationToken` | `-` | - | - | - | `GetAuthorizationTokenResponse` | `InvalidParameterException`, `ServerException` | Retrieves an authorization token. An authorization token represents your IAM authentication credentials and can be used to access any Amazon ECR registry that your IAM principal has access to. The authorization token ... |
| `GetDownloadUrlForLayer` | `-` | - | `repositoryName`, `layerDigest` | - | `GetDownloadUrlForLayerResponse` | `InvalidParameterException`, `LayerInaccessibleException`, `LayersNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `UnableToGetUpstreamLayerException` | Retrieves the pre-signed Amazon S3 download URL corresponding to an image layer. You can only get URLs for image layers that are referenced in an image. When an image is pulled, the GetDownloadUrlForLayer API is call ... |
| `GetLifecyclePolicy` | `-` | - | `repositoryName` | - | `GetLifecyclePolicyResponse` | `InvalidParameterException`, `LifecyclePolicyNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Retrieves the lifecycle policy for the specified repository. |
| `GetLifecyclePolicyPreview` | `-` | `paginated` | `repositoryName` | - | `GetLifecyclePolicyPreviewResponse` | `InvalidParameterException`, `LifecyclePolicyPreviewNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Retrieves the results of the lifecycle policy preview request for the specified repository. |
| `GetRegistryPolicy` | `-` | - | - | - | `GetRegistryPolicyResponse` | `InvalidParameterException`, `RegistryPolicyNotFoundException`, `ServerException`, `ValidationException` | Retrieves the permissions policy for a registry. |
| `GetRegistryScanningConfiguration` | `-` | - | - | - | `GetRegistryScanningConfigurationResponse` | `InvalidParameterException`, `ServerException`, `ValidationException` | Retrieves the scanning configuration for a registry. |
| `GetRepositoryPolicy` | `-` | - | `repositoryName` | - | `GetRepositoryPolicyResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `RepositoryPolicyNotFoundException`, `ServerException` | Retrieves the repository policy for the specified repository. |
| `GetSigningConfiguration` | `-` | - | - | - | `GetSigningConfigurationResponse` | `InvalidParameterException`, `ServerException`, `SigningConfigurationNotFoundException`, `ValidationException` | Retrieves the registry's signing configuration, which defines rules for automatically signing images using Amazon Web Services Signer. For more information, see Managed signing in the Amazon Elastic Container Registr ... |
| `InitiateLayerUpload` | `-` | - | `repositoryName` | - | `InitiateLayerUploadResponse` | `InvalidParameterException`, `KmsException`, `RepositoryNotFoundException`, `ServerException` | Notifies Amazon ECR that you intend to upload an image layer. When an image is pushed, the InitiateLayerUpload API is called once per image layer that has not already been uploaded. Whether or not an image layer has ... |
| `ListImageReferrers` | `-` | - | `repositoryName`, `subjectId` | - | `ListImageReferrersResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnableToListUpstreamImageReferrersException`, `ValidationException` | Lists the artifacts associated with a specified subject image. The IAM principal invoking this operation must have the ecr:BatchGetImage permission. |
| `ListImages` | `-` | `paginated` | `repositoryName` | - | `ListImagesResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | Lists all the image IDs for the specified repository. You can filter images based on whether or not they are tagged by using the tagStatus filter and specifying either TAGGED , UNTAGGED or ANY . For example, you can ... |
| `ListPullTimeUpdateExclusions` | `-` | - | - | - | `ListPullTimeUpdateExclusionsResponse` | `InvalidParameterException`, `LimitExceededException`, `ServerException`, `ValidationException` | Lists the IAM principals that are excluded from having their image pull times recorded. |
| `ListTagsForResource` | `-` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | List the tags for an Amazon ECR resource. |
| `PutAccountSetting` | `-` | - | `name`, `value` | - | `PutAccountSettingResponse` | `InvalidParameterException`, `LimitExceededException`, `ServerException`, `ValidationException` | Allows you to change the basic scan type version or registry policy scope. |
| `PutImage` | `-` | - | `repositoryName`, `imageManifest` | - | `PutImageResponse` | `ImageAlreadyExistsException`, `ImageDigestDoesNotMatchException`, `ImageTagAlreadyExistsException`, `InvalidParameterException`, `KmsException`, `LayersNotFoundException`, `LimitExceededException`, `ReferencedImagesNotFoundException`, `RepositoryNotFoundException`, `ServerException` | Creates or updates the image manifest and tags associated with an image. When an image is pushed and all new image layers have been uploaded, the PutImage API is called once to create or update the image manifest and ... |
| `PutImageScanningConfiguration` | `-` | - | `repositoryName`, `imageScanningConfiguration` | - | `PutImageScanningConfigurationResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | The PutImageScanningConfiguration API is being deprecated, in favor of specifying the image scanning configuration at the registry level. For more information, see PutRegistryScanningConfiguration . Updates the image ... |
| `PutImageTagMutability` | `-` | - | `repositoryName`, `imageTagMutability` | - | `PutImageTagMutabilityResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | Updates the image tag mutability settings for the specified repository. For more information, see Image tag mutability in the Amazon Elastic Container Registry User Guide . |
| `PutLifecyclePolicy` | `-` | - | `repositoryName`, `lifecyclePolicyText` | - | `PutLifecyclePolicyResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Creates or updates the lifecycle policy for the specified repository. For more information, see Lifecycle policy template . |
| `PutRegistryPolicy` | `-` | - | `policyText` | - | `PutRegistryPolicyResponse` | `InvalidParameterException`, `ServerException`, `ValidationException` | Creates or updates the permissions policy for your registry. A registry policy is used to specify permissions for another Amazon Web Services account and is used when configuring cross-account replication. For more i ... |
| `PutRegistryScanningConfiguration` | `-` | - | - | - | `PutRegistryScanningConfigurationResponse` | `BlockedByOrganizationPolicyException`, `InvalidParameterException`, `ServerException`, `ValidationException` | Creates or updates the scanning configuration for your private registry. |
| `PutReplicationConfiguration` | `-` | - | `replicationConfiguration` | - | `PutReplicationConfigurationResponse` | `InvalidParameterException`, `ServerException`, `ValidationException` | Creates or updates the replication configuration for a registry. The existing replication configuration for a repository can be retrieved with the DescribeRegistry API action. The first time the PutReplicationConfigu ... |
| `PutSigningConfiguration` | `-` | - | `signingConfiguration` | - | `PutSigningConfigurationResponse` | `InvalidParameterException`, `ServerException`, `ValidationException` | Creates or updates the registry's signing configuration, which defines rules for automatically signing images with Amazon Web Services Signer. For more information, see Managed signing in the Amazon Elastic Container ... |
| `RegisterPullTimeUpdateExclusion` | `-` | - | `principalArn` | - | `RegisterPullTimeUpdateExclusionResponse` | `ExclusionAlreadyExistsException`, `InvalidParameterException`, `LimitExceededException`, `ServerException`, `ValidationException` | Adds an IAM principal to the pull time update exclusion list for a registry. Amazon ECR will not record the pull time if an excluded principal pulls an image. |
| `SetRepositoryPolicy` | `-` | - | `repositoryName`, `policyText` | - | `SetRepositoryPolicyResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException` | Applies a repository policy to the specified repository to control access permissions. For more information, see Amazon ECR Repository policies in the Amazon Elastic Container Registry User Guide . |
| `StartImageScan` | `-` | - | `repositoryName`, `imageId` | - | `StartImageScanResponse` | `ImageArchivedException`, `ImageNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedImageTypeException`, `ValidationException` | Starts a basic image vulnerability scan. A basic image scan can only be started once per 24 hours on an individual image. This limit includes if an image was scanned on initial push. You can start up to 100,000 basic ... |
| `StartLifecyclePolicyPreview` | `-` | - | `repositoryName` | - | `StartLifecyclePolicyPreviewResponse` | `InvalidParameterException`, `LifecyclePolicyNotFoundException`, `LifecyclePolicyPreviewInProgressException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Starts a preview of a lifecycle policy for the specified repository. This allows you to see the results before associating the lifecycle policy with the repository. |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InvalidParameterException`, `InvalidTagParameterException`, `RepositoryNotFoundException`, `ServerException`, `TooManyTagsException` | Adds specified tags to a resource with the specified ARN. Existing tags on a resource are not changed if they are not specified in the request parameters. |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InvalidParameterException`, `InvalidTagParameterException`, `RepositoryNotFoundException`, `ServerException`, `TooManyTagsException` | Deletes specified tags from a resource. |
| `UpdateImageStorageClass` | `-` | - | `repositoryName`, `imageId`, `targetStorageClass` | - | `UpdateImageStorageClassResponse` | `ImageNotFoundException`, `ImageStorageClassUpdateNotSupportedException`, `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `ValidationException` | Transitions an image between storage classes. You can transition images from Amazon ECR standard storage class to Amazon ECR archival storage class for long-term storage, or restore archived images back to Amazon ECR ... |
| `UpdatePullThroughCacheRule` | `-` | - | `ecrRepositoryPrefix` | - | `UpdatePullThroughCacheRuleResponse` | `InvalidParameterException`, `PullThroughCacheRuleNotFoundException`, `SecretNotFoundException`, `ServerException`, `UnableToAccessSecretException`, `UnableToDecryptSecretValueException`, `ValidationException` | Updates an existing pull through cache rule. |
| `UpdateRepositoryCreationTemplate` | `-` | - | `prefix` | - | `UpdateRepositoryCreationTemplateResponse` | `InvalidParameterException`, `ServerException`, `TemplateNotFoundException`, `ValidationException` | Updates an existing repository creation template. |
| `UploadLayerPart` | `-` | - | `repositoryName`, `uploadId`, `partFirstByte`, `partLastByte`, `layerPartBlob` | - | `UploadLayerPartResponse` | `InvalidLayerPartException`, `InvalidParameterException`, `KmsException`, `LimitExceededException`, `RepositoryNotFoundException`, `ServerException`, `UploadNotFoundException` | Uploads an image layer part to Amazon ECR. When an image is pushed, each new image layer is uploaded in parts. The maximum size of each image layer part can be 20971520 bytes (or about 20MB). The UploadLayerPart API ... |
| `ValidatePullThroughCacheRule` | `-` | - | `ecrRepositoryPrefix` | - | `ValidatePullThroughCacheRuleResponse` | `InvalidParameterException`, `PullThroughCacheRuleNotFoundException`, `ServerException`, `ValidationException` | Validates an existing pull through cache rule for an upstream registry that requires authentication. This will retrieve the contents of the Amazon Web Services Secrets Manager secret, verify the syntax, and then vali ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BlockedByOrganizationPolicyException` | `structure` | message | The operation did not succeed because the account is managed by a organization policy. |
| `EmptyUploadException` | `structure` | message | The specified layer upload does not contain any layer parts. |
| `ExclusionAlreadyExistsException` | `structure` | message | The specified pull time update exclusion already exists for the registry. |
| `ExclusionNotFoundException` | `structure` | message | The specified pull time update exclusion was not found. |
| `ImageAlreadyExistsException` | `structure` | message | The specified image has already been pushed, and there were no changes to the manifest or image tag after the last push. |
| `ImageArchivedException` | `structure` | message | The specified image is archived and cannot be scanned. |
| `ImageDigestDoesNotMatchException` | `structure` | message | The specified image digest does not match the digest that Amazon ECR calculated for the image. |
| `ImageNotFoundException` | `structure` | message | The image requested does not exist in the specified repository. |
| `ImageStorageClassUpdateNotSupportedException` | `structure` | message | The requested image storage class update is not supported. |
| `ImageTagAlreadyExistsException` | `structure` | message | The specified image is tagged with a tag that already exists. The repository is configured for tag immutability. |
| `InvalidLayerException` | `structure` | message | The layer digest calculation performed by Amazon ECR upon receipt of the image layer does not match the digest specified. |
| `InvalidLayerPartException` | `structure` | registryId, repositoryName, uploadId, lastValidByteReceived, message | The layer part size is not valid, or the first byte specified is not consecutive to the last byte of a previous layer part upload. |
| `InvalidParameterException` | `structure` | message | The specified parameter is invalid. Review the available parameters for the API request. |
| `InvalidTagParameterException` | `structure` | message | An invalid parameter has been specified. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 cha ... |
| `KmsException` | `structure` | message, kmsError | The operation failed due to a KMS exception. |
| `LayerAlreadyExistsException` | `structure` | message | The image layer already exists in the associated repository. |
| `LayerInaccessibleException` | `structure` | message | The specified layer is not available because it is not associated with an image. Unassociated image layers may be cleaned up at any time. |
| `LayerPartTooSmallException` | `structure` | message | Layer parts must be at least 5 MiB in size. |
| `LayersNotFoundException` | `structure` | message | The specified layers could not be found, or the specified layer is not valid for this repository. |
| `LifecyclePolicyNotFoundException` | `structure` | message | The lifecycle policy could not be found, and no policy is set to the repository. |
| `LifecyclePolicyPreviewInProgressException` | `structure` | message | The previous lifecycle policy preview request has not completed. Wait and try again. |
| `LifecyclePolicyPreviewNotFoundException` | `structure` | message | There is no dry run for this repository. |
| `LimitExceededException` | `structure` | message | The operation did not succeed because it would have exceeded a service limit for your account. For more information, see Amazon ECR service quotas in the Am ... |
| `PullThroughCacheRuleAlreadyExistsException` | `structure` | message | A pull through cache rule with these settings already exists for the private registry. |
| `PullThroughCacheRuleNotFoundException` | `structure` | message | The pull through cache rule was not found. Specify a valid pull through cache rule and try again. |
| `ReferencedImagesNotFoundException` | `structure` | message | The manifest list is referencing an image that does not exist. |
| `RegistryPolicyNotFoundException` | `structure` | message | The registry doesn't have an associated registry policy. |
| `RepositoryAlreadyExistsException` | `structure` | message | The specified repository already exists in the specified registry. |
| `RepositoryNotEmptyException` | `structure` | message | The specified repository contains images. To delete a repository that contains images, you must force the deletion with the force parameter. |
| `RepositoryNotFoundException` | `structure` | message | The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct reg ... |
| `RepositoryPolicyNotFoundException` | `structure` | message | The specified repository and registry combination does not have an associated repository policy. |
| `ScanNotFoundException` | `structure` | message | The specified image scan could not be found. Ensure that image scanning is enabled on the repository and try again. |
| `SecretNotFoundException` | `structure` | message | The ARN of the secret specified in the pull through cache rule was not found. Update the pull through cache rule with a valid secret ARN and try again. |
| `ServerException` | `structure` | message | These errors are usually caused by a server-side issue. |
| `SigningConfigurationNotFoundException` | `structure` | message | The specified signing configuration was not found. This occurs when attempting to retrieve or delete a signing configuration that does not exist. |
| `TemplateAlreadyExistsException` | `structure` | message | The repository creation template already exists. Specify a unique prefix and try again. |
| `TemplateNotFoundException` | `structure` | message | The specified repository creation template can't be found. Verify the registry ID and prefix and try again. |
| `TooManyTagsException` | `structure` | message | The list of tags on the repository is over the limit. The maximum number of tags that can be applied to a repository is 50. |
| `UnableToAccessSecretException` | `structure` | message | The secret is unable to be accessed. Verify the resource permissions for the secret and try again. |
| `UnableToDecryptSecretValueException` | `structure` | message | The secret is accessible but is unable to be decrypted. Verify the resource permisisons and try again. |
| `UnableToGetUpstreamImageException` | `structure` | message | The image or images were unable to be pulled using the pull through cache rule. This is usually caused because of an issue with the Secrets Manager secret c ... |
| `UnableToGetUpstreamLayerException` | `structure` | message | There was an issue getting the upstream layer matching the pull through cache rule. |
| `UnableToListUpstreamImageReferrersException` | `structure` | message | The referrer or referrers were unable to be listed using the pull through cache rule. This is usually caused because of an issue with the Secrets Manager se ... |
| `UnsupportedImageTypeException` | `structure` | message | The image is of a type that cannot be scanned. |
| `UnsupportedUpstreamRegistryException` | `structure` | message | The specified upstream registry isn't supported. |
| `UploadNotFoundException` | `structure` | message | The upload could not be found, or the specified upload ID is not valid for this repository. |
| `ValidationException` | `structure` | message | There was an exception validating this request. |
| `ArtifactStatus` | `enum` | ACTIVE, ARCHIVED, ACTIVATING | - |
| `ArtifactStatusFilter` | `enum` | ACTIVE, ARCHIVED, ACTIVATING, ANY | - |
| `EncryptionType` | `enum` | AES256, KMS, KMS_DSSE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
