# AWS S3 Control

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services S3 Control provides access to Amazon S3 control plane actions.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS S3 Control where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS S3 Control by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS S3 Control resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS S3 Control workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Delete`, `Put`, `List`, `Create` operation families, including `GetAccessGrant`, `GetAccessGrantsInstance`, `GetAccessGrantsInstanceForPrefix`, `GetAccessGrantsInstanceResourcePolicy`, `DeleteAccessGrant`, `DeleteAccessGrantsInstance`.

## Service Identity and Protocol

- AWS model slug: `s3-control`
- AWS SDK for Rust slug: `s3control`
- Model version: `2018-08-20`
- Model file: `vendor/api-models-aws/models/s3-control/service/2018-08-20/s3-control-2018-08-20.json`
- SDK ID: `S3 Control`
- Endpoint prefix: `s3-control`
- ARN namespace: `s3`
- CloudFormation name: `S3Control`
- CloudTrail event source: `s3control.amazonaws.com`
- Protocols: `restXml`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `AccessPointName`, `AccountId`, `Bucket`, `Endpoint`, `OutpostId`, `Region`, `RequiresAccountId`, `ResourceArn`, `UseArnRegion`, `UseDualStack`, `UseFIPS`, `UseS3ExpressControlEndpoint`
- Client context parameters: `UseArnRegion`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (29), `Delete` (20), `Put` (15), `List` (13), `Create` (9), `Update` (4), `Describe` (2), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAccessGrantsIdentityCenter`, `CreateAccessGrant`, `CreateAccessGrantsInstance`, `CreateAccessGrantsLocation`, `CreateAccessPoint`, `CreateAccessPointForObjectLambda`, `CreateBucket`, `CreateJob`, `CreateMultiRegionAccessPoint`, `CreateStorageLensGroup`, `DeleteAccessGrant`, `DeleteAccessGrantsInstance`, `DeleteAccessGrantsInstanceResourcePolicy`, `DeleteAccessGrantsLocation`, `DeleteAccessPoint`, `DeleteAccessPointForObjectLambda`, `DeleteAccessPointPolicy`, `DeleteAccessPointPolicyForObjectLambda`, `DeleteAccessPointScope`, `DeleteBucket`, ... (+32).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeJob`, `DescribeMultiRegionAccessPointOperation`, `GetAccessGrant`, `GetAccessGrantsInstance`, `GetAccessGrantsInstanceForPrefix`, `GetAccessGrantsInstanceResourcePolicy`, `GetAccessGrantsLocation`, `GetAccessPoint`, `GetAccessPointConfigurationForObjectLambda`, `GetAccessPointForObjectLambda`, `GetAccessPointPolicy`, `GetAccessPointPolicyForObjectLambda`, `GetAccessPointPolicyStatus`, `GetAccessPointPolicyStatusForObjectLambda`, `GetAccessPointScope`, `GetBucket`, `GetBucketLifecycleConfiguration`, `GetBucketPolicy`, `GetBucketReplication`, `GetBucketTagging`, ... (+24).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateJob`, `DeleteJobTagging`, `DescribeJob`, `GetJobTagging`, `ListJobs`, `PutJobTagging`, `UpdateJobPriority`, `UpdateJobStatus`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 10 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `Lambda`, `EC2/VPC`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Current Network Resource Stub Semantics

S3 Control currently stores access point VPC configuration on the access point record.

