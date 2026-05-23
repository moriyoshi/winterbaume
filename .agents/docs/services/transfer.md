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

- Operations: `ListAccesses`, `ListExecutions`, `ListFileTransferResults`, `ListHostKeys`, `ListSecurityPolicies`, `ListTagsForResource`
- Traits: `readonly` (5), `paginated` (5)
- Common required input members in this group: `ServerId`

### Start

- Operations: `StartDirectoryListing`, `StartFileTransfer`, `StartRemoteDelete`, `StartRemoteMove`, `StartServer`
- Common required input members in this group: `ConnectorId`

### Describe

- Operations: `DescribeAccess`, `DescribeExecution`, `DescribeHostKey`, `DescribeSecurityPolicy`
- Traits: `readonly` (4)
- Common required input members in this group: `ServerId`

### Delete

- Operations: `DeleteAccess`, `DeleteHostKey`, `DeleteSshPublicKey`
- Common required input members in this group: `ServerId`

### Import

- Operations: `ImportHostKey`, `ImportSshPublicKey`
- Common required input members in this group: `ServerId`

### Test

- Operations: `TestConnection`, `TestIdentityProvider`
- Common required input members in this group: -

### Update

- Operations: `UpdateAccess`, `UpdateHostKey`
- Common required input members in this group: `ServerId`

### Create

- Operations: `CreateAccess`
- Common required input members in this group: -

### Send

- Operations: `SendWorkflowStepState`
- Common required input members in this group: -

### Stop

