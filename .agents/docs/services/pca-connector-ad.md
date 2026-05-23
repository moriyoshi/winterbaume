# PcaConnectorAd

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Private CA Connector for Active Directory creates a connector between Amazon Web Services Private CA and Active Directory (AD) that enables you to provision security certificates for AD signed by a private CA that you own. For more information, see Amazon Web Services Private CA Connector for Active Directory.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented PcaConnectorAd workflows in the local mock. Key resources include `ConnectorResource`, `DirectoryRegistrationResource`, `ServicePrincipalNameResource`, `TemplateGroupAccessControlEntryResource`, `TemplateResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Get`, `Update` operation families, including `ListConnectors`, `ListDirectoryRegistrations`, `ListServicePrincipalNames`, `ListTagsForResource`, `CreateConnector`, `CreateDirectoryRegistration`.

## Service Identity and Protocol

- AWS model slug: `pca-connector-ad`
- AWS SDK for Rust slug: `pcaconnectorad`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/pca-connector-ad/service/2018-05-10/pca-connector-ad-2018-05-10.json`
- SDK ID: `Pca Connector Ad`
- Endpoint prefix: `-`
- ARN namespace: `pca-connector-ad`
- CloudFormation name: `PCAConnectorAD`
- CloudTrail event source: `pca-connector-ad.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Create` (5), `Delete` (5), `Get` (5), `Update` (2), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateConnector`, `CreateDirectoryRegistration`, `CreateServicePrincipalName`, `CreateTemplate`, `CreateTemplateGroupAccessControlEntry`, `DeleteConnector`, `DeleteDirectoryRegistration`, `DeleteServicePrincipalName`, `DeleteTemplate`, `DeleteTemplateGroupAccessControlEntry`, `TagResource`, `UntagResource`, `UpdateTemplate`, `UpdateTemplateGroupAccessControlEntry`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetConnector`, `GetDirectoryRegistration`, `GetServicePrincipalName`, `GetTemplate`, `GetTemplateGroupAccessControlEntry`, `ListConnectors`, `ListDirectoryRegistrations`, `ListServicePrincipalNames`, `ListTagsForResource`, `ListTemplateGroupAccessControlEntries`, `ListTemplates`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 25 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ConnectorResource` | `ConnectorArn` | create: `CreateConnector`; read: `GetConnector`; delete: `DeleteConnector`; list: `ListConnectors` | - | - |
| `DirectoryRegistrationResource` | `DirectoryRegistrationArn` | create: `CreateDirectoryRegistration`; read: `GetDirectoryRegistration`; delete: `DeleteDirectoryRegistration`; list: `ListDirectoryRegistrations` | - | - |
| `ServicePrincipalNameResource` | `ConnectorArn`, `DirectoryRegistrationArn` | put: `CreateServicePrincipalName`; read: `GetServicePrincipalName`; delete: `DeleteServicePrincipalName`; list: `ListServicePrincipalNames` | - | - |
| `TemplateGroupAccessControlEntryResource` | `GroupSecurityIdentifier`, `TemplateArn` | put: `CreateTemplateGroupAccessControlEntry`; read: `GetTemplateGroupAccessControlEntry`; update: `UpdateTemplateGroupAccessControlEntry`; delete: `DeleteTemplateGroupAccessControlEntry`; list: `ListTemplateGroupAccessControlEntries` | - | - |
| `TemplateResource` | `TemplateArn` | create: `CreateTemplate`; read: `GetTemplate`; update: `UpdateTemplate`; delete: `DeleteTemplate`; list: `ListTemplates` | - | - |
## Operation Groups

### List

- Operations: `ListConnectors`, `ListDirectoryRegistrations`, `ListServicePrincipalNames`, `ListTagsForResource`, `ListTemplateGroupAccessControlEntries`, `ListTemplates`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `ConnectorArn`, `DirectoryRegistrationArn`, `ResourceArn`, `TemplateArn`

### Create

- Operations: `CreateConnector`, `CreateDirectoryRegistration`, `CreateServicePrincipalName`, `CreateTemplate`, `CreateTemplateGroupAccessControlEntry`
- Traits: `idempotency-token` (5), `idempotent` (2)
- Common required input members in this group: `AccessRights`, `CertificateAuthorityArn`, `ConnectorArn`, `Definition`, `DirectoryId`, `DirectoryRegistrationArn`, `GroupDisplayName`, `GroupSecurityIdentifier`, `Name`, `TemplateArn`, `VpcInformation`

### Delete

