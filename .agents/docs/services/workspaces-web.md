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

### List

- Operations: `ListSessions`, `ListTagsForResource`
- Traits: `readonly` (2), `paginated` (1)
- Common required input members in this group: -

### Expire

- Operations: `ExpireSession`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Get

- Operations: `GetSession`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ExpireSession` | `DELETE /portals/{portalId}/sessions/{sessionId}` | `idempotent` | `portalId`, `sessionId` | - | `ExpireSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Expires an active secure browser session. |
| `GetSession` | `GET /portals/{portalId}/sessions/{sessionId}` | `readonly` | `portalId`, `sessionId` | - | `GetSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information for a secure browser session. |
| `ListSessions` | `GET /portals/{portalId}/sessions` | `readonly`, `paginated` | `portalId` | - | `ListSessionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists information for multiple secure browser sessions from a specific portal. |
| `ListTagsForResource` | `GET /tags/{resourceArn+}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of tags for a resource. |
| `TagResource` | `POST /tags/{resourceArn+}` | `idempotency-token` | `resourceArn`, `tags` | `clientToken` | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Adds or overwrites one or more tags for the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn+}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListSessions` | - | `username -> username`, `sessionId -> sessionId`, `sortBy -> sortBy`, `status -> status`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | Access is denied. |
| `ConflictException` | `structure` | message, resourceId, resourceType | There is a conflict. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | There is an internal server error. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The resource cannot be found. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The service quota has been exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | There is a throttling error. |
| `TooManyTagsException` | `structure` | message, resourceName | There are too many tags. |
| `ValidationException` | `structure` | message, reason, fieldList | There is a validation error. |
| `ExpireSessionRequest` | `structure` | portalId, sessionId | - |
| `ExpireSessionResponse` | `structure` | **empty (no members)** | - |
| `GetSessionRequest` | `structure` | portalId, sessionId | - |
| `GetSessionResponse` | `structure` | session | - |
| `ListSessionsRequest` | `structure` | portalId, username, sessionId, sortBy, status, maxResults, nextToken | - |
| `ListSessionsResponse` | `structure` | sessions, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags, clientToken | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `Category` | `enum` | CULTS, GAMBLING, NUDITY, PORNOGRAPHY, SEX_EDUCATION, TASTELESS, VIOLENCE, DOWNLOAD_SITES, IMAGE_SHARING, PEER_TO_PEER, STREAMING_MEDIA_AND_DOWNLOADS, GENERATIVE_AI, ... (+15) | - |
| `ColorTheme` | `enum` | LIGHT, DARK | - |
| `Event` | `enum` | WEBSITE_INTERACT, FILE_DOWNLOAD_FROM_SECURE_BROWSER_TO_REMOTE_DISK, FILE_TRANSFER_FROM_REMOTE_TO_LOCAL_DISK, FILE_TRANSFER_FROM_LOCAL_TO_REMOTE_DISK, FILE_UPLOAD_FROM_REMOTE_DISK_TO_SECURE_BROWSER, CONTENT_PASTE_TO_WEBSITE, CONTENT_TRANSFER_FROM_LOCAL_TO_REMOTE_CLIPBOARD, CONTENT_COPY_FROM_WEBSITE, URL_LOAD, TAB_OPEN, TAB_CLOSE, PRINT_JOB_SUBMIT, ... (+5) | - |
| `FolderStructure` | `enum` | FLAT, NESTED_BY_DATE | - |
| `Locale` | `enum` | DE, EN, ES, FR, ID, IT, JP, KR, BR, CN, TW | - |
| `LogFileFormat` | `enum` | JSON_LINES, JSON | - |
| `MimeType` | `enum` | PNG, JPEG, ICO | - |
| `SessionSortBy` | `enum` | START_TIME_ASCENDING, START_TIME_DESCENDING | - |
| `SessionStatus` | `enum` | ACTIVE, TERMINATED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
