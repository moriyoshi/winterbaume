# Redshift Serverless

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is an interface reference for Amazon Redshift Serverless. It contains documentation for one of the programming or command line interfaces you can use to manage Amazon Redshift Serverless. Amazon Redshift Serverless automatically provisions data warehouse capacity and intelligently scales the underlying resources based on workload demands. Amazon Redshift Serverless adjusts capacity in seconds to deliver consistently high performance and simplified operations for even the most demanding and volatile workloads. Amazon Redshift Serverless lets you focus on using your data to acquire new insights for your business and customers.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Redshift Serverless where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Redshift Serverless by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Redshift Serverless workflows in the local mock. Key resources include `CrossVpcEndpointResource`, `ManagedWorkgroupResource`, `NamespaceResource`, `RecoveryPointResource`, `ReservationResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetCredentials`, `GetCustomDomainAssociation`, `GetEndpointAccess`, `GetIdentityCenterAuthToken`, `ListCustomDomainAssociations`, `ListEndpointAccess`.

## Service Identity and Protocol

- AWS model slug: `redshift-serverless`
- AWS SDK for Rust slug: `redshiftserverless`
- Model version: `2021-04-21`
- Model file: `vendor/api-models-aws/models/redshift-serverless/service/2021-04-21/redshift-serverless-2021-04-21.json`
- SDK ID: `Redshift Serverless`
- Endpoint prefix: `-`
- ARN namespace: `redshift-serverless`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (15), `List` (15), `Create` (9), `Delete` (9), `Update` (9), `Restore` (4), `Convert` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCustomDomainAssociation`, `CreateEndpointAccess`, `CreateNamespace`, `CreateReservation`, `CreateScheduledAction`, `CreateSnapshot`, `CreateSnapshotCopyConfiguration`, `CreateUsageLimit`, `CreateWorkgroup`, `DeleteCustomDomainAssociation`, `DeleteEndpointAccess`, `DeleteNamespace`, `DeleteResourcePolicy`, `DeleteScheduledAction`, `DeleteSnapshot`, `DeleteSnapshotCopyConfiguration`, `DeleteUsageLimit`, `DeleteWorkgroup`, `PutResourcePolicy`, `RestoreFromRecoveryPoint`, ... (+14).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCredentials`, `GetCustomDomainAssociation`, `GetEndpointAccess`, `GetIdentityCenterAuthToken`, `GetNamespace`, `GetRecoveryPoint`, `GetReservation`, `GetReservationOffering`, `GetResourcePolicy`, `GetScheduledAction`, `GetSnapshot`, `GetTableRestoreStatus`, `GetTrack`, `GetUsageLimit`, `GetWorkgroup`, `ListCustomDomainAssociations`, `ListEndpointAccess`, `ListManagedWorkgroups`, `ListNamespaces`, `ListRecoveryPoints`, ... (+10).
- Pagination is modelled for 14 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 17 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 65 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Glue`, `EC2/VPC`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CrossVpcEndpointResource` | - | - | `CreateEndpointAccess`, `DeleteEndpointAccess`, `GetEndpointAccess`, `ListEndpointAccess`, `UpdateEndpointAccess` | - |
| `ManagedWorkgroupResource` | - | - | `ListManagedWorkgroups` | - |
| `NamespaceResource` | `namespaceName` | put: `CreateNamespace`; read: `GetNamespace`; update: `UpdateNamespace`; delete: `DeleteNamespace`; list: `ListNamespaces` | `UpdateLakehouseConfiguration` | - |
| `RecoveryPointResource` | - | - | `ConvertRecoveryPointToSnapshot`, `GetRecoveryPoint`, `ListRecoveryPoints`, `RestoreFromRecoveryPoint`, `RestoreTableFromRecoveryPoint` | - |
| `ReservationResource` | - | - | `CreateReservation`, `GetReservation`, `GetReservationOffering`, `ListReservationOfferings`, `ListReservations` | - |
| `ScheduledActionResource` | - | - | `CreateScheduledAction`, `DeleteScheduledAction`, `GetScheduledAction`, `ListScheduledActions`, `UpdateScheduledAction` | - |
| `SnapshotResource` | - | - | `CreateSnapshot`, `CreateSnapshotCopyConfiguration`, `DeleteSnapshot`, `DeleteSnapshotCopyConfiguration`, `GetSnapshot`, `GetTableRestoreStatus`, `ListSnapshotCopyConfigurations`, `ListSnapshots`, `ListTableRestoreStatus`, `RestoreFromSnapshot`, ... (+3) | - |
| `UsageLimitResource` | - | - | `CreateUsageLimit`, `DeleteUsageLimit`, `GetUsageLimit`, `ListUsageLimits`, `UpdateUsageLimit` | - |
| `WorkgroupResource` | `workgroupName` | put: `CreateWorkgroup`; read: `GetWorkgroup`; update: `UpdateWorkgroup`; delete: `DeleteWorkgroup`; list: `ListWorkgroups` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-workgroup-namespace.html
- https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-connecting.html
- https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-usage-considerations.html

