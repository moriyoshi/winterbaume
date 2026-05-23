# AWS Invoicing

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Invoice Configuration You can use Amazon Web Services Invoice Configuration APIs to programmatically create, update, delete, get, and list invoice units. You can also programmatically fetch the information of the invoice receiver. For example, business legal name, address, and invoicing contacts. You can use Amazon Web Services Invoice Configuration to receive separate Amazon Web Services invoices based your organizational needs. By using Amazon Web Services Invoice Configuration, you can configure invoice units that are groups of Amazon Web Services accounts that represent your business entities, and receive separate invoices for each business entity.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Invoicing workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListInvoiceSummaries`, `ListInvoiceUnits`, `ListProcurementPortalPreferences`, `ListTagsForResource`, `GetInvoicePDF`, `GetInvoiceUnit`.

## Service Identity and Protocol

- AWS model slug: `invoicing`
- AWS SDK for Rust slug: `invoicing`
- Model version: `2024-12-01`
- Model file: `vendor/api-models-aws/models/invoicing/service/2024-12-01/invoicing-2024-12-01.json`
- SDK ID: `Invoicing`
- Endpoint prefix: `invoicing`
- ARN namespace: `invoicing`
- CloudFormation name: `-`
- CloudTrail event source: `invoicing.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Create` (2), `Delete` (2), `Update` (2), `Batch` (1), `Put` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetInvoiceProfile`, `CreateInvoiceUnit`, `CreateProcurementPortalPreference`, `DeleteInvoiceUnit`, `DeleteProcurementPortalPreference`, `PutProcurementPortalPreference`, `TagResource`, `UntagResource`, `UpdateInvoiceUnit`, `UpdateProcurementPortalPreferenceStatus`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetInvoiceProfile`, `GetInvoicePDF`, `GetInvoiceUnit`, `GetProcurementPortalPreference`, `ListInvoiceSummaries`, `ListInvoiceUnits`, `ListProcurementPortalPreferences`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListInvoiceSummaries`, `ListInvoiceUnits`, `ListProcurementPortalPreferences`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (2)
- Common required input members in this group: -

### Get

- Operations: `GetInvoicePDF`, `GetInvoiceUnit`, `GetProcurementPortalPreference`
- Traits: `readonly` (2)
- Common required input members in this group: -

### Create

- Operations: `CreateInvoiceUnit`, `CreateProcurementPortalPreference`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Delete

- Operations: `DeleteInvoiceUnit`, `DeleteProcurementPortalPreference`
- Common required input members in this group: -

### Update

- Operations: `UpdateInvoiceUnit`, `UpdateProcurementPortalPreferenceStatus`
- Common required input members in this group: -

### Batch

