# Amazon Elastic Container Registry Public

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elastic Container Registry Public Amazon Elastic Container Registry Public (Amazon ECR Public) is a managed container image registry service. Amazon ECR provides both public and private registries to host your container images. You can use the Docker CLI or your preferred client to push, pull, and manage images. Amazon ECR provides a secure, scalable, and reliable registry for your Docker or Open Container Initiative (OCI) images. Amazon ECR supports public repositories with this API.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Elastic Container Registry Public workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Get`, `Put`, `Batch`, `Delete` operation families, including `DescribeImageTags`, `DescribeImages`, `DescribeRegistries`, `DescribeRepositories`, `GetAuthorizationToken`, `GetRegistryCatalogData`.

## Service Identity and Protocol

- AWS model slug: `ecr-public`
- AWS SDK for Rust slug: `ecrpublic`
- Model version: `2020-10-30`
- Model file: `vendor/api-models-aws/models/ecr-public/service/2020-10-30/ecr-public-2020-10-30.json`
- SDK ID: `ECR PUBLIC`
- Endpoint prefix: `api.ecr-public`
- ARN namespace: `ecr-public`
- CloudFormation name: `ECRPUBLIC`
- CloudTrail event source: `ecrpublic.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (4), `Get` (4), `Put` (3), `Batch` (2), `Delete` (2), `Complete` (1), `Create` (1), `Initiate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCheckLayerAvailability`, `BatchDeleteImage`, `CreateRepository`, `DeleteRepository`, `DeleteRepositoryPolicy`, `PutImage`, `PutRegistryCatalogData`, `PutRepositoryCatalogData`, `SetRepositoryPolicy`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeImageTags`, `DescribeImages`, `DescribeRegistries`, `DescribeRepositories`, `GetAuthorizationToken`, `GetRegistryCatalogData`, `GetRepositoryCatalogData`, `GetRepositoryPolicy`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `ECR`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonECR/latest/public/public-registry-auth.html
- https://docs.aws.amazon.com/AmazonECR/latest/userguide/getting-started-cli.html

Research outcomes:
- ECR Public hosts public container image repositories and exposes public registry endpoints.
- Docker clients authenticate to ECR Public using `GetAuthorizationToken`, `get-login-password`, or HTTP API authentication.
- Repositories contain images addressable by tag or digest.
- Image lifecycle includes push, pull, delete image, and delete repository operations.
- Public repositories are discoverable differently from private ECR repositories and have public gallery semantics.
- Registry authentication is distinct from repository policy and IAM control-plane permissions.

Parity implications:
- Model public repositories, image manifests, layers, tags, digests, registry aliases/gallery metadata, and auth tokens separately.
- Pull by digest and tag should resolve against image manifest state.
- Public registry auth and private ECR auth should not be conflated.

## Operation Groups

### Describe

- Operations: `DescribeImages`, `DescribeImageTags`, `DescribeRegistries`, `DescribeRepositories`
- Traits: `paginated` (4)
- Common required input members in this group: `repositoryName`

### Get

- Operations: `GetAuthorizationToken`, `GetRegistryCatalogData`, `GetRepositoryCatalogData`, `GetRepositoryPolicy`
- Common required input members in this group: `repositoryName`

### Put

- Operations: `PutImage`, `PutRegistryCatalogData`, `PutRepositoryCatalogData`
- Common required input members in this group: `repositoryName`

### Batch

- Operations: `BatchCheckLayerAvailability`, `BatchDeleteImage`
- Common required input members in this group: `repositoryName`

### Delete

- Operations: `DeleteRepository`, `DeleteRepositoryPolicy`
- Common required input members in this group: `repositoryName`

### Complete

- Operations: `CompleteLayerUpload`
- Common required input members in this group: -

### Create

- Operations: `CreateRepository`
- Common required input members in this group: -

### Initiate

- Operations: `InitiateLayerUpload`
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
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

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCheckLayerAvailability` | `-` | - | `repositoryName`, `layerDigests` | - | `BatchCheckLayerAvailabilityResponse` | `InvalidParameterException`, `RegistryNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Checks the availability of one or more image layers that are within a repository in a public registry. When an image is pushed to a repository, each image layer is checked to verify if it has been uploaded before. If ... |
| `BatchDeleteImage` | `-` | - | `repositoryName`, `imageIds` | - | `BatchDeleteImageResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Deletes a list of specified images that are within a repository in a public registry. Images are specified with either an imageTag or imageDigest . You can remove a tag from an image by specifying the image's tag in ... |
| `CompleteLayerUpload` | `-` | - | `repositoryName`, `uploadId`, `layerDigests` | - | `CompleteLayerUploadResponse` | `EmptyUploadException`, `InvalidLayerException`, `InvalidParameterException`, `LayerAlreadyExistsException`, `LayerPartTooSmallException`, `RegistryNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException`, `UploadNotFoundException` | Informs Amazon ECR that the image layer upload is complete for a specified public registry, repository name, and upload ID. You can optionally provide a sha256 digest of the image layer for data validation purposes. ... |
| `CreateRepository` | `-` | - | `repositoryName` | - | `CreateRepositoryResponse` | `InvalidParameterException`, `InvalidTagParameterException`, `LimitExceededException`, `RepositoryAlreadyExistsException`, `ServerException`, `TooManyTagsException`, `UnsupportedCommandException` | Creates a repository in a public registry. For more information, see Amazon ECR repositories in the Amazon Elastic Container Registry User Guide . |
| `DeleteRepository` | `-` | - | `repositoryName` | - | `DeleteRepositoryResponse` | `InvalidParameterException`, `RepositoryNotEmptyException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Deletes a repository in a public registry. If the repository contains images, you must either manually delete all images in the repository or use the force option. This option deletes all images on your behalf before ... |
| `DeleteRepositoryPolicy` | `-` | - | `repositoryName` | - | `DeleteRepositoryPolicyResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `RepositoryPolicyNotFoundException`, `ServerException`, `UnsupportedCommandException` | Deletes the repository policy that's associated with the specified repository. |
| `DescribeImages` | `-` | `paginated` | `repositoryName` | - | `DescribeImagesResponse` | `ImageNotFoundException`, `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Returns metadata that's related to the images in a repository in a public registry. Beginning with Docker version 1.9, the Docker client compresses image layers before pushing them to a V2 Docker registry. The output ... |
| `DescribeImageTags` | `-` | `paginated` | `repositoryName` | - | `DescribeImageTagsResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Returns the image tag details for a repository in a public registry. |
| `DescribeRegistries` | `-` | `paginated` | - | - | `DescribeRegistriesResponse` | `InvalidParameterException`, `ServerException`, `UnsupportedCommandException` | Returns details for a public registry. |
| `DescribeRepositories` | `-` | `paginated` | - | - | `DescribeRepositoriesResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Describes repositories that are in a public registry. |
| `GetAuthorizationToken` | `-` | - | - | - | `GetAuthorizationTokenResponse` | `InvalidParameterException`, `ServerException`, `UnsupportedCommandException` | Retrieves an authorization token. An authorization token represents your IAM authentication credentials. You can use it to access any Amazon ECR registry that your IAM principal has access to. The authorization token ... |
| `GetRegistryCatalogData` | `-` | - | - | - | `GetRegistryCatalogDataResponse` | `ServerException`, `UnsupportedCommandException` | Retrieves catalog metadata for a public registry. |
| `GetRepositoryCatalogData` | `-` | - | `repositoryName` | - | `GetRepositoryCatalogDataResponse` | `InvalidParameterException`, `RepositoryCatalogDataNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Retrieve catalog metadata for a repository in a public registry. This metadata is displayed publicly in the Amazon ECR Public Gallery. |
| `GetRepositoryPolicy` | `-` | - | `repositoryName` | - | `GetRepositoryPolicyResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `RepositoryPolicyNotFoundException`, `ServerException`, `UnsupportedCommandException` | Retrieves the repository policy for the specified repository. |
| `InitiateLayerUpload` | `-` | - | `repositoryName` | - | `InitiateLayerUploadResponse` | `InvalidParameterException`, `RegistryNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Notifies Amazon ECR that you intend to upload an image layer. When an image is pushed, the InitiateLayerUpload API is called once for each image layer that hasn't already been uploaded. Whether an image layer uploads ... |
| `ListTagsForResource` | `-` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | List the tags for an Amazon ECR Public resource. |
| `PutImage` | `-` | - | `repositoryName`, `imageManifest` | - | `PutImageResponse` | `ImageAlreadyExistsException`, `ImageDigestDoesNotMatchException`, `ImageTagAlreadyExistsException`, `InvalidParameterException`, `LayersNotFoundException`, `LimitExceededException`, `ReferencedImagesNotFoundException`, `RegistryNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Creates or updates the image manifest and tags that are associated with an image. When an image is pushed and all new image layers have been uploaded, the PutImage API is called once to create or update the image man ... |
| `PutRegistryCatalogData` | `-` | - | - | - | `PutRegistryCatalogDataResponse` | `InvalidParameterException`, `ServerException`, `UnsupportedCommandException` | Create or update the catalog data for a public registry. |
| `PutRepositoryCatalogData` | `-` | - | `repositoryName`, `catalogData` | - | `PutRepositoryCatalogDataResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Creates or updates the catalog data for a repository in a public registry. |
| `SetRepositoryPolicy` | `-` | - | `repositoryName`, `policyText` | - | `SetRepositoryPolicyResponse` | `InvalidParameterException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException` | Applies a repository policy to the specified public repository to control access permissions. For more information, see Amazon ECR Repository Policies in the Amazon Elastic Container Registry User Guide . |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InvalidParameterException`, `InvalidTagParameterException`, `RepositoryNotFoundException`, `ServerException`, `TooManyTagsException`, `UnsupportedCommandException` | Associates the specified tags to a resource with the specified resourceArn . If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags associ ... |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InvalidParameterException`, `InvalidTagParameterException`, `RepositoryNotFoundException`, `ServerException`, `TooManyTagsException`, `UnsupportedCommandException` | Deletes specified tags from a resource. |
| `UploadLayerPart` | `-` | - | `repositoryName`, `uploadId`, `partFirstByte`, `partLastByte`, `layerPartBlob` | - | `UploadLayerPartResponse` | `InvalidLayerPartException`, `InvalidParameterException`, `LimitExceededException`, `RegistryNotFoundException`, `RepositoryNotFoundException`, `ServerException`, `UnsupportedCommandException`, `UploadNotFoundException` | Uploads an image layer part to Amazon ECR. When an image is pushed, each new image layer is uploaded in parts. The maximum size of each image layer part can be 20971520 bytes (about 20MB). The UploadLayerPart API is ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `EmptyUploadException` | `structure` | message | The specified layer upload doesn't contain any layer parts. |
| `ImageAlreadyExistsException` | `structure` | message | The specified image has already been pushed, and there were no changes to the manifest or image tag after the last push. |
| `ImageDigestDoesNotMatchException` | `structure` | message | The specified image digest doesn't match the digest that Amazon ECR calculated for the image. |
| `ImageNotFoundException` | `structure` | message | The image requested doesn't exist in the specified repository. |
| `ImageTagAlreadyExistsException` | `structure` | message | The specified image is tagged with a tag that already exists. The repository is configured for tag immutability. |
| `InvalidLayerException` | `structure` | message | The layer digest calculation performed by Amazon ECR when the image layer doesn't match the digest specified. |
| `InvalidLayerPartException` | `structure` | registryId, repositoryName, uploadId, lastValidByteReceived, message | The layer part size isn't valid, or the first byte specified isn't consecutive to the last byte of a previous layer part upload. |
| `InvalidParameterException` | `structure` | message | The specified parameter is invalid. Review the available parameters for the API request. |
| `InvalidTagParameterException` | `structure` | message | An invalid parameter has been specified. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 cha ... |
| `LayerAlreadyExistsException` | `structure` | message | The image layer already exists in the associated repository. |
| `LayerPartTooSmallException` | `structure` | message | Layer parts must be at least 5 MiB in size. |
| `LayersNotFoundException` | `structure` | message | The specified layers can't be found, or the specified layer isn't valid for this repository. |
| `LimitExceededException` | `structure` | message | The operation didn't succeed because it would have exceeded a service limit for your account. For more information, see Amazon ECR Service Quotas in the Ama ... |
| `ReferencedImagesNotFoundException` | `structure` | message | The manifest list is referencing an image that doesn't exist. |
| `RegistryNotFoundException` | `structure` | message | The registry doesn't exist. |
| `RepositoryAlreadyExistsException` | `structure` | message | The specified repository already exists in the specified registry. |
| `RepositoryCatalogDataNotFoundException` | `structure` | message | The repository catalog data doesn't exist. |
| `RepositoryNotEmptyException` | `structure` | message | The specified repository contains images. To delete a repository that contains images, you must force the deletion with the force parameter. |
| `RepositoryNotFoundException` | `structure` | message | The specified repository can't be found. Check the spelling of the specified repository and ensure that you're performing operations on the correct registry. |
| `RepositoryPolicyNotFoundException` | `structure` | message | The specified repository and registry combination doesn't have an associated repository policy. |
| `ServerException` | `structure` | message | These errors are usually caused by a server-side issue. |
| `TooManyTagsException` | `structure` | message | The list of tags on the repository is over the limit. The maximum number of tags that can be applied to a repository is 50. |
| `UnsupportedCommandException` | `structure` | message | The action isn't supported in this Region. |
| `UploadNotFoundException` | `structure` | message | The upload can't be found, or the specified upload ID isn't valid for this repository. |
| `BatchCheckLayerAvailabilityRequest` | `structure` | registryId, repositoryName, layerDigests | - |
| `BatchCheckLayerAvailabilityResponse` | `structure` | layers, failures | - |
| `BatchDeleteImageRequest` | `structure` | registryId, repositoryName, imageIds | - |
| `BatchDeleteImageResponse` | `structure` | imageIds, failures | - |
| `CompleteLayerUploadRequest` | `structure` | registryId, repositoryName, uploadId, layerDigests | - |
| `CompleteLayerUploadResponse` | `structure` | registryId, repositoryName, uploadId, layerDigest | - |
| `CreateRepositoryRequest` | `structure` | repositoryName, catalogData, tags | - |
| `CreateRepositoryResponse` | `structure` | repository, catalogData | - |
| `DeleteRepositoryRequest` | `structure` | registryId, repositoryName, force | - |
| `DeleteRepositoryResponse` | `structure` | repository | - |
| `DeleteRepositoryPolicyRequest` | `structure` | registryId, repositoryName | - |
| `DeleteRepositoryPolicyResponse` | `structure` | registryId, repositoryName, policyText | - |
| `DescribeImagesRequest` | `structure` | registryId, repositoryName, imageIds, nextToken, maxResults | - |
| `DescribeImagesResponse` | `structure` | imageDetails, nextToken | - |
| `DescribeImageTagsRequest` | `structure` | registryId, repositoryName, nextToken, maxResults | - |
| `DescribeImageTagsResponse` | `structure` | imageTagDetails, nextToken | - |
| `ImageFailureCode` | `enum` | InvalidImageDigest, InvalidImageTag, ImageTagDoesNotMatchDigest, ImageNotFound, MissingDigestAndTag, ImageReferencedByManifestList, KmsError | - |
| `LayerAvailability` | `enum` | AVAILABLE, UNAVAILABLE | - |
| `LayerFailureCode` | `enum` | InvalidLayerDigest, MissingLayerDigest | - |
| `RegistryAliasStatus` | `enum` | ACTIVE, PENDING, REJECTED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