Research outcomes:
- Redshift Serverless separates storage and compute into namespaces and workgroups.
- Namespaces contain databases, schemas, users, permissions, encryption settings, and snapshots.
- Workgroups contain compute and network settings and expose endpoints for client connections.
- Data API can connect to Redshift Serverless workgroups.
- Serverless usage has subnet, Availability Zone, IP address, maintenance, patching, and datasharing considerations.
- Snapshots can restore data into serverless namespaces, including from provisioned cluster snapshots in supported scenarios.

Parity implications:
- Model namespaces, workgroups, endpoints, snapshots, usage limits, network settings, encryption, and data API connectivity separately.
- Namespace and workgroup lifecycle should be independent but cross-referenced.
- Restore operations should create or populate namespace state asynchronously.

## Operation Groups

### Get

- Operations: `GetCredentials`, `GetCustomDomainAssociation`, `GetIdentityCenterAuthToken`, `GetResourcePolicy`, `GetTrack`
- Traits: `readonly` (2)
- Common required input members in this group: -

### List

- Operations: `ListCustomDomainAssociations`, `ListTagsForResource`, `ListTracks`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: -

### Delete

- Operations: `DeleteCustomDomainAssociation`, `DeleteResourcePolicy`
- Common required input members in this group: -

### Create

- Operations: `CreateCustomDomainAssociation`
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateCustomDomainAssociation`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCustomDomainAssociation` | `-` | - | `workgroupName`, `customDomainName`, `customDomainCertificateArn` | - | `CreateCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a custom domain association for Amazon Redshift Serverless. |
| `DeleteCustomDomainAssociation` | `-` | - | `workgroupName`, `customDomainName` | - | `DeleteCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a custom domain association for Amazon Redshift Serverless. |
| `DeleteResourcePolicy` | `-` | - | `resourceArn` | - | `DeleteResourcePolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified resource policy. |
| `GetCredentials` | `-` | - | - | - | `GetCredentialsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a database user name and temporary password with temporary authorization to log in to Amazon Redshift Serverless. By default, the temporary credentials expire in 900 seconds. You can optionally specify a dura ... |
| `GetCustomDomainAssociation` | `-` | - | `customDomainName`, `workgroupName` | - | `GetCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specific custom domain association. |
| `GetIdentityCenterAuthToken` | `-` | - | `workgroupNames` | - | `GetIdentityCenterAuthTokenResponse` | `AccessDeniedException`, `ConflictException`, `DryRunException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an Identity Center authentication token for accessing Amazon Redshift Serverless workgroups. The token provides secure access to data within the specified workgroups using Identity Center identity propagation ... |
| `GetResourcePolicy` | `-` | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a resource policy. |
| `GetTrack` | `-` | `readonly` | `trackName` | - | `GetTrackResponse` | `AccessDeniedException`, `ConflictException`, `DryRunException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the Redshift Serverless version for a specified track. |
| `ListCustomDomainAssociations` | `-` | `readonly`, `paginated` | - | - | `ListCustomDomainAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidPaginationException`, `ThrottlingException`, `ValidationException` | Lists custom domain associations for Amazon Redshift Serverless. |
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags assigned to a resource. |
| `ListTracks` | `-` | `readonly`, `paginated` | - | - | `ListTracksResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidPaginationException`, `ThrottlingException`, `ValidationException` | List the Amazon Redshift Serverless versions. |
| `PutResourcePolicy` | `-` | - | `resourceArn`, `policy` | - | `PutResourcePolicyResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates or updates a resource policy. Currently, you can use policies to share snapshots across Amazon Web Services accounts. |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Assigns one or more tags to a resource. |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or set of tags from a resource. |
| `UpdateCustomDomainAssociation` | `-` | - | `workgroupName`, `customDomainName`, `customDomainCertificateArn` | - | `UpdateCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an Amazon Redshift Serverless certificate associated with a custom domain. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListCustomDomainAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListTracks` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | code, message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | The submitted action has conflicts. |
| `DryRunException` | `structure` | message | This exception is thrown when the request was successful, but dry run was enabled so no action was taken. |
| `InsufficientCapacityException` | `structure` | message | There is an insufficient capacity to perform the action. |
| `InternalServerException` | `structure` | message | The request processing has failed because of an unknown error, exception or failure. |
| `InvalidPaginationException` | `structure` | message | The provided pagination token is invalid. |
| `Ipv6CidrBlockNotFoundException` | `structure` | message | There are no subnets in your VPC with associated IPv6 CIDR blocks. To use dual-stack mode, associate an IPv6 CIDR block with each subnet in your VPC. |
| `ResourceNotFoundException` | `structure` | message, resourceName | The resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | message | The service limit was exceeded. |
| `ThrottlingException` | `structure` | code, message | The request was denied due to request throttling. |
| `TooManyTagsException` | `structure` | message, resourceName | The request exceeded the number of tags allowed for a resource. |
| `ValidationException` | `structure` | message | The input failed to satisfy the constraints specified by an Amazon Web Services service. |
| `CreateCustomDomainAssociationRequest` | `structure` | workgroupName, customDomainName, customDomainCertificateArn | - |
| `CreateCustomDomainAssociationResponse` | `structure` | customDomainName, workgroupName, customDomainCertificateArn, customDomainCertificateExpiryTime | - |
| `DeleteCustomDomainAssociationRequest` | `structure` | workgroupName, customDomainName | - |
| `DeleteCustomDomainAssociationResponse` | `structure` | **empty (no members)** | - |
| `DeleteResourcePolicyRequest` | `structure` | resourceArn | - |
| `DeleteResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `GetCredentialsRequest` | `structure` | dbName, durationSeconds, workgroupName, customDomainName | - |
| `GetCredentialsResponse` | `structure` | dbUser, dbPassword, expiration, nextRefreshTime | - |
| `GetCustomDomainAssociationRequest` | `structure` | customDomainName, workgroupName | - |
| `GetCustomDomainAssociationResponse` | `structure` | customDomainName, workgroupName, customDomainCertificateArn, customDomainCertificateExpiryTime | - |
| `GetIdentityCenterAuthTokenRequest` | `structure` | workgroupNames | - |
| `GetIdentityCenterAuthTokenResponse` | `structure` | token, expirationTime | - |
| `GetResourcePolicyRequest` | `structure` | resourceArn | - |
| `GetResourcePolicyResponse` | `structure` | resourcePolicy | - |
| `GetTrackRequest` | `structure` | trackName | - |
| `GetTrackResponse` | `structure` | track | - |
| `ListCustomDomainAssociationsRequest` | `structure` | nextToken, maxResults, customDomainName, customDomainCertificateArn | - |
| `ListCustomDomainAssociationsResponse` | `structure` | nextToken, associations | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `ListTracksRequest` | `structure` | nextToken, maxResults | - |
| `ListTracksResponse` | `structure` | tracks, nextToken | - |
| `PutResourcePolicyRequest` | `structure` | resourceArn, policy | - |
| `PutResourcePolicyResponse` | `structure` | resourcePolicy | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `ManagedWorkgroupStatus` | `enum` | CREATING, DELETING, MODIFYING, AVAILABLE, NOT_AVAILABLE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