- Operations: `StopServer`
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
| `CreateAccess` | `-` | - | `Role`, `ServerId`, `ExternalId` | - | `CreateAccessResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Used by administrators to choose which groups in the directory should have access to upload and download files over the enabled protocols using Transfer Family. For example, a Microsoft Active Directory might contain ... |
| `DeleteAccess` | `-` | - | `ServerId`, `ExternalId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Allows you to delete the access specified in the ServerID and ExternalID parameters. |
| `DeleteHostKey` | `-` | - | `ServerId`, `HostKeyId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes the host key that's specified in the HostKeyId parameter. |
| `DeleteSshPublicKey` | `-` | - | `ServerId`, `SshPublicKeyId`, `UserName` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes a user's Secure Shell (SSH) public key. |
| `DescribeAccess` | `-` | `readonly` | `ServerId`, `ExternalId` | - | `DescribeAccessResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the access that is assigned to the specific file transfer protocol-enabled server, as identified by its ServerId property and its ExternalId . The response from this call returns the properties of the acces ... |
| `DescribeExecution` | `-` | `readonly` | `ExecutionId`, `WorkflowId` | - | `DescribeExecutionResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | You can use DescribeExecution to check the details of the execution of the specified workflow. This API call only returns details for in-progress workflows. If you provide an ID for an execution that is not in progre ... |
| `DescribeHostKey` | `-` | `readonly` | `ServerId`, `HostKeyId` | - | `DescribeHostKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the details of the host key that's specified by the HostKeyId and ServerId . |
| `DescribeSecurityPolicy` | `-` | `readonly` | `SecurityPolicyName` | - | `DescribeSecurityPolicyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Describes the security policy that is attached to your server or SFTP connector. The response contains a description of the security policy's properties. For more information about security policies, see Working with ... |
| `ImportHostKey` | `-` | - | `ServerId`, `HostKeyBody` | - | `ImportHostKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Adds a host key to the server that's specified by the ServerId parameter. |
| `ImportSshPublicKey` | `-` | - | `ServerId`, `SshPublicKeyBody`, `UserName` | - | `ImportSshPublicKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Adds a Secure Shell (SSH) public key to a Transfer Family user identified by a UserName value assigned to the specific file transfer protocol-enabled server, identified by ServerId . The response returns the UserName ... |
| `ListAccesses` | `-` | `readonly`, `paginated` | `ServerId` | - | `ListAccessesResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the details for all the accesses you have on your server. |
| `ListExecutions` | `-` | `readonly`, `paginated` | `WorkflowId` | - | `ListExecutionsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists all in-progress executions for the specified workflow. If the specified workflow ID cannot be found, ListExecutions returns a ResourceNotFound exception. |
| `ListFileTransferResults` | `POST /listFileTransferResults` | `paginated` | `ConnectorId`, `TransferId` | - | `ListFileTransferResultsResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns real-time updates and detailed information on the status of each individual file being transferred in a specific file transfer operation. You specify the file transfer by providing its ConnectorId and its Tra ... |
| `ListHostKeys` | `-` | `readonly` | `ServerId` | - | `ListHostKeysResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of host keys for the server that's specified by the ServerId parameter. |
| `ListSecurityPolicies` | `-` | `readonly`, `paginated` | - | - | `ListSecurityPoliciesResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ServiceUnavailableException` | Lists the security policies that are attached to your servers and SFTP connectors. For more information about security policies, see Working with security policies for servers or Working with security policies for SF ... |
| `ListTagsForResource` | `-` | `readonly`, `paginated` | `Arn` | - | `ListTagsForResourceResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidRequestException`, `ServiceUnavailableException` | Lists all of the tags associated with the Amazon Resource Name (ARN) that you specify. The resource can be a user, server, or role. |
| `SendWorkflowStepState` | `-` | - | `WorkflowId`, `ExecutionId`, `Token`, `Status` | - | `SendWorkflowStepStateResponse` | `AccessDeniedException`, `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Sends a callback for asynchronous custom steps. The ExecutionId , WorkflowId , and Token are passed to the target resource during execution of a custom step of a workflow. You must include those with their callback a ... |
| `StartDirectoryListing` | `-` | - | `ConnectorId`, `RemoteDirectoryPath`, `OutputDirectoryPath` | - | `StartDirectoryListingResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves a list of the contents of a directory from a remote SFTP server. You specify the connector ID, the output path, and the remote directory path. You can also specify the optional MaxItems value to control the ... |
| `StartFileTransfer` | `-` | - | `ConnectorId` | - | `StartFileTransferResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Begins a file transfer between local Amazon Web Services storage and a remote AS2 or SFTP server. For an AS2 connector, you specify the ConnectorId and one or more SendFilePaths to identify the files you want to tran ... |
| `StartRemoteDelete` | `POST /startRemoteDelete` | - | `ConnectorId`, `DeletePath` | - | `StartRemoteDeleteResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes a file or directory on the remote SFTP server. |
| `StartRemoteMove` | `POST /startRemoteMove` | - | `ConnectorId`, `SourcePath`, `TargetPath` | - | `StartRemoteMoveResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Moves or renames a file or directory on the remote SFTP server. |
| `StartServer` | `-` | - | `ServerId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Changes the state of a file transfer protocol-enabled server from OFFLINE to ONLINE . It has no impact on a server that is already ONLINE . An ONLINE server can accept and process file transfer jobs. The state of STA ... |
| `StopServer` | `-` | - | `ServerId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Changes the state of a file transfer protocol-enabled server from ONLINE to OFFLINE . An OFFLINE server cannot accept and process file transfer jobs. Information tied to your server, such as server and user propertie ... |
| `TagResource` | `-` | - | `Arn`, `Tags` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Attaches a key-value pair to a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities. There is no response returned from this call. |
| `TestConnection` | `-` | - | `ConnectorId` | - | `TestConnectionResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Tests whether your SFTP connector is set up successfully. We highly recommend that you call this operation to test your ability to transfer files between local Amazon Web Services storage and a trading partner's SFTP ... |
| `TestIdentityProvider` | `-` | - | `ServerId`, `UserName` | - | `TestIdentityProviderResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | If the IdentityProviderType of a file transfer protocol-enabled server is AWS_DIRECTORY_SERVICE or API_Gateway , tests whether your identity provider is set up successfully. We highly recommend that you call this ope ... |
| `UntagResource` | `-` | - | `Arn`, `TagKeys` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Detaches a key-value pair from a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities. No response is returned from this call. |
| `UpdateAccess` | `-` | - | `ServerId`, `ExternalId` | - | `UpdateAccessResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Allows you to update parameters for the access specified in the ServerID and ExternalID parameters. |
| `UpdateHostKey` | `-` | - | `ServerId`, `HostKeyId`, `Description` | - | `UpdateHostKeyResponse` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates the description for the host key that's specified by the ServerId and HostKeyId parameters. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message | This exception is thrown when the UpdateServer is called for a file transfer protocol-enabled server that has VPC as the endpoint type and the server's VpcE ... |
| `InternalServiceError` | `structure` | Message | This exception is thrown when an error occurs in the Transfer Family service. |
| `InvalidNextTokenException` | `structure` | Message | The NextToken parameter that was passed is invalid. |
| `InvalidRequestException` | `structure` | Message | This exception is thrown when the client submits a malformed request. |
| `ResourceExistsException` | `structure` | Message, Resource, ResourceType | The requested resource does not exist, or exists in a region other than the one specified for the command. |
| `ResourceNotFoundException` | `structure` | Message, Resource, ResourceType | This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service. |
| `ServiceUnavailableException` | `structure` | Message | The request has failed because the Amazon Web ServicesTransfer Family service is not available. |
| `ThrottlingException` | `structure` | RetryAfterSeconds | The request was denied due to request throttling. |
| `CreateAccessRequest` | `structure` | HomeDirectory, HomeDirectoryType, HomeDirectoryMappings, Policy, PosixProfile, Role, ServerId, ExternalId | - |
| `CreateAccessResponse` | `structure` | ServerId, ExternalId | - |
| `DeleteAccessRequest` | `structure` | ServerId, ExternalId | - |
| `DeleteHostKeyRequest` | `structure` | ServerId, HostKeyId | - |
| `DeleteSshPublicKeyRequest` | `structure` | ServerId, SshPublicKeyId, UserName | - |
| `DescribeAccessRequest` | `structure` | ServerId, ExternalId | - |
| `DescribeAccessResponse` | `structure` | ServerId, Access | - |
| `DescribeExecutionRequest` | `structure` | ExecutionId, WorkflowId | - |
| `DescribeExecutionResponse` | `structure` | WorkflowId, Execution | - |
| `DescribeHostKeyRequest` | `structure` | ServerId, HostKeyId | - |
| `DescribeHostKeyResponse` | `structure` | HostKey | - |
| `DescribeSecurityPolicyRequest` | `structure` | SecurityPolicyName | - |
| `DescribeSecurityPolicyResponse` | `structure` | SecurityPolicy | - |
| `ImportHostKeyRequest` | `structure` | ServerId, HostKeyBody, Description, Tags | - |
| `ImportHostKeyResponse` | `structure` | ServerId, HostKeyId | - |
| `ImportSshPublicKeyRequest` | `structure` | ServerId, SshPublicKeyBody, UserName | - |
| `ImportSshPublicKeyResponse` | `structure` | ServerId, SshPublicKeyId, UserName | Identifies the user, the server they belong to, and the identifier of the SSH public key associated with that user. A user can have more than one key on eac ... |
| `ListAccessesRequest` | `structure` | MaxResults, NextToken, ServerId | - |
| `ListAccessesResponse` | `structure` | NextToken, ServerId, Accesses | - |
| `ListExecutionsRequest` | `structure` | MaxResults, NextToken, WorkflowId | - |
| `ListExecutionsResponse` | `structure` | NextToken, WorkflowId, Executions | - |
| `ListFileTransferResultsRequest` | `structure` | ConnectorId, TransferId, NextToken, MaxResults | - |
| `ListFileTransferResultsResponse` | `structure` | FileTransferResults, NextToken | - |
| `ListHostKeysRequest` | `structure` | MaxResults, NextToken, ServerId | - |
| `ListHostKeysResponse` | `structure` | NextToken, ServerId, HostKeys | - |
| `ListSecurityPoliciesRequest` | `structure` | MaxResults, NextToken | - |
| `ListSecurityPoliciesResponse` | `structure` | NextToken, SecurityPolicyNames | - |
| `ListTagsForResourceRequest` | `structure` | Arn, MaxResults, NextToken | - |
| `ListTagsForResourceResponse` | `structure` | Arn, NextToken, Tags | - |
| `SendWorkflowStepStateRequest` | `structure` | WorkflowId, ExecutionId, Token, Status | - |
| `SendWorkflowStepStateResponse` | `structure` | **empty (no members)** | - |
| `AgreementStatusType` | `enum` | ACTIVE, INACTIVE | - |
| `As2Transport` | `enum` | HTTP | - |
| `CertificateStatusType` | `enum` | ACTIVE, PENDING_ROTATION, INACTIVE | - |
| `CertificateType` | `enum` | CERTIFICATE, CERTIFICATE_WITH_PRIVATE_KEY | - |
| `CertificateUsageType` | `enum` | SIGNING, ENCRYPTION, TLS | - |
| `CompressionEnum` | `enum` | ZLIB, DISABLED | - |
| `ConnectorEgressType` | `enum` | SERVICE_MANAGED, VPC_LATTICE | - |
| `ConnectorStatus` | `enum` | ACTIVE, ERRORED, PENDING | - |
| `ConnectorsIpAddressType` | `enum` | IPV4, DUALSTACK | - |
| `CustomStepStatus` | `enum` | SUCCESS, FAILURE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
