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

- Operations: `ListAccountAssociations`, `ListBillingGroupCostReports`, `ListBillingGroups`, `ListCustomLineItemVersions`, `ListCustomLineItems`, `ListPricingPlans`, `ListPricingPlansAssociatedWithPricingRule`, `ListPricingRules`, `ListPricingRulesAssociatedToPricingPlan`, `ListResourcesAssociatedToCustomLineItem`, `ListTagsForResource`
- Traits: `paginated` (10), `readonly` (10)
- Common required input members in this group: `Arn`, `PricingPlanArn`, `PricingRuleArn`, `ResourceArn`

### Create

- Operations: `CreateBillingGroup`, `CreateCustomLineItem`, `CreatePricingPlan`, `CreatePricingRule`
- Traits: `idempotency-token` (4), `idempotent` (3)
- Common required input members in this group: `AccountGrouping`, `BillingGroupArn`, `ChargeDetails`, `ComputationPreference`, `Description`, `Name`, `Scope`, `Type`

### Delete

- Operations: `DeleteBillingGroup`, `DeleteCustomLineItem`, `DeletePricingPlan`, `DeletePricingRule`
- Traits: `idempotent` (4)
- Common required input members in this group: `Arn`

### Update

- Operations: `UpdateBillingGroup`, `UpdateCustomLineItem`, `UpdatePricingPlan`, `UpdatePricingRule`
- Traits: `idempotent` (4)
- Common required input members in this group: `Arn`

### Associate

- Operations: `AssociateAccounts`, `AssociatePricingRules`
- Traits: `idempotent` (2)
- Common required input members in this group: `AccountIds`, `Arn`, `PricingRuleArns`

### Batch

- Operations: `BatchAssociateResourcesToCustomLineItem`, `BatchDisassociateResourcesFromCustomLineItem`
- Traits: `idempotent` (2)
- Common required input members in this group: `ResourceArns`, `TargetArn`

### Disassociate

- Operations: `DisassociateAccounts`, `DisassociatePricingRules`
- Traits: `idempotent` (2)
- Common required input members in this group: `AccountIds`, `Arn`, `PricingRuleArns`

### Get

