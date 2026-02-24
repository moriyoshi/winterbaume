# AWS WAF Regional

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is AWS WAF Classic Regional documentation. For more information, see AWS WAF Classic in the developer guide. For the latest version of AWS WAF , use the AWS WAFV2 API and see the AWS WAF Developer Guide. With the latest version, AWS WAF has a single set of endpoints for regional and global use. This is the AWS WAF Regional Classic API Reference for using AWS WAF Classic with the AWS resources, Elastic Load Balancing (ELB) Application Load Balancers and API Gateway APIs.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS WAF Regional where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS WAF Regional by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS WAF Regional workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Create`, `Update` operation families, including `GetByteMatchSet`, `GetChangeToken`, `GetChangeTokenStatus`, `GetGeoMatchSet`, `ListActivatedRulesInRuleGroup`, `ListByteMatchSets`.

## Service Identity and Protocol

- AWS model slug: `waf-regional`
- AWS SDK for Rust slug: `wafregional`
- Model version: `2016-11-28`
- Model file: `vendor/api-models-aws/models/waf-regional/service/2016-11-28/waf-regional-2016-11-28.json`
- SDK ID: `WAF Regional`
- Endpoint prefix: `waf-regional`
- ARN namespace: `waf-regional`
- CloudFormation name: `WAFRegional`
- CloudTrail event source: `wafregional.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (19), `List` (17), `Delete` (14), `Create` (13), `Update` (12), `Put` (2), `Associate` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateWebACL`, `CreateByteMatchSet`, `CreateGeoMatchSet`, `CreateIPSet`, `CreateRateBasedRule`, `CreateRegexMatchSet`, `CreateRegexPatternSet`, `CreateRule`, `CreateRuleGroup`, `CreateSizeConstraintSet`, `CreateSqlInjectionMatchSet`, `CreateWebACL`, `CreateWebACLMigrationStack`, `CreateXssMatchSet`, `DeleteByteMatchSet`, `DeleteGeoMatchSet`, `DeleteIPSet`, `DeleteLoggingConfiguration`, `DeletePermissionPolicy`, `DeleteRateBasedRule`, ... (+25).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetByteMatchSet`, `GetChangeToken`, `GetChangeTokenStatus`, `GetGeoMatchSet`, `GetIPSet`, `GetLoggingConfiguration`, `GetPermissionPolicy`, `GetRateBasedRule`, `GetRateBasedRuleManagedKeys`, `GetRegexMatchSet`, `GetRegexPatternSet`, `GetRule`, `GetRuleGroup`, `GetSampledRequests`, `GetSizeConstraintSet`, `GetSqlInjectionMatchSet`, `GetWebACL`, `GetWebACLForResource`, `GetXssMatchSet`, `ListActivatedRulesInRuleGroup`, ... (+16).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 81 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.

## Operation Groups

### Get

- Operations: `GetByteMatchSet`, `GetChangeToken`, `GetChangeTokenStatus`, `GetGeoMatchSet`, `GetIPSet`, `GetLoggingConfiguration`, `GetPermissionPolicy`, `GetRateBasedRule`, `GetRateBasedRuleManagedKeys`, `GetRegexMatchSet`, `GetRegexPatternSet`, `GetRule`, `GetRuleGroup`, `GetSampledRequests`, `GetSizeConstraintSet`, `GetSqlInjectionMatchSet`, `GetWebACL`, `GetWebACLForResource`, `GetXssMatchSet`
- Common required input members in this group: `ByteMatchSetId`, `ChangeToken`, `GeoMatchSetId`, `IPSetId`, `MaxItems`, `RegexMatchSetId`, `RegexPatternSetId`, `ResourceArn`, `RuleGroupId`, `RuleId`, `SizeConstraintSetId`, `SqlInjectionMatchSetId`, `TimeWindow`, `WebACLId`, `WebAclId`, `XssMatchSetId`

### List

