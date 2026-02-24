# AWS Directory Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Directory Service Directory Service is a web service that makes it easy for you to setup and run directories in the Amazon Web Services cloud, or connect your Amazon Web Services resources with an existing self-managed Microsoft Active Directory. This guide provides detailed information about Directory Service operations, data types, parameters, and errors. For information about Directory Services features, see Directory Service and the Directory Service Administration Guide. Amazon Web Services provides SDKs that consist of libraries and sample code for various programming languages and platforms (Java, Ruby, .Net, iOS, Android, etc.). The SDKs provide a convenient way to create programmatic access to Directory Service and other Amazon Web Services services.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Directory Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Directory Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS Directory Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Directory Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Update`, `Delete`, `Disable` operation families, including `DescribeADAssessment`, `DescribeCAEnrollmentPolicy`, `DescribeCertificate`, `DescribeClientAuthenticationSettings`, `CreateAlias`, `CreateComputer`.

## Service Identity and Protocol

- AWS model slug: `directory-service`
- AWS SDK for Rust slug: `directory`
- Model version: `2015-04-16`
- Model file: `vendor/api-models-aws/models/directory-service/service/2015-04-16/directory-service-2015-04-16.json`
- SDK ID: `Directory Service`
- Endpoint prefix: `ds`
- ARN namespace: `ds`
- CloudFormation name: `DirectoryService`
- CloudTrail event source: `ds.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (17), `Create` (9), `Update` (7), `Delete` (6), `Disable` (6), `Enable` (6), `List` (6), `Add` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptSharedDirectory`, `AddIpRoutes`, `AddRegion`, `AddTagsToResource`, `CancelSchemaExtension`, `CreateAlias`, `CreateComputer`, `CreateConditionalForwarder`, `CreateDirectory`, `CreateHybridAD`, `CreateLogSubscription`, `CreateMicrosoftAD`, `CreateSnapshot`, `CreateTrust`, `DeleteADAssessment`, `DeleteConditionalForwarder`, `DeleteDirectory`, `DeleteLogSubscription`, `DeleteSnapshot`, `DeleteTrust`, ... (+30).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeADAssessment`, `DescribeCAEnrollmentPolicy`, `DescribeCertificate`, `DescribeClientAuthenticationSettings`, `DescribeConditionalForwarders`, `DescribeDirectories`, `DescribeDirectoryDataAccess`, `DescribeDomainControllers`, `DescribeEventTopics`, `DescribeHybridADUpdate`, `DescribeLDAPSSettings`, `DescribeRegions`, `DescribeSettings`, `DescribeSharedDirectories`, `DescribeSnapshots`, `DescribeTrusts`, `DescribeUpdateDirectory`, `GetDirectoryLimits`, `GetSnapshotLimits`, `ListADAssessments`, ... (+5).
- Pagination is modelled for 15 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelSchemaExtension`, `StartADAssessment`, `StartSchemaExtension`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 80 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `EC2/VPC`, `ECS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/directoryservice/latest/admin-guide/what_is.html
- https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_microsoft_ad.html
- https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_ad_connector.html

Research outcomes:
- Directory Service offers AWS Managed Microsoft AD, AD Connector, and Simple AD directory types.
- AWS Managed Microsoft AD deploys managed domain controllers and supports domain-joined AWS workloads and trust relationships.
- AD Connector is a proxy/gateway that redirects requests to an existing on-premises Active Directory rather than storing directory data itself.
- Directories are deployed in VPC subnets and have DNS, security, edition, and size settings.
- Directory lifecycle includes creation, sharing, snapshot/backup capabilities where supported, trust configuration, and deletion.
- Directory users/groups are directory data-plane concepts and are separate from AWS IAM users.

Parity implications:
- Model directories, directory type, VPC/subnet placement, DNS addresses, snapshots, trusts, shares, aliases, and status separately.
- AD Connector should not behave like a stored managed directory.
- Directory state should gate dependent integrations such as WorkSpaces, IAM Identity Center, and domain joins.

## Current Network Resource Stub Semantics

Directory Service stores directory networking settings as directory-local metadata.

- Microsoft AD creation records the supplied `VpcSettings.VpcId` and `SubnetIds`, then returns a `DirectoryVpcSettings` record with those raw IDs.
- AD Connector creation records the supplied connect settings VPC ID and subnet IDs in the same local directory state.
- The implementation synthesises directory security group IDs from the directory ID and derives availability-zone-looking values from subnet positions rather than querying EC2.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeADAssessment`, `DescribeCAEnrollmentPolicy`, `DescribeCertificate`, `DescribeClientAuthenticationSettings`, `DescribeConditionalForwarders`, `DescribeDirectories`, `DescribeDirectoryDataAccess`, `DescribeDomainControllers`, `DescribeEventTopics`, `DescribeHybridADUpdate`, `DescribeLDAPSSettings`, `DescribeRegions`, `DescribeSettings`, `DescribeSharedDirectories`, `DescribeSnapshots`, `DescribeTrusts`, `DescribeUpdateDirectory`
- Traits: `paginated` (9)
- Common required input members in this group: `AssessmentId`, `CertificateId`, `DirectoryId`, `OwnerDirectoryId`, `UpdateType`

