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
- Common required input members in this group: `APIKey`, `Id`, `Limit`, `MaxItems`, `Name`, `NumberOfTopTrafficBotsPerPath`, `Platform`, `ReleaseVersion`, `ResourceArn`, `RuleMetricName`, `RuleName`, `Scope`, `TimeWindow`, `WebACLId`, `WebACLName`, `WebAclArn`

### List

- Operations: `ListAPIKeys`, `ListAvailableManagedRuleGroupVersions`, `ListAvailableManagedRuleGroups`, `ListIPSets`, `ListLoggingConfigurations`, `ListManagedRuleSets`, `ListMobileSdkReleases`, `ListRegexPatternSets`, `ListResourcesForWebACL`, `ListRuleGroups`, `ListTagsForResource`, `ListWebACLs`
- Common required input members in this group: `Name`, `Platform`, `ResourceARN`, `Scope`, `VendorName`, `WebACLArn`

### Delete

- Operations: `DeleteAPIKey`, `DeleteFirewallManagerRuleGroups`, `DeleteIPSet`, `DeleteLoggingConfiguration`, `DeletePermissionPolicy`, `DeleteRegexPatternSet`, `DeleteRuleGroup`, `DeleteWebACL`
- Common required input members in this group: `APIKey`, `Id`, `LockToken`, `Name`, `ResourceArn`, `Scope`, `WebACLArn`, `WebACLLockToken`

### Create

- Operations: `CreateAPIKey`, `CreateIPSet`, `CreateRegexPatternSet`, `CreateRuleGroup`, `CreateWebACL`
- Common required input members in this group: `Addresses`, `Capacity`, `DefaultAction`, `IPAddressVersion`, `Name`, `RegularExpressionList`, `Scope`, `TokenDomains`, `VisibilityConfig`

### Update

- Operations: `UpdateIPSet`, `UpdateManagedRuleSetVersionExpiryDate`, `UpdateRegexPatternSet`, `UpdateRuleGroup`, `UpdateWebACL`
- Common required input members in this group: `Addresses`, `DefaultAction`, `ExpiryTimestamp`, `Id`, `LockToken`, `Name`, `RegularExpressionList`, `Scope`, `VersionToExpire`, `VisibilityConfig`

### Describe

- Operations: `DescribeAllManagedProducts`, `DescribeManagedProductsByVendor`, `DescribeManagedRuleGroup`
- Common required input members in this group: `Name`, `Scope`, `VendorName`

### Put

- Operations: `PutLoggingConfiguration`, `PutManagedRuleSetVersions`, `PutPermissionPolicy`
- Common required input members in this group: `Id`, `LockToken`, `LoggingConfiguration`, `Name`, `Policy`, `ResourceArn`, `Scope`

### Associate

- Operations: `AssociateWebACL`
- Common required input members in this group: `ResourceArn`, `WebACLArn`

### Check

- Operations: `CheckCapacity`
- Common required input members in this group: `Rules`, `Scope`

### Disassociate

- Operations: `DisassociateWebACL`
- Common required input members in this group: `ResourceArn`

### Generate