- Operations: `BatchGetInvoiceProfile`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutProcurementPortalPreference`
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
| `BatchGetInvoiceProfile` | `-` | `readonly` | `AccountIds` | - | `BatchGetInvoiceProfileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This gets the invoice profile associated with a set of accounts. The accounts must be linked accounts under the requester management account organization. |
| `CreateInvoiceUnit` | `-` | - | `Name`, `InvoiceReceiver`, `Rule` | - | `CreateInvoiceUnitResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | This creates a new invoice unit with the provided definition. |
| `CreateProcurementPortalPreference` | `-` | `idempotent`, `idempotency-token` | `ProcurementPortalName`, `BuyerDomain`, `BuyerIdentifier`, `SupplierDomain`, `SupplierIdentifier`, `EinvoiceDeliveryEnabled`, `PurchaseOrderRetrievalEnabled`, `Contacts` | `ClientToken` | `CreateProcurementPortalPreferenceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a procurement portal preference configuration for e-invoice delivery and purchase order retrieval. This preference defines how invoices are delivered to a procurement portal and how purchase orders are retrieved. |
| `DeleteInvoiceUnit` | `-` | - | `InvoiceUnitArn` | - | `DeleteInvoiceUnitResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This deletes an invoice unit with the provided invoice unit ARN. |
| `DeleteProcurementPortalPreference` | `-` | - | `ProcurementPortalPreferenceArn` | - | `DeleteProcurementPortalPreferenceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes an existing procurement portal preference. This action cannot be undone. Active e-invoice delivery and PO retrieval configurations will be terminated. |
| `GetInvoicePDF` | `-` | `readonly` | `InvoiceId` | - | `GetInvoicePDFResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a URL to download the invoice document and supplemental documents associated with an invoice. The URLs are pre-signed and have expiration time. For special cases like Brazil, where Amazon Web Services generat ... |
| `GetInvoiceUnit` | `-` | `readonly` | `InvoiceUnitArn` | - | `GetInvoiceUnitResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This retrieves the invoice unit definition. |
| `GetProcurementPortalPreference` | `-` | - | `ProcurementPortalPreferenceArn` | - | `GetProcurementPortalPreferenceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the details of a specific procurement portal preference configuration. |
| `ListInvoiceSummaries` | `-` | `paginated` | `Selector` | - | `ListInvoiceSummariesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves your invoice details programmatically, without line item details. |
| `ListInvoiceUnits` | `-` | `readonly`, `paginated` | - | - | `ListInvoiceUnitsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | This fetches a list of all invoice unit definitions for a given account, as of the provided AsOf date. |
| `ListProcurementPortalPreferences` | `-` | `paginated` | - | - | `ListProcurementPortalPreferencesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves a list of procurement portal preferences associated with the Amazon Web Services account. |
| `ListTagsForResource` | `-` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags for a resource. |
| `PutProcurementPortalPreference` | `-` | - | `ProcurementPortalPreferenceArn`, `EinvoiceDeliveryEnabled`, `PurchaseOrderRetrievalEnabled`, `Contacts` | - | `PutProcurementPortalPreferenceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing procurement portal preference configuration. This operation can modify settings for e-invoice delivery and purchase order retrieval. |
| `TagResource` | `-` | - | `ResourceArn`, `ResourceTags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a tag to a resource. |
| `UntagResource` | `-` | - | `ResourceArn`, `ResourceTagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag from a resource. |
| `UpdateInvoiceUnit` | `-` | - | `InvoiceUnitArn` | - | `UpdateInvoiceUnitResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You can update the invoice unit configuration at any time, and Amazon Web Services will use the latest configuration at the end of the month. |
| `UpdateProcurementPortalPreferenceStatus` | `-` | - | `ProcurementPortalPreferenceArn` | - | `UpdateProcurementPortalPreferenceStatusResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the status of a procurement portal preference, including the activation state of e-invoice delivery and purchase order retrieval features. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, resourceName | You don't have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request could not be completed due to a conflict with the current state of the resource. This exception occurs when a concurrent modification is detecte ... |
| `InternalServerException` | `structure` | retryAfterSeconds, message | The processing request failed because of an unknown error, exception, or failure. |
| `ResourceNotFoundException` | `structure` | message, resourceName | The resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | message | The request was rejected because it attempted to create resources beyond the current Amazon Web Services account limits. The error message describes the lim ... |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, resourceName, reason, fieldList | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `BatchGetInvoiceProfileRequest` | `structure` | AccountIds | - |
| `BatchGetInvoiceProfileResponse` | `structure` | Profiles | - |
| `CreateInvoiceUnitRequest` | `structure` | Name, InvoiceReceiver, Description, TaxInheritanceDisabled, Rule, ResourceTags | - |
| `CreateInvoiceUnitResponse` | `structure` | InvoiceUnitArn | - |
| `CreateProcurementPortalPreferenceRequest` | `structure` | ProcurementPortalName, BuyerDomain, BuyerIdentifier, SupplierDomain, SupplierIdentifier, Selector, ProcurementPortalSharedSecret, ProcurementPortalInstanceEndpoint, TestEnvPreference, EinvoiceDeliveryEnabled, EinvoiceDeliveryPreference, PurchaseOrderRetrievalEnabled, ... (+3) | - |
| `CreateProcurementPortalPreferenceResponse` | `structure` | ProcurementPortalPreferenceArn | - |
| `DeleteInvoiceUnitRequest` | `structure` | InvoiceUnitArn | - |
| `DeleteInvoiceUnitResponse` | `structure` | InvoiceUnitArn | - |
| `DeleteProcurementPortalPreferenceRequest` | `structure` | ProcurementPortalPreferenceArn | - |
| `DeleteProcurementPortalPreferenceResponse` | `structure` | ProcurementPortalPreferenceArn | - |
| `GetInvoicePDFRequest` | `structure` | InvoiceId | - |
| `GetInvoicePDFResponse` | `structure` | InvoicePDF | - |
| `GetInvoiceUnitRequest` | `structure` | InvoiceUnitArn, AsOf | - |
| `GetInvoiceUnitResponse` | `structure` | InvoiceUnitArn, InvoiceReceiver, Name, Description, TaxInheritanceDisabled, Rule, LastModified | - |
| `GetProcurementPortalPreferenceRequest` | `structure` | ProcurementPortalPreferenceArn | - |
| `GetProcurementPortalPreferenceResponse` | `structure` | ProcurementPortalPreference | - |
| `ListInvoiceSummariesRequest` | `structure` | Selector, Filter, NextToken, MaxResults | - |
| `ListInvoiceSummariesResponse` | `structure` | InvoiceSummaries, NextToken | - |
| `ListInvoiceUnitsRequest` | `structure` | Filters, NextToken, MaxResults, AsOf | - |
| `ListInvoiceUnitsResponse` | `structure` | InvoiceUnits, NextToken | - |
| `ListProcurementPortalPreferencesRequest` | `structure` | NextToken, MaxResults | - |
| `ListProcurementPortalPreferencesResponse` | `structure` | ProcurementPortalPreferences, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | ResourceTags | - |
| `PutProcurementPortalPreferenceRequest` | `structure` | ProcurementPortalPreferenceArn, Selector, ProcurementPortalSharedSecret, ProcurementPortalInstanceEndpoint, TestEnvPreference, EinvoiceDeliveryEnabled, EinvoiceDeliveryPreference, PurchaseOrderRetrievalEnabled, Contacts | - |
| `PutProcurementPortalPreferenceResponse` | `structure` | ProcurementPortalPreferenceArn | - |
| `TagResourceRequest` | `structure` | ResourceArn, ResourceTags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, ResourceTagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateInvoiceUnitRequest` | `structure` | InvoiceUnitArn, Description, TaxInheritanceDisabled, Rule | - |
| `UpdateInvoiceUnitResponse` | `structure` | InvoiceUnitArn | - |
| `UpdateProcurementPortalPreferenceStatusRequest` | `structure` | ProcurementPortalPreferenceArn, EinvoiceDeliveryPreferenceStatus, EinvoiceDeliveryPreferenceStatusReason, PurchaseOrderRetrievalPreferenceStatus, PurchaseOrderRetrievalPreferenceStatusReason | - |
| `BuyerDomain` | `enum` | NetworkID | - |
| `ConnectionTestingMethod` | `enum` | PROD_ENV_DOLLAR_TEST, TEST_ENV_REPLAY_TEST | - |
| `EinvoiceDeliveryAttachmentType` | `enum` | INVOICE_PDF, RFP_PDF | - |
| `EinvoiceDeliveryDocumentType` | `enum` | AWS_CLOUD_INVOICE, AWS_CLOUD_CREDIT_MEMO, AWS_MARKETPLACE_INVOICE, AWS_MARKETPLACE_CREDIT_MEMO, AWS_REQUEST_FOR_PAYMENT | - |
| `InvoiceType` | `enum` | INVOICE, CREDIT_MEMO | - |
| `ListInvoiceSummariesResourceType` | `enum` | ACCOUNT_ID, INVOICE_ID | - |
| `ProcurementPortalName` | `enum` | SAP_BUSINESS_NETWORK, COUPA | - |
| `ProcurementPortalPreferenceStatus` | `enum` | PENDING_VERIFICATION, TEST_INITIALIZED, TEST_INITIALIZATION_FAILED, TEST_FAILED, ACTIVE, SUSPENDED | - |
| `Protocol` | `enum` | CXML | - |
| `PurchaseOrderDataSourceType` | `enum` | ASSOCIATED_PURCHASE_ORDER_REQUIRED, PURCHASE_ORDER_NOT_REQUIRED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
