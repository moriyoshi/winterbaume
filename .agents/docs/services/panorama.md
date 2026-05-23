# AWS Panorama

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Panorama Overview This is the AWS Panorama API Reference . For an introduction to the service, see What is AWS Panorama? in the AWS Panorama Developer Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Panorama workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Deregister` operation families, including `ListApplicationInstanceDependencies`, `ListApplicationInstanceNodeInstances`, `ListApplicationInstances`, `ListDevices`, `DescribeApplicationInstance`, `DescribeApplicationInstanceDetails`.

## Service Identity and Protocol

- AWS model slug: `panorama`
- AWS SDK for Rust slug: `panorama`
- Model version: `2019-07-24`
- Model file: `vendor/api-models-aws/models/panorama/service/2019-07-24/panorama-2019-07-24.json`
- SDK ID: `Panorama`
- Endpoint prefix: `-`
- ARN namespace: `panorama`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Describe` (9), `Create` (5), `Delete` (2), `Deregister` (1), `Provision` (1), `Register` (1), `Remove` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApplicationInstance`, `CreateJobForDevices`, `CreateNodeFromTemplateJob`, `CreatePackage`, `CreatePackageImportJob`, `DeleteDevice`, `DeletePackage`, `DeregisterPackageVersion`, `RegisterPackageVersion`, `RemoveApplicationInstance`, `TagResource`, `UntagResource`, `UpdateDeviceMetadata`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApplicationInstance`, `DescribeApplicationInstanceDetails`, `DescribeDevice`, `DescribeDeviceJob`, `DescribeNode`, `DescribeNodeFromTemplateJob`, `DescribePackage`, `DescribePackageImportJob`, `DescribePackageVersion`, `ListApplicationInstanceDependencies`, `ListApplicationInstanceNodeInstances`, `ListApplicationInstances`, `ListDevices`, `ListDevicesJobs`, `ListNodeFromTemplateJobs`, `ListNodes`, `ListPackageImportJobs`, `ListPackages`, `ListTagsForResource`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateJobForDevices`, `CreateNodeFromTemplateJob`, `CreatePackageImportJob`, `DescribeDeviceJob`, `DescribeNodeFromTemplateJob`, `DescribePackageImportJob`, `ListDevicesJobs`, `ListNodeFromTemplateJobs`, `ListPackageImportJobs`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 34 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListApplicationInstanceDependencies`, `ListApplicationInstanceNodeInstances`, `ListApplicationInstances`, `ListDevices`, `ListDevicesJobs`, `ListNodeFromTemplateJobs`, `ListNodes`, `ListPackageImportJobs`, `ListPackages`, `ListTagsForResource`
- Traits: `paginated` (9)
- Common required input members in this group: `ApplicationInstanceId`

### Describe

- Operations: `DescribeApplicationInstance`, `DescribeApplicationInstanceDetails`, `DescribeDevice`, `DescribeDeviceJob`, `DescribeNode`, `DescribeNodeFromTemplateJob`, `DescribePackage`, `DescribePackageImportJob`, `DescribePackageVersion`
- Common required input members in this group: `ApplicationInstanceId`, `JobId`, `PackageId`

### Create

- Operations: `CreateApplicationInstance`, `CreateJobForDevices`, `CreateNodeFromTemplateJob`, `CreatePackage`, `CreatePackageImportJob`
- Common required input members in this group: `JobType`

### Delete

