# AWS Certificate Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Certificate Manager You can use Certificate Manager (ACM) to manage SSL/TLS certificates for your Amazon Web Services-based websites and applications. For more information about using ACM, see the Certificate Manager User Guide.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-acm/tests/scenario_test.rs`: request a certificate with tags, add and remove tags during its lifecycle, and verify the final tag set for Terraform-style `aws_acm_certificate` management.
- Backported from `scenario_test.rs`: manage a certificate fleet containing requested and imported certificates, list by status, update certificate transparency options, and delete retired certificates.
- Backported from `scenario_test.rs`: import an external certificate, inspect its metadata, and exercise the export gate for certificate material.
- Scenario insight from EC2: exercise account or service defaults for AWS Certificate Manager by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent DNS validation records, pending validation and renewal states, import/export eligibility, account expiry notifications, and integrations where ACM certificates are attached to services such as Elastic Load Balancing or CloudFront.

## Service Identity and Protocol

- AWS model slug: `acm`
- AWS SDK for Rust slug: `acm`
- Model version: `2015-12-08`
- Model file: `vendor/api-models-aws/models/acm/service/2015-12-08/acm-2015-12-08.json`
- SDK ID: `ACM`
- Endpoint prefix: `acm`
- ARN namespace: `acm`
- CloudFormation name: `CertificateManager`
- CloudTrail event source: `acm.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (2), `List` (2), `Add` (1), `Delete` (1), `Describe` (1), `Export` (1), `Import` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToCertificate`, `DeleteCertificate`, `ImportCertificate`, `PutAccountConfiguration`, `RemoveTagsFromCertificate`, `RevokeCertificate`, `UpdateCertificateOptions`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCertificate`, `GetAccountConfiguration`, `GetCertificate`, `ListCertificates`, `ListTagsForCertificate`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportCertificate`, `ImportCertificate`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/acm/latest/userguide/dns-validation.html
- https://docs.aws.amazon.com/acm/latest/userguide/check-certificate-renewal-status.html
- https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-request-public.html

Research outcomes:
- Public ACM certificates require domain validation. DNS validation uses ACM-generated CNAME record name/value pairs to prove domain ownership.
- The same DNS validation CNAME values are generated for a wildcard domain and its base domain, such as `*.example.com` and `example.com`.
- DNS validation supports ongoing automatic renewal when the CNAME remains present.
- ACM can create Route 53 validation records from the console when the caller has suitable permissions, but the documentation states this automatic Route 53 record creation is not available as an ACM API operation.
- A newly requested certificate can remain Pending validation after DNS records are created. If ACM cannot validate within 72 hours after generating the CNAME, status changes to Validation timed out and a new certificate request is required.
- Renewal status values include Pending automatic renewal, Pending validation, Success, and Failed.
- A certificate is eligible for managed renewal when it is associated with another AWS service such as Elastic Load Balancing or CloudFront, or has been exported since issue or last renewal.

Parity implications:
- Model certificates, domain validation options, per-domain validation status, issuance status, renewal eligibility, and renewal status separately.
- DNS validation should produce stable CNAME records per domain/base-domain relationship and support delayed validation plus timeout behaviour.
- Route 53 helper behaviour should be modelled as console-side convenience, not as an ACM service API side effect.

## Operation Groups

### Get

- Operations: `GetAccountConfiguration`, `GetCertificate`
- Common required input members in this group: `CertificateArn`

### List

- Operations: `ListCertificates`, `ListTagsForCertificate`
- Traits: `paginated` (1)
- Common required input members in this group: `CertificateArn`

### Add

- Operations: `AddTagsToCertificate`
- Common required input members in this group: `CertificateArn`, `Tags`

### Delete

- Operations: `DeleteCertificate`
- Common required input members in this group: `CertificateArn`

### Describe

- Operations: `DescribeCertificate`
- Common required input members in this group: `CertificateArn`

### Export

- Operations: `ExportCertificate`
- Common required input members in this group: `CertificateArn`, `Passphrase`

### Import

- Operations: `ImportCertificate`
- Common required input members in this group: `Certificate`, `PrivateKey`

### Put

