# AWS Elemental MediaPackage VOD

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Elemental MediaPackage VOD

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Elemental MediaPackage VOD workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Describe`, `Configure` operation families, including `ListAssets`, `ListPackagingConfigurations`, `ListPackagingGroups`, `ListTagsForResource`, `CreateAsset`, `CreatePackagingConfiguration`.

## Service Identity and Protocol

- AWS model slug: `mediapackage-vod`
- AWS SDK for Rust slug: `mediapackagevod`
- Model version: `2018-11-07`
- Model file: `vendor/api-models-aws/models/mediapackage-vod/service/2018-11-07/mediapackage-vod-2018-11-07.json`
- SDK ID: `MediaPackage Vod`
- Endpoint prefix: `mediapackage-vod`
- ARN namespace: `mediapackage-vod`
- CloudFormation name: `MediaPackageVod`
- CloudTrail event source: `mediapackagevod.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (3), `Delete` (3), `Describe` (3), `Configure` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAsset`, `CreatePackagingConfiguration`, `CreatePackagingGroup`, `DeleteAsset`, `DeletePackagingConfiguration`, `DeletePackagingGroup`, `TagResource`, `UntagResource`, `UpdatePackagingGroup`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAsset`, `DescribePackagingConfiguration`, `DescribePackagingGroup`, `ListAssets`, `ListPackagingConfigurations`, `ListPackagingGroups`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### List

- Operations: `ListAssets`, `ListPackagingConfigurations`, `ListPackagingGroups`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: `ResourceArn`

### Create

- Operations: `CreateAsset`, `CreatePackagingConfiguration`, `CreatePackagingGroup`
- Common required input members in this group: `Id`, `PackagingGroupId`, `SourceArn`, `SourceRoleArn`

### Delete

- Operations: `DeleteAsset`, `DeletePackagingConfiguration`, `DeletePackagingGroup`
- Common required input members in this group: `Id`

### Describe

- Operations: `DescribeAsset`, `DescribePackagingConfiguration`, `DescribePackagingGroup`
- Common required input members in this group: `Id`

### Configure

