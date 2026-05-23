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
- Common required input members in this group: `AccountId`, `Name`, `Bucket`, `ConfigId`

### Delete

- Operations: `DeleteAccessGrant`, `DeleteAccessGrantsInstance`, `DeleteAccessGrantsInstanceResourcePolicy`, `DeleteAccessGrantsLocation`, `DeleteAccessPoint`, `DeleteAccessPointForObjectLambda`, `DeleteAccessPointPolicy`, `DeleteAccessPointPolicyForObjectLambda`, `DeleteAccessPointScope`, `DeleteBucket`, `DeleteBucketLifecycleConfiguration`, `DeleteBucketPolicy`, `DeleteBucketReplication`, `DeleteBucketTagging`, `DeleteJobTagging`, `DeleteMultiRegionAccessPoint`, `DeletePublicAccessBlock`, `DeleteStorageLensConfiguration`, `DeleteStorageLensConfigurationTagging`, `DeleteStorageLensGroup`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AccountId`, `Name`, `Bucket`, `ConfigId`

### Put

- Operations: `PutAccessGrantsInstanceResourcePolicy`, `PutAccessPointConfigurationForObjectLambda`, `PutAccessPointPolicy`, `PutAccessPointPolicyForObjectLambda`, `PutAccessPointScope`, `PutBucketLifecycleConfiguration`, `PutBucketPolicy`, `PutBucketReplication`, `PutBucketTagging`, `PutBucketVersioning`, `PutJobTagging`, `PutMultiRegionAccessPointPolicy`, `PutPublicAccessBlock`, `PutStorageLensConfiguration`, `PutStorageLensConfigurationTagging`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AccountId`, `Policy`, `Name`, `Bucket`, `Tags`, `ConfigId`

### List

- Operations: `ListAccessGrants`, `ListAccessGrantsInstances`, `ListAccessGrantsLocations`, `ListAccessPoints`, `ListAccessPointsForDirectoryBuckets`, `ListAccessPointsForObjectLambda`, `ListCallerAccessGrants`, `ListJobs`, `ListMultiRegionAccessPoints`, `ListRegionalBuckets`, `ListStorageLensConfigurations`, `ListStorageLensGroups`, `ListTagsForResource`
- Traits: `paginated` (12)
- Common required input members in this group: `AccountId`

### Create

- Operations: `CreateAccessGrant`, `CreateAccessGrantsInstance`, `CreateAccessGrantsLocation`, `CreateAccessPoint`, `CreateAccessPointForObjectLambda`, `CreateBucket`, `CreateJob`, `CreateMultiRegionAccessPoint`, `CreateStorageLensGroup`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `AccountId`, `Name`, `Bucket`

### Update

- Operations: `UpdateAccessGrantsLocation`, `UpdateJobPriority`, `UpdateJobStatus`, `UpdateStorageLensGroup`
- Common required input members in this group: `AccountId`, `JobId`

### Describe

- Operations: `DescribeJob`, `DescribeMultiRegionAccessPointOperation`
- Common required input members in this group: `AccountId`

### Associate

- Operations: `AssociateAccessGrantsIdentityCenter`
- Common required input members in this group: -

### Dissociate

- Operations: `DissociateAccessGrantsIdentityCenter`
- Common required input members in this group: -

### Submit

