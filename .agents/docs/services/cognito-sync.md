# Amazon Cognito Sync

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Cognito Sync Amazon Cognito Sync provides an AWS service and client library that enable cross-device syncing of application-related user data. High-level client libraries are available for both iOS and Android. You can use these libraries to persist data locally so that it's available even if the device is offline. Developer credentials don't need to be stored on the mobile device to access the service. You can use Amazon Cognito to obtain a normalized user ID and credentials.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Cognito Sync where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented Amazon Cognito Sync workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Describe`, `Get`, `List`, `Set`, `Bulk` operation families, including `DescribeDataset`, `DescribeIdentityPoolUsage`, `DescribeIdentityUsage`, `GetBulkPublishDetails`, `GetCognitoEvents`, `GetIdentityPoolConfiguration`.

## Service Identity and Protocol

- AWS model slug: `cognito-sync`
- AWS SDK for Rust slug: `cognitosync`
- Model version: `2014-06-30`
- Model file: `vendor/api-models-aws/models/cognito-sync/service/2014-06-30/cognito-sync-2014-06-30.json`
- SDK ID: `Cognito Sync`
- Endpoint prefix: `cognito-sync`
- ARN namespace: `cognito-sync`
- CloudFormation name: `CognitoSync`
- CloudTrail event source: `cognitosync.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (3), `Get` (3), `List` (3), `Set` (2), `Bulk` (1), `Delete` (1), `Register` (1), `Subscribe` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteDataset`, `RegisterDevice`, `SetCognitoEvents`, `SetIdentityPoolConfiguration`, `UpdateRecords`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDataset`, `DescribeIdentityPoolUsage`, `DescribeIdentityUsage`, `GetBulkPublishDetails`, `GetCognitoEvents`, `GetIdentityPoolConfiguration`, `ListDatasets`, `ListIdentityPoolUsage`, `ListRecords`.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `Lambda`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-sync.html
- https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-streams.html
- https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-events.html

Research outcomes:
- Amazon Cognito Sync synchronises application-related user data, such as preferences or game state, across mobile devices and the web.
- Client libraries cache data locally so applications can read and write data while offline, then synchronise when the device is online.
- Push sync can notify other devices that an update is available.
- Cognito Streams can publish each dataset change to a customer-owned Kinesis stream in real time.
- Streams configuration selects a Kinesis stream, an IAM role that Cognito can assume to publish, and an enabled or disabled stream status.
- Each stream record represents a single synchronisation and includes identity pool id, identity id, dataset name, operation, records with sync counts, and timestamps.
- For updates larger than the Kinesis 1 MB maximum payload size, Cognito includes a presigned S3 URL containing the full update.
- If the Kinesis stream is deleted or role trust no longer lets Cognito assume the role, Cognito turns off streams until the stream or role is fixed and streams are enabled again.
- Bulk publish sends existing identity-pool data to the configured stream. Cognito does not guarantee uniqueness during bulk publish, and duplicate updates can appear.
- Only one bulk publish can be in progress, and only one successful bulk publish request is allowed every 24 hours.

Parity implications:
- Model identity-pool usage, datasets, records, sync counts, local/server versioning, push sync, stream configuration, stream records, bulk publish state, and event triggers separately.
- Stream publication should be tied to successful dataset synchronisation and should degrade when stream or role configuration becomes invalid.
- Bulk publish should expose asynchronous state, duplicate-delivery possibility, and the 24-hour success window.

## Operation Groups

### Describe

- Operations: `DescribeDataset`, `DescribeIdentityPoolUsage`, `DescribeIdentityUsage`
- Common required input members in this group: `IdentityPoolId`, `IdentityId`

### Get

- Operations: `GetBulkPublishDetails`, `GetCognitoEvents`, `GetIdentityPoolConfiguration`
- Common required input members in this group: `IdentityPoolId`

### List

- Operations: `ListDatasets`, `ListIdentityPoolUsage`, `ListRecords`
- Common required input members in this group: `IdentityPoolId`, `IdentityId`

### Set

- Operations: `SetCognitoEvents`, `SetIdentityPoolConfiguration`
- Common required input members in this group: `IdentityPoolId`

### Bulk