- Operations: `ConfigureLogs`
- Common required input members in this group: `Id`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdatePackagingGroup`
- Common required input members in this group: `Id`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ConfigureLogs` | `PUT /packaging_groups/{Id}/configure_logs` | - | `Id` | - | `ConfigureLogsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Changes the packaging group's properities to configure log subscription |
| `CreateAsset` | `POST /assets` | - | `Id`, `PackagingGroupId`, `SourceArn`, `SourceRoleArn` | - | `CreateAssetResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Creates a new MediaPackage VOD Asset resource. |
| `CreatePackagingConfiguration` | `POST /packaging_configurations` | - | `Id`, `PackagingGroupId` | - | `CreatePackagingConfigurationResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Creates a new MediaPackage VOD PackagingConfiguration resource. |
| `CreatePackagingGroup` | `POST /packaging_groups` | - | `Id` | - | `CreatePackagingGroupResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Creates a new MediaPackage VOD PackagingGroup resource. |
| `DeleteAsset` | `DELETE /assets/{Id}` | - | `Id` | - | `DeleteAssetResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Deletes an existing MediaPackage VOD Asset resource. |
| `DeletePackagingConfiguration` | `DELETE /packaging_configurations/{Id}` | - | `Id` | - | `DeletePackagingConfigurationResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Deletes a MediaPackage VOD PackagingConfiguration resource. |
| `DeletePackagingGroup` | `DELETE /packaging_groups/{Id}` | - | `Id` | - | `DeletePackagingGroupResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Deletes a MediaPackage VOD PackagingGroup resource. |
| `DescribeAsset` | `GET /assets/{Id}` | - | `Id` | - | `DescribeAssetResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a description of a MediaPackage VOD Asset resource. |
| `DescribePackagingConfiguration` | `GET /packaging_configurations/{Id}` | - | `Id` | - | `DescribePackagingConfigurationResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a description of a MediaPackage VOD PackagingConfiguration resource. |
| `DescribePackagingGroup` | `GET /packaging_groups/{Id}` | - | `Id` | - | `DescribePackagingGroupResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a description of a MediaPackage VOD PackagingGroup resource. |
| `ListAssets` | `GET /assets` | `paginated` | - | - | `ListAssetsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a collection of MediaPackage VOD Asset resources. |
| `ListPackagingConfigurations` | `GET /packaging_configurations` | `paginated` | - | - | `ListPackagingConfigurationsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a collection of MediaPackage VOD PackagingConfiguration resources. |
| `ListPackagingGroups` | `GET /packaging_groups` | `paginated` | - | - | `ListPackagingGroupsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a collection of MediaPackage VOD PackagingGroup resources. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | - | Returns a list of the tags assigned to the specified resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | - | Adds tags to the specified resource. You can specify one or more tags to add. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | - | Removes tags from the specified resource. You can specify one or more tags to remove. |
| `UpdatePackagingGroup` | `PUT /packaging_groups/{Id}` | - | `Id` | - | `UpdatePackagingGroupResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Updates a specific packaging group. You can't change the id attribute or any other system-generated attributes. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListAssets` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `PackagingGroupId -> packagingGroupId` | - | - |
| `ListPackagingConfigurations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `PackagingGroupId -> packagingGroupId` | - | - |
| `ListPackagingGroups` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ForbiddenException` | `structure` | `Message` | The client is not authorized to access the requested resource. |
| `InternalServerErrorException` | `structure` | `Message` | An unexpected error occurred. |
| `NotFoundException` | `structure` | `Message` | The requested resource does not exist. |
| `ServiceUnavailableException` | `structure` | `Message` | An unexpected error occurred. |
| `TooManyRequestsException` | `structure` | `Message` | The client has exceeded their resource or throttling limits. |
| `UnprocessableEntityException` | `structure` | `Message` | The parameters sent in the request are not valid. |
| `ConfigureLogsRequest` | `structure` | `EgressAccessLogs`, `Id` | The option to configure log subscription. |
| `ConfigureLogsResponse` | `structure` | `Arn`, `Authorization`, `CreatedAt`, `DomainName`, `EgressAccessLogs`, `Id`, `Tags` | - |
| `CreateAssetRequest` | `structure` | `Id`, `PackagingGroupId`, `ResourceId`, `SourceArn`, `SourceRoleArn`, `Tags` | A new MediaPackage VOD Asset configuration. |
| `CreateAssetResponse` | `structure` | `Arn`, `CreatedAt`, `EgressEndpoints`, `Id`, `PackagingGroupId`, `ResourceId`, `SourceArn`, `SourceRoleArn`, `Tags` | - |
| `CreatePackagingConfigurationRequest` | `structure` | `CmafPackage`, `DashPackage`, `HlsPackage`, `Id`, `MssPackage`, `PackagingGroupId`, `Tags` | A new MediaPackage VOD PackagingConfiguration resource configuration. |
| `CreatePackagingConfigurationResponse` | `structure` | `Arn`, `CmafPackage`, `CreatedAt`, `DashPackage`, `HlsPackage`, `Id`, `MssPackage`, `PackagingGroupId`, `Tags` | - |
| `CreatePackagingGroupRequest` | `structure` | `Authorization`, `EgressAccessLogs`, `Id`, `Tags` | A new MediaPackage VOD PackagingGroup resource configuration. |
| `CreatePackagingGroupResponse` | `structure` | `Arn`, `Authorization`, `CreatedAt`, `DomainName`, `EgressAccessLogs`, `Id`, `Tags` | - |
| `DeleteAssetRequest` | `structure` | `Id` | - |
| `DeleteAssetResponse` | `structure` | - | - |
| `DeletePackagingConfigurationRequest` | `structure` | `Id` | - |
| `DeletePackagingConfigurationResponse` | `structure` | - | - |
| `DeletePackagingGroupRequest` | `structure` | `Id` | - |
| `DeletePackagingGroupResponse` | `structure` | - | - |
| `DescribeAssetRequest` | `structure` | `Id` | - |
| `DescribeAssetResponse` | `structure` | `Arn`, `CreatedAt`, `EgressEndpoints`, `Id`, `PackagingGroupId`, `ResourceId`, `SourceArn`, `SourceRoleArn`, `Tags` | - |
| `DescribePackagingConfigurationRequest` | `structure` | `Id` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
