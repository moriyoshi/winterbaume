# AWS License Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

License Manager makes it easier to manage licenses from software vendors across multiple Amazon Web Services accounts and on-premises servers.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS License Manager workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Get`, `Delete`, `Update` operation families, including `ListAssetsForLicenseAssetGroup`, `ListAssociationsForLicenseConfiguration`, `ListDistributedGrants`, `ListFailuresForLicenseConfigurationOperations`, `CreateGrant`, `CreateGrantVersion`.

## Service Identity and Protocol

- AWS model slug: `license-manager`
- AWS SDK for Rust slug: `licensemanager`
- Model version: `2018-08-01`
- Model file: `vendor/api-models-aws/models/license-manager/service/2018-08-01/license-manager-2018-08-01.json`
- SDK ID: `License Manager`
- Endpoint prefix: `license-manager`
- ARN namespace: `license-manager`
- CloudFormation name: `LicenseManager`
- CloudTrail event source: `license-manager.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (21), `Create` (10), `Get` (10), `Delete` (7), `Update` (6), `Checkout` (2), `Accept` (1), `Check` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptGrant`, `CreateGrant`, `CreateGrantVersion`, `CreateLicense`, `CreateLicenseAssetGroup`, `CreateLicenseAssetRuleset`, `CreateLicenseConfiguration`, `CreateLicenseConversionTaskForResource`, `CreateLicenseManagerReportGenerator`, `CreateLicenseVersion`, `CreateToken`, `DeleteGrant`, `DeleteLicense`, `DeleteLicenseAssetGroup`, `DeleteLicenseAssetRuleset`, `DeleteLicenseConfiguration`, `DeleteLicenseManagerReportGenerator`, `DeleteToken`, `RejectGrant`, `TagResource`, ... (+7).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CheckInLicense`, `GetAccessToken`, `GetGrant`, `GetLicense`, `GetLicenseAssetGroup`, `GetLicenseAssetRuleset`, `GetLicenseConfiguration`, `GetLicenseConversionTask`, `GetLicenseManagerReportGenerator`, `GetLicenseUsage`, `GetServiceSettings`, `ListAssetsForLicenseAssetGroup`, `ListAssociationsForLicenseConfiguration`, `ListDistributedGrants`, `ListFailuresForLicenseConfigurationOperations`, `ListLicenseAssetGroups`, `ListLicenseAssetRulesets`, `ListLicenseConfigurations`, `ListLicenseConfigurationsForOrganization`, `ListLicenseConversionTasks`, ... (+12).
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateLicenseConversionTaskForResource`, `CreateLicenseManagerReportGenerator`, `DeleteLicenseManagerReportGenerator`, `GetLicenseConversionTask`, `GetLicenseManagerReportGenerator`, `ListLicenseConversionTasks`, `ListLicenseManagerReportGenerators`, `UpdateLicenseManagerReportGenerator`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 62 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/license-manager/latest/userguide/settings-managed-licenses.html
- https://docs.aws.amazon.com/license-manager/latest/userguide/granted-licenses.html
- https://docs.aws.amazon.com/license-manager/latest/userguide/manage-granted-licenses.html

Research outcomes:
- License Manager centralises software licence tracking and governance across accounts and organisations.
- Managed licence settings control discovery, cross-account resource discovery, organisation integration, and SNS alerts.
- Granted licences represent entitlements distributed to AWS accounts and can be accepted, activated, and replicated across Regions.
- Seller-issued licences support ISV distribution models such as floating, perpetual, subscription, and usage-based licences.
- Licence configurations and rules can be associated with discovered resources to track consumption.
- Cross-account scenarios depend on AWS Organizations, resource discovery settings, and service-linked roles.

Parity implications:
- Model licence configurations, associations, grants, granted licences, issuer metadata, consumption records, organisation settings, and SNS alert configuration separately.
- Licence consumption should be derived from associated resources and rule constraints.
- Granted licence activation and cross-Region replication should be stateful operations.

## Operation Groups

### List

- Operations: `ListAssetsForLicenseAssetGroup`, `ListAssociationsForLicenseConfiguration`, `ListDistributedGrants`, `ListFailuresForLicenseConfigurationOperations`, `ListLicenseAssetGroups`, `ListLicenseAssetRulesets`, `ListLicenseConfigurations`, `ListLicenseConfigurationsForOrganization`, `ListLicenseConversionTasks`, `ListLicenseManagerReportGenerators`, `ListLicenseSpecificationsForResource`, `ListLicenseVersions`, `ListLicenses`, `ListReceivedGrants`, `ListReceivedGrantsForOrganization`, `ListReceivedLicenses`, `ListReceivedLicensesForOrganization`, `ListResourceInventory`, `ListTagsForResource`, `ListTokens`, `ListUsageForLicenseConfiguration`
- Common required input members in this group: `AssetType`, `LicenseArn`, `LicenseAssetGroupArn`, `LicenseConfigurationArn`, `ResourceArn`

### Create

- Operations: `CreateGrant`, `CreateGrantVersion`, `CreateLicense`, `CreateLicenseAssetGroup`, `CreateLicenseAssetRuleset`, `CreateLicenseConfiguration`, `CreateLicenseConversionTaskForResource`, `CreateLicenseManagerReportGenerator`, `CreateLicenseVersion`, `CreateToken`
- Common required input members in this group: `AllowedOperations`, `AssociatedLicenseAssetRulesetARNs`, `Beneficiary`, `ClientToken`, `ConsumptionConfiguration`, `DestinationLicenseContext`, `Entitlements`, `GrantArn`, `GrantName`, `HomeRegion`, `Issuer`, `LicenseArn`, `LicenseAssetGroupConfigurations`, `LicenseCountingType`, `LicenseName`, `Name`, `Principals`, `ProductName`, `ProductSKU`, `ReportContext`, `ReportFrequency`, `ReportGeneratorName`, `ResourceArn`, `Rules`, ... (+4)

### Get

- Operations: `GetAccessToken`, `GetGrant`, `GetLicense`, `GetLicenseAssetGroup`, `GetLicenseAssetRuleset`, `GetLicenseConfiguration`, `GetLicenseConversionTask`, `GetLicenseManagerReportGenerator`, `GetLicenseUsage`, `GetServiceSettings`
- Common required input members in this group: `GrantArn`, `LicenseArn`, `LicenseAssetGroupArn`, `LicenseAssetRulesetArn`, `LicenseConfigurationArn`, `LicenseConversionTaskId`, `LicenseManagerReportGeneratorArn`, `Token`

### Delete

- Operations: `DeleteGrant`, `DeleteLicense`, `DeleteLicenseAssetGroup`, `DeleteLicenseAssetRuleset`, `DeleteLicenseConfiguration`, `DeleteLicenseManagerReportGenerator`, `DeleteToken`
- Common required input members in this group: `GrantArn`, `LicenseArn`, `LicenseAssetGroupArn`, `LicenseAssetRulesetArn`, `LicenseConfigurationArn`, `LicenseManagerReportGeneratorArn`, `SourceVersion`, `TokenId`, `Version`

### Update

- Operations: `UpdateLicenseAssetGroup`, `UpdateLicenseAssetRuleset`, `UpdateLicenseConfiguration`, `UpdateLicenseManagerReportGenerator`, `UpdateLicenseSpecificationsForResource`, `UpdateServiceSettings`
- Common required input members in this group: `AssociatedLicenseAssetRulesetARNs`, `ClientToken`, `LicenseAssetGroupArn`, `LicenseAssetRulesetArn`, `LicenseConfigurationArn`, `LicenseManagerReportGeneratorArn`, `ReportContext`, `ReportFrequency`, `ReportGeneratorName`, `ResourceArn`, `Rules`, `Type`

### Checkout

- Operations: `CheckoutBorrowLicense`, `CheckoutLicense`
- Common required input members in this group: `CheckoutType`, `ClientToken`, `DigitalSignatureMethod`, `Entitlements`, `KeyFingerprint`, `LicenseArn`, `ProductSKU`

### Accept

- Operations: `AcceptGrant`
- Common required input members in this group: `GrantArn`

### Check

- Operations: `CheckInLicense`
- Common required input members in this group: `LicenseConsumptionToken`

### Extend

- Operations: `ExtendLicenseConsumption`
- Common required input members in this group: `LicenseConsumptionToken`

### Reject

- Operations: `RejectGrant`
- Common required input members in this group: `GrantArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptGrant` | - | - | `GrantArn` | - | `AcceptGrantResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Accepts the specified grant. |
| `CheckInLicense` | - | - | `LicenseConsumptionToken` | - | `CheckInLicenseResponse` | `AccessDeniedException`, `AuthorizationException`, `ConflictException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Checks in the specified license. Check in a license when it is no longer in use. |
| `CheckoutBorrowLicense` | - | - | `ClientToken`, `DigitalSignatureMethod`, `Entitlements`, `LicenseArn` | - | `CheckoutBorrowLicenseResponse` | `AccessDeniedException`, `AuthorizationException`, `EntitlementNotAllowedException`, `InvalidParameterValueException`, `NoEntitlementsAllowedException`, `RateLimitExceededException`, `RedirectException`, `ResourceNotFoundException`, ... (+3) | Checks out the specified license for offline use. |
| `CheckoutLicense` | - | - | `CheckoutType`, `ClientToken`, `Entitlements`, `KeyFingerprint`, `ProductSKU` | - | `CheckoutLicenseResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `NoEntitlementsAllowedException`, `RateLimitExceededException`, `RedirectException`, `ResourceNotFoundException`, `ServerInternalException`, ... (+2) | Checks out the specified license. If the account that created the license is the same that is performing the check out, you must specify the account as the beneficiary. |
| `CreateGrant` | - | - | `AllowedOperations`, `ClientToken`, `GrantName`, `HomeRegion`, `LicenseArn`, `Principals` | - | `CreateGrantResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Creates a grant for the specified license. A grant shares the use of license entitlements with a specific Amazon Web Services account, an organization, or an organizational unit (OU). |
| `CreateGrantVersion` | - | - | `ClientToken`, `GrantArn` | - | `CreateGrantVersionResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Creates a new version of the specified grant. For more information, see Granted licenses in License Manager in the License Manager User Guide . |
| `CreateLicense` | - | - | `Beneficiary`, `ClientToken`, `ConsumptionConfiguration`, `Entitlements`, `HomeRegion`, `Issuer`, `LicenseName`, `ProductName`, `ProductSKU`, `Validity` | - | `CreateLicenseResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `RedirectException`, `ServerInternalException`, `ValidationException` | Creates a license. |
| `CreateLicenseAssetGroup` | - | - | `AssociatedLicenseAssetRulesetARNs`, `ClientToken`, `LicenseAssetGroupConfigurations`, `Name` | - | `CreateLicenseAssetGroupResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Creates a license asset group. |
| `CreateLicenseAssetRuleset` | - | - | `ClientToken`, `Name`, `Rules` | - | `CreateLicenseAssetRulesetResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Creates a license asset ruleset. |
| `CreateLicenseConfiguration` | - | - | `LicenseCountingType`, `Name` | - | `CreateLicenseConfigurationResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException` | Creates a license configuration. A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. |
| `CreateLicenseConversionTaskForResource` | - | - | `DestinationLicenseContext`, `ResourceArn`, `SourceLicenseContext` | - | `CreateLicenseConversionTaskForResourceResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Creates a new license conversion task. |
| `CreateLicenseManagerReportGenerator` | - | - | `ClientToken`, `ReportContext`, `ReportFrequency`, `ReportGeneratorName`, `Type` | - | `CreateLicenseManagerReportGeneratorResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Creates a report generator. |
| `CreateLicenseVersion` | - | - | `ClientToken`, `ConsumptionConfiguration`, `Entitlements`, `HomeRegion`, `Issuer`, `LicenseArn`, `LicenseName`, `ProductName`, `Status`, `Validity` | - | `CreateLicenseVersionResponse` | `AccessDeniedException`, `AuthorizationException`, `ConflictException`, `RateLimitExceededException`, `RedirectException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Creates a new version of the specified license. |
| `CreateToken` | - | - | `ClientToken`, `LicenseArn` | - | `CreateTokenResponse` | `AccessDeniedException`, `AuthorizationException`, `RateLimitExceededException`, `RedirectException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Creates a long-lived token. A refresh token is a JWT token used to get an access token. |
| `DeleteGrant` | - | - | `GrantArn`, `Version` | - | `DeleteGrantResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Deletes the specified grant. |
| `DeleteLicense` | - | - | `LicenseArn`, `SourceVersion` | - | `DeleteLicenseResponse` | `AccessDeniedException`, `AuthorizationException`, `ConflictException`, `InvalidParameterValueException`, `RateLimitExceededException`, `RedirectException`, `ServerInternalException`, `ValidationException` | Deletes the specified license. |
| `DeleteLicenseAssetGroup` | - | - | `LicenseAssetGroupArn` | - | `DeleteLicenseAssetGroupResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Deletes a license asset group. |
| `DeleteLicenseAssetRuleset` | - | - | `LicenseAssetRulesetArn` | - | `DeleteLicenseAssetRulesetResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Deletes a license asset ruleset. |
| `DeleteLicenseConfiguration` | - | - | `LicenseConfigurationArn` | - | `DeleteLicenseConfigurationResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Deletes the specified license configuration. You cannot delete a license configuration that is in use. |
| `DeleteLicenseManagerReportGenerator` | - | - | `LicenseManagerReportGeneratorArn` | - | `DeleteLicenseManagerReportGeneratorResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Deletes the specified report generator. This action deletes the report generator, which stops it from generating future reports. |
| `DeleteToken` | - | - | `TokenId` | - | `DeleteTokenResponse` | `AccessDeniedException`, `AuthorizationException`, `RateLimitExceededException`, `RedirectException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Deletes the specified token. Must be called in the license home Region. |
| `ExtendLicenseConsumption` | - | - | `LicenseConsumptionToken` | - | `ExtendLicenseConsumptionResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Extends the expiration date for license consumption. |
| `GetAccessToken` | - | - | `Token` | - | `GetAccessTokenResponse` | `AccessDeniedException`, `AuthorizationException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Gets a temporary access token to use with AssumeRoleWithWebIdentity. Access tokens are valid for one hour. |
| `GetGrant` | - | - | `GrantArn` | - | `GetGrantResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Gets detailed information about the specified grant. |
| `GetLicense` | - | - | `LicenseArn` | - | `GetLicenseResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Gets detailed information about the specified license. |
| `GetLicenseAssetGroup` | - | - | `LicenseAssetGroupArn` | - | `GetLicenseAssetGroupResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Gets a license asset group. |
| `GetLicenseAssetRuleset` | - | - | `LicenseAssetRulesetArn` | - | `GetLicenseAssetRulesetResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Gets a license asset ruleset. |
| `GetLicenseConfiguration` | - | - | `LicenseConfigurationArn` | - | `GetLicenseConfigurationResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Gets detailed information about the specified license configuration. |
| `GetLicenseConversionTask` | - | - | `LicenseConversionTaskId` | - | `GetLicenseConversionTaskResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Gets information about the specified license type conversion task. |
| `GetLicenseManagerReportGenerator` | - | - | `LicenseManagerReportGeneratorArn` | - | `GetLicenseManagerReportGeneratorResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Gets information about the specified report generator. |
| `GetLicenseUsage` | - | - | `LicenseArn` | - | `GetLicenseUsageResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Gets detailed information about the usage of the specified license. |
| `GetServiceSettings` | - | - | - | - | `GetServiceSettingsResponse` | `AccessDeniedException`, `AuthorizationException`, `RateLimitExceededException`, `ServerInternalException` | Gets the License Manager settings for the current Region. |
| `ListAssetsForLicenseAssetGroup` | - | - | `AssetType`, `LicenseAssetGroupArn` | - | `ListAssetsForLicenseAssetGroupResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists assets for a license asset group. |
| `ListAssociationsForLicenseConfiguration` | - | - | `LicenseConfigurationArn` | - | `ListAssociationsForLicenseConfigurationResponse` | `AccessDeniedException`, `AuthorizationException`, `FilterLimitExceededException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists the resource associations for the specified license configuration. Resource associations need not consume licenses from a license configuration. |
| `ListDistributedGrants` | - | - | - | - | `ListDistributedGrantsResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists the grants distributed for the specified license. |
| `ListFailuresForLicenseConfigurationOperations` | - | - | `LicenseConfigurationArn` | - | `ListFailuresForLicenseConfigurationOperationsResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists the license configuration operations that failed. |
| `ListLicenseAssetGroups` | - | - | - | - | `ListLicenseAssetGroupsResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists license asset groups. |
| `ListLicenseAssetRulesets` | - | - | - | - | `ListLicenseAssetRulesetsResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists license asset rulesets. |
| `ListLicenseConfigurations` | - | - | - | - | `ListLicenseConfigurationsResponse` | `AccessDeniedException`, `AuthorizationException`, `FilterLimitExceededException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists the license configurations for your account. |
| `ListLicenseConfigurationsForOrganization` | - | - | - | - | `ListLicenseConfigurationsForOrganizationResponse` | `AccessDeniedException`, `AuthorizationException`, `FilterLimitExceededException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists license configurations for an organization. |
| `ListLicenseConversionTasks` | - | - | - | - | `ListLicenseConversionTasksResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists the license type conversion tasks for your account. |
| `ListLicenseManagerReportGenerators` | - | - | - | - | `ListLicenseManagerReportGeneratorsResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Lists the report generators for your account. |
| `ListLicenseSpecificationsForResource` | - | - | `ResourceArn` | - | `ListLicenseSpecificationsForResourceResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Describes the license configurations for the specified resource. |
| `ListLicenseVersions` | - | - | `LicenseArn` | - | `ListLicenseVersionsResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists all versions of the specified license. |
| `ListLicenses` | - | - | - | - | `ListLicensesResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists the licenses for your account. |
| `ListReceivedGrants` | - | - | - | - | `ListReceivedGrantsResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists grants that are received. Received grants are grants created while specifying the recipient as this Amazon Web Services account, your organization, or an organizational unit (OU) to which this member account belongs. |
| `ListReceivedGrantsForOrganization` | - | - | `LicenseArn` | - | `ListReceivedGrantsForOrganizationResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists the grants received for all accounts in the organization. |
| `ListReceivedLicenses` | - | - | - | - | `ListReceivedLicensesResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists received licenses. |
| `ListReceivedLicensesForOrganization` | - | - | - | - | `ListReceivedLicensesForOrganizationResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists the licenses received for all accounts in the organization. |
| `ListResourceInventory` | - | - | - | - | `ListResourceInventoryResponse` | `AccessDeniedException`, `AuthorizationException`, `FailedDependencyException`, `FilterLimitExceededException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists resources managed using Systems Manager inventory. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists the tags for the specified resource. For more information about tagging support in License Manager, see the TagResource operation. |
| `ListTokens` | - | - | - | - | `ListTokensResponse` | `AccessDeniedException`, `AuthorizationException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Lists your tokens. |
| `ListUsageForLicenseConfiguration` | - | - | `LicenseConfigurationArn` | - | `ListUsageForLicenseConfigurationResponse` | `AccessDeniedException`, `AuthorizationException`, `FilterLimitExceededException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException` | Lists all license usage records for a license configuration, displaying license consumption details by resource at a selected point in time. Use this action to audit the current license consumption for any license inventory and configuration. |
| `RejectGrant` | - | - | `GrantArn` | - | `RejectGrantResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException`, `ValidationException` | Rejects the specified grant. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Adds the specified tags to the specified resource. The following resources support tagging in License Manager: Licenses Grants License configurations Report generators |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Removes the specified tags from the specified resource. |
| `UpdateLicenseAssetGroup` | - | - | `AssociatedLicenseAssetRulesetARNs`, `ClientToken`, `LicenseAssetGroupArn` | - | `UpdateLicenseAssetGroupResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Updates a license asset group. |
| `UpdateLicenseAssetRuleset` | - | - | `ClientToken`, `LicenseAssetRulesetArn`, `Rules` | - | `UpdateLicenseAssetRulesetResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Updates a license asset ruleset. |
| `UpdateLicenseConfiguration` | - | - | `LicenseConfigurationArn` | - | `UpdateLicenseConfigurationResponse` | `AccessDeniedException`, `AuthorizationException`, `ConflictException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ServerInternalException` | Modifies the attributes of an existing license configuration. |
| `UpdateLicenseManagerReportGenerator` | - | - | `ClientToken`, `LicenseManagerReportGeneratorArn`, `ReportContext`, `ReportFrequency`, `ReportGeneratorName`, `Type` | - | `UpdateLicenseManagerReportGeneratorResponse` | `AccessDeniedException`, `AuthorizationException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerInternalException`, `ValidationException` | Updates a report generator. After you make changes to a report generator, it starts generating new reports within 60 minutes of being updated. |
| `UpdateLicenseSpecificationsForResource` | - | - | `ResourceArn` | - | `UpdateLicenseSpecificationsForResourceResponse` | `AccessDeniedException`, `AuthorizationException`, `ConflictException`, `InvalidParameterValueException`, `InvalidResourceStateException`, `LicenseUsageException`, `RateLimitExceededException`, `ServerInternalException` | Adds or removes the specified license configurations for the specified Amazon Web Services resource. You can update the license specifications of AMIs, instances, and hosts. |
| `UpdateServiceSettings` | - | - | - | - | `UpdateServiceSettingsResponse` | `AccessDeniedException`, `AuthorizationException`, `ConflictException`, `InvalidParameterValueException`, `RateLimitExceededException`, `ServerInternalException`, `ValidationException` | Updates License Manager settings for the current Region. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | Access to resource denied. |
| `AuthorizationException` | `structure` | `Message` | The Amazon Web Services user account does not have permission to perform the action. |
| `RateLimitExceededException` | `structure` | `Message` | Too many requests have been submitted. |
| `ServerInternalException` | `structure` | `Message` | The server experienced an internal error. |
| `InvalidParameterValueException` | `structure` | `Message` | One or more parameter values are not valid. |
| `ValidationException` | `structure` | `Message` | The provided input is not valid. |
| `ResourceLimitExceededException` | `structure` | `Message` | Your resource limits have been exceeded. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource cannot be found. |
| `RedirectException` | `structure` | `Location`, `Message` | This is not the correct Region for the resource. |
| `ConflictException` | `structure` | `Message` | There was a conflict processing the request. |
| `FilterLimitExceededException` | `structure` | `Message` | The request uses too many filters or too many filter values. |
| `NoEntitlementsAllowedException` | `structure` | `Message` | There are no entitlements found for this license, or the entitlement maximum count is reached. |
| `UnsupportedDigitalSignatureMethodException` | `structure` | `Message` | The digital signature method is unsupported. |
| `AcceptGrantRequest` | `structure` | `GrantArn` | - |
| `AcceptGrantResponse` | `structure` | `GrantArn`, `Status`, `Version` | - |
| `CheckInLicenseRequest` | `structure` | `Beneficiary`, `LicenseConsumptionToken` | - |
| `CheckInLicenseResponse` | `structure` | - | - |
| `CheckoutBorrowLicenseRequest` | `structure` | `CheckoutMetadata`, `ClientToken`, `DigitalSignatureMethod`, `Entitlements`, `LicenseArn`, `NodeId` | - |
| `CheckoutBorrowLicenseResponse` | `structure` | `CheckoutMetadata`, `EntitlementsAllowed`, `Expiration`, `IssuedAt`, `LicenseArn`, `LicenseConsumptionToken`, `NodeId`, `SignedToken` | - |
| `EntitlementNotAllowedException` | `structure` | `Message` | The entitlement is not allowed. |
| `CheckoutLicenseRequest` | `structure` | `Beneficiary`, `CheckoutType`, `ClientToken`, `Entitlements`, `KeyFingerprint`, `NodeId`, `ProductSKU` | - |
| `CheckoutLicenseResponse` | `structure` | `CheckoutType`, `EntitlementsAllowed`, `Expiration`, `IssuedAt`, `LicenseArn`, `LicenseConsumptionToken`, `NodeId`, `SignedToken` | - |
| `CreateGrantRequest` | `structure` | `AllowedOperations`, `ClientToken`, `GrantName`, `HomeRegion`, `LicenseArn`, `Principals`, `Tags` | - |
| `CreateGrantResponse` | `structure` | `GrantArn`, `Status`, `Version` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