- Operations: `GenerateMobileSdkReleaseUrl`
- Common required input members in this group: `Platform`, `ReleaseVersion`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateWebACL` | - | - | `ResourceArn`, `WebACLArn` | - | `AssociateWebACLResponse` | `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFUnavailableEntityException` | Associates a web ACL with a resource, to protect the resource. Use this for all resource types except for Amazon CloudFront distributions. |
| `CheckCapacity` | - | - | `Rules`, `Scope` | - | `CheckCapacityResponse` | `WAFExpiredManagedRuleGroupVersionException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFSubscriptionNotFoundException`, ... (+1) | Returns the web ACL capacity unit (WCU) requirements for a specified scope and set of rules. You can use this to check the capacity requirements for the rules you want to use in a RuleGroup or WebACL. |
| `CreateAPIKey` | - | - | `Scope`, `TokenDomains` | - | `CreateAPIKeyResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException` | Creates an API key that contains a set of token domains. API keys are required for the integration of the CAPTCHA API in your JavaScript client applications. |
| `CreateIPSet` | - | - | `Addresses`, `IPAddressVersion`, `Name`, `Scope` | - | `CreateIPSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Creates an IPSet, which you use to identify web requests that originate from specific IP addresses or ranges of IP addresses. For example, if you're receiving a lot of requests from a ranges of IP addresses, you can configure WAF to block them using an IPSet... |
| `CreateRegexPatternSet` | - | - | `Name`, `RegularExpressionList`, `Scope` | - | `CreateRegexPatternSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Creates a RegexPatternSet, which you reference in a RegexPatternSetReferenceStatement, to have WAF inspect a web request component for the specified patterns. |
| `CreateRuleGroup` | - | - | `Capacity`, `Name`, `Scope`, `VisibilityConfig` | - | `CreateRuleGroupResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFSubscriptionNotFoundException`, ... (+3) | Creates a RuleGroup per the specifications provided. A rule group defines a collection of rules to inspect and control web requests that you can use in a WebACL. |
| `CreateWebACL` | - | - | `DefaultAction`, `Name`, `Scope`, `VisibilityConfig` | - | `CreateWebACLResponse` | `WAFConfigurationWarningException`, `WAFDuplicateItemException`, `WAFExpiredManagedRuleGroupVersionException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFLimitsExceededException`, ... (+6) | Creates a WebACL per the specifications provided. A web ACL defines a collection of rules to use to inspect and control web requests. |
| `DeleteAPIKey` | - | - | `APIKey`, `Scope` | - | `DeleteAPIKeyResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Deletes the specified API key. After you delete a key, it can take up to 24 hours for WAF to disallow use of the key in all regions. |
| `DeleteFirewallManagerRuleGroups` | - | - | `WebACLArn`, `WebACLLockToken` | - | `DeleteFirewallManagerRuleGroupsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Deletes all rule groups that are managed by Firewall Manager from the specified WebACL. You can only use this if `ManagedByFirewallManager` and `RetrofittedByFirewallManager` are both false in the web ACL. |
| `DeleteIPSet` | - | - | `Id`, `LockToken`, `Name`, `Scope` | - | `DeleteIPSetResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified IPSet. |
| `DeleteLoggingConfiguration` | - | - | `ResourceArn` | - | `DeleteLoggingConfigurationResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Deletes the LoggingConfiguration from the specified web ACL. |
| `DeletePermissionPolicy` | - | - | `ResourceArn` | - | `DeletePermissionPolicyResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Permanently deletes an IAM policy from the specified rule group. You must be the owner of the rule group to perform this operation. |
| `DeleteRegexPatternSet` | - | - | `Id`, `LockToken`, `Name`, `Scope` | - | `DeleteRegexPatternSetResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified RegexPatternSet. |
| `DeleteRuleGroup` | - | - | `Id`, `LockToken`, `Name`, `Scope` | - | `DeleteRuleGroupResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified RuleGroup. |
| `DeleteWebACL` | - | - | `Id`, `LockToken`, `Name`, `Scope` | - | `DeleteWebACLResponse` | `WAFAssociatedItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Deletes the specified WebACL. You can only use this if `ManagedByFirewallManager` is false in the web ACL. |
| `DescribeAllManagedProducts` | - | - | `Scope` | - | `DescribeAllManagedProductsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Provides high-level information for the Amazon Web Services Managed Rules rule groups and Amazon Web Services Marketplace managed rule groups. |
| `DescribeManagedProductsByVendor` | - | - | `Scope`, `VendorName` | - | `DescribeManagedProductsByVendorResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Provides high-level information for the managed rule groups owned by a specific vendor. |
| `DescribeManagedRuleGroup` | - | - | `Name`, `Scope`, `VendorName` | - | `DescribeManagedRuleGroupResponse` | `WAFExpiredManagedRuleGroupVersionException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFNonexistentItemException` | Provides high-level information for a managed rule group, including descriptions of the rules. |
| `DisassociateWebACL` | - | - | `ResourceArn` | - | `DisassociateWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Disassociates the specified resource from its web ACL association, if it has one. Use this for all resource types except for Amazon CloudFront distributions. |
| `GenerateMobileSdkReleaseUrl` | - | - | `Platform`, `ReleaseVersion` | - | `GenerateMobileSdkReleaseUrlResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Generates a presigned download URL for the specified release of the mobile SDK. The mobile SDK is not generally available. |
| `GetDecryptedAPIKey` | - | - | `APIKey`, `Scope` | - | `GetDecryptedAPIKeyResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, `WAFNonexistentItemException` | Returns your API key in decrypted form. Use this to check the token domains that you have defined for the key. |
| `GetIPSet` | - | - | `Id`, `Name`, `Scope` | - | `GetIPSetResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified IPSet. |
| `GetLoggingConfiguration` | - | - | `ResourceArn` | - | `GetLoggingConfigurationResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Returns the LoggingConfiguration for the specified web ACL. |
| `GetManagedRuleSet` | - | - | `Id`, `Name`, `Scope` | - | `GetManagedRuleSetResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified managed rule set. This is intended for use only by vendors of managed rule sets. |
| `GetMobileSdkRelease` | - | - | `Platform`, `ReleaseVersion` | - | `GetMobileSdkReleaseResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves information for the specified mobile SDK release, including release notes and tags. The mobile SDK is not generally available. |
| `GetPermissionPolicy` | - | - | `ResourceArn` | - | `GetPermissionPolicyResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Returns the IAM policy that is attached to the specified rule group. You must be the owner of the rule group to perform this operation. |
| `GetRateBasedStatementManagedKeys` | - | - | `RuleName`, `Scope`, `WebACLId`, `WebACLName` | - | `GetRateBasedStatementManagedKeysResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFUnsupportedAggregateKeyTypeException` | Retrieves the IP addresses that are currently blocked by a rate-based rule instance. This is only available for rate-based rules that aggregate solely on the IP address or on the forwarded IP address. |
| `GetRegexPatternSet` | - | - | `Id`, `Name`, `Scope` | - | `GetRegexPatternSetResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified RegexPatternSet. |
| `GetRuleGroup` | - | - | - | - | `GetRuleGroupResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified RuleGroup. |
| `GetSampledRequests` | - | - | `MaxItems`, `RuleMetricName`, `Scope`, `TimeWindow`, `WebAclArn` | - | `GetSampledRequestsResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Gets detailed information about a specified number of requests--a sample--that WAF randomly selects from among the first 5,000 requests that your Amazon Web Services resource received during a time range that you choose. You can specify a sample size of up to... |
| `GetTopPathStatisticsByTraffic` | - | - | `Limit`, `NumberOfTopTrafficBotsPerPath`, `Scope`, `TimeWindow`, `WebAclArn` | - | `GetTopPathStatisticsByTrafficResponse` | `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves aggregated statistics about the top URI paths accessed by bot traffic for a specified web ACL and time window. You can use this operation to analyze which paths on your web application receive the most bot traffic and identify the specific bots... |
| `GetWebACL` | - | - | - | - | `GetWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves the specified WebACL. |
| `GetWebACLForResource` | - | - | `ResourceArn` | - | `GetWebACLForResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFUnavailableEntityException` | Retrieves the WebACL for the specified resource. This call uses `GetWebACL`, to verify that your account has permission to access the retrieved web ACL. |
| `ListAPIKeys` | - | - | `Scope` | - | `ListAPIKeysResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException` | Retrieves a list of the API keys that you've defined for the specified scope. API keys are required for the integration of the CAPTCHA API in your JavaScript client applications. |
| `ListAvailableManagedRuleGroupVersions` | - | - | `Name`, `Scope`, `VendorName` | - | `ListAvailableManagedRuleGroupVersionsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Returns a list of the available versions for the specified managed rule group. |
| `ListAvailableManagedRuleGroups` | - | - | `Scope` | - | `ListAvailableManagedRuleGroupsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of managed rule groups that are available for you to use. This list includes all Amazon Web Services Managed Rules rule groups and all of the Amazon Web Services Marketplace managed rule groups that you're subscribed to. |
| `ListIPSets` | - | - | `Scope` | - | `ListIPSetsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of IPSetSummary objects for the IP sets that you manage. |
| `ListLoggingConfigurations` | - | - | `Scope` | - | `ListLoggingConfigurationsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of your LoggingConfiguration objects. |
| `ListManagedRuleSets` | - | - | `Scope` | - | `ListManagedRuleSetsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves the managed rule sets that you own. This is intended for use only by vendors of managed rule sets. |
| `ListMobileSdkReleases` | - | - | `Platform` | - | `ListMobileSdkReleasesResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves a list of the available releases for the mobile SDK and the specified device platform. The mobile SDK is not generally available. |
| `ListRegexPatternSets` | - | - | `Scope` | - | `ListRegexPatternSetsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of RegexPatternSetSummary objects for the regex pattern sets that you manage. |
| `ListResourcesForWebACL` | - | - | `WebACLArn` | - | `ListResourcesForWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Retrieves an array of the Amazon Resource Names (ARNs) for the resources that are associated with the specified web ACL. For Amazon CloudFront, don't use this call. |
| `ListRuleGroups` | - | - | `Scope` | - | `ListRuleGroupsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of RuleGroupSummary objects for the rule groups that you manage. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Retrieves the TagInfoForResource for the specified resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. |
| `ListWebACLs` | - | - | `Scope` | - | `ListWebACLsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException` | Retrieves an array of WebACLSummary objects for the web ACLs that you manage. |
| `PutLoggingConfiguration` | - | - | `LoggingConfiguration` | - | `PutLoggingConfigurationResponse` | `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFLogDestinationPermissionIssueException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, ... (+1) | Enables the specified LoggingConfiguration, to start logging from a web ACL, according to the configuration provided. If you configure data protection for the web ACL, the protection applies to the data that WAF sends to the logs. |
| `PutManagedRuleSetVersions` | - | - | `Id`, `LockToken`, `Name`, `Scope` | - | `PutManagedRuleSetVersionsResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Defines the versions of your managed rule set that you are offering to the customers. Customers see your offerings as managed rule groups with versioning. |
| `PutPermissionPolicy` | - | - | `Policy`, `ResourceArn` | - | `PutPermissionPolicyResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFInvalidPermissionPolicyException`, `WAFNonexistentItemException` | Use this to share a rule group with other accounts. This action attaches an IAM policy to the specified resource. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Associates tags with the specified Amazon Web Services resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | Disassociates tags from an Amazon Web Services resource. Tags are key:value pairs that you can associate with Amazon Web Services resources. |
| `UpdateIPSet` | - | - | `Addresses`, `Id`, `LockToken`, `Name`, `Scope` | - | `UpdateIPSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Updates the specified IPSet. This operation completely replaces the mutable specifications that you already have for the IP set with the ones that you provide to this call. |
| `UpdateManagedRuleSetVersionExpiryDate` | - | - | `ExpiryTimestamp`, `Id`, `LockToken`, `Name`, `Scope`, `VersionToExpire` | - | `UpdateManagedRuleSetVersionExpiryDateResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Updates the expiration information for your managed rule set. Use this to initiate the expiration of a managed rule group version. |
| `UpdateRegexPatternSet` | - | - | `Id`, `LockToken`, `Name`, `RegularExpressionList`, `Scope` | - | `UpdateRegexPatternSetResponse` | `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException` | Updates the specified RegexPatternSet. This operation completely replaces the mutable specifications that you already have for the regex pattern set with the ones that you provide to this call. |
| `UpdateRuleGroup` | - | - | `Id`, `LockToken`, `Name`, `Scope`, `VisibilityConfig` | - | `UpdateRuleGroupResponse` | `WAFConfigurationWarningException`, `WAFDuplicateItemException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFOptimisticLockException`, ... (+2) | Updates the specified RuleGroup. This operation completely replaces the mutable specifications that you already have for the rule group with the ones that you provide to this call. |
| `UpdateWebACL` | - | - | `DefaultAction`, `Id`, `LockToken`, `Name`, `Scope`, `VisibilityConfig` | - | `UpdateWebACLResponse` | `WAFConfigurationWarningException`, `WAFDuplicateItemException`, `WAFExpiredManagedRuleGroupVersionException`, `WAFFeatureNotIncludedInPricingPlanException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFInvalidResourceException`, ... (+5) | Updates the specified WebACL. While updating a web ACL, WAF provides continuous coverage to the resources that you have associated with the web ACL. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `WAFInternalErrorException` | `structure` | `Message` | Your request is valid, but WAF couldn’t perform the operation because of a system problem. |
| `WAFInvalidParameterException` | `structure` | `Field`, `Parameter`, `Reason`, `message` | The operation failed because WAF didn't recognize a parameter in the request. |
| `WAFInvalidOperationException` | `structure` | `Message` | The operation isn't valid. |
| `WAFNonexistentItemException` | `structure` | `Message` | WAF couldn’t perform the operation because your resource doesn't exist. |
| `WAFOptimisticLockException` | `structure` | `Message` | WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. |
| `WAFLimitsExceededException` | `structure` | `Message`, `SourceType` | WAF couldn’t perform the operation because you exceeded your resource limit. |
| `WAFTagOperationException` | `structure` | `Message` | An error occurred during the tagging operation. |
| `WAFTagOperationInternalErrorException` | `structure` | `Message` | WAF couldn’t perform your tagging operation because of an internal error. |
| `WAFDuplicateItemException` | `structure` | `Message` | WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one. |
| `WAFUnavailableEntityException` | `structure` | `Message` | WAF couldn’t retrieve a resource that you specified for this operation. |
| `WAFInvalidResourceException` | `structure` | `Message` | WAF couldn’t perform the operation because the resource that you requested isn’t valid. |
| `WAFSubscriptionNotFoundException` | `structure` | `Message` | You tried to use a managed rule group that's available by subscription, but you aren't subscribed to it yet. |
| `WAFFeatureNotIncludedInPricingPlanException` | `structure` | `DisallowedFeatures`, `Message` | The operation failed because the specified WAF feature isn't supported by the CloudFront pricing plan associated with the web ACL. |
| `WAFExpiredManagedRuleGroupVersionException` | `structure` | `Message` | The operation failed because the specified version for the managed rule group has expired. |
| `WAFAssociatedItemException` | `structure` | `Message` | WAF couldn’t perform the operation because your resource is being used by another resource or it’s associated with another resource. |
| `WAFConfigurationWarningException` | `structure` | `Message` | The operation failed because you are inspecting the web request body, headers, or cookies without specifying how to handle oversize components. |
| `AssociateWebACLRequest` | `structure` | `ResourceArn`, `WebACLArn` | - |
| `AssociateWebACLResponse` | `structure` | - | - |
| `CheckCapacityRequest` | `structure` | `Rules`, `Scope` | - |
| `CheckCapacityResponse` | `structure` | `Capacity` | - |
| `CreateAPIKeyRequest` | `structure` | `Scope`, `TokenDomains` | - |
| `CreateAPIKeyResponse` | `structure` | `APIKey` | - |
| `CreateIPSetRequest` | `structure` | `Addresses`, `Description`, `IPAddressVersion`, `Name`, `Scope`, `Tags` | - |
| `CreateIPSetResponse` | `structure` | `Summary` | - |

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
