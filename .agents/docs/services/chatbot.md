# AWS Chatbot

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The AWS Chatbot API Reference provides descriptions, API request parameters, and the XML response for each of the AWS Chatbot API actions. AWS Chatbot APIs are currently available in the following Regions: US East (Ohio) - `us-east-2` US West (Oregon) - `us-west-2` Asia Pacific (Singapore) - `ap-southeast-1` Europe (Ireland) - `eu-west-1` The AWS Chatbot console can only be used in US East (Ohio). Your configuration data however, is stored in each of the relevant available Regions. Your AWS CloudTrail events are logged in whatever Region you call from, not US East (N. Virginia) by default.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Chatbot where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Chatbot by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Chatbot by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: configure chat clients, Slack channels, Microsoft Teams channels, custom actions, and account preferences for operational notifications.
- From the operation surface: support incident/chatops integration, notification channel lifecycle, guardrail policies, tagging, and deletion of chat configurations.

## Service Identity and Protocol

- AWS model slug: `chatbot`
- AWS SDK for Rust slug: `chatbot`
- Model version: `2017-10-11`
- Model file: `vendor/api-models-aws/models/chatbot/service/2017-10-11/chatbot-2017-10-11.json`
- SDK ID: `chatbot`
- Endpoint prefix: `chatbot`
- ARN namespace: `chatbot`
- CloudFormation name: `Chatbot`
- CloudTrail event source: `chatbot.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (8), `List` (6), `Update` (5), `Create` (4), `Describe` (4), `Get` (3), `Associate` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateToConfiguration`, `CreateChimeWebhookConfiguration`, `CreateCustomAction`, `CreateMicrosoftTeamsChannelConfiguration`, `CreateSlackChannelConfiguration`, `DeleteChimeWebhookConfiguration`, `DeleteCustomAction`, `DeleteMicrosoftTeamsChannelConfiguration`, `DeleteMicrosoftTeamsConfiguredTeam`, `DeleteMicrosoftTeamsUserIdentity`, `DeleteSlackChannelConfiguration`, `DeleteSlackUserIdentity`, `DeleteSlackWorkspaceAuthorization`, `DisassociateFromConfiguration`, `TagResource`, `UntagResource`, `UpdateAccountPreferences`, `UpdateChimeWebhookConfiguration`, `UpdateCustomAction`, `UpdateMicrosoftTeamsChannelConfiguration`, ... (+1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeChimeWebhookConfigurations`, `DescribeSlackChannelConfigurations`, `DescribeSlackUserIdentities`, `DescribeSlackWorkspaces`, `GetAccountPreferences`, `GetCustomAction`, `GetMicrosoftTeamsChannelConfiguration`, `ListAssociations`, `ListCustomActions`, `ListMicrosoftTeamsChannelConfigurations`, `ListMicrosoftTeamsConfiguredTeams`, `ListMicrosoftTeamsUserIdentities`, `ListTagsForResource`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 33 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CustomActionResource` | `CustomActionArn` | create: `CreateCustomAction`; read: `GetCustomAction`; update: `UpdateCustomAction`; delete: `DeleteCustomAction`; list: `ListCustomActions` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/chatbot/latest/adminguide/what-is.html
- https://docs.aws.amazon.com/chatbot/latest/adminguide/create-eventbridge-rule.html

Research outcomes:
- Amazon Q Developer in chat applications, formerly AWS Chatbot, connects AWS notifications and commands to chat channels such as Slack and Microsoft Teams.
- Chat configurations map chat workspaces/channels to IAM roles, guardrail policies, SNS topics, and logging settings.
- EventBridge and SNS can deliver AWS service notifications into configured chat channels.
- Channel roles and guardrail policies limit which AWS CLI-style commands users can run from chat.
- Chat channel configuration is distinct from the SNS/EventBridge events that trigger notifications.

Parity implications:
- Model chat clients, Slack/Teams workspaces, channel configurations, IAM roles, guardrail policies, SNS topic associations, logging, and command permissions separately.
- Notification delivery should depend on target topic/event rules and channel configuration.
- Command execution should evaluate chat user, channel role, and guardrail policy.

## Operation Groups

### Delete

- Operations: `DeleteChimeWebhookConfiguration`, `DeleteCustomAction`, `DeleteMicrosoftTeamsChannelConfiguration`, `DeleteMicrosoftTeamsConfiguredTeam`, `DeleteMicrosoftTeamsUserIdentity`, `DeleteSlackChannelConfiguration`, `DeleteSlackUserIdentity`, `DeleteSlackWorkspaceAuthorization`
- Traits: `idempotent` (1)
- Common required input members in this group: `ChatConfigurationArn`, `CustomActionArn`, `SlackTeamId`, `SlackUserId`, `TeamId`, `UserId`

### List

- Operations: `ListAssociations`, `ListCustomActions`, `ListMicrosoftTeamsChannelConfigurations`, `ListMicrosoftTeamsConfiguredTeams`, `ListMicrosoftTeamsUserIdentities`, `ListTagsForResource`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `ChatConfiguration`, `ResourceARN`

### Update

- Operations: `UpdateAccountPreferences`, `UpdateChimeWebhookConfiguration`, `UpdateCustomAction`, `UpdateMicrosoftTeamsChannelConfiguration`, `UpdateSlackChannelConfiguration`
- Common required input members in this group: `ChannelId`, `ChatConfigurationArn`, `CustomActionArn`, `Definition`, `SlackChannelId`

### Create

- Operations: `CreateChimeWebhookConfiguration`, `CreateCustomAction`, `CreateMicrosoftTeamsChannelConfiguration`, `CreateSlackChannelConfiguration`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `ActionName`, `ChannelId`, `ConfigurationName`, `Definition`, `IamRoleArn`, `SlackChannelId`, `SlackTeamId`, `SnsTopicArns`, `TeamId`, `TenantId`, `WebhookDescription`, `WebhookUrl`

### Describe

- Operations: `DescribeChimeWebhookConfigurations`, `DescribeSlackChannelConfigurations`, `DescribeSlackUserIdentities`, `DescribeSlackWorkspaces`
- Traits: `paginated` (4), `readonly` (4)

### Get

- Operations: `GetAccountPreferences`, `GetCustomAction`, `GetMicrosoftTeamsChannelConfiguration`
- Traits: `readonly` (3)
- Common required input members in this group: `ChatConfigurationArn`, `CustomActionArn`

### Associate

- Operations: `AssociateToConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `ChatConfiguration`, `Resource`

### Disassociate

- Operations: `DisassociateFromConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `ChatConfiguration`, `Resource`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateToConfiguration` | `POST /associate-to-configuration` | `idempotent` | `ChatConfiguration`, `Resource` | - | `AssociateToConfigurationResult` | `InternalServiceError`, `InvalidRequestException`, `UnauthorizedException` | Links a resource (for example, a custom action) to a channel configuration. |
| `CreateChimeWebhookConfiguration` | `POST /create-chime-webhook-configuration` | - | `ConfigurationName`, `IamRoleArn`, `SnsTopicArns`, `WebhookDescription`, `WebhookUrl` | - | `CreateChimeWebhookConfigurationResult` | `ConflictException`, `CreateChimeWebhookConfigurationException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException` | Creates an AWS Chatbot configuration for Amazon Chime. |
| `CreateCustomAction` | `POST /create-custom-action` | `idempotent`, `idempotency-token` | `ActionName`, `Definition` | `ClientToken` | `CreateCustomActionResult` | `ConflictException`, `InternalServiceError`, `InvalidRequestException`, `LimitExceededException`, `UnauthorizedException` | Creates a custom action that can be invoked as an alias or as a button on a notification. |
| `CreateMicrosoftTeamsChannelConfiguration` | `POST /create-ms-teams-channel-configuration` | - | `ChannelId`, `ConfigurationName`, `IamRoleArn`, `TeamId`, `TenantId` | - | `CreateTeamsChannelConfigurationResult` | `ConflictException`, `CreateTeamsChannelConfigurationException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException` | Creates an AWS Chatbot configuration for Microsoft Teams. |
| `CreateSlackChannelConfiguration` | `POST /create-slack-channel-configuration` | - | `ConfigurationName`, `IamRoleArn`, `SlackChannelId`, `SlackTeamId` | - | `CreateSlackChannelConfigurationResult` | `ConflictException`, `CreateSlackChannelConfigurationException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException` | Creates an AWS Chatbot confugration for Slack. |
| `DeleteChimeWebhookConfiguration` | `POST /delete-chime-webhook-configuration` | - | `ChatConfigurationArn` | - | `DeleteChimeWebhookConfigurationResult` | `DeleteChimeWebhookConfigurationException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Deletes a Amazon Chime webhook configuration for AWS Chatbot. |
| `DeleteCustomAction` | `POST /delete-custom-action` | `idempotent` | `CustomActionArn` | - | `DeleteCustomActionResult` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `UnauthorizedException` | Deletes a custom action. |
| `DeleteMicrosoftTeamsChannelConfiguration` | `POST /delete-ms-teams-channel-configuration` | - | `ChatConfigurationArn` | - | `DeleteTeamsChannelConfigurationResult` | `DeleteTeamsChannelConfigurationException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Deletes a Microsoft Teams channel configuration for AWS Chatbot |
| `DeleteMicrosoftTeamsConfiguredTeam` | `POST /delete-ms-teams-configured-teams` | - | `TeamId` | - | `DeleteTeamsConfiguredTeamResult` | `DeleteTeamsConfiguredTeamException`, `InvalidParameterException` | Deletes the Microsoft Teams team authorization allowing for channels to be configured in that Microsoft Teams team. Note that the Microsoft Teams team must have no channels configured to remove it. |
| `DeleteMicrosoftTeamsUserIdentity` | `POST /delete-ms-teams-user-identity` | - | `ChatConfigurationArn`, `UserId` | - | `DeleteMicrosoftTeamsUserIdentityResult` | `DeleteMicrosoftTeamsUserIdentityException`, `InvalidParameterException`, `ResourceNotFoundException` | Identifes a user level permission for a channel configuration. |
| `DeleteSlackChannelConfiguration` | `POST /delete-slack-channel-configuration` | - | `ChatConfigurationArn` | - | `DeleteSlackChannelConfigurationResult` | `DeleteSlackChannelConfigurationException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Deletes a Slack channel configuration for AWS Chatbot |
| `DeleteSlackUserIdentity` | `POST /delete-slack-user-identity` | - | `ChatConfigurationArn`, `SlackTeamId`, `SlackUserId` | - | `DeleteSlackUserIdentityResult` | `DeleteSlackUserIdentityException`, `InvalidParameterException`, `ResourceNotFoundException` | Deletes a user level permission for a Slack channel configuration. |
| `DeleteSlackWorkspaceAuthorization` | `POST /delete-slack-workspace-authorization` | - | `SlackTeamId` | - | `DeleteSlackWorkspaceAuthorizationResult` | `DeleteSlackWorkspaceAuthorizationFault`, `InvalidParameterException` | Deletes the Slack workspace authorization that allows channels to be configured in that workspace. This requires all configured channels in the workspace to be deleted. |
| `DescribeChimeWebhookConfigurations` | `POST /describe-chime-webhook-configurations` | `readonly`, `paginated` | - | - | `DescribeChimeWebhookConfigurationsResult` | `DescribeChimeWebhookConfigurationsException`, `InvalidParameterException`, `InvalidRequestException` | Lists Amazon Chime webhook configurations optionally filtered by ChatConfigurationArn |
| `DescribeSlackChannelConfigurations` | `POST /describe-slack-channel-configurations` | `readonly`, `paginated` | - | - | `DescribeSlackChannelConfigurationsResult` | `DescribeSlackChannelConfigurationsException`, `InvalidParameterException`, `InvalidRequestException` | Lists Slack channel configurations optionally filtered by ChatConfigurationArn |
| `DescribeSlackUserIdentities` | `POST /describe-slack-user-identities` | `readonly`, `paginated` | - | - | `DescribeSlackUserIdentitiesResult` | `DescribeSlackUserIdentitiesException`, `InvalidParameterException`, `InvalidRequestException` | Lists all Slack user identities with a mapped role. |
| `DescribeSlackWorkspaces` | `POST /describe-slack-workspaces` | `readonly`, `paginated` | - | - | `DescribeSlackWorkspacesResult` | `DescribeSlackWorkspacesException`, `InvalidParameterException`, `InvalidRequestException` | List all authorized Slack workspaces connected to the AWS Account onboarded with AWS Chatbot. |
| `DisassociateFromConfiguration` | `POST /disassociate-from-configuration` | `idempotent` | `ChatConfiguration`, `Resource` | - | `DisassociateFromConfigurationResult` | `InternalServiceError`, `InvalidRequestException`, `UnauthorizedException` | Unlink a resource, for example a custom action, from a channel configuration. |
| `GetAccountPreferences` | `POST /get-account-preferences` | `readonly` | - | - | `GetAccountPreferencesResult` | `GetAccountPreferencesException`, `InvalidRequestException` | Returns AWS Chatbot account preferences. |
| `GetCustomAction` | `POST /get-custom-action` | `readonly` | `CustomActionArn` | - | `GetCustomActionResult` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `UnauthorizedException` | Returns a custom action. |
| `GetMicrosoftTeamsChannelConfiguration` | `POST /get-ms-teams-channel-configuration` | `readonly` | `ChatConfigurationArn` | - | `GetTeamsChannelConfigurationResult` | `GetTeamsChannelConfigurationException`, `InvalidParameterException`, `InvalidRequestException` | Returns a Microsoft Teams channel configuration in an AWS account. |
| `ListAssociations` | `POST /list-associations` | `readonly`, `paginated` | `ChatConfiguration` | - | `ListAssociationsResult` | - | Lists resources associated with a channel configuration. |
| `ListCustomActions` | `POST /list-custom-actions` | `readonly`, `paginated` | - | - | `ListCustomActionsResult` | `InternalServiceError`, `InvalidRequestException`, `UnauthorizedException` | Lists custom actions defined in this account. |
| `ListMicrosoftTeamsChannelConfigurations` | `POST /list-ms-teams-channel-configurations` | `readonly`, `paginated` | - | - | `ListTeamsChannelConfigurationsResult` | `InvalidParameterException`, `InvalidRequestException`, `ListTeamsChannelConfigurationsException` | Lists all AWS Chatbot Microsoft Teams channel configurations in an AWS account. |
| `ListMicrosoftTeamsConfiguredTeams` | `POST /list-ms-teams-configured-teams` | `readonly`, `paginated` | - | - | `ListMicrosoftTeamsConfiguredTeamsResult` | `InvalidParameterException`, `InvalidRequestException`, `ListMicrosoftTeamsConfiguredTeamsException` | Lists all authorized Microsoft Teams for an AWS Account |
| `ListMicrosoftTeamsUserIdentities` | `POST /list-ms-teams-user-identities` | `readonly`, `paginated` | - | - | `ListMicrosoftTeamsUserIdentitiesResult` | `InvalidParameterException`, `InvalidRequestException`, `ListMicrosoftTeamsUserIdentitiesException` | A list all Microsoft Teams user identities with a mapped role. |
| `ListTagsForResource` | `POST /list-tags-for-resource` | `readonly` | `ResourceARN` | - | `ListTagsForResourceResponse` | `InternalServiceError`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists all of the tags associated with the Amazon Resource Name (ARN) that you specify. The resource can be a user, server, or role. |
| `TagResource` | `POST /tag-resource` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InternalServiceError`, `ResourceNotFoundException`, `ServiceUnavailableException`, `TooManyTagsException` | Attaches a key-value pair to a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities. |
| `UntagResource` | `POST /untag-resource` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InternalServiceError`, `ResourceNotFoundException`, `ServiceUnavailableException` | Detaches a key-value pair from a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities. |
| `UpdateAccountPreferences` | `POST /update-account-preferences` | - | - | - | `UpdateAccountPreferencesResult` | `InvalidParameterException`, `InvalidRequestException`, `UpdateAccountPreferencesException` | Updates AWS Chatbot account preferences. |
| `UpdateChimeWebhookConfiguration` | `POST /update-chime-webhook-configuration` | - | `ChatConfigurationArn` | - | `UpdateChimeWebhookConfigurationResult` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `UpdateChimeWebhookConfigurationException` | Updates a Amazon Chime webhook configuration. |
| `UpdateCustomAction` | `POST /update-custom-action` | - | `CustomActionArn`, `Definition` | - | `UpdateCustomActionResult` | `InternalServiceError`, `InvalidRequestException`, `ResourceNotFoundException`, `UnauthorizedException` | Updates a custom action. |
| `UpdateMicrosoftTeamsChannelConfiguration` | `POST /update-ms-teams-channel-configuration` | - | `ChannelId`, `ChatConfigurationArn` | - | `UpdateTeamsChannelConfigurationResult` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `UpdateTeamsChannelConfigurationException` | Updates an Microsoft Teams channel configuration. |
| `UpdateSlackChannelConfiguration` | `POST /update-slack-channel-configuration` | - | `ChatConfigurationArn`, `SlackChannelId` | - | `UpdateSlackChannelConfigurationResult` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `UpdateSlackChannelConfigurationException` | Updates a Slack channel configuration. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidRequestException` | `structure` | `message` | Your request input doesn't meet the constraints required by AWS Chatbot. |
| `InvalidParameterException` | `structure` | `message` | Your request input doesn't meet the constraints required by AWS Chatbot. |
| `ResourceNotFoundException` | `structure` | `Message` | We were unable to find the resource for your request |
| `InternalServiceError` | `structure` | `Message` | Unexpected error during processing of request. |
| `UnauthorizedException` | `structure` | `message` | The request was rejected because it doesn't have valid credentials for the target resource. |
| `ConflictException` | `structure` | `message` | There was an issue processing your request. |
| `LimitExceededException` | `structure` | `message` | You have exceeded a service limit for AWS Chatbot. |
| `ServiceUnavailableException` | `structure` | `message` | We can’t process your request right now because of a server issue. |
| `AssociateToConfigurationRequest` | `structure` | `ChatConfiguration`, `Resource` | - |
| `AssociateToConfigurationResult` | `structure` | - | - |
| `CreateChimeWebhookConfigurationRequest` | `structure` | `ConfigurationName`, `IamRoleArn`, `LoggingLevel`, `SnsTopicArns`, `Tags`, `WebhookDescription`, `WebhookUrl` | - |
| `CreateChimeWebhookConfigurationResult` | `structure` | `WebhookConfiguration` | - |
| `CreateChimeWebhookConfigurationException` | `structure` | `Message` | We can’t process your request right now because of a server issue. |
| `CreateCustomActionRequest` | `structure` | `ActionName`, `AliasName`, `Attachments`, `ClientToken`, `Definition`, `Tags` | - |
| `CreateCustomActionResult` | `structure` | `CustomActionArn` | - |
| `CreateTeamsChannelConfigurationRequest` | `structure` | `ChannelId`, `ChannelName`, `ConfigurationName`, `GuardrailPolicyArns`, `IamRoleArn`, `LoggingLevel`, `SnsTopicArns`, `Tags`, `TeamId`, `TeamName`, `TenantId`, `UserAuthorizationRequired` | - |
| `CreateTeamsChannelConfigurationResult` | `structure` | `ChannelConfiguration` | - |
| `CreateTeamsChannelConfigurationException` | `structure` | `Message` | We can’t process your request right now because of a server issue. |
| `CreateSlackChannelConfigurationRequest` | `structure` | `ConfigurationName`, `GuardrailPolicyArns`, `IamRoleArn`, `LoggingLevel`, `SlackChannelId`, `SlackChannelName`, `SlackTeamId`, `SnsTopicArns`, `Tags`, `UserAuthorizationRequired` | - |
| `CreateSlackChannelConfigurationResult` | `structure` | `ChannelConfiguration` | - |
| `CreateSlackChannelConfigurationException` | `structure` | `Message` | We can’t process your request right now because of a server issue. |
| `DeleteChimeWebhookConfigurationRequest` | `structure` | `ChatConfigurationArn` | - |
| `DeleteChimeWebhookConfigurationResult` | `structure` | - | - |
| `DeleteChimeWebhookConfigurationException` | `structure` | `Message` | We can’t process your request right now because of a server issue. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