- Operations: `ListActivatedRulesInRuleGroup`, `ListByteMatchSets`, `ListGeoMatchSets`, `ListIPSets`, `ListLoggingConfigurations`, `ListRateBasedRules`, `ListRegexMatchSets`, `ListRegexPatternSets`, `ListResourcesForWebACL`, `ListRuleGroups`, `ListRules`, `ListSizeConstraintSets`, `ListSqlInjectionMatchSets`, `ListSubscribedRuleGroups`, `ListTagsForResource`, `ListWebACLs`, `ListXssMatchSets`
- Common required input members in this group: `ResourceARN`, `WebACLId`

### Delete

- Operations: `DeleteByteMatchSet`, `DeleteGeoMatchSet`, `DeleteIPSet`, `DeleteLoggingConfiguration`, `DeletePermissionPolicy`, `DeleteRateBasedRule`, `DeleteRegexMatchSet`, `DeleteRegexPatternSet`, `DeleteRule`, `DeleteRuleGroup`, `DeleteSizeConstraintSet`, `DeleteSqlInjectionMatchSet`, `DeleteWebACL`, `DeleteXssMatchSet`
- Common required input members in this group: `ByteMatchSetId`, `ChangeToken`, `GeoMatchSetId`, `IPSetId`, `RegexMatchSetId`, `RegexPatternSetId`, `ResourceArn`, `RuleGroupId`, `RuleId`, `SizeConstraintSetId`, `SqlInjectionMatchSetId`, `WebACLId`, `XssMatchSetId`

### Create

- Operations: `CreateByteMatchSet`, `CreateGeoMatchSet`, `CreateIPSet`, `CreateRateBasedRule`, `CreateRegexMatchSet`, `CreateRegexPatternSet`, `CreateRule`, `CreateRuleGroup`, `CreateSizeConstraintSet`, `CreateSqlInjectionMatchSet`, `CreateWebACL`, `CreateWebACLMigrationStack`, `CreateXssMatchSet`
- Common required input members in this group: `ChangeToken`, `DefaultAction`, `IgnoreUnsupportedType`, `MetricName`, `Name`, `RateKey`, `RateLimit`, `S3BucketName`, `WebACLId`

### Update

- Operations: `UpdateByteMatchSet`, `UpdateGeoMatchSet`, `UpdateIPSet`, `UpdateRateBasedRule`, `UpdateRegexMatchSet`, `UpdateRegexPatternSet`, `UpdateRule`, `UpdateRuleGroup`, `UpdateSizeConstraintSet`, `UpdateSqlInjectionMatchSet`, `UpdateWebACL`, `UpdateXssMatchSet`
- Common required input members in this group: `ByteMatchSetId`, `ChangeToken`, `GeoMatchSetId`, `IPSetId`, `RateLimit`, `RegexMatchSetId`, `RegexPatternSetId`, `RuleGroupId`, `RuleId`, `SizeConstraintSetId`, `SqlInjectionMatchSetId`, `Updates`, `WebACLId`, `XssMatchSetId`

### Put

- Operations: `PutLoggingConfiguration`, `PutPermissionPolicy`
- Common required input members in this group: `LoggingConfiguration`, `Policy`, `ResourceArn`

### Associate

- Operations: `AssociateWebACL`
- Common required input members in this group: `ResourceArn`, `WebACLId`

### Disassociate

