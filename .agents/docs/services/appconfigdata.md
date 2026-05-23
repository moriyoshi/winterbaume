# AWS AppConfig Data

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AppConfig Data provides the data plane APIs your application uses to retrieve configuration data. Here's how it works: Your application retrieves configuration data by first establishing a configuration session using the AppConfig Data StartConfigurationSession API action. Your session's client then makes periodic calls to GetLatestConfiguration to check for and retrieve the latest data available. When calling `StartConfigurationSession`, your code sends the following information: Identifiers (ID or name) of an AppConfig application, environment, and configuration profile that the session tracks. (Optional) The minimum amount of time the session's client must wait between calls to `GetLatestConfiguration`.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS AppConfig Data resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: start configuration sessions and retrieve latest deployed configuration data for application clients.
- From the operation surface: model polling tokens, configuration version delivery, cache-aware client behaviour, and runtime consumption of AppConfig deployments.

## Service Identity and Protocol

- AWS model slug: `appconfigdata`
- AWS SDK for Rust slug: `appconfigdata`
- Model version: `2021-11-11`
- Model file: `vendor/api-models-aws/models/appconfigdata/service/2021-11-11/appconfigdata-2021-11-11.json`
- SDK ID: `AppConfigData`
- Endpoint prefix: `appconfigdata`
- ARN namespace: `appconfigdata`
- CloudFormation name: `AppConfigData`
- CloudTrail event source: `appconfigdata.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartConfigurationSession`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetLatestConfiguration`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartConfigurationSession`.
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ConfigurationSession` | `SessionId` | create: `StartConfigurationSession` | - | - |
## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/appconfig/2019-10-09/APIReference/API_appconfigdata_StartConfigurationSession.html
- https://docs.aws.amazon.com/appconfig/2019-10-09/APIReference/API_appconfigdata_GetLatestConfiguration.html
- https://docs.aws.amazon.com/appconfig/latest/userguide/retrieving-feature-flags.html

Research outcomes:
- StartConfigurationSession creates a retrieval session for a deployed configuration using application, environment, and configuration profile identifiers.
- RequiredMinimumPollIntervalInSeconds constrains how frequently the client can call GetLatestConfiguration for that session. Valid values are 15 to 86400 seconds.
- StartConfigurationSession returns an InitialConfigurationToken that must be used only for the first GetLatestConfiguration call.
- Every GetLatestConfiguration response returns a NextPollConfigurationToken. The client must use the new token on the next call.
- Initial and next-poll tokens are single-use and valid for up to 24 hours. Expired-token use returns BadRequestException.
- GetLatestConfiguration can return an empty body when the client already has the latest configuration.
- Next-Poll-Interval-In-Seconds tells the client how long to wait before polling again; the default is 60 seconds unless the session requested a stricter interval.
- GetLatestConfiguration is a priced call, and AWS recommends local caching through AppConfig Agent or extensions where possible.

Parity implications:
- Model configuration sessions, single-use poll tokens, token expiry, minimum poll interval, last-delivered version, and empty-response semantics.
- GetLatestConfiguration should rotate tokens on every successful call and reject reused, expired, or too-frequent tokens.
- Retrieval should return deployed configuration state, not draft profile data.

## Control-Plane / Data-Plane Coherence

- **Paired with `appconfig`.** This data plane reads the configuration that the AppConfig control plane ( `winterbaume-appconfig` ) deploys: `StartConfigurationSession` references a `(application, environment, configuration_profile)` tuple owned by `winterbaume-appconfig`, and `GetLatestConfiguration` should return the active deployment's content for that environment+profile.
- **Current Winterbaume status: divergent.** `winterbaume-appconfigdata` does not depend on `winterbaume-appconfig`; it holds only opaque session tokens and never looks up the deployed configuration. As a result `GetLatestConfiguration` returns an empty payload regardless of what was deployed via `winterbaume-appconfig`'s `StartDeployment`.
- **What needs to change:** this crate should observe `winterbaume-appconfig`'s deployment state ( latest active deployment per environment, including the resolved configuration content and version label ) so `GetLatestConfiguration` returns what was actually deployed. The bullet just above ( "Retrieval should return deployed configuration state, not draft profile data" ) is the parity outcome this gap blocks.

## Operation Groups

### Get

- Operations: `GetLatestConfiguration`
- Traits: `readonly` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetLatestConfiguration` | `GET /configuration` | `readonly` | `ConfigurationToken` | - | `GetLatestConfigurationResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the latest deployed configuration. This API may return empty configuration data if the client already has the latest version. For more information about this API action and to view example CLI commands that ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetLatestConfiguration` | - | `ConfigurationToken -> configuration_token` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Message, Reason, Details | The input fails to satisfy the constraints specified by the service. |
| `InternalServerException` | `structure` | Message | There was an internal failure in the service. |
| `ResourceNotFoundException` | `structure` | Message, ResourceType, ReferencedBy | The requested resource could not be found. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `GetLatestConfigurationRequest` | `structure` | ConfigurationToken | - |
| `GetLatestConfigurationResponse` | `structure` | NextPollConfigurationToken, NextPollIntervalInSeconds, ContentType, Configuration, VersionLabel | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
