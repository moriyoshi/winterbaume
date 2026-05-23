# Amazon Import/Export Snowball

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Web Services Snow Family provides a petabyte-scale data transport solution that uses secure devices to transfer large amounts of data between your on-premises data centers and Amazon Simple Storage Service (Amazon S3). The Snow Family commands described here provide access to the same functionality that is available in the Amazon Web Services Snow Family Management Console, which enables you to create and manage jobs for a Snow Family device. To transfer data locally with a Snow Family device, you'll need to use the Snowball Edge client or the Amazon S3 API Interface for Snowball or OpsHub for Snow Family. For more information, see the User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Import/Export Snowball resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Import/Export Snowball workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Describe`, `Get`, `Update` operation families, including `ListClusterJobs`, `ListClusters`, `ListCompatibleImages`, `ListJobs`, `CreateAddress`, `CreateCluster`.

## Service Identity and Protocol

- AWS model slug: `snowball`
- AWS SDK for Rust slug: `snowball`
- Model version: `2016-06-30`
- Model file: `vendor/api-models-aws/models/snowball/service/2016-06-30/snowball-2016-06-30.json`
- SDK ID: `Snowball`
- Endpoint prefix: `snowball`
- ARN namespace: `snowball`
- CloudFormation name: `Snowball`
- CloudTrail event source: `snowball.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Create` (5), `Describe` (5), `Get` (4), `Update` (4), `Cancel` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelCluster`, `CancelJob`, `CreateAddress`, `CreateCluster`, `CreateJob`, `CreateLongTermPricing`, `CreateReturnShippingLabel`, `UpdateCluster`, `UpdateJob`, `UpdateJobShipmentState`, `UpdateLongTermPricing`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAddress`, `DescribeAddresses`, `DescribeCluster`, `DescribeJob`, `DescribeReturnShippingLabel`, `GetJobManifest`, `GetJobUnlockCode`, `GetSnowballUsage`, `GetSoftwareUpdates`, `ListClusterJobs`, `ListClusters`, `ListCompatibleImages`, `ListJobs`, `ListLongTermPricing`, `ListPickupLocations`, `ListServiceVersions`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelCluster`, `CancelJob`, `CreateJob`, `DescribeJob`, `GetJobManifest`, `GetJobUnlockCode`, `ListClusterJobs`, `ListJobs`, `UpdateJob`, `UpdateJobShipmentState`.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `SNS`, `Lambda`, `EC2/VPC`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListClusterJobs`, `ListClusters`, `ListCompatibleImages`, `ListJobs`, `ListLongTermPricing`, `ListPickupLocations`, `ListServiceVersions`
- Traits: `paginated` (6)
- Common required input members in this group: -

### Create

- Operations: `CreateAddress`, `CreateCluster`, `CreateJob`, `CreateLongTermPricing`, `CreateReturnShippingLabel`
- Common required input members in this group: `SnowballType`

### Describe

- Operations: `DescribeAddress`, `DescribeAddresses`, `DescribeCluster`, `DescribeJob`, `DescribeReturnShippingLabel`
- Traits: `paginated` (1)
- Common required input members in this group: `JobId`

### Get

- Operations: `GetJobManifest`, `GetJobUnlockCode`, `GetSnowballUsage`, `GetSoftwareUpdates`
- Common required input members in this group: `JobId`

### Update

- Operations: `UpdateCluster`, `UpdateJob`, `UpdateJobShipmentState`, `UpdateLongTermPricing`
- Common required input members in this group: `JobId`

### Cancel

