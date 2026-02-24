# Amazon WorkSpaces Web

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon WorkSpaces Secure Browser is a low cost, fully managed WorkSpace built specifically to facilitate secure, web-based workloads. WorkSpaces Secure Browser makes it easy for customers to safely provide their employees with access to internal websites and SaaS web applications without the administrative burden of appliances or specialized client software. WorkSpaces Secure Browser provides simple policy tools tailored for user interactions, while offloading common tasks like capacity management, scaling, and maintaining browser images.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon WorkSpaces Web where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon WorkSpaces Web by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon WorkSpaces Web workflows in the local mock. Key resources include `BrowserSettingsResource`, `DataProtectionSettingsResource`, `IdentityProviderResource`, `IpAccessSettingsResource`, `NetworkSettingsResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetBrowserSettings`, `GetDataProtectionSettings`, `GetIdentityProvider`, `GetIpAccessSettings`, `ListBrowserSettings`, `ListDataProtectionSettings`.

## Service Identity and Protocol

- AWS model slug: `workspaces-web`
- AWS SDK for Rust slug: `workspacesweb`
- Model version: `2020-07-08`
- Model file: `vendor/api-models-aws/models/workspaces-web/service/2020-07-08/workspaces-web-2020-07-08.json`
- SDK ID: `WorkSpaces Web`
- Endpoint prefix: `workspaces-web`
- ARN namespace: `workspaces-web`
- CloudFormation name: `WorkSpacesWeb`
- CloudTrail event source: `workspaces-web.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (13), `List` (13), `Create` (10), `Delete` (10), `Update` (10), `Associate` (8), `Disassociate` (8), `Expire` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateBrowserSettings`, `AssociateDataProtectionSettings`, `AssociateIpAccessSettings`, `AssociateNetworkSettings`, `AssociateSessionLogger`, `AssociateTrustStore`, `AssociateUserAccessLoggingSettings`, `AssociateUserSettings`, `CreateBrowserSettings`, `CreateDataProtectionSettings`, `CreateIdentityProvider`, `CreateIpAccessSettings`, `CreateNetworkSettings`, `CreatePortal`, `CreateSessionLogger`, `CreateTrustStore`, `CreateUserAccessLoggingSettings`, `CreateUserSettings`, `DeleteBrowserSettings`, `DeleteDataProtectionSettings`, ... (+28).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBrowserSettings`, `GetDataProtectionSettings`, `GetIdentityProvider`, `GetIpAccessSettings`, `GetNetworkSettings`, `GetPortal`, `GetPortalServiceProviderMetadata`, `GetSession`, `GetSessionLogger`, `GetTrustStore`, `GetTrustStoreCertificate`, `GetUserAccessLoggingSettings`, `GetUserSettings`, `ListBrowserSettings`, `ListDataProtectionSettings`, `ListIdentityProviders`, `ListIpAccessSettings`, `ListNetworkSettings`, `ListPortals`, `ListSessionLoggers`, ... (+6).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 48 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 75 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BrowserSettingsResource` | `browserSettingsArn` | create: `CreateBrowserSettings`; read: `GetBrowserSettings`; update: `UpdateBrowserSettings`; delete: `DeleteBrowserSettings`; list: `ListBrowserSettings` | - | - |
| `DataProtectionSettingsResource` | `dataProtectionSettingsArn` | create: `CreateDataProtectionSettings`; read: `GetDataProtectionSettings`; update: `UpdateDataProtectionSettings`; delete: `DeleteDataProtectionSettings`; list: `ListDataProtectionSettings` | - | - |
| `IdentityProviderResource` | `identityProviderArn` | create: `CreateIdentityProvider`; read: `GetIdentityProvider`; update: `UpdateIdentityProvider`; delete: `DeleteIdentityProvider`; list: `ListIdentityProviders` | - | - |
| `IpAccessSettingsResource` | `ipAccessSettingsArn` | create: `CreateIpAccessSettings`; read: `GetIpAccessSettings`; update: `UpdateIpAccessSettings`; delete: `DeleteIpAccessSettings`; list: `ListIpAccessSettings` | - | - |
| `NetworkSettingsResource` | `networkSettingsArn` | create: `CreateNetworkSettings`; read: `GetNetworkSettings`; update: `UpdateNetworkSettings`; delete: `DeleteNetworkSettings`; list: `ListNetworkSettings` | - | - |
| `PortalResource` | `portalArn` | create: `CreatePortal`; read: `GetPortal`; update: `UpdatePortal`; delete: `DeletePortal`; list: `ListPortals` | `AssociateBrowserSettings`, `AssociateDataProtectionSettings`, `AssociateIpAccessSettings`, `AssociateNetworkSettings`, `AssociateSessionLogger`, `AssociateTrustStore`, `AssociateUserAccessLoggingSettings`, `AssociateUserSettings`, `DisassociateBrowserSettings`, `DisassociateDataProtectionSettings`, ... (+7) | - |
| `SessionLoggerResource` | `sessionLoggerArn` | create: `CreateSessionLogger`; read: `GetSessionLogger`; update: `UpdateSessionLogger`; delete: `DeleteSessionLogger`; list: `ListSessionLoggers` | - | - |
| `TrustStoreResource` | `trustStoreArn` | create: `CreateTrustStore`; read: `GetTrustStore`; update: `UpdateTrustStore`; delete: `DeleteTrustStore`; list: `ListTrustStores` | `GetTrustStoreCertificate`, `ListTrustStoreCertificates` | - |
| `UserAccessLoggingSettingsResource` | `userAccessLoggingSettingsArn` | create: `CreateUserAccessLoggingSettings`; read: `GetUserAccessLoggingSettings`; update: `UpdateUserAccessLoggingSettings`; delete: `DeleteUserAccessLoggingSettings`; list: `ListUserAccessLoggingSettings` | - | - |
| `UserSettingsResource` | `userSettingsArn` | create: `CreateUserSettings`; read: `GetUserSettings`; update: `UpdateUserSettings`; delete: `DeleteUserSettings`; list: `ListUserSettings` | - | - |

