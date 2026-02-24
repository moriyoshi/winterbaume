# AWS Transfer Family

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Transfer Family is a fully managed service that enables the transfer of files over the File Transfer Protocol (FTP), File Transfer Protocol over SSL (FTPS), or Secure Shell (SSH) File Transfer Protocol (SFTP) directly into and out of Amazon Simple Storage Service (Amazon S3) or Amazon EFS. Additionally, you can use Applicability Statement 2 (AS2) to transfer files into and out of Amazon S3. Amazon Web Services helps you seamlessly migrate your file transfer workflows to Transfer Family by integrating with existing authentication systems, and providing DNS routing with Amazon Route 53 so nothing changes for your customers and partners, or their applications. With your data in Amazon S3, you can use it with Amazon Web Services services for processing, analytics, machine learning, and archiving. Getting started with Transfer Family is easy since there is no infrastructure to buy and set up.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-transfer/tests/scenario_test.rs`: provision an SFTP server, create a user, import SSH keys, inspect/list, and delete the chain.
- Backported from `scenario_test.rs`: configure an AS2 partner agreement chain with profiles, certificates/connectors or related agreement resources.
- Scenario insight from EC2: add full state-machine walks for AWS Transfer Family resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support managed file-transfer servers, users, SSH keys, workflows, connectors, AS2 profiles/agreements/certificates, host keys, tagging, and server lifecycle state.

## Service Identity and Protocol

- AWS model slug: `transfer`
- AWS SDK for Rust slug: `transfer`
- Model version: `2018-11-05`
- Model file: `vendor/api-models-aws/models/transfer/service/2018-11-05/transfer-2018-11-05.json`
- SDK ID: `Transfer`
- Endpoint prefix: `transfer`
- ARN namespace: `transfer`
- CloudFormation name: `Transfer`
- CloudTrail event source: `transfer.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (14), `Describe` (13), `Delete` (12), `Update` (10), `Create` (8), `Start` (5), `Import` (3), `Test` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAccess`, `CreateAgreement`, `CreateConnector`, `CreateProfile`, `CreateServer`, `CreateUser`, `CreateWebApp`, `CreateWorkflow`, `DeleteAccess`, `DeleteAgreement`, `DeleteCertificate`, `DeleteConnector`, `DeleteHostKey`, `DeleteProfile`, `DeleteServer`, `DeleteSshPublicKey`, `DeleteUser`, `DeleteWebApp`, `DeleteWebAppCustomization`, `DeleteWorkflow`, ... (+21).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccess`, `DescribeAgreement`, `DescribeCertificate`, `DescribeConnector`, `DescribeExecution`, `DescribeHostKey`, `DescribeProfile`, `DescribeSecurityPolicy`, `DescribeServer`, `DescribeUser`, `DescribeWebApp`, `DescribeWebAppCustomization`, `DescribeWorkflow`, `ListAccesses`, `ListAgreements`, `ListCertificates`, `ListConnectors`, `ListExecutions`, `ListFileTransferResults`, `ListHostKeys`, ... (+7).
- Pagination is modelled for 13 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 10 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeExecution`, `ImportCertificate`, `ImportHostKey`, `ImportSshPublicKey`, `ListExecutions`, `StartDirectoryListing`, `StartFileTransfer`, `StartRemoteDelete`, `StartRemoteMove`, `StartServer`, `StopServer`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 71 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AgreementResource` | `AgreementId`, `ServerId` | create: `CreateAgreement`; read: `DescribeAgreement`; update: `UpdateAgreement`; delete: `DeleteAgreement`; list: `ListAgreements` | - | - |
| `CertificateResource` | `CertificateId` | create: `ImportCertificate`; read: `DescribeCertificate`; update: `UpdateCertificate`; delete: `DeleteCertificate`; list: `ListCertificates` | - | - |
| `ConnectorResource` | `ConnectorId` | create: `CreateConnector`; read: `DescribeConnector`; update: `UpdateConnector`; delete: `DeleteConnector`; list: `ListConnectors` | - | - |
| `ProfileResource` | `ProfileId` | create: `CreateProfile`; read: `DescribeProfile`; update: `UpdateProfile`; delete: `DeleteProfile`; list: `ListProfiles` | - | - |
| `ServerResource` | `ServerId` | create: `CreateServer`; read: `DescribeServer`; update: `UpdateServer`; delete: `DeleteServer`; list: `ListServers` | - | - |
| `UserResource` | `ServerId`, `UserName` | put: `CreateUser`; read: `DescribeUser`; update: `UpdateUser`; delete: `DeleteUser`; list: `ListUsers` | - | - |
| `WebAppCustomizationResource` | `WebAppId` | read: `DescribeWebAppCustomization`; update: `UpdateWebAppCustomization`; delete: `DeleteWebAppCustomization` | - | - |
| `WebAppResource` | `WebAppId` | create: `CreateWebApp`; read: `DescribeWebApp`; update: `UpdateWebApp`; delete: `DeleteWebApp`; list: `ListWebApps` | - | - |
| `WorkflowResource` | `WorkflowId` | create: `CreateWorkflow`; read: `DescribeWorkflow`; delete: `DeleteWorkflow`; list: `ListWorkflows` | - | - |
## Operation Groups

