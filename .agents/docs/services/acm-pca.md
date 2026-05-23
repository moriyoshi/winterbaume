# AWS Certificate Manager Private Certificate Authority

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Amazon Web Services Private Certificate Authority API Reference . It provides descriptions, syntax, and usage examples for each of the actions and data types involved in creating and managing a private certificate authority (CA) for your organization. The documentation for each action shows the API request parameters and the JSON response. Alternatively, you can use one of the Amazon Web Services SDKs to access an API that is tailored to the programming language or platform that you prefer. For more information, see Amazon Web Services SDKs.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-acmpca/tests/scenario_test.rs`: create a root CA, generate and import a self-signed certificate, activate the authority, then disable and restore it.
- Backported from `scenario_test.rs`: grant and revoke delegated permissions against a private CA, verifying the permission lifecycle around principals and actions.
- Backported from `scenario_test.rs`: attach, read, and remove a CA resource policy.
- From the AWS documentation and model: model private CA issuance, activation states, audit reports, certificate revocation lists, policy-based delegation, and cleanup guardrails for CA-dependent resources.

## Service Identity and Protocol

- AWS model slug: `acm-pca`
- AWS SDK for Rust slug: `acmpca`
- Model version: `2017-08-22`
- Model file: `vendor/api-models-aws/models/acm-pca/service/2017-08-22/acm-pca-2017-08-22.json`
- SDK ID: `ACM PCA`
- Endpoint prefix: `acm-pca`
- ARN namespace: `acm-pca`
- CloudFormation name: `ACMPCA`
- CloudTrail event source: `acm-pca.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `Create` (3), `Delete` (3), `List` (3), `Describe` (2), `Import` (1), `Issue` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCertificateAuthority`, `CreateCertificateAuthorityAuditReport`, `CreatePermission`, `DeleteCertificateAuthority`, `DeletePermission`, `DeletePolicy`, `ImportCertificateAuthorityCertificate`, `PutPolicy`, `RestoreCertificateAuthority`, `RevokeCertificate`, `TagCertificateAuthority`, `UntagCertificateAuthority`, `UpdateCertificateAuthority`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCertificateAuthority`, `DescribeCertificateAuthorityAuditReport`, `GetCertificate`, `GetCertificateAuthorityCertificate`, `GetCertificateAuthorityCsr`, `GetPolicy`, `ListCertificateAuthorities`, `ListPermissions`, `ListTags`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateCertificateAuthorityAuditReport`, `DescribeCertificateAuthorityAuditReport`, `ImportCertificateAuthorityCertificate`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html
- https://docs.aws.amazon.com/privateca/latest/userguide/PCACertInstall.html
- https://docs.aws.amazon.com/privateca/latest/userguide/revocation-setup.html

Research outcomes:
- AWS Private CA creates private X.509 certificate authority hierarchies and issues RFC 5280-compliant private certificates.
- A private CA can be a root CA or subordinate CA. A newly created CA needs a CA certificate installed before it can issue certificates.
- Installing a root CA certificate uses a self-signed root certificate generated from the CA CSR. Installing a subordinate CA certificate uses either an AWS Private CA parent or an external parent CA.
- Private CA supports multiple signing algorithms and key algorithms, with compatibility constraints between CA keys and certificate signing algorithms.
- Certificate issuance uses templates to constrain certificate purpose and extensions.
- Revocation can be implemented with certificate revocation lists (CRLs), Online Certificate Status Protocol (OCSP), or short-lived certificates.
- CRL and OCSP configuration require reachable distribution endpoints and service permissions to publish revocation information.
- Revoking a certificate affects revocation status publication but does not delete the certificate or CA resources.
- Deleting an unused CA and separating root CA usage from issuing CA usage are documented best practices.

Parity implications:
- Model certificate authorities, CA status, CSR, CA certificate installation, issued certificates, templates, revocation configuration, CRLs, OCSP, and deletion lifecycle separately.
- IssueCertificate should require an active CA with an installed certificate and should produce retrievable certificate material asynchronously.
- Revocation should update revocation status and publication artefacts without mutating the original issued certificate.

## Operation Groups

### Get

- Operations: `GetCertificate`, `GetCertificateAuthorityCertificate`, `GetCertificateAuthorityCsr`, `GetPolicy`
- Common required input members in this group: `CertificateAuthorityArn`

### Create

- Operations: `CreateCertificateAuthority`, `CreateCertificateAuthorityAuditReport`, `CreatePermission`
- Traits: `idempotent` (2)
- Common required input members in this group: `CertificateAuthorityArn`

### Delete

- Operations: `DeleteCertificateAuthority`, `DeletePermission`, `DeletePolicy`
- Common required input members in this group: `CertificateAuthorityArn`

### List

- Operations: `ListCertificateAuthorities`, `ListPermissions`, `ListTags`
- Traits: `paginated` (3)
- Common required input members in this group: `CertificateAuthorityArn`

### Describe

- Operations: `DescribeCertificateAuthority`, `DescribeCertificateAuthorityAuditReport`
- Common required input members in this group: `CertificateAuthorityArn`

### Import

- Operations: `ImportCertificateAuthorityCertificate`
- Common required input members in this group: -

### Issue

- Operations: `IssueCertificate`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Put

- Operations: `PutPolicy`
- Common required input members in this group: -

### Restore

- Operations: `RestoreCertificateAuthority`
- Common required input members in this group: -

### Revoke

- Operations: `RevokeCertificate`
- Common required input members in this group: -

### Tag

- Operations: `TagCertificateAuthority`
- Common required input members in this group: -

### Untag

- Operations: `UntagCertificateAuthority`
- Common required input members in this group: -

### Update

- Operations: `UpdateCertificateAuthority`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCertificateAuthority` | `-` | `idempotent` | `CertificateAuthorityConfiguration`, `CertificateAuthorityType` | - | `CreateCertificateAuthorityResponse` | `InvalidArgsException`, `InvalidPolicyException`, `InvalidTagException`, `LimitExceededException` | Creates a root or subordinate private certificate authority (CA). You must specify the CA configuration, an optional configuration for Online Certificate Status Protocol (OCSP) and/or a certificate revocation list (C ... |
| `CreateCertificateAuthorityAuditReport` | `-` | `idempotent` | `CertificateAuthorityArn`, `S3BucketName`, `AuditReportResponseFormat` | - | `CreateCertificateAuthorityAuditReportResponse` | `InvalidArgsException`, `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Creates an audit report that lists every time that your CA private key is used to issue a certificate. The IssueCertificate and RevokeCertificate actions use the private key. To save the audit report to your designat ... |
| `CreatePermission` | `-` | - | `CertificateAuthorityArn`, `Principal`, `Actions` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `LimitExceededException`, `PermissionAlreadyExistsException`, `RequestFailedException`, `ResourceNotFoundException` | Grants one or more permissions on a private CA to the Certificate Manager (ACM) service principal ( acm.amazonaws.com ). These permissions allow ACM to issue and renew ACM certificates that reside in the same Amazon ... |
| `DeleteCertificateAuthority` | `-` | - | `CertificateAuthorityArn` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidStateException`, `ResourceNotFoundException` | Deletes a private certificate authority (CA). You must provide the Amazon Resource Name (ARN) of the private CA that you want to delete. You can find the ARN by calling the ListCertificateAuthorities action. Deleting ... |
| `DeletePermission` | `-` | - | `CertificateAuthorityArn`, `Principal` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | Revokes permissions on a private CA granted to the Certificate Manager (ACM) service principal (acm.amazonaws.com). These permissions allow ACM to issue and renew ACM certificates that reside in the same Amazon Web S ... |
| `DeletePolicy` | `-` | - | `ResourceArn` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidStateException`, `LockoutPreventedException`, `RequestFailedException`, `ResourceNotFoundException` | Deletes the resource-based policy attached to a private CA. Deletion will remove any access that the policy has granted. If there is no policy attached to the private CA, this action will return successful. If you de ... |
| `DescribeCertificateAuthority` | `-` | - | `CertificateAuthorityArn` | - | `DescribeCertificateAuthorityResponse` | `InvalidArnException`, `ResourceNotFoundException` | Lists information about your private certificate authority (CA) or one that has been shared with you. You specify the private CA on input by its ARN (Amazon Resource Name). The output contains the status of your CA. ... |
| `DescribeCertificateAuthorityAuditReport` | `-` | - | `CertificateAuthorityArn`, `AuditReportId` | - | `DescribeCertificateAuthorityAuditReportResponse` | `InvalidArgsException`, `InvalidArnException`, `ResourceNotFoundException` | Lists information about a specific audit report created by calling the CreateCertificateAuthorityAuditReport action. Audit information is created every time the certificate authority (CA) private key is used. The pri ... |
| `GetCertificate` | `-` | - | `CertificateAuthorityArn`, `CertificateArn` | - | `GetCertificateResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Retrieves a certificate from your private CA or one that has been shared with you. The ARN of the certificate is returned when you call the IssueCertificate action. You must specify both the ARN of your private CA an ... |
| `GetCertificateAuthorityCertificate` | `-` | - | `CertificateAuthorityArn` | - | `GetCertificateAuthorityCertificateResponse` | `InvalidArnException`, `InvalidStateException`, `ResourceNotFoundException` | Retrieves the certificate and certificate chain for your private certificate authority (CA) or one that has been shared with you. Both the certificate and the chain are base64 PEM-encoded. The chain does not include ... |
| `GetCertificateAuthorityCsr` | `-` | - | `CertificateAuthorityArn` | - | `GetCertificateAuthorityCsrResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Retrieves the certificate signing request (CSR) for your private certificate authority (CA). The CSR is created when you call the CreateCertificateAuthority action. Sign the CSR with your Amazon Web Services Private ... |
| `GetPolicy` | `-` | - | `ResourceArn` | - | `GetPolicyResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | Retrieves the resource-based policy attached to a private CA. If either the private CA resource or the policy cannot be found, this action returns a ResourceNotFoundException . The policy can be attached or updated w ... |
| `ImportCertificateAuthorityCertificate` | `-` | - | `CertificateAuthorityArn`, `Certificate` | - | `Unit` | `CertificateMismatchException`, `ConcurrentModificationException`, `InvalidArnException`, `InvalidRequestException`, `InvalidStateException`, `MalformedCertificateException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Imports a signed private CA certificate into Amazon Web Services Private CA. This action is used when you are using a chain of trust whose root is located outside Amazon Web Services Private CA. Before you can call t ... |
| `IssueCertificate` | `-` | `idempotent` | `CertificateAuthorityArn`, `Csr`, `SigningAlgorithm`, `Validity` | - | `IssueCertificateResponse` | `InvalidArgsException`, `InvalidArnException`, `InvalidStateException`, `LimitExceededException`, `MalformedCSRException`, `ResourceNotFoundException` | Uses your private certificate authority (CA), or one that has been shared with you, to issue a client certificate. This action returns the Amazon Resource Name (ARN) of the certificate. You can retrieve the certifica ... |
| `ListCertificateAuthorities` | `-` | `paginated` | - | - | `ListCertificateAuthoritiesResponse` | `InvalidNextTokenException` | Lists the private certificate authorities that you created by using the CreateCertificateAuthority action. |
| `ListPermissions` | `-` | `paginated` | `CertificateAuthorityArn` | - | `ListPermissionsResponse` | `InvalidArnException`, `InvalidNextTokenException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | List all permissions on a private CA, if any, granted to the Certificate Manager (ACM) service principal (acm.amazonaws.com). These permissions allow ACM to issue and renew ACM certificates that reside in the same Am ... |
| `ListTags` | `-` | `paginated` | `CertificateAuthorityArn` | - | `ListTagsResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | Lists the tags, if any, that are associated with your private CA or one that has been shared with you. Tags are labels that you can use to identify and organize your CAs. Each tag consists of a key and an optional va ... |
| `PutPolicy` | `-` | - | `ResourceArn`, `Policy` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidPolicyException`, `InvalidStateException`, `LockoutPreventedException`, `RequestFailedException`, `ResourceNotFoundException` | Attaches a resource-based policy to a private CA. A policy can also be applied by sharing a private CA through Amazon Web Services Resource Access Manager (RAM). For more information, see Attach a Policy for Cross-Ac ... |
| `RestoreCertificateAuthority` | `-` | - | `CertificateAuthorityArn` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `ResourceNotFoundException` | Restores a certificate authority (CA) that is in the DELETED state. You can restore a CA during the period that you defined in the PermanentDeletionTimeInDays parameter of the DeleteCertificateAuthority action. Curre ... |
| `RevokeCertificate` | `-` | - | `CertificateAuthorityArn`, `CertificateSerial`, `RevocationReason` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidRequestException`, `InvalidStateException`, `LimitExceededException`, `RequestAlreadyProcessedException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Revokes a certificate that was issued inside Amazon Web Services Private CA. If you enable a certificate revocation list (CRL) when you create or update your private CA, information about the revoked certificates wil ... |
| `TagCertificateAuthority` | `-` | - | `CertificateAuthorityArn`, `Tags` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `InvalidTagException`, `ResourceNotFoundException`, `TooManyTagsException` | Adds one or more tags to your private CA. Tags are labels that you can use to identify and organize your Amazon Web Services resources. Each tag consists of a key and an optional value. You specify the private CA on ... |
| `UntagCertificateAuthority` | `-` | - | `CertificateAuthorityArn`, `Tags` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `InvalidTagException`, `ResourceNotFoundException` | Remove one or more tags from your private CA. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this action, the tag will be removed regardless of value. If you speci ... |
| `UpdateCertificateAuthority` | `-` | - | `CertificateAuthorityArn` | - | `Unit` | `ConcurrentModificationException`, `InvalidArgsException`, `InvalidArnException`, `InvalidPolicyException`, `InvalidStateException`, `ResourceNotFoundException` | Updates the status or configuration of a private certificate authority (CA). Your private CA must be in the ACTIVE or DISABLED state before you can update it. You can disable a private CA that is in the ACTIVE state ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CertificateMismatchException` | `structure` | message | The certificate authority certificate you are importing does not comply with conditions specified in the certificate that signed it. |
| `ConcurrentModificationException` | `structure` | message | A previous update to your private CA is still ongoing. |
| `InvalidArgsException` | `structure` | message | One or more of the specified arguments was not valid. |
| `InvalidArnException` | `structure` | message | The requested Amazon Resource Name (ARN) does not refer to an existing resource. |
| `InvalidNextTokenException` | `structure` | message | The token specified in the NextToken argument is not valid. Use the token returned from your previous call to ListCertificateAuthorities . |
| `InvalidPolicyException` | `structure` | message | The resource policy is invalid or is missing a required statement. For general information about IAM policy and statement structure, see Overview of JSON Po ... |
| `InvalidRequestException` | `structure` | message | The request action cannot be performed or is prohibited. |
| `InvalidStateException` | `structure` | message | The state of the private CA does not allow this action to occur. |
| `InvalidTagException` | `structure` | message | The tag associated with the CA is not valid. The invalid argument is contained in the message field. |
| `LimitExceededException` | `structure` | message | An Amazon Web Services Private CA quota has been exceeded. See the exception message returned to determine the quota that was exceeded. |
| `LockoutPreventedException` | `structure` | message | The current action was prevented because it would lock the caller out from performing subsequent actions. Verify that the specified parameters would not res ... |
| `MalformedCSRException` | `structure` | message | The certificate signing request is invalid. |
| `MalformedCertificateException` | `structure` | message | One or more fields in the certificate are invalid. |
| `PermissionAlreadyExistsException` | `structure` | message | The designated permission has already been given to the user. |
| `RequestAlreadyProcessedException` | `structure` | message | Your request has already been completed. |
| `RequestFailedException` | `structure` | message | The request has failed for an unspecified reason. |
| `RequestInProgressException` | `structure` | message | Your request is already in progress. |
| `ResourceNotFoundException` | `structure` | message | A resource such as a private CA, S3 bucket, certificate, audit report, or policy cannot be found. |
| `TooManyTagsException` | `structure` | message | You can associate up to 50 tags with a private CA. Exception information is contained in the exception message field. |
| `CreateCertificateAuthorityRequest` | `structure` | CertificateAuthorityConfiguration, RevocationConfiguration, CertificateAuthorityType, IdempotencyToken, KeyStorageSecurityStandard, Tags, UsageMode | - |
| `CreateCertificateAuthorityResponse` | `structure` | CertificateAuthorityArn | - |
| `CreateCertificateAuthorityAuditReportRequest` | `structure` | CertificateAuthorityArn, S3BucketName, AuditReportResponseFormat | - |
| `CreateCertificateAuthorityAuditReportResponse` | `structure` | AuditReportId, S3Key | - |
| `CreatePermissionRequest` | `structure` | CertificateAuthorityArn, Principal, SourceAccount, Actions | - |
| `DeleteCertificateAuthorityRequest` | `structure` | CertificateAuthorityArn, PermanentDeletionTimeInDays | - |
| `DeletePermissionRequest` | `structure` | CertificateAuthorityArn, Principal, SourceAccount | - |
| `DeletePolicyRequest` | `structure` | ResourceArn | - |
| `DescribeCertificateAuthorityRequest` | `structure` | CertificateAuthorityArn | - |
| `DescribeCertificateAuthorityResponse` | `structure` | CertificateAuthority | - |
| `DescribeCertificateAuthorityAuditReportRequest` | `structure` | CertificateAuthorityArn, AuditReportId | - |
| `DescribeCertificateAuthorityAuditReportResponse` | `structure` | AuditReportStatus, S3BucketName, S3Key, CreatedAt | - |
| `GetCertificateRequest` | `structure` | CertificateAuthorityArn, CertificateArn | - |
| `GetCertificateResponse` | `structure` | Certificate, CertificateChain | - |
| `GetCertificateAuthorityCertificateRequest` | `structure` | CertificateAuthorityArn | - |
| `GetCertificateAuthorityCertificateResponse` | `structure` | Certificate, CertificateChain | - |
| `GetCertificateAuthorityCsrRequest` | `structure` | CertificateAuthorityArn | - |
| `GetCertificateAuthorityCsrResponse` | `structure` | Csr | - |
| `GetPolicyRequest` | `structure` | ResourceArn | - |
| `GetPolicyResponse` | `structure` | Policy | - |
| `ImportCertificateAuthorityCertificateRequest` | `structure` | CertificateAuthorityArn, Certificate, CertificateChain | - |
| `AccessMethodType` | `enum` | CA_REPOSITORY, RESOURCE_PKI_MANIFEST, RESOURCE_PKI_NOTIFY | - |
| `ActionType` | `enum` | IssueCertificate, GetCertificate, ListPermissions | - |
| `AuditReportResponseFormat` | `enum` | JSON, CSV | - |
| `AuditReportStatus` | `enum` | CREATING, SUCCESS, FAILED | - |
| `CertificateAuthorityStatus` | `enum` | CREATING, PENDING_CERTIFICATE, ACTIVE, DELETED, DISABLED, EXPIRED, FAILED | - |
| `CertificateAuthorityType` | `enum` | ROOT, SUBORDINATE | - |
| `CertificateAuthorityUsageMode` | `enum` | GENERAL_PURPOSE, SHORT_LIVED_CERTIFICATE | - |
| `CrlType` | `enum` | COMPLETE, PARTITIONED | - |
| `ExtendedKeyUsageType` | `enum` | SERVER_AUTH, CLIENT_AUTH, CODE_SIGNING, EMAIL_PROTECTION, TIME_STAMPING, OCSP_SIGNING, SMART_CARD_LOGIN, DOCUMENT_SIGNING, CERTIFICATE_TRANSPARENCY | - |
| `FailureReason` | `enum` | REQUEST_TIMED_OUT, UNSUPPORTED_ALGORITHM, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
