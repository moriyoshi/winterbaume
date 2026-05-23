# AWSBillingConductor

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Billing Conductor is a fully managed service that you can use to customize a pro forma version of your billing data each month, to accurately show or chargeback your end customers. Billing Conductor doesn't change the way you're billed by Amazon Web Services each month by design. Instead, it provides you with a mechanism to configure, generate, and display rates to certain customers over a given billing period. You can also analyze the difference between the rates you apply to your accounting groupings relative to your actual rates from Amazon Web Services. As a result of your Billing Conductor configuration, the payer account can also see the custom rate applied on the billing details page of the Billing console, or configure a cost and usage report per billing group.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWSBillingConductor where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWSBillingConductor by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: create billing groups, custom line items, pricing plans, pricing rules, and account associations.
- From the operation surface: model pro forma billing, account grouping, pricing overrides, custom charge lines, and tag-based billing operations.

## Service Identity and Protocol

- AWS model slug: `billingconductor`
- AWS SDK for Rust slug: `billingconductor`
- Model version: `2021-07-30`
- Model file: `vendor/api-models-aws/models/billingconductor/service/2021-07-30/billingconductor-2021-07-30.json`
- SDK ID: `billingconductor`
- Endpoint prefix: `-`
- ARN namespace: `billingconductor`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (11), `Create` (4), `Delete` (4), `Update` (4), `Associate` (2), `Batch` (2), `Disassociate` (2), `Get` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAccounts`, `AssociatePricingRules`, `BatchAssociateResourcesToCustomLineItem`, `BatchDisassociateResourcesFromCustomLineItem`, `CreateBillingGroup`, `CreateCustomLineItem`, `CreatePricingPlan`, `CreatePricingRule`, `DeleteBillingGroup`, `DeleteCustomLineItem`, `DeletePricingPlan`, `DeletePricingRule`, `DisassociateAccounts`, `DisassociatePricingRules`, `TagResource`, `UntagResource`, `UpdateBillingGroup`, `UpdateCustomLineItem`, `UpdatePricingPlan`, `UpdatePricingRule`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBillingGroupCostReport`, `ListAccountAssociations`, `ListBillingGroupCostReports`, `ListBillingGroups`, `ListCustomLineItemVersions`, `ListCustomLineItems`, `ListPricingPlans`, `ListPricingPlansAssociatedWithPricingRule`, `ListPricingRules`, `ListPricingRulesAssociatedToPricingPlan`, `ListResourcesAssociatedToCustomLineItem`, `ListTagsForResource`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 18 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetBillingGroupCostReport`, `ListBillingGroupCostReports`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 32 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BillingGroup` | `Arn` | create: `CreateBillingGroup`; update: `UpdateBillingGroup`; delete: `DeleteBillingGroup`; list: `ListBillingGroups` | `AssociateAccounts`, `DisassociateAccounts` | A billing group is a set of linked account which belong to the same end customer. |
| `CustomLineItem` | `Arn` | create: `CreateCustomLineItem`; update: `UpdateCustomLineItem`; delete: `DeleteCustomLineItem`; list: `ListCustomLineItems` | `BatchAssociateResourcesToCustomLineItem`, `BatchDisassociateResourcesFromCustomLineItem`, `ListCustomLineItemVersions`, `ListResourcesAssociatedToCustomLineItem` | Represents the custom line item |
| `PricingPlan` | `Arn` | create: `CreatePricingPlan`; update: `UpdatePricingPlan`; delete: `DeletePricingPlan`; list: `ListPricingPlans` | `ListPricingPlansAssociatedWithPricingRule`, `AssociatePricingRules`, `DisassociatePricingRules` | Pricing Plan enables you to customize your billing details consistent with the usage that accrues in each of your billing groups. |
| `PricingRule` | `Arn` | create: `CreatePricingRule`; update: `UpdatePricingRule`; delete: `DeletePricingRule`; list: `ListPricingRules` | `ListPricingRulesAssociatedToPricingPlan` | A markup/discount that is defined for a specific set of services that can later be associated with a pricing plan. |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/billingconductor/latest/userguide/understanding-proforma.html
- https://docs.aws.amazon.com/billingconductor/latest/userguide/custom-pricing-view.html
- https://docs.aws.amazon.com/billingconductor/latest/userguide/viewing-in-billing.html

Research outcomes:
- AWS Billing Conductor creates pro forma billing data for showback and chargeback without changing the standard AWS invoice.
- Billing groups define which accounts are billed together under custom pricing.
- Pricing plans and pricing rules customise rates used for pro forma calculations.
- Custom line items add credits or charges to pro forma bills.
- Pro forma costs can be viewed in Billing and Cost Management views and supported services such as Cost Explorer and Budgets.
- Pro forma billing data is separate from chargeable AWS billing data and should not be treated as the legal invoice.

Parity implications:
- Model billing groups, account associations, pricing plans, pricing rules, custom line items, billing views, and pro forma calculations separately.
- Pro forma data should affect supported cost views but not issued invoices or actual charges.
- Account movement between billing groups should affect future pro forma billing periods according to Billing Conductor rules.

