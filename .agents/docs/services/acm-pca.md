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
- Common required input members in this group: `CertificateArn`, `CertificateAuthorityArn`, `ResourceArn`

### Create

- Operations: `CreateCertificateAuthority`, `CreateCertificateAuthorityAuditReport`, `CreatePermission`
- Traits: `idempotent` (2)
- Common required input members in this group: `Actions`, `AuditReportResponseFormat`, `CertificateAuthorityArn`, `CertificateAuthorityConfiguration`, `CertificateAuthorityType`, `Principal`, `S3BucketName`

### Delete

- Operations: `DeleteCertificateAuthority`, `DeletePermission`, `DeletePolicy`
- Common required input members in this group: `CertificateAuthorityArn`, `Principal`, `ResourceArn`

### List

- Operations: `ListCertificateAuthorities`, `ListPermissions`, `ListTags`
- Traits: `paginated` (3)
- Common required input members in this group: `CertificateAuthorityArn`

### Describe

- Operations: `DescribeCertificateAuthority`, `DescribeCertificateAuthorityAuditReport`
- Common required input members in this group: `AuditReportId`, `CertificateAuthorityArn`

### Import

- Operations: `ImportCertificateAuthorityCertificate`
- Common required input members in this group: `Certificate`, `CertificateAuthorityArn`

### Issue

- Operations: `IssueCertificate`
- Traits: `idempotent` (1)
- Common required input members in this group: `CertificateAuthorityArn`, `Csr`, `SigningAlgorithm`, `Validity`

### Put

- Operations: `PutPolicy`
- Common required input members in this group: `Policy`, `ResourceArn`

### Restore

- Operations: `RestoreCertificateAuthority`
- Common required input members in this group: `CertificateAuthorityArn`

### Revoke

- Operations: `RevokeCertificate`
- Common required input members in this group: `CertificateAuthorityArn`, `CertificateSerial`, `RevocationReason`

### Tag

- Operations: `TagCertificateAuthority`
- Common required input members in this group: `CertificateAuthorityArn`, `Tags`

### Untag

- Operations: `UntagCertificateAuthority`
- Common required input members in this group: `CertificateAuthorityArn`, `Tags`

### Update

