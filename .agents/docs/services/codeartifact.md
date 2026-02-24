# CodeArtifact

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

CodeArtifact is a fully managed artifact repository compatible with language-native package managers and build tools such as npm, Apache Maven, pip, and dotnet. You can use CodeArtifact to share packages with development teams and pull packages. Packages can be pulled from both public and CodeArtifact repositories. You can also create an upstream relationship between a CodeArtifact repository and another repository, which effectively merges their contents from the point of view of a package manager client. CodeArtifact concepts Repository : A CodeArtifact repository contains a set of package versions, each of which maps to a set of assets, or files.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for CodeArtifact where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for CodeArtifact by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented CodeArtifact workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Get`, `Describe`, `Update` operation families, including `ListAllowedRepositoriesForGroup`, `ListAssociatedPackages`, `ListDomains`, `ListPackageGroups`, `DeleteDomain`, `DeleteDomainPermissionsPolicy`.

## Service Identity and Protocol

- AWS model slug: `codeartifact`
- AWS SDK for Rust slug: `codeartifact`
- Model version: `2018-09-22`
- Model file: `vendor/api-models-aws/models/codeartifact/service/2018-09-22/codeartifact-2018-09-22.json`
- SDK ID: `codeartifact`
- Endpoint prefix: `codeartifact`
- ARN namespace: `codeartifact`
- CloudFormation name: `Codeartifact`
- CloudTrail event source: `codeartifact.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Delete` (7), `Get` (7), `Describe` (5), `Update` (4), `Create` (3), `Put` (3), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateExternalConnection`, `CreateDomain`, `CreatePackageGroup`, `CreateRepository`, `DeleteDomain`, `DeleteDomainPermissionsPolicy`, `DeletePackage`, `DeletePackageGroup`, `DeletePackageVersions`, `DeleteRepository`, `DeleteRepositoryPermissionsPolicy`, `DisassociateExternalConnection`, `PutDomainPermissionsPolicy`, `PutPackageOriginConfiguration`, `PutRepositoryPermissionsPolicy`, `TagResource`, `UntagResource`, `UpdatePackageGroup`, `UpdatePackageGroupOriginConfiguration`, `UpdatePackageVersionsStatus`, ... (+1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDomain`, `DescribePackage`, `DescribePackageGroup`, `DescribePackageVersion`, `DescribeRepository`, `GetAssociatedPackageGroup`, `GetAuthorizationToken`, `GetDomainPermissionsPolicy`, `GetPackageVersionAsset`, `GetPackageVersionReadme`, `GetRepositoryEndpoint`, `GetRepositoryPermissionsPolicy`, `ListAllowedRepositoriesForGroup`, `ListAssociatedPackages`, `ListDomains`, `ListPackageGroups`, `ListPackageVersionAssets`, `ListPackageVersionDependencies`, `ListPackageVersions`, `ListPackages`, ... (+4).
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 48 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `ECR`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html
- https://docs.aws.amazon.com/codeartifact/latest/ug/external-connection.html
- https://docs.aws.amazon.com/codeartifact/latest/ug/maven-snapshots.html

Research outcomes:
- CodeArtifact is a managed package repository service that stores and shares software packages across supported package ecosystems.
- A domain owns repositories and package storage. Repositories belong to a domain.
- Repositories can have upstream repositories. Package requests can resolve through upstream chains when the package is not present locally.
- Repositories can connect to supported public package repositories through external connections.
- Authentication uses CodeArtifact authorisation tokens and package-manager-specific repository endpoints.
- Packages contain versions and assets. Version state and revision metadata determine what can be consumed or modified.
- Maven snapshot versions have special publishing and consumption behaviour with time-based snapshot builds and interactions with upstream and external repositories.
- Removing an external connection changes future package resolution but does not by itself delete packages already cached in CodeArtifact.

Parity implications:
- Model domains, repositories, upstream relationships, external connections, packages, package versions, assets, version status, authorisation tokens, and endpoints separately.
- Package resolution should walk repository, upstream, and external-connection paths according to package-manager rules.
- Publish, dispose, copy, and delete operations should update package-version state and assets independently.

## Operation Groups

### List

- Operations: `ListAllowedRepositoriesForGroup`, `ListAssociatedPackages`, `ListDomains`, `ListPackageGroups`, `ListPackageVersionAssets`, `ListPackageVersionDependencies`, `ListPackageVersions`, `ListPackages`, `ListRepositories`, `ListRepositoriesInDomain`, `ListSubPackageGroups`, `ListTagsForResource`
- Traits: `paginated` (10)
- Common required input members in this group: `domain`, `format`, `originRestrictionType`, `package`, `packageGroup`, `packageVersion`, `repository`, `resourceArn`

### Delete

- Operations: `DeleteDomain`, `DeleteDomainPermissionsPolicy`, `DeletePackage`, `DeletePackageGroup`, `DeletePackageVersions`, `DeleteRepository`, `DeleteRepositoryPermissionsPolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: `domain`, `format`, `package`, `packageGroup`, `repository`, `versions`

### Get

- Operations: `GetAssociatedPackageGroup`, `GetAuthorizationToken`, `GetDomainPermissionsPolicy`, `GetPackageVersionAsset`, `GetPackageVersionReadme`, `GetRepositoryEndpoint`, `GetRepositoryPermissionsPolicy`
- Common required input members in this group: `asset`, `domain`, `format`, `package`, `packageVersion`, `repository`

### Describe

- Operations: `DescribeDomain`, `DescribePackage`, `DescribePackageGroup`, `DescribePackageVersion`, `DescribeRepository`
- Common required input members in this group: `domain`, `format`, `package`, `packageGroup`, `packageVersion`, `repository`

### Update

- Operations: `UpdatePackageGroup`, `UpdatePackageGroupOriginConfiguration`, `UpdatePackageVersionsStatus`, `UpdateRepository`
- Traits: `idempotent` (2)
- Common required input members in this group: `domain`, `format`, `package`, `packageGroup`, `repository`, `targetStatus`, `versions`

### Create

- Operations: `CreateDomain`, `CreatePackageGroup`, `CreateRepository`
- Common required input members in this group: `domain`, `packageGroup`, `repository`

### Put

- Operations: `PutDomainPermissionsPolicy`, `PutPackageOriginConfiguration`, `PutRepositoryPermissionsPolicy`
- Common required input members in this group: `domain`, `format`, `package`, `policyDocument`, `repository`, `restrictions`

### Associate

- Operations: `AssociateExternalConnection`
- Common required input members in this group: `domain`, `externalConnection`, `repository`

### Copy

- Operations: `CopyPackageVersions`
- Common required input members in this group: `destinationRepository`, `domain`, `format`, `package`, `sourceRepository`

### Disassociate

- Operations: `DisassociateExternalConnection`
- Common required input members in this group: `domain`, `externalConnection`, `repository`

### Dispose

- Operations: `DisposePackageVersions`
- Common required input members in this group: `domain`, `format`, `package`, `repository`, `versions`

### Publish

- Operations: `PublishPackageVersion`
- Common required input members in this group: `assetContent`, `assetName`, `assetSHA256`, `domain`, `format`, `package`, `packageVersion`, `repository`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateExternalConnection` | `POST /v1/repository/external-connection` | - | `domain`, `externalConnection`, `repository` | - | `AssociateExternalConnectionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds an existing external connection to a repository. One external connection is allowed per repository. |
| `CopyPackageVersions` | `POST /v1/package/versions/copy` | - | `destinationRepository`, `domain`, `format`, `package`, `sourceRepository` | - | `CopyPackageVersionsResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Copies package versions from one repository to another repository in the same domain. You must specify `versions` or `versionRevisions`. |
| `CreateDomain` | `POST /v1/domain` | - | `domain` | - | `CreateDomainResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a domain. CodeArtifact domains make it easier to manage multiple repositories across an organization. |
| `CreatePackageGroup` | `POST /v1/package-group` | - | `domain`, `packageGroup` | - | `CreatePackageGroupResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a package group. For more information about creating package groups, including example CLI commands, see Create a package group in the CodeArtifact User Guide . |
| `CreateRepository` | `POST /v1/repository` | - | `domain`, `repository` | - | `CreateRepositoryResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a repository. |
| `DeleteDomain` | `DELETE /v1/domain` | - | `domain` | - | `DeleteDomainResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a domain. You cannot delete a domain that contains repositories. |
| `DeleteDomainPermissionsPolicy` | `DELETE /v1/domain/permissions/policy` | - | `domain` | - | `DeleteDomainPermissionsPolicyResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource policy set on a domain. |
| `DeletePackage` | `DELETE /v1/package` | - | `domain`, `format`, `package`, `repository` | - | `DeletePackageResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a package and all associated package versions. A deleted package cannot be restored. |
| `DeletePackageGroup` | `DELETE /v1/package-group` | `idempotent` | `domain`, `packageGroup` | - | `DeletePackageGroupResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a package group. Deleting a package group does not delete packages or package versions associated with the package group. |
| `DeletePackageVersions` | `POST /v1/package/versions/delete` | - | `domain`, `format`, `package`, `repository`, `versions` | - | `DeletePackageVersionsResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes one or more versions of a package. A deleted package version cannot be restored in your repository. |
| `DeleteRepository` | `DELETE /v1/repository` | - | `domain`, `repository` | - | `DeleteRepositoryResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a repository. |
| `DeleteRepositoryPermissionsPolicy` | `DELETE /v1/repository/permissions/policies` | - | `domain`, `repository` | - | `DeleteRepositoryPermissionsPolicyResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource policy that is set on a repository. After a resource policy is deleted, the permissions allowed and denied by the deleted policy are removed. |
| `DescribeDomain` | `GET /v1/domain` | - | `domain` | - | `DescribeDomainResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a DomainDescription object that contains information about the requested domain. |
| `DescribePackage` | `GET /v1/package` | - | `domain`, `format`, `package`, `repository` | - | `DescribePackageResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a PackageDescription object that contains information about the requested package. |
| `DescribePackageGroup` | `GET /v1/package-group` | - | `domain`, `packageGroup` | - | `DescribePackageGroupResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a PackageGroupDescription object that contains information about the requested package group. |
| `DescribePackageVersion` | `GET /v1/package/version` | - | `domain`, `format`, `package`, `packageVersion`, `repository` | - | `DescribePackageVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a PackageVersionDescription object that contains information about the requested package version. |
| `DescribeRepository` | `GET /v1/repository` | - | `domain`, `repository` | - | `DescribeRepositoryResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a `RepositoryDescription` object that contains detailed information about the requested repository. |
| `DisassociateExternalConnection` | `DELETE /v1/repository/external-connection` | - | `domain`, `externalConnection`, `repository` | - | `DisassociateExternalConnectionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes an existing external connection from a repository. |
| `DisposePackageVersions` | `POST /v1/package/versions/dispose` | - | `domain`, `format`, `package`, `repository`, `versions` | - | `DisposePackageVersionsResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the assets in package versions and sets the package versions' status to `Disposed`. A disposed package version cannot be restored in your repository because its assets are deleted. |
| `GetAssociatedPackageGroup` | `GET /v1/get-associated-package-group` | - | `domain`, `format`, `package` | - | `GetAssociatedPackageGroupResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the most closely associated package group to the specified package. This API does not require that the package exist in any repository in the domain. |
| `GetAuthorizationToken` | `POST /v1/authorization-token` | - | `domain` | - | `GetAuthorizationTokenResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates a temporary authorization token for accessing repositories in the domain. This API requires the `codeartifact:GetAuthorizationToken` and `sts:GetServiceBearerToken` permissions. |
| `GetDomainPermissionsPolicy` | `GET /v1/domain/permissions/policy` | - | `domain` | - | `GetDomainPermissionsPolicyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the resource policy attached to the specified domain. The policy is a resource-based policy, not an identity-based policy. |
| `GetPackageVersionAsset` | `GET /v1/package/version/asset` | - | `asset`, `domain`, `format`, `package`, `packageVersion`, `repository` | - | `GetPackageVersionAssetResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an asset (or file) that is in a package. For example, for a Maven package version, use `GetPackageVersionAsset` to download a `JAR` file, a `POM` file, or any other assets in the package version. |
| `GetPackageVersionReadme` | `GET /v1/package/version/readme` | - | `domain`, `format`, `package`, `packageVersion`, `repository` | - | `GetPackageVersionReadmeResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the readme file or descriptive text for a package version. The returned text might contain formatting. |
| `GetRepositoryEndpoint` | `GET /v1/repository/endpoint` | - | `domain`, `format`, `repository` | - | `GetRepositoryEndpointResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the endpoint of a repository for a specific package format. A repository has one endpoint for each package format: `cargo` `generic` `maven` `npm` `nuget` `pypi` `ruby` `swift` |
| `GetRepositoryPermissionsPolicy` | `GET /v1/repository/permissions/policy` | - | `domain`, `repository` | - | `GetRepositoryPermissionsPolicyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the resource policy that is set on a repository. |
| `ListAllowedRepositoriesForGroup` | `GET /v1/package-group-allowed-repositories` | `paginated` | `domain`, `originRestrictionType`, `packageGroup` | - | `ListAllowedRepositoriesForGroupResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists the repositories in the added repositories list of the specified restriction type for a package group. For more information about restriction types and added repository lists, see Package group origin controls in the CodeArtifact User Guide . |
| `ListAssociatedPackages` | `GET /v1/list-associated-packages` | `paginated` | `domain`, `packageGroup` | - | `ListAssociatedPackagesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of packages associated with the requested package group. For information package group association and matching, see Package group definition syntax and matching behavior in the CodeArtifact User Guide . |
| `ListDomains` | `POST /v1/domains` | `paginated` | - | - | `ListDomainsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of DomainSummary objects for all domains owned by the Amazon Web Services account that makes this call. Each returned `DomainSummary` object contains information about a domain. |
| `ListPackageGroups` | `POST /v1/package-groups` | `paginated` | `domain` | - | `ListPackageGroupsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of package groups in the requested domain. |
| `ListPackageVersionAssets` | `POST /v1/package/version/assets` | `paginated` | `domain`, `format`, `package`, `packageVersion`, `repository` | - | `ListPackageVersionAssetsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of AssetSummary objects for assets in a package version. |
| `ListPackageVersionDependencies` | `POST /v1/package/version/dependencies` | - | `domain`, `format`, `package`, `packageVersion`, `repository` | - | `ListPackageVersionDependenciesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the direct dependencies for a package version. The dependencies are returned as PackageDependency objects. |
| `ListPackageVersions` | `POST /v1/package/versions` | `paginated` | `domain`, `format`, `package`, `repository` | - | `ListPackageVersionsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of PackageVersionSummary objects for package versions in a repository that match the request parameters. Package versions of all statuses will be returned by default when calling `list-package-versions` with no `--status` parameter. |
| `ListPackages` | `POST /v1/packages` | `paginated` | `domain`, `repository` | - | `ListPackagesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of PackageSummary objects for packages in a repository that match the request parameters. |
| `ListRepositories` | `POST /v1/repositories` | `paginated` | - | - | `ListRepositoriesResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of RepositorySummary objects. Each `RepositorySummary` contains information about a repository in the specified Amazon Web Services account and that matches the input parameters. |
| `ListRepositoriesInDomain` | `POST /v1/domain/repositories` | `paginated` | `domain` | - | `ListRepositoriesInDomainResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of RepositorySummary objects. Each `RepositorySummary` contains information about a repository in the specified domain and that matches the input parameters. |
| `ListSubPackageGroups` | `POST /v1/package-groups/sub-groups` | `paginated` | `domain`, `packageGroup` | - | `ListSubPackageGroupsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of direct children of the specified package group. For information package group hierarchy, see Package group definition syntax and matching behavior in the CodeArtifact User Guide . |
| `ListTagsForResource` | `POST /v1/tags` | - | `resourceArn` | - | `ListTagsForResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about Amazon Web Services tags for a specified Amazon Resource Name (ARN) in CodeArtifact. |
| `PublishPackageVersion` | `POST /v1/package/version/publish` | - | `assetContent`, `assetName`, `assetSHA256`, `domain`, `format`, `package`, `packageVersion`, `repository` | - | `PublishPackageVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new package version containing one or more assets (or files). The `unfinished` flag can be used to keep the package version in the `Unfinished` state until all of its assets have been uploaded (see Package version status in the CodeArtifact user... |
| `PutDomainPermissionsPolicy` | `PUT /v1/domain/permissions/policy` | - | `domain`, `policyDocument` | - | `PutDomainPermissionsPolicyResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Sets a resource policy on a domain that specifies permissions to access it. When you call `PutDomainPermissionsPolicy`, the resource policy on the domain is ignored when evaluting permissions. |
| `PutPackageOriginConfiguration` | `POST /v1/package` | - | `domain`, `format`, `package`, `repository`, `restrictions` | - | `PutPackageOriginConfigurationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets the package origin configuration for a package. The package origin configuration determines how new versions of a package can be added to a repository. |
| `PutRepositoryPermissionsPolicy` | `PUT /v1/repository/permissions/policy` | - | `domain`, `policyDocument`, `repository` | - | `PutRepositoryPermissionsPolicyResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Sets the resource policy on a repository that specifies permissions to access it. When you call `PutRepositoryPermissionsPolicy`, the resource policy on the repository is ignored when evaluting permissions. |
| `TagResource` | `POST /v1/tag` | - | `resourceArn`, `tags` | - | `TagResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a resource in CodeArtifact. |
| `UntagResource` | `POST /v1/untag` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResult` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a resource in CodeArtifact. |
| `UpdatePackageGroup` | `PUT /v1/package-group` | `idempotent` | `domain`, `packageGroup` | - | `UpdatePackageGroupResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a package group. This API cannot be used to update a package group's origin configuration or pattern. |
| `UpdatePackageGroupOriginConfiguration` | `PUT /v1/package-group-origin-configuration` | `idempotent` | `domain`, `packageGroup` | - | `UpdatePackageGroupOriginConfigurationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the package origin configuration for a package group. The package origin configuration determines how new versions of a package can be added to a repository. |
| `UpdatePackageVersionsStatus` | `POST /v1/package/versions/update_status` | - | `domain`, `format`, `package`, `repository`, `targetStatus`, `versions` | - | `UpdatePackageVersionsStatusResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the status of one or more versions of a package. Using `UpdatePackageVersionsStatus`, you can update the status of package versions to `Archived`, `Published`, or `Unlisted`. |
| `UpdateRepository` | `PUT /v1/repository` | - | `domain`, `repository` | - | `UpdateRepositoryResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update the properties of a repository. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The operation did not succeed because of an unauthorized access attempt. |
| `ValidationException` | `structure` | `message`, `reason` | The operation did not succeed because a parameter in the request was sent with an invalid value. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | The operation did not succeed because too many requests are sent to the service. |
| `InternalServerException` | `structure` | `message` | The operation did not succeed because of an error that occurred inside CodeArtifact. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The operation did not succeed because the resource requested is not found in the service. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The operation did not succeed because prerequisites are not met. |
| `ServiceQuotaExceededException` | `structure` | `message`, `resourceId`, `resourceType` | The operation did not succeed because it would have exceeded a service limit for your account. |
| `AssociateExternalConnectionRequest` | `structure` | `domain`, `domainOwner`, `externalConnection`, `repository` | - |
| `AssociateExternalConnectionResult` | `structure` | `repository` | - |
| `CopyPackageVersionsRequest` | `structure` | `allowOverwrite`, `destinationRepository`, `domain`, `domainOwner`, `format`, `includeFromUpstream`, `namespace`, `package`, `sourceRepository`, `versionRevisions`, `versions` | - |
| `CopyPackageVersionsResult` | `structure` | `failedVersions`, `successfulVersions` | - |
| `CreateDomainRequest` | `structure` | `domain`, `encryptionKey`, `tags` | - |
| `CreateDomainResult` | `structure` | `domain` | - |
| `CreatePackageGroupRequest` | `structure` | `contactInfo`, `description`, `domain`, `domainOwner`, `packageGroup`, `tags` | - |
| `CreatePackageGroupResult` | `structure` | `packageGroup` | - |
| `CreateRepositoryRequest` | `structure` | `description`, `domain`, `domainOwner`, `repository`, `tags`, `upstreams` | - |
| `CreateRepositoryResult` | `structure` | `repository` | - |
| `DeleteDomainRequest` | `structure` | `domain`, `domainOwner` | - |
| `DeleteDomainResult` | `structure` | `domain` | - |
| `DeleteDomainPermissionsPolicyRequest` | `structure` | `domain`, `domainOwner`, `policyRevision` | - |
| `DeleteDomainPermissionsPolicyResult` | `structure` | `policy` | - |
| `DeletePackageRequest` | `structure` | `domain`, `domainOwner`, `format`, `namespace`, `package`, `repository` | - |
| `DeletePackageResult` | `structure` | `deletedPackage` | - |
| `DeletePackageGroupRequest` | `structure` | `domain`, `domainOwner`, `packageGroup` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