### Create

- Operations: `CreateAlias`, `CreateComputer`, `CreateConditionalForwarder`, `CreateDirectory`, `CreateHybridAD`, `CreateLogSubscription`, `CreateMicrosoftAD`, `CreateSnapshot`, `CreateTrust`
- Common required input members in this group: `Alias`, `AssessmentId`, `ComputerName`, `DirectoryId`, `LogGroupName`, `Name`, `Password`, `RemoteDomainName`, `SecretArn`, `Size`, `TrustDirection`, `TrustPassword`, `VpcSettings`

### Update

- Operations: `UpdateConditionalForwarder`, `UpdateDirectorySetup`, `UpdateHybridAD`, `UpdateNumberOfDomainControllers`, `UpdateRadius`, `UpdateSettings`, `UpdateTrust`
- Common required input members in this group: `DesiredNumber`, `DirectoryId`, `RadiusSettings`, `RemoteDomainName`, `Settings`, `TrustId`, `UpdateType`

### Delete

- Operations: `DeleteADAssessment`, `DeleteConditionalForwarder`, `DeleteDirectory`, `DeleteLogSubscription`, `DeleteSnapshot`, `DeleteTrust`
- Common required input members in this group: `AssessmentId`, `DirectoryId`, `RemoteDomainName`, `SnapshotId`, `TrustId`

### Disable

- Operations: `DisableCAEnrollmentPolicy`, `DisableClientAuthentication`, `DisableDirectoryDataAccess`, `DisableLDAPS`, `DisableRadius`, `DisableSso`
- Common required input members in this group: `DirectoryId`, `Type`

### Enable

- Operations: `EnableCAEnrollmentPolicy`, `EnableClientAuthentication`, `EnableDirectoryDataAccess`, `EnableLDAPS`, `EnableRadius`, `EnableSso`
- Common required input members in this group: `DirectoryId`, `PcaConnectorArn`, `RadiusSettings`, `Type`

### List

- Operations: `ListADAssessments`, `ListCertificates`, `ListIpRoutes`, `ListLogSubscriptions`, `ListSchemaExtensions`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: `DirectoryId`, `ResourceId`

### Add

- Operations: `AddIpRoutes`, `AddRegion`, `AddTagsToResource`
- Common required input members in this group: `DirectoryId`, `IpRoutes`, `RegionName`, `ResourceId`, `Tags`, `VPCSettings`

### Remove

- Operations: `RemoveIpRoutes`, `RemoveRegion`, `RemoveTagsFromResource`
- Common required input members in this group: `DirectoryId`, `ResourceId`, `TagKeys`

### Deregister

- Operations: `DeregisterCertificate`, `DeregisterEventTopic`
- Common required input members in this group: `CertificateId`, `DirectoryId`, `TopicName`

### Get

- Operations: `GetDirectoryLimits`, `GetSnapshotLimits`
- Common required input members in this group: `DirectoryId`

### Register

- Operations: `RegisterCertificate`, `RegisterEventTopic`
- Common required input members in this group: `CertificateData`, `DirectoryId`, `TopicName`

### Start

- Operations: `StartADAssessment`, `StartSchemaExtension`
- Common required input members in this group: `CreateSnapshotBeforeSchemaExtension`, `Description`, `DirectoryId`, `LdifContent`

### Accept

- Operations: `AcceptSharedDirectory`
- Common required input members in this group: `SharedDirectoryId`

### Cancel

- Operations: `CancelSchemaExtension`
- Common required input members in this group: `DirectoryId`, `SchemaExtensionId`

