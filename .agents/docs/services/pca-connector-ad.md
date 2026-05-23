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

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags, if any, that are associated with your resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds one or more tags to your resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from your resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You can receive this error if you attempt to create a resource share when you don't have the required permissions. This can be caused by insufficient permis ... |
| `ConflictException` | `structure` | Message, ResourceId, ResourceType | This request cannot be completed for one of the following reasons because the requested resource was being concurrently modified by another request. |
| `InternalServerException` | `structure` | Message | The request processing has failed because of an unknown error, exception or failure with an internal server. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, ServiceCode, QuotaCode | Request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | Message, ServiceCode, QuotaCode | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | Message, Reason | An input validation error occurred. For example, invalid characters in a template name, or if a pagination token is invalid. |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `AccessRight` | `enum` | ALLOW, DENY | - |
| `ApplicationPolicyType` | `enum` | ALL_APPLICATION_POLICIES, ANY_PURPOSE, ATTESTATION_IDENTITY_KEY_CERTIFICATE, CERTIFICATE_REQUEST_AGENT, CLIENT_AUTHENTICATION, CODE_SIGNING, CTL_USAGE, DIGITAL_RIGHTS, DIRECTORY_SERVICE_EMAIL_REPLICATION, DISALLOWED_LIST, DNS_SERVER_TRUST, DOCUMENT_ENCRYPTION, ... (+55) | - |
| `ClientCompatibilityV2` | `enum` | WINDOWS_SERVER_2003, WINDOWS_SERVER_2008, WINDOWS_SERVER_2008_R2, WINDOWS_SERVER_2012, WINDOWS_SERVER_2012_R2, WINDOWS_SERVER_2016 | - |
| `ClientCompatibilityV3` | `enum` | WINDOWS_SERVER_2008, WINDOWS_SERVER_2008_R2, WINDOWS_SERVER_2012, WINDOWS_SERVER_2012_R2, WINDOWS_SERVER_2016 | - |
| `ClientCompatibilityV4` | `enum` | WINDOWS_SERVER_2012, WINDOWS_SERVER_2012_R2, WINDOWS_SERVER_2016 | - |
| `ConnectorStatus` | `enum` | CREATING, ACTIVE, DELETING, FAILED | - |
| `ConnectorStatusReason` | `enum` | CA_CERTIFICATE_REGISTRATION_FAILED, DIRECTORY_ACCESS_DENIED, INTERNAL_FAILURE, INSUFFICIENT_FREE_ADDRESSES, INVALID_SUBNET_IP_PROTOCOL, PRIVATECA_ACCESS_DENIED, PRIVATECA_RESOURCE_NOT_FOUND, SECURITY_GROUP_NOT_IN_VPC, VPC_ACCESS_DENIED, VPC_ENDPOINT_LIMIT_EXCEEDED, VPC_RESOURCE_NOT_FOUND | - |
| `DirectoryRegistrationStatus` | `enum` | CREATING, ACTIVE, DELETING, FAILED | - |
| `DirectoryRegistrationStatusReason` | `enum` | DIRECTORY_ACCESS_DENIED, DIRECTORY_RESOURCE_NOT_FOUND, DIRECTORY_NOT_ACTIVE, DIRECTORY_NOT_REACHABLE, DIRECTORY_TYPE_NOT_SUPPORTED, INTERNAL_FAILURE | - |
| `HashAlgorithm` | `enum` | SHA256, SHA384, SHA512 | - |
| `IpAddressType` | `enum` | IPV4, DUALSTACK | - |
| `KeySpec` | `enum` | KEY_EXCHANGE, SIGNATURE | - |
| `KeyUsagePropertyType` | `enum` | ALL | - |
| `PrivateKeyAlgorithm` | `enum` | RSA, ECDH_P256, ECDH_P384, ECDH_P521 | - |
| `ServicePrincipalNameStatus` | `enum` | CREATING, ACTIVE, DELETING, FAILED | - |
| `ServicePrincipalNameStatusReason` | `enum` | DIRECTORY_ACCESS_DENIED, DIRECTORY_NOT_REACHABLE, DIRECTORY_RESOURCE_NOT_FOUND, SPN_EXISTS_ON_DIFFERENT_AD_OBJECT, SPN_LIMIT_EXCEEDED, INTERNAL_FAILURE | - |
| `TemplateStatus` | `enum` | ACTIVE, DELETING | - |
| `ValidationExceptionReason` | `enum` | FIELD_VALIDATION_FAILED, INVALID_CA_SUBJECT, INVALID_PERMISSION, INVALID_STATE, MISMATCHED_CONNECTOR, MISMATCHED_VPC, NO_CLIENT_TOKEN, UNKNOWN_OPERATION, OTHER | - |
| `ValidityPeriodType` | `enum` | HOURS, DAYS, WEEKS, MONTHS, YEARS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