- Operations: `UpdateCertificateAuthority`
- Common required input members in this group: `CertificateAuthorityArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCertificateAuthority` | - | `idempotent` | `CertificateAuthorityConfiguration`, `CertificateAuthorityType` | - | `CreateCertificateAuthorityResponse` | `InvalidArgsException`, `InvalidPolicyException`, `InvalidTagException`, `LimitExceededException` | Creates a root or subordinate private certificate authority (CA). You must specify the CA configuration, an optional configuration for Online Certificate Status Protocol (OCSP) and/or a certificate revocation list (CRL), the CA type, and an optional... |
| `CreateCertificateAuthorityAuditReport` | - | `idempotent` | `AuditReportResponseFormat`, `CertificateAuthorityArn`, `S3BucketName` | - | `CreateCertificateAuthorityAuditReportResponse` | `InvalidArgsException`, `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Creates an audit report that lists every time that your CA private key is used to issue a certificate. The IssueCertificate and RevokeCertificate actions use the private key. |
| `CreatePermission` | - | - | `Actions`, `CertificateAuthorityArn`, `Principal` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `LimitExceededException`, `PermissionAlreadyExistsException`, `RequestFailedException`, `ResourceNotFoundException` | Grants one or more permissions on a private CA to the Certificate Manager (ACM) service principal (`acm.amazonaws.com`). These permissions allow ACM to issue and renew ACM certificates that reside in the same Amazon Web Services account as the CA. |
| `DeleteCertificateAuthority` | - | - | `CertificateAuthorityArn` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidStateException`, `ResourceNotFoundException` | Deletes a private certificate authority (CA). You must provide the Amazon Resource Name (ARN) of the private CA that you want to delete. |
| `DeletePermission` | - | - | `CertificateAuthorityArn`, `Principal` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | Revokes permissions on a private CA granted to the Certificate Manager (ACM) service principal (acm.amazonaws.com). These permissions allow ACM to issue and renew ACM certificates that reside in the same Amazon Web Services account as the CA. |
| `DeletePolicy` | - | - | `ResourceArn` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidStateException`, `LockoutPreventedException`, `RequestFailedException`, `ResourceNotFoundException` | Deletes the resource-based policy attached to a private CA. Deletion will remove any access that the policy has granted. |
| `DescribeCertificateAuthority` | - | - | `CertificateAuthorityArn` | - | `DescribeCertificateAuthorityResponse` | `InvalidArnException`, `ResourceNotFoundException` | Lists information about your private certificate authority (CA) or one that has been shared with you. You specify the private CA on input by its ARN (Amazon Resource Name). |
| `DescribeCertificateAuthorityAuditReport` | - | - | `AuditReportId`, `CertificateAuthorityArn` | - | `DescribeCertificateAuthorityAuditReportResponse` | `InvalidArgsException`, `InvalidArnException`, `ResourceNotFoundException` | Lists information about a specific audit report created by calling the CreateCertificateAuthorityAuditReport action. Audit information is created every time the certificate authority (CA) private key is used. |
| `GetCertificate` | - | - | `CertificateArn`, `CertificateAuthorityArn` | - | `GetCertificateResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Retrieves a certificate from your private CA or one that has been shared with you. The ARN of the certificate is returned when you call the IssueCertificate action. |
| `GetCertificateAuthorityCertificate` | - | - | `CertificateAuthorityArn` | - | `GetCertificateAuthorityCertificateResponse` | `InvalidArnException`, `InvalidStateException`, `ResourceNotFoundException` | Retrieves the certificate and certificate chain for your private certificate authority (CA) or one that has been shared with you. Both the certificate and the chain are base64 PEM-encoded. |
| `GetCertificateAuthorityCsr` | - | - | `CertificateAuthorityArn` | - | `GetCertificateAuthorityCsrResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `RequestInProgressException`, `ResourceNotFoundException` | Retrieves the certificate signing request (CSR) for your private certificate authority (CA). The CSR is created when you call the CreateCertificateAuthority action. |
| `GetPolicy` | - | - | `ResourceArn` | - | `GetPolicyResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | Retrieves the resource-based policy attached to a private CA. If either the private CA resource or the policy cannot be found, this action returns a `ResourceNotFoundException`. |
| `ImportCertificateAuthorityCertificate` | - | - | `Certificate`, `CertificateAuthorityArn` | - | `Unit` | `CertificateMismatchException`, `ConcurrentModificationException`, `InvalidArnException`, `InvalidRequestException`, `InvalidStateException`, `MalformedCertificateException`, `RequestFailedException`, `RequestInProgressException`, ... (+1) | Imports a signed private CA certificate into Amazon Web Services Private CA. This action is used when you are using a chain of trust whose root is located outside Amazon Web Services Private CA. |
| `IssueCertificate` | - | `idempotent` | `CertificateAuthorityArn`, `Csr`, `SigningAlgorithm`, `Validity` | - | `IssueCertificateResponse` | `InvalidArgsException`, `InvalidArnException`, `InvalidStateException`, `LimitExceededException`, `MalformedCSRException`, `ResourceNotFoundException` | Uses your private certificate authority (CA), or one that has been shared with you, to issue a client certificate. This action returns the Amazon Resource Name (ARN) of the certificate. |
| `ListCertificateAuthorities` | - | `paginated` | - | - | `ListCertificateAuthoritiesResponse` | `InvalidNextTokenException` | Lists the private certificate authorities that you created by using the CreateCertificateAuthority action. |
| `ListPermissions` | - | `paginated` | `CertificateAuthorityArn` | - | `ListPermissionsResponse` | `InvalidArnException`, `InvalidNextTokenException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | List all permissions on a private CA, if any, granted to the Certificate Manager (ACM) service principal (acm.amazonaws.com). These permissions allow ACM to issue and renew ACM certificates that reside in the same Amazon Web Services account as the CA. |
| `ListTags` | - | `paginated` | `CertificateAuthorityArn` | - | `ListTagsResponse` | `InvalidArnException`, `InvalidStateException`, `RequestFailedException`, `ResourceNotFoundException` | Lists the tags, if any, that are associated with your private CA or one that has been shared with you. Tags are labels that you can use to identify and organize your CAs. |
| `PutPolicy` | - | - | `Policy`, `ResourceArn` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidPolicyException`, `InvalidStateException`, `LockoutPreventedException`, `RequestFailedException`, `ResourceNotFoundException` | Attaches a resource-based policy to a private CA. A policy can also be applied by sharing a private CA through Amazon Web Services Resource Access Manager (RAM). |
| `RestoreCertificateAuthority` | - | - | `CertificateAuthorityArn` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `ResourceNotFoundException` | Restores a certificate authority (CA) that is in the `DELETED` state. You can restore a CA during the period that you defined in the PermanentDeletionTimeInDays parameter of the DeleteCertificateAuthority action. |
| `RevokeCertificate` | - | - | `CertificateAuthorityArn`, `CertificateSerial`, `RevocationReason` | - | `Unit` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidRequestException`, `InvalidStateException`, `LimitExceededException`, `RequestAlreadyProcessedException`, `RequestFailedException`, `RequestInProgressException`, ... (+1) | Revokes a certificate that was issued inside Amazon Web Services Private CA. If you enable a certificate revocation list (CRL) when you create or update your private CA, information about the revoked certificates will be included in the CRL. |
| `TagCertificateAuthority` | - | - | `CertificateAuthorityArn`, `Tags` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `InvalidTagException`, `ResourceNotFoundException`, `TooManyTagsException` | Adds one or more tags to your private CA. Tags are labels that you can use to identify and organize your Amazon Web Services resources. |
| `UntagCertificateAuthority` | - | - | `CertificateAuthorityArn`, `Tags` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `InvalidTagException`, `ResourceNotFoundException` | Remove one or more tags from your private CA. A tag consists of a key-value pair. |
| `UpdateCertificateAuthority` | - | - | `CertificateAuthorityArn` | - | `Unit` | `ConcurrentModificationException`, `InvalidArgsException`, `InvalidArnException`, `InvalidPolicyException`, `InvalidStateException`, `ResourceNotFoundException` | Updates the status or configuration of a private certificate authority (CA). Your private CA must be in the `ACTIVE` or `DISABLED` state before you can update it. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidArnException` | `structure` | `message` | The requested Amazon Resource Name (ARN) does not refer to an existing resource. |
| `ResourceNotFoundException` | `structure` | `message` | A resource such as a private CA, S3 bucket, certificate, audit report, or policy cannot be found. |
| `InvalidStateException` | `structure` | `message` | The state of the private CA does not allow this action to occur. |
| `RequestFailedException` | `structure` | `message` | The request has failed for an unspecified reason. |
| `ConcurrentModificationException` | `structure` | `message` | A previous update to your private CA is still ongoing. |
| `InvalidArgsException` | `structure` | `message` | One or more of the specified arguments was not valid. |
| `RequestInProgressException` | `structure` | `message` | Your request is already in progress. |
| `LimitExceededException` | `structure` | `message` | An Amazon Web Services Private CA quota has been exceeded. |
| `InvalidPolicyException` | `structure` | `message` | The resource policy is invalid or is missing a required statement. |
| `InvalidTagException` | `structure` | `message` | The tag associated with the CA is not valid. |
| `LockoutPreventedException` | `structure` | `message` | The current action was prevented because it would lock the caller out from performing subsequent actions. |
| `InvalidRequestException` | `structure` | `message` | The request action cannot be performed or is prohibited. |
| `InvalidNextTokenException` | `structure` | `message` | The token specified in the `NextToken` argument is not valid. |
| `CreateCertificateAuthorityRequest` | `structure` | `CertificateAuthorityConfiguration`, `CertificateAuthorityType`, `IdempotencyToken`, `KeyStorageSecurityStandard`, `RevocationConfiguration`, `Tags`, `UsageMode` | - |
| `CreateCertificateAuthorityResponse` | `structure` | `CertificateAuthorityArn` | - |
| `CreateCertificateAuthorityAuditReportRequest` | `structure` | `AuditReportResponseFormat`, `CertificateAuthorityArn`, `S3BucketName` | - |
| `CreateCertificateAuthorityAuditReportResponse` | `structure` | `AuditReportId`, `S3Key` | - |
| `CreatePermissionRequest` | `structure` | `Actions`, `CertificateAuthorityArn`, `Principal`, `SourceAccount` | - |
| `PermissionAlreadyExistsException` | `structure` | `message` | The designated permission has already been given to the user. |
| `DeleteCertificateAuthorityRequest` | `structure` | `CertificateAuthorityArn`, `PermanentDeletionTimeInDays` | - |
| `DeletePermissionRequest` | `structure` | `CertificateAuthorityArn`, `Principal`, `SourceAccount` | - |
| `DeletePolicyRequest` | `structure` | `ResourceArn` | - |
| `DescribeCertificateAuthorityRequest` | `structure` | `CertificateAuthorityArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