### Connect

- Operations: `ConnectDirectory`
- Common required input members in this group: `ConnectSettings`, `Name`, `Password`, `Size`

### Reject

- Operations: `RejectSharedDirectory`
- Common required input members in this group: `SharedDirectoryId`

### Reset

- Operations: `ResetUserPassword`
- Common required input members in this group: `DirectoryId`, `NewPassword`, `UserName`

### Restore

- Operations: `RestoreFromSnapshot`
- Common required input members in this group: `SnapshotId`

### Share

- Operations: `ShareDirectory`
- Common required input members in this group: `DirectoryId`, `ShareMethod`, `ShareTarget`

### Unshare

- Operations: `UnshareDirectory`
- Common required input members in this group: `DirectoryId`, `UnshareTarget`

### Verify

- Operations: `VerifyTrust`
- Common required input members in this group: `TrustId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptSharedDirectory` | - | - | `SharedDirectoryId` | - | `AcceptSharedDirectoryResult` | `ClientException`, `DirectoryAlreadySharedException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Accepts a directory sharing request that was sent from the directory owner account. |
| `AddIpRoutes` | - | - | `DirectoryId`, `IpRoutes` | - | `AddIpRoutesResult` | `ClientException`, `DirectoryUnavailableException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InvalidParameterException`, `IpRouteLimitExceededException`, `ServiceException` | If the DNS server for your self-managed domain uses a publicly addressable IP address, you must add a CIDR address block to correctly route traffic to and from your Microsoft AD on Amazon Web Services. AddIpRoutes adds this address block. |
| `AddRegion` | - | - | `DirectoryId`, `RegionName`, `VPCSettings` | - | `AddRegionResult` | `AccessDeniedException`, `ClientException`, `DirectoryAlreadyInRegionException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `EntityDoesNotExistException`, `InvalidParameterException`, `RegionLimitExceededException`, ... (+2) | Adds two domain controllers in the specified Region for the specified directory. |
| `AddTagsToResource` | - | - | `ResourceId`, `Tags` | - | `AddTagsToResourceResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `TagLimitExceededException` | Adds or overwrites one or more tags for the specified directory. Each directory can have a maximum of 50 tags. |
| `CancelSchemaExtension` | - | - | `DirectoryId`, `SchemaExtensionId` | - | `CancelSchemaExtensionResult` | `ClientException`, `EntityDoesNotExistException`, `ServiceException` | Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. |
| `ConnectDirectory` | - | - | `ConnectSettings`, `Name`, `Password`, `Size` | - | `ConnectDirectoryResult` | `ClientException`, `DirectoryLimitExceededException`, `InvalidParameterException`, `ServiceException` | Creates an AD Connector to connect to a self-managed directory. Before you call `ConnectDirectory`, ensure that all of the required permissions have been explicitly granted through a policy. |
| `CreateAlias` | - | - | `Alias`, `DirectoryId` | - | `CreateAliasResult` | `ClientException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Creates an alias for a directory and assigns the alias to the directory. The alias is used to construct the access URL for the directory, such as `http:// .awsapps.com`. |
| `CreateComputer` | - | - | `ComputerName`, `DirectoryId`, `Password` | - | `CreateComputerResult` | `AuthenticationFailedException`, `ClientException`, `DirectoryUnavailableException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Creates an Active Directory computer object in the specified directory. |
| `CreateConditionalForwarder` | - | - | `DirectoryId`, `RemoteDomainName` | - | `CreateConditionalForwarderResult` | `ClientException`, `DirectoryUnavailableException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Creates a conditional forwarder associated with your Amazon Web Services directory. Conditional forwarders are required in order to set up a trust relationship with another domain. |
| `CreateDirectory` | - | - | `Name`, `Password`, `Size` | - | `CreateDirectoryResult` | `ClientException`, `DirectoryLimitExceededException`, `InvalidParameterException`, `ServiceException` | Creates a Simple AD directory. For more information, see Simple Active Directory in the Directory Service Admin Guide . |
| `CreateHybridAD` | - | - | `AssessmentId`, `SecretArn` | - | `CreateHybridADResult` | `ADAssessmentLimitExceededException`, `ClientException`, `DirectoryLimitExceededException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Creates a hybrid directory that connects your self-managed Active Directory (AD) infrastructure and Amazon Web Services. You must have a successful directory assessment using StartADAssessment to validate your environment compatibility before you use this... |
| `CreateLogSubscription` | - | - | `DirectoryId`, `LogGroupName` | - | `CreateLogSubscriptionResult` | `ClientException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InsufficientPermissionsException`, `ServiceException`, `UnsupportedOperationException` | Creates a subscription to forward real-time Directory Service domain controller security logs to the specified Amazon CloudWatch log group in your Amazon Web Services account. |
| `CreateMicrosoftAD` | - | - | `Name`, `Password`, `VpcSettings` | - | `CreateMicrosoftADResult` | `ClientException`, `DirectoryLimitExceededException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Creates a Microsoft AD directory in the Amazon Web Services Cloud. For more information, see Managed Microsoft AD in the Directory Service Admin Guide . |
| `CreateSnapshot` | - | - | `DirectoryId` | - | `CreateSnapshotResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `SnapshotLimitExceededException` | Creates a snapshot of a Simple AD or Microsoft AD directory in the Amazon Web Services cloud. You cannot take snapshots of AD Connector directories. |
| `CreateTrust` | - | - | `DirectoryId`, `RemoteDomainName`, `TrustDirection`, `TrustPassword` | - | `CreateTrustResult` | `ClientException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your Managed Microsoft AD directory, and your existing self-managed Microsoft Active Directory. |
| `DeleteADAssessment` | - | - | `AssessmentId` | - | `DeleteADAssessmentResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Deletes a directory assessment and all associated data. This operation permanently removes the assessment results, validation reports, and configuration information. |
| `DeleteConditionalForwarder` | - | - | `DirectoryId`, `RemoteDomainName` | - | `DeleteConditionalForwarderResult` | `ClientException`, `DirectoryUnavailableException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Deletes a conditional forwarder that has been set up for your Amazon Web Services directory. |
| `DeleteDirectory` | - | - | `DirectoryId` | - | `DeleteDirectoryResult` | `ClientException`, `EntityDoesNotExistException`, `ServiceException` | Deletes an Directory Service directory. Before you call `DeleteDirectory`, ensure that all of the required permissions have been explicitly granted through a policy. |
| `DeleteLogSubscription` | - | - | `DirectoryId` | - | `DeleteLogSubscriptionResult` | `ClientException`, `EntityDoesNotExistException`, `ServiceException`, `UnsupportedOperationException` | Deletes the specified log subscription. |
| `DeleteSnapshot` | - | - | `SnapshotId` | - | `DeleteSnapshotResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Deletes a directory snapshot. |
| `DeleteTrust` | - | - | `TrustId` | - | `DeleteTrustResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Deletes an existing trust relationship between your Managed Microsoft AD directory and an external domain. |
| `DeregisterCertificate` | - | - | `CertificateId`, `DirectoryId` | - | `DeregisterCertificateResult` | `CertificateDoesNotExistException`, `CertificateInUseException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Deletes from the system the certificate that was registered for secure LDAP or client certificate authentication. |
| `DeregisterEventTopic` | - | - | `DirectoryId`, `TopicName` | - | `DeregisterEventTopicResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Removes the specified directory as a publisher to the specified Amazon SNS topic. |
| `DescribeADAssessment` | - | - | `AssessmentId` | - | `DescribeADAssessmentResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Retrieves detailed information about a directory assessment, including its current status, validation results, and configuration details. Use this operation to monitor assessment progress and review results. |
| `DescribeCAEnrollmentPolicy` | - | - | `DirectoryId` | - | `DescribeCAEnrollmentPolicyResult` | `ClientException`, `DirectoryDoesNotExistException`, `ServiceException`, `UnsupportedOperationException` | Retrieves detailed information about the certificate authority (CA) enrollment policy for the specified directory. This policy determines how client certificates are automatically enrolled and managed through Amazon Web Services Private Certificate Authority. |
| `DescribeCertificate` | - | - | `CertificateId`, `DirectoryId` | - | `DescribeCertificateResult` | `CertificateDoesNotExistException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Displays information about the certificate registered for secure LDAP or client certificate authentication. |
| `DescribeClientAuthenticationSettings` | - | `paginated` | `DirectoryId` | - | `DescribeClientAuthenticationSettingsResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Retrieves information about the type of client authentication for the specified directory, if the type is specified. If no type is specified, information about all client authentication types that are supported for the specified directory is retrieved. |
| `DescribeConditionalForwarders` | - | - | `DirectoryId` | - | `DescribeConditionalForwardersResult` | `ClientException`, `DirectoryUnavailableException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Obtains information about the conditional forwarders for this account. If no input parameters are provided for RemoteDomainNames, this request describes all conditional forwarders for the specified directory ID. |
| `DescribeDirectories` | - | `paginated` | - | - | `DescribeDirectoriesResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException` | Obtains information about the directories that belong to this account. You can retrieve information about specific directories by passing the directory identifiers in the `DirectoryIds` parameter. |
| `DescribeDirectoryDataAccess` | - | - | `DirectoryId` | - | `DescribeDirectoryDataAccessResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `ServiceException`, `UnsupportedOperationException` | Obtains status of directory data access enablement through the Directory Service Data API for the specified directory. |
| `DescribeDomainControllers` | - | `paginated` | `DirectoryId` | - | `DescribeDomainControllersResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Provides information about any domain controllers in your directory. |
| `DescribeEventTopics` | - | - | - | - | `DescribeEventTopicsResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Obtains information about which Amazon SNS topics receive status messages from the specified directory. If no input parameters are provided, such as DirectoryId or TopicName, this request describes all of the associations in the account. |
| `DescribeHybridADUpdate` | - | - | `DirectoryId` | - | `DescribeHybridADUpdateResult` | `ClientException`, `DirectoryDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Retrieves information about update activities for a hybrid directory. This operation provides details about configuration changes, administrator account updates, and self-managed instance settings (IDs and DNS IPs). |
| `DescribeLDAPSSettings` | - | `paginated` | `DirectoryId` | - | `DescribeLDAPSSettingsResult` | `ClientException`, `DirectoryDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Describes the status of LDAP security for the specified directory. |
| `DescribeRegions` | - | `paginated` | `DirectoryId` | - | `DescribeRegionsResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Provides information about the Regions that are configured for multi-Region replication. |
| `DescribeSettings` | - | - | `DirectoryId` | - | `DescribeSettingsResult` | `ClientException`, `DirectoryDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Retrieves information about the configurable settings for the specified directory. |
| `DescribeSharedDirectories` | - | `paginated` | `OwnerDirectoryId` | - | `DescribeSharedDirectoriesResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Returns the shared directories in your account. |
| `DescribeSnapshots` | - | `paginated` | - | - | `DescribeSnapshotsResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException` | Obtains information about the directory snapshots that belong to this account. This operation supports pagination with the use of the NextToken request and response parameters. |
| `DescribeTrusts` | - | `paginated` | - | - | `DescribeTrustsResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Obtains information about the trust relationships for this account. If no input parameters are provided, such as DirectoryId or TrustIds, this request describes all the trust relationships belonging to the account. |
| `DescribeUpdateDirectory` | - | `paginated` | `DirectoryId`, `UpdateType` | - | `DescribeUpdateDirectoryResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException` | Describes the updates of a directory for a particular update type. |
| `DisableCAEnrollmentPolicy` | - | - | `DirectoryId` | - | `DisableCAEnrollmentPolicyResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `DisableAlreadyInProgressException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Disables the certificate authority (CA) enrollment policy for the specified directory. This stops automatic certificate enrollment and management for domain-joined clients, but does not affect existing certificates. |
| `DisableClientAuthentication` | - | - | `DirectoryId`, `Type` | - | `DisableClientAuthenticationResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidClientAuthStatusException`, `ServiceException`, `UnsupportedOperationException` | Disables alternative client authentication methods for the specified directory. |
| `DisableDirectoryDataAccess` | - | - | `DirectoryId` | - | `DisableDirectoryDataAccessResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryInDesiredStateException`, `DirectoryUnavailableException`, `ServiceException`, `UnsupportedOperationException` | Deactivates access to directory data via the Directory Service Data API for the specified directory. For more information, see Directory Service Data API Reference. |
| `DisableLDAPS` | - | - | `DirectoryId`, `Type` | - | `DisableLDAPSResult` | `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `InvalidLDAPSStatusException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Deactivates LDAP secure calls for the specified directory. |
| `DisableRadius` | - | - | `DirectoryId` | - | `DisableRadiusResult` | `ClientException`, `EntityDoesNotExistException`, `ServiceException` | Disables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory. |
| `DisableSso` | - | - | `DirectoryId` | - | `DisableSsoResult` | `AuthenticationFailedException`, `ClientException`, `EntityDoesNotExistException`, `InsufficientPermissionsException`, `ServiceException` | Disables single-sign on for a directory. |
| `EnableCAEnrollmentPolicy` | - | - | `DirectoryId`, `PcaConnectorArn` | - | `EnableCAEnrollmentPolicyResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `EnableAlreadyInProgressException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InvalidParameterException`, ... (+1) | Enables certificate authority (CA) enrollment policy for the specified directory. This allows domain-joined clients to automatically request and receive certificates from the specified Amazon Web Services Private Certificate Authority. |
| `EnableClientAuthentication` | - | - | `DirectoryId`, `Type` | - | `EnableClientAuthenticationResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidClientAuthStatusException`, `NoAvailableCertificateException`, `ServiceException`, `UnsupportedOperationException` | Enables alternative client authentication methods for the specified directory. |
| `EnableDirectoryDataAccess` | - | - | `DirectoryId` | - | `EnableDirectoryDataAccessResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryInDesiredStateException`, `DirectoryUnavailableException`, `ServiceException`, `UnsupportedOperationException` | Enables access to directory data via the Directory Service Data API for the specified directory. For more information, see Directory Service Data API Reference. |
| `EnableLDAPS` | - | - | `DirectoryId`, `Type` | - | `EnableLDAPSResult` | `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `InvalidLDAPSStatusException`, `InvalidParameterException`, `NoAvailableCertificateException`, `ServiceException`, `UnsupportedOperationException` | Activates the switch for the specific directory to always use LDAP secure calls. |
| `EnableRadius` | - | - | `DirectoryId`, `RadiusSettings` | - | `EnableRadiusResult` | `ClientException`, `EntityAlreadyExistsException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Enables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory. |
| `EnableSso` | - | - | `DirectoryId` | - | `EnableSsoResult` | `AuthenticationFailedException`, `ClientException`, `EntityDoesNotExistException`, `InsufficientPermissionsException`, `ServiceException` | Enables single sign-on for a directory. Single sign-on allows users in your directory to access certain Amazon Web Services services from a computer joined to the directory without having to enter their credentials separately. |
| `GetDirectoryLimits` | - | - | - | - | `GetDirectoryLimitsResult` | `ClientException`, `EntityDoesNotExistException`, `ServiceException` | Obtains directory limit information for the current Region. |
| `GetSnapshotLimits` | - | - | `DirectoryId` | - | `GetSnapshotLimitsResult` | `ClientException`, `EntityDoesNotExistException`, `ServiceException` | Obtains the manual snapshot limits for a directory. |
| `ListADAssessments` | - | `paginated` | - | - | `ListADAssessmentsResult` | `ClientException`, `DirectoryDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Retrieves a list of directory assessments for the specified directory or all assessments in your account. Use this operation to monitor assessment status and manage multiple assessments. |
| `ListCertificates` | - | `paginated` | `DirectoryId` | - | `ListCertificatesResult` | `ClientException`, `DirectoryDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | For the specified directory, lists all the certificates registered for a secure LDAP or client certificate authentication. |
| `ListIpRoutes` | - | `paginated` | `DirectoryId` | - | `ListIpRoutesResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException` | Lists the address blocks that you have added to a directory. |
| `ListLogSubscriptions` | - | `paginated` | - | - | `ListLogSubscriptionsResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `ServiceException` | Lists the active log subscriptions for the Amazon Web Services account. |
| `ListSchemaExtensions` | - | `paginated` | `DirectoryId` | - | `ListSchemaExtensionsResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `ServiceException` | Lists all schema extensions applied to a Microsoft AD Directory. |
| `ListTagsForResource` | - | `paginated` | `ResourceId` | - | `ListTagsForResourceResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidNextTokenException`, `InvalidParameterException`, `ServiceException` | Lists all tags on a directory. |
| `RegisterCertificate` | - | - | `CertificateData`, `DirectoryId` | - | `RegisterCertificateResult` | `CertificateAlreadyExistsException`, `CertificateLimitExceededException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `InvalidCertificateException`, `InvalidParameterException`, `ServiceException`, ... (+1) | Registers a certificate for a secure LDAP or client certificate authentication. |
| `RegisterEventTopic` | - | - | `DirectoryId`, `TopicName` | - | `RegisterEventTopicResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Associates a directory with an Amazon SNS topic. This establishes the directory as a publisher to the specified Amazon SNS topic. |
| `RejectSharedDirectory` | - | - | `SharedDirectoryId` | - | `RejectSharedDirectoryResult` | `ClientException`, `DirectoryAlreadySharedException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Rejects a directory sharing request that was sent from the directory owner account. |
| `RemoveIpRoutes` | - | - | `DirectoryId` | - | `RemoveIpRoutesResult` | `ClientException`, `DirectoryUnavailableException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Removes IP address blocks from a directory. |
| `RemoveRegion` | - | - | `DirectoryId` | - | `RemoveRegionResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `ServiceException`, `UnsupportedOperationException` | Stops all replication and removes the domain controllers from the specified Region. You cannot remove the primary Region with this operation. |
| `RemoveTagsFromResource` | - | - | `ResourceId`, `TagKeys` | - | `RemoveTagsFromResourceResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Removes tags from a directory. |
| `ResetUserPassword` | - | - | `DirectoryId`, `NewPassword`, `UserName` | - | `ResetUserPasswordResult` | `ClientException`, `DirectoryUnavailableException`, `EntityDoesNotExistException`, `InvalidPasswordException`, `ServiceException`, `UnsupportedOperationException`, `UserDoesNotExistException` | Resets the password for any user in your Managed Microsoft AD or Simple AD directory. Disabled users will become enabled and can be authenticated following the API call. |
| `RestoreFromSnapshot` | - | - | `SnapshotId` | - | `RestoreFromSnapshotResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Restores a directory using an existing directory snapshot. When you restore a directory from a snapshot, any changes made to the directory after the snapshot date are overwritten. |
| `ShareDirectory` | - | - | `DirectoryId`, `ShareMethod`, `ShareTarget` | - | `ShareDirectoryResult` | `AccessDeniedException`, `ClientException`, `DirectoryAlreadySharedException`, `EntityDoesNotExistException`, `InvalidParameterException`, `InvalidTargetException`, `OrganizationsException`, `ServiceException`, ... (+2) | Shares a specified directory (`DirectoryId`) in your Amazon Web Services account (directory owner) with another Amazon Web Services account (directory consumer). With this operation you can use your directory from any Amazon Web Services account and from any... |
| `StartADAssessment` | - | - | - | - | `StartADAssessmentResult` | `ADAssessmentLimitExceededException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Initiates a directory assessment to validate your self-managed AD environment for hybrid domain join. The assessment checks compatibility and connectivity of the self-managed AD environment. |
| `StartSchemaExtension` | - | - | `CreateSnapshotBeforeSchemaExtension`, `Description`, `DirectoryId`, `LdifContent` | - | `StartSchemaExtensionResult` | `ClientException`, `DirectoryUnavailableException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `SnapshotLimitExceededException` | Applies a schema extension to a Microsoft AD directory. |
| `UnshareDirectory` | - | - | `DirectoryId`, `UnshareTarget` | - | `UnshareDirectoryResult` | `ClientException`, `DirectoryNotSharedException`, `EntityDoesNotExistException`, `InvalidTargetException`, `ServiceException` | Stops the directory sharing between the directory owner and consumer accounts. |
| `UpdateConditionalForwarder` | - | - | `DirectoryId`, `RemoteDomainName` | - | `UpdateConditionalForwarderResult` | `ClientException`, `DirectoryUnavailableException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Updates a conditional forwarder that has been set up for your Amazon Web Services directory. |
| `UpdateDirectorySetup` | - | - | `DirectoryId`, `UpdateType` | - | `UpdateDirectorySetupResult` | `AccessDeniedException`, `ClientException`, `DirectoryDoesNotExistException`, `DirectoryInDesiredStateException`, `DirectoryUnavailableException`, `InvalidParameterException`, `ServiceException`, `SnapshotLimitExceededException`, ... (+1) | Updates directory configuration for the specified update type. |
| `UpdateHybridAD` | - | - | `DirectoryId` | - | `UpdateHybridADResult` | `ADAssessmentLimitExceededException`, `ClientException`, `DirectoryDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Updates the configuration of an existing hybrid directory. You can recover hybrid directory administrator account or modify self-managed instance settings. |
| `UpdateNumberOfDomainControllers` | - | - | `DesiredNumber`, `DirectoryId` | - | `UpdateNumberOfDomainControllersResult` | `ClientException`, `DirectoryUnavailableException`, `DomainControllerLimitExceededException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Adds or removes domain controllers to or from the directory. Based on the difference between current value and new value (provided through this API call), domain controllers will be added or removed. |
| `UpdateRadius` | - | - | `DirectoryId`, `RadiusSettings` | - | `UpdateRadiusResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Updates the Remote Authentication Dial In User Service (RADIUS) server information for an AD Connector or Microsoft AD directory. |
| `UpdateSettings` | - | - | `DirectoryId`, `Settings` | - | `UpdateSettingsResult` | `ClientException`, `DirectoryDoesNotExistException`, `DirectoryUnavailableException`, `IncompatibleSettingsException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException`, `UnsupportedSettingsException` | Updates the configurable settings for the specified directory. |
| `UpdateTrust` | - | - | `TrustId` | - | `UpdateTrustResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException` | Updates the trust that has been set up between your Managed Microsoft AD directory and an self-managed Active Directory. |
| `VerifyTrust` | - | - | `TrustId` | - | `VerifyTrustResult` | `ClientException`, `EntityDoesNotExistException`, `InvalidParameterException`, `ServiceException`, `UnsupportedOperationException` | Directory Service for Microsoft Active Directory allows you to configure and verify trust relationships. This action verifies a trust relationship between your Managed Microsoft AD directory and an external domain. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClientException` | `structure` | `Message`, `RequestId` | A client exception has occurred. |
| `ServiceException` | `structure` | `Message`, `RequestId` | An exception has occurred in Directory Service. |
| `InvalidParameterException` | `structure` | `Message`, `RequestId` | One or more parameters are not valid. |
| `EntityDoesNotExistException` | `structure` | `Message`, `RequestId` | The specified entity could not be found. |
| `UnsupportedOperationException` | `structure` | `Message`, `RequestId` | The operation is not supported. |
| `DirectoryDoesNotExistException` | `structure` | `Message`, `RequestId` | The specified directory does not exist in the system. |
| `DirectoryUnavailableException` | `structure` | `Message`, `RequestId` | The specified directory is unavailable. |
| `InvalidNextTokenException` | `structure` | `Message`, `RequestId` | The `NextToken` value is not valid. |
| `AccessDeniedException` | `structure` | `Message`, `RequestId` | You do not have sufficient access to perform this action. |
| `EntityAlreadyExistsException` | `structure` | `Message`, `RequestId` | The specified entity already exists. |
| `DirectoryLimitExceededException` | `structure` | `Message`, `RequestId` | The maximum number of directories in the region has been reached. |
| `DirectoryAlreadySharedException` | `structure` | `Message`, `RequestId` | The specified directory has already been shared with this Amazon Web Services account. |
| `AuthenticationFailedException` | `structure` | `Message`, `RequestId` | An authentication error occurred. |
| `ADAssessmentLimitExceededException` | `structure` | `Message`, `RequestId` | A directory assessment is automatically created when you create a hybrid directory. |
| `InsufficientPermissionsException` | `structure` | `Message`, `RequestId` | The account does not have sufficient permission to perform the operation. |
| `SnapshotLimitExceededException` | `structure` | `Message`, `RequestId` | The maximum number of manual snapshots for the directory has been reached. |
| `DirectoryInDesiredStateException` | `structure` | `Message`, `RequestId` | The directory is already updated to desired update type settings. |
| `CertificateDoesNotExistException` | `structure` | `Message`, `RequestId` | The certificate is not present in the system for describe or deregister activities. |
| `InvalidClientAuthStatusException` | `structure` | `Message`, `RequestId` | Client authentication is already enabled. |
| `InvalidLDAPSStatusException` | `structure` | `Message`, `RequestId` | The LDAP activities could not be performed because they are limited by the LDAPS status. |
| `NoAvailableCertificateException` | `structure` | `Message`, `RequestId` | Client authentication setup could not be completed because at least one valid certificate must be registered in the system. |
| `InvalidTargetException` | `structure` | `Message`, `RequestId` | The specified shared target is not valid. |
| `AcceptSharedDirectoryRequest` | `structure` | `SharedDirectoryId` | - |
| `AcceptSharedDirectoryResult` | `structure` | `SharedDirectory` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
