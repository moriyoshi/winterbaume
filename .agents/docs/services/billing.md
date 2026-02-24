# AWS Billing

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Billing API to programatically list the billing views available to you for a given time period. A billing view represents a set of billing data. The Billing API provides the following endpoint: `https://billing.us-east-1.api.aws`

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Billing where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Billing by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: retrieve and manage billing views, invoice units, and source views for account-level billing administration.
- From the operation surface: support billing data organisation, invoice grouping, list/get/update flows, and tag-based billing resource inventory.

## Service Identity and Protocol

- AWS model slug: `billing`
- AWS SDK for Rust slug: `billing`
- Model version: `2023-09-07`
- Model file: `vendor/api-models-aws/models/billing/service/2023-09-07/billing-2023-09-07.json`
- SDK ID: `Billing`
- Endpoint prefix: `billing`
- ARN namespace: `billing`
- CloudFormation name: `-`
- CloudTrail event source: `billing.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (3), `Get` (2), `Associate` (1), `Create` (1), `Delete` (1), `Disassociate` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateSourceViews`, `CreateBillingView`, `DeleteBillingView`, `DisassociateSourceViews`, `TagResource`, `UntagResource`, `UpdateBillingView`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBillingView`, `GetResourcePolicy`, `ListBillingViews`, `ListSourceViewsForBillingView`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-pref.html
- https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/getting-viewing-bill.html
- https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/manage-paymentprofiles.html

Research outcomes:
- The Bills page shows estimated charges for open billing periods with status `Pending`; the summary is not an invoice until the billing period closes and AWS calculates final charges.
- Closed billing periods and one-time fees generate invoices with status `Issued`. Invoices can be viewed and downloaded as PDFs.
- In AWS Organizations consolidated billing, the management account can view total charges and account-level charges across member accounts.
- Billing transfer changes invoice ownership: the bill transfer account receives invoices, while bill source accounts and linked accounts do not receive invoices while transfer is active.
- Billing transfer invoices include bill source account id and billing transfer name metadata.
- Invoice delivery preferences can send PDF invoices by email to the root user, billing contacts, or alternate billing contacts. If a billing contact is specified on payment preferences, the root user does not receive the PDF invoice or additional invoice email.
- Free Tier usage alerts can be enabled or disabled. CloudWatch billing alerts can be enabled from billing preferences and then cannot be deactivated later.
- Credit sharing preferences are controlled by the management account in AWS Organizations. Sharing can target all accounts, selected accounts, and newly created member accounts by default.
- Reserved Instance and Savings Plans discount sharing supports open sharing, prioritised group sharing, and restricted group sharing. The final monthly bill uses the preferences set at 23:59:59 UTC on the last day of the month.
- Group sharing depends on Cost Categories. Accounts can belong to only one sharing group, groups must not overlap, and the management account cannot belong to a sharing group.

Parity implications:
- Model billing periods, pending versus issued bills, invoices, invoice documents, payment methods, payment profiles, contacts, billing transfer, credit sharing, and RI/SP sharing preferences separately.
- Billing views should distinguish estimated month-to-date charges from final issued invoices.
- Organisation-level sharing preferences should be management-account controlled and affect generated bill allocation only at the documented billing cut-off.

## Operation Groups

### List

- Operations: `ListBillingViews`, `ListSourceViewsForBillingView`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `arn`, `resourceArn`

### Get

- Operations: `GetBillingView`, `GetResourcePolicy`
- Traits: `readonly` (2)
- Common required input members in this group: `arn`, `resourceArn`

### Associate

- Operations: `AssociateSourceViews`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `sourceViews`

### Create

- Operations: `CreateBillingView`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `name`, `sourceViews`

### Delete

- Operations: `DeleteBillingView`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`

### Disassociate