## Operation Groups

### List

- Operations: `ListAccountAssociations`, `ListBillingGroupCostReports`, `ListTagsForResource`
- Traits: `readonly` (2), `paginated` (2)
- Common required input members in this group: -

### Get

- Operations: `GetBillingGroupCostReport`
- Traits: `readonly` (1), `paginated` (1)
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
| `GetBillingGroupCostReport` | `POST /get-billing-group-cost-report` | `readonly`, `paginated` | `Arn` | - | `GetBillingGroupCostReportOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the margin summary report, which includes the Amazon Web Services cost and charged amount (pro forma cost) by Amazon Web Services service for a specific billing group. |
| `ListAccountAssociations` | `POST /list-account-associations` | `readonly`, `paginated` | - | - | `ListAccountAssociationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This is a paginated call to list linked accounts that are linked to the payer account for the specified time period. If no information is provided, the current billing period is used. The response will optionally inc ... |
| `ListBillingGroupCostReports` | `POST /list-billing-group-cost-reports` | `readonly`, `paginated` | - | - | `ListBillingGroupCostReportsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A paginated call to retrieve a summary report of actual Amazon Web Services charges and the calculated Amazon Web Services charges based on the associated pricing plan of a billing group. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A list the tags for a resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates the specified tags to a resource with the specified resourceArn . If existing tags on a resource are not specified in the request parameters, they are not changed. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes specified tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message, ResourceId, ResourceType, Reason | You can cause an inconsistent state by updating or deleting a resource. |
| `InternalServerException` | `structure` | Message, RetryAfterSeconds | An unexpected error occurred while processing a request. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The request references a resource that doesn't exist. |
| `ServiceLimitExceededException` | `structure` | Message, ResourceId, ResourceType, LimitCode, ServiceCode | The request would cause a service limit to exceed. |
| `ThrottlingException` | `structure` | Message, RetryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | Message, Reason, Fields | The input doesn't match with the constraints specified by Amazon Web Services services. |
| `GetBillingGroupCostReportInput` | `structure` | Arn, BillingPeriodRange, GroupBy, MaxResults, NextToken | - |
| `GetBillingGroupCostReportOutput` | `structure` | BillingGroupCostReportResults, NextToken | - |
| `ListAccountAssociationsInput` | `structure` | BillingPeriod, Filters, NextToken | - |
| `ListAccountAssociationsOutput` | `structure` | LinkedAccounts, NextToken | - |
| `ListBillingGroupCostReportsInput` | `structure` | BillingPeriod, MaxResults, NextToken, Filters | - |
| `ListBillingGroupCostReportsOutput` | `structure` | BillingGroupCostReports, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AssociateResourceErrorReason` | `enum` | INVALID_ARN, SERVICE_LIMIT_EXCEEDED, ILLEGAL_CUSTOMLINEITEM, INTERNAL_SERVER_EXCEPTION, INVALID_BILLING_PERIOD_RANGE | - |
| `BillingGroupStatus` | `enum` | ACTIVE, PRIMARY_ACCOUNT_MISSING, PENDING | - |
| `BillingGroupType` | `enum` | STANDARD, TRANSFER_BILLING | - |
| `ComputationRuleEnum` | `enum` | ITEMIZED, CONSOLIDATED | The display settings of the custom line item |
| `ConflictExceptionReason` | `enum` | RESOURCE_NAME_CONFLICT, PRICING_RULE_IN_PRICING_PLAN_CONFLICT, PRICING_PLAN_ATTACHED_TO_BILLING_GROUP_DELETE_CONFLICT, PRICING_RULE_ATTACHED_TO_PRICING_PLAN_DELETE_CONFLICT, WRITE_CONFLICT_RETRY | - |
| `CurrencyCode` | `enum` | USD, CNY | - |
| `CustomLineItemRelationship` | `enum` | PARENT, CHILD | - |
| `CustomLineItemType` | `enum` | CREDIT, FEE | - |
| `GroupByAttributeName` | `enum` | PRODUCT_NAME, BILLING_PERIOD | - |
| `LineItemFilterAttributeName` | `enum` | LINE_ITEM_TYPE, SERVICE | - |
| `LineItemFilterValue` | `enum` | SAVINGS_PLAN_NEGATION | - |
| `MatchOption` | `enum` | NOT_EQUAL, EQUAL | - |
| `PricingRuleScope` | `enum` | GLOBAL, SERVICE, BILLING_ENTITY, SKU | - |
| `PricingRuleType` | `enum` | MARKUP, DISCOUNT, TIERING | - |
| `SearchOption` | `enum` | STARTS_WITH | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER, PRIMARY_NOT_ASSOCIATED, PRIMARY_CANNOT_DISASSOCIATE, ACCOUNTS_NOT_ASSOCIATED, ACCOUNTS_ALREADY_ASSOCIATED, ILLEGAL_PRIMARY_ACCOUNT, ILLEGAL_ACCOUNTS, MISMATCHED_BILLINGGROUP_ARN, MISSING_BILLINGGROUP, ... (+54) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
