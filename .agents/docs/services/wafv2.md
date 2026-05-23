# AWS WAFV2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

WAF This is the latest version of the WAF API, released in November, 2019. The names of the entities that you use to access this API, like endpoints and namespaces, all have the versioning information added, like "V2" or "v2", to distinguish from the prior version. We recommend migrating your resources to this version, because it has a number of significant improvements. If you used WAF prior to this release, you can't use this WAFV2 API to access any WAF resources that you created before. WAF Classic support will end on September 30, 2025.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-wafv2/tests/scenario_test.rs`: build an IP block-list pipeline with IP sets, rule groups, web ACLs, updates, and cleanup.
- Backported from `scenario_test.rs`: associate and disassociate a web ACL with an application resource.
- Backported from `scenario_test.rs`: manage tags and resource policies across WAF resource types.
- Scenario insight from EC2: include mutable binding failover for AWS WAFV2 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS WAFV2 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: support web ACL protection, managed/custom rule groups, IP/regex pattern sets, logging, resource associations, lock-token update semantics, tags, and policy-based sharing.

## Service Identity and Protocol

- AWS model slug: `wafv2`
- AWS SDK for Rust slug: `wafv2`
- Model version: `2019-07-29`
- Model file: `vendor/api-models-aws/models/wafv2/service/2019-07-29/wafv2-2019-07-29.json`
- SDK ID: `WAFV2`
- Endpoint prefix: `wafv2`
- ARN namespace: `wafv2`
- CloudFormation name: `WAFv2`
- CloudTrail event source: `wafv2.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (13), `List` (12), `Delete` (8), `Create` (5), `Update` (5), `Describe` (3), `Put` (3), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateWebACL`, `CreateAPIKey`, `CreateIPSet`, `CreateRegexPatternSet`, `CreateRuleGroup`, `CreateWebACL`, `DeleteAPIKey`, `DeleteFirewallManagerRuleGroups`, `DeleteIPSet`, `DeleteLoggingConfiguration`, `DeletePermissionPolicy`, `DeleteRegexPatternSet`, `DeleteRuleGroup`, `DeleteWebACL`, `DisassociateWebACL`, `PutLoggingConfiguration`, `PutManagedRuleSetVersions`, `PutPermissionPolicy`, `TagResource`, `UntagResource`, ... (+5).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CheckCapacity`, `DescribeAllManagedProducts`, `DescribeManagedProductsByVendor`, `DescribeManagedRuleGroup`, `GetDecryptedAPIKey`, `GetIPSet`, `GetLoggingConfiguration`, `GetManagedRuleSet`, `GetMobileSdkRelease`, `GetPermissionPolicy`, `GetRateBasedStatementManagedKeys`, `GetRegexPatternSet`, `GetRuleGroup`, `GetSampledRequests`, `GetTopPathStatisticsByTraffic`, `GetWebACL`, `GetWebACLForResource`, `ListAPIKeys`, `ListAvailableManagedRuleGroupVersions`, `ListAvailableManagedRuleGroups`, ... (+9).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 55 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/waf/latest/developerguide/web-acl-processing-order.html
- https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-action.html
- https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-statement-type-rate-based.html

Research outcomes:
- Web ACL and rule group rules each have unique numeric priorities inside their container.
- AWS WAF evaluates rules from lowest numeric priority upward until it reaches a terminating match or exhausts all rules.
- Rule actions include Allow, Block, Count, CAPTCHA, and Challenge.
- Allow and Block are terminating actions. Allow can insert custom request headers, and Block defaults to HTTP 403 but can return a custom response.
- Count is non-terminating. WAF continues evaluating later rules and can add labels or custom request headers.
- CAPTCHA and Challenge terminate or continue depending on token state. Valid unexpired tokens behave like Count; invalid or expired tokens block or challenge the request.
- CAPTCHA and Challenge are documented for browsers over HTTPS secure contexts.
- Rate-based rules aggregate requests by configured criteria and rate-limit by evaluation window, request limit, and action.
- Each rate-based rule instance tracks requests independently, even when multiple web ACLs or rule groups use the same settings.
- Rate-based statements cannot be nested inside other statements and have WCU costs based on base cost, custom aggregation keys, and scope-down statement cost.

Parity implications:
- Model web ACLs, rule groups, rule priorities, statements, actions, labels, managed rule groups, rate counters, lock tokens, and associated resources separately.
- Request evaluation should implement priority ordering, terminating/non-terminating actions, labels, default action, and rate-counter scope.
- CAPTCHA and Challenge can be approximated by token-state semantics even when the visual browser challenge itself is out of scope.

## v1/v2 State Coherence

- **Paired with `waf` ( different SDK slug, complete rewrite ).** AWS WAF ( WAFv2, this crate, released November 2019 ) and AWS WAF Classic are **completely separate services** in real AWS. Web ACLs, IP sets, regex pattern sets, and rule groups in WAFv2 are not visible to WAF Classic `ListWebACLs` / `ListIPSets` / `ListRuleGroups` and vice versa. AWS provides an explicit migration tool ( `CreateWebACLMigrationStack` on the Classic API ) to convert Classic resources into a WAFv2-compatible CloudFormation template; there is no shared backend.
- WAF Classic is going through a planned end-of-life process ( support ends 30 September 2025 per AWS docs ); customers are expected to migrate to WAFv2 — but the migration is one-way and resource-by-resource.
- **Current Winterbaume status: correctly separate.** `winterbaume-wafv2` and `winterbaume-waf` each own their own state, mirroring real AWS. No dependency between the crates is needed; do not introduce cross-API visibility. Note also that WAFv2's `Scope` parameter ( `CLOUDFRONT` vs `REGIONAL` ) further partitions WAFv2's own state — REGIONAL Web ACLs and CLOUDFRONT Web ACLs do not share a namespace within WAFv2.

## Operation Groups

### Get

- Operations: `GetDecryptedAPIKey`, `GetIPSet`, `GetLoggingConfiguration`, `GetManagedRuleSet`, `GetMobileSdkRelease`, `GetPermissionPolicy`, `GetRateBasedStatementManagedKeys`, `GetRegexPatternSet`, `GetRuleGroup`, `GetSampledRequests`, `GetTopPathStatisticsByTraffic`, `GetWebACL`, `GetWebACLForResource`
- Common required input members in this group: `Scope`, `Name`, `Id`, `ResourceArn`, `WebAclArn`, `TimeWindow`

### List

- Operations: `ListAPIKeys`, `ListAvailableManagedRuleGroups`, `ListAvailableManagedRuleGroupVersions`, `ListIPSets`, `ListLoggingConfigurations`, `ListManagedRuleSets`, `ListMobileSdkReleases`, `ListRegexPatternSets`, `ListResourcesForWebACL`, `ListRuleGroups`, `ListTagsForResource`, `ListWebACLs`
- Common required input members in this group: `Scope`

### Delete

- Operations: `DeleteAPIKey`, `DeleteFirewallManagerRuleGroups`, `DeleteIPSet`, `DeleteLoggingConfiguration`, `DeletePermissionPolicy`, `DeleteRegexPatternSet`, `DeleteRuleGroup`, `DeleteWebACL`
- Common required input members in this group: `Scope`, `Name`, `Id`, `LockToken`, `ResourceArn`

### Create

- Operations: `CreateAPIKey`, `CreateIPSet`, `CreateRegexPatternSet`, `CreateRuleGroup`, `CreateWebACL`
- Common required input members in this group: `Scope`, `Name`, `VisibilityConfig`

### Update

- Operations: `UpdateIPSet`, `UpdateManagedRuleSetVersionExpiryDate`, `UpdateRegexPatternSet`, `UpdateRuleGroup`, `UpdateWebACL`
- Common required input members in this group: `Name`, `Scope`, `Id`, `LockToken`, `VisibilityConfig`

### Describe

- Operations: `DescribeAllManagedProducts`, `DescribeManagedProductsByVendor`, `DescribeManagedRuleGroup`
- Common required input members in this group: `Scope`, `VendorName`

### Put

- Operations: `PutLoggingConfiguration`, `PutManagedRuleSetVersions`, `PutPermissionPolicy`
- Common required input members in this group: -

### Associate

- Operations: `AssociateWebACL`
- Common required input members in this group: -

### Check

- Operations: `CheckCapacity`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateWebACL`
- Common required input members in this group: -

