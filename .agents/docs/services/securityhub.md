# AWS SecurityHub

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Security Hub CSPM provides you with a comprehensive view of your security state in Amazon Web Services and helps you assess your Amazon Web Services environment against security industry standards and best practices. Security Hub CSPM collects security data across Amazon Web Services accounts, Amazon Web Services services, and supported third-party products and helps you analyze your security trends and identify the highest priority security issues. To help you manage the security state of your organization, Security Hub CSPM supports multiple security standards. These include the Amazon Web Services Foundational Security Best Practices (FSBP) standard developed by Amazon Web Services, and external compliance frameworks such as the Center for Internet Security (CIS), the Payment Card Industry Data Security Standard (PCI DSS), and the National Institute of Standards and Technology (NIST). Each standard includes several security controls, each of which represents a security best practice.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-securityhub/tests/scenario_test.rs`: import findings, triage them by updating workflow/state fields, and verify finding query behaviour.
- Backported from `scenario_test.rs`: manage member accounts.
- Backported from `scenario_test.rs`: create, update, describe, and delete automation rules.
- Scenario insight from EC2: include mutable binding failover for AWS SecurityHub where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS SecurityHub by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS SecurityHub by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: support security finding aggregation, standards/subscriptions, insights, member administration, automation rules, finding batch import/update, and integration metadata.

## Service Identity and Protocol

- AWS model slug: `securityhub`
- AWS SDK for Rust slug: `securityhub`
- Model version: `2018-10-26`
- Model file: `vendor/api-models-aws/models/securityhub/service/2018-10-26/securityhub-2018-10-26.json`
- SDK ID: `SecurityHub`
- Endpoint prefix: `securityhub`
- ARN namespace: `securityhub`
- CloudFormation name: `SecurityHub`
- CloudTrail event source: `securityhub.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (22), `List` (14), `Batch` (12), `Update` (12), `Create` (10), `Delete` (9), `Describe` (8), `Disable` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptAdministratorInvitation`, `AcceptInvitation`, `BatchDeleteAutomationRules`, `BatchDisableStandards`, `BatchEnableStandards`, `BatchGetAutomationRules`, `BatchGetConfigurationPolicyAssociations`, `BatchGetSecurityControls`, `BatchGetStandardsControlAssociations`, `BatchImportFindings`, `BatchUpdateAutomationRules`, `BatchUpdateFindings`, `BatchUpdateFindingsV2`, `BatchUpdateStandardsControlAssociations`, `CreateActionTarget`, `CreateAggregatorV2`, `CreateAutomationRule`, `CreateAutomationRuleV2`, `CreateConfigurationPolicy`, `CreateConnectorV2`, ... (+41).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeActionTargets`, `DescribeHub`, `DescribeOrganizationConfiguration`, `DescribeProducts`, `DescribeProductsV2`, `DescribeSecurityHubV2`, `DescribeStandards`, `DescribeStandardsControls`, `GetAdministratorAccount`, `GetAggregatorV2`, `GetAutomationRuleV2`, `GetConfigurationPolicy`, `GetConfigurationPolicyAssociation`, `GetConnectorV2`, `GetEnabledStandards`, `GetFindingAggregator`, `GetFindingHistory`, `GetFindingStatisticsV2`, `GetFindings`, `GetFindingsTrendsV2`, ... (+24).
- Pagination is modelled for 23 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchImportFindings`, `DisableImportFindingsForProduct`, `EnableImportFindingsForProduct`, `ListEnabledProductsForImport`, `StartConfigurationPolicyAssociation`, `StartConfigurationPolicyDisassociation`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 107 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SNS`, `SQS`, `Lambda`, `EC2/VPC`, `ECR`, `ECS`, `EKS`, `RDS`, `Redshift`, `Secrets Manager`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAdministratorAccount`, `GetAggregatorV2`, `GetAutomationRuleV2`, `GetConfigurationPolicy`, `GetConfigurationPolicyAssociation`, `GetConnectorV2`, `GetEnabledStandards`, `GetFindingAggregator`, `GetFindingHistory`, `GetFindingStatisticsV2`, `GetFindings`, `GetFindingsTrendsV2`, `GetFindingsV2`, `GetInsightResults`, `GetInsights`, `GetInvitationsCount`, `GetMasterAccount`, `GetMembers`, `GetResourcesStatisticsV2`, `GetResourcesTrendsV2`, `GetResourcesV2`, `GetSecurityControlDefinition`
- Traits: `paginated` (8)
- Common required input members in this group: `AccountIds`, `AggregatorV2Arn`, `ConnectorId`, `EndTime`, `FindingAggregatorArn`, `FindingIdentifier`, `GroupByRules`, `Identifier`, `InsightArn`, `SecurityControlId`, `StartTime`, `Target`

### List

- Operations: `ListAggregatorsV2`, `ListAutomationRules`, `ListAutomationRulesV2`, `ListConfigurationPolicies`, `ListConfigurationPolicyAssociations`, `ListConnectorsV2`, `ListEnabledProductsForImport`, `ListFindingAggregators`, `ListInvitations`, `ListMembers`, `ListOrganizationAdminAccounts`, `ListSecurityControlDefinitions`, `ListStandardsControlAssociations`, `ListTagsForResource`
- Traits: `paginated` (10)
- Common required input members in this group: `ResourceArn`, `SecurityControlId`

### Batch

- Operations: `BatchDeleteAutomationRules`, `BatchDisableStandards`, `BatchEnableStandards`, `BatchGetAutomationRules`, `BatchGetConfigurationPolicyAssociations`, `BatchGetSecurityControls`, `BatchGetStandardsControlAssociations`, `BatchImportFindings`, `BatchUpdateAutomationRules`, `BatchUpdateFindings`, `BatchUpdateFindingsV2`, `BatchUpdateStandardsControlAssociations`
- Common required input members in this group: `AutomationRulesArns`, `ConfigurationPolicyAssociationIdentifiers`, `FindingIdentifiers`, `Findings`, `SecurityControlIds`, `StandardsControlAssociationIds`, `StandardsControlAssociationUpdates`, `StandardsSubscriptionArns`, `StandardsSubscriptionRequests`, `UpdateAutomationRulesRequestItems`

### Update

- Operations: `UpdateActionTarget`, `UpdateAggregatorV2`, `UpdateAutomationRuleV2`, `UpdateConfigurationPolicy`, `UpdateConnectorV2`, `UpdateFindingAggregator`, `UpdateFindings`, `UpdateInsight`, `UpdateOrganizationConfiguration`, `UpdateSecurityControl`, `UpdateSecurityHubConfiguration`, `UpdateStandardsControl`
- Common required input members in this group: `ActionTargetArn`, `AggregatorV2Arn`, `AutoEnable`, `ConnectorId`, `Filters`, `FindingAggregatorArn`, `Identifier`, `InsightArn`, `Parameters`, `RegionLinkingMode`, `SecurityControlId`, `StandardsControlArn`

### Create

- Operations: `CreateActionTarget`, `CreateAggregatorV2`, `CreateAutomationRule`, `CreateAutomationRuleV2`, `CreateConfigurationPolicy`, `CreateConnectorV2`, `CreateFindingAggregator`, `CreateInsight`, `CreateMembers`, `CreateTicketV2`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `AccountDetails`, `Actions`, `ConfigurationPolicy`, `ConnectorId`, `Criteria`, `Description`, `Filters`, `FindingMetadataUid`, `GroupByAttribute`, `Id`, `Name`, `Provider`, `RegionLinkingMode`, `RuleName`, `RuleOrder`

### Delete

- Operations: `DeleteActionTarget`, `DeleteAggregatorV2`, `DeleteAutomationRuleV2`, `DeleteConfigurationPolicy`, `DeleteConnectorV2`, `DeleteFindingAggregator`, `DeleteInsight`, `DeleteInvitations`, `DeleteMembers`
- Common required input members in this group: `AccountIds`, `ActionTargetArn`, `AggregatorV2Arn`, `ConnectorId`, `FindingAggregatorArn`, `Identifier`, `InsightArn`

### Describe

- Operations: `DescribeActionTargets`, `DescribeHub`, `DescribeOrganizationConfiguration`, `DescribeProducts`, `DescribeProductsV2`, `DescribeSecurityHubV2`, `DescribeStandards`, `DescribeStandardsControls`
- Traits: `paginated` (5)
- Common required input members in this group: `StandardsSubscriptionArn`

### Disable

- Operations: `DisableImportFindingsForProduct`, `DisableOrganizationAdminAccount`, `DisableSecurityHub`, `DisableSecurityHubV2`
- Common required input members in this group: `AdminAccountId`, `ProductSubscriptionArn`

### Enable

- Operations: `EnableImportFindingsForProduct`, `EnableOrganizationAdminAccount`, `EnableSecurityHub`, `EnableSecurityHubV2`
- Common required input members in this group: `AdminAccountId`, `ProductArn`

### Disassociate

- Operations: `DisassociateFromAdministratorAccount`, `DisassociateFromMasterAccount`, `DisassociateMembers`
- Common required input members in this group: `AccountIds`

### Accept

- Operations: `AcceptAdministratorInvitation`, `AcceptInvitation`
- Common required input members in this group: `AdministratorId`, `InvitationId`, `MasterId`

### Start

- Operations: `StartConfigurationPolicyAssociation`, `StartConfigurationPolicyDisassociation`
- Common required input members in this group: `ConfigurationPolicyIdentifier`, `Target`

### Decline

- Operations: `DeclineInvitations`
- Common required input members in this group: `AccountIds`

### Invite

- Operations: `InviteMembers`
- Common required input members in this group: `AccountIds`

### Register

- Operations: `RegisterConnectorV2`
- Common required input members in this group: `AuthCode`, `AuthState`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptAdministratorInvitation` | `POST /administrator` | - | `AdministratorId`, `InvitationId` | - | `AcceptAdministratorInvitationResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | We recommend using Organizations instead of Security Hub CSPM invitations to manage your member accounts. For information, see Managing Security Hub CSPM administrator and member accounts with Organizations in the Security Hub CSPM User Guide . |
| `AcceptInvitation` | `POST /master` | - | `InvitationId`, `MasterId` | - | `AcceptInvitationResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | This method is deprecated. Instead, use `AcceptAdministratorInvitation`. |
| `BatchDeleteAutomationRules` | `POST /automationrules/delete` | - | `AutomationRulesArns` | - | `BatchDeleteAutomationRulesResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Deletes one or more automation rules. |
| `BatchDisableStandards` | `POST /standards/deregister` | - | `StandardsSubscriptionArns` | - | `BatchDisableStandardsResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Disables the standards specified by the provided `StandardsSubscriptionArns`. For more information, see Security Standards section of the Security Hub CSPM User Guide . |
| `BatchEnableStandards` | `POST /standards/register` | - | `StandardsSubscriptionRequests` | - | `BatchEnableStandardsResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Enables the standards specified by the provided `StandardsArn`. To obtain the ARN for a standard, use the `DescribeStandards` operation. |
| `BatchGetAutomationRules` | `POST /automationrules/get` | - | `AutomationRulesArns` | - | `BatchGetAutomationRulesResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Retrieves a list of details for automation rules based on rule Amazon Resource Names (ARNs). |
| `BatchGetConfigurationPolicyAssociations` | `POST /configurationPolicyAssociation/batchget` | - | `ConfigurationPolicyAssociationIdentifiers` | - | `BatchGetConfigurationPolicyAssociationsResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Returns associations between an Security Hub CSPM configuration and a batch of target accounts, organizational units, or the root. Only the Security Hub CSPM delegated administrator can invoke this operation from the home Region. |
| `BatchGetSecurityControls` | `POST /securityControls/batchGet` | - | `SecurityControlIds` | - | `BatchGetSecurityControlsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Provides details about a batch of security controls for the current Amazon Web Services account and Amazon Web Services Region. |
| `BatchGetStandardsControlAssociations` | `POST /associations/batchGet` | - | `StandardsControlAssociationIds` | - | `BatchGetStandardsControlAssociationsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | For a batch of security controls and standards, identifies whether each control is currently enabled or disabled in a standard. Calls to this operation return a `RESOURCE_NOT_FOUND_EXCEPTION` error when the standard subscription for the association has a... |
| `BatchImportFindings` | `POST /findings/import` | - | `Findings` | - | `BatchImportFindingsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Imports security findings generated by a finding provider into Security Hub CSPM. This action is requested by the finding provider to import its findings into Security Hub CSPM. |
| `BatchUpdateAutomationRules` | `PATCH /automationrules/update` | - | `UpdateAutomationRulesRequestItems` | - | `BatchUpdateAutomationRulesResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Updates one or more automation rules based on rule Amazon Resource Names (ARNs) and input parameters. |
| `BatchUpdateFindings` | `PATCH /findings/batchupdate` | - | `FindingIdentifiers` | - | `BatchUpdateFindingsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Used by Security Hub CSPM customers to update information about their investigation into one or more findings. Requested by administrator accounts or member accounts. |
| `BatchUpdateFindingsV2` | `PATCH /findingsv2/batchupdatev2` | - | - | - | `BatchUpdateFindingsV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Used by customers to update information about their investigation into a finding. Requested by delegated administrator accounts or member accounts. |
| `BatchUpdateStandardsControlAssociations` | `PATCH /associations` | - | `StandardsControlAssociationUpdates` | - | `BatchUpdateStandardsControlAssociationsResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | For a batch of security controls and standards, this operation updates the enablement status of a control in a standard. |
| `CreateActionTarget` | `POST /actionTargets` | - | `Description`, `Id`, `Name` | - | `CreateActionTargetResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException` | Creates a custom action target in Security Hub CSPM. You can use custom actions on findings and insights in Security Hub CSPM to trigger target actions in Amazon CloudWatch Events. |
| `CreateAggregatorV2` | `POST /aggregatorv2/create` | `idempotency-token` | `RegionLinkingMode` | `ClientToken` | `CreateAggregatorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables aggregation across Amazon Web Services Regions. |
| `CreateAutomationRule` | `POST /automationrules/create` | - | `Actions`, `Criteria`, `Description`, `RuleName`, `RuleOrder` | - | `CreateAutomationRuleResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Creates an automation rule based on input parameters. |
| `CreateAutomationRuleV2` | `POST /automationrulesv2/create` | `idempotency-token` | `Actions`, `Criteria`, `Description`, `RuleName`, `RuleOrder` | `ClientToken` | `CreateAutomationRuleV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a V2 automation rule. |
| `CreateConfigurationPolicy` | `POST /configurationPolicy/create` | - | `ConfigurationPolicy`, `Name` | - | `CreateConfigurationPolicyResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException` | Creates a configuration policy with the defined configuration. Only the Security Hub CSPM delegated administrator can invoke this operation from the home Region. |
| `CreateConnectorV2` | `POST /connectorsv2` | `idempotency-token` | `Name`, `Provider` | `ClientToken` | `CreateConnectorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Grants permission to create a connectorV2 based on input parameters. |
| `CreateFindingAggregator` | `POST /findingAggregator/create` | - | `RegionLinkingMode` | - | `CreateFindingAggregatorResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | The aggregation Region is now called the home Region . Used to enable cross-Region aggregation. |
| `CreateInsight` | `POST /insights` | - | `Filters`, `GroupByAttribute`, `Name` | - | `CreateInsightResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException` | Creates a custom insight in Security Hub CSPM. An insight is a consolidation of findings that relate to a security issue that requires attention or remediation. |
| `CreateMembers` | `POST /members` | - | `AccountDetails` | - | `CreateMembersResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException` | Creates a member association in Security Hub CSPM between the specified accounts and the account used to make the request, which is the administrator account. If you are integrated with Organizations, then the administrator account is designated by the... |
| `CreateTicketV2` | `POST /ticketsv2` | `idempotency-token` | `ConnectorId`, `FindingMetadataUid` | `ClientToken` | `CreateTicketV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Grants permission to create a ticket in the chosen ITSM based on finding information for the provided finding metadata UID. |
| `DeclineInvitations` | `POST /invitations/decline` | - | `AccountIds` | - | `DeclineInvitationsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `ResourceNotFoundException` | We recommend using Organizations instead of Security Hub CSPM invitations to manage your member accounts. For information, see Managing Security Hub CSPM administrator and member accounts with Organizations in the Security Hub CSPM User Guide . |
| `DeleteActionTarget` | `DELETE /actionTargets/{ActionTargetArn+}` | - | `ActionTargetArn` | - | `DeleteActionTargetResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `ResourceNotFoundException` | Deletes a custom action target from Security Hub CSPM. Deleting a custom action target does not affect any findings or insights that were already sent to Amazon CloudWatch Events using the custom action. |
| `DeleteAggregatorV2` | `DELETE /aggregatorv2/delete/{AggregatorV2Arn+}` | - | `AggregatorV2Arn` | - | `DeleteAggregatorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the Aggregator V2. |
| `DeleteAutomationRuleV2` | `DELETE /automationrulesv2/{Identifier}` | - | `Identifier` | - | `DeleteAutomationRuleV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a V2 automation rule. |
| `DeleteConfigurationPolicy` | `DELETE /configurationPolicy/{Identifier}` | - | `Identifier` | - | `DeleteConfigurationPolicyResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException`, `ResourceNotFoundException` | Deletes a configuration policy. Only the Security Hub CSPM delegated administrator can invoke this operation from the home Region. |
| `DeleteConnectorV2` | `DELETE /connectorsv2/{ConnectorId+}` | - | `ConnectorId` | - | `DeleteConnectorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Grants permission to delete a connectorV2. |
| `DeleteFindingAggregator` | `DELETE /findingAggregator/delete/{FindingAggregatorArn+}` | - | `FindingAggregatorArn` | - | `DeleteFindingAggregatorResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | The aggregation Region is now called the home Region . Deletes a finding aggregator. |
| `DeleteInsight` | `DELETE /insights/{InsightArn+}` | - | `InsightArn` | - | `DeleteInsightResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Deletes the insight specified by the `InsightArn`. |
| `DeleteInvitations` | `POST /invitations/delete` | - | `AccountIds` | - | `DeleteInvitationsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | We recommend using Organizations instead of Security Hub CSPM invitations to manage your member accounts. For information, see Managing Security Hub CSPM administrator and member accounts with Organizations in the Security Hub CSPM User Guide . |
| `DeleteMembers` | `POST /members/delete` | - | `AccountIds` | - | `DeleteMembersResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Deletes the specified member accounts from Security Hub CSPM. You can invoke this API only to delete accounts that became members through invitation. |
| `DescribeActionTargets` | `POST /actionTargets/get` | `paginated` | - | - | `DescribeActionTargetsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `ResourceNotFoundException` | Returns a list of the custom action targets in Security Hub CSPM in your account. |
| `DescribeHub` | `GET /accounts` | - | - | - | `DescribeHubResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Returns details about the Hub resource in your account, including the `HubArn` and the time when you enabled Security Hub CSPM. |
| `DescribeOrganizationConfiguration` | `GET /organization/configuration` | - | - | - | `DescribeOrganizationConfigurationResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Returns information about the way your organization is configured in Security Hub CSPM. Only the Security Hub CSPM administrator account can invoke this operation. |
| `DescribeProducts` | `GET /products` | `paginated` | - | - | `DescribeProductsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Returns information about product integrations in Security Hub CSPM. You can optionally provide an integration ARN. |
| `DescribeProductsV2` | `GET /productsV2` | `paginated` | - | - | `DescribeProductsV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets information about the product integration. |
| `DescribeSecurityHubV2` | `GET /hubv2` | - | - | - | `DescribeSecurityHubV2Response` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about the service resource in your account. |
| `DescribeStandards` | `GET /standards` | `paginated` | - | - | `DescribeStandardsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException` | Returns a list of the available standards in Security Hub CSPM. For each standard, the results include the standard ARN, the name, and a description. |
| `DescribeStandardsControls` | `GET /standards/controls/{StandardsSubscriptionArn+}` | `paginated` | `StandardsSubscriptionArn` | - | `DescribeStandardsControlsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `ResourceNotFoundException` | Returns a list of security standards controls. For each control, the results include information about whether it is currently enabled, the severity, and a link to remediation information. |
| `DisableImportFindingsForProduct` | `DELETE /productSubscriptions/{ProductSubscriptionArn+}` | - | `ProductSubscriptionArn` | - | `DisableImportFindingsForProductResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Disables the integration of the specified product with Security Hub CSPM. After the integration is disabled, findings from that product are no longer sent to Security Hub CSPM. |
| `DisableOrganizationAdminAccount` | `POST /organization/admin/disable` | - | `AdminAccountId` | - | `DisableOrganizationAdminAccountResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Disables a Security Hub CSPM administrator account. Can only be called by the organization management account. |
| `DisableSecurityHub` | `DELETE /accounts` | - | - | - | `DisableSecurityHubResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `LimitExceededException`, `ResourceNotFoundException` | Disables Security Hub CSPM in your account only in the current Amazon Web Services Region. To disable Security Hub CSPM in all Regions, you must submit one request per Region where you have enabled Security Hub CSPM. |
| `DisableSecurityHubV2` | `DELETE /hubv2` | - | - | - | `DisableSecurityHubV2Response` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Disable the service for the current Amazon Web Services Region or specified Amazon Web Services Region. |
| `DisassociateFromAdministratorAccount` | `POST /administrator/disassociate` | - | - | - | `DisassociateFromAdministratorAccountResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Disassociates the current Security Hub CSPM member account from the associated administrator account. This operation is only used by accounts that are not part of an organization. |
| `DisassociateFromMasterAccount` | `POST /master/disassociate` | - | - | - | `DisassociateFromMasterAccountResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | This method is deprecated. Instead, use `DisassociateFromAdministratorAccount`. |
| `DisassociateMembers` | `POST /members/disassociate` | - | `AccountIds` | - | `DisassociateMembersResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Disassociates the specified member accounts from the associated administrator account. Can be used to disassociate both accounts that are managed using Organizations and accounts that were invited manually. |
| `EnableImportFindingsForProduct` | `POST /productSubscriptions` | - | `ProductArn` | - | `EnableImportFindingsForProductResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException` | Enables the integration of a partner product with Security Hub CSPM. Integrated products send findings to Security Hub CSPM. |
| `EnableOrganizationAdminAccount` | `POST /organization/admin/enable` | - | `AdminAccountId` | - | `EnableOrganizationAdminAccountResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Designates the Security Hub CSPM administrator account for an organization. Can only be called by the organization management account. |
| `EnableSecurityHub` | `POST /accounts` | - | - | - | `EnableSecurityHubResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `LimitExceededException`, `ResourceConflictException` | Enables Security Hub CSPM for your account in the current Region or the Region you specify in the request. When you enable Security Hub CSPM, you grant to Security Hub CSPM the permissions necessary to gather findings from other services that are integrated... |
| `EnableSecurityHubV2` | `POST /hubv2` | - | - | - | `EnableSecurityHubV2Response` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Enables the service in account for the current Amazon Web Services Region or specified Amazon Web Services Region. |
| `GetAdministratorAccount` | `GET /administrator` | - | - | - | `GetAdministratorAccountResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Provides the details for the Security Hub CSPM administrator account for the current member account. Can be used by both member accounts that are managed using Organizations and accounts that were invited manually. |
| `GetAggregatorV2` | `GET /aggregatorv2/get/{AggregatorV2Arn+}` | - | `AggregatorV2Arn` | - | `GetAggregatorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the configuration of the specified Aggregator V2. |
| `GetAutomationRuleV2` | `GET /automationrulesv2/{Identifier}` | - | `Identifier` | - | `GetAutomationRuleV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an automation rule for the V2 service. |
| `GetConfigurationPolicy` | `GET /configurationPolicy/get/{Identifier}` | - | `Identifier` | - | `GetConfigurationPolicyResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Provides information about a configuration policy. Only the Security Hub CSPM delegated administrator can invoke this operation from the home Region. |
| `GetConfigurationPolicyAssociation` | `POST /configurationPolicyAssociation/get` | - | `Target` | - | `GetConfigurationPolicyAssociationResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Returns the association between a configuration and a target account, organizational unit, or the root. The configuration can be a configuration policy or self-managed behavior. |
| `GetConnectorV2` | `GET /connectorsv2/{ConnectorId+}` | - | `ConnectorId` | - | `GetConnectorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Grants permission to retrieve details for a connectorV2 based on connector id. |
| `GetEnabledStandards` | `POST /standards/get` | `paginated` | - | - | `GetEnabledStandardsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Returns a list of the standards that are currently enabled. |
| `GetFindingAggregator` | `GET /findingAggregator/get/{FindingAggregatorArn+}` | - | `FindingAggregatorArn` | - | `GetFindingAggregatorResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | The aggregation Region is now called the home Region . Returns the current configuration in the calling account for cross-Region aggregation. |
| `GetFindingHistory` | `POST /findingHistory/get` | `paginated` | `FindingIdentifier` | - | `GetFindingHistoryResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Returns the history of a Security Hub CSPM finding. The history includes changes made to any fields in the Amazon Web Services Security Finding Format (ASFF) except top-level timestamp fields, such as the `CreatedAt` and `UpdatedAt` fields. |
| `GetFindingStatisticsV2` | `POST /findingsv2/statistics` | - | `GroupByRules` | - | `GetFindingStatisticsV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns aggregated statistical data about findings. `GetFindingStatisticsV2` use `securityhub:GetAdhocInsightResults` in the `Action` element of an IAM policy statement. |
| `GetFindings` | `POST /findings` | `paginated` | - | - | `GetFindingsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Returns a list of findings that match the specified criteria. If cross-Region aggregation is enabled, then when you call `GetFindings` from the home Region, the results include all of the matching findings from both the home Region and linked Regions. |
| `GetFindingsTrendsV2` | `POST /findingsTrendsv2` | `paginated` | `EndTime`, `StartTime` | - | `GetFindingsTrendsV2Response` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns findings trend data based on the specified criteria. This operation helps you analyze patterns and changes in findings over time. |
| `GetFindingsV2` | `POST /findingsv2` | `paginated` | - | - | `GetFindingsV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Return a list of findings that match the specified criteria. `GetFindings` and `GetFindingsV2` both use `securityhub:GetFindings` in the `Action` element of an IAM policy statement. |
| `GetInsightResults` | `GET /insights/results/{InsightArn+}` | - | `InsightArn` | - | `GetInsightResultsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Lists the results of the Security Hub CSPM insight specified by the insight ARN. |
| `GetInsights` | `POST /insights/get` | `paginated` | - | - | `GetInsightsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Lists and describes insights for the specified insight ARNs. |
| `GetInvitationsCount` | `GET /invitations/count` | - | - | - | `GetInvitationsCountResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | We recommend using Organizations instead of Security Hub CSPM invitations to manage your member accounts. For information, see Managing Security Hub CSPM administrator and member accounts with Organizations in the Security Hub CSPM User Guide . |
| `GetMasterAccount` | `GET /master` | - | - | - | `GetMasterAccountResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | This method is deprecated. Instead, use `GetAdministratorAccount`. |
| `GetMembers` | `POST /members/get` | - | `AccountIds` | - | `GetMembersResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Returns the details for the Security Hub CSPM member accounts for the specified account IDs. An administrator account can be either the delegated Security Hub CSPM administrator account for an organization or an administrator account that enabled Security Hub... |
| `GetResourcesStatisticsV2` | `POST /resourcesv2/statistics` | - | `GroupByRules` | - | `GetResourcesStatisticsV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves statistical information about Amazon Web Services resources and their associated security findings. |
| `GetResourcesTrendsV2` | `POST /resourcesTrendsv2` | `paginated` | `EndTime`, `StartTime` | - | `GetResourcesTrendsV2Response` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns resource trend data based on the specified criteria. This operation helps you analyze patterns and changes in resource compliance over time. |
| `GetResourcesV2` | `POST /resourcesv2` | `paginated` | - | - | `GetResourcesV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of resources. |
| `GetSecurityControlDefinition` | `GET /securityControl/definition` | - | `SecurityControlId` | - | `GetSecurityControlDefinitionResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Retrieves the definition of a security control. The definition includes the control title, description, Region availability, parameter definitions, and other details. |
| `InviteMembers` | `POST /members/invite` | - | `AccountIds` | - | `InviteMembersResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | We recommend using Organizations instead of Security Hub CSPM invitations to manage your member accounts. For information, see Managing Security Hub CSPM administrator and member accounts with Organizations in the Security Hub CSPM User Guide . |
| `ListAggregatorsV2` | `GET /aggregatorv2/list` | `paginated` | - | - | `ListAggregatorsV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of V2 aggregators. |
| `ListAutomationRules` | `GET /automationrules/list` | - | - | - | `ListAutomationRulesResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | A list of automation rules and their metadata for the calling account. |
| `ListAutomationRulesV2` | `GET /automationrulesv2/list` | - | - | - | `ListAutomationRulesV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of automation rules and metadata for the calling account. |
| `ListConfigurationPolicies` | `GET /configurationPolicy/list` | `paginated` | - | - | `ListConfigurationPoliciesResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Lists the configuration policies that the Security Hub CSPM delegated administrator has created for your organization. Only the delegated administrator can invoke this operation from the home Region. |
| `ListConfigurationPolicyAssociations` | `POST /configurationPolicyAssociation/list` | `paginated` | - | - | `ListConfigurationPolicyAssociationsResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Provides information about the associations for your configuration policies and self-managed behavior. Only the Security Hub CSPM delegated administrator can invoke this operation from the home Region. |
| `ListConnectorsV2` | `GET /connectorsv2` | - | - | - | `ListConnectorsV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Grants permission to retrieve a list of connectorsV2 and their metadata for the calling account. |
| `ListEnabledProductsForImport` | `GET /productSubscriptions` | `paginated` | - | - | `ListEnabledProductsForImportResponse` | `InternalException`, `InvalidAccessException`, `LimitExceededException` | Lists all findings-generating solutions (products) that you are subscribed to receive findings from in Security Hub CSPM. |
| `ListFindingAggregators` | `GET /findingAggregator/list` | `paginated` | - | - | `ListFindingAggregatorsResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | If cross-Region aggregation is enabled, then `ListFindingAggregators` returns the Amazon Resource Name (ARN) of the finding aggregator. You can run this operation from any Amazon Web Services Region. |
| `ListInvitations` | `GET /invitations` | `paginated` | - | - | `ListInvitationsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | We recommend using Organizations instead of Security Hub CSPM invitations to manage your member accounts. For information, see Managing Security Hub CSPM administrator and member accounts with Organizations in the Security Hub CSPM User Guide . |
| `ListMembers` | `GET /members` | `paginated` | - | - | `ListMembersResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Lists details about all member accounts for the current Security Hub CSPM administrator account. The results include both member accounts that belong to an organization and member accounts that were invited manually. |
| `ListOrganizationAdminAccounts` | `GET /organization/admin` | `paginated` | - | - | `ListOrganizationAdminAccountsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Lists the Security Hub CSPM administrator accounts. Can only be called by the organization management account. |
| `ListSecurityControlDefinitions` | `GET /securityControls/definitions` | `paginated` | - | - | `ListSecurityControlDefinitionsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Lists all of the security controls that apply to a specified standard. |
| `ListStandardsControlAssociations` | `GET /associations` | `paginated` | `SecurityControlId` | - | `ListStandardsControlAssociationsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException` | Specifies whether a control is currently enabled or disabled in each enabled standard in the calling account. This operation omits standards control associations for standard subscriptions where `StandardsControlsUpdatable` has value `NOT_READY_FOR_UPDATES`. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalException`, `InvalidInputException`, `ResourceNotFoundException` | Returns a list of tags associated with a resource. |
| `RegisterConnectorV2` | `POST /connectorsv2/register` | - | `AuthCode`, `AuthState` | - | `RegisterConnectorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Grants permission to complete the authorization based on input parameters. |
| `StartConfigurationPolicyAssociation` | `POST /configurationPolicyAssociation/associate` | - | `ConfigurationPolicyIdentifier`, `Target` | - | `StartConfigurationPolicyAssociationResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Associates a target account, organizational unit, or the root with a specified configuration. The target can be associated with a configuration policy or self-managed behavior. |
| `StartConfigurationPolicyDisassociation` | `POST /configurationPolicyAssociation/disassociate` | - | `ConfigurationPolicyIdentifier` | - | `StartConfigurationPolicyDisassociationResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Disassociates a target account, organizational unit, or the root from a specified configuration. When you disassociate a configuration from its target, the target inherits the configuration of the closest parent. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalException`, `InvalidInputException`, `ResourceNotFoundException` | Adds one or more tags to a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalException`, `InvalidInputException`, `ResourceNotFoundException` | Removes one or more tags from a resource. |
| `UpdateActionTarget` | `PATCH /actionTargets/{ActionTargetArn+}` | - | `ActionTargetArn` | - | `UpdateActionTargetResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `ResourceNotFoundException` | Updates the name and description of a custom action target in Security Hub CSPM. |
| `UpdateAggregatorV2` | `PATCH /aggregatorv2/update/{AggregatorV2Arn+}` | - | `AggregatorV2Arn`, `RegionLinkingMode` | - | `UpdateAggregatorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Udpates the configuration for the Aggregator V2. |
| `UpdateAutomationRuleV2` | `PATCH /automationrulesv2/{Identifier}` | - | `Identifier` | - | `UpdateAutomationRuleV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a V2 automation rule. |
| `UpdateConfigurationPolicy` | `PATCH /configurationPolicy/{Identifier}` | - | `Identifier` | - | `UpdateConfigurationPolicyResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException`, `ResourceNotFoundException` | Updates a configuration policy. Only the Security Hub CSPM delegated administrator can invoke this operation from the home Region. |
| `UpdateConnectorV2` | `PATCH /connectorsv2/{ConnectorId+}` | - | `ConnectorId` | - | `UpdateConnectorV2Response` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Grants permission to update a connectorV2 based on its id and input parameters. |
| `UpdateFindingAggregator` | `PATCH /findingAggregator/update` | - | `FindingAggregatorArn`, `RegionLinkingMode` | - | `UpdateFindingAggregatorResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | The aggregation Region is now called the home Region . Updates cross-Region aggregation settings. |
| `UpdateFindings` | `PATCH /findings` | - | `Filters` | - | `UpdateFindingsResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | `UpdateFindings` is a deprecated operation. Instead of `UpdateFindings`, use the `BatchUpdateFindings` operation. |
| `UpdateInsight` | `PATCH /insights/{InsightArn+}` | - | `InsightArn` | - | `UpdateInsightResponse` | `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Updates the Security Hub CSPM insight identified by the specified insight ARN. |
| `UpdateOrganizationConfiguration` | `POST /organization/configuration` | - | `AutoEnable` | - | `UpdateOrganizationConfigurationResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceConflictException`, `ResourceNotFoundException` | Updates the configuration of your organization in Security Hub CSPM. Only the Security Hub CSPM administrator account can invoke this operation. |
| `UpdateSecurityControl` | `PATCH /securityControl/update` | - | `Parameters`, `SecurityControlId` | - | `UpdateSecurityControlResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates the properties of a security control. |
| `UpdateSecurityHubConfiguration` | `PATCH /accounts` | - | - | - | `UpdateSecurityHubConfigurationResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Updates configuration options for Security Hub CSPM. |
| `UpdateStandardsControl` | `PATCH /standards/control/{StandardsControlArn+}` | - | `StandardsControlArn` | - | `UpdateStandardsControlResponse` | `AccessDeniedException`, `InternalException`, `InvalidAccessException`, `InvalidInputException`, `ResourceNotFoundException` | Used to control whether an individual security standard control is enabled or disabled. Calls to this operation return a `RESOURCE_NOT_FOUND_EXCEPTION` error when the standard subscription for the control has `StandardsControlsUpdatable` value... |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalException` | `structure` | `Code`, `Message` | Internal server error. |
| `InvalidAccessException` | `structure` | `Code`, `Message` | The account doesn't have permission to perform this action. |
| `InvalidInputException` | `structure` | `Code`, `Message` | The request was rejected because you supplied an invalid or out-of-range value for an input parameter. |
| `LimitExceededException` | `structure` | `Code`, `Message` | The request was rejected because it attempted to create resources beyond the current Amazon Web Services account or throttling limits. |
| `ResourceNotFoundException` | `structure` | `Code`, `Message` | The request was rejected because we can't find the specified resource. |
| `AccessDeniedException` | `structure` | `Code`, `Message` | You don't have permission to perform the action specified in the request. |
| `InternalServerException` | `structure` | `Code`, `Message` | The request has failed due to an internal failure of the service. |
| `ThrottlingException` | `structure` | `Code`, `Message` | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | `Code`, `Message` | The request has failed validation because it's missing required fields or has invalid inputs. |
| `ConflictException` | `structure` | `Code`, `Message` | The request causes conflict with the current state of the service resource. |
| `ResourceConflictException` | `structure` | `Code`, `Message` | The resource specified in the request conflicts with an existing resource. |
| `ServiceQuotaExceededException` | `structure` | `Code`, `Message` | The request was rejected because it would exceed the service quota limit. |
| `AcceptAdministratorInvitationRequest` | `structure` | `AdministratorId`, `InvitationId` | - |
| `AcceptAdministratorInvitationResponse` | `structure` | - | - |
| `AcceptInvitationRequest` | `structure` | `InvitationId`, `MasterId` | - |
| `AcceptInvitationResponse` | `structure` | - | - |
| `BatchDeleteAutomationRulesRequest` | `structure` | `AutomationRulesArns` | - |
| `BatchDeleteAutomationRulesResponse` | `structure` | `ProcessedAutomationRules`, `UnprocessedAutomationRules` | - |
| `BatchDisableStandardsRequest` | `structure` | `StandardsSubscriptionArns` | - |
| `BatchDisableStandardsResponse` | `structure` | `StandardsSubscriptions` | - |
| `BatchEnableStandardsRequest` | `structure` | `StandardsSubscriptionRequests` | - |
| `BatchEnableStandardsResponse` | `structure` | `StandardsSubscriptions` | - |
| `BatchGetAutomationRulesRequest` | `structure` | `AutomationRulesArns` | - |
| `BatchGetAutomationRulesResponse` | `structure` | `Rules`, `UnprocessedAutomationRules` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