### List

- Operations: `ListAccesses`, `ListAgreements`, `ListCertificates`, `ListConnectors`, `ListExecutions`, `ListFileTransferResults`, `ListHostKeys`, `ListProfiles`, `ListSecurityPolicies`, `ListServers`, `ListTagsForResource`, `ListUsers`, `ListWebApps`, `ListWorkflows`
- Traits: `paginated` (13), `readonly` (13)
- Common required input members in this group: `Arn`, `ConnectorId`, `ServerId`, `TransferId`, `WorkflowId`

### Describe

- Operations: `DescribeAccess`, `DescribeAgreement`, `DescribeCertificate`, `DescribeConnector`, `DescribeExecution`, `DescribeHostKey`, `DescribeProfile`, `DescribeSecurityPolicy`, `DescribeServer`, `DescribeUser`, `DescribeWebApp`, `DescribeWebAppCustomization`, `DescribeWorkflow`
- Traits: `readonly` (13)
- Common required input members in this group: `AgreementId`, `CertificateId`, `ConnectorId`, `ExecutionId`, `ExternalId`, `HostKeyId`, `ProfileId`, `SecurityPolicyName`, `ServerId`, `UserName`, `WebAppId`, `WorkflowId`

### Delete

- Operations: `DeleteAccess`, `DeleteAgreement`, `DeleteCertificate`, `DeleteConnector`, `DeleteHostKey`, `DeleteProfile`, `DeleteServer`, `DeleteSshPublicKey`, `DeleteUser`, `DeleteWebApp`, `DeleteWebAppCustomization`, `DeleteWorkflow`
- Traits: `idempotent` (9)
- Common required input members in this group: `AgreementId`, `CertificateId`, `ConnectorId`, `ExternalId`, `HostKeyId`, `ProfileId`, `ServerId`, `SshPublicKeyId`, `UserName`, `WebAppId`, `WorkflowId`

### Update

- Operations: `UpdateAccess`, `UpdateAgreement`, `UpdateCertificate`, `UpdateConnector`, `UpdateHostKey`, `UpdateProfile`, `UpdateServer`, `UpdateUser`, `UpdateWebApp`, `UpdateWebAppCustomization`
- Common required input members in this group: `AgreementId`, `CertificateId`, `ConnectorId`, `Description`, `ExternalId`, `HostKeyId`, `ProfileId`, `ServerId`, `UserName`, `WebAppId`

### Create

- Operations: `CreateAccess`, `CreateAgreement`, `CreateConnector`, `CreateProfile`, `CreateServer`, `CreateUser`, `CreateWebApp`, `CreateWorkflow`
- Traits: `idempotent` (1)
- Common required input members in this group: `AccessRole`, `As2Id`, `ExternalId`, `IdentityProviderDetails`, `LocalProfileId`, `PartnerProfileId`, `ProfileType`, `Role`, `ServerId`, `Steps`, `UserName`