## Current Network Resource Stub Semantics

WorkSpaces Web currently stores portal networking as portal/resource metadata.

- Portal and network settings shapes include VPC ID, subnet IDs, and security group IDs.
- Current implemented state records those values only where the surrounding resource handler persists the portal or settings object.
- Browser session lifecycle does not create VPC endpoints or ENIs.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetBrowserSettings`, `GetDataProtectionSettings`, `GetIdentityProvider`, `GetIpAccessSettings`, `GetNetworkSettings`, `GetPortal`, `GetPortalServiceProviderMetadata`, `GetSession`, `GetSessionLogger`, `GetTrustStore`, `GetTrustStoreCertificate`, `GetUserAccessLoggingSettings`, `GetUserSettings`
- Traits: `readonly` (13)
- Common required input members in this group: `browserSettingsArn`, `dataProtectionSettingsArn`, `identityProviderArn`, `ipAccessSettingsArn`, `networkSettingsArn`, `portalArn`, `portalId`, `sessionId`, `sessionLoggerArn`, `thumbprint`, `trustStoreArn`, `userAccessLoggingSettingsArn`, `userSettingsArn`

### List

- Operations: `ListBrowserSettings`, `ListDataProtectionSettings`, `ListIdentityProviders`, `ListIpAccessSettings`, `ListNetworkSettings`, `ListPortals`, `ListSessionLoggers`, `ListSessions`, `ListTagsForResource`, `ListTrustStoreCertificates`, `ListTrustStores`, `ListUserAccessLoggingSettings`, `ListUserSettings`
- Traits: `paginated` (12), `readonly` (13)
- Common required input members in this group: `portalArn`, `portalId`, `resourceArn`, `trustStoreArn`

### Create

- Operations: `CreateBrowserSettings`, `CreateDataProtectionSettings`, `CreateIdentityProvider`, `CreateIpAccessSettings`, `CreateNetworkSettings`, `CreatePortal`, `CreateSessionLogger`, `CreateTrustStore`, `CreateUserAccessLoggingSettings`, `CreateUserSettings`
- Traits: `idempotency-token` (10)
- Common required input members in this group: `certificateList`, `copyAllowed`, `downloadAllowed`, `eventFilter`, `identityProviderDetails`, `identityProviderName`, `identityProviderType`, `ipRules`, `kinesisStreamArn`, `logConfiguration`, `pasteAllowed`, `portalArn`, `printAllowed`, `securityGroupIds`, `subnetIds`, `uploadAllowed`, `vpcId`

### Delete

- Operations: `DeleteBrowserSettings`, `DeleteDataProtectionSettings`, `DeleteIdentityProvider`, `DeleteIpAccessSettings`, `DeleteNetworkSettings`, `DeletePortal`, `DeleteSessionLogger`, `DeleteTrustStore`, `DeleteUserAccessLoggingSettings`, `DeleteUserSettings`
- Traits: `idempotent` (10)
- Common required input members in this group: `browserSettingsArn`, `dataProtectionSettingsArn`, `identityProviderArn`, `ipAccessSettingsArn`, `networkSettingsArn`, `portalArn`, `sessionLoggerArn`, `trustStoreArn`, `userAccessLoggingSettingsArn`, `userSettingsArn`

### Update

- Operations: `UpdateBrowserSettings`, `UpdateDataProtectionSettings`, `UpdateIdentityProvider`, `UpdateIpAccessSettings`, `UpdateNetworkSettings`, `UpdatePortal`, `UpdateSessionLogger`, `UpdateTrustStore`, `UpdateUserAccessLoggingSettings`, `UpdateUserSettings`
- Traits: `idempotency-token` (8), `idempotent` (1)
- Common required input members in this group: `browserSettingsArn`, `dataProtectionSettingsArn`, `identityProviderArn`, `ipAccessSettingsArn`, `networkSettingsArn`, `portalArn`, `sessionLoggerArn`, `trustStoreArn`, `userAccessLoggingSettingsArn`, `userSettingsArn`

### Associate

- Operations: `AssociateBrowserSettings`, `AssociateDataProtectionSettings`, `AssociateIpAccessSettings`, `AssociateNetworkSettings`, `AssociateSessionLogger`, `AssociateTrustStore`, `AssociateUserAccessLoggingSettings`, `AssociateUserSettings`
- Traits: `idempotent` (8)
- Common required input members in this group: `browserSettingsArn`, `dataProtectionSettingsArn`, `ipAccessSettingsArn`, `networkSettingsArn`, `portalArn`, `sessionLoggerArn`, `trustStoreArn`, `userAccessLoggingSettingsArn`, `userSettingsArn`

### Disassociate

- Operations: `DisassociateBrowserSettings`, `DisassociateDataProtectionSettings`, `DisassociateIpAccessSettings`, `DisassociateNetworkSettings`, `DisassociateSessionLogger`, `DisassociateTrustStore`, `DisassociateUserAccessLoggingSettings`, `DisassociateUserSettings`
- Traits: `idempotent` (8)
- Common required input members in this group: `portalArn`

### Expire

- Operations: `ExpireSession`
- Traits: `idempotent` (1)
- Common required input members in this group: `portalId`, `sessionId`

### Tag

- Operations: `TagResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateBrowserSettings` | `PUT /portals/{portalArn+}/browserSettings` | `idempotent` | `browserSettingsArn`, `portalArn` | - | `AssociateBrowserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a browser settings resource with a web portal. |
| `AssociateDataProtectionSettings` | `PUT /portals/{portalArn+}/dataProtectionSettings` | `idempotent` | `dataProtectionSettingsArn`, `portalArn` | - | `AssociateDataProtectionSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a data protection settings resource with a web portal. |
| `AssociateIpAccessSettings` | `PUT /portals/{portalArn+}/ipAccessSettings` | `idempotent` | `ipAccessSettingsArn`, `portalArn` | - | `AssociateIpAccessSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates an IP access settings resource with a web portal. |
| `AssociateNetworkSettings` | `PUT /portals/{portalArn+}/networkSettings` | `idempotent` | `networkSettingsArn`, `portalArn` | - | `AssociateNetworkSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a network settings resource with a web portal. |
| `AssociateSessionLogger` | `PUT /portals/{portalArn+}/sessionLogger` | `idempotent` | `portalArn`, `sessionLoggerArn` | - | `AssociateSessionLoggerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a session logger with a portal. |
| `AssociateTrustStore` | `PUT /portals/{portalArn+}/trustStores` | `idempotent` | `portalArn`, `trustStoreArn` | - | `AssociateTrustStoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a trust store with a web portal. |
| `AssociateUserAccessLoggingSettings` | `PUT /portals/{portalArn+}/userAccessLoggingSettings` | `idempotent` | `portalArn`, `userAccessLoggingSettingsArn` | - | `AssociateUserAccessLoggingSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a user access logging settings resource with a web portal. |
| `AssociateUserSettings` | `PUT /portals/{portalArn+}/userSettings` | `idempotent` | `portalArn`, `userSettingsArn` | - | `AssociateUserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a user settings resource with a web portal. |
| `CreateBrowserSettings` | `POST /browserSettings` | `idempotency-token` | - | `clientToken` | `CreateBrowserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a browser settings resource that can be associated with a web portal. Once associated with a web portal, browser settings control how the browser will behave once a user starts a streaming session for the web portal. |
| `CreateDataProtectionSettings` | `POST /dataProtectionSettings` | `idempotency-token` | - | `clientToken` | `CreateDataProtectionSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a data protection settings resource that can be associated with a web portal. |
| `CreateIdentityProvider` | `POST /identityProviders` | `idempotency-token` | `identityProviderDetails`, `identityProviderName`, `identityProviderType`, `portalArn` | `clientToken` | `CreateIdentityProviderResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an identity provider resource that is then associated with a web portal. |
| `CreateIpAccessSettings` | `POST /ipAccessSettings` | `idempotency-token` | `ipRules` | `clientToken` | `CreateIpAccessSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an IP access settings resource that can be associated with a web portal. |
| `CreateNetworkSettings` | `POST /networkSettings` | `idempotency-token` | `securityGroupIds`, `subnetIds`, `vpcId` | `clientToken` | `CreateNetworkSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a network settings resource that can be associated with a web portal. Once associated with a web portal, network settings define how streaming instances will connect with your specified VPC. |
| `CreatePortal` | `POST /portals` | `idempotency-token` | - | `clientToken` | `CreatePortalResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a web portal. |
| `CreateSessionLogger` | `POST /sessionLoggers` | `idempotency-token` | `eventFilter`, `logConfiguration` | `clientToken` | `CreateSessionLoggerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a session logger. |
| `CreateTrustStore` | `POST /trustStores` | `idempotency-token` | `certificateList` | `clientToken` | `CreateTrustStoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a trust store that can be associated with a web portal. A trust store contains certificate authority (CA) certificates. |
| `CreateUserAccessLoggingSettings` | `POST /userAccessLoggingSettings` | `idempotency-token` | `kinesisStreamArn` | `clientToken` | `CreateUserAccessLoggingSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a user access logging settings resource that can be associated with a web portal. |
| `CreateUserSettings` | `POST /userSettings` | `idempotency-token` | `copyAllowed`, `downloadAllowed`, `pasteAllowed`, `printAllowed`, `uploadAllowed` | `clientToken` | `CreateUserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a user settings resource that can be associated with a web portal. Once associated with a web portal, user settings control how users can transfer data between a streaming session and the their local devices. |
| `DeleteBrowserSettings` | `DELETE /browserSettings/{browserSettingsArn+}` | `idempotent` | `browserSettingsArn` | - | `DeleteBrowserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes browser settings. |
| `DeleteDataProtectionSettings` | `DELETE /dataProtectionSettings/{dataProtectionSettingsArn+}` | `idempotent` | `dataProtectionSettingsArn` | - | `DeleteDataProtectionSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes data protection settings. |
| `DeleteIdentityProvider` | `DELETE /identityProviders/{identityProviderArn+}` | `idempotent` | `identityProviderArn` | - | `DeleteIdentityProviderResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the identity provider. |
| `DeleteIpAccessSettings` | `DELETE /ipAccessSettings/{ipAccessSettingsArn+}` | `idempotent` | `ipAccessSettingsArn` | - | `DeleteIpAccessSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes IP access settings. |
| `DeleteNetworkSettings` | `DELETE /networkSettings/{networkSettingsArn+}` | `idempotent` | `networkSettingsArn` | - | `DeleteNetworkSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes network settings. |
| `DeletePortal` | `DELETE /portals/{portalArn+}` | `idempotent` | `portalArn` | - | `DeletePortalResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a web portal. |
| `DeleteSessionLogger` | `DELETE /sessionLoggers/{sessionLoggerArn+}` | `idempotent` | `sessionLoggerArn` | - | `DeleteSessionLoggerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a session logger resource. |
| `DeleteTrustStore` | `DELETE /trustStores/{trustStoreArn+}` | `idempotent` | `trustStoreArn` | - | `DeleteTrustStoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the trust store. |
| `DeleteUserAccessLoggingSettings` | `DELETE /userAccessLoggingSettings/{userAccessLoggingSettingsArn+}` | `idempotent` | `userAccessLoggingSettingsArn` | - | `DeleteUserAccessLoggingSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes user access logging settings. |
| `DeleteUserSettings` | `DELETE /userSettings/{userSettingsArn+}` | `idempotent` | `userSettingsArn` | - | `DeleteUserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes user settings. |
| `DisassociateBrowserSettings` | `DELETE /portals/{portalArn+}/browserSettings` | `idempotent` | `portalArn` | - | `DisassociateBrowserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates browser settings from a web portal. |
| `DisassociateDataProtectionSettings` | `DELETE /portals/{portalArn+}/dataProtectionSettings` | `idempotent` | `portalArn` | - | `DisassociateDataProtectionSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates data protection settings from a web portal. |
| `DisassociateIpAccessSettings` | `DELETE /portals/{portalArn+}/ipAccessSettings` | `idempotent` | `portalArn` | - | `DisassociateIpAccessSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates IP access settings from a web portal. |
| `DisassociateNetworkSettings` | `DELETE /portals/{portalArn+}/networkSettings` | `idempotent` | `portalArn` | - | `DisassociateNetworkSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates network settings from a web portal. |
| `DisassociateSessionLogger` | `DELETE /portals/{portalArn+}/sessionLogger` | `idempotent` | `portalArn` | - | `DisassociateSessionLoggerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a session logger from a portal. |
| `DisassociateTrustStore` | `DELETE /portals/{portalArn+}/trustStores` | `idempotent` | `portalArn` | - | `DisassociateTrustStoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a trust store from a web portal. |
| `DisassociateUserAccessLoggingSettings` | `DELETE /portals/{portalArn+}/userAccessLoggingSettings` | `idempotent` | `portalArn` | - | `DisassociateUserAccessLoggingSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates user access logging settings from a web portal. |
| `DisassociateUserSettings` | `DELETE /portals/{portalArn+}/userSettings` | `idempotent` | `portalArn` | - | `DisassociateUserSettingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates user settings from a web portal. |
| `ExpireSession` | `DELETE /portals/{portalId}/sessions/{sessionId}` | `idempotent` | `portalId`, `sessionId` | - | `ExpireSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Expires an active secure browser session. |
| `GetBrowserSettings` | `GET /browserSettings/{browserSettingsArn+}` | `readonly` | `browserSettingsArn` | - | `GetBrowserSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets browser settings. |
| `GetDataProtectionSettings` | `GET /dataProtectionSettings/{dataProtectionSettingsArn+}` | `readonly` | `dataProtectionSettingsArn` | - | `GetDataProtectionSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the data protection settings. |
| `GetIdentityProvider` | `GET /identityProviders/{identityProviderArn+}` | `readonly` | `identityProviderArn` | - | `GetIdentityProviderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the identity provider. |
| `GetIpAccessSettings` | `GET /ipAccessSettings/{ipAccessSettingsArn+}` | `readonly` | `ipAccessSettingsArn` | - | `GetIpAccessSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the IP access settings. |
| `GetNetworkSettings` | `GET /networkSettings/{networkSettingsArn+}` | `readonly` | `networkSettingsArn` | - | `GetNetworkSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the network settings. |
| `GetPortal` | `GET /portals/{portalArn+}` | `readonly` | `portalArn` | - | `GetPortalResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the web portal. |
| `GetPortalServiceProviderMetadata` | `GET /portalIdp/{portalArn+}` | `readonly` | `portalArn` | - | `GetPortalServiceProviderMetadataResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the service provider metadata. |
| `GetSession` | `GET /portals/{portalId}/sessions/{sessionId}` | `readonly` | `portalId`, `sessionId` | - | `GetSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information for a secure browser session. |
| `GetSessionLogger` | `GET /sessionLoggers/{sessionLoggerArn+}` | `readonly` | `sessionLoggerArn` | - | `GetSessionLoggerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets details about a specific session logger resource. |
| `GetTrustStore` | `GET /trustStores/{trustStoreArn+}` | `readonly` | `trustStoreArn` | - | `GetTrustStoreResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the trust store. |
| `GetTrustStoreCertificate` | `GET /trustStores/{trustStoreArn+}/certificate` | `readonly` | `thumbprint`, `trustStoreArn` | - | `GetTrustStoreCertificateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the trust store certificate. |
| `GetUserAccessLoggingSettings` | `GET /userAccessLoggingSettings/{userAccessLoggingSettingsArn+}` | `readonly` | `userAccessLoggingSettingsArn` | - | `GetUserAccessLoggingSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets user access logging settings. |
| `GetUserSettings` | `GET /userSettings/{userSettingsArn+}` | `readonly` | `userSettingsArn` | - | `GetUserSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets user settings. |
| `ListBrowserSettings` | `GET /browserSettings` | `readonly`, `paginated` | - | - | `ListBrowserSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of browser settings. |
| `ListDataProtectionSettings` | `GET /dataProtectionSettings` | `readonly`, `paginated` | - | - | `ListDataProtectionSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of data protection settings. |
| `ListIdentityProviders` | `GET /portals/{portalArn+}/identityProviders` | `readonly`, `paginated` | `portalArn` | - | `ListIdentityProvidersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of identity providers for a specific web portal. |
| `ListIpAccessSettings` | `GET /ipAccessSettings` | `readonly`, `paginated` | - | - | `ListIpAccessSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of IP access settings. |
| `ListNetworkSettings` | `GET /networkSettings` | `readonly`, `paginated` | - | - | `ListNetworkSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of network settings. |
| `ListPortals` | `GET /portals` | `readonly`, `paginated` | - | - | `ListPortalsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list or web portals. |
| `ListSessionLoggers` | `GET /sessionLoggers` | `readonly`, `paginated` | - | - | `ListSessionLoggersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available session logger resources. |
| `ListSessions` | `GET /portals/{portalId}/sessions` | `readonly`, `paginated` | `portalId` | - | `ListSessionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists information for multiple secure browser sessions from a specific portal. |
| `ListTagsForResource` | `GET /tags/{resourceArn+}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of tags for a resource. |
| `ListTrustStoreCertificates` | `GET /trustStores/{trustStoreArn+}/certificates` | `readonly`, `paginated` | `trustStoreArn` | - | `ListTrustStoreCertificatesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of trust store certificates. |
| `ListTrustStores` | `GET /trustStores` | `readonly`, `paginated` | - | - | `ListTrustStoresResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of trust stores. |
| `ListUserAccessLoggingSettings` | `GET /userAccessLoggingSettings` | `readonly`, `paginated` | - | - | `ListUserAccessLoggingSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of user access logging settings. |
| `ListUserSettings` | `GET /userSettings` | `readonly`, `paginated` | - | - | `ListUserSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of user settings. |
| `TagResource` | `POST /tags/{resourceArn+}` | `idempotency-token` | `resourceArn`, `tags` | `clientToken` | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Adds or overwrites one or more tags for the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn+}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from the specified resource. |
| `UpdateBrowserSettings` | `PATCH /browserSettings/{browserSettingsArn+}` | `idempotency-token` | `browserSettingsArn` | `clientToken` | `UpdateBrowserSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates browser settings. |
| `UpdateDataProtectionSettings` | `PATCH /dataProtectionSettings/{dataProtectionSettingsArn+}` | `idempotency-token` | `dataProtectionSettingsArn` | `clientToken` | `UpdateDataProtectionSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates data protection settings. |
| `UpdateIdentityProvider` | `PATCH /identityProviders/{identityProviderArn+}` | `idempotency-token` | `identityProviderArn` | `clientToken` | `UpdateIdentityProviderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the identity provider. |
| `UpdateIpAccessSettings` | `PATCH /ipAccessSettings/{ipAccessSettingsArn+}` | `idempotency-token` | `ipAccessSettingsArn` | `clientToken` | `UpdateIpAccessSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates IP access settings. |
| `UpdateNetworkSettings` | `PATCH /networkSettings/{networkSettingsArn+}` | `idempotency-token` | `networkSettingsArn` | `clientToken` | `UpdateNetworkSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates network settings. |
| `UpdatePortal` | `PUT /portals/{portalArn+}` | `idempotent` | `portalArn` | - | `UpdatePortalResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a web portal. |
| `UpdateSessionLogger` | `POST /sessionLoggers/{sessionLoggerArn+}` | - | `sessionLoggerArn` | - | `UpdateSessionLoggerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the details of a session logger. |
| `UpdateTrustStore` | `PATCH /trustStores/{trustStoreArn+}` | `idempotency-token` | `trustStoreArn` | `clientToken` | `UpdateTrustStoreResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the trust store. |
| `UpdateUserAccessLoggingSettings` | `PATCH /userAccessLoggingSettings/{userAccessLoggingSettingsArn+}` | `idempotency-token` | `userAccessLoggingSettingsArn` | `clientToken` | `UpdateUserAccessLoggingSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the user access logging settings. |
| `UpdateUserSettings` | `PATCH /userSettings/{userSettingsArn+}` | `idempotency-token` | `userSettingsArn` | `clientToken` | `UpdateUserSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the user settings. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | Access is denied. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | There is an internal server error. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | There is a throttling error. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | There is a validation error. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The resource cannot be found. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | There is a conflict. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The service quota has been exceeded. |
| `AssociateBrowserSettingsRequest` | `structure` | `browserSettingsArn`, `portalArn` | - |
| `AssociateBrowserSettingsResponse` | `structure` | `browserSettingsArn`, `portalArn` | - |
| `AssociateDataProtectionSettingsRequest` | `structure` | `dataProtectionSettingsArn`, `portalArn` | - |
| `AssociateDataProtectionSettingsResponse` | `structure` | `dataProtectionSettingsArn`, `portalArn` | - |
| `AssociateIpAccessSettingsRequest` | `structure` | `ipAccessSettingsArn`, `portalArn` | - |
| `AssociateIpAccessSettingsResponse` | `structure` | `ipAccessSettingsArn`, `portalArn` | - |
| `AssociateNetworkSettingsRequest` | `structure` | `networkSettingsArn`, `portalArn` | - |
| `AssociateNetworkSettingsResponse` | `structure` | `networkSettingsArn`, `portalArn` | - |
| `AssociateSessionLoggerRequest` | `structure` | `portalArn`, `sessionLoggerArn` | - |
| `AssociateSessionLoggerResponse` | `structure` | `portalArn`, `sessionLoggerArn` | - |
| `AssociateTrustStoreRequest` | `structure` | `portalArn`, `trustStoreArn` | - |
| `AssociateTrustStoreResponse` | `structure` | `portalArn`, `trustStoreArn` | - |
| `AssociateUserAccessLoggingSettingsRequest` | `structure` | `portalArn`, `userAccessLoggingSettingsArn` | - |
| `AssociateUserAccessLoggingSettingsResponse` | `structure` | `portalArn`, `userAccessLoggingSettingsArn` | - |
| `AssociateUserSettingsRequest` | `structure` | `portalArn`, `userSettingsArn` | - |
| `AssociateUserSettingsResponse` | `structure` | `portalArn`, `userSettingsArn` | - |
| `CreateBrowserSettingsRequest` | `structure` | `additionalEncryptionContext`, `browserPolicy`, `clientToken`, `customerManagedKey`, `tags`, `webContentFilteringPolicy` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