- Operations: `PutAccountConfiguration`
- Common required input members in this group: `IdempotencyToken`

### Remove

- Operations: `RemoveTagsFromCertificate`
- Common required input members in this group: `CertificateArn`, `Tags`

### Renew

- Operations: `RenewCertificate`
- Common required input members in this group: `CertificateArn`

### Request

- Operations: `RequestCertificate`
- Common required input members in this group: `DomainName`

### Resend

- Operations: `ResendValidationEmail`
- Common required input members in this group: `CertificateArn`, `Domain`, `ValidationDomain`

### Revoke

- Operations: `RevokeCertificate`
- Common required input members in this group: `CertificateArn`, `RevocationReason`

### Update

- Operations: `UpdateCertificateOptions`
- Common required input members in this group: `CertificateArn`, `Options`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToCertificate` | - | - | `CertificateArn`, `Tags` | - | `Unit` | `InvalidArnException`, `InvalidParameterException`, `InvalidTagException`, `ResourceNotFoundException`, `TagPolicyException`, `ThrottlingException`, `TooManyTagsException` | Adds one or more tags to an ACM certificate. Tags are labels that you can use to identify and organize your Amazon Web Services resources. |
| `DeleteCertificate` | - | - | `CertificateArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InvalidArnException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a certificate and its associated private key. If this action succeeds, the certificate no longer appears in the list that can be displayed by calling the ListCertificates action or be retrieved by calling the GetCertificate action. |
| `DescribeCertificate` | - | - | `CertificateArn` | - | `DescribeCertificateResponse` | `InvalidArnException`, `ResourceNotFoundException` | Returns detailed metadata about the specified ACM certificate. If you have just created a certificate using the `RequestCertificate` action, there is a delay of several seconds before you can retrieve information about it. |
| `ExportCertificate` | - | - | `CertificateArn`, `Passphrase` | - | `ExportCertificateResponse` | `InvalidArnException`, `RequestInProgressException`, `ResourceNotFoundException` | Exports a private certificate issued by a private certificate authority (CA) or public certificate for use anywhere. The exported file contains the certificate, the certificate chain, and the encrypted private key associated with the public key that is... |
| `GetAccountConfiguration` | - | - | - | - | `GetAccountConfigurationResponse` | `AccessDeniedException`, `ThrottlingException` | Returns the account configuration options associated with an Amazon Web Services account. |
| `GetCertificate` | - | - | `CertificateArn` | - | `GetCertificateResponse` | `InvalidArnException`, `RequestInProgressException`, `ResourceNotFoundException` | Retrieves a certificate and its certificate chain. The certificate may be either a public or private certificate issued using the ACM `RequestCertificate` action, or a certificate imported into ACM using the `ImportCertificate` action. |
| `ImportCertificate` | - | - | `Certificate`, `PrivateKey` | - | `ImportCertificateResponse` | `InvalidArnException`, `InvalidParameterException`, `InvalidTagException`, `LimitExceededException`, `ResourceNotFoundException`, `TagPolicyException`, `TooManyTagsException` | Imports a certificate into Certificate Manager (ACM) to use with services that are integrated with ACM. Note that integrated services allow only certificate types and keys they support to be associated with their resources. |
| `ListCertificates` | - | `paginated` | - | - | `ListCertificatesResponse` | `InvalidArgsException`, `ValidationException` | Retrieves a list of certificate ARNs and domain names. You can request that only certificates that match a specific status be listed. |
| `ListTagsForCertificate` | - | - | `CertificateArn` | - | `ListTagsForCertificateResponse` | `InvalidArnException`, `ResourceNotFoundException` | Lists the tags that have been applied to the ACM certificate. Use the certificate's Amazon Resource Name (ARN) to specify the certificate. |
| `PutAccountConfiguration` | - | - | `IdempotencyToken` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `ThrottlingException`, `ValidationException` | Adds or modifies account-level configurations in ACM. The supported configuration option is `DaysBeforeExpiry`. |
| `RemoveTagsFromCertificate` | - | - | `CertificateArn`, `Tags` | - | `Unit` | `InvalidArnException`, `InvalidParameterException`, `InvalidTagException`, `ResourceNotFoundException`, `TagPolicyException`, `ThrottlingException` | Remove one or more tags from an ACM certificate. A tag consists of a key-value pair. |
| `RenewCertificate` | - | - | `CertificateArn` | - | `Unit` | `InvalidArnException`, `RequestInProgressException`, `ResourceNotFoundException` | Renews an eligible ACM certificate. In order to renew your Amazon Web Services Private CA certificates with ACM, you must first grant the ACM service principal permission to do so. |
| `RequestCertificate` | - | - | `DomainName` | - | `RequestCertificateResponse` | `InvalidArnException`, `InvalidDomainValidationOptionsException`, `InvalidParameterException`, `InvalidTagException`, `LimitExceededException`, `TagPolicyException`, `TooManyTagsException` | Requests an ACM certificate for use with other Amazon Web Services services. To request an ACM certificate, you must specify a fully qualified domain name (FQDN) in the `DomainName` parameter. |
| `ResendValidationEmail` | - | - | `CertificateArn`, `Domain`, `ValidationDomain` | - | `Unit` | `InvalidArnException`, `InvalidDomainValidationOptionsException`, `InvalidStateException`, `ResourceNotFoundException` | Resends the email that requests domain ownership validation. The domain owner or an authorized representative must approve the ACM certificate before it can be issued. |
| `RevokeCertificate` | - | - | `CertificateArn`, `RevocationReason` | - | `RevokeCertificateResponse` | `AccessDeniedException`, `ConflictException`, `InvalidArnException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Revokes a public ACM certificate. You can only revoke certificates that have been previously exported. |
| `UpdateCertificateOptions` | - | - | `CertificateArn`, `Options` | - | `Unit` | `InvalidArnException`, `InvalidStateException`, `LimitExceededException`, `ResourceNotFoundException` | Updates a certificate. You can use this function to specify whether to opt in to or out of recording your certificate in a certificate transparency log and exporting. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidArnException` | `structure` | `message` | The requested Amazon Resource Name (ARN) does not refer to an existing resource. |
| `ResourceNotFoundException` | `structure` | `message` | The specified certificate cannot be found in the caller's account or the caller's account cannot be found. |
| `ThrottlingException` | `structure` | `message` | The request was denied because it exceeded a quota. |
| `InvalidParameterException` | `structure` | `message` | An input parameter was invalid. |
| `InvalidTagException` | `structure` | `message` | One or both of the values that make up the key-value pair is not valid. |
| `TagPolicyException` | `structure` | `message` | A specified tag did not comply with an existing tag policy and was rejected. |
| `AccessDeniedException` | `structure` | `Message` | You do not have access required to perform this action. |
| `TooManyTagsException` | `structure` | `message` | The request contains too many tags. |
| `ConflictException` | `structure` | `message` | You are trying to update a resource or configuration that is already being created or updated. |
| `RequestInProgressException` | `structure` | `message` | The certificate request is in process and the certificate in your account has not yet been issued. |
| `LimitExceededException` | `structure` | `message` | An ACM quota has been exceeded. |
| `ResourceInUseException` | `structure` | `message` | The certificate is in use by another Amazon Web Services service in the caller's account. |
| `ValidationException` | `structure` | `message` | The supplied input failed to satisfy constraints of an Amazon Web Services service. |
| `InvalidDomainValidationOptionsException` | `structure` | `message` | One or more values in the DomainValidationOption structure is incorrect. |
| `InvalidStateException` | `structure` | `message` | Processing has reached an invalid state. |
| `AddTagsToCertificateRequest` | `structure` | `CertificateArn`, `Tags` | - |
| `DeleteCertificateRequest` | `structure` | `CertificateArn` | - |
| `DescribeCertificateRequest` | `structure` | `CertificateArn` | - |
| `DescribeCertificateResponse` | `structure` | `Certificate` | - |
| `ExportCertificateRequest` | `structure` | `CertificateArn`, `Passphrase` | - |
| `ExportCertificateResponse` | `structure` | `Certificate`, `CertificateChain`, `PrivateKey` | - |
| `GetAccountConfigurationResponse` | `structure` | `ExpiryEvents` | - |
| `GetCertificateRequest` | `structure` | `CertificateArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