### Start

- Operations: `StartDirectoryListing`, `StartFileTransfer`, `StartRemoteDelete`, `StartRemoteMove`, `StartServer`
- Common required input members in this group: `ConnectorId`, `DeletePath`, `OutputDirectoryPath`, `RemoteDirectoryPath`, `ServerId`, `SourcePath`, `TargetPath`

### Import

- Operations: `ImportCertificate`, `ImportHostKey`, `ImportSshPublicKey`
- Common required input members in this group: `Certificate`, `HostKeyBody`, `ServerId`, `SshPublicKeyBody`, `Usage`, `UserName`

### Test

- Operations: `TestConnection`, `TestIdentityProvider`
- Common required input members in this group: `ConnectorId`, `ServerId`, `UserName`

### Send

- Operations: `SendWorkflowStepState`
- Common required input members in this group: `ExecutionId`, `Status`, `Token`, `WorkflowId`

### Stop

- Operations: `StopServer`
- Common required input members in this group: `ServerId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `Arn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `Arn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAccess` | - | - | `ExternalId`, `Role`, `ServerId` | - | `CreateAccessResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Used by administrators to choose which groups in the directory should have access to upload and download files over the enabled protocols using Transfer Family. For example, a Microsoft Active Directory might contain 50,000 users, but only a small fraction... |
| `CreateAgreement` | - | - | `AccessRole`, `LocalProfileId`, `PartnerProfileId`, `ServerId` | - | `CreateAgreementResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Creates an agreement. An agreement is a bilateral trading partner agreement, or partnership, between an Transfer Family server and an AS2 process. |
| `CreateConnector` | - | - | `AccessRole` | - | `CreateConnectorResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Creates the connector, which captures the parameters for a connection for the AS2 or SFTP protocol. For AS2, the connector is required for sending files to an externally hosted AS2 server. |
| `CreateProfile` | - | - | `As2Id`, `ProfileType` | - | `CreateProfileResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Creates the local or partner profile to use for AS2 transfers. |
| `CreateServer` | - | - | - | - | `CreateServerResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Instantiates an auto-scaling virtual server based on the selected file transfer protocol in Amazon Web Services. When you make updates to your file transfer protocol-enabled server or when you work with users, use the service-generated `ServerId` property... |
| `CreateUser` | - | `idempotent` | `Role`, `ServerId`, `UserName` | - | `CreateUserResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates a user and associates them with an existing file transfer protocol-enabled server. You can only create and associate users with servers that have the `IdentityProviderType` set to `SERVICE_MANAGED`. |
| `CreateWebApp` | `POST /createWebApp` | - | `IdentityProviderDetails` | - | `CreateWebAppResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a web app based on specified parameters, and returns the ID for the new web app. You can configure the web app to be publicly accessible or hosted within a VPC. |
| `CreateWorkflow` | - | - | `Steps` | - | `CreateWorkflowResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ServiceUnavailableException`, `ThrottlingException` | Allows you to create a workflow with specified steps and step details the workflow invokes after file transfer completes. After creating a workflow, you can associate the workflow created with any transfer servers by specifying the `workflow-details` field in... |
| `DeleteAccess` | - | - | `ExternalId`, `ServerId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Allows you to delete the access specified in the `ServerID` and `ExternalID` parameters. |
| `DeleteAgreement` | - | `idempotent` | `AgreementId`, `ServerId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Delete the agreement that's specified in the provided `AgreementId`. |
| `DeleteCertificate` | - | `idempotent` | `CertificateId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the certificate that's specified in the `CertificateId` parameter. |
| `DeleteConnector` | - | `idempotent` | `ConnectorId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the connector that's specified in the provided `ConnectorId`. |
| `DeleteHostKey` | - | - | `HostKeyId`, `ServerId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes the host key that's specified in the `HostKeyId` parameter. |
| `DeleteProfile` | - | `idempotent` | `ProfileId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the profile that's specified in the `ProfileId` parameter. |
| `DeleteServer` | - | `idempotent` | `ServerId` | - | `Unit` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the file transfer protocol-enabled server that you specify. No response returns from this operation. |
| `DeleteSshPublicKey` | - | - | `ServerId`, `SshPublicKeyId`, `UserName` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes a user's Secure Shell (SSH) public key. |
| `DeleteUser` | - | `idempotent` | `ServerId`, `UserName` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the user belonging to a file transfer protocol-enabled server you specify. No response returns from this operation. |
| `DeleteWebApp` | `POST /deleteWebApp` | `idempotent` | `WebAppId` | - | `Unit` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified web app. |
| `DeleteWebAppCustomization` | `POST /deleteWebAppCustomization` | `idempotent` | `WebAppId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the `WebAppCustomization` object that corresponds to the web app ID specified. |
| `DeleteWorkflow` | - | `idempotent` | `WorkflowId` | - | `Unit` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the specified workflow. |
| `DescribeAccess` | - | `readonly` | `ExternalId`, `ServerId` | - | `DescribeAccessResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the access that is assigned to the specific file transfer protocol-enabled server, as identified by its `ServerId` property and its `ExternalId`. The response from this call returns the properties of the access that is associated with the `ServerId`... |
| `DescribeAgreement` | - | `readonly` | `AgreementId`, `ServerId` | - | `DescribeAgreementResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the agreement that's identified by the `AgreementId`. |
| `DescribeCertificate` | - | `readonly` | `CertificateId` | - | `DescribeCertificateResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the certificate that's identified by the `CertificateId`. Transfer Family automatically publishes a Amazon CloudWatch metric called `DaysUntilExpiry` for imported certificates. |
| `DescribeConnector` | - | `readonly` | `ConnectorId` | - | `DescribeConnectorResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the connector that's identified by the `ConnectorId.` |
| `DescribeExecution` | - | `readonly` | `ExecutionId`, `WorkflowId` | - | `DescribeExecutionResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | You can use `DescribeExecution` to check the details of the execution of the specified workflow. This API call only returns details for in-progress workflows. |
| `DescribeHostKey` | - | `readonly` | `HostKeyId`, `ServerId` | - | `DescribeHostKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the details of the host key that's specified by the `HostKeyId` and `ServerId`. |
| `DescribeProfile` | - | `readonly` | `ProfileId` | - | `DescribeProfileResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the details of the profile that's specified by the `ProfileId`. |
| `DescribeSecurityPolicy` | - | `readonly` | `SecurityPolicyName` | - | `DescribeSecurityPolicyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the security policy that is attached to your server or SFTP connector. The response contains a description of the security policy's properties. |
| `DescribeServer` | - | `readonly` | `ServerId` | - | `DescribeServerResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes a file transfer protocol-enabled server that you specify by passing the `ServerId` parameter. The response contains a description of a server's properties. |
| `DescribeUser` | - | `readonly` | `ServerId`, `UserName` | - | `DescribeUserResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the user assigned to the specific file transfer protocol-enabled server, as identified by its `ServerId` property. The response from this call returns the properties of the user associated with the `ServerId` value that was specified. |
| `DescribeWebApp` | `POST /describeWebApp` | `readonly` | `WebAppId` | - | `DescribeWebAppResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Describes the web app that's identified by `WebAppId`. The response includes endpoint configuration details such as whether the web app is publicly accessible or VPC hosted. |
| `DescribeWebAppCustomization` | `POST /describeWebAppCustomization` | `readonly` | `WebAppId` | - | `DescribeWebAppCustomizationResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Describes the web app customization object that's identified by `WebAppId`. |
| `DescribeWorkflow` | - | `readonly` | `WorkflowId` | - | `DescribeWorkflowResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the specified workflow. |
| `ImportCertificate` | - | - | `Certificate`, `Usage` | - | `ImportCertificateResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Imports the signing and encryption certificates that you need to create local (AS2) profiles and partner profiles. You can import both the certificate and its chain in the `Certificate` parameter. |
| `ImportHostKey` | - | - | `HostKeyBody`, `ServerId` | - | `ImportHostKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Adds a host key to the server that's specified by the `ServerId` parameter. |
| `ImportSshPublicKey` | - | - | `ServerId`, `SshPublicKeyBody`, `UserName` | - | `ImportSshPublicKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Adds a Secure Shell (SSH) public key to a Transfer Family user identified by a `UserName` value assigned to the specific file transfer protocol-enabled server, identified by `ServerId`. The response returns the `UserName` value, the `ServerId` value, and the... |
| `ListAccesses` | - | `readonly`, `paginated` | `ServerId` | - | `ListAccessesResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the details for all the accesses you have on your server. |
| `ListAgreements` | - | `readonly`, `paginated` | `ServerId` | - | `ListAgreementsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of the agreements for the server that's identified by the `ServerId` that you supply. If you want to limit the results to a certain number, supply a value for the `MaxResults` parameter. |
| `ListCertificates` | - | `readonly`, `paginated` | - | - | `ListCertificatesResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of the current certificates that have been imported into Transfer Family. If you want to limit the results to a certain number, supply a value for the `MaxResults` parameter. |
| `ListConnectors` | - | `readonly`, `paginated` | - | - | `ListConnectorsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the connectors for the specified Region. |
| `ListExecutions` | - | `readonly`, `paginated` | `WorkflowId` | - | `ListExecutionsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists all in-progress executions for the specified workflow. If the specified workflow ID cannot be found, `ListExecutions` returns a `ResourceNotFound` exception. |
| `ListFileTransferResults` | `POST /listFileTransferResults` | `paginated` | `ConnectorId`, `TransferId` | - | `ListFileTransferResultsResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns real-time updates and detailed information on the status of each individual file being transferred in a specific file transfer operation. You specify the file transfer by providing its `ConnectorId` and its `TransferId`. |
| `ListHostKeys` | - | `readonly` | `ServerId` | - | `ListHostKeysResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of host keys for the server that's specified by the `ServerId` parameter. |
| `ListProfiles` | - | `readonly`, `paginated` | - | - | `ListProfilesResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of the profiles for your system. If you want to limit the results to a certain number, supply a value for the `MaxResults` parameter. |
| `ListSecurityPolicies` | - | `readonly`, `paginated` | - | - | `ListSecurityPoliciesResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ServiceUnavailableException` | Lists the security policies that are attached to your servers and SFTP connectors. For more information about security policies, see Working with security policies for servers or Working with security policies for SFTP connectors. |
| `ListServers` | - | `readonly`, `paginated` | - | - | `ListServersResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ServiceUnavailableException` | Lists the file transfer protocol-enabled servers that are associated with your Amazon Web Services account. |
| `ListTagsForResource` | - | `readonly`, `paginated` | `Arn` | - | `ListTagsForResourceResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ServiceUnavailableException` | Lists all of the tags associated with the Amazon Resource Name (ARN) that you specify. The resource can be a user, server, or role. |
| `ListUsers` | - | `readonly`, `paginated` | `ServerId` | - | `ListUsersResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the users for a file transfer protocol-enabled server that you specify by passing the `ServerId` parameter. |
| `ListWebApps` | `POST /listWebApps` | `readonly`, `paginated` | - | - | `ListWebAppsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ThrottlingException` | Lists all web apps associated with your Amazon Web Services account for your current region. The response includes the endpoint type for each web app, showing whether it is publicly accessible or VPC hosted. |
| `ListWorkflows` | - | `readonly`, `paginated` | - | - | `ListWorkflowsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ServiceUnavailableException` | Lists all workflows associated with your Amazon Web Services account for your current region. |
| `SendWorkflowStepState` | - | - | `ExecutionId`, `Status`, `Token`, `WorkflowId` | - | `SendWorkflowStepStateResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Sends a callback for asynchronous custom steps. The `ExecutionId`, `WorkflowId`, and `Token` are passed to the target resource during execution of a custom step of a workflow. |
| `StartDirectoryListing` | - | - | `ConnectorId`, `OutputDirectoryPath`, `RemoteDirectoryPath` | - | `StartDirectoryListingResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves a list of the contents of a directory from a remote SFTP server. You specify the connector ID, the output path, and the remote directory path. |
| `StartFileTransfer` | - | - | `ConnectorId` | - | `StartFileTransferResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Begins a file transfer between local Amazon Web Services storage and a remote AS2 or SFTP server. For an AS2 connector, you specify the `ConnectorId` and one or more `SendFilePaths` to identify the files you want to transfer. |
| `StartRemoteDelete` | `POST /startRemoteDelete` | - | `ConnectorId`, `DeletePath` | - | `StartRemoteDeleteResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes a file or directory on the remote SFTP server. |
| `StartRemoteMove` | `POST /startRemoteMove` | - | `ConnectorId`, `SourcePath`, `TargetPath` | - | `StartRemoteMoveResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Moves or renames a file or directory on the remote SFTP server. |
| `StartServer` | - | - | `ServerId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Changes the state of a file transfer protocol-enabled server from `OFFLINE` to `ONLINE`. It has no impact on a server that is already `ONLINE`. |
| `StopServer` | - | - | `ServerId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Changes the state of a file transfer protocol-enabled server from `ONLINE` to `OFFLINE`. An `OFFLINE` server cannot accept and process file transfer jobs. |
| `TagResource` | - | - | `Arn`, `Tags` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Attaches a key-value pair to a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities. |
| `TestConnection` | - | - | `ConnectorId` | - | `TestConnectionResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Tests whether your SFTP connector is set up successfully. We highly recommend that you call this operation to test your ability to transfer files between local Amazon Web Services storage and a trading partner's SFTP server. |
| `TestIdentityProvider` | - | - | `ServerId`, `UserName` | - | `TestIdentityProviderResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | If the `IdentityProviderType` of a file transfer protocol-enabled server is `AWS_DIRECTORY_SERVICE` or `API_Gateway`, tests whether your identity provider is set up successfully. We highly recommend that you call this operation to test your authentication... |
| `UntagResource` | - | - | `Arn`, `TagKeys` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Detaches a key-value pair from a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities. |
| `UpdateAccess` | - | - | `ExternalId`, `ServerId` | - | `UpdateAccessResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Allows you to update parameters for the access specified in the `ServerID` and `ExternalID` parameters. |
| `UpdateAgreement` | - | - | `AgreementId`, `ServerId` | - | `UpdateAgreementResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates some of the parameters for an existing agreement. Provide the `AgreementId` and the `ServerId` for the agreement that you want to update, along with the new values for the parameters to update. |
| `UpdateCertificate` | - | - | `CertificateId` | - | `UpdateCertificateResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates the active and inactive dates for a certificate. |
| `UpdateConnector` | - | - | `ConnectorId` | - | `UpdateConnectorResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates some of the parameters for an existing connector. Provide the `ConnectorId` for the connector that you want to update, along with the new values for the parameters to update. |
| `UpdateHostKey` | - | - | `Description`, `HostKeyId`, `ServerId` | - | `UpdateHostKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates the description for the host key that's specified by the `ServerId` and `HostKeyId` parameters. |
| `UpdateProfile` | - | - | `ProfileId` | - | `UpdateProfileResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates some of the parameters for an existing profile. Provide the `ProfileId` for the profile that you want to update, along with the new values for the parameters to update. |
| `UpdateServer` | - | - | `ServerId` | - | `UpdateServerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates the file transfer protocol-enabled server's properties after that server has been created. The `UpdateServer` call returns the `ServerId` of the server you updated. |
| `UpdateUser` | - | - | `ServerId`, `UserName` | - | `UpdateUserResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Assigns new properties to a user. Parameters you pass modify any or all of the following: the home directory, role, and policy for the `UserName` and `ServerId` you specify. |
| `UpdateWebApp` | `POST /updateWebApp` | - | `WebAppId` | - | `UpdateWebAppResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Assigns new properties to a web app. You can modify the access point, identity provider details, endpoint configuration, and the web app units. |
| `UpdateWebAppCustomization` | `POST /updateWebAppCustomization` | - | `WebAppId` | - | `UpdateWebAppCustomizationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Assigns new customization properties to a web app. You can modify the icon file, logo file, and title. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceError` | `structure` | `Message` | This exception is thrown when an error occurs in the Transfer Family service. |
| `InvalidRequestException` | `structure` | `Message` | This exception is thrown when the client submits a malformed request. |
| `ResourceNotFoundException` | `structure` | `Message`, `Resource`, `ResourceType` | This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service. |
| `ServiceUnavailableException` | `structure` | `Message` | The request has failed because the Amazon Web ServicesTransfer Family service is not available. |
| `ThrottlingException` | `structure` | `RetryAfterSeconds` | The request was denied due to request throttling. |
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InvalidNextTokenException` | `structure` | `Message` | The `NextToken` parameter that was passed is invalid. |
| `ResourceExistsException` | `structure` | `Message`, `Resource`, `ResourceType` | The requested resource does not exist, or exists in a region other than the one specified for the command. |
| `ConflictException` | `structure` | `Message` | This exception is thrown when the `UpdateServer` is called for a file transfer protocol-enabled server that has VPC as the endpoint type and the server's `VpcEndpointID` is not in... |
| `CreateAccessRequest` | `structure` | `ExternalId`, `HomeDirectory`, `HomeDirectoryMappings`, `HomeDirectoryType`, `Policy`, `PosixProfile`, `Role`, `ServerId` | - |
| `CreateAccessResponse` | `structure` | `ExternalId`, `ServerId` | - |
| `CreateAgreementRequest` | `structure` | `AccessRole`, `BaseDirectory`, `CustomDirectories`, `Description`, `EnforceMessageSigning`, `LocalProfileId`, `PartnerProfileId`, `PreserveFilename`, `ServerId`, `Status`, `Tags` | - |
| `CreateAgreementResponse` | `structure` | `AgreementId` | - |
| `CreateConnectorRequest` | `structure` | `AccessRole`, `As2Config`, `EgressConfig`, `LoggingRole`, `SecurityPolicyName`, `SftpConfig`, `Tags`, `Url` | - |
| `CreateConnectorResponse` | `structure` | `ConnectorId` | - |
| `CreateProfileRequest` | `structure` | `As2Id`, `CertificateIds`, `ProfileType`, `Tags` | - |
| `CreateProfileResponse` | `structure` | `ProfileId` | - |
| `CreateServerRequest` | `structure` | `Certificate`, `Domain`, `EndpointDetails`, `EndpointType`, `HostKey`, `IdentityProviderDetails`, `IdentityProviderType`, `IpAddressType`, `LoggingRole`, `PostAuthenticationLoginBanner`, `PreAuthenticationLoginBanner`, `ProtocolDetails`, ... (+6) | - |
| `CreateServerResponse` | `structure` | `ServerId` | - |
| `CreateUserRequest` | `structure` | `HomeDirectory`, `HomeDirectoryMappings`, `HomeDirectoryType`, `Policy`, `PosixProfile`, `Role`, `ServerId`, `SshPublicKeyBody`, `Tags`, `UserName` | - |
| `CreateUserResponse` | `structure` | `ServerId`, `UserName` | - |
| `CreateWebAppRequest` | `structure` | `AccessEndpoint`, `EndpointDetails`, `IdentityProviderDetails`, `Tags`, `WebAppEndpointPolicy`, `WebAppUnits` | - |
| `CreateWebAppResponse` | `structure` | `WebAppId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