- Operations: `DisassociateSourceViews`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `sourceViews`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `resourceTags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `resourceTagKeys`

### Update

- Operations: `UpdateBillingView`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateSourceViews` | - | `idempotent` | `arn`, `sourceViews` | - | `AssociateSourceViewsResponse` | `AccessDeniedException`, `BillingViewHealthStatusException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates one or more source billing views with an existing billing view. This allows creating aggregate billing views that combine data from multiple sources. |
| `CreateBillingView` | - | `idempotent`, `idempotency-token` | `name`, `sourceViews` | `clientToken` | `CreateBillingViewResponse` | `AccessDeniedException`, `BillingViewHealthStatusException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a billing view with the specified billing view attributes. |
| `DeleteBillingView` | - | `idempotent` | `arn` | - | `DeleteBillingViewResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the specified billing view. |
| `DisassociateSourceViews` | - | `idempotent` | `arn`, `sourceViews` | - | `DisassociateSourceViewsResponse` | `AccessDeniedException`, `BillingViewHealthStatusException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between one or more source billing views and an existing billing view. This allows modifying the composition of aggregate billing views. |
| `GetBillingView` | - | `readonly` | `arn` | - | `GetBillingViewResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the metadata associated to the specified billing view ARN. |
| `GetResourcePolicy` | - | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the resource-based policy document attached to the resource in `JSON` format. |
| `ListBillingViews` | `POST /` | `readonly`, `paginated` | - | - | `ListBillingViewsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the billing views available for a given time period. Every Amazon Web Services account has a unique `PRIMARY` billing view that represents the billing data available by default. |
| `ListSourceViewsForBillingView` | - | `readonly`, `paginated` | `arn` | - | `ListSourceViewsForBillingViewResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the source views (managed Amazon Web Services billing views) associated with the billing view. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags associated with the billing view resource. |
| `TagResource` | - | - | `resourceArn`, `resourceTags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | An API operation for adding one or more tags (key-value pairs) to a resource. |
| `UntagResource` | - | - | `resourceArn`, `resourceTagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from a resource. Specify only tag keys in your request. |
| `UpdateBillingView` | - | `idempotent` | `arn` | - | `UpdateBillingViewResponse` | `AccessDeniedException`, `BillingViewHealthStatusException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | An API to update the attributes of the billing view. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | The request processing failed because of an unknown error, exception, or failure. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The specified ARN in the request doesn't exist. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `BillingViewHealthStatusException` | `structure` | `message` | Exception thrown when a billing view's health status prevents an operation from being performed. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | You've reached the limit of resources you can create, or exceeded the size of an individual resource. |
| `AssociateSourceViewsRequest` | `structure` | `arn`, `sourceViews` | - |
| `AssociateSourceViewsResponse` | `structure` | `arn` | - |
| `CreateBillingViewRequest` | `structure` | `clientToken`, `dataFilterExpression`, `description`, `name`, `resourceTags`, `sourceViews` | - |
| `CreateBillingViewResponse` | `structure` | `arn`, `createdAt` | - |
| `DeleteBillingViewRequest` | `structure` | `arn`, `force` | - |
| `DeleteBillingViewResponse` | `structure` | `arn` | - |
| `DisassociateSourceViewsRequest` | `structure` | `arn`, `sourceViews` | - |
| `DisassociateSourceViewsResponse` | `structure` | `arn` | - |
| `GetBillingViewRequest` | `structure` | `arn` | - |
| `GetBillingViewResponse` | `structure` | `billingView` | - |
| `GetResourcePolicyRequest` | `structure` | `resourceArn` | - |
| `GetResourcePolicyResponse` | `structure` | `policy`, `resourceArn` | - |
| `ListBillingViewsRequest` | `structure` | `activeTimeRange`, `arns`, `billingViewTypes`, `maxResults`, `names`, `nextToken`, `ownerAccountId`, `sourceAccountId` | - |
| `ListBillingViewsResponse` | `structure` | `billingViews`, `nextToken` | - |
| `ListSourceViewsForBillingViewRequest` | `structure` | `arn`, `maxResults`, `nextToken` | - |
| `ListSourceViewsForBillingViewResponse` | `structure` | `nextToken`, `sourceViews` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
