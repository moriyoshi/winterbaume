# AWS Marketplace Agreement Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Marketplace is a curated digital catalog that customers can use to find, buy, deploy, and manage third-party software, data, and services to build solutions and run their businesses. The AWS Marketplace Agreement Service provides an API interface that helps AWS Marketplace sellers manage their product-related agreements, including listing, searching, and filtering agreements. To manage agreements in AWS Marketplace, you must ensure that your AWS Identity and Access Management (IAM) policies and roles are set up. The user must have the required policies/permissions that allow them to carry out the actions in AWS: `DescribeAgreement` – Grants permission to users to obtain detailed meta data about any of their agreements. `GetAgreementTerms` – Grants permission to users to obtain details about the terms of an agreement.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Marketplace Agreement Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Describe`, `Get`, `Search` operation families, including `DescribeAgreement`, `GetAgreementTerms`, `SearchAgreements`.

## Service Identity and Protocol

- AWS model slug: `marketplace-agreement`
- AWS SDK for Rust slug: `marketplaceagreement`
- Model version: `2020-03-01`
- Model file: `vendor/api-models-aws/models/marketplace-agreement/service/2020-03-01/marketplace-agreement-2020-03-01.json`
- SDK ID: `Marketplace Agreement`
- Endpoint prefix: `agreement-marketplace`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (1), `Get` (1), `Search` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAgreement`, `GetAgreementTerms`, `SearchAgreements`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAgreementCancellationRequest`, `GetAgreementPaymentRequest`, `GetAgreementTerms`, `GetBillingAdjustmentRequest`
- Traits: `paginated` (1)
- Common required input members in this group: `agreementId`

### List

- Operations: `ListAgreementCancellationRequests`, `ListAgreementInvoiceLineItems`, `ListAgreementPaymentRequests`, `ListBillingAdjustmentRequests`
- Traits: `paginated` (4)
- Common required input members in this group: `partyType`

### Cancel

- Operations: `CancelAgreementCancellationRequest`, `CancelAgreementPaymentRequest`
- Common required input members in this group: `agreementId`

### Send

- Operations: `SendAgreementCancellationRequest`, `SendAgreementPaymentRequest`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `agreementId`

### Batch

- Operations: `BatchCreateBillingAdjustmentRequest`
- Common required input members in this group: -

### Describe

- Operations: `DescribeAgreement`
- Common required input members in this group: -

### Search

