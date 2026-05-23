# Tax Settings

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the tax setting API to programmatically set, modify, and delete the tax registration number (TRN), associated business legal name, and address (Collectively referred to as "TRN information"). You can also programmatically view TRN information and tax addresses ("Tax profiles"). You can use this API to automate your TRN information settings instead of manually using the console. Service Endpoint https://tax.us-east-1.amazonaws.com

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Tax Settings workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Put`, `Batch`, `List`, `Delete` operation families, including `GetTaxExemptionTypes`, `GetTaxInheritance`, `GetTaxRegistration`, `GetTaxRegistrationDocument`, `PutSupplementalTaxRegistration`, `PutTaxExemption`.

## Service Identity and Protocol

- AWS model slug: `taxsettings`
- AWS SDK for Rust slug: `taxsettings`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/taxsettings/service/2018-05-10/taxsettings-2018-05-10.json`
- SDK ID: `TaxSettings`
- Endpoint prefix: `-`
- ARN namespace: `tax`
- CloudFormation name: `-`
- CloudTrail event source: `tax.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `Put` (4), `Batch` (3), `List` (3), `Delete` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchDeleteTaxRegistration`, `BatchGetTaxExemptions`, `BatchPutTaxRegistration`, `DeleteSupplementalTaxRegistration`, `DeleteTaxRegistration`, `PutSupplementalTaxRegistration`, `PutTaxExemption`, `PutTaxInheritance`, `PutTaxRegistration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetTaxExemptions`, `GetTaxExemptionTypes`, `GetTaxInheritance`, `GetTaxRegistration`, `GetTaxRegistrationDocument`, `ListSupplementalTaxRegistrations`, `ListTaxExemptions`, `ListTaxRegistrations`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetTaxExemptionTypes`, `GetTaxInheritance`, `GetTaxRegistration`, `GetTaxRegistrationDocument`
- Traits: `readonly` (4)
- Common required input members in this group: -

### Put

- Operations: `PutSupplementalTaxRegistration`, `PutTaxExemption`, `PutTaxInheritance`, `PutTaxRegistration`
- Common required input members in this group: `taxRegistrationEntry`

### Batch

- Operations: `BatchDeleteTaxRegistration`, `BatchGetTaxExemptions`, `BatchPutTaxRegistration`
- Traits: `readonly` (1)
- Common required input members in this group: `accountIds`

### List

- Operations: `ListSupplementalTaxRegistrations`, `ListTaxExemptions`, `ListTaxRegistrations`
- Traits: `readonly` (3), `paginated` (3)
- Common required input members in this group: -

### Delete

- Operations: `DeleteSupplementalTaxRegistration`, `DeleteTaxRegistration`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchDeleteTaxRegistration` | `POST /BatchDeleteTaxRegistration` | - | `accountIds` | - | `BatchDeleteTaxRegistrationResponse` | `ConflictException`, `InternalServerException`, `ValidationException` | Deletes tax registration for multiple accounts in batch. This can be used to delete tax registrations for up to five accounts in one batch. This API operation can't be used to delete your tax registration in Brazil. ... |
| `BatchGetTaxExemptions` | `POST /BatchGetTaxExemptions` | `readonly` | `accountIds` | - | `BatchGetTaxExemptionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Get the active tax exemptions for a given list of accounts. The IAM action is tax:GetExemptions . |
| `BatchPutTaxRegistration` | `POST /BatchPutTaxRegistration` | - | `accountIds`, `taxRegistrationEntry` | - | `BatchPutTaxRegistrationResponse` | `ConflictException`, `InternalServerException`, `ValidationException` | Adds or updates tax registration for multiple accounts in batch. This can be used to add or update tax registrations for up to five accounts in one batch. You can't set a TRN if there's a pending TRN. You'll need to ... |
| `DeleteSupplementalTaxRegistration` | `POST /DeleteSupplementalTaxRegistration` | - | `authorityId` | - | `DeleteSupplementalTaxRegistrationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a supplemental tax registration for a single account. |
| `DeleteTaxRegistration` | `POST /DeleteTaxRegistration` | - | - | - | `DeleteTaxRegistrationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes tax registration for a single account. This API operation can't be used to delete your tax registration in Brazil. Use the Payment preferences page in the Billing and Cost Management console instead. |
| `GetTaxExemptionTypes` | `POST /GetTaxExemptionTypes` | `readonly` | - | - | `GetTaxExemptionTypesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Get supported tax exemption types. The IAM action is tax:GetExemptions . |
| `GetTaxInheritance` | `POST /GetTaxInheritance` | `readonly` | - | - | `GetTaxInheritanceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | The get account tax inheritance status. |
| `GetTaxRegistration` | `POST /GetTaxRegistration` | `readonly` | - | - | `GetTaxRegistrationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves tax registration for a single account. |
| `GetTaxRegistrationDocument` | `POST /GetTaxRegistrationDocument` | `readonly` | `taxDocumentMetadata` | - | `GetTaxRegistrationDocumentResponse` | `InternalServerException`, `ValidationException` | Downloads your tax documents to the Amazon S3 bucket that you specify in your request. |
| `ListSupplementalTaxRegistrations` | `POST /ListSupplementalTaxRegistrations` | `readonly`, `paginated` | - | - | `ListSupplementalTaxRegistrationsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves supplemental tax registrations for a single account. |
| `ListTaxExemptions` | `POST /ListTaxExemptions` | `readonly`, `paginated` | - | - | `ListTaxExemptionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the tax exemption of accounts listed in a consolidated billing family. The IAM action is tax:GetExemptions . |
| `ListTaxRegistrations` | `POST /ListTaxRegistrations` | `readonly`, `paginated` | - | - | `ListTaxRegistrationsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the tax registration of accounts listed in a consolidated billing family. This can be used to retrieve up to 100 accounts' tax registrations in one call (default 50). |
| `PutSupplementalTaxRegistration` | `POST /PutSupplementalTaxRegistration` | - | `taxRegistrationEntry` | - | `PutSupplementalTaxRegistrationResponse` | `ConflictException`, `InternalServerException`, `ValidationException` | Stores supplemental tax registration for a single account. |
| `PutTaxExemption` | `POST /PutTaxExemption` | - | `accountIds`, `authority`, `exemptionType`, `exemptionCertificate` | - | `PutTaxExemptionResponse` | `AccessDeniedException`, `AttachmentUploadException`, `CaseCreationLimitExceededException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds the tax exemption for a single account or all accounts listed in a consolidated billing family. The IAM action is tax:UpdateExemptions . |
| `PutTaxInheritance` | `POST /PutTaxInheritance` | - | - | - | `PutTaxInheritanceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | The updated tax inheritance status. |
| `PutTaxRegistration` | `POST /PutTaxRegistration` | - | `taxRegistrationEntry` | - | `PutTaxRegistrationResponse` | `ConflictException`, `InternalServerException`, `ValidationException` | Adds or updates tax registration for a single account. You can't set a TRN if there's a pending TRN. You'll need to delete the pending TRN first. To call this API operation for specific countries, see the following c ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The access is denied for the Amazon Web ServicesSupport API. |
| `AttachmentUploadException` | `structure` | message | Failed to upload the tax exemption document to Amazon Web ServicesSupport case. |
| `CaseCreationLimitExceededException` | `structure` | message | You've exceeded the Amazon Web ServicesSupport case creation limit for your account. |
| `ConflictException` | `structure` | message, errorCode | The exception when the input is creating conflict with the given state. |
| `InternalServerException` | `structure` | message, errorCode | The exception thrown when an unexpected error occurs when processing a request. |
| `ResourceNotFoundException` | `structure` | message, errorCode | The exception thrown when the input doesn't have a resource associated to it. |
| `ValidationException` | `structure` | message, errorCode, fieldList | The exception when the input doesn't pass validation for at least one of the input parameters. |
| `BatchDeleteTaxRegistrationRequest` | `structure` | accountIds | - |
| `BatchDeleteTaxRegistrationResponse` | `structure` | errors | - |
| `BatchGetTaxExemptionsRequest` | `structure` | accountIds | - |
| `BatchGetTaxExemptionsResponse` | `structure` | taxExemptionDetailsMap, failedAccounts | - |
| `BatchPutTaxRegistrationRequest` | `structure` | accountIds, taxRegistrationEntry | - |
| `BatchPutTaxRegistrationResponse` | `structure` | status, errors | - |
| `DeleteSupplementalTaxRegistrationRequest` | `structure` | authorityId | - |
| `DeleteSupplementalTaxRegistrationResponse` | `structure` | **empty (no members)** | - |
| `DeleteTaxRegistrationRequest` | `structure` | accountId | - |
| `DeleteTaxRegistrationResponse` | `structure` | **empty (no members)** | - |
| `GetTaxExemptionTypesRequest` | `structure` | **empty (no members)** | - |
| `GetTaxExemptionTypesResponse` | `structure` | taxExemptionTypes | - |
| `GetTaxInheritanceRequest` | `structure` | **empty (no members)** | - |
| `GetTaxInheritanceResponse` | `structure` | heritageStatus | - |
| `GetTaxRegistrationRequest` | `structure` | accountId | - |
| `GetTaxRegistrationResponse` | `structure` | taxRegistration | - |
| `GetTaxRegistrationDocumentRequest` | `structure` | destinationS3Location, taxDocumentMetadata | - |
| `GetTaxRegistrationDocumentResponse` | `structure` | destinationFilePath, presignedS3Url | - |
| `ListSupplementalTaxRegistrationsRequest` | `structure` | maxResults, nextToken | - |
| `ListSupplementalTaxRegistrationsResponse` | `structure` | taxRegistrations, nextToken | - |
| `ListTaxExemptionsRequest` | `structure` | maxResults, nextToken | - |
| `ListTaxExemptionsResponse` | `structure` | nextToken, taxExemptionDetailsMap | - |
| `ListTaxRegistrationsRequest` | `structure` | maxResults, nextToken | - |
| `ListTaxRegistrationsResponse` | `structure` | accountDetails, nextToken | - |
| `PutSupplementalTaxRegistrationRequest` | `structure` | taxRegistrationEntry | - |
| `PutSupplementalTaxRegistrationResponse` | `structure` | authorityId, status | - |
| `PutTaxExemptionRequest` | `structure` | accountIds, authority, exemptionType, exemptionCertificate | - |
| `PutTaxExemptionResponse` | `structure` | caseId | - |
| `PutTaxInheritanceRequest` | `structure` | heritageStatus | - |
| `PutTaxInheritanceResponse` | `structure` | **empty (no members)** | - |
| `PutTaxRegistrationRequest` | `structure` | accountId, taxRegistrationEntry | - |
| `PutTaxRegistrationResponse` | `structure` | status | - |
| `AddressRoleType` | `enum` | TAX_ADDRESS, BILLING_ADDRESS, CONTACT_ADDRESS | - |
| `EntityExemptionAccountStatus` | `enum` | None, Valid, Expired, Pending | - |
| `HeritageStatus` | `enum` | OptIn, OptOut | - |
| `IndonesiaTaxRegistrationNumberType` | `enum` | NIK, PASSPORT_NUMBER, NPWP, NITKU | - |
| `Industries` | `enum` | CIRCULATING_ORG, PROFESSIONAL_ORG, BANKS, INSURANCE, PENSION_AND_BENEFIT_FUNDS, DEVELOPMENT_AGENCIES | - |
| `IsraelCustomerType` | `enum` | BUSINESS, INDIVIDUAL | - |
| `IsraelDealerType` | `enum` | AUTHORIZED, NON_AUTHORIZED | - |
| `MalaysiaServiceTaxCode` | `enum` | CONSULTANCY, DIGITAL_SVC_ELECTRONIC_MEDIUM, IT_SERVICES, TRAINING_OR_COACHING | - |
| `PersonType` | `enum` | LEGAL_PERSON, PHYSICAL_PERSON, BUSINESS | - |
| `RegistrationType` | `enum` | INTRA_EU, LOCAL | - |
| `SaudiArabiaTaxRegistrationNumberType` | `enum` | TAX_REGISTRATION_NUMBER, TAX_IDENTIFICATION_NUMBER, COMMERCIAL_REGISTRATION_NUMBER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