- Operations: `DisassociateWebACL`
- Common required input members in this group: `ResourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateWebACL` | - | - | `ResourceArn`, `WebACLId` | - | `AssociateWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFUnavailableEntityException` | This is AWS WAF Classic Regional documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateByteMatchSet` | - | - | `ChangeToken`, `Name` | - | `CreateByteMatchSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateGeoMatchSet` | - | - | `ChangeToken`, `Name` | - | `CreateGeoMatchSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateIPSet` | - | - | `ChangeToken`, `Name` | - | `CreateIPSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateRateBasedRule` | - | - | `ChangeToken`, `MetricName`, `Name`, `RateKey`, `RateLimit` | - | `CreateRateBasedRuleResponse` | `WAFBadRequestException`, `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateRegexMatchSet` | - | - | `ChangeToken`, `Name` | - | `CreateRegexMatchSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateRegexPatternSet` | - | - | `ChangeToken`, `Name` | - | `CreateRegexPatternSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateRule` | - | - | `ChangeToken`, `MetricName`, `Name` | - | `CreateRuleResponse` | `WAFBadRequestException`, `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateRuleGroup` | - | - | `ChangeToken`, `MetricName`, `Name` | - | `CreateRuleGroupResponse` | `WAFBadRequestException`, `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFLimitsExceededException`, `WAFStaleDataException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateSizeConstraintSet` | - | - | `ChangeToken`, `Name` | - | `CreateSizeConstraintSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateSqlInjectionMatchSet` | - | - | `ChangeToken`, `Name` | - | `CreateSqlInjectionMatchSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateWebACL` | - | - | `ChangeToken`, `DefaultAction`, `MetricName`, `Name` | - | `CreateWebACLResponse` | `WAFBadRequestException`, `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException`, `WAFTagOperationException`, ... (+1) | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `CreateWebACLMigrationStack` | - | - | `IgnoreUnsupportedType`, `S3BucketName`, `WebACLId` | - | `CreateWebACLMigrationStackResponse` | `WAFEntityMigrationException`, `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | Creates an AWS CloudFormation WAFV2 template for the specified web ACL in the specified Amazon S3 bucket. Then, in CloudFormation, you create a stack from the template, to create the web ACL and its resources in AWS WAFV2. |
| `CreateXssMatchSet` | - | - | `ChangeToken`, `Name` | - | `CreateXssMatchSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteByteMatchSet` | - | - | `ByteMatchSetId`, `ChangeToken` | - | `DeleteByteMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteGeoMatchSet` | - | - | `ChangeToken`, `GeoMatchSetId` | - | `DeleteGeoMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteIPSet` | - | - | `ChangeToken`, `IPSetId` | - | `DeleteIPSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteLoggingConfiguration` | - | - | `ResourceArn` | - | `DeleteLoggingConfigurationResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeletePermissionPolicy` | - | - | `ResourceArn` | - | `DeletePermissionPolicyResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteRateBasedRule` | - | - | `ChangeToken`, `RuleId` | - | `DeleteRateBasedRuleResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteRegexMatchSet` | - | - | `ChangeToken`, `RegexMatchSetId` | - | `DeleteRegexMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteRegexPatternSet` | - | - | `ChangeToken`, `RegexPatternSetId` | - | `DeleteRegexPatternSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteRule` | - | - | `ChangeToken`, `RuleId` | - | `DeleteRuleResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteRuleGroup` | - | - | `ChangeToken`, `RuleGroupId` | - | `DeleteRuleGroupResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteSizeConstraintSet` | - | - | `ChangeToken`, `SizeConstraintSetId` | - | `DeleteSizeConstraintSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteSqlInjectionMatchSet` | - | - | `ChangeToken`, `SqlInjectionMatchSetId` | - | `DeleteSqlInjectionMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteWebACL` | - | - | `ChangeToken`, `WebACLId` | - | `DeleteWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DeleteXssMatchSet` | - | - | `ChangeToken`, `XssMatchSetId` | - | `DeleteXssMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonEmptyEntityException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `DisassociateWebACL` | - | - | `ResourceArn` | - | `DisassociateWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | This is AWS WAF Classic Regional documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetByteMatchSet` | - | - | `ByteMatchSetId` | - | `GetByteMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetChangeToken` | - | - | - | - | `GetChangeTokenResponse` | `WAFInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetChangeTokenStatus` | - | - | `ChangeToken` | - | `GetChangeTokenStatusResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetGeoMatchSet` | - | - | `GeoMatchSetId` | - | `GetGeoMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetIPSet` | - | - | `IPSetId` | - | `GetIPSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetLoggingConfiguration` | - | - | `ResourceArn` | - | `GetLoggingConfigurationResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetPermissionPolicy` | - | - | `ResourceArn` | - | `GetPermissionPolicyResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetRateBasedRule` | - | - | `RuleId` | - | `GetRateBasedRuleResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetRateBasedRuleManagedKeys` | - | - | `RuleId` | - | `GetRateBasedRuleManagedKeysResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetRegexMatchSet` | - | - | `RegexMatchSetId` | - | `GetRegexMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetRegexPatternSet` | - | - | `RegexPatternSetId` | - | `GetRegexPatternSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetRule` | - | - | `RuleId` | - | `GetRuleResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetRuleGroup` | - | - | `RuleGroupId` | - | `GetRuleGroupResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetSampledRequests` | - | - | `MaxItems`, `RuleId`, `TimeWindow`, `WebAclId` | - | `GetSampledRequestsResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetSizeConstraintSet` | - | - | `SizeConstraintSetId` | - | `GetSizeConstraintSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetSqlInjectionMatchSet` | - | - | `SqlInjectionMatchSetId` | - | `GetSqlInjectionMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetWebACL` | - | - | `WebACLId` | - | `GetWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetWebACLForResource` | - | - | `ResourceArn` | - | `GetWebACLForResourceResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFUnavailableEntityException` | This is AWS WAF Classic Regional documentation. For more information, see AWS WAF Classic in the developer guide. |
| `GetXssMatchSet` | - | - | `XssMatchSetId` | - | `GetXssMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListActivatedRulesInRuleGroup` | - | - | - | - | `ListActivatedRulesInRuleGroupResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListByteMatchSets` | - | - | - | - | `ListByteMatchSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListGeoMatchSets` | - | - | - | - | `ListGeoMatchSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListIPSets` | - | - | - | - | `ListIPSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListLoggingConfigurations` | - | - | - | - | `ListLoggingConfigurationsResponse` | `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListRateBasedRules` | - | - | - | - | `ListRateBasedRulesResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListRegexMatchSets` | - | - | - | - | `ListRegexMatchSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListRegexPatternSets` | - | - | - | - | `ListRegexPatternSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListResourcesForWebACL` | - | - | `WebACLId` | - | `ListResourcesForWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidParameterException`, `WAFNonexistentItemException` | This is AWS WAF Classic Regional documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListRuleGroups` | - | - | - | - | `ListRuleGroupsResponse` | `WAFInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListRules` | - | - | - | - | `ListRulesResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListSizeConstraintSets` | - | - | - | - | `ListSizeConstraintSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListSqlInjectionMatchSets` | - | - | - | - | `ListSqlInjectionMatchSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListSubscribedRuleGroups` | - | - | - | - | `ListSubscribedRuleGroupsResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `WAFBadRequestException`, `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListWebACLs` | - | - | - | - | `ListWebACLsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `ListXssMatchSets` | - | - | - | - | `ListXssMatchSetsResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `PutLoggingConfiguration` | - | - | `LoggingConfiguration` | - | `PutLoggingConfigurationResponse` | `WAFInternalErrorException`, `WAFNonexistentItemException`, `WAFServiceLinkedRoleErrorException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `PutPermissionPolicy` | - | - | `Policy`, `ResourceArn` | - | `PutPermissionPolicyResponse` | `WAFInternalErrorException`, `WAFInvalidPermissionPolicyException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `WAFBadRequestException`, `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `WAFBadRequestException`, `WAFInternalErrorException`, `WAFInvalidParameterException`, `WAFNonexistentItemException`, `WAFTagOperationException`, `WAFTagOperationInternalErrorException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateByteMatchSet` | - | - | `ByteMatchSetId`, `ChangeToken`, `Updates` | - | `UpdateByteMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateGeoMatchSet` | - | - | `ChangeToken`, `GeoMatchSetId`, `Updates` | - | `UpdateGeoMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, ... (+1) | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateIPSet` | - | - | `ChangeToken`, `IPSetId`, `Updates` | - | `UpdateIPSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, ... (+1) | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateRateBasedRule` | - | - | `ChangeToken`, `RateLimit`, `RuleId`, `Updates` | - | `UpdateRateBasedRuleResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, ... (+1) | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateRegexMatchSet` | - | - | `ChangeToken`, `RegexMatchSetId`, `Updates` | - | `UpdateRegexMatchSetResponse` | `WAFDisallowedNameException`, `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateRegexPatternSet` | - | - | `ChangeToken`, `RegexPatternSetId`, `Updates` | - | `UpdateRegexPatternSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidRegexPatternException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateRule` | - | - | `ChangeToken`, `RuleId`, `Updates` | - | `UpdateRuleResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, ... (+1) | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateRuleGroup` | - | - | `ChangeToken`, `RuleGroupId`, `Updates` | - | `UpdateRuleGroupResponse` | `WAFInternalErrorException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateSizeConstraintSet` | - | - | `ChangeToken`, `SizeConstraintSetId`, `Updates` | - | `UpdateSizeConstraintSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, ... (+1) | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateSqlInjectionMatchSet` | - | - | `ChangeToken`, `SqlInjectionMatchSetId`, `Updates` | - | `UpdateSqlInjectionMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateWebACL` | - | - | `ChangeToken`, `WebACLId` | - | `UpdateWebACLResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFReferencedItemException`, ... (+2) | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |
| `UpdateXssMatchSet` | - | - | `ChangeToken`, `Updates`, `XssMatchSetId` | - | `UpdateXssMatchSetResponse` | `WAFInternalErrorException`, `WAFInvalidAccountException`, `WAFInvalidOperationException`, `WAFInvalidParameterException`, `WAFLimitsExceededException`, `WAFNonexistentContainerException`, `WAFNonexistentItemException`, `WAFStaleDataException` | This is AWS WAF Classic documentation. For more information, see AWS WAF Classic in the developer guide. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `WAFInternalErrorException` | `structure` | `message` | The operation failed because of a system problem, even though the request was valid. |
| `WAFInvalidAccountException` | `structure` | - | The operation failed because you tried to create, update, or delete an object by using an invalid account identifier. |
| `WAFNonexistentItemException` | `structure` | `message` | The operation failed because the referenced object doesn't exist. |
| `WAFStaleDataException` | `structure` | `message` | The operation failed because you tried to create, update, or delete an object by using a change token that has already been used. |
| `WAFInvalidParameterException` | `structure` | `field`, `parameter`, `reason` | The operation failed because AWS WAF didn't recognize a parameter in the request. |
| `WAFLimitsExceededException` | `structure` | `message` | The operation exceeds a resource limit, for example, the maximum number of `WebACL` objects that you can create for an AWS account. |
| `WAFReferencedItemException` | `structure` | `message` | The operation failed because you tried to delete an object that is still in use. |
| `WAFInvalidOperationException` | `structure` | `message` | The operation failed because there was nothing to do. |
| `WAFDisallowedNameException` | `structure` | `message` | The name specified is invalid. |
| `WAFNonEmptyEntityException` | `structure` | `message` | The operation failed because you tried to delete an object that isn't empty. |
| `WAFNonexistentContainerException` | `structure` | `message` | The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. |
| `WAFTagOperationException` | `structure` | `message` | - |
| `WAFTagOperationInternalErrorException` | `structure` | `message` | - |
| `WAFBadRequestException` | `structure` | `message` | - |
| `WAFUnavailableEntityException` | `structure` | `message` | The operation failed because the entity referenced is temporarily unavailable. |
| `AssociateWebACLRequest` | `structure` | `ResourceArn`, `WebACLId` | - |
| `AssociateWebACLResponse` | `structure` | - | - |
| `CreateByteMatchSetRequest` | `structure` | `ChangeToken`, `Name` | - |
| `CreateByteMatchSetResponse` | `structure` | `ByteMatchSet`, `ChangeToken` | - |
| `CreateGeoMatchSetRequest` | `structure` | `ChangeToken`, `Name` | - |
| `CreateGeoMatchSetResponse` | `structure` | `ChangeToken`, `GeoMatchSet` | - |
| `CreateIPSetRequest` | `structure` | `ChangeToken`, `Name` | - |
| `CreateIPSetResponse` | `structure` | `ChangeToken`, `IPSet` | - |
| `CreateRateBasedRuleRequest` | `structure` | `ChangeToken`, `MetricName`, `Name`, `RateKey`, `RateLimit`, `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