- Operations: `DeleteConnector`, `DeleteDirectoryRegistration`, `DeleteServicePrincipalName`, `DeleteTemplate`, `DeleteTemplateGroupAccessControlEntry`
- Traits: `idempotent` (5)
- Common required input members in this group: `ConnectorArn`, `DirectoryRegistrationArn`, `GroupSecurityIdentifier`, `TemplateArn`

### Get

- Operations: `GetConnector`, `GetDirectoryRegistration`, `GetServicePrincipalName`, `GetTemplate`, `GetTemplateGroupAccessControlEntry`
- Traits: `readonly` (5)
- Common required input members in this group: `ConnectorArn`, `DirectoryRegistrationArn`, `GroupSecurityIdentifier`, `TemplateArn`

### Update

- Operations: `UpdateTemplate`, `UpdateTemplateGroupAccessControlEntry`
- Common required input members in this group: `GroupSecurityIdentifier`, `TemplateArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateConnector` | `POST /connectors` | `idempotency-token` | `CertificateAuthorityArn`, `DirectoryId`, `VpcInformation` | `ClientToken` | `CreateConnectorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a connector between Amazon Web Services Private CA and an Active Directory. You must specify the private CA, directory ID, and security groups. |
| `CreateDirectoryRegistration` | `POST /directoryRegistrations` | `idempotency-token` | `DirectoryId` | `ClientToken` | `CreateDirectoryRegistrationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a directory registration that authorizes communication between Amazon Web Services Private CA and an Active Directory |
| `CreateServicePrincipalName` | `POST /directoryRegistrations/{DirectoryRegistrationArn}/servicePrincipalNames/{ConnectorArn}` | `idempotent`, `idempotency-token` | `ConnectorArn`, `DirectoryRegistrationArn` | `ClientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a service principal name (SPN) for the service account in Active Directory. Kerberos authentication uses SPNs to associate a service instance with a service sign-in account. |
| `CreateTemplate` | `POST /templates` | `idempotency-token` | `ConnectorArn`, `Definition`, `Name` | `ClientToken` | `CreateTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Active Directory compatible certificate template. The connectors issues certificates using these templates based on the requester’s Active Directory group membership. |
| `CreateTemplateGroupAccessControlEntry` | `POST /templates/{TemplateArn}/accessControlEntries` | `idempotent`, `idempotency-token` | `AccessRights`, `GroupDisplayName`, `GroupSecurityIdentifier`, `TemplateArn` | `ClientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a group access control entry. Allow or deny Active Directory groups from enrolling and/or autoenrolling with the template based on the group security identifiers (SIDs). |
| `DeleteConnector` | `DELETE /connectors/{ConnectorArn}` | `idempotent` | `ConnectorArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a connector for Active Directory. You must provide the Amazon Resource Name (ARN) of the connector that you want to delete. |
| `DeleteDirectoryRegistration` | `DELETE /directoryRegistrations/{DirectoryRegistrationArn}` | `idempotent` | `DirectoryRegistrationArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a directory registration. Deleting a directory registration deauthorizes Amazon Web Services Private CA with the directory. |
| `DeleteServicePrincipalName` | `DELETE /directoryRegistrations/{DirectoryRegistrationArn}/servicePrincipalNames/{ConnectorArn}` | `idempotent` | `ConnectorArn`, `DirectoryRegistrationArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the service principal name (SPN) used by a connector to authenticate with your Active Directory. |
| `DeleteTemplate` | `DELETE /templates/{TemplateArn}` | `idempotent` | `TemplateArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a template. Certificates issued using the template are still valid until they are revoked or expired. |
| `DeleteTemplateGroupAccessControlEntry` | `DELETE /templates/{TemplateArn}/accessControlEntries/{GroupSecurityIdentifier}` | `idempotent` | `GroupSecurityIdentifier`, `TemplateArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a group access control entry. |
| `GetConnector` | `GET /connectors/{ConnectorArn}` | `readonly` | `ConnectorArn` | - | `GetConnectorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists information about your connector. You specify the connector on input by its ARN (Amazon Resource Name). |
| `GetDirectoryRegistration` | `GET /directoryRegistrations/{DirectoryRegistrationArn}` | `readonly` | `DirectoryRegistrationArn` | - | `GetDirectoryRegistrationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A structure that contains information about your directory registration. |
| `GetServicePrincipalName` | `GET /directoryRegistrations/{DirectoryRegistrationArn}/servicePrincipalNames/{ConnectorArn}` | `readonly` | `ConnectorArn`, `DirectoryRegistrationArn` | - | `GetServicePrincipalNameResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the service principal name that the connector uses to authenticate with Active Directory. |
| `GetTemplate` | `GET /templates/{TemplateArn}` | `readonly` | `TemplateArn` | - | `GetTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a certificate template that the connector uses to issue certificates from a private CA. |
| `GetTemplateGroupAccessControlEntry` | `GET /templates/{TemplateArn}/accessControlEntries/{GroupSecurityIdentifier}` | `readonly` | `GroupSecurityIdentifier`, `TemplateArn` | - | `GetTemplateGroupAccessControlEntryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the group access control entries for a template. |
| `ListConnectors` | `GET /connectors` | `readonly`, `paginated` | - | - | `ListConnectorsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the connectors that you created by using the https://docs.aws.amazon.com/pca-connector-ad/latest/APIReference/API_CreateConnector action. |
| `ListDirectoryRegistrations` | `GET /directoryRegistrations` | `readonly`, `paginated` | - | - | `ListDirectoryRegistrationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the directory registrations that you created by using the https://docs.aws.amazon.com/pca-connector-ad/latest/APIReference/API_CreateDirectoryRegistration action. |
| `ListServicePrincipalNames` | `GET /directoryRegistrations/{DirectoryRegistrationArn}/servicePrincipalNames` | `readonly`, `paginated` | `DirectoryRegistrationArn` | - | `ListServicePrincipalNamesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the service principal names that the connector uses to authenticate with Active Directory. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags, if any, that are associated with your resource. |
| `ListTemplateGroupAccessControlEntries` | `GET /templates/{TemplateArn}/accessControlEntries` | `readonly`, `paginated` | `TemplateArn` | - | `ListTemplateGroupAccessControlEntriesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists group access control entries you created. |
| `ListTemplates` | `GET /templates` | `readonly`, `paginated` | `ConnectorArn` | - | `ListTemplatesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the templates, if any, that are associated with a connector. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds one or more tags to your resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from your resource. |
| `UpdateTemplate` | `PATCH /templates/{TemplateArn}` | - | `TemplateArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update template configuration to define the information included in certificates. |
| `UpdateTemplateGroupAccessControlEntry` | `PATCH /templates/{TemplateArn}/accessControlEntries/{GroupSecurityIdentifier}` | - | `GroupSecurityIdentifier`, `TemplateArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a group access control entry you created using CreateTemplateGroupAccessControlEntry. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You can receive this error if you attempt to create a resource share when you don't have the required permissions. |
| `InternalServerException` | `structure` | `Message` | The request processing has failed because of an unknown error, exception or failure with an internal server. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `ServiceCode` | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | `Message`, `Reason` | An input validation error occurred. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The operation tried to access a nonexistent resource. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | This request cannot be completed for one of the following reasons because the requested resource was being concurrently modified by another request. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode` | Request would cause a service quota to be exceeded. |
| `CreateConnectorRequest` | `structure` | `CertificateAuthorityArn`, `ClientToken`, `DirectoryId`, `Tags`, `VpcInformation` | - |
| `CreateConnectorResponse` | `structure` | `ConnectorArn` | - |
| `CreateDirectoryRegistrationRequest` | `structure` | `ClientToken`, `DirectoryId`, `Tags` | - |
| `CreateDirectoryRegistrationResponse` | `structure` | `DirectoryRegistrationArn` | - |
| `CreateServicePrincipalNameRequest` | `structure` | `ClientToken`, `ConnectorArn`, `DirectoryRegistrationArn` | - |
| `CreateTemplateRequest` | `structure` | `ClientToken`, `ConnectorArn`, `Definition`, `Name`, `Tags` | - |
| `CreateTemplateResponse` | `structure` | `TemplateArn` | - |
| `CreateTemplateGroupAccessControlEntryRequest` | `structure` | `AccessRights`, `ClientToken`, `GroupDisplayName`, `GroupSecurityIdentifier`, `TemplateArn` | - |
| `DeleteConnectorRequest` | `structure` | `ConnectorArn` | - |
| `DeleteDirectoryRegistrationRequest` | `structure` | `DirectoryRegistrationArn` | - |
| `DeleteServicePrincipalNameRequest` | `structure` | `ConnectorArn`, `DirectoryRegistrationArn` | - |
| `DeleteTemplateRequest` | `structure` | `TemplateArn` | - |
| `DeleteTemplateGroupAccessControlEntryRequest` | `structure` | `GroupSecurityIdentifier`, `TemplateArn` | - |
| `GetConnectorRequest` | `structure` | `ConnectorArn` | - |
| `GetConnectorResponse` | `structure` | `Connector` | - |
| `GetDirectoryRegistrationRequest` | `structure` | `DirectoryRegistrationArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
