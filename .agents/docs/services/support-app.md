# AWS Support App

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Support App in Slack You can use the Amazon Web Services Support App in Slack API to manage your support cases in Slack for your Amazon Web Services account. After you configure your Slack workspace and channel with the Amazon Web Services Support App, you can perform the following tasks directly in your Slack channel: Create, search, update, and resolve your support cases Request service quota increases for your account Invite Amazon Web Services Support agents to your channel so that you can chat directly about your support cases For more information about how to perform these actions in Slack, see the following documentation in the Amazon Web Services Support User Guide : Amazon Web Services Support App in Slack Joining a live chat session with Amazon Web Services Support Requesting service quota increases Amazon Web Services Support App commands in Slack You can also use the Amazon Web Services Management Console instead of the Amazon Web Services Support App API to manage your Slack configurations. For more information, see Authorize a Slack workspace to enable the Amazon Web Services Support App. You must have a Business or Enterprise Support plan to use the Amazon Web Services Support App API. For more information about the Amazon Web Services Support App endpoints, see the Amazon Web Services Support App in Slack endpoints in the Amazon Web Services...

## Possible Usage Scenarios
- Backported from `crates/winterbaume-supportapp/tests/scenario_test.rs`: manage a Slack workspace and channel lifecycle for Support App integration.
- Backported from `scenario_test.rs`: delete a workspace and verify member channels are removed.
- Backported from `scenario_test.rs`: manage the account alias lifecycle.
- From the AWS documentation and model: represent support-channel configuration for chat platforms, workspace/channel registration, account alias metadata, and cleanup relationships between parent workspaces and channels.

## Service Identity and Protocol

- AWS model slug: `support-app`
- AWS SDK for Rust slug: `supportapp`
- Model version: `2021-08-20`
- Model file: `vendor/api-models-aws/models/support-app/service/2021-08-20/support-app-2021-08-20.json`
- SDK ID: `Support App`
- Endpoint prefix: `-`
- ARN namespace: `supportapp`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (3), `List` (2), `Create` (1), `Get` (1), `Put` (1), `Register` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateSlackChannelConfiguration`, `DeleteAccountAlias`, `DeleteSlackChannelConfiguration`, `DeleteSlackWorkspaceConfiguration`, `PutAccountAlias`, `RegisterSlackWorkspaceForOrganization`, `UpdateSlackChannelConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountAlias`, `ListSlackChannelConfigurations`, `ListSlackWorkspaceConfigurations`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 10 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `STS`.

## Operation Groups

### Delete

- Operations: `DeleteAccountAlias`, `DeleteSlackChannelConfiguration`, `DeleteSlackWorkspaceConfiguration`
- Common required input members in this group: `teamId`

### List

- Operations: `ListSlackChannelConfigurations`, `ListSlackWorkspaceConfigurations`
- Traits: `readonly` (2), `paginated` (2)
- Common required input members in this group: -

### Create

- Operations: `CreateSlackChannelConfiguration`
- Common required input members in this group: -

### Get

- Operations: `GetAccountAlias`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutAccountAlias`
- Common required input members in this group: -

### Register

- Operations: `RegisterSlackWorkspaceForOrganization`
- Common required input members in this group: -

### Update