- Operations: `SearchAgreements`
- Traits: `paginated` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCreateBillingAdjustmentRequest` | `-` | - | `billingAdjustmentRequestEntries` | - | `BatchCreateBillingAdjustmentRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Allows sellers (proposers) to submit billing adjustment requests for one or more invoices within an agreement. Each entry in the batch specifies an invoice and the adjustment amount. The operation returns successfull ... |
| `CancelAgreementCancellationRequest` | `-` | - | `agreementId`, `agreementCancellationRequestId`, `cancellationReason` | - | `CancelAgreementCancellationRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows sellers (proposers) to withdraw an existing agreement cancellation request that is in a pending state. Once cancelled, the cancellation request transitions to CANCELLED status and can no longer be approved or ... |
| `CancelAgreementPaymentRequest` | `-` | - | `paymentRequestId`, `agreementId` | - | `CancelAgreementPaymentRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows sellers (proposers) to cancel a payment request that is in PENDING_APPROVAL status. Once cancelled, the payment request transitions to CANCELLED status and can no longer be accepted or rejected by the buyer. O ... |
| `DescribeAgreement` | `-` | - | `agreementId` | - | `DescribeAgreementOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides details about an agreement, such as the proposer, acceptor, start date, and end date. |
| `GetAgreementCancellationRequest` | `-` | - | `agreementCancellationRequestId`, `agreementId` | - | `GetAgreementCancellationRequestOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific agreement cancellation request. Both sellers (proposers) and buyers (acceptors) can use this operation to view cancellation requests associated with their agreements. T ... |
| `GetAgreementPaymentRequest` | `-` | - | `paymentRequestId`, `agreementId` | - | `GetAgreementPaymentRequestOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific payment request. Both sellers (proposers) and buyers (acceptors) can use this operation to view payment requests associated with their agreements. The response includes ... |
| `GetAgreementTerms` | `-` | `paginated` | `agreementId` | - | `GetAgreementTermsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Obtains details about the terms in an agreement that you participated in as proposer or acceptor. The details include: TermType – The type of term, such as LegalTerm , RenewalTerm , or ConfigurableUpfrontPricingTerm ... |
| `GetBillingAdjustmentRequest` | `-` | - | `agreementId`, `billingAdjustmentRequestId` | - | `GetBillingAdjustmentRequestOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific billing adjustment request. Sellers (proposers) can use this operation to view the status and details of a billing adjustment request they submitted. A ResourceNotFound ... |
| `ListAgreementCancellationRequests` | `-` | `paginated` | `partyType` | - | `ListAgreementCancellationRequestsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists agreement cancellation requests available to you as a seller or buyer. Both sellers (proposers) and buyers (acceptors) can use this operation to find cancellation requests by specifying their party type and app ... |
| `ListAgreementInvoiceLineItems` | `-` | `paginated` | `agreementId`, `groupBy` | - | `ListAgreementInvoiceLineItemsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows sellers (proposers) to retrieve aggregated billing data from AWS Marketplace agreements using flexible grouping. Supports invoice-level aggregation with filtering by billing period, invoice type, and issued da ... |
| `ListAgreementPaymentRequests` | `-` | `paginated` | `partyType` | - | `ListAgreementPaymentRequestsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists payment requests available to you as a seller or buyer. Both sellers (proposers) and buyers (acceptors) can use this operation to find payment requests by specifying their party type and applying optional param ... |
| `ListBillingAdjustmentRequests` | `-` | `paginated` | - | - | `ListBillingAdjustmentRequestsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists billing adjustment requests for a specific agreement. Sellers (proposers) can use this operation to view all billing adjustment requests associated with an agreement. Pagination is supported through maxResults ... |
| `SearchAgreements` | `-` | `paginated` | - | - | `SearchAgreementsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Searches across all agreements that a proposer has in AWS Marketplace. The search returns a list of agreements with basic agreement information. The following filter combinations are supported when the PartyType is P ... |
| `SendAgreementCancellationRequest` | `-` | `idempotency-token` | `agreementId`, `reasonCode` | `clientToken` | `SendAgreementCancellationRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows sellers (proposers) to submit a cancellation request for an active agreement. The cancellation request is created in PENDING_APPROVAL status, at which point the buyer can review it. |
| `SendAgreementPaymentRequest` | `-` | `idempotency-token` | `agreementId`, `termId`, `name`, `chargeAmount` | `clientToken` | `SendAgreementPaymentRequestOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows sellers (proposers) to submit a payment request to buyers (acceptors) for a specific charge amount for an agreement that includes a VariablePaymentTerm . The payment request is created in PENDING_APPROVAL stat ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | requestId, message | User does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | requestId, message, resourceId, resourceType | The request could not be completed due to a conflict with the current state of the resource. |
| `InternalServerException` | `structure` | requestId, message | Unexpected error during processing of request. |
| `ResourceNotFoundException` | `structure` | requestId, message, resourceId, resourceType | Request references a resource which does not exist. |
| `ThrottlingException` | `structure` | requestId, message | Request was denied due to request throttling. |
| `ValidationException` | `structure` | requestId, message, reason, fields | The input fails to satisfy the constraints specified by the service. |
| `BatchCreateBillingAdjustmentRequestInput` | `structure` | billingAdjustmentRequestEntries | - |
| `BatchCreateBillingAdjustmentRequestOutput` | `structure` | items, errors | - |
| `CancelAgreementCancellationRequestInput` | `structure` | agreementId, agreementCancellationRequestId, cancellationReason | - |
| `CancelAgreementCancellationRequestOutput` | `structure` | agreementCancellationRequestId, agreementId, reasonCode, description, status, statusMessage, createdAt, updatedAt | - |
| `CancelAgreementPaymentRequestInput` | `structure` | paymentRequestId, agreementId | - |
| `CancelAgreementPaymentRequestOutput` | `structure` | paymentRequestId, agreementId, status, name, description, chargeAmount, currencyCode, createdAt, updatedAt | - |
| `DescribeAgreementInput` | `structure` | agreementId | - |
| `DescribeAgreementOutput` | `structure` | agreementId, acceptor, proposer, startTime, endTime, acceptanceTime, agreementType, estimatedCharges, proposalSummary, status | - |
| `GetAgreementCancellationRequestInput` | `structure` | agreementCancellationRequestId, agreementId | - |
| `GetAgreementCancellationRequestOutput` | `structure` | agreementCancellationRequestId, agreementId, reasonCode, description, status, statusMessage, createdAt, updatedAt | - |
| `GetAgreementPaymentRequestInput` | `structure` | paymentRequestId, agreementId | - |
| `GetAgreementPaymentRequestOutput` | `structure` | paymentRequestId, agreementId, status, statusMessage, name, description, chargeId, chargeAmount, currencyCode, createdAt, updatedAt | - |
| `GetAgreementTermsInput` | `structure` | agreementId, maxResults, nextToken | - |
| `GetAgreementTermsOutput` | `structure` | acceptedTerms, nextToken | - |
| `GetBillingAdjustmentRequestInput` | `structure` | agreementId, billingAdjustmentRequestId | - |
| `GetBillingAdjustmentRequestOutput` | `structure` | billingAdjustmentRequestId, agreementId, adjustmentReasonCode, description, originalInvoiceId, adjustmentAmount, currencyCode, status, statusMessage, createdAt, updatedAt | - |
| `ListAgreementCancellationRequestsInput` | `structure` | partyType, agreementId, status, agreementType, catalog, maxResults, nextToken | - |
| `ListAgreementCancellationRequestsOutput` | `structure` | nextToken, items | - |
| `ListAgreementInvoiceLineItemsInput` | `structure` | agreementId, groupBy, invoiceId, invoiceType, invoiceBillingPeriod, beforeIssuedTime, afterIssuedTime, maxResults, nextToken | - |
| `ListAgreementInvoiceLineItemsOutput` | `structure` | agreementInvoiceLineItemGroupSummaries, nextToken | - |
| `ListAgreementPaymentRequestsInput` | `structure` | partyType, agreementType, catalog, agreementId, status, maxResults, nextToken | - |
| `ListAgreementPaymentRequestsOutput` | `structure` | nextToken, items | - |
| `ListBillingAdjustmentRequestsInput` | `structure` | agreementId, status, createdAfter, createdBefore, maxResults, catalog, agreementType, nextToken | - |
| `ListBillingAdjustmentRequestsOutput` | `structure` | nextToken, items | - |
| `SearchAgreementsInput` | `structure` | catalog, filters, sort, maxResults, nextToken | - |
| `SearchAgreementsOutput` | `structure` | agreementViewSummaries, nextToken | - |
| `SendAgreementCancellationRequestInput` | `structure` | agreementId, reasonCode, clientToken, description | - |
| `SendAgreementCancellationRequestOutput` | `structure` | agreementId, agreementCancellationRequestId, status, reasonCode, description, createdAt, updatedAt | - |
| `SendAgreementPaymentRequestInput` | `structure` | clientToken, agreementId, termId, name, chargeAmount, description | - |
| `SendAgreementPaymentRequestOutput` | `structure` | paymentRequestId, agreementId, status, name, description, chargeAmount, currencyCode, createdAt | - |
| `AgreementCancellationRequestReasonCode` | `enum` | INCORRECT_TERMS_ACCEPTED, REPLACING_AGREEMENT, TEST_AGREEMENT, ALTERNATIVE_PROCUREMENT_CHANNEL, PRODUCT_DISCONTINUED, UNINTENDED_RENEWAL, BUYER_DISSATISFACTION, OTHER | - |
| `AgreementCancellationRequestStatus` | `enum` | PENDING_APPROVAL, APPROVED, REJECTED, CANCELLED, VALIDATION_FAILED | - |
| `AgreementStatus` | `enum` | ACTIVE, ARCHIVED, CANCELLED, EXPIRED, RENEWED, REPLACED, ROLLED_BACK, SUPERSEDED, TERMINATED | - |
| `BillingAdjustmentErrorCode` | `enum` | CONFLICT_EXCEPTION, VALIDATION_EXCEPTION, RESOURCE_NOT_FOUND_EXCEPTION, INTERNAL_FAILURE | - |
| `BillingAdjustmentReasonCode` | `enum` | INCORRECT_TERMS_ACCEPTED, INCORRECT_METERING, TEST_ENVIRONMENT_CHARGES, ALTERNATIVE_PROCUREMENT_CHANNEL, UNINTENDED_RENEWAL, BUYER_DISSATISFACTION, OTHER | - |
| `BillingAdjustmentStatus` | `enum` | PENDING, VALIDATION_FAILED, COMPLETED | - |
| `InvoiceType` | `enum` | INVOICE, CREDIT_MEMO | - |
| `LineItemGroupBy` | `enum` | INVOICE_ID | - |
| `PaymentRequestApprovalStrategy` | `enum` | AUTO_APPROVE_ON_EXPIRATION, WAIT_FOR_APPROVAL | - |
| `PaymentRequestStatus` | `enum` | VALIDATING, VALIDATION_FAILED, PENDING_APPROVAL, APPROVED, REJECTED, CANCELLED | - |
| `ResourceType` | `enum` | AGREEMENT, CHARGE, PAYMENT_REQUEST | - |
| `SortOrder` | `enum` | ASCENDING, DESCENDING | - |
| `ValidationExceptionReason` | `enum` | INVALID_AGREEMENT_ID, MISSING_AGREEMENT_ID, INVALID_CATALOG, INVALID_FILTER_NAME, INVALID_FILTER_VALUES, INVALID_SORT_BY, INVALID_SORT_ORDER, INVALID_NEXT_TOKEN, INVALID_MAX_RESULTS, INVALID_TERM_ID, MISSING_TERM_ID, MISSING_NAME, ... (+12) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