- Operations: `CancelCluster`, `CancelJob`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelCluster` | `-` | - | `ClusterId` | - | `CancelClusterResult` | `InvalidJobStateException`, `InvalidResourceException`, `KMSRequestFailedException` | Cancels a cluster job. You can only cancel a cluster job while it's in the AwaitingQuorum status. You'll have at least an hour after creating a cluster job to cancel it. |
| `CancelJob` | `-` | - | `JobId` | - | `CancelJobResult` | `InvalidJobStateException`, `InvalidResourceException`, `KMSRequestFailedException` | Cancels the specified job. You can only cancel a job before its JobState value changes to PreparingAppliance . Requesting the ListJobs or DescribeJob action returns a job's JobState as part of the response element da ... |
| `CreateAddress` | `-` | - | `Address` | - | `CreateAddressResult` | `InvalidAddressException`, `UnsupportedAddressException` | Creates an address for a Snow device to be shipped to. In most regions, addresses are validated at the time of creation. The address you provide must be located within the serviceable area of your region. If the addr ... |
| `CreateCluster` | `-` | - | `JobType`, `AddressId`, `SnowballType`, `ShippingOption` | - | `CreateClusterResult` | `Ec2RequestFailedException`, `InvalidInputCombinationException`, `InvalidResourceException`, `KMSRequestFailedException` | Creates an empty cluster. Each cluster supports five nodes. You use the CreateJob action separately to create the jobs for each of these nodes. The cluster does not ship until these five node jobs have been created. |
| `CreateJob` | `-` | - | - | - | `CreateJobResult` | `ClusterLimitExceededException`, `Ec2RequestFailedException`, `InvalidInputCombinationException`, `InvalidResourceException`, `KMSRequestFailedException` | Creates a job to import or export data between Amazon S3 and your on-premises data center. Your Amazon Web Services account must have the right trust policies and permissions in place to create a job for a Snow devic ... |
| `CreateLongTermPricing` | `-` | - | `LongTermPricingType`, `SnowballType` | - | `CreateLongTermPricingResult` | `InvalidResourceException` | Creates a job with the long-term usage option for a device. The long-term usage is a 1-year or 3-year long-term pricing type for the device. You are billed upfront, and Amazon Web Services provides discounts for long ... |
| `CreateReturnShippingLabel` | `-` | - | `JobId` | - | `CreateReturnShippingLabelResult` | `ConflictException`, `InvalidInputCombinationException`, `InvalidJobStateException`, `InvalidResourceException`, `ReturnShippingLabelAlreadyExistsException` | Creates a shipping label that will be used to return the Snow device to Amazon Web Services. |
| `DescribeAddress` | `-` | - | `AddressId` | - | `DescribeAddressResult` | `InvalidResourceException` | Takes an AddressId and returns specific details about that address in the form of an Address object. |
| `DescribeAddresses` | `-` | `paginated` | - | - | `DescribeAddressesResult` | `InvalidNextTokenException`, `InvalidResourceException` | Returns a specified number of ADDRESS objects. Calling this API in one of the US regions will return addresses from the list of all addresses associated with this account in all US regions. |
| `DescribeCluster` | `-` | - | `ClusterId` | - | `DescribeClusterResult` | `InvalidResourceException` | Returns information about a specific cluster including shipping information, cluster status, and other important metadata. |
| `DescribeJob` | `-` | - | `JobId` | - | `DescribeJobResult` | `InvalidResourceException` | Returns information about a specific job including shipping information, job status, and other important metadata. |
| `DescribeReturnShippingLabel` | `-` | - | `JobId` | - | `DescribeReturnShippingLabelResult` | `ConflictException`, `InvalidJobStateException`, `InvalidResourceException` | Information on the shipping label of a Snow device that is being returned to Amazon Web Services. |
| `GetJobManifest` | `-` | - | `JobId` | - | `GetJobManifestResult` | `InvalidJobStateException`, `InvalidResourceException` | Returns a link to an Amazon S3 presigned URL for the manifest file associated with the specified JobId value. You can access the manifest file for up to 60 minutes after this request has been made. To access the mani ... |
| `GetJobUnlockCode` | `-` | - | `JobId` | - | `GetJobUnlockCodeResult` | `InvalidJobStateException`, `InvalidResourceException` | Returns the UnlockCode code value for the specified job. A particular UnlockCode value can be accessed for up to 360 days after the associated job has been created. The UnlockCode value is a 29-character code with 25 ... |
| `GetSnowballUsage` | `-` | - | - | - | `GetSnowballUsageResult` | - | Returns information about the Snow Family service limit for your account, and also the number of Snow devices your account has in use. The default service limit for the number of Snow devices that you can have at one ... |
| `GetSoftwareUpdates` | `-` | - | `JobId` | - | `GetSoftwareUpdatesResult` | `InvalidJobStateException`, `InvalidResourceException` | Returns an Amazon S3 presigned URL for an update file associated with a specified JobId . |
| `ListClusterJobs` | `-` | `paginated` | `ClusterId` | - | `ListClusterJobsResult` | `InvalidNextTokenException`, `InvalidResourceException` | Returns an array of JobListEntry objects of the specified length. Each JobListEntry object is for a job in the specified cluster and contains a job's state, a job's ID, and other information. |
| `ListClusters` | `-` | `paginated` | - | - | `ListClustersResult` | `InvalidNextTokenException` | Returns an array of ClusterListEntry objects of the specified length. Each ClusterListEntry object contains a cluster's state, a cluster's ID, and other important status information. |
| `ListCompatibleImages` | `-` | `paginated` | - | - | `ListCompatibleImagesResult` | `Ec2RequestFailedException`, `InvalidNextTokenException` | This action returns a list of the different Amazon EC2-compatible Amazon Machine Images (AMIs) that are owned by your Amazon Web Services accountthat would be supported for use on a Snow device. Currently, supported ... |
| `ListJobs` | `-` | `paginated` | - | - | `ListJobsResult` | `InvalidNextTokenException` | Returns an array of JobListEntry objects of the specified length. Each JobListEntry object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. Cal ... |
| `ListLongTermPricing` | `-` | `paginated` | - | - | `ListLongTermPricingResult` | `InvalidNextTokenException`, `InvalidResourceException` | Lists all long-term pricing types. |
| `ListPickupLocations` | `-` | `paginated` | - | - | `ListPickupLocationsResult` | `InvalidResourceException` | A list of locations from which the customer can choose to pickup a device. |
| `ListServiceVersions` | `-` | - | `ServiceName` | - | `ListServiceVersionsResult` | `InvalidNextTokenException`, `InvalidResourceException` | Lists all supported versions for Snow on-device services. Returns an array of ServiceVersion object containing the supported versions for a particular service. |
| `UpdateCluster` | `-` | - | `ClusterId` | - | `UpdateClusterResult` | `Ec2RequestFailedException`, `InvalidInputCombinationException`, `InvalidJobStateException`, `InvalidResourceException`, `KMSRequestFailedException` | While a cluster's ClusterState value is in the AwaitingQuorum state, you can update some of the information associated with a cluster. Once the cluster changes to a different job state, usually 60 minutes after the c ... |
| `UpdateJob` | `-` | - | `JobId` | - | `UpdateJobResult` | `ClusterLimitExceededException`, `Ec2RequestFailedException`, `InvalidInputCombinationException`, `InvalidJobStateException`, `InvalidResourceException`, `KMSRequestFailedException` | While a job's JobState value is New , you can update some of the information associated with a job. Once the job changes to a different job state, usually within 60 minutes of the job being created, this action is no ... |
| `UpdateJobShipmentState` | `-` | - | `JobId`, `ShipmentState` | - | `UpdateJobShipmentStateResult` | `InvalidJobStateException`, `InvalidResourceException` | Updates the state when a shipment state changes to a different state. |
| `UpdateLongTermPricing` | `-` | - | `LongTermPricingId` | - | `UpdateLongTermPricingResult` | `InvalidResourceException` | Updates the long-term pricing type. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClusterLimitExceededException` | `structure` | Message | Job creation failed. Currently, clusters support five nodes. If you have fewer than five nodes for your cluster and you have more nodes to create for this c ... |
| `ConflictException` | `structure` | ConflictResource, Message | You get this exception when you call CreateReturnShippingLabel more than once when other requests are not completed. |
| `Ec2RequestFailedException` | `structure` | Message | Your user lacks the necessary Amazon EC2 permissions to perform the attempted action. |
| `InvalidAddressException` | `structure` | Message | The address provided was invalid. Check the address with your region's carrier, and try again. |
| `InvalidInputCombinationException` | `structure` | Message | Job or cluster creation failed. One or more inputs were invalid. Confirm that the CreateClusterRequest$SnowballType value supports your CreateJobRequest$Job ... |
| `InvalidJobStateException` | `structure` | Message | The action can't be performed because the job's current state doesn't allow that action to be performed. |
| `InvalidNextTokenException` | `structure` | Message | The NextToken string was altered unexpectedly, and the operation has stopped. Run the operation without changing the NextToken string, and try again. |
| `InvalidResourceException` | `structure` | Message, ResourceType | The specified resource can't be found. Check the information you provided in your last request, and try again. |
| `KMSRequestFailedException` | `structure` | Message | The provided Key Management Service key lacks the permissions to perform the specified CreateJob or UpdateJob action. |
| `ReturnShippingLabelAlreadyExistsException` | `structure` | Message | You get this exception if you call CreateReturnShippingLabel and a valid return shipping label already exists. In this case, use DescribeReturnShippingLabel ... |
| `UnsupportedAddressException` | `structure` | Message | The address is either outside the serviceable area for your region, or an error occurred. Check the address with your region's carrier and try again. If the ... |
| `CancelClusterRequest` | `structure` | ClusterId | - |
| `CancelClusterResult` | `structure` | **empty (no members)** | - |
| `CancelJobRequest` | `structure` | JobId | - |
| `CancelJobResult` | `structure` | **empty (no members)** | - |
| `CreateAddressRequest` | `structure` | Address | - |
| `CreateAddressResult` | `structure` | AddressId | - |
| `CreateClusterRequest` | `structure` | JobType, Resources, OnDeviceServiceConfiguration, Description, AddressId, KmsKeyARN, RoleARN, SnowballType, ShippingOption, Notification, ForwardingAddressId, TaxDocuments, ... (+5) | - |
| `CreateClusterResult` | `structure` | ClusterId, JobListEntries | - |
| `CreateJobRequest` | `structure` | JobType, Resources, OnDeviceServiceConfiguration, Description, AddressId, KmsKeyARN, RoleARN, SnowballCapacityPreference, ShippingOption, Notification, ClusterId, SnowballType, ... (+7) | - |
| `CreateJobResult` | `structure` | JobId | - |
| `CreateLongTermPricingRequest` | `structure` | LongTermPricingType, IsLongTermPricingAutoRenew, SnowballType | - |
| `CreateLongTermPricingResult` | `structure` | LongTermPricingId | - |
| `CreateReturnShippingLabelRequest` | `structure` | JobId, ShippingOption | - |
| `CreateReturnShippingLabelResult` | `structure` | Status | - |
| `DescribeAddressRequest` | `structure` | AddressId | - |
| `DescribeAddressResult` | `structure` | Address | - |
| `DescribeAddressesRequest` | `structure` | MaxResults, NextToken | - |
| `DescribeAddressesResult` | `structure` | Addresses, NextToken | - |
| `DescribeClusterRequest` | `structure` | ClusterId | - |
| `DescribeClusterResult` | `structure` | ClusterMetadata | - |
| `DescribeJobRequest` | `structure` | JobId | - |
| `DescribeJobResult` | `structure` | JobMetadata, SubJobMetadata | - |
| `DescribeReturnShippingLabelRequest` | `structure` | JobId | - |
| `DescribeReturnShippingLabelResult` | `structure` | Status, ExpirationDate, ReturnShippingLabelURI | - |
| `GetJobManifestRequest` | `structure` | JobId | - |
| `GetJobManifestResult` | `structure` | ManifestURI | - |
| `GetJobUnlockCodeRequest` | `structure` | JobId | - |
| `GetJobUnlockCodeResult` | `structure` | UnlockCode | - |
| `GetSnowballUsageRequest` | `structure` | **empty (no members)** | - |
| `AddressType` | `enum` | CUST_PICKUP, AWS_SHIP | - |
| `ClusterState` | `enum` | AWAITING_QUORUM, PENDING, IN_USE, COMPLETE, CANCELLED | - |
| `DeviceServiceName` | `enum` | NFS_ON_DEVICE_SERVICE, S3_ON_DEVICE_SERVICE | - |
| `ImpactLevel` | `enum` | IL2, IL4, IL5, IL6, IL99 | - |
| `JobState` | `enum` | NEW, PREPARING_APPLIANCE, PREPARING_SHIPMENT, IN_TRANSIT_TO_CUSTOMER, WITH_CUSTOMER, IN_TRANSIT_TO_AWS, WITH_AWS_SORTING_FACILITY, WITH_AWS, IN_PROGRESS, COMPLETE, CANCELLED, LISTING, ... (+1) | - |
| `JobType` | `enum` | IMPORT, EXPORT, LOCAL_USE | - |
| `LongTermPricingType` | `enum` | ONE_YEAR, THREE_YEAR, ONE_MONTH | - |
| `RemoteManagement` | `enum` | INSTALLED_ONLY, INSTALLED_AUTOSTART, NOT_INSTALLED | - |
| `ServiceName` | `enum` | KUBERNETES, EKS_ANYWHERE | - |
| `ShipmentState` | `enum` | RECEIVED, RETURNED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