- Operations: `UpdateSlackChannelConfiguration`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateSlackChannelConfiguration` | `POST /control/create-slack-channel-configuration` | - | `teamId`, `channelId`, `notifyOnCaseSeverity`, `channelRoleArn` | - | `CreateSlackChannelConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a Slack channel configuration for your Amazon Web Services account. You can add up to 5 Slack workspaces for your account. You can add up to 20 Slack channels for your account. A Slack channel can have up to ... |
| `DeleteAccountAlias` | `POST /control/delete-account-alias` | - | - | - | `DeleteAccountAliasResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException` | Deletes an alias for an Amazon Web Services account ID. The alias appears in the Amazon Web Services Support App page of the Amazon Web Services Support Center. The alias also appears in Slack messages from the Amazo ... |
| `DeleteSlackChannelConfiguration` | `POST /control/delete-slack-channel-configuration` | - | `teamId`, `channelId` | - | `DeleteSlackChannelConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a Slack channel configuration from your Amazon Web Services account. This operation doesn't delete your Slack channel. |
| `DeleteSlackWorkspaceConfiguration` | `POST /control/delete-slack-workspace-configuration` | - | `teamId` | - | `DeleteSlackWorkspaceConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a Slack workspace configuration from your Amazon Web Services account. This operation doesn't delete your Slack workspace. |
| `GetAccountAlias` | `POST /control/get-account-alias` | `readonly` | - | - | `GetAccountAliasResult` | `InternalServerException` | Retrieves the alias from an Amazon Web Services account ID. The alias appears in the Amazon Web Services Support App page of the Amazon Web Services Support Center. The alias also appears in Slack messages from the A ... |
| `ListSlackChannelConfigurations` | `POST /control/list-slack-channel-configurations` | `readonly`, `paginated` | - | - | `ListSlackChannelConfigurationsResult` | `AccessDeniedException`, `InternalServerException` | Lists the Slack channel configurations for an Amazon Web Services account. |
| `ListSlackWorkspaceConfigurations` | `POST /control/list-slack-workspace-configurations` | `readonly`, `paginated` | - | - | `ListSlackWorkspaceConfigurationsResult` | `AccessDeniedException`, `InternalServerException` | Lists the Slack workspace configurations for an Amazon Web Services account. |
| `PutAccountAlias` | `POST /control/put-account-alias` | - | `accountAlias` | - | `PutAccountAliasResult` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Creates or updates an individual alias for each Amazon Web Services account ID. The alias appears in the Amazon Web Services Support App page of the Amazon Web Services Support Center. The alias also appears in Slack ... |
| `RegisterSlackWorkspaceForOrganization` | `POST /control/register-slack-workspace-for-organization` | - | `teamId` | - | `RegisterSlackWorkspaceForOrganizationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Registers a Slack workspace for your Amazon Web Services account. To call this API, your account must be part of an organization in Organizations. If you're the management account and you want to register Slack works ... |
| `UpdateSlackChannelConfiguration` | `POST /control/update-slack-channel-configuration` | - | `teamId`, `channelId` | - | `UpdateSlackChannelConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the configuration for a Slack channel, such as case update notifications. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient permission to perform this action. |
| `ConflictException` | `structure` | message | Your request has a conflict. For example, you might receive this error if you try the following: Add, update, or delete a Slack channel configuration before ... |
| `InternalServerException` | `structure` | message | We can’t process your request right now because of a server issue. Try again later. |
| `ResourceNotFoundException` | `structure` | message | The specified resource is missing or doesn't exist, such as an account alias, Slack channel configuration, or Slack workspace configuration. |
| `ServiceQuotaExceededException` | `structure` | message | Your Service Quotas request exceeds the quota for the service. For example, your Service Quotas request to Amazon Web Services Support App might exceed the ... |
| `ValidationException` | `structure` | message | Your request input doesn't meet the constraints that the Amazon Web Services Support App specifies. |
| `CreateSlackChannelConfigurationRequest` | `structure` | teamId, channelId, channelName, notifyOnCreateOrReopenCase, notifyOnAddCorrespondenceToCase, notifyOnResolveCase, notifyOnCaseSeverity, channelRoleArn | - |
| `CreateSlackChannelConfigurationResult` | `structure` | **empty (no members)** | - |
| `DeleteAccountAliasRequest` | `structure` | **empty (no members)** | - |
| `DeleteAccountAliasResult` | `structure` | **empty (no members)** | - |
| `DeleteSlackChannelConfigurationRequest` | `structure` | teamId, channelId | - |
| `DeleteSlackChannelConfigurationResult` | `structure` | **empty (no members)** | - |
| `DeleteSlackWorkspaceConfigurationRequest` | `structure` | teamId | - |
| `DeleteSlackWorkspaceConfigurationResult` | `structure` | **empty (no members)** | - |
| `GetAccountAliasRequest` | `structure` | **empty (no members)** | - |
| `GetAccountAliasResult` | `structure` | accountAlias | - |
| `ListSlackChannelConfigurationsRequest` | `structure` | nextToken | - |
| `ListSlackChannelConfigurationsResult` | `structure` | nextToken, slackChannelConfigurations | - |
| `ListSlackWorkspaceConfigurationsRequest` | `structure` | nextToken | - |
| `ListSlackWorkspaceConfigurationsResult` | `structure` | nextToken, slackWorkspaceConfigurations | - |
| `PutAccountAliasRequest` | `structure` | accountAlias | - |
| `PutAccountAliasResult` | `structure` | **empty (no members)** | - |
| `RegisterSlackWorkspaceForOrganizationRequest` | `structure` | teamId | - |
| `RegisterSlackWorkspaceForOrganizationResult` | `structure` | teamId, teamName, accountType | - |
| `UpdateSlackChannelConfigurationRequest` | `structure` | teamId, channelId, channelName, notifyOnCreateOrReopenCase, notifyOnAddCorrespondenceToCase, notifyOnResolveCase, notifyOnCaseSeverity, channelRoleArn | - |
| `UpdateSlackChannelConfigurationResult` | `structure` | teamId, channelId, channelName, notifyOnCreateOrReopenCase, notifyOnAddCorrespondenceToCase, notifyOnResolveCase, notifyOnCaseSeverity, channelRoleArn | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