### Generate

- Operations: `GenerateMobileSdkReleaseUrl`
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
| `AssociateWebACL` | `-` | - | `WebACLArn`, `ResourceArn` | - | `AssociateWebACLResponse` | `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFUnavailableEntityException` | Associates a web ACL with a resource, to protect the resource. Use this for all resource types except for Amazon CloudFront distributions. For Amazon CloudFront, call UpdateDistribution for the distribution and provi ... |
| `CheckCapacity` | `-` | - | `Scope`, `Rules` | - | `CheckCapacityResponse` | `WAFExpiredManagedRuleGroupVersionException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFSubscriptionNotFoundException`, `WAFUnavailableEntityException` | Returns the web ACL capacity unit (WCU) requirements for a specified scope and set of rules. You can use this to check the capacity requirements for the rules you want to use in a RuleGroup or WebACL . WAF uses WCUs ... |
| `CreateAPIKey` | `-` | - | `Scope`, `TokenDomains` | - | `CreateAPIKeyResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException` | Creates an API key that contains a set of token domains. API keys are required for the integration of the CAPTCHA API in your JavaScript client applications. The API lets you customize the placement and characteristi ... |
| `CreateIPSet` | `-` | - | `Name`, `Scope`, `IPAddressVersion`, `Addresses` | - | `CreateIPSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Creates an IPSet , which you use to identify web requests that originate from specific IP addresses or ranges of IP addresses. For example, if you're receiving a lot of requests from a ranges of IP addresses, you can ... |
| `CreateRegexPatternSet` | `-` | - | `Name`, `Scope`, `RegularExpressionList` | - | `CreateRegexPatternSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Creates a RegexPatternSet , which you reference in a RegexPatternSetReferenceStatement , to have WAF inspect a web request component for the specified patterns. |
| `CreateRuleGroup` | `-` | - | `Name`, `Scope`, `Capacity`, `VisibilityConfig` | - | `CreateRuleGroupResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFSubscriptionNotFoundException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException`, `WAFUnavailableEntityException` | Creates a RuleGroup per the specifications provided. A rule group defines a collection of rules to inspect and control web requests that you can use in a WebACL . When you create a rule group, you define an immutable ... |
| `CreateWebACL` | `-` | - | `Name`, `Scope`, `DefaultAction`, `VisibilityConfig` | - | `CreateWebACLResponse` | `WAFConfigurationWarningException`, `WAFDuplicateItemException`, `WAFExpiredManagedRuleGroupVersionException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFSubscriptionNotFoundException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException`, `WAFUnavailableEntityException` | Creates a WebACL per the specifications provided. A web ACL defines a collection of rules to use to inspect and control web requests. Each rule has a statement that defines what to look for in web requests and an act ... |
| `DeleteAPIKey` | `-` | - | `Scope`, `APIKey` | - | `DeleteAPIKeyResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Deletes the specified API key. After you delete a key, it can take up to 24 hours for WAF to disallow use of the key in all regions. |
| `DeleteFirewallManagerRuleGroups` | `-` | - | `WebACLArn`, `WebACLLockToken` | - | `DeleteFirewallManagerRuleGroupsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Deletes all rule groups that are managed by Firewall Manager from the specified WebACL . You can only use this if ManagedByFirewallManager and RetrofittedByFirewallManager are both false in the web ACL. |
| `DeleteIPSet` | `-` | - | `Name`, `Scope`, `Id`, `LockToken` | - | `DeleteIPSetResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified IPSet . |
| `DeleteLoggingConfiguration` | `-` | - | `ResourceArn` | - | `DeleteLoggingConfigurationResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Deletes the LoggingConfiguration from the specified web ACL. |
| `DeletePermissionPolicy` | `-` | - | `ResourceArn` | - | `DeletePermissionPolicyResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Permanently deletes an IAM policy from the specified rule group. You must be the owner of the rule group to perform this operation. |
| `DeleteRegexPatternSet` | `-` | - | `Name`, `Scope`, `Id`, `LockToken` | - | `DeleteRegexPatternSetResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified RegexPatternSet . |
| `DeleteRuleGroup` | `-` | - | `Name`, `Scope`, `Id`, `LockToken` | - | `DeleteRuleGroupResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified RuleGroup . |
| `DeleteWebACL` | `-` | - | `Name`, `Scope`, `Id`, `LockToken` | - | `DeleteWebACLResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified WebACL . You can only use this if ManagedByFirewallManager is false in the web ACL. Before deleting any web ACL, first disassociate it from all resources. To retrieve a list of the resources tha ... |
| `DescribeAllManagedProducts` | `-` | - | `Scope` | - | `DescribeAllManagedProductsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Provides high-level information for the Amazon Web Services Managed Rules rule groups and Amazon Web Services Marketplace managed rule groups. |
| `DescribeManagedProductsByVendor` | `-` | - | `VendorName`, `Scope` | - | `DescribeManagedProductsByVendorResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Provides high-level information for the managed rule groups owned by a specific vendor. |
| `DescribeManagedRuleGroup` | `-` | - | `VendorName`, `Name`, `Scope` | - | `DescribeManagedRuleGroupResponse` | `WAFExpiredManagedRuleGroupVersionException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFNonexistentItemException` | Provides high-level information for a managed rule group, including descriptions of the rules. |
| `DisassociateWebACL` | `-` | - | `ResourceArn` | - | `DisassociateWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Disassociates the specified resource from its web ACL association, if it has one. Use this for all resource types except for Amazon CloudFront distributions. For Amazon CloudFront, call UpdateDistribution for the dis ... |
| `GenerateMobileSdkReleaseUrl` | `-` | - | `Platform`, `ReleaseVersion` | - | `GenerateMobileSdkReleaseUrlResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Generates a presigned download URL for the specified release of the mobile SDK. The mobile SDK is not generally available. Customers who have access to the mobile SDK can use it to establish and manage WAF tokens for ... |
| `GetDecryptedAPIKey` | `-` | - | `Scope`, `APIKey` | - | `GetDecryptedAPIKeyResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFNonexistentItemException` | Returns your API key in decrypted form. Use this to check the token domains that you have defined for the key. API keys are required for the integration of the CAPTCHA API in your JavaScript client applications. The ... |
| `GetIPSet` | `-` | - | `Name`, `Scope`, `Id` | - | `GetIPSetResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified IPSet . |
| `GetLoggingConfiguration` | `-` | - | `ResourceArn` | - | `GetLoggingConfigurationResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Returns the LoggingConfiguration for the specified web ACL. |
| `GetManagedRuleSet` | `-` | - | `Name`, `Scope`, `Id` | - | `GetManagedRuleSetResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified managed rule set. This is intended for use only by vendors of managed rule sets. Vendors are Amazon Web Services and Amazon Web Services Marketplace sellers. Vendors, you can use the managed r ... |
| `GetMobileSdkRelease` | `-` | - | `Platform`, `ReleaseVersion` | - | `GetMobileSdkReleaseResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves information for the specified mobile SDK release, including release notes and tags. The mobile SDK is not generally available. Customers who have access to the mobile SDK can use it to establish and manage ... |
| `GetPermissionPolicy` | `-` | - | `ResourceArn` | - | `GetPermissionPolicyResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Returns the IAM policy that is attached to the specified rule group. You must be the owner of the rule group to perform this operation. |
| `GetRateBasedStatementManagedKeys` | `-` | - | `Scope`, `WebACLName`, `WebACLId`, `RuleName` | - | `GetRateBasedStatementManagedKeysResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFUnsupportedAggregateKeyTypeException` | Retrieves the IP addresses that are currently blocked by a rate-based rule instance. This is only available for rate-based rules that aggregate solely on the IP address or on the forwarded IP address. The maximum num ... |
| `GetRegexPatternSet` | `-` | - | `Name`, `Scope`, `Id` | - | `GetRegexPatternSetResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified RegexPatternSet . |
| `GetRuleGroup` | `-` | - | - | - | `GetRuleGroupResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified RuleGroup . |
| `GetSampledRequests` | `-` | - | `WebAclArn`, `RuleMetricName`, `Scope`, `TimeWindow`, `MaxItems` | - | `GetSampledRequestsResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Gets detailed information about a specified number of requests--a sample--that WAF randomly selects from among the first 5,000 requests that your Amazon Web Services resource received during a time range that you cho ... |
| `GetTopPathStatisticsByTraffic` | `-` | - | `WebAclArn`, `Scope`, `TimeWindow`, `Limit`, `NumberOfTopTrafficBotsPerPath` | - | `GetTopPathStatisticsByTrafficResponse` | `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves aggregated statistics about the top URI paths accessed by bot traffic for a specified web ACL and time window. You can use this operation to analyze which paths on your web application receive the most bot ... |
| `GetWebACL` | `-` | - | - | - | `GetWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified WebACL . |
| `GetWebACLForResource` | `-` | - | `ResourceArn` | - | `GetWebACLForResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFUnavailableEntityException` | Retrieves the WebACL for the specified resource. This call uses GetWebACL , to verify that your account has permission to access the retrieved web ACL. If you get an error that indicates that your account isn't autho ... |
| `ListAPIKeys` | `-` | - | `Scope` | - | `ListAPIKeysResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException` | Retrieves a list of the API keys that you've defined for the specified scope. API keys are required for the integration of the CAPTCHA API in your JavaScript client applications. The API lets you customize the placem ... |
| `ListAvailableManagedRuleGroups` | `-` | - | `Scope` | - | `ListAvailableManagedRuleGroupsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of managed rule groups that are available for you to use. This list includes all Amazon Web Services Managed Rules rule groups and all of the Amazon Web Services Marketplace managed rule groups tha ... |
| `ListAvailableManagedRuleGroupVersions` | `-` | - | `VendorName`, `Name`, `Scope` | - | `ListAvailableManagedRuleGroupVersionsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Returns a list of the available versions for the specified managed rule group. |
| `ListIPSets` | `-` | - | `Scope` | - | `ListIPSetsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of IPSetSummary objects for the IP sets that you manage. |
| `ListLoggingConfigurations` | `-` | - | `Scope` | - | `ListLoggingConfigurationsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of your LoggingConfiguration objects. |
| `ListManagedRuleSets` | `-` | - | `Scope` | - | `ListManagedRuleSetsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves the managed rule sets that you own. This is intended for use only by vendors of managed rule sets. Vendors are Amazon Web Services and Amazon Web Services Marketplace sellers. Vendors, you can use the manag ... |
| `ListMobileSdkReleases` | `-` | - | `Platform` | - | `ListMobileSdkReleasesResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves a list of the available releases for the mobile SDK and the specified device platform. The mobile SDK is not generally available. Customers who have access to the mobile SDK can use it to establish and mana ... |
| `ListRegexPatternSets` | `-` | - | `Scope` | - | `ListRegexPatternSetsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of RegexPatternSetSummary objects for the regex pattern sets that you manage. |
| `ListResourcesForWebACL` | `-` | - | `WebACLArn` | - | `ListResourcesForWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves an array of the Amazon Resource Names (ARNs) for the resources that are associated with the specified web ACL. For Amazon CloudFront, don't use this call. Instead, use the CloudFront call ListDistributionsB ... |
| `ListRuleGroups` | `-` | - | `Scope` | - | `ListRuleGroupsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of RuleGroupSummary objects for the rule groups that you manage. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Retrieves the TagInfoForResource for the specified resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to "cu ... |
| `ListWebACLs` | `-` | - | `Scope` | - | `ListWebACLsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of WebACLSummary objects for the web ACLs that you manage. |
| `PutLoggingConfiguration` | `-` | - | `LoggingConfiguration` | - | `PutLoggingConfigurationResponse` | `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFLogDestinationPermissionIssueException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFServiceLinkedRoleErrorException` | Enables the specified LoggingConfiguration , to start logging from a web ACL, according to the configuration provided. If you configure data protection for the web ACL, the protection applies to the data that WAF sen ... |
| `PutManagedRuleSetVersions` | `-` | - | `Name`, `Scope`, `Id`, `LockToken` | - | `PutManagedRuleSetVersionsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Defines the versions of your managed rule set that you are offering to the customers. Customers see your offerings as managed rule groups with versioning. This is intended for use only by vendors of managed rule sets ... |
| `PutPermissionPolicy` | `-` | - | `ResourceArn`, `Policy` | - | `PutPermissionPolicyResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFInvalidPermissionPolicyException`, `WAFNonexistentItemException` | Use this to share a rule group with other accounts. This action attaches an IAM policy to the specified resource. You must be the owner of the rule group to perform this operation. This action is subject to the follo ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Associates tags with the specified Amazon Web Services resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to ... |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Disassociates tags from an Amazon Web Services resource. Tags are key:value pairs that you can associate with Amazon Web Services resources. For example, the tag key might be "customer" and the tag value might be "co ... |
| `UpdateIPSet` | `-` | - | `Name`, `Scope`, `Id`, `Addresses`, `LockToken` | - | `UpdateIPSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Updates the specified IPSet . This operation completely replaces the mutable specifications that you already have for the IP set with the ones that you provide to this call. To modify an IP set, do the following: Ret ... |
| `UpdateManagedRuleSetVersionExpiryDate` | `-` | - | `Name`, `Scope`, `Id`, `LockToken`, `VersionToExpire`, `ExpiryTimestamp` | - | `UpdateManagedRuleSetVersionExpiryDateResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Updates the expiration information for your managed rule set. Use this to initiate the expiration of a managed rule group version. After you initiate expiration for a version, WAF excludes it from the response to Lis ... |
| `UpdateRegexPatternSet` | `-` | - | `Name`, `Scope`, `Id`, `RegularExpressionList`, `LockToken` | - | `UpdateRegexPatternSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Updates the specified RegexPatternSet . This operation completely replaces the mutable specifications that you already have for the regex pattern set with the ones that you provide to this call. To modify a regex pat ... |
| `UpdateRuleGroup` | `-` | - | `Name`, `Scope`, `Id`, `VisibilityConfig`, `LockToken` | - | `UpdateRuleGroupResponse` | `WAFConfigurationWarningException`, `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFSubscriptionNotFoundException`, `WAFUnavailableEntityException` | Updates the specified RuleGroup . This operation completely replaces the mutable specifications that you already have for the rule group with the ones that you provide to this call. To modify a rule group, do the fol ... |
| `UpdateWebACL` | `-` | - | `Name`, `Scope`, `Id`, `DefaultAction`, `VisibilityConfig`, `LockToken` | - | `UpdateWebACLResponse` | `WAFConfigurationWarningException`, `WAFDuplicateItemException`, `WAFExpiredManagedRuleGroupVersionException`, `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFSubscriptionNotFoundException`, `WAFUnavailableEntityException` | Updates the specified WebACL . While updating a web ACL, WAF provides continuous coverage to the resources that you have associated with the web ACL. This operation completely replaces the mutable specifications that ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `WAFAssociatedItemException` | `structure` | Message | WAF couldn’t perform the operation because your resource is being used by another resource or it’s associated with another resource. |
| `WAFConfigurationWarningException` | `structure` | Message | The operation failed because you are inspecting the web request body, headers, or cookies without specifying how to handle oversize components. Rules that i ... |
| `WAFDuplicateItemException` | `structure` | Message | WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one. |
| `WAFExpiredManagedRuleGroupVersionException` | `structure` | Message | The operation failed because the specified version for the managed rule group has expired. You can retrieve the available versions for the managed rule grou ... |
| `WAFFeatureNotIncludedInPricingPlanException` | `structure` | Message, DisallowedFeatures | The operation failed because the specified WAF feature isn't supported by the CloudFront pricing plan associated with the web ACL. |
| `WAFInternalErrorException` | `structure` | Message | Your request is valid, but WAF couldn’t perform the operation because of a system problem. Retry your request. |
| `WAFInvalidOperationException` | `structure` | Message | The operation isn't valid. |
| `WAFInvalidParameterException` | `structure` | message, Field, Parameter, Reason | The operation failed because WAF didn't recognize a parameter in the request. For example: You specified a parameter name or value that isn't valid. Your ne ... |
| `WAFInvalidPermissionPolicyException` | `structure` | Message | The operation failed because the specified policy isn't in the proper format. The policy specifications must conform to the following: The policy must be co ... |
| `WAFInvalidResourceException` | `structure` | Message | WAF couldn’t perform the operation because the resource that you requested isn’t valid. Check the resource, and try again. |
| `WAFLimitsExceededException` | `structure` | Message, SourceType | WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of WebACL objects that you can create for an Am ... |
| `WAFLogDestinationPermissionIssueException` | `structure` | Message | The operation failed because you don't have the permissions that your logging configuration requires. For information, see Logging web ACL traffic informati ... |
| `WAFNonexistentItemException` | `structure` | Message | WAF couldn’t perform the operation because your resource doesn't exist. If you've just created a resource that you're using in this operation, you might jus ... |
| `WAFOptimisticLockException` | `structure` | Message | WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make a ... |
| `WAFServiceLinkedRoleErrorException` | `structure` | message | WAF is not able to access the service linked role. This can be caused by a previous PutLoggingConfiguration request, which can lock the service linked role ... |
| `WAFSubscriptionNotFoundException` | `structure` | Message | You tried to use a managed rule group that's available by subscription, but you aren't subscribed to it yet. |
| `WAFTagOperationException` | `structure` | Message | An error occurred during the tagging operation. Retry your request. |
| `WAFTagOperationInternalErrorException` | `structure` | Message | WAF couldn’t perform your tagging operation because of an internal error. Retry your request. |
| `WAFUnavailableEntityException` | `structure` | Message | WAF couldn’t retrieve a resource that you specified for this operation. If you've just created a resource that you're using in this operation, you might jus ... |
| `WAFUnsupportedAggregateKeyTypeException` | `structure` | Message | The rule that you've named doesn't aggregate solely on the IP address or solely on the forwarded IP address. This call is only available for rate-based rule ... |
| `AssociateWebACLRequest` | `structure` | WebACLArn, ResourceArn | - |
| `AssociateWebACLResponse` | `structure` | **empty (no members)** | - |
| `CheckCapacityRequest` | `structure` | Scope, Rules | - |
| `CheckCapacityResponse` | `structure` | Capacity | - |
| `CreateAPIKeyRequest` | `structure` | Scope, TokenDomains | - |
| `CreateAPIKeyResponse` | `structure` | APIKey | - |
| `CreateIPSetRequest` | `structure` | Name, Scope, Description, IPAddressVersion, Addresses, Tags | - |
| `CreateIPSetResponse` | `structure` | Summary | - |
| `CreateRegexPatternSetRequest` | `structure` | Name, Scope, Description, RegularExpressionList, Tags | - |
| `CreateRegexPatternSetResponse` | `structure` | Summary | - |
| `CreateRuleGroupRequest` | `structure` | Name, Scope, Capacity, Description, Rules, VisibilityConfig, Tags, CustomResponseBodies | - |
| `CreateRuleGroupResponse` | `structure` | Summary | - |
| `CreateWebACLRequest` | `structure` | Name, Scope, DefaultAction, Description, Rules, VisibilityConfig, DataProtectionConfig, Tags, CustomResponseBodies, CaptchaConfig, ChallengeConfig, TokenDomains, ... (+3) | - |
| `CreateWebACLResponse` | `structure` | Summary | - |
| `DeleteAPIKeyRequest` | `structure` | Scope, APIKey | - |
| `DeleteAPIKeyResponse` | `structure` | **empty (no members)** | - |
| `DeleteFirewallManagerRuleGroupsRequest` | `structure` | WebACLArn, WebACLLockToken | - |
| `DeleteFirewallManagerRuleGroupsResponse` | `structure` | NextWebACLLockToken | - |
| `DeleteIPSetRequest` | `structure` | Name, Scope, Id, LockToken | - |
| `DeleteIPSetResponse` | `structure` | **empty (no members)** | - |
| `ActionValue` | `enum` | ALLOW, BLOCK, COUNT, CAPTCHA, CHALLENGE, EXCLUDED_AS_COUNT | - |
| `AssociatedResourceType` | `enum` | CLOUDFRONT, API_GATEWAY, COGNITO_USER_POOL, APP_RUNNER_SERVICE, VERIFIED_ACCESS_INSTANCE | - |
| `BodyParsingFallbackBehavior` | `enum` | MATCH, NO_MATCH, EVALUATE_AS_STRING | - |
| `ComparisonOperator` | `enum` | EQ, NE, LE, LT, GE, GT | - |
| `CountryCode` | `enum` | AF, AX, AL, DZ, AS, AD, AO, AI, AQ, AG, AR, AM, ... (+238) | - |
| `DataProtectionAction` | `enum` | SUBSTITUTION, HASH | - |
| `FailureReason` | `enum` | TOKEN_MISSING, TOKEN_EXPIRED, TOKEN_INVALID, TOKEN_DOMAIN_MISMATCH | - |
| `FallbackBehavior` | `enum` | MATCH, NO_MATCH | - |
| `FieldToProtectType` | `enum` | SINGLE_HEADER, SINGLE_COOKIE, SINGLE_QUERY_ARGUMENT, QUERY_STRING, BODY | - |
| `FilterBehavior` | `enum` | KEEP, DROP | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/rule-evaluator-and-validator-crates.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: full distillation.

### Rule Parser and WCU Calculator

- WAFv2 semantic logic is intentionally split between `winterbaume-wafv2-webacl-rule-parser` for JSON-to-AST parsing and `winterbaume-wafv2-wcu-calculator` for capacity maths.
- Handler-side `CheckCapacity` integration should use the parser and calculator boundary rather than duplicating rule semantics in `winterbaume-wafv2` handlers.
- Parser and calculator tests are infrastructure coverage, not proof of full AWS parity. Service tests still need request-shape, lock-token, scope, and error-mapping assertions around `CheckCapacity` and WebACL operations.

### Known Capacity Gaps

- Calculator follow-up includes exact text-transformation costs, `FieldToMatch` and component surcharges, and custom-key or forwarded-IP surcharges.
- When adding new rule families, add AST coverage first, then WCU calculation tests, then service-level `CheckCapacity` tests that verify the WAFv2 response shape.
- Keep JSON-to-AST parsing strict enough to reject unsupported or ambiguous rule structures through WAFv2-compatible validation errors.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
