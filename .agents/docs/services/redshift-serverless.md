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

- Operations: `GetCredentials`, `GetCustomDomainAssociation`, `GetEndpointAccess`, `GetIdentityCenterAuthToken`, `GetNamespace`, `GetRecoveryPoint`, `GetReservation`, `GetReservationOffering`, `GetResourcePolicy`, `GetScheduledAction`, `GetSnapshot`, `GetTableRestoreStatus`, `GetTrack`, `GetUsageLimit`, `GetWorkgroup`
- Traits: `readonly` (12)
- Common required input members in this group: `customDomainName`, `endpointName`, `namespaceName`, `offeringId`, `recoveryPointId`, `reservationId`, `resourceArn`, `scheduledActionName`, `tableRestoreRequestId`, `trackName`, `usageLimitId`, `workgroupName`, `workgroupNames`

### List

- Operations: `ListCustomDomainAssociations`, `ListEndpointAccess`, `ListManagedWorkgroups`, `ListNamespaces`, `ListRecoveryPoints`, `ListReservationOfferings`, `ListReservations`, `ListScheduledActions`, `ListSnapshotCopyConfigurations`, `ListSnapshots`, `ListTableRestoreStatus`, `ListTagsForResource`, `ListTracks`, `ListUsageLimits`, `ListWorkgroups`
- Traits: `paginated` (14), `readonly` (14)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreateCustomDomainAssociation`, `CreateEndpointAccess`, `CreateNamespace`, `CreateReservation`, `CreateScheduledAction`, `CreateSnapshot`, `CreateSnapshotCopyConfiguration`, `CreateUsageLimit`, `CreateWorkgroup`
- Traits: `idempotency-token` (1), `idempotent` (8)
- Common required input members in this group: `amount`, `capacity`, `customDomainCertificateArn`, `customDomainName`, `destinationRegion`, `endpointName`, `namespaceName`, `offeringId`, `resourceArn`, `roleArn`, `schedule`, `scheduledActionName`, `snapshotName`, `subnetIds`, `targetAction`, `usageType`, `workgroupName`

### Delete

- Operations: `DeleteCustomDomainAssociation`, `DeleteEndpointAccess`, `DeleteNamespace`, `DeleteResourcePolicy`, `DeleteScheduledAction`, `DeleteSnapshot`, `DeleteSnapshotCopyConfiguration`, `DeleteUsageLimit`, `DeleteWorkgroup`
- Traits: `idempotent` (7)
- Common required input members in this group: `customDomainName`, `endpointName`, `namespaceName`, `resourceArn`, `scheduledActionName`, `snapshotCopyConfigurationId`, `snapshotName`, `usageLimitId`, `workgroupName`

### Update

- Operations: `UpdateCustomDomainAssociation`, `UpdateEndpointAccess`, `UpdateLakehouseConfiguration`, `UpdateNamespace`, `UpdateScheduledAction`, `UpdateSnapshot`, `UpdateSnapshotCopyConfiguration`, `UpdateUsageLimit`, `UpdateWorkgroup`
- Traits: `idempotent` (1)
- Common required input members in this group: `customDomainCertificateArn`, `customDomainName`, `endpointName`, `namespaceName`, `scheduledActionName`, `snapshotCopyConfigurationId`, `snapshotName`, `usageLimitId`, `workgroupName`

### Restore

- Operations: `RestoreFromRecoveryPoint`, `RestoreFromSnapshot`, `RestoreTableFromRecoveryPoint`, `RestoreTableFromSnapshot`
- Traits: `idempotent` (1)
- Common required input members in this group: `namespaceName`, `newTableName`, `recoveryPointId`, `snapshotName`, `sourceDatabaseName`, `sourceTableName`, `workgroupName`

### Convert

- Operations: `ConvertRecoveryPointToSnapshot`
- Common required input members in this group: `recoveryPointId`, `snapshotName`

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: `policy`, `resourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ConvertRecoveryPointToSnapshot` | - | - | `recoveryPointId`, `snapshotName` | - | `ConvertRecoveryPointToSnapshotResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyTagsException`, `ValidationException` | Converts a recovery point to a snapshot. For more information about recovery points and snapshots, see Working with snapshots and recovery points. |
| `CreateCustomDomainAssociation` | - | - | `customDomainCertificateArn`, `customDomainName`, `workgroupName` | - | `CreateCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a custom domain association for Amazon Redshift Serverless. |
| `CreateEndpointAccess` | - | `idempotent` | `endpointName`, `subnetIds`, `workgroupName` | - | `CreateEndpointAccessResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an Amazon Redshift Serverless managed VPC endpoint. |
| `CreateNamespace` | - | `idempotent` | `namespaceName` | - | `CreateNamespaceResponse` | `ConflictException`, `InternalServerException`, `TooManyTagsException`, `ValidationException` | Creates a namespace in Amazon Redshift Serverless. |
| `CreateReservation` | - | `idempotent`, `idempotency-token` | `capacity`, `offeringId` | `clientToken` | `CreateReservationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Creates an Amazon Redshift Serverless reservation, which gives you the option to commit to a specified number of Redshift Processing Units (RPUs) for a year at a discount from Serverless on-demand (OD) rates. |
| `CreateScheduledAction` | - | `idempotent` | `namespaceName`, `roleArn`, `schedule`, `scheduledActionName`, `targetAction` | - | `CreateScheduledActionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a scheduled action. A scheduled action contains a schedule and an Amazon Redshift API action. |
| `CreateSnapshot` | - | `idempotent` | `namespaceName`, `snapshotName` | - | `CreateSnapshotResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyTagsException`, `ValidationException` | Creates a snapshot of all databases in a namespace. For more information about snapshots, see Working with snapshots and recovery points. |
| `CreateSnapshotCopyConfiguration` | - | `idempotent` | `destinationRegion`, `namespaceName` | - | `CreateSnapshotCopyConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a snapshot copy configuration that lets you copy snapshots to another Amazon Web Services Region. |
| `CreateUsageLimit` | - | `idempotent` | `amount`, `resourceArn`, `usageType` | - | `CreateUsageLimitResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a usage limit for a specified Amazon Redshift Serverless usage type. The usage limit is identified by the returned usage limit identifier. |
| `CreateWorkgroup` | - | `idempotent` | `namespaceName`, `workgroupName` | - | `CreateWorkgroupResponse` | `ConflictException`, `InsufficientCapacityException`, `InternalServerException`, `Ipv6CidrBlockNotFoundException`, `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Creates an workgroup in Amazon Redshift Serverless. VPC Block Public Access (BPA) enables you to block resources in VPCs and subnets that you own in a Region from reaching or being reached from the internet through internet gateways and egress-only internet... |
| `DeleteCustomDomainAssociation` | - | - | `customDomainName`, `workgroupName` | - | `DeleteCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a custom domain association for Amazon Redshift Serverless. |
| `DeleteEndpointAccess` | - | `idempotent` | `endpointName` | - | `DeleteEndpointAccessResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an Amazon Redshift Serverless managed VPC endpoint. |
| `DeleteNamespace` | - | `idempotent` | `namespaceName` | - | `DeleteNamespaceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a namespace from Amazon Redshift Serverless. Before you delete the namespace, you can create a final snapshot that has all of the data within the namespace. |
| `DeleteResourcePolicy` | - | - | `resourceArn` | - | `DeleteResourcePolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified resource policy. |
| `DeleteScheduledAction` | - | `idempotent` | `scheduledActionName` | - | `DeleteScheduledActionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a scheduled action. |
| `DeleteSnapshot` | - | `idempotent` | `snapshotName` | - | `DeleteSnapshotResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a snapshot from Amazon Redshift Serverless. |
| `DeleteSnapshotCopyConfiguration` | - | `idempotent` | `snapshotCopyConfigurationId` | - | `DeleteSnapshotCopyConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a snapshot copy configuration |
| `DeleteUsageLimit` | - | `idempotent` | `usageLimitId` | - | `DeleteUsageLimitResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a usage limit from Amazon Redshift Serverless. |
| `DeleteWorkgroup` | - | `idempotent` | `workgroupName` | - | `DeleteWorkgroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a workgroup. |
| `GetCredentials` | - | - | - | - | `GetCredentialsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a database user name and temporary password with temporary authorization to log in to Amazon Redshift Serverless. By default, the temporary credentials expire in 900 seconds. |
| `GetCustomDomainAssociation` | - | - | `customDomainName`, `workgroupName` | - | `GetCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specific custom domain association. |
| `GetEndpointAccess` | - | `readonly` | `endpointName` | - | `GetEndpointAccessResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information, such as the name, about a VPC endpoint. |
| `GetIdentityCenterAuthToken` | - | - | `workgroupNames` | - | `GetIdentityCenterAuthTokenResponse` | `AccessDeniedException`, `ConflictException`, `DryRunException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an Identity Center authentication token for accessing Amazon Redshift Serverless workgroups. The token provides secure access to data within the specified workgroups using Identity Center identity propagation. |
| `GetNamespace` | - | `readonly` | `namespaceName` | - | `GetNamespaceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a namespace in Amazon Redshift Serverless. |
| `GetRecoveryPoint` | - | `readonly` | `recoveryPointId` | - | `GetRecoveryPointResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a recovery point. |
| `GetReservation` | - | `readonly` | `reservationId` | - | `GetReservationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Redshift Serverless reservation. A reservation gives you the option to commit to a specified number of Redshift Processing Units (RPUs) for a year at a discount from Serverless on-demand (OD) rates. |
| `GetReservationOffering` | - | `readonly` | `offeringId` | - | `GetReservationOfferingResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the reservation offering. The offering determines the payment schedule for the reservation. |
| `GetResourcePolicy` | - | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a resource policy. |
| `GetScheduledAction` | - | `readonly` | `scheduledActionName` | - | `GetScheduledActionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a scheduled action. |
| `GetSnapshot` | - | `readonly` | - | - | `GetSnapshotResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a specific snapshot. |
| `GetTableRestoreStatus` | - | `readonly` | `tableRestoreRequestId` | - | `GetTableRestoreStatusResponse` | `ResourceNotFoundException`, `ValidationException` | Returns information about a `TableRestoreStatus` object. |
| `GetTrack` | - | `readonly` | `trackName` | - | `GetTrackResponse` | `AccessDeniedException`, `ConflictException`, `DryRunException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the Redshift Serverless version for a specified track. |
| `GetUsageLimit` | - | `readonly` | `usageLimitId` | - | `GetUsageLimitResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a usage limit. |
| `GetWorkgroup` | - | `readonly` | `workgroupName` | - | `GetWorkgroupResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a specific workgroup. |
| `ListCustomDomainAssociations` | - | `readonly`, `paginated` | - | - | `ListCustomDomainAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidPaginationException`, `ThrottlingException`, `ValidationException` | Lists custom domain associations for Amazon Redshift Serverless. |
| `ListEndpointAccess` | - | `readonly`, `paginated` | - | - | `ListEndpointAccessResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns an array of `EndpointAccess` objects and relevant information. |
| `ListManagedWorkgroups` | - | `paginated` | - | - | `ListManagedWorkgroupsResponse` | `AccessDeniedException`, `InternalServerException` | Returns information about a list of specified managed workgroups in your account. |
| `ListNamespaces` | - | `readonly`, `paginated` | - | - | `ListNamespacesResponse` | `InternalServerException`, `ValidationException` | Returns information about a list of specified namespaces. |
| `ListRecoveryPoints` | - | `readonly`, `paginated` | - | - | `ListRecoveryPointsResponse` | `InternalServerException`, `ValidationException` | Returns an array of recovery points. |
| `ListReservationOfferings` | - | `readonly`, `paginated` | - | - | `ListReservationOfferingsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the current reservation offerings in your account. |
| `ListReservations` | - | `readonly`, `paginated` | - | - | `ListReservationsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of Reservation objects. |
| `ListScheduledActions` | - | `readonly`, `paginated` | - | - | `ListScheduledActionsResponse` | `InternalServerException`, `InvalidPaginationException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of scheduled actions. You can use the flags to filter the list of returned scheduled actions. |
| `ListSnapshotCopyConfigurations` | - | `readonly`, `paginated` | - | - | `ListSnapshotCopyConfigurationsResponse` | `ConflictException`, `InternalServerException`, `InvalidPaginationException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of snapshot copy configurations. |
| `ListSnapshots` | - | `readonly`, `paginated` | - | - | `ListSnapshotsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of snapshots. |
| `ListTableRestoreStatus` | - | `readonly`, `paginated` | - | - | `ListTableRestoreStatusResponse` | `InvalidPaginationException`, `ResourceNotFoundException`, `ValidationException` | Returns information about an array of `TableRestoreStatus` objects. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags assigned to a resource. |
| `ListTracks` | - | `readonly`, `paginated` | - | - | `ListTracksResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidPaginationException`, `ThrottlingException`, `ValidationException` | List the Amazon Redshift Serverless versions. |
| `ListUsageLimits` | - | `readonly`, `paginated` | - | - | `ListUsageLimitsResponse` | `ConflictException`, `InternalServerException`, `InvalidPaginationException`, `ResourceNotFoundException`, `ValidationException` | Lists all usage limits within Amazon Redshift Serverless. |
| `ListWorkgroups` | - | `readonly`, `paginated` | - | - | `ListWorkgroupsResponse` | `InternalServerException`, `ValidationException` | Returns information about a list of specified workgroups. |
| `PutResourcePolicy` | - | - | `policy`, `resourceArn` | - | `PutResourcePolicyResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates or updates a resource policy. Currently, you can use policies to share snapshots across Amazon Web Services accounts. |
| `RestoreFromRecoveryPoint` | - | - | `namespaceName`, `recoveryPointId`, `workgroupName` | - | `RestoreFromRecoveryPointResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Restore the data from a recovery point. |
| `RestoreFromSnapshot` | - | `idempotent` | `namespaceName`, `workgroupName` | - | `RestoreFromSnapshotResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Restores a namespace from a snapshot. |
| `RestoreTableFromRecoveryPoint` | - | - | `namespaceName`, `newTableName`, `recoveryPointId`, `sourceDatabaseName`, `sourceTableName`, `workgroupName` | - | `RestoreTableFromRecoveryPointResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Restores a table from a recovery point to your Amazon Redshift Serverless instance. You can't use this operation to restore tables with interleaved sort keys. |
| `RestoreTableFromSnapshot` | - | - | `namespaceName`, `newTableName`, `snapshotName`, `sourceDatabaseName`, `sourceTableName`, `workgroupName` | - | `RestoreTableFromSnapshotResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Restores a table from a snapshot to your Amazon Redshift Serverless instance. You can't use this operation to restore tables with interleaved sort keys. |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Assigns one or more tags to a resource. |
| `UntagResource` | - | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or set of tags from a resource. |
| `UpdateCustomDomainAssociation` | - | - | `customDomainCertificateArn`, `customDomainName`, `workgroupName` | - | `UpdateCustomDomainAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an Amazon Redshift Serverless certificate associated with a custom domain. |
| `UpdateEndpointAccess` | - | - | `endpointName` | - | `UpdateEndpointAccessResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates an Amazon Redshift Serverless managed endpoint. |
| `UpdateLakehouseConfiguration` | - | - | `namespaceName` | - | `UpdateLakehouseConfigurationResponse` | `ConflictException`, `DryRunException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Modifies the lakehouse configuration for a namespace. This operation allows you to manage Amazon Redshift federated permissions and Amazon Web Services IAM Identity Center trusted identity propagation. |
| `UpdateNamespace` | - | - | `namespaceName` | - | `UpdateNamespaceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a namespace with the specified settings. Unless required, you can't update multiple parameters in one request. |
| `UpdateScheduledAction` | - | `idempotent` | `scheduledActionName` | - | `UpdateScheduledActionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a scheduled action. |
| `UpdateSnapshot` | - | - | `snapshotName` | - | `UpdateSnapshotResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a snapshot. |
| `UpdateSnapshotCopyConfiguration` | - | - | `snapshotCopyConfigurationId` | - | `UpdateSnapshotCopyConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a snapshot copy configuration. |
| `UpdateUsageLimit` | - | - | `usageLimitId` | - | `UpdateUsageLimitResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Update a usage limit in Amazon Redshift Serverless. You can't update the usage type or period of a usage limit. |
| `UpdateWorkgroup` | - | - | `workgroupName` | - | `UpdateWorkgroupResponse` | `ConflictException`, `InsufficientCapacityException`, `InternalServerException`, `Ipv6CidrBlockNotFoundException`, `ResourceNotFoundException`, `ValidationException` | Updates a workgroup with the specified configuration settings. You can't update multiple parameters in one request. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListCustomDomainAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListTracks` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `message` | The input failed to satisfy the constraints specified by an Amazon Web Services service. |
| `InternalServerException` | `structure` | `message` | The request processing has failed because of an unknown error, exception or failure. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceName` | The resource could not be found. |
| `ConflictException` | `structure` | `message` | The submitted action has conflicts. |
| `ThrottlingException` | `structure` | `code`, `message` | The request was denied due to request throttling. |
| `AccessDeniedException` | `structure` | `code`, `message` | You do not have sufficient access to perform this action. |
| `ServiceQuotaExceededException` | `structure` | `message` | The service limit was exceeded. |
| `TooManyTagsException` | `structure` | `message`, `resourceName` | The request exceeded the number of tags allowed for a resource. |
| `InvalidPaginationException` | `structure` | `message` | The provided pagination token is invalid. |
| `DryRunException` | `structure` | `message` | This exception is thrown when the request was successful, but dry run was enabled so no action was taken. |
| `InsufficientCapacityException` | `structure` | `message` | There is an insufficient capacity to perform the action. |
| `Ipv6CidrBlockNotFoundException` | `structure` | `message` | There are no subnets in your VPC with associated IPv6 CIDR blocks. |
| `ConvertRecoveryPointToSnapshotRequest` | `structure` | `recoveryPointId`, `retentionPeriod`, `snapshotName`, `tags` | - |
| `ConvertRecoveryPointToSnapshotResponse` | `structure` | `snapshot` | - |
| `CreateCustomDomainAssociationRequest` | `structure` | `customDomainCertificateArn`, `customDomainName`, `workgroupName` | - |
| `CreateCustomDomainAssociationResponse` | `structure` | `customDomainCertificateArn`, `customDomainCertificateExpiryTime`, `customDomainName`, `workgroupName` | - |
| `CreateEndpointAccessRequest` | `structure` | `endpointName`, `ownerAccount`, `subnetIds`, `vpcSecurityGroupIds`, `workgroupName` | - |
| `CreateEndpointAccessResponse` | `structure` | `endpoint` | - |
| `CreateNamespaceRequest` | `structure` | `adminPasswordSecretKmsKeyId`, `adminUserPassword`, `adminUsername`, `dbName`, `defaultIamRoleArn`, `iamRoles`, `kmsKeyId`, `logExports`, `manageAdminPassword`, `namespaceName`, `redshiftIdcApplicationArn`, `tags` | - |
| `CreateNamespaceResponse` | `structure` | `namespace` | - |
| `CreateReservationRequest` | `structure` | `capacity`, `clientToken`, `offeringId` | - |
| `CreateReservationResponse` | `structure` | `reservation` | - |
| `CreateScheduledActionRequest` | `structure` | `enabled`, `endTime`, `namespaceName`, `roleArn`, `schedule`, `scheduledActionDescription`, `scheduledActionName`, `startTime`, `targetAction` | - |
| `CreateScheduledActionResponse` | `structure` | `scheduledAction` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