- `CreateAccessPoint` records `VpcConfiguration.VpcId` when present.
- Access point network origin is inferred from that stored field: VPC when a VPC ID exists, otherwise Internet.
- No EC2 VPC endpoint, subnet, or security group resource is created for the access point.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetAccessGrant`, `GetAccessGrantsInstance`, `GetAccessGrantsInstanceForPrefix`, `GetAccessGrantsInstanceResourcePolicy`, `GetAccessGrantsLocation`, `GetAccessPoint`, `GetAccessPointConfigurationForObjectLambda`, `GetAccessPointForObjectLambda`, `GetAccessPointPolicy`, `GetAccessPointPolicyForObjectLambda`, `GetAccessPointPolicyStatus`, `GetAccessPointPolicyStatusForObjectLambda`, `GetAccessPointScope`, `GetBucket`, `GetBucketLifecycleConfiguration`, `GetBucketPolicy`, `GetBucketReplication`, `GetBucketTagging`, `GetBucketVersioning`, `GetDataAccess`, `GetJobTagging`, `GetMultiRegionAccessPoint`, `GetMultiRegionAccessPointPolicy`, `GetMultiRegionAccessPointPolicyStatus`, `GetMultiRegionAccessPointRoutes`, `GetPublicAccessBlock`, `GetStorageLensConfiguration`, `GetStorageLensConfigurationTagging`, `GetStorageLensGroup`
- Traits: `endpoint-bound` (28)
- Common required input members in this group: `AccessGrantId`, `AccessGrantsLocationId`, `AccountId`, `Bucket`, `ConfigId`, `JobId`, `Mrap`, `Name`, `Permission`, `S3Prefix`, `Target`

### Delete

- Operations: `DeleteAccessGrant`, `DeleteAccessGrantsInstance`, `DeleteAccessGrantsInstanceResourcePolicy`, `DeleteAccessGrantsLocation`, `DeleteAccessPoint`, `DeleteAccessPointForObjectLambda`, `DeleteAccessPointPolicy`, `DeleteAccessPointPolicyForObjectLambda`, `DeleteAccessPointScope`, `DeleteBucket`, `DeleteBucketLifecycleConfiguration`, `DeleteBucketPolicy`, `DeleteBucketReplication`, `DeleteBucketTagging`, `DeleteJobTagging`, `DeleteMultiRegionAccessPoint`, `DeletePublicAccessBlock`, `DeleteStorageLensConfiguration`, `DeleteStorageLensConfigurationTagging`, `DeleteStorageLensGroup`
- Traits: `endpoint-bound` (19), `idempotency-token` (1)
- Common required input members in this group: `AccessGrantId`, `AccessGrantsLocationId`, `AccountId`, `Bucket`, `ClientToken`, `ConfigId`, `Details`, `JobId`, `Name`

### Put

- Operations: `PutAccessGrantsInstanceResourcePolicy`, `PutAccessPointConfigurationForObjectLambda`, `PutAccessPointPolicy`, `PutAccessPointPolicyForObjectLambda`, `PutAccessPointScope`, `PutBucketLifecycleConfiguration`, `PutBucketPolicy`, `PutBucketReplication`, `PutBucketTagging`, `PutBucketVersioning`, `PutJobTagging`, `PutMultiRegionAccessPointPolicy`, `PutPublicAccessBlock`, `PutStorageLensConfiguration`, `PutStorageLensConfigurationTagging`
- Traits: `endpoint-bound` (14), `idempotency-token` (1)
- Common required input members in this group: `AccountId`, `Bucket`, `ClientToken`, `ConfigId`, `Configuration`, `Details`, `JobId`, `Name`, `Policy`, `PublicAccessBlockConfiguration`, `ReplicationConfiguration`, `Scope`, `StorageLensConfiguration`, `Tagging`, `Tags`, `VersioningConfiguration`

### List

- Operations: `ListAccessGrants`, `ListAccessGrantsInstances`, `ListAccessGrantsLocations`, `ListAccessPoints`, `ListAccessPointsForDirectoryBuckets`, `ListAccessPointsForObjectLambda`, `ListCallerAccessGrants`, `ListJobs`, `ListMultiRegionAccessPoints`, `ListRegionalBuckets`, `ListStorageLensConfigurations`, `ListStorageLensGroups`, `ListTagsForResource`
- Traits: `endpoint-bound` (12), `paginated` (12)
- Common required input members in this group: `AccountId`, `ResourceArn`

### Create

- Operations: `CreateAccessGrant`, `CreateAccessGrantsInstance`, `CreateAccessGrantsLocation`, `CreateAccessPoint`, `CreateAccessPointForObjectLambda`, `CreateBucket`, `CreateJob`, `CreateMultiRegionAccessPoint`, `CreateStorageLensGroup`
- Traits: `endpoint-bound` (8), `idempotency-token` (2)
- Common required input members in this group: `AccessGrantsLocationId`, `AccountId`, `Bucket`, `ClientRequestToken`, `ClientToken`, `Configuration`, `Details`, `Grantee`, `IAMRoleArn`, `LocationScope`, `Name`, `Operation`, `Permission`, `Priority`, `Report`, `RoleArn`, `StorageLensGroup`

### Update

- Operations: `UpdateAccessGrantsLocation`, `UpdateJobPriority`, `UpdateJobStatus`, `UpdateStorageLensGroup`
- Traits: `endpoint-bound` (4)
- Common required input members in this group: `AccessGrantsLocationId`, `AccountId`, `IAMRoleArn`, `JobId`, `Name`, `Priority`, `RequestedJobStatus`, `StorageLensGroup`

### Describe

- Operations: `DescribeJob`, `DescribeMultiRegionAccessPointOperation`
- Traits: `endpoint-bound` (2)
- Common required input members in this group: `AccountId`, `JobId`, `RequestTokenARN`

### Associate

- Operations: `AssociateAccessGrantsIdentityCenter`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `AccountId`, `IdentityCenterArn`

### Dissociate

- Operations: `DissociateAccessGrantsIdentityCenter`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `AccountId`

### Submit

- Operations: `SubmitMultiRegionAccessPointRoutes`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `AccountId`, `Mrap`, `RouteUpdates`

### Tag

- Operations: `TagResource`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `AccountId`, `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `AccountId`, `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAccessGrantsIdentityCenter` | `POST /v20180820/accessgrantsinstance/identitycenter` | `endpoint-bound` | `AccountId`, `IdentityCenterArn` | - | `Unit` | - | Associate your S3 Access Grants instance with an Amazon Web Services IAM Identity Center instance. Use this action if you want to create access grants for users or groups from your corporate identity directory. |
| `CreateAccessGrant` | `POST /v20180820/accessgrantsinstance/grant` | `endpoint-bound` | `AccessGrantsLocationId`, `AccountId`, `Grantee`, `Permission` | - | `CreateAccessGrantResult` | - | Creates an access grant that gives a grantee access to your S3 data. The grantee can be an IAM user or role or a directory user, or group. |
| `CreateAccessGrantsInstance` | `POST /v20180820/accessgrantsinstance` | `endpoint-bound` | `AccountId` | - | `CreateAccessGrantsInstanceResult` | - | Creates an S3 Access Grants instance, which serves as a logical grouping for access grants. You can create one S3 Access Grants instance per Region per account. |
| `CreateAccessGrantsLocation` | `POST /v20180820/accessgrantsinstance/location` | `endpoint-bound` | `AccountId`, `IAMRoleArn`, `LocationScope` | - | `CreateAccessGrantsLocationResult` | - | The S3 data location that you would like to register in your S3 Access Grants instance. Your S3 data must be in the same Region as your S3 Access Grants instance. |
| `CreateAccessPoint` | `PUT /v20180820/accesspoint/{Name}` | `endpoint-bound` | `AccountId`, `Bucket`, `Name` | - | `CreateAccessPointResult` | - | Creates an access point and associates it to a specified bucket. For more information, see Managing access to shared datasets with access points or Managing access to shared datasets in directory buckets with access points in the Amazon S3 User Guide . |
| `CreateAccessPointForObjectLambda` | `PUT /v20180820/accesspointforobjectlambda/{Name}` | `endpoint-bound` | `AccountId`, `Configuration`, `Name` | - | `CreateAccessPointForObjectLambdaResult` | - | This operation is not supported by directory buckets. Creates an Object Lambda Access Point. |
| `CreateBucket` | `PUT /v20180820/bucket/{Bucket}` | - | `Bucket` | - | `CreateBucketResult` | `BucketAlreadyExists`, `BucketAlreadyOwnedByYou` | This action creates an Amazon S3 on Outposts bucket. To create an S3 bucket, see Create Bucket in the Amazon S3 API Reference . |
| `CreateJob` | `POST /v20180820/jobs` | `endpoint-bound`, `idempotency-token` | `AccountId`, `ClientRequestToken`, `Operation`, `Priority`, `Report`, `RoleArn` | `ClientRequestToken` | `CreateJobResult` | `BadRequestException`, `IdempotencyException`, `InternalServiceException`, `TooManyRequestsException` | This operation creates an S3 Batch Operations job. You can use S3 Batch Operations to perform large-scale batch actions on Amazon S3 objects. |
| `CreateMultiRegionAccessPoint` | `POST /v20180820/async-requests/mrap/create` | `endpoint-bound`, `idempotency-token` | `AccountId`, `ClientToken`, `Details` | `ClientToken` | `CreateMultiRegionAccessPointResult` | - | This operation is not supported by directory buckets. Creates a Multi-Region Access Point and associates it with the specified buckets. |
| `CreateStorageLensGroup` | `POST /v20180820/storagelensgroup` | `endpoint-bound` | `AccountId`, `StorageLensGroup` | - | `Unit` | - | Creates a new S3 Storage Lens group and associates it with the specified Amazon Web Services account ID. An S3 Storage Lens group is a custom grouping of objects based on prefix, suffix, object tags, object size, object age, or a combination of these filters. |
| `DeleteAccessGrant` | `DELETE /v20180820/accessgrantsinstance/grant/{AccessGrantId}` | `endpoint-bound` | `AccessGrantId`, `AccountId` | - | `Unit` | - | Deletes the access grant from the S3 Access Grants instance. You cannot undo an access grant deletion and the grantee will no longer have access to the S3 data. |
| `DeleteAccessGrantsInstance` | `DELETE /v20180820/accessgrantsinstance` | `endpoint-bound` | `AccountId` | - | `Unit` | - | Deletes your S3 Access Grants instance. You must first delete the access grants and locations before S3 Access Grants can delete the instance. |
| `DeleteAccessGrantsInstanceResourcePolicy` | `DELETE /v20180820/accessgrantsinstance/resourcepolicy` | `endpoint-bound` | `AccountId` | - | `Unit` | - | Deletes the resource policy of the S3 Access Grants instance. The resource policy is used to manage cross-account access to your S3 Access Grants instance. |
| `DeleteAccessGrantsLocation` | `DELETE /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}` | `endpoint-bound` | `AccessGrantsLocationId`, `AccountId` | - | `Unit` | - | Deregisters a location from your S3 Access Grants instance. You can only delete a location registration from an S3 Access Grants instance if there are no grants associated with this location. |
| `DeleteAccessPoint` | `DELETE /v20180820/accesspoint/{Name}` | `endpoint-bound` | `AccountId`, `Name` | - | `Unit` | - | Deletes the specified access point. All Amazon S3 on Outposts REST API requests for this action require an additional parameter of `x-amz-outpost-id` to be passed with the request. |
| `DeleteAccessPointForObjectLambda` | `DELETE /v20180820/accesspointforobjectlambda/{Name}` | `endpoint-bound` | `AccountId`, `Name` | - | `Unit` | - | This operation is not supported by directory buckets. Deletes the specified Object Lambda Access Point. |
| `DeleteAccessPointPolicy` | `DELETE /v20180820/accesspoint/{Name}/policy` | `endpoint-bound` | `AccountId`, `Name` | - | `Unit` | - | Deletes the access point policy for the specified access point. All Amazon S3 on Outposts REST API requests for this action require an additional parameter of `x-amz-outpost-id` to be passed with the request. |
| `DeleteAccessPointPolicyForObjectLambda` | `DELETE /v20180820/accesspointforobjectlambda/{Name}/policy` | `endpoint-bound` | `AccountId`, `Name` | - | `Unit` | - | This operation is not supported by directory buckets. Removes the resource policy for an Object Lambda Access Point. |
| `DeleteAccessPointScope` | `DELETE /v20180820/accesspoint/{Name}/scope` | - | `AccountId`, `Name` | - | `Unit` | - | Deletes an existing access point scope for a directory bucket. When you delete the scope of an access point, all prefixes and permissions are deleted. |
| `DeleteBucket` | `DELETE /v20180820/bucket/{Bucket}` | `endpoint-bound` | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket. To delete an S3 bucket, see DeleteBucket in the Amazon S3 API Reference . |
| `DeleteBucketLifecycleConfiguration` | `DELETE /v20180820/bucket/{Bucket}/lifecycleconfiguration` | `endpoint-bound` | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket's lifecycle configuration. To delete an S3 bucket's lifecycle configuration, see DeleteBucketLifecycle in the Amazon S3 API Reference . |
| `DeleteBucketPolicy` | `DELETE /v20180820/bucket/{Bucket}/policy` | `endpoint-bound` | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket policy. To delete an S3 bucket policy, see DeleteBucketPolicy in the Amazon S3 API Reference . |
| `DeleteBucketReplication` | `DELETE /v20180820/bucket/{Bucket}/replication` | `endpoint-bound` | `AccountId`, `Bucket` | - | `Unit` | - | This operation deletes an Amazon S3 on Outposts bucket's replication configuration. To delete an S3 bucket's replication configuration, see DeleteBucketReplication in the Amazon S3 API Reference . |
| `DeleteBucketTagging` | `DELETE /v20180820/bucket/{Bucket}/tagging` | `endpoint-bound` | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket's tags. To delete an S3 bucket tags, see DeleteBucketTagging in the Amazon S3 API Reference . |
| `DeleteJobTagging` | `DELETE /v20180820/jobs/{JobId}/tagging` | `endpoint-bound` | `AccountId`, `JobId` | - | `DeleteJobTaggingResult` | `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Removes the entire tag set from the specified S3 Batch Operations job. Permissions To use the `DeleteJobTagging` operation, you must have permission to perform the `s3:DeleteJobTagging` action. |
| `DeleteMultiRegionAccessPoint` | `POST /v20180820/async-requests/mrap/delete` | `endpoint-bound`, `idempotency-token` | `AccountId`, `ClientToken`, `Details` | `ClientToken` | `DeleteMultiRegionAccessPointResult` | - | This operation is not supported by directory buckets. Deletes a Multi-Region Access Point. |
| `DeletePublicAccessBlock` | `DELETE /v20180820/configuration/publicAccessBlock` | `endpoint-bound` | `AccountId` | - | `Unit` | - | This operation is not supported by directory buckets. Removes the `PublicAccessBlock` configuration for an Amazon Web Services account. |
| `DeleteStorageLensConfiguration` | `DELETE /v20180820/storagelens/{ConfigId}` | `endpoint-bound` | `AccountId`, `ConfigId` | - | `Unit` | - | This operation is not supported by directory buckets. Deletes the Amazon S3 Storage Lens configuration. |
| `DeleteStorageLensConfigurationTagging` | `DELETE /v20180820/storagelens/{ConfigId}/tagging` | `endpoint-bound` | `AccountId`, `ConfigId` | - | `DeleteStorageLensConfigurationTaggingResult` | - | This operation is not supported by directory buckets. Deletes the Amazon S3 Storage Lens configuration tags. |
| `DeleteStorageLensGroup` | `DELETE /v20180820/storagelensgroup/{Name}` | `endpoint-bound` | `AccountId`, `Name` | - | `Unit` | - | Deletes an existing S3 Storage Lens group. To use this operation, you must have the permission to perform the `s3:DeleteStorageLensGroup` action. |
| `DescribeJob` | `GET /v20180820/jobs/{JobId}` | `endpoint-bound` | `AccountId`, `JobId` | - | `DescribeJobResult` | `BadRequestException`, `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the configuration parameters and status for a Batch Operations job. For more information, see S3 Batch Operations in the Amazon S3 User Guide . |
| `DescribeMultiRegionAccessPointOperation` | `GET /v20180820/async-requests/mrap/{RequestTokenARN+}` | `endpoint-bound` | `AccountId`, `RequestTokenARN` | - | `DescribeMultiRegionAccessPointOperationResult` | - | This operation is not supported by directory buckets. Retrieves the status of an asynchronous request to manage a Multi-Region Access Point. |
| `DissociateAccessGrantsIdentityCenter` | `DELETE /v20180820/accessgrantsinstance/identitycenter` | `endpoint-bound` | `AccountId` | - | `Unit` | - | Dissociates the Amazon Web Services IAM Identity Center instance from the S3 Access Grants instance. Permissions You must have the `s3:DissociateAccessGrantsIdentityCenter` permission to use this operation. |
| `GetAccessGrant` | `GET /v20180820/accessgrantsinstance/grant/{AccessGrantId}` | `endpoint-bound` | `AccessGrantId`, `AccountId` | - | `GetAccessGrantResult` | - | Get the details of an access grant from your S3 Access Grants instance. Permissions You must have the `s3:GetAccessGrant` permission to use this operation. |
| `GetAccessGrantsInstance` | `GET /v20180820/accessgrantsinstance` | `endpoint-bound` | `AccountId` | - | `GetAccessGrantsInstanceResult` | - | Retrieves the S3 Access Grants instance for a Region in your account. Permissions You must have the `s3:GetAccessGrantsInstance` permission to use this operation. |
| `GetAccessGrantsInstanceForPrefix` | `GET /v20180820/accessgrantsinstance/prefix` | `endpoint-bound` | `AccountId`, `S3Prefix` | - | `GetAccessGrantsInstanceForPrefixResult` | - | Retrieve the S3 Access Grants instance that contains a particular prefix. Permissions You must have the `s3:GetAccessGrantsInstanceForPrefix` permission for the caller account to use this operation. |
| `GetAccessGrantsInstanceResourcePolicy` | `GET /v20180820/accessgrantsinstance/resourcepolicy` | `endpoint-bound` | `AccountId` | - | `GetAccessGrantsInstanceResourcePolicyResult` | - | Returns the resource policy of the S3 Access Grants instance. Permissions You must have the `s3:GetAccessGrantsInstanceResourcePolicy` permission to use this operation. |
| `GetAccessGrantsLocation` | `GET /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}` | `endpoint-bound` | `AccessGrantsLocationId`, `AccountId` | - | `GetAccessGrantsLocationResult` | - | Retrieves the details of a particular location registered in your S3 Access Grants instance. Permissions You must have the `s3:GetAccessGrantsLocation` permission to use this operation. |
| `GetAccessPoint` | `GET /v20180820/accesspoint/{Name}` | `endpoint-bound` | `AccountId`, `Name` | - | `GetAccessPointResult` | - | Returns configuration information about the specified access point. All Amazon S3 on Outposts REST API requests for this action require an additional parameter of `x-amz-outpost-id` to be passed with the request. |
| `GetAccessPointConfigurationForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}/configuration` | `endpoint-bound` | `AccountId`, `Name` | - | `GetAccessPointConfigurationForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns configuration for an Object Lambda Access Point. |
| `GetAccessPointForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}` | `endpoint-bound` | `AccountId`, `Name` | - | `GetAccessPointForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns configuration information about the specified Object Lambda Access Point The following actions are related to `GetAccessPointForObjectLambda`: CreateAccessPointForObjectLambda... |
| `GetAccessPointPolicy` | `GET /v20180820/accesspoint/{Name}/policy` | `endpoint-bound` | `AccountId`, `Name` | - | `GetAccessPointPolicyResult` | - | Returns the access point policy associated with the specified access point. The following actions are related to `GetAccessPointPolicy`: PutAccessPointPolicy DeleteAccessPointPolicy |
| `GetAccessPointPolicyForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}/policy` | `endpoint-bound` | `AccountId`, `Name` | - | `GetAccessPointPolicyForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns the resource policy for an Object Lambda Access Point. |
| `GetAccessPointPolicyStatus` | `GET /v20180820/accesspoint/{Name}/policyStatus` | `endpoint-bound` | `AccountId`, `Name` | - | `GetAccessPointPolicyStatusResult` | - | This operation is not supported by directory buckets. Indicates whether the specified access point currently has a policy that allows public access. |
| `GetAccessPointPolicyStatusForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}/policyStatus` | `endpoint-bound` | `AccountId`, `Name` | - | `GetAccessPointPolicyStatusForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns the status of the resource policy associated with an Object Lambda Access Point. |
| `GetAccessPointScope` | `GET /v20180820/accesspoint/{Name}/scope` | - | `AccountId`, `Name` | - | `GetAccessPointScopeResult` | - | Returns the access point scope for a directory bucket. To use this operation, you must have the permission to perform the `s3express:GetAccessPointScope` action. |
| `GetBucket` | `GET /v20180820/bucket/{Bucket}` | `endpoint-bound` | `AccountId`, `Bucket` | - | `GetBucketResult` | - | Gets an Amazon S3 on Outposts bucket. For more information, see Using Amazon S3 on Outposts in the Amazon S3 User Guide . |
| `GetBucketLifecycleConfiguration` | `GET /v20180820/bucket/{Bucket}/lifecycleconfiguration` | `endpoint-bound` | `AccountId`, `Bucket` | - | `GetBucketLifecycleConfigurationResult` | - | This action gets an Amazon S3 on Outposts bucket's lifecycle configuration. To get an S3 bucket's lifecycle configuration, see GetBucketLifecycleConfiguration in the Amazon S3 API Reference . |
| `GetBucketPolicy` | `GET /v20180820/bucket/{Bucket}/policy` | `endpoint-bound` | `AccountId`, `Bucket` | - | `GetBucketPolicyResult` | - | This action gets a bucket policy for an Amazon S3 on Outposts bucket. To get a policy for an S3 bucket, see GetBucketPolicy in the Amazon S3 API Reference . |
| `GetBucketReplication` | `GET /v20180820/bucket/{Bucket}/replication` | `endpoint-bound` | `AccountId`, `Bucket` | - | `GetBucketReplicationResult` | - | This operation gets an Amazon S3 on Outposts bucket's replication configuration. To get an S3 bucket's replication configuration, see GetBucketReplication in the Amazon S3 API Reference . |
| `GetBucketTagging` | `GET /v20180820/bucket/{Bucket}/tagging` | `endpoint-bound` | `AccountId`, `Bucket` | - | `GetBucketTaggingResult` | - | This action gets an Amazon S3 on Outposts bucket's tags. To get an S3 bucket tags, see GetBucketTagging in the Amazon S3 API Reference . |
| `GetBucketVersioning` | `GET /v20180820/bucket/{Bucket}/versioning` | `endpoint-bound` | `AccountId`, `Bucket` | - | `GetBucketVersioningResult` | - | This operation returns the versioning state for S3 on Outposts buckets only. To return the versioning state for an S3 bucket, see GetBucketVersioning in the Amazon S3 API Reference . |
| `GetDataAccess` | `GET /v20180820/accessgrantsinstance/dataaccess` | `endpoint-bound` | `AccountId`, `Permission`, `Target` | - | `GetDataAccessResult` | - | Returns a temporary access credential from S3 Access Grants to the grantee or client application. The temporary credential is an Amazon Web Services STS token that grants them access to the S3 data. |
| `GetJobTagging` | `GET /v20180820/jobs/{JobId}/tagging` | `endpoint-bound` | `AccountId`, `JobId` | - | `GetJobTaggingResult` | `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Returns the tags on an S3 Batch Operations job. Permissions To use the `GetJobTagging` operation, you must have permission to perform the `s3:GetJobTagging` action. |
| `GetMultiRegionAccessPoint` | `GET /v20180820/mrap/instances/{Name+}` | `endpoint-bound` | `AccountId`, `Name` | - | `GetMultiRegionAccessPointResult` | - | This operation is not supported by directory buckets. Returns configuration information about the specified Multi-Region Access Point. |
| `GetMultiRegionAccessPointPolicy` | `GET /v20180820/mrap/instances/{Name+}/policy` | `endpoint-bound` | `AccountId`, `Name` | - | `GetMultiRegionAccessPointPolicyResult` | - | This operation is not supported by directory buckets. Returns the access control policy of the specified Multi-Region Access Point. |
| `GetMultiRegionAccessPointPolicyStatus` | `GET /v20180820/mrap/instances/{Name+}/policystatus` | `endpoint-bound` | `AccountId`, `Name` | - | `GetMultiRegionAccessPointPolicyStatusResult` | - | This operation is not supported by directory buckets. Indicates whether the specified Multi-Region Access Point has an access control policy that allows public access. |
| `GetMultiRegionAccessPointRoutes` | `GET /v20180820/mrap/instances/{Mrap+}/routes` | `endpoint-bound` | `AccountId`, `Mrap` | - | `GetMultiRegionAccessPointRoutesResult` | - | This operation is not supported by directory buckets. Returns the routing configuration for a Multi-Region Access Point, indicating which Regions are active or passive. |
| `GetPublicAccessBlock` | `GET /v20180820/configuration/publicAccessBlock` | `endpoint-bound` | `AccountId` | - | `GetPublicAccessBlockOutput` | `NoSuchPublicAccessBlockConfiguration` | This operation is not supported by directory buckets. Retrieves the `PublicAccessBlock` configuration for an Amazon Web Services account. |
| `GetStorageLensConfiguration` | `GET /v20180820/storagelens/{ConfigId}` | `endpoint-bound` | `AccountId`, `ConfigId` | - | `GetStorageLensConfigurationResult` | - | This operation is not supported by directory buckets. Gets the Amazon S3 Storage Lens configuration. |
| `GetStorageLensConfigurationTagging` | `GET /v20180820/storagelens/{ConfigId}/tagging` | `endpoint-bound` | `AccountId`, `ConfigId` | - | `GetStorageLensConfigurationTaggingResult` | - | This operation is not supported by directory buckets. Gets the tags of Amazon S3 Storage Lens configuration. |
| `GetStorageLensGroup` | `GET /v20180820/storagelensgroup/{Name}` | `endpoint-bound` | `AccountId`, `Name` | - | `GetStorageLensGroupResult` | - | Retrieves the Storage Lens group configuration details. To use this operation, you must have the permission to perform the `s3:GetStorageLensGroup` action. |
| `ListAccessGrants` | `GET /v20180820/accessgrantsinstance/grants` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListAccessGrantsResult` | - | Returns the list of access grants in your S3 Access Grants instance. Permissions You must have the `s3:ListAccessGrants` permission to use this operation. |
| `ListAccessGrantsInstances` | `GET /v20180820/accessgrantsinstances` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListAccessGrantsInstancesResult` | - | Returns a list of S3 Access Grants instances. An S3 Access Grants instance serves as a logical grouping for your individual access grants. |
| `ListAccessGrantsLocations` | `GET /v20180820/accessgrantsinstance/locations` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListAccessGrantsLocationsResult` | - | Returns a list of the locations registered in your S3 Access Grants instance. Permissions You must have the `s3:ListAccessGrantsLocations` permission to use this operation. |
| `ListAccessPoints` | `GET /v20180820/accesspoint` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListAccessPointsResult` | - | This operation is not supported by directory buckets. Returns a list of the access points. |
| `ListAccessPointsForDirectoryBuckets` | `GET /v20180820/accesspointfordirectory` | `paginated` | `AccountId` | - | `ListAccessPointsForDirectoryBucketsResult` | - | Returns a list of the access points that are owned by the Amazon Web Services account and that are associated with the specified directory bucket. To list access points for general purpose buckets, see ListAccesspoints. |
| `ListAccessPointsForObjectLambda` | `GET /v20180820/accesspointforobjectlambda` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListAccessPointsForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns some or all (up to 1,000) access points associated with the Object Lambda Access Point per call. |
| `ListCallerAccessGrants` | `GET /v20180820/accessgrantsinstance/caller/grants` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListCallerAccessGrantsResult` | - | Use this API to list the access grants that grant the caller access to Amazon S3 data through S3 Access Grants. The caller (grantee) can be an Identity and Access Management (IAM) identity or Amazon Web Services Identity Center corporate directory identity. |
| `ListJobs` | `GET /v20180820/jobs` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListJobsResult` | `InternalServiceException`, `InvalidNextTokenException`, `InvalidRequestException` | Lists current S3 Batch Operations jobs as well as the jobs that have ended within the last 90 days for the Amazon Web Services account making the request. For more information, see S3 Batch Operations in the Amazon S3 User Guide . |
| `ListMultiRegionAccessPoints` | `GET /v20180820/mrap/instances` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListMultiRegionAccessPointsResult` | - | This operation is not supported by directory buckets. Returns a list of the Multi-Region Access Points currently associated with the specified Amazon Web Services account. |
| `ListRegionalBuckets` | `GET /v20180820/bucket` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListRegionalBucketsResult` | - | This operation is not supported by directory buckets. Returns a list of all Outposts buckets in an Outpost that are owned by the authenticated sender of the request. |
| `ListStorageLensConfigurations` | `GET /v20180820/storagelens` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListStorageLensConfigurationsResult` | - | This operation is not supported by directory buckets. Gets a list of Amazon S3 Storage Lens configurations. |
| `ListStorageLensGroups` | `GET /v20180820/storagelensgroup` | `paginated`, `endpoint-bound` | `AccountId` | - | `ListStorageLensGroupsResult` | - | Lists all the Storage Lens groups in the specified home Region. To use this operation, you must have the permission to perform the `s3:ListStorageLensGroups` action. |
| `ListTagsForResource` | `GET /v20180820/tags/{ResourceArn+}` | `endpoint-bound` | `AccountId`, `ResourceArn` | - | `ListTagsForResourceResult` | - | This operation allows you to list all of the tags for a specified resource. Each tag is a label consisting of a key and value. |
| `PutAccessGrantsInstanceResourcePolicy` | `PUT /v20180820/accessgrantsinstance/resourcepolicy` | `endpoint-bound` | `AccountId`, `Policy` | - | `PutAccessGrantsInstanceResourcePolicyResult` | - | Updates the resource policy of the S3 Access Grants instance. Permissions You must have the `s3:PutAccessGrantsInstanceResourcePolicy` permission to use this operation. |
| `PutAccessPointConfigurationForObjectLambda` | `PUT /v20180820/accesspointforobjectlambda/{Name}/configuration` | `endpoint-bound` | `AccountId`, `Configuration`, `Name` | - | `Unit` | - | This operation is not supported by directory buckets. Replaces configuration for an Object Lambda Access Point. |
| `PutAccessPointPolicy` | `PUT /v20180820/accesspoint/{Name}/policy` | `endpoint-bound` | `AccountId`, `Name`, `Policy` | - | `Unit` | - | Associates an access policy with the specified access point. Each access point can have only one policy, so a request made to this API replaces any existing policy associated with the specified access point. |
| `PutAccessPointPolicyForObjectLambda` | `PUT /v20180820/accesspointforobjectlambda/{Name}/policy` | `endpoint-bound` | `AccountId`, `Name`, `Policy` | - | `Unit` | - | This operation is not supported by directory buckets. Creates or replaces resource policy for an Object Lambda Access Point. |
| `PutAccessPointScope` | `PUT /v20180820/accesspoint/{Name}/scope` | - | `AccountId`, `Name`, `Scope` | - | `Unit` | - | Creates or replaces the access point scope for a directory bucket. You can use the access point scope to restrict access to specific prefixes, API operations, or a combination of both. |
| `PutBucketLifecycleConfiguration` | `PUT /v20180820/bucket/{Bucket}/lifecycleconfiguration` | `endpoint-bound` | `AccountId`, `Bucket` | - | `Unit` | - | This action puts a lifecycle configuration to an Amazon S3 on Outposts bucket. To put a lifecycle configuration to an S3 bucket, see PutBucketLifecycleConfiguration in the Amazon S3 API Reference . |
| `PutBucketPolicy` | `PUT /v20180820/bucket/{Bucket}/policy` | `endpoint-bound` | `AccountId`, `Bucket`, `Policy` | - | `Unit` | - | This action puts a bucket policy to an Amazon S3 on Outposts bucket. To put a policy on an S3 bucket, see PutBucketPolicy in the Amazon S3 API Reference . |
| `PutBucketReplication` | `PUT /v20180820/bucket/{Bucket}/replication` | `endpoint-bound` | `AccountId`, `Bucket`, `ReplicationConfiguration` | - | `Unit` | - | This action creates an Amazon S3 on Outposts bucket's replication configuration. To create an S3 bucket's replication configuration, see PutBucketReplication in the Amazon S3 API Reference . |
| `PutBucketTagging` | `PUT /v20180820/bucket/{Bucket}/tagging` | `endpoint-bound` | `AccountId`, `Bucket`, `Tagging` | - | `Unit` | - | This action puts tags on an Amazon S3 on Outposts bucket. To put tags on an S3 bucket, see PutBucketTagging in the Amazon S3 API Reference . |
| `PutBucketVersioning` | `PUT /v20180820/bucket/{Bucket}/versioning` | `endpoint-bound` | `AccountId`, `Bucket`, `VersioningConfiguration` | - | `Unit` | - | This operation sets the versioning state for S3 on Outposts buckets only. To set the versioning state for an S3 bucket, see PutBucketVersioning in the Amazon S3 API Reference . |
| `PutJobTagging` | `PUT /v20180820/jobs/{JobId}/tagging` | `endpoint-bound` | `AccountId`, `JobId`, `Tags` | - | `PutJobTaggingResult` | `InternalServiceException`, `NotFoundException`, `TooManyRequestsException`, `TooManyTagsException` | Sets the supplied tag-set on an S3 Batch Operations job. A tag is a key-value pair. |
| `PutMultiRegionAccessPointPolicy` | `POST /v20180820/async-requests/mrap/put-policy` | `endpoint-bound`, `idempotency-token` | `AccountId`, `ClientToken`, `Details` | `ClientToken` | `PutMultiRegionAccessPointPolicyResult` | - | This operation is not supported by directory buckets. Associates an access control policy with the specified Multi-Region Access Point. |
| `PutPublicAccessBlock` | `PUT /v20180820/configuration/publicAccessBlock` | `endpoint-bound` | `AccountId`, `PublicAccessBlockConfiguration` | - | `Unit` | - | This operation is not supported by directory buckets. Creates or modifies the `PublicAccessBlock` configuration for an Amazon Web Services account. |
| `PutStorageLensConfiguration` | `PUT /v20180820/storagelens/{ConfigId}` | `endpoint-bound` | `AccountId`, `ConfigId`, `StorageLensConfiguration` | - | `Unit` | - | This operation is not supported by directory buckets. Puts an Amazon S3 Storage Lens configuration. |
| `PutStorageLensConfigurationTagging` | `PUT /v20180820/storagelens/{ConfigId}/tagging` | `endpoint-bound` | `AccountId`, `ConfigId`, `Tags` | - | `PutStorageLensConfigurationTaggingResult` | - | This operation is not supported by directory buckets. Put or replace tags on an existing Amazon S3 Storage Lens configuration. |
| `SubmitMultiRegionAccessPointRoutes` | `PATCH /v20180820/mrap/instances/{Mrap+}/routes` | `endpoint-bound` | `AccountId`, `Mrap`, `RouteUpdates` | - | `SubmitMultiRegionAccessPointRoutesResult` | - | This operation is not supported by directory buckets. Submits an updated route configuration for a Multi-Region Access Point. |
| `TagResource` | `POST /v20180820/tags/{ResourceArn+}` | `endpoint-bound` | `AccountId`, `ResourceArn`, `Tags` | - | `TagResourceResult` | - | Creates a new user-defined tag or updates an existing tag. Each tag is a label consisting of a key and value that is applied to your resource. |
| `UntagResource` | `DELETE /v20180820/tags/{ResourceArn+}` | `endpoint-bound` | `AccountId`, `ResourceArn`, `TagKeys` | - | `UntagResourceResult` | - | This operation removes the specified user-defined tags from an S3 resource. You can pass one or more tag keys. |
| `UpdateAccessGrantsLocation` | `PUT /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}` | `endpoint-bound` | `AccessGrantsLocationId`, `AccountId`, `IAMRoleArn` | - | `UpdateAccessGrantsLocationResult` | - | Updates the IAM role of a registered location in your S3 Access Grants instance. Permissions You must have the `s3:UpdateAccessGrantsLocation` permission to use this operation. |
| `UpdateJobPriority` | `POST /v20180820/jobs/{JobId}/priority` | `endpoint-bound` | `AccountId`, `JobId`, `Priority` | - | `UpdateJobPriorityResult` | `BadRequestException`, `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Updates an existing S3 Batch Operations job's priority. For more information, see S3 Batch Operations in the Amazon S3 User Guide . |
| `UpdateJobStatus` | `POST /v20180820/jobs/{JobId}/status` | `endpoint-bound` | `AccountId`, `JobId`, `RequestedJobStatus` | - | `UpdateJobStatusResult` | `BadRequestException`, `InternalServiceException`, `JobStatusException`, `NotFoundException`, `TooManyRequestsException` | Updates the status for the specified job. Use this operation to confirm that you want to run a job or to cancel an existing job. |
| `UpdateStorageLensGroup` | `PUT /v20180820/storagelensgroup/{Name}` | `endpoint-bound` | `AccountId`, `Name`, `StorageLensGroup` | - | `Unit` | - | Updates the existing Storage Lens group. To use this operation, you must have the permission to perform the `s3:UpdateStorageLensGroup` action. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AssociateAccessGrantsIdentityCenter` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateAccessGrant` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateAccessGrantsInstance` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateAccessGrantsLocation` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateAccessPoint` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateAccessPointForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateBucket` | `ACL -> x-amz-acl`, `GrantFullControl -> x-amz-grant-full-control`, `GrantRead -> x-amz-grant-read`, `GrantReadACP -> x-amz-grant-read-acp`, `GrantWrite -> x-amz-grant-write`, `GrantWriteACP -> x-amz-grant-write-acp`, `ObjectLockEnabledForBucket -> x-amz-bucket-object-lock-enabled`, `OutpostId -> x-amz-outpost-id` | - | - | `CreateBucketConfiguration` |
| `CreateJob` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateMultiRegionAccessPoint` | `AccountId -> x-amz-account-id` | - | - | - |
| `CreateStorageLensGroup` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessGrant` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessGrantsInstance` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessGrantsInstanceResourcePolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessGrantsLocation` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessPoint` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessPointForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessPointPolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessPointPolicyForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteAccessPointScope` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteBucket` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteBucketLifecycleConfiguration` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteBucketPolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteBucketReplication` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteBucketTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteJobTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteMultiRegionAccessPoint` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeletePublicAccessBlock` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteStorageLensConfiguration` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteStorageLensConfigurationTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `DeleteStorageLensGroup` | `AccountId -> x-amz-account-id` | - | - | - |
| `DescribeJob` | `AccountId -> x-amz-account-id` | - | - | - |
| `DescribeMultiRegionAccessPointOperation` | `AccountId -> x-amz-account-id` | - | - | - |
| `DissociateAccessGrantsIdentityCenter` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessGrant` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessGrantsInstance` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessGrantsInstanceForPrefix` | `AccountId -> x-amz-account-id` | `S3Prefix -> s3prefix` | - | - |
| `GetAccessGrantsInstanceResourcePolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessGrantsLocation` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPoint` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPointConfigurationForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPointForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPointPolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPointPolicyForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPointPolicyStatus` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPointPolicyStatusForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetAccessPointScope` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetBucket` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetBucketLifecycleConfiguration` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetBucketPolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetBucketReplication` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetBucketTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetBucketVersioning` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetDataAccess` | `AccountId -> x-amz-account-id` | `Target -> target`, `Permission -> permission`, `DurationSeconds -> durationSeconds`, `Privilege -> privilege`, `TargetType -> targetType`, `AuditContext -> auditContext` | - | - |
| `GetJobTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetMultiRegionAccessPoint` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetMultiRegionAccessPointPolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetMultiRegionAccessPointPolicyStatus` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetMultiRegionAccessPointRoutes` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetPublicAccessBlock` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetStorageLensConfiguration` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetStorageLensConfigurationTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `GetStorageLensGroup` | `AccountId -> x-amz-account-id` | - | - | - |
| `ListAccessGrants` | `AccountId -> x-amz-account-id` | `NextToken -> nextToken`, `MaxResults -> maxResults`, `GranteeType -> granteetype`, `GranteeIdentifier -> granteeidentifier`, `Permission -> permission`, `GrantScope -> grantscope`, `ApplicationArn -> application_arn` | - | - |
| `ListAccessGrantsInstances` | `AccountId -> x-amz-account-id` | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListAccessGrantsLocations` | `AccountId -> x-amz-account-id` | `NextToken -> nextToken`, `MaxResults -> maxResults`, `LocationScope -> locationscope` | - | - |
| `ListAccessPoints` | `AccountId -> x-amz-account-id` | `Bucket -> bucket`, `NextToken -> nextToken`, `MaxResults -> maxResults`, `DataSourceId -> dataSourceId`, `DataSourceType -> dataSourceType` | - | - |
| `ListAccessPointsForDirectoryBuckets` | `AccountId -> x-amz-account-id` | `DirectoryBucket -> directoryBucket`, `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListAccessPointsForObjectLambda` | `AccountId -> x-amz-account-id` | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListCallerAccessGrants` | `AccountId -> x-amz-account-id` | `GrantScope -> grantscope`, `NextToken -> nextToken`, `MaxResults -> maxResults`, `AllowedByApplication -> allowedByApplication` | - | - |
| `ListJobs` | `AccountId -> x-amz-account-id` | `JobStatuses -> jobStatuses`, `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListMultiRegionAccessPoints` | `AccountId -> x-amz-account-id` | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListRegionalBuckets` | `AccountId -> x-amz-account-id`, `OutpostId -> x-amz-outpost-id` | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListStorageLensConfigurations` | `AccountId -> x-amz-account-id` | `NextToken -> nextToken` | - | - |
| `ListStorageLensGroups` | `AccountId -> x-amz-account-id` | `NextToken -> nextToken` | - | - |
| `ListTagsForResource` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutAccessGrantsInstanceResourcePolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutAccessPointConfigurationForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutAccessPointPolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutAccessPointPolicyForObjectLambda` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutAccessPointScope` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutBucketLifecycleConfiguration` | `AccountId -> x-amz-account-id` | - | - | `LifecycleConfiguration` |
| `PutBucketPolicy` | `AccountId -> x-amz-account-id`, `ConfirmRemoveSelfBucketAccess -> x-amz-confirm-remove-self-bucket-access` | - | - | - |
| `PutBucketReplication` | `AccountId -> x-amz-account-id` | - | - | `ReplicationConfiguration` |
| `PutBucketTagging` | `AccountId -> x-amz-account-id` | - | - | `Tagging` |
| `PutBucketVersioning` | `AccountId -> x-amz-account-id`, `MFA -> x-amz-mfa` | - | - | `VersioningConfiguration` |
| `PutJobTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutMultiRegionAccessPointPolicy` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutPublicAccessBlock` | `AccountId -> x-amz-account-id` | - | - | `PublicAccessBlockConfiguration` |
| `PutStorageLensConfiguration` | `AccountId -> x-amz-account-id` | - | - | - |
| `PutStorageLensConfigurationTagging` | `AccountId -> x-amz-account-id` | - | - | - |
| `SubmitMultiRegionAccessPointRoutes` | `AccountId -> x-amz-account-id` | - | - | - |
| `TagResource` | `AccountId -> x-amz-account-id` | - | - | - |
| `UntagResource` | `AccountId -> x-amz-account-id` | `TagKeys -> tagKeys` | - | - |
| `UpdateAccessGrantsLocation` | `AccountId -> x-amz-account-id` | - | - | - |
| `UpdateJobPriority` | `AccountId -> x-amz-account-id` | `Priority -> priority` | - | - |
| `UpdateJobStatus` | `AccountId -> x-amz-account-id` | `RequestedJobStatus -> requestedJobStatus`, `StatusUpdateReason -> statusUpdateReason` | - | - |
| `UpdateStorageLensGroup` | `AccountId -> x-amz-account-id` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceException` | `structure` | `Message` | - |
| `TooManyRequestsException` | `structure` | `Message` | - |
| `NotFoundException` | `structure` | `Message` | - |
| `BadRequestException` | `structure` | `Message` | - |
| `AssociateAccessGrantsIdentityCenterRequest` | `structure` | `AccountId`, `IdentityCenterArn` | - |
| `CreateAccessGrantRequest` | `structure` | `AccessGrantsLocationConfiguration`, `AccessGrantsLocationId`, `AccountId`, `ApplicationArn`, `Grantee`, `Permission`, `S3PrefixType`, `Tags` | - |
| `CreateAccessGrantResult` | `structure` | `AccessGrantArn`, `AccessGrantId`, `AccessGrantsLocationConfiguration`, `AccessGrantsLocationId`, `ApplicationArn`, `CreatedAt`, `GrantScope`, `Grantee`, `Permission` | - |
| `CreateAccessGrantsInstanceRequest` | `structure` | `AccountId`, `IdentityCenterArn`, `Tags` | - |
| `CreateAccessGrantsInstanceResult` | `structure` | `AccessGrantsInstanceArn`, `AccessGrantsInstanceId`, `CreatedAt`, `IdentityCenterApplicationArn`, `IdentityCenterArn`, `IdentityCenterInstanceArn` | - |
| `CreateAccessGrantsLocationRequest` | `structure` | `AccountId`, `IAMRoleArn`, `LocationScope`, `Tags` | - |
| `CreateAccessGrantsLocationResult` | `structure` | `AccessGrantsLocationArn`, `AccessGrantsLocationId`, `CreatedAt`, `IAMRoleArn`, `LocationScope` | - |
| `CreateAccessPointRequest` | `structure` | `AccountId`, `Bucket`, `BucketAccountId`, `Name`, `PublicAccessBlockConfiguration`, `Scope`, `Tags`, `VpcConfiguration` | - |
| `CreateAccessPointResult` | `structure` | `AccessPointArn`, `Alias` | - |
| `CreateAccessPointForObjectLambdaRequest` | `structure` | `AccountId`, `Configuration`, `Name` | - |
| `CreateAccessPointForObjectLambdaResult` | `structure` | `Alias`, `ObjectLambdaAccessPointArn` | - |
| `CreateBucketRequest` | `structure` | `ACL`, `Bucket`, `CreateBucketConfiguration`, `GrantFullControl`, `GrantRead`, `GrantReadACP`, `GrantWrite`, `GrantWriteACP`, `ObjectLockEnabledForBucket`, `OutpostId` | - |
| `CreateBucketResult` | `structure` | `BucketArn`, `Location` | - |
| `BucketAlreadyExists` | `structure` | - | The requested Outposts bucket name is not available. |
| `BucketAlreadyOwnedByYou` | `structure` | - | The Outposts bucket you tried to create already exists, and you own it. |
| `CreateJobRequest` | `structure` | `AccountId`, `ClientRequestToken`, `ConfirmationRequired`, `Description`, `Manifest`, `ManifestGenerator`, `Operation`, `Priority`, `Report`, `RoleArn`, `Tags` | - |
| `CreateJobResult` | `structure` | `JobId` | - |
| `IdempotencyException` | `structure` | `Message` | - |
| `CreateMultiRegionAccessPointRequest` | `structure` | `AccountId`, `ClientToken`, `Details` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