- Operations: `DeleteDevice`, `DeletePackage`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterPackageVersion`
- Common required input members in this group: -

### Provision

- Operations: `ProvisionDevice`
- Common required input members in this group: -

### Register

- Operations: `RegisterPackageVersion`
- Common required input members in this group: -

### Remove

- Operations: `RemoveApplicationInstance`
- Common required input members in this group: -

### Signal

- Operations: `SignalApplicationInstanceNodeInstances`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateDeviceMetadata`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApplicationInstance` | `POST /application-instances` | - | `ManifestPayload`, `DefaultRuntimeContextDevice` | - | `CreateApplicationInstanceResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an application instance and deploys it to a device. |
| `CreateJobForDevices` | `POST /jobs` | - | `DeviceIds`, `JobType` | - | `CreateJobForDevicesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a job to run on a device. A job can update a device's software or reboot it. |
| `CreateNodeFromTemplateJob` | `POST /packages/template-job` | - | `TemplateType`, `OutputPackageName`, `OutputPackageVersion`, `NodeName`, `TemplateParameters` | - | `CreateNodeFromTemplateJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Creates a camera stream node. |
| `CreatePackage` | `POST /packages` | - | `PackageName` | - | `CreatePackageResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Creates a package and storage location in an Amazon S3 access point. |
| `CreatePackageImportJob` | `POST /packages/import-jobs` | - | `JobType`, `InputConfig`, `OutputConfig`, `ClientToken` | - | `CreatePackageImportJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Imports a node package. |
| `DeleteDevice` | `DELETE /devices/{DeviceId}` | - | `DeviceId` | - | `DeleteDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a device. |
| `DeletePackage` | `DELETE /packages/{PackageId}` | - | `PackageId` | - | `DeletePackageResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a package. To delete a package, you need permission to call s3:DeleteObject in addition to permissions for the AWS Panorama API. |
| `DeregisterPackageVersion` | `DELETE /packages/{PackageId}/versions/{PackageVersion}/patch/{PatchVersion}` | - | `PackageId`, `PackageVersion`, `PatchVersion` | - | `DeregisterPackageVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deregisters a package version. |
| `DescribeApplicationInstance` | `GET /application-instances/{ApplicationInstanceId}` | - | `ApplicationInstanceId` | - | `DescribeApplicationInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about an application instance on a device. |
| `DescribeApplicationInstanceDetails` | `GET /application-instances/{ApplicationInstanceId}/details` | - | `ApplicationInstanceId` | - | `DescribeApplicationInstanceDetailsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about an application instance's configuration manifest. |
| `DescribeDevice` | `GET /devices/{DeviceId}` | - | `DeviceId` | - | `DescribeDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a device. |
| `DescribeDeviceJob` | `GET /jobs/{JobId}` | - | `JobId` | - | `DescribeDeviceJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a device job. |
| `DescribeNode` | `GET /nodes/{NodeId}` | - | `NodeId` | - | `DescribeNodeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a node. |
| `DescribeNodeFromTemplateJob` | `GET /packages/template-job/{JobId}` | - | `JobId` | - | `DescribeNodeFromTemplateJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Returns information about a job to create a camera stream node. |
| `DescribePackage` | `GET /packages/metadata/{PackageId}` | - | `PackageId` | - | `DescribePackageResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a package. |
| `DescribePackageImportJob` | `GET /packages/import-jobs/{JobId}` | - | `JobId` | - | `DescribePackageImportJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Returns information about a package import job. |
| `DescribePackageVersion` | `GET /packages/metadata/{PackageId}/versions/{PackageVersion}` | - | `PackageId`, `PackageVersion` | - | `DescribePackageVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a package version. |
| `ListApplicationInstanceDependencies` | `GET /application-instances/{ApplicationInstanceId}/package-dependencies` | `paginated` | `ApplicationInstanceId` | - | `ListApplicationInstanceDependenciesResponse` | `AccessDeniedException`, `InternalServerException` | Returns a list of application instance dependencies. |
| `ListApplicationInstanceNodeInstances` | `GET /application-instances/{ApplicationInstanceId}/node-instances` | `paginated` | `ApplicationInstanceId` | - | `ListApplicationInstanceNodeInstancesResponse` | `AccessDeniedException`, `InternalServerException` | Returns a list of application node instances. |
| `ListApplicationInstances` | `GET /application-instances` | `paginated` | - | - | `ListApplicationInstancesResponse` | `AccessDeniedException`, `InternalServerException` | Returns a list of application instances. |
| `ListDevices` | `GET /devices` | `paginated` | - | - | `ListDevicesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Returns a list of devices. |
| `ListDevicesJobs` | `GET /jobs` | `paginated` | - | - | `ListDevicesJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of jobs. |
| `ListNodeFromTemplateJobs` | `GET /packages/template-job` | `paginated` | - | - | `ListNodeFromTemplateJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Returns a list of camera stream node jobs. |
| `ListNodes` | `GET /nodes` | `paginated` | - | - | `ListNodesResponse` | `ConflictException`, `InternalServerException`, `ValidationException` | Returns a list of nodes. |
| `ListPackageImportJobs` | `GET /packages/import-jobs` | `paginated` | - | - | `ListPackageImportJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Returns a list of package import jobs. |
| `ListPackages` | `GET /packages` | `paginated` | - | - | `ListPackagesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of packages. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of tags for a resource. |
| `ProvisionDevice` | `POST /devices` | - | `Name` | - | `ProvisionDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a device and returns a configuration archive. The configuration archive is a ZIP file that contains a provisioning certificate that is valid for 5 minutes. Name the configuration archive certificates-omni_ de ... |
| `RegisterPackageVersion` | `PUT /packages/{PackageId}/versions/{PackageVersion}/patch/{PatchVersion}` | - | `PackageId`, `PackageVersion`, `PatchVersion` | - | `RegisterPackageVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ValidationException` | Registers a package version. |
| `RemoveApplicationInstance` | `DELETE /application-instances/{ApplicationInstanceId}` | - | `ApplicationInstanceId` | - | `RemoveApplicationInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes an application instance. |
| `SignalApplicationInstanceNodeInstances` | `PUT /application-instances/{ApplicationInstanceId}/node-signals` | - | `ApplicationInstanceId`, `NodeSignals` | - | `SignalApplicationInstanceNodeInstancesResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Signal camera nodes to stop or resume. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Tags a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from a resource. |
| `UpdateDeviceMetadata` | `PUT /devices/{DeviceId}` | - | `DeviceId` | - | `UpdateDeviceMetadataResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a device's metadata. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeletePackage` | - | `ForceDelete -> ForceDelete` | - | - |
| `DeregisterPackageVersion` | - | `OwnerAccount -> OwnerAccount`, `UpdatedLatestPatchVersion -> UpdatedLatestPatchVersion` | - | - |
| `DescribeNode` | - | `OwnerAccount -> OwnerAccount` | - | - |
| `DescribePackageVersion` | - | `OwnerAccount -> OwnerAccount`, `PatchVersion -> PatchVersion` | - | - |
| `ListApplicationInstanceDependencies` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListApplicationInstanceNodeInstances` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListApplicationInstances` | - | `DeviceId -> deviceId`, `StatusFilter -> statusFilter`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListDevices` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults`, `SortBy -> SortBy`, `SortOrder -> SortOrder`, `NameFilter -> NameFilter`, `DeviceAggregatedStatusFilter -> DeviceAggregatedStatusFilter` | - | - |
| `ListDevicesJobs` | - | `DeviceId -> DeviceId`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListNodeFromTemplateJobs` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListNodes` | - | `Category -> category`, `OwnerAccount -> ownerAccount`, `PackageName -> packageName`, `PackageVersion -> packageVersion`, `PatchVersion -> patchVersion`, `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListPackageImportJobs` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListPackages` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | The requestor does not have permission to access the target action or resource. |
| `ConflictException` | `structure` | Message, ResourceId, ResourceType, ErrorId, ErrorArguments | The target resource is in use. |
| `InternalServerException` | `structure` | Message, RetryAfterSeconds | An internal error occurred. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The target resource was not found. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, QuotaCode, ServiceCode | The request would cause a limit to be exceeded. |
| `ValidationException` | `structure` | Message, Reason, ErrorId, ErrorArguments, Fields | The request contains an invalid parameter value. |
| `CreateApplicationInstanceRequest` | `structure` | Name, Description, ManifestPayload, ManifestOverridesPayload, ApplicationInstanceIdToReplace, RuntimeRoleArn, DefaultRuntimeContextDevice, Tags | - |
| `CreateApplicationInstanceResponse` | `structure` | ApplicationInstanceId | - |
| `CreateJobForDevicesRequest` | `structure` | DeviceIds, DeviceJobConfig, JobType | - |
| `CreateJobForDevicesResponse` | `structure` | Jobs | - |
| `CreateNodeFromTemplateJobRequest` | `structure` | TemplateType, OutputPackageName, OutputPackageVersion, NodeName, NodeDescription, TemplateParameters, JobTags | - |
| `CreateNodeFromTemplateJobResponse` | `structure` | JobId | - |
| `CreatePackageRequest` | `structure` | PackageName, Tags | - |
| `CreatePackageResponse` | `structure` | PackageId, Arn, StorageLocation | - |
| `CreatePackageImportJobRequest` | `structure` | JobType, InputConfig, OutputConfig, ClientToken, JobTags | - |
| `CreatePackageImportJobResponse` | `structure` | JobId | - |
| `DeleteDeviceRequest` | `structure` | DeviceId | - |
| `DeleteDeviceResponse` | `structure` | DeviceId | - |
| `DeletePackageRequest` | `structure` | PackageId, ForceDelete | - |
| `DeletePackageResponse` | `structure` | **empty (no members)** | - |
| `DeregisterPackageVersionRequest` | `structure` | OwnerAccount, PackageId, PackageVersion, PatchVersion, UpdatedLatestPatchVersion | - |
| `DeregisterPackageVersionResponse` | `structure` | **empty (no members)** | - |
| `DescribeApplicationInstanceRequest` | `structure` | ApplicationInstanceId | - |
| `DescribeApplicationInstanceResponse` | `structure` | Name, Description, DefaultRuntimeContextDevice, DefaultRuntimeContextDeviceName, ApplicationInstanceIdToReplace, RuntimeRoleArn, Status, HealthStatus, StatusDescription, CreatedTime, LastUpdatedTime, ApplicationInstanceId, ... (+3) | - |
| `DescribeApplicationInstanceDetailsRequest` | `structure` | ApplicationInstanceId | - |
| `DescribeApplicationInstanceDetailsResponse` | `structure` | Name, Description, DefaultRuntimeContextDevice, ManifestPayload, ManifestOverridesPayload, ApplicationInstanceIdToReplace, CreatedTime, ApplicationInstanceId | - |
| `DescribeDeviceRequest` | `structure` | DeviceId | - |
| `DescribeDeviceResponse` | `structure` | DeviceId, Name, Arn, Description, Type, DeviceConnectionStatus, CreatedTime, ProvisioningStatus, LatestSoftware, CurrentSoftware, SerialNumber, Tags, ... (+8) | - |
| `DescribeDeviceJobRequest` | `structure` | JobId | - |
| `DescribeDeviceJobResponse` | `structure` | JobId, DeviceId, DeviceArn, DeviceName, DeviceType, ImageVersion, Status, CreatedTime, JobType | - |
| `DescribeNodeRequest` | `structure` | NodeId, OwnerAccount | - |
| `DescribeNodeResponse` | `structure` | NodeId, Name, Category, OwnerAccount, PackageName, PackageId, PackageArn, PackageVersion, PatchVersion, NodeInterface, AssetName, Description, ... (+2) | - |
| `DescribeNodeFromTemplateJobRequest` | `structure` | JobId | - |
| `DescribeNodeFromTemplateJobResponse` | `structure` | JobId, Status, StatusMessage, CreatedTime, LastUpdatedTime, OutputPackageName, OutputPackageVersion, NodeName, NodeDescription, TemplateType, TemplateParameters, JobTags | - |
| `DescribePackageRequest` | `structure` | PackageId | - |
| `DescribePackageResponse` | `structure` | PackageId, PackageName, Arn, StorageLocation, ReadAccessPrincipalArns, WriteAccessPrincipalArns, CreatedTime, Tags | - |
| `DescribePackageImportJobRequest` | `structure` | JobId | - |
| `DescribePackageImportJobResponse` | `structure` | JobId, ClientToken, JobType, InputConfig, OutputConfig, Output, CreatedTime, LastUpdatedTime, Status, StatusMessage, JobTags | - |
| `DescribePackageVersionRequest` | `structure` | OwnerAccount, PackageId, PackageVersion, PatchVersion | - |
| `DescribePackageVersionResponse` | `structure` | OwnerAccount, PackageId, PackageArn, PackageName, PackageVersion, PatchVersion, IsLatestPatch, Status, StatusDescription, RegisteredTime | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