- Operations: `GetBillingGroupCostReport`
- Traits: `paginated` (1), `readonly` (1)
- Common required input members in this group: `Arn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAccounts` | `POST /associate-accounts` | `idempotent` | `AccountIds`, `Arn` | - | `AssociateAccountsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Connects an array of account IDs in a consolidated billing family to a predefined billing group. The account IDs must be a part of the consolidated billing family during the current month, and not already associated with another billing group. |
| `AssociatePricingRules` | `PUT /associate-pricing-rules` | `idempotent` | `Arn`, `PricingRuleArns` | - | `AssociatePricingRulesOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Connects an array of `PricingRuleArns` to a defined `PricingPlan`. The maximum number `PricingRuleArn` that can be associated in one call is 30. |
| `BatchAssociateResourcesToCustomLineItem` | `PUT /batch-associate-resources-to-custom-line-item` | `idempotent` | `ResourceArns`, `TargetArn` | - | `BatchAssociateResourcesToCustomLineItemOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Associates a batch of resources to a percentage custom line item. |
| `BatchDisassociateResourcesFromCustomLineItem` | `PUT /batch-disassociate-resources-from-custom-line-item` | `idempotent` | `ResourceArns`, `TargetArn` | - | `BatchDisassociateResourcesFromCustomLineItemOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a batch of resources from a percentage custom line item. |
| `CreateBillingGroup` | `POST /create-billing-group` | `idempotent`, `idempotency-token` | `AccountGrouping`, `ComputationPreference`, `Name` | `ClientToken` | `CreateBillingGroupOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a billing group that resembles a consolidated billing family that Amazon Web Services charges, based off of the predefined pricing plan computation. |
| `CreateCustomLineItem` | `POST /create-custom-line-item` | `idempotency-token` | `BillingGroupArn`, `ChargeDetails`, `Description`, `Name` | `ClientToken` | `CreateCustomLineItemOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a custom line item that can be used to create a one-time fixed charge that can be applied to a single billing group for the current or previous billing period. The one-time fixed charge is either a fee or discount. |
| `CreatePricingPlan` | `POST /create-pricing-plan` | `idempotent`, `idempotency-token` | `Name` | `ClientToken` | `CreatePricingPlanOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a pricing plan that is used for computing Amazon Web Services charges for billing groups. |
| `CreatePricingRule` | `POST /create-pricing-rule` | `idempotent`, `idempotency-token` | `Name`, `Scope`, `Type` | `ClientToken` | `CreatePricingRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a pricing rule can be associated to a pricing plan, or a set of pricing plans. |
| `DeleteBillingGroup` | `POST /delete-billing-group` | `idempotent` | `Arn` | - | `DeleteBillingGroupOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a billing group. |
| `DeleteCustomLineItem` | `POST /delete-custom-line-item` | `idempotent` | `Arn` | - | `DeleteCustomLineItemOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the custom line item identified by the given ARN in the current, or previous billing period. |
| `DeletePricingPlan` | `POST /delete-pricing-plan` | `idempotent` | `Arn` | - | `DeletePricingPlanOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a pricing plan. The pricing plan must not be associated with any billing groups to delete successfully. |
| `DeletePricingRule` | `POST /delete-pricing-rule` | `idempotent` | `Arn` | - | `DeletePricingRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the pricing rule that's identified by the input Amazon Resource Name (ARN). |
| `DisassociateAccounts` | `POST /disassociate-accounts` | `idempotent` | `AccountIds`, `Arn` | - | `DisassociateAccountsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified list of account IDs from the given billing group. |
| `DisassociatePricingRules` | `PUT /disassociate-pricing-rules` | `idempotent` | `Arn`, `PricingRuleArns` | - | `DisassociatePricingRulesOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a list of pricing rules from a pricing plan. |
| `GetBillingGroupCostReport` | `POST /get-billing-group-cost-report` | `readonly`, `paginated` | `Arn` | - | `GetBillingGroupCostReportOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the margin summary report, which includes the Amazon Web Services cost and charged amount (pro forma cost) by Amazon Web Services service for a specific billing group. |
| `ListAccountAssociations` | `POST /list-account-associations` | `readonly`, `paginated` | - | - | `ListAccountAssociationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This is a paginated call to list linked accounts that are linked to the payer account for the specified time period. If no information is provided, the current billing period is used. |
| `ListBillingGroupCostReports` | `POST /list-billing-group-cost-reports` | `readonly`, `paginated` | - | - | `ListBillingGroupCostReportsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A paginated call to retrieve a summary report of actual Amazon Web Services charges and the calculated Amazon Web Services charges based on the associated pricing plan of a billing group. |
| `ListBillingGroups` | `POST /list-billing-groups` | `readonly`, `paginated` | - | - | `ListBillingGroupsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A paginated call to retrieve a list of billing groups for the given billing period. If you don't provide a billing group, the current billing period is used. |
| `ListCustomLineItemVersions` | `POST /list-custom-line-item-versions` | `readonly`, `paginated` | `Arn` | - | `ListCustomLineItemVersionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | A paginated call to get a list of all custom line item versions. |
| `ListCustomLineItems` | `POST /list-custom-line-items` | `readonly`, `paginated` | - | - | `ListCustomLineItemsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A paginated call to get a list of all custom line items (FFLIs) for the given billing period. If you don't provide a billing period, the current billing period is used. |
| `ListPricingPlans` | `POST /list-pricing-plans` | `readonly`, `paginated` | - | - | `ListPricingPlansOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | A paginated call to get pricing plans for the given billing period. If you don't provide a billing period, the current billing period is used. |
| `ListPricingPlansAssociatedWithPricingRule` | `POST /list-pricing-plans-associated-with-pricing-rule` | `readonly`, `paginated` | `PricingRuleArn` | - | `ListPricingPlansAssociatedWithPricingRuleOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A list of the pricing plans that are associated with a pricing rule. |
| `ListPricingRules` | `POST /list-pricing-rules` | `readonly`, `paginated` | - | - | `ListPricingRulesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Describes a pricing rule that can be associated to a pricing plan, or set of pricing plans. |
| `ListPricingRulesAssociatedToPricingPlan` | `POST /list-pricing-rules-associated-to-pricing-plan` | `readonly`, `paginated` | `PricingPlanArn` | - | `ListPricingRulesAssociatedToPricingPlanOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the pricing rules that are associated with a pricing plan. |
| `ListResourcesAssociatedToCustomLineItem` | `POST /list-resources-associated-to-custom-line-item` | `readonly`, `paginated` | `Arn` | - | `ListResourcesAssociatedToCustomLineItemOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the resources that are associated to a custom line item. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A list the tags for a resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates the specified tags to a resource with the specified `resourceArn`. If existing tags on a resource are not specified in the request parameters, they are not changed. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes specified tags from a resource. |
| `UpdateBillingGroup` | `POST /update-billing-group` | `idempotent` | `Arn` | - | `UpdateBillingGroupOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This updates an existing billing group. |
| `UpdateCustomLineItem` | `POST /update-custom-line-item` | `idempotent` | `Arn` | - | `UpdateCustomLineItemOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an existing custom line item in the current or previous billing period. |
| `UpdatePricingPlan` | `PUT /update-pricing-plan` | `idempotent` | `Arn` | - | `UpdatePricingPlanOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This updates an existing pricing plan. |
| `UpdatePricingRule` | `PUT /update-pricing-rule` | `idempotent` | `Arn` | - | `UpdatePricingRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing pricing rule. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message`, `RetryAfterSeconds` | An unexpected error occurred while processing a request. |
| `ThrottlingException` | `structure` | `Message`, `RetryAfterSeconds` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Fields`, `Message`, `Reason` | The input doesn't match with the constraints specified by Amazon Web Services services. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The request references a resource that doesn't exist. |
| `ConflictException` | `structure` | `Message`, `Reason`, `ResourceId`, `ResourceType` | You can cause an inconsistent state by updating or deleting a resource. |
| `ServiceLimitExceededException` | `structure` | `LimitCode`, `Message`, `ResourceId`, `ResourceType`, `ServiceCode` | The request would cause a service limit to exceed. |
| `AssociateAccountsInput` | `structure` | `AccountIds`, `Arn` | - |
| `AssociateAccountsOutput` | `structure` | `Arn` | - |
| `AssociatePricingRulesInput` | `structure` | `Arn`, `PricingRuleArns` | - |
| `AssociatePricingRulesOutput` | `structure` | `Arn` | - |
| `BatchAssociateResourcesToCustomLineItemInput` | `structure` | `BillingPeriodRange`, `ResourceArns`, `TargetArn` | - |
| `BatchAssociateResourcesToCustomLineItemOutput` | `structure` | `FailedAssociatedResources`, `SuccessfullyAssociatedResources` | - |
| `BatchDisassociateResourcesFromCustomLineItemInput` | `structure` | `BillingPeriodRange`, `ResourceArns`, `TargetArn` | - |
| `BatchDisassociateResourcesFromCustomLineItemOutput` | `structure` | `FailedDisassociatedResources`, `SuccessfullyDisassociatedResources` | - |
| `CreateBillingGroupInput` | `structure` | `AccountGrouping`, `ClientToken`, `ComputationPreference`, `Description`, `Name`, `PrimaryAccountId`, `Tags` | - |
| `CreateBillingGroupOutput` | `structure` | `Arn` | - |
| `CreateCustomLineItemInput` | `structure` | `AccountId`, `BillingGroupArn`, `BillingPeriodRange`, `ChargeDetails`, `ClientToken`, `ComputationRule`, `Description`, `Name`, `PresentationDetails`, `Tags` | - |
| `CreateCustomLineItemOutput` | `structure` | `Arn` | - |
| `CreatePricingPlanInput` | `structure` | `ClientToken`, `Description`, `Name`, `PricingRuleArns`, `Tags` | - |
| `CreatePricingPlanOutput` | `structure` | `Arn` | - |
| `CreatePricingRuleInput` | `structure` | `BillingEntity`, `ClientToken`, `Description`, `ModifierPercentage`, `Name`, `Operation`, `Scope`, `Service`, `Tags`, `Tiering`, `Type`, `UsageType` | - |
| `CreatePricingRuleOutput` | `structure` | `Arn` | - |
| `DeleteBillingGroupInput` | `structure` | `Arn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