- Operations: `BulkPublish`
- Common required input members in this group: -

### Delete

- Operations: `DeleteDataset`
- Common required input members in this group: -

### Register

- Operations: `RegisterDevice`
- Common required input members in this group: -

### Subscribe

- Operations: `SubscribeToDataset`
- Common required input members in this group: -

### Unsubscribe

- Operations: `UnsubscribeFromDataset`
- Common required input members in this group: -

### Update

- Operations: `UpdateRecords`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BulkPublish` | `POST /identitypools/{IdentityPoolId}/bulkpublish` | - | `IdentityPoolId` | - | `BulkPublishResponse` | `AlreadyStreamedException`, `DuplicateRequestException`, `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException` | Initiates a bulk publish of all existing datasets for an Identity Pool to the configured stream. Customers are limited to one successful bulk publish per 24 hours. Bulk publish is an asynchronous request, customers c ... |
| `DeleteDataset` | `DELETE /identitypools/{IdentityPoolId}/identities/{IdentityId}/datasets/{DatasetName}` | - | `IdentityPoolId`, `IdentityId`, `DatasetName` | - | `DeleteDatasetResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Deletes the specific dataset. The dataset will be deleted permanently, and the action can't be undone. Datasets that this dataset was merged with will no longer report the merge. Any subsequent operation on this data ... |
| `DescribeDataset` | `GET /identitypools/{IdentityPoolId}/identities/{IdentityId}/datasets/{DatasetName}` | - | `IdentityPoolId`, `IdentityId`, `DatasetName` | - | `DescribeDatasetResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets meta data about a dataset by identity and dataset name. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identi ... |
| `DescribeIdentityPoolUsage` | `GET /identitypools/{IdentityPoolId}` | - | `IdentityPoolId` | - | `DescribeIdentityPoolUsageResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets usage details (for example, data storage) about a particular identity pool. This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognit ... |
| `DescribeIdentityUsage` | `GET /identitypools/{IdentityPoolId}/identities/{IdentityId}` | - | `IdentityPoolId`, `IdentityId` | - | `DescribeIdentityUsageResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets usage information for an identity, including number of datasets and data usage. This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials. DescribeIdentity ... |
| `GetBulkPublishDetails` | `POST /identitypools/{IdentityPoolId}/getBulkPublishDetails` | - | `IdentityPoolId` | - | `GetBulkPublishDetailsResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException` | Get the status of the last BulkPublish operation for an identity pool. This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity. |
| `GetCognitoEvents` | `GET /identitypools/{IdentityPoolId}/events` | - | `IdentityPoolId` | - | `GetCognitoEventsResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the events and the corresponding Lambda functions associated with an identity pool. This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided b ... |
| `GetIdentityPoolConfiguration` | `GET /identitypools/{IdentityPoolId}/configuration` | - | `IdentityPoolId` | - | `GetIdentityPoolConfigurationResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the configuration settings of an identity pool. This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity. GetIdentityPoolC ... |
| `ListDatasets` | `GET /identitypools/{IdentityPoolId}/identities/{IdentityId}/datasets` | - | `IdentityPoolId`, `IdentityId` | - | `ListDatasetsResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `TooManyRequestsException` | Lists datasets for an identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data. ListDatasets can be ... |
| `ListIdentityPoolUsage` | `GET /identitypools` | - | - | - | `ListIdentityPoolUsageResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `TooManyRequestsException` | Gets a list of identity pools registered with Cognito. ListIdentityPoolUsage can only be called with developer credentials. You cannot make this API call with the temporary user credentials provided by Cognito Identi ... |
| `ListRecords` | `GET /identitypools/{IdentityPoolId}/identities/{IdentityId}/datasets/{DatasetName}/records` | - | `IdentityPoolId`, `IdentityId`, `DatasetName` | - | `ListRecordsResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `TooManyRequestsException` | Gets paginated records, optionally changed after a particular sync count for a dataset and identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this AP ... |
| `RegisterDevice` | `POST /identitypools/{IdentityPoolId}/identity/{IdentityId}/device` | - | `IdentityPoolId`, `IdentityId`, `Platform`, `Token` | - | `RegisterDeviceResponse` | `InternalErrorException`, `InvalidConfigurationException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Registers a device to receive push sync notifications. This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials. RegisterDevice The foll ... |
| `SetCognitoEvents` | `POST /identitypools/{IdentityPoolId}/events` | - | `IdentityPoolId`, `Events` | - | `Unit` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Sets the AWS Lambda function for a given event type for an identity pool. This request only updates the key/value pair specified. Other key/values pairs are not updated. To remove a key value pair, pass a empty value ... |
| `SetIdentityPoolConfiguration` | `POST /identitypools/{IdentityPoolId}/configuration` | - | `IdentityPoolId` | - | `SetIdentityPoolConfigurationResponse` | `ConcurrentModificationException`, `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Sets the necessary configuration for push sync. This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity. SetIdentityPoolConfig ... |
| `SubscribeToDataset` | `POST /identitypools/{IdentityPoolId}/identities/{IdentityId}/datasets/{DatasetName}/subscriptions/{DeviceId}` | - | `IdentityPoolId`, `IdentityId`, `DatasetName`, `DeviceId` | - | `SubscribeToDatasetResponse` | `InternalErrorException`, `InvalidConfigurationException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Subscribes to receive notifications when a dataset is modified by another device. This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentia ... |
| `UnsubscribeFromDataset` | `DELETE /identitypools/{IdentityPoolId}/identities/{IdentityId}/datasets/{DatasetName}/subscriptions/{DeviceId}` | - | `IdentityPoolId`, `IdentityId`, `DatasetName`, `DeviceId` | - | `UnsubscribeFromDatasetResponse` | `InternalErrorException`, `InvalidConfigurationException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Unsubscribes from receiving notifications when a dataset is modified by another device. This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer cre ... |
| `UpdateRecords` | `POST /identitypools/{IdentityPoolId}/identities/{IdentityId}/datasets/{DatasetName}` | - | `IdentityPoolId`, `IdentityId`, `DatasetName`, `SyncSessionToken` | - | `UpdateRecordsResponse` | `InternalErrorException`, `InvalidLambdaFunctionOutputException`, `InvalidParameterException`, `LambdaThrottledException`, `LimitExceededException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Posts updates to records and adds and deletes records for a dataset and user. The sync count in the record patch is your last known sync count for that record. The server will reject an UpdateRecords request with a R ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListDatasets` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListIdentityPoolUsage` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListRecords` | - | `LastSyncCount -> lastSyncCount`, `NextToken -> nextToken`, `MaxResults -> maxResults`, `SyncSessionToken -> syncSessionToken` | - | - |
| `UpdateRecords` | `ClientContext -> x-amz-Client-Context` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AlreadyStreamedException` | `structure` | message | An exception thrown when a bulk publish operation is requested less than 24 hours after a previous bulk publish operation completed successfully. |
| `ConcurrentModificationException` | `structure` | message | Thrown if there are parallel requests to modify a resource. |
| `DuplicateRequestException` | `structure` | message | An exception thrown when there is an IN_PROGRESS bulk publish operation for the given identity pool. |
| `InternalErrorException` | `structure` | message | Indicates an internal service error. |
| `InvalidConfigurationException` | `structure` | message | - |
| `InvalidLambdaFunctionOutputException` | `structure` | message | The AWS Lambda function returned invalid output or an exception. |
| `InvalidParameterException` | `structure` | message | Thrown when a request parameter does not comply with the associated constraints. |
| `LambdaThrottledException` | `structure` | message | AWS Lambda throttled your account, please contact AWS Support |
| `LimitExceededException` | `structure` | message | Thrown when the limit on the number of objects or operations has been exceeded. |
| `NotAuthorizedException` | `structure` | message | Thrown when a user is not authorized to access the requested resource. |
| `ResourceConflictException` | `structure` | message | Thrown if an update can't be applied because the resource was changed by another call and this would result in a conflict. |
| `ResourceNotFoundException` | `structure` | message | Thrown if the resource doesn't exist. |
| `TooManyRequestsException` | `structure` | message | Thrown if the request is throttled. |
| `BulkPublishRequest` | `structure` | IdentityPoolId | The input for the BulkPublish operation. |
| `BulkPublishResponse` | `structure` | IdentityPoolId | The output for the BulkPublish operation. |
| `DeleteDatasetRequest` | `structure` | IdentityPoolId, IdentityId, DatasetName | A request to delete the specific dataset. |
| `DeleteDatasetResponse` | `structure` | Dataset | Response to a successful DeleteDataset request. |
| `DescribeDatasetRequest` | `structure` | IdentityPoolId, IdentityId, DatasetName | A request for meta data about a dataset (creation date, number of records, size) by owner and dataset name. |
| `DescribeDatasetResponse` | `structure` | Dataset | Response to a successful DescribeDataset request. |
| `DescribeIdentityPoolUsageRequest` | `structure` | IdentityPoolId | A request for usage information about the identity pool. |
| `DescribeIdentityPoolUsageResponse` | `structure` | IdentityPoolUsage | Response to a successful DescribeIdentityPoolUsage request. |
| `DescribeIdentityUsageRequest` | `structure` | IdentityPoolId, IdentityId | A request for information about the usage of an identity pool. |
| `DescribeIdentityUsageResponse` | `structure` | IdentityUsage | The response to a successful DescribeIdentityUsage request. |
| `GetBulkPublishDetailsRequest` | `structure` | IdentityPoolId | The input for the GetBulkPublishDetails operation. |
| `GetBulkPublishDetailsResponse` | `structure` | IdentityPoolId, BulkPublishStartTime, BulkPublishCompleteTime, BulkPublishStatus, FailureMessage | The output for the GetBulkPublishDetails operation. |
| `GetCognitoEventsRequest` | `structure` | IdentityPoolId | A request for a list of the configured Cognito Events |
| `GetCognitoEventsResponse` | `structure` | Events | The response from the GetCognitoEvents request |
| `GetIdentityPoolConfigurationRequest` | `structure` | IdentityPoolId | The input for the GetIdentityPoolConfiguration operation. |
| `GetIdentityPoolConfigurationResponse` | `structure` | IdentityPoolId, PushSync, CognitoStreams | The output for the GetIdentityPoolConfiguration operation. |
| `ListDatasetsRequest` | `structure` | IdentityPoolId, IdentityId, NextToken, MaxResults | Request for a list of datasets for an identity. |
| `ListDatasetsResponse` | `structure` | Datasets, Count, NextToken | Returned for a successful ListDatasets request. |
| `ListIdentityPoolUsageRequest` | `structure` | NextToken, MaxResults | A request for usage information on an identity pool. |
| `ListIdentityPoolUsageResponse` | `structure` | IdentityPoolUsages, MaxResults, Count, NextToken | Returned for a successful ListIdentityPoolUsage request. |
| `ListRecordsRequest` | `structure` | IdentityPoolId, IdentityId, DatasetName, LastSyncCount, NextToken, MaxResults, SyncSessionToken | A request for a list of records. |
| `ListRecordsResponse` | `structure` | Records, NextToken, Count, DatasetSyncCount, LastModifiedBy, MergedDatasetNames, DatasetExists, DatasetDeletedAfterRequestedSyncCount, SyncSessionToken | Returned for a successful ListRecordsRequest. |
| `RegisterDeviceRequest` | `structure` | IdentityPoolId, IdentityId, Platform, Token | A request to RegisterDevice. |
| `RegisterDeviceResponse` | `structure` | DeviceId | Response to a RegisterDevice request. |
| `SetCognitoEventsRequest` | `structure` | IdentityPoolId, Events | A request to configure Cognito Events" " |
| `SetIdentityPoolConfigurationRequest` | `structure` | IdentityPoolId, PushSync, CognitoStreams | The input for the SetIdentityPoolConfiguration operation. |
| `SetIdentityPoolConfigurationResponse` | `structure` | IdentityPoolId, PushSync, CognitoStreams | The output for the SetIdentityPoolConfiguration operation |
| `BulkPublishStatus` | `enum` | NOT_STARTED, IN_PROGRESS, FAILED, SUCCEEDED | - |
| `Operation` | `enum` | replace, remove | - |
| `Platform` | `enum` | APNS, APNS_SANDBOX, GCM, ADM | - |
| `StreamingStatus` | `enum` | ENABLED, DISABLED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