- Operations: `SubmitMultiRegionAccessPointRoutes`
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
| `AssociateAccessGrantsIdentityCenter` | `POST /v20180820/accessgrantsinstance/identitycenter` | - | `AccountId`, `IdentityCenterArn` | - | `Unit` | - | Associate your S3 Access Grants instance with an Amazon Web Services IAM Identity Center instance. Use this action if you want to create access grants for users or groups from your corporate identity directory. First ... |
| `CreateAccessGrant` | `POST /v20180820/accessgrantsinstance/grant` | - | `AccountId`, `AccessGrantsLocationId`, `Grantee`, `Permission` | - | `CreateAccessGrantResult` | - | Creates an access grant that gives a grantee access to your S3 data. The grantee can be an IAM user or role or a directory user, or group. Before you can create a grant, you must have an S3 Access Grants instance in ... |
| `CreateAccessGrantsInstance` | `POST /v20180820/accessgrantsinstance` | - | `AccountId` | - | `CreateAccessGrantsInstanceResult` | - | Creates an S3 Access Grants instance, which serves as a logical grouping for access grants. You can create one S3 Access Grants instance per Region per account. Permissions You must have the s3:CreateAccessGrantsInst ... |
| `CreateAccessGrantsLocation` | `POST /v20180820/accessgrantsinstance/location` | - | `AccountId`, `LocationScope`, `IAMRoleArn` | - | `CreateAccessGrantsLocationResult` | - | The S3 data location that you would like to register in your S3 Access Grants instance. Your S3 data must be in the same Region as your S3 Access Grants instance. The location can be one of the following: The default ... |
| `CreateAccessPoint` | `PUT /v20180820/accesspoint/{Name}` | - | `AccountId`, `Name`, `Bucket` | - | `CreateAccessPointResult` | - | Creates an access point and associates it to a specified bucket. For more information, see Managing access to shared datasets with access points or Managing access to shared datasets in directory buckets with access ... |
| `CreateAccessPointForObjectLambda` | `PUT /v20180820/accesspointforobjectlambda/{Name}` | - | `AccountId`, `Name`, `Configuration` | - | `CreateAccessPointForObjectLambdaResult` | - | This operation is not supported by directory buckets. Creates an Object Lambda Access Point. For more information, see Transforming objects with Object Lambda Access Points in the Amazon S3 User Guide . The following ... |
| `CreateBucket` | `PUT /v20180820/bucket/{Bucket}` | - | `Bucket` | - | `CreateBucketResult` | `BucketAlreadyExists`, `BucketAlreadyOwnedByYou` | This action creates an Amazon S3 on Outposts bucket. To create an S3 bucket, see Create Bucket in the Amazon S3 API Reference . Creates a new Outposts bucket. By creating the bucket, you become the bucket owner. To c ... |
| `CreateJob` | `POST /v20180820/jobs` | `idempotency-token` | `AccountId`, `Operation`, `Report`, `ClientRequestToken`, `Priority`, `RoleArn` | `ClientRequestToken` | `CreateJobResult` | `BadRequestException`, `IdempotencyException`, `InternalServiceException`, `TooManyRequestsException` | This operation creates an S3 Batch Operations job. You can use S3 Batch Operations to perform large-scale batch actions on Amazon S3 objects. Batch Operations can run a single action on lists of Amazon S3 objects tha ... |
| `CreateMultiRegionAccessPoint` | `POST /v20180820/async-requests/mrap/create` | `idempotency-token` | `AccountId`, `ClientToken`, `Details` | `ClientToken` | `CreateMultiRegionAccessPointResult` | - | This operation is not supported by directory buckets. Creates a Multi-Region Access Point and associates it with the specified buckets. For more information about creating Multi-Region Access Points, see Creating Mul ... |
| `CreateStorageLensGroup` | `POST /v20180820/storagelensgroup` | - | `AccountId`, `StorageLensGroup` | - | `Unit` | - | Creates a new S3 Storage Lens group and associates it with the specified Amazon Web Services account ID. An S3 Storage Lens group is a custom grouping of objects based on prefix, suffix, object tags, object size, obj ... |
| `DeleteAccessGrant` | `DELETE /v20180820/accessgrantsinstance/grant/{AccessGrantId}` | - | `AccountId`, `AccessGrantId` | - | `Unit` | - | Deletes the access grant from the S3 Access Grants instance. You cannot undo an access grant deletion and the grantee will no longer have access to the S3 data. Permissions You must have the s3:DeleteAccessGrant perm ... |
| `DeleteAccessGrantsInstance` | `DELETE /v20180820/accessgrantsinstance` | - | `AccountId` | - | `Unit` | - | Deletes your S3 Access Grants instance. You must first delete the access grants and locations before S3 Access Grants can delete the instance. See DeleteAccessGrant and DeleteAccessGrantsLocation . If you have associ ... |
| `DeleteAccessGrantsInstanceResourcePolicy` | `DELETE /v20180820/accessgrantsinstance/resourcepolicy` | - | `AccountId` | - | `Unit` | - | Deletes the resource policy of the S3 Access Grants instance. The resource policy is used to manage cross-account access to your S3 Access Grants instance. By deleting the resource policy, you delete any cross-accoun ... |
| `DeleteAccessGrantsLocation` | `DELETE /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}` | - | `AccountId`, `AccessGrantsLocationId` | - | `Unit` | - | Deregisters a location from your S3 Access Grants instance. You can only delete a location registration from an S3 Access Grants instance if there are no grants associated with this location. See Delete a grant for i ... |
| `DeleteAccessPoint` | `DELETE /v20180820/accesspoint/{Name}` | - | `AccountId`, `Name` | - | `Unit` | - | Deletes the specified access point. All Amazon S3 on Outposts REST API requests for this action require an additional parameter of x-amz-outpost-id to be passed with the request. In addition, you must use an S3 on Ou ... |
| `DeleteAccessPointForObjectLambda` | `DELETE /v20180820/accesspointforobjectlambda/{Name}` | - | `AccountId`, `Name` | - | `Unit` | - | This operation is not supported by directory buckets. Deletes the specified Object Lambda Access Point. The following actions are related to DeleteAccessPointForObjectLambda : CreateAccessPointForObjectLambda GetAcce ... |
| `DeleteAccessPointPolicy` | `DELETE /v20180820/accesspoint/{Name}/policy` | - | `AccountId`, `Name` | - | `Unit` | - | Deletes the access point policy for the specified access point. All Amazon S3 on Outposts REST API requests for this action require an additional parameter of x-amz-outpost-id to be passed with the request. In additi ... |
| `DeleteAccessPointPolicyForObjectLambda` | `DELETE /v20180820/accesspointforobjectlambda/{Name}/policy` | - | `AccountId`, `Name` | - | `Unit` | - | This operation is not supported by directory buckets. Removes the resource policy for an Object Lambda Access Point. The following actions are related to DeleteAccessPointPolicyForObjectLambda : GetAccessPointPolicyF ... |
| `DeleteAccessPointScope` | `DELETE /v20180820/accesspoint/{Name}/scope` | - | `AccountId`, `Name` | - | `Unit` | - | Deletes an existing access point scope for a directory bucket. When you delete the scope of an access point, all prefixes and permissions are deleted. To use this operation, you must have the permission to perform th ... |
| `DeleteBucket` | `DELETE /v20180820/bucket/{Bucket}` | - | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket. To delete an S3 bucket, see DeleteBucket in the Amazon S3 API Reference . Deletes the Amazon S3 on Outposts bucket. All objects (including all object versions and ... |
| `DeleteBucketLifecycleConfiguration` | `DELETE /v20180820/bucket/{Bucket}/lifecycleconfiguration` | - | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket's lifecycle configuration. To delete an S3 bucket's lifecycle configuration, see DeleteBucketLifecycle in the Amazon S3 API Reference . Deletes the lifecycle config ... |
| `DeleteBucketPolicy` | `DELETE /v20180820/bucket/{Bucket}/policy` | - | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket policy. To delete an S3 bucket policy, see DeleteBucketPolicy in the Amazon S3 API Reference . This implementation of the DELETE action uses the policy subresource ... |
| `DeleteBucketReplication` | `DELETE /v20180820/bucket/{Bucket}/replication` | - | `AccountId`, `Bucket` | - | `Unit` | - | This operation deletes an Amazon S3 on Outposts bucket's replication configuration. To delete an S3 bucket's replication configuration, see DeleteBucketReplication in the Amazon S3 API Reference . Deletes the replica ... |
| `DeleteBucketTagging` | `DELETE /v20180820/bucket/{Bucket}/tagging` | - | `AccountId`, `Bucket` | - | `Unit` | - | This action deletes an Amazon S3 on Outposts bucket's tags. To delete an S3 bucket tags, see DeleteBucketTagging in the Amazon S3 API Reference . Deletes the tags from the Outposts bucket. For more information, see U ... |
| `DeleteJobTagging` | `DELETE /v20180820/jobs/{JobId}/tagging` | - | `AccountId`, `JobId` | - | `DeleteJobTaggingResult` | `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Removes the entire tag set from the specified S3 Batch Operations job. Permissions To use the DeleteJobTagging operation, you must have permission to perform the s3:DeleteJobTagging action. For more information, see ... |
| `DeleteMultiRegionAccessPoint` | `POST /v20180820/async-requests/mrap/delete` | `idempotency-token` | `AccountId`, `ClientToken`, `Details` | `ClientToken` | `DeleteMultiRegionAccessPointResult` | - | This operation is not supported by directory buckets. Deletes a Multi-Region Access Point. This action does not delete the buckets associated with the Multi-Region Access Point, only the Multi-Region Access Point its ... |
| `DeletePublicAccessBlock` | `DELETE /v20180820/configuration/publicAccessBlock` | - | `AccountId` | - | `Unit` | - | This operation is not supported by directory buckets. Removes the PublicAccessBlock configuration for an Amazon Web Services account. This operation might be restricted when the account is managed by organization-lev ... |
| `DeleteStorageLensConfiguration` | `DELETE /v20180820/storagelens/{ConfigId}` | - | `ConfigId`, `AccountId` | - | `Unit` | - | This operation is not supported by directory buckets. Deletes the Amazon S3 Storage Lens configuration. For more information about S3 Storage Lens, see Assessing your storage activity and usage with Amazon S3 Storage ... |
| `DeleteStorageLensConfigurationTagging` | `DELETE /v20180820/storagelens/{ConfigId}/tagging` | - | `ConfigId`, `AccountId` | - | `DeleteStorageLensConfigurationTaggingResult` | - | This operation is not supported by directory buckets. Deletes the Amazon S3 Storage Lens configuration tags. For more information about S3 Storage Lens, see Assessing your storage activity and usage with Amazon S3 St ... |
| `DeleteStorageLensGroup` | `DELETE /v20180820/storagelensgroup/{Name}` | - | `Name`, `AccountId` | - | `Unit` | - | Deletes an existing S3 Storage Lens group. To use this operation, you must have the permission to perform the s3:DeleteStorageLensGroup action. For more information about the required Storage Lens Groups permissions, ... |
| `DescribeJob` | `GET /v20180820/jobs/{JobId}` | - | `AccountId`, `JobId` | - | `DescribeJobResult` | `BadRequestException`, `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the configuration parameters and status for a Batch Operations job. For more information, see S3 Batch Operations in the Amazon S3 User Guide . Permissions To use the DescribeJob operation, you must have pe ... |
| `DescribeMultiRegionAccessPointOperation` | `GET /v20180820/async-requests/mrap/{RequestTokenARN+}` | - | `AccountId`, `RequestTokenARN` | - | `DescribeMultiRegionAccessPointOperationResult` | - | This operation is not supported by directory buckets. Retrieves the status of an asynchronous request to manage a Multi-Region Access Point. For more information about managing Multi-Region Access Points and how asyn ... |
| `DissociateAccessGrantsIdentityCenter` | `DELETE /v20180820/accessgrantsinstance/identitycenter` | - | `AccountId` | - | `Unit` | - | Dissociates the Amazon Web Services IAM Identity Center instance from the S3 Access Grants instance. Permissions You must have the s3:DissociateAccessGrantsIdentityCenter permission to use this operation. Additional ... |
| `GetAccessGrant` | `GET /v20180820/accessgrantsinstance/grant/{AccessGrantId}` | - | `AccountId`, `AccessGrantId` | - | `GetAccessGrantResult` | - | Get the details of an access grant from your S3 Access Grants instance. Permissions You must have the s3:GetAccessGrant permission to use this operation. |
| `GetAccessGrantsInstance` | `GET /v20180820/accessgrantsinstance` | - | `AccountId` | - | `GetAccessGrantsInstanceResult` | - | Retrieves the S3 Access Grants instance for a Region in your account. Permissions You must have the s3:GetAccessGrantsInstance permission to use this operation. GetAccessGrantsInstance is not supported for cross-acco ... |
| `GetAccessGrantsInstanceForPrefix` | `GET /v20180820/accessgrantsinstance/prefix` | - | `AccountId`, `S3Prefix` | - | `GetAccessGrantsInstanceForPrefixResult` | - | Retrieve the S3 Access Grants instance that contains a particular prefix. Permissions You must have the s3:GetAccessGrantsInstanceForPrefix permission for the caller account to use this operation. Additional Permissi ... |
| `GetAccessGrantsInstanceResourcePolicy` | `GET /v20180820/accessgrantsinstance/resourcepolicy` | - | `AccountId` | - | `GetAccessGrantsInstanceResourcePolicyResult` | - | Returns the resource policy of the S3 Access Grants instance. Permissions You must have the s3:GetAccessGrantsInstanceResourcePolicy permission to use this operation. |
| `GetAccessGrantsLocation` | `GET /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}` | - | `AccountId`, `AccessGrantsLocationId` | - | `GetAccessGrantsLocationResult` | - | Retrieves the details of a particular location registered in your S3 Access Grants instance. Permissions You must have the s3:GetAccessGrantsLocation permission to use this operation. |
| `GetAccessPoint` | `GET /v20180820/accesspoint/{Name}` | - | `AccountId`, `Name` | - | `GetAccessPointResult` | - | Returns configuration information about the specified access point. All Amazon S3 on Outposts REST API requests for this action require an additional parameter of x-amz-outpost-id to be passed with the request. In ad ... |
| `GetAccessPointConfigurationForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}/configuration` | - | `AccountId`, `Name` | - | `GetAccessPointConfigurationForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns configuration for an Object Lambda Access Point. The following actions are related to GetAccessPointConfigurationForObjectLambda : PutAccessPointConfigura ... |
| `GetAccessPointForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}` | - | `AccountId`, `Name` | - | `GetAccessPointForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns configuration information about the specified Object Lambda Access Point The following actions are related to GetAccessPointForObjectLambda : CreateAccess ... |
| `GetAccessPointPolicy` | `GET /v20180820/accesspoint/{Name}/policy` | - | `AccountId`, `Name` | - | `GetAccessPointPolicyResult` | - | Returns the access point policy associated with the specified access point. The following actions are related to GetAccessPointPolicy : PutAccessPointPolicy DeleteAccessPointPolicy |
| `GetAccessPointPolicyForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}/policy` | - | `AccountId`, `Name` | - | `GetAccessPointPolicyForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns the resource policy for an Object Lambda Access Point. The following actions are related to GetAccessPointPolicyForObjectLambda : DeleteAccessPointPolicyF ... |
| `GetAccessPointPolicyStatus` | `GET /v20180820/accesspoint/{Name}/policyStatus` | - | `AccountId`, `Name` | - | `GetAccessPointPolicyStatusResult` | - | This operation is not supported by directory buckets. Indicates whether the specified access point currently has a policy that allows public access. For more information about public access through access points, see ... |
| `GetAccessPointPolicyStatusForObjectLambda` | `GET /v20180820/accesspointforobjectlambda/{Name}/policyStatus` | - | `AccountId`, `Name` | - | `GetAccessPointPolicyStatusForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns the status of the resource policy associated with an Object Lambda Access Point. |
| `GetAccessPointScope` | `GET /v20180820/accesspoint/{Name}/scope` | - | `AccountId`, `Name` | - | `GetAccessPointScopeResult` | - | Returns the access point scope for a directory bucket. To use this operation, you must have the permission to perform the s3express:GetAccessPointScope action. For information about REST API errors, see REST error re ... |
| `GetBucket` | `GET /v20180820/bucket/{Bucket}` | - | `AccountId`, `Bucket` | - | `GetBucketResult` | - | Gets an Amazon S3 on Outposts bucket. For more information, see Using Amazon S3 on Outposts in the Amazon S3 User Guide . If you are using an identity other than the root user of the Amazon Web Services account that ... |
| `GetBucketLifecycleConfiguration` | `GET /v20180820/bucket/{Bucket}/lifecycleconfiguration` | - | `AccountId`, `Bucket` | - | `GetBucketLifecycleConfigurationResult` | - | This action gets an Amazon S3 on Outposts bucket's lifecycle configuration. To get an S3 bucket's lifecycle configuration, see GetBucketLifecycleConfiguration in the Amazon S3 API Reference . Returns the lifecycle co ... |
| `GetBucketPolicy` | `GET /v20180820/bucket/{Bucket}/policy` | - | `AccountId`, `Bucket` | - | `GetBucketPolicyResult` | - | This action gets a bucket policy for an Amazon S3 on Outposts bucket. To get a policy for an S3 bucket, see GetBucketPolicy in the Amazon S3 API Reference . Returns the policy of a specified Outposts bucket. For more ... |
| `GetBucketReplication` | `GET /v20180820/bucket/{Bucket}/replication` | - | `AccountId`, `Bucket` | - | `GetBucketReplicationResult` | - | This operation gets an Amazon S3 on Outposts bucket's replication configuration. To get an S3 bucket's replication configuration, see GetBucketReplication in the Amazon S3 API Reference . Returns the replication conf ... |
| `GetBucketTagging` | `GET /v20180820/bucket/{Bucket}/tagging` | - | `AccountId`, `Bucket` | - | `GetBucketTaggingResult` | - | This action gets an Amazon S3 on Outposts bucket's tags. To get an S3 bucket tags, see GetBucketTagging in the Amazon S3 API Reference . Returns the tag set associated with the Outposts bucket. For more information, ... |
| `GetBucketVersioning` | `GET /v20180820/bucket/{Bucket}/versioning` | - | `AccountId`, `Bucket` | - | `GetBucketVersioningResult` | - | This operation returns the versioning state for S3 on Outposts buckets only. To return the versioning state for an S3 bucket, see GetBucketVersioning in the Amazon S3 API Reference . Returns the versioning state for ... |
| `GetDataAccess` | `GET /v20180820/accessgrantsinstance/dataaccess` | - | `AccountId`, `Target`, `Permission` | - | `GetDataAccessResult` | - | Returns a temporary access credential from S3 Access Grants to the grantee or client application. The temporary credential is an Amazon Web Services STS token that grants them access to the S3 data. Permissions You m ... |
| `GetJobTagging` | `GET /v20180820/jobs/{JobId}/tagging` | - | `AccountId`, `JobId` | - | `GetJobTaggingResult` | `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Returns the tags on an S3 Batch Operations job. Permissions To use the GetJobTagging operation, you must have permission to perform the s3:GetJobTagging action. For more information, see Controlling access and labeli ... |
| `GetMultiRegionAccessPoint` | `GET /v20180820/mrap/instances/{Name+}` | - | `AccountId`, `Name` | - | `GetMultiRegionAccessPointResult` | - | This operation is not supported by directory buckets. Returns configuration information about the specified Multi-Region Access Point. This action will always be routed to the US West (Oregon) Region. For more inform ... |
| `GetMultiRegionAccessPointPolicy` | `GET /v20180820/mrap/instances/{Name+}/policy` | - | `AccountId`, `Name` | - | `GetMultiRegionAccessPointPolicyResult` | - | This operation is not supported by directory buckets. Returns the access control policy of the specified Multi-Region Access Point. This action will always be routed to the US West (Oregon) Region. For more informati ... |
| `GetMultiRegionAccessPointPolicyStatus` | `GET /v20180820/mrap/instances/{Name+}/policystatus` | - | `AccountId`, `Name` | - | `GetMultiRegionAccessPointPolicyStatusResult` | - | This operation is not supported by directory buckets. Indicates whether the specified Multi-Region Access Point has an access control policy that allows public access. This action will always be routed to the US West ... |
| `GetMultiRegionAccessPointRoutes` | `GET /v20180820/mrap/instances/{Mrap+}/routes` | - | `AccountId`, `Mrap` | - | `GetMultiRegionAccessPointRoutesResult` | - | This operation is not supported by directory buckets. Returns the routing configuration for a Multi-Region Access Point, indicating which Regions are active or passive. To obtain routing control changes and failover ... |
| `GetPublicAccessBlock` | `GET /v20180820/configuration/publicAccessBlock` | - | `AccountId` | - | `GetPublicAccessBlockOutput` | `NoSuchPublicAccessBlockConfiguration` | This operation is not supported by directory buckets. Retrieves the PublicAccessBlock configuration for an Amazon Web Services account. This operation returns the effective account-level configuration, which may inhe ... |
| `GetStorageLensConfiguration` | `GET /v20180820/storagelens/{ConfigId}` | - | `ConfigId`, `AccountId` | - | `GetStorageLensConfigurationResult` | - | This operation is not supported by directory buckets. Gets the Amazon S3 Storage Lens configuration. For more information, see Assessing your storage activity and usage with Amazon S3 Storage Lens in the Amazon S3 Us ... |
| `GetStorageLensConfigurationTagging` | `GET /v20180820/storagelens/{ConfigId}/tagging` | - | `ConfigId`, `AccountId` | - | `GetStorageLensConfigurationTaggingResult` | - | This operation is not supported by directory buckets. Gets the tags of Amazon S3 Storage Lens configuration. For more information about S3 Storage Lens, see Assessing your storage activity and usage with Amazon S3 St ... |
| `GetStorageLensGroup` | `GET /v20180820/storagelensgroup/{Name}` | - | `Name`, `AccountId` | - | `GetStorageLensGroupResult` | - | Retrieves the Storage Lens group configuration details. To use this operation, you must have the permission to perform the s3:GetStorageLensGroup action. For more information about the required Storage Lens Groups pe ... |
| `ListAccessGrants` | `GET /v20180820/accessgrantsinstance/grants` | `paginated` | `AccountId` | - | `ListAccessGrantsResult` | - | Returns the list of access grants in your S3 Access Grants instance. Permissions You must have the s3:ListAccessGrants permission to use this operation. |
| `ListAccessGrantsInstances` | `GET /v20180820/accessgrantsinstances` | `paginated` | `AccountId` | - | `ListAccessGrantsInstancesResult` | - | Returns a list of S3 Access Grants instances. An S3 Access Grants instance serves as a logical grouping for your individual access grants. You can only have one S3 Access Grants instance per Region per account. Permi ... |
| `ListAccessGrantsLocations` | `GET /v20180820/accessgrantsinstance/locations` | `paginated` | `AccountId` | - | `ListAccessGrantsLocationsResult` | - | Returns a list of the locations registered in your S3 Access Grants instance. Permissions You must have the s3:ListAccessGrantsLocations permission to use this operation. |
| `ListAccessPoints` | `GET /v20180820/accesspoint` | `paginated` | `AccountId` | - | `ListAccessPointsResult` | - | This operation is not supported by directory buckets. Returns a list of the access points. You can retrieve up to 1,000 access points per call. If the call returns more than 1,000 access points (or the number specifi ... |
| `ListAccessPointsForDirectoryBuckets` | `GET /v20180820/accesspointfordirectory` | `paginated` | `AccountId` | - | `ListAccessPointsForDirectoryBucketsResult` | - | Returns a list of the access points that are owned by the Amazon Web Services account and that are associated with the specified directory bucket. To list access points for general purpose buckets, see ListAccesspoin ... |
| `ListAccessPointsForObjectLambda` | `GET /v20180820/accesspointforobjectlambda` | `paginated` | `AccountId` | - | `ListAccessPointsForObjectLambdaResult` | - | This operation is not supported by directory buckets. Returns some or all (up to 1,000) access points associated with the Object Lambda Access Point per call. If there are more access points than what can be returned ... |
| `ListCallerAccessGrants` | `GET /v20180820/accessgrantsinstance/caller/grants` | `paginated` | `AccountId` | - | `ListCallerAccessGrantsResult` | - | Use this API to list the access grants that grant the caller access to Amazon S3 data through S3 Access Grants. The caller (grantee) can be an Identity and Access Management (IAM) identity or Amazon Web Services Iden ... |
| `ListJobs` | `GET /v20180820/jobs` | `paginated` | `AccountId` | - | `ListJobsResult` | `InternalServiceException`, `InvalidNextTokenException`, `InvalidRequestException` | Lists current S3 Batch Operations jobs as well as the jobs that have ended within the last 90 days for the Amazon Web Services account making the request. For more information, see S3 Batch Operations in the Amazon S ... |
| `ListMultiRegionAccessPoints` | `GET /v20180820/mrap/instances` | `paginated` | `AccountId` | - | `ListMultiRegionAccessPointsResult` | - | This operation is not supported by directory buckets. Returns a list of the Multi-Region Access Points currently associated with the specified Amazon Web Services account. Each call can return up to 100 Multi-Region ... |
| `ListRegionalBuckets` | `GET /v20180820/bucket` | `paginated` | `AccountId` | - | `ListRegionalBucketsResult` | - | This operation is not supported by directory buckets. Returns a list of all Outposts buckets in an Outpost that are owned by the authenticated sender of the request. For more information, see Using Amazon S3 on Outpo ... |
| `ListStorageLensConfigurations` | `GET /v20180820/storagelens` | `paginated` | `AccountId` | - | `ListStorageLensConfigurationsResult` | - | This operation is not supported by directory buckets. Gets a list of Amazon S3 Storage Lens configurations. For more information about S3 Storage Lens, see Assessing your storage activity and usage with Amazon S3 Sto ... |
| `ListStorageLensGroups` | `GET /v20180820/storagelensgroup` | `paginated` | `AccountId` | - | `ListStorageLensGroupsResult` | - | Lists all the Storage Lens groups in the specified home Region. To use this operation, you must have the permission to perform the s3:ListStorageLensGroups action. For more information about the required Storage Lens ... |
| `ListTagsForResource` | `GET /v20180820/tags/{ResourceArn+}` | - | `AccountId`, `ResourceArn` | - | `ListTagsForResourceResult` | - | This operation allows you to list all of the tags for a specified resource. Each tag is a label consisting of a key and value. Tags can help you organize, track costs for, and control access to resources. This operat ... |
| `PutAccessGrantsInstanceResourcePolicy` | `PUT /v20180820/accessgrantsinstance/resourcepolicy` | - | `AccountId`, `Policy` | - | `PutAccessGrantsInstanceResourcePolicyResult` | - | Updates the resource policy of the S3 Access Grants instance. Permissions You must have the s3:PutAccessGrantsInstanceResourcePolicy permission to use this operation. |
| `PutAccessPointConfigurationForObjectLambda` | `PUT /v20180820/accesspointforobjectlambda/{Name}/configuration` | - | `AccountId`, `Name`, `Configuration` | - | `Unit` | - | This operation is not supported by directory buckets. Replaces configuration for an Object Lambda Access Point. The following actions are related to PutAccessPointConfigurationForObjectLambda : GetAccessPointConfigur ... |
| `PutAccessPointPolicy` | `PUT /v20180820/accesspoint/{Name}/policy` | - | `AccountId`, `Name`, `Policy` | - | `Unit` | - | Associates an access policy with the specified access point. Each access point can have only one policy, so a request made to this API replaces any existing policy associated with the specified access point. All Amaz ... |
| `PutAccessPointPolicyForObjectLambda` | `PUT /v20180820/accesspointforobjectlambda/{Name}/policy` | - | `AccountId`, `Name`, `Policy` | - | `Unit` | - | This operation is not supported by directory buckets. Creates or replaces resource policy for an Object Lambda Access Point. For an example policy, see Creating Object Lambda Access Points in the Amazon S3 User Guide ... |
| `PutAccessPointScope` | `PUT /v20180820/accesspoint/{Name}/scope` | - | `AccountId`, `Name`, `Scope` | - | `Unit` | - | Creates or replaces the access point scope for a directory bucket. You can use the access point scope to restrict access to specific prefixes, API operations, or a combination of both. You can specify any amount of p ... |
| `PutBucketLifecycleConfiguration` | `PUT /v20180820/bucket/{Bucket}/lifecycleconfiguration` | - | `AccountId`, `Bucket` | - | `Unit` | - | This action puts a lifecycle configuration to an Amazon S3 on Outposts bucket. To put a lifecycle configuration to an S3 bucket, see PutBucketLifecycleConfiguration in the Amazon S3 API Reference . Creates a new life ... |
| `PutBucketPolicy` | `PUT /v20180820/bucket/{Bucket}/policy` | - | `AccountId`, `Bucket`, `Policy` | - | `Unit` | - | This action puts a bucket policy to an Amazon S3 on Outposts bucket. To put a policy on an S3 bucket, see PutBucketPolicy in the Amazon S3 API Reference . Applies an Amazon S3 bucket policy to an Outposts bucket. For ... |
| `PutBucketReplication` | `PUT /v20180820/bucket/{Bucket}/replication` | - | `AccountId`, `Bucket`, `ReplicationConfiguration` | - | `Unit` | - | This action creates an Amazon S3 on Outposts bucket's replication configuration. To create an S3 bucket's replication configuration, see PutBucketReplication in the Amazon S3 API Reference . Creates a replication con ... |
| `PutBucketTagging` | `PUT /v20180820/bucket/{Bucket}/tagging` | - | `AccountId`, `Bucket`, `Tagging` | - | `Unit` | - | This action puts tags on an Amazon S3 on Outposts bucket. To put tags on an S3 bucket, see PutBucketTagging in the Amazon S3 API Reference . Sets the tags for an S3 on Outposts bucket. For more information, see Using ... |
| `PutBucketVersioning` | `PUT /v20180820/bucket/{Bucket}/versioning` | - | `AccountId`, `Bucket`, `VersioningConfiguration` | - | `Unit` | - | This operation sets the versioning state for S3 on Outposts buckets only. To set the versioning state for an S3 bucket, see PutBucketVersioning in the Amazon S3 API Reference . Sets the versioning state for an S3 on ... |
| `PutJobTagging` | `PUT /v20180820/jobs/{JobId}/tagging` | - | `AccountId`, `JobId`, `Tags` | - | `PutJobTaggingResult` | `InternalServiceException`, `NotFoundException`, `TooManyRequestsException`, `TooManyTagsException` | Sets the supplied tag-set on an S3 Batch Operations job. A tag is a key-value pair. You can associate S3 Batch Operations tags with any job by sending a PUT request against the tagging subresource that is associated ... |
| `PutMultiRegionAccessPointPolicy` | `POST /v20180820/async-requests/mrap/put-policy` | `idempotency-token` | `AccountId`, `ClientToken`, `Details` | `ClientToken` | `PutMultiRegionAccessPointPolicyResult` | - | This operation is not supported by directory buckets. Associates an access control policy with the specified Multi-Region Access Point. Each Multi-Region Access Point can have only one policy, so a request made to th ... |
| `PutPublicAccessBlock` | `PUT /v20180820/configuration/publicAccessBlock` | - | `PublicAccessBlockConfiguration`, `AccountId` | - | `Unit` | - | This operation is not supported by directory buckets. Creates or modifies the PublicAccessBlock configuration for an Amazon Web Services account. This operation may be restricted when the account is managed by organi ... |
| `PutStorageLensConfiguration` | `PUT /v20180820/storagelens/{ConfigId}` | - | `ConfigId`, `AccountId`, `StorageLensConfiguration` | - | `Unit` | - | This operation is not supported by directory buckets. Puts an Amazon S3 Storage Lens configuration. For more information about S3 Storage Lens, see Working with Amazon S3 Storage Lens in the Amazon S3 User Guide . Fo ... |
| `PutStorageLensConfigurationTagging` | `PUT /v20180820/storagelens/{ConfigId}/tagging` | - | `ConfigId`, `AccountId`, `Tags` | - | `PutStorageLensConfigurationTaggingResult` | - | This operation is not supported by directory buckets. Put or replace tags on an existing Amazon S3 Storage Lens configuration. For more information about S3 Storage Lens, see Assessing your storage activity and usage ... |
| `SubmitMultiRegionAccessPointRoutes` | `PATCH /v20180820/mrap/instances/{Mrap+}/routes` | - | `AccountId`, `Mrap`, `RouteUpdates` | - | `SubmitMultiRegionAccessPointRoutesResult` | - | This operation is not supported by directory buckets. Submits an updated route configuration for a Multi-Region Access Point. This API operation updates the routing status for the specified Regions from active to pas ... |
| `TagResource` | `POST /v20180820/tags/{ResourceArn+}` | - | `AccountId`, `ResourceArn`, `Tags` | - | `TagResourceResult` | - | Creates a new user-defined tag or updates an existing tag. Each tag is a label consisting of a key and value that is applied to your resource. Tags can help you organize, track costs for, and control access to your r ... |
| `UntagResource` | `DELETE /v20180820/tags/{ResourceArn+}` | - | `AccountId`, `ResourceArn`, `TagKeys` | - | `UntagResourceResult` | - | This operation removes the specified user-defined tags from an S3 resource. You can pass one or more tag keys. This operation is only supported for the following Amazon S3 resources: General purpose buckets Access Po ... |
| `UpdateAccessGrantsLocation` | `PUT /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}` | - | `AccountId`, `AccessGrantsLocationId`, `IAMRoleArn` | - | `UpdateAccessGrantsLocationResult` | - | Updates the IAM role of a registered location in your S3 Access Grants instance. Permissions You must have the s3:UpdateAccessGrantsLocation permission to use this operation. Additional Permissions You must also have ... |
| `UpdateJobPriority` | `POST /v20180820/jobs/{JobId}/priority` | - | `AccountId`, `JobId`, `Priority` | - | `UpdateJobPriorityResult` | `BadRequestException`, `InternalServiceException`, `NotFoundException`, `TooManyRequestsException` | Updates an existing S3 Batch Operations job's priority. For more information, see S3 Batch Operations in the Amazon S3 User Guide . Permissions To use the UpdateJobPriority operation, you must have permission to perf ... |
| `UpdateJobStatus` | `POST /v20180820/jobs/{JobId}/status` | - | `AccountId`, `JobId`, `RequestedJobStatus` | - | `UpdateJobStatusResult` | `BadRequestException`, `InternalServiceException`, `JobStatusException`, `NotFoundException`, `TooManyRequestsException` | Updates the status for the specified job. Use this operation to confirm that you want to run a job or to cancel an existing job. For more information, see S3 Batch Operations in the Amazon S3 User Guide . Permissions ... |
| `UpdateStorageLensGroup` | `PUT /v20180820/storagelensgroup/{Name}` | - | `Name`, `AccountId`, `StorageLensGroup` | - | `Unit` | - | Updates the existing Storage Lens group. To use this operation, you must have the permission to perform the s3:UpdateStorageLensGroup action. For more information about the required Storage Lens Groups permissions, s ... |

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
| `BadRequestException` | `structure` | Message | - |
| `BucketAlreadyExists` | `structure` | **empty (no members)** | The requested Outposts bucket name is not available. The bucket namespace is shared by all users of the Outposts in this Region. Select a different name and ... |
| `BucketAlreadyOwnedByYou` | `structure` | **empty (no members)** | The Outposts bucket you tried to create already exists, and you own it. |
| `IdempotencyException` | `structure` | Message | - |
| `InternalServiceException` | `structure` | Message | - |
| `InvalidNextTokenException` | `structure` | Message | - |
| `InvalidRequestException` | `structure` | Message | - |
| `JobStatusException` | `structure` | Message | - |
| `NoSuchPublicAccessBlockConfiguration` | `structure` | Message | Amazon S3 throws this exception if you make a GetPublicAccessBlock request against an account that doesn't have a PublicAccessBlockConfiguration set. |
| `NotFoundException` | `structure` | Message | - |
| `TooManyRequestsException` | `structure` | Message | - |
| `TooManyTagsException` | `structure` | Message | Amazon S3 throws this exception if you have too many tags in your tag set. |
| `AssociateAccessGrantsIdentityCenterRequest` | `structure` | AccountId, IdentityCenterArn | - |
| `CreateAccessGrantRequest` | `structure` | AccountId, AccessGrantsLocationId, AccessGrantsLocationConfiguration, Grantee, Permission, ApplicationArn, S3PrefixType, Tags | - |
| `CreateAccessGrantResult` | `structure` | CreatedAt, AccessGrantId, AccessGrantArn, Grantee, AccessGrantsLocationId, AccessGrantsLocationConfiguration, Permission, ApplicationArn, GrantScope | - |
| `CreateAccessGrantsInstanceRequest` | `structure` | AccountId, IdentityCenterArn, Tags | - |
| `CreateAccessGrantsInstanceResult` | `structure` | CreatedAt, AccessGrantsInstanceId, AccessGrantsInstanceArn, IdentityCenterArn, IdentityCenterInstanceArn, IdentityCenterApplicationArn | - |
| `CreateAccessGrantsLocationRequest` | `structure` | AccountId, LocationScope, IAMRoleArn, Tags | - |
| `CreateAccessGrantsLocationResult` | `structure` | CreatedAt, AccessGrantsLocationId, AccessGrantsLocationArn, LocationScope, IAMRoleArn | - |
| `CreateAccessPointRequest` | `structure` | AccountId, Name, Bucket, VpcConfiguration, PublicAccessBlockConfiguration, BucketAccountId, Scope, Tags | - |
| `CreateAccessPointResult` | `structure` | AccessPointArn, Alias | - |
| `CreateAccessPointForObjectLambdaRequest` | `structure` | AccountId, Name, Configuration | - |
| `CreateAccessPointForObjectLambdaResult` | `structure` | ObjectLambdaAccessPointArn, Alias | - |
| `CreateBucketRequest` | `structure` | ACL, Bucket, CreateBucketConfiguration, GrantFullControl, GrantRead, GrantReadACP, GrantWrite, GrantWriteACP, ObjectLockEnabledForBucket, OutpostId | - |
| `CreateBucketResult` | `structure` | Location, BucketArn | - |
| `CreateJobRequest` | `structure` | AccountId, ConfirmationRequired, Operation, Report, ClientRequestToken, Manifest, Description, Priority, RoleArn, Tags, ManifestGenerator | - |
| `CreateJobResult` | `structure` | JobId | - |
| `CreateMultiRegionAccessPointRequest` | `structure` | AccountId, ClientToken, Details | - |
| `CreateMultiRegionAccessPointResult` | `structure` | RequestTokenARN | - |
| `CreateStorageLensGroupRequest` | `structure` | AccountId, StorageLensGroup, Tags | - |
| `DeleteAccessGrantRequest` | `structure` | AccountId, AccessGrantId | - |
| `DeleteAccessGrantsInstanceRequest` | `structure` | AccountId | - |
| `DeleteAccessGrantsInstanceResourcePolicyRequest` | `structure` | AccountId | - |
| `DeleteAccessGrantsLocationRequest` | `structure` | AccountId, AccessGrantsLocationId | - |
| `DeleteAccessPointRequest` | `structure` | AccountId, Name | - |
| `DeleteAccessPointForObjectLambdaRequest` | `structure` | AccountId, Name | - |
| `DeleteAccessPointPolicyRequest` | `structure` | AccountId, Name | - |
| `DeleteAccessPointPolicyForObjectLambdaRequest` | `structure` | AccountId, Name | - |
| `DeleteAccessPointScopeRequest` | `structure` | AccountId, Name | - |
| `DeleteBucketRequest` | `structure` | AccountId, Bucket | - |
| `AsyncOperationName` | `enum` | CreateMultiRegionAccessPoint, DeleteMultiRegionAccessPoint, PutMultiRegionAccessPointPolicy | - |
| `BucketCannedACL` | `enum` | private, public_read, public_read_write, authenticated_read | - |
| `BucketLocationConstraint` | `enum` | EU, eu_west_1, us_west_1, us_west_2, ap_south_1, ap_southeast_1, ap_southeast_2, ap_northeast_1, sa_east_1, cn_north_1, eu_central_1 | - |
| `BucketVersioningStatus` | `enum` | Enabled, Suspended | - |
| `ComputeObjectChecksumAlgorithm` | `enum` | CRC32, CRC32C, CRC64NVME, MD5, SHA1, SHA256, SHA512, XXHASH64, XXHASH3, XXHASH128 | - |
| `ComputeObjectChecksumType` | `enum` | FULL_OBJECT, COMPOSITE | - |
| `DeleteMarkerReplicationStatus` | `enum` | Enabled, Disabled | - |
| `ExistingObjectReplicationStatus` | `enum` | Enabled, Disabled | - |
| `ExpirationStatus` | `enum` | Enabled, Disabled | - |
| `Format` | `enum` | CSV, Parquet | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
