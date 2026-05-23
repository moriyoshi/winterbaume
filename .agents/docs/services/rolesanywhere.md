# IAM Roles Anywhere

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Identity and Access Management Roles Anywhere provides a secure way for your workloads such as servers, containers, and applications that run outside of Amazon Web Services to obtain temporary Amazon Web Services credentials. Your workloads can use the same IAM policies and roles you have for native Amazon Web Services applications to access Amazon Web Services resources. Using IAM Roles Anywhere eliminates the need to manage long-term credentials for workloads running outside of Amazon Web Services. To use IAM Roles Anywhere, your workloads must use X.509 certificates issued by their certificate authority (CA). You register the CA with IAM Roles Anywhere as a trust anchor to establish trust between your public key infrastructure (PKI) and IAM Roles Anywhere.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-rolesanywhere/tests/scenario_test.rs`: create a trust anchor and profile with tags, describe/list them, update state, and delete them.
- Backported from `scenario_test.rs`: manage a certificate revocation list tied to a trust anchor ARN.
- Scenario insight from EC2: exercise account or service defaults for IAM Roles Anywhere by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent X.509 based IAM role credential issuance, trust anchors, profiles, subjects, CRLs, tagging, and enable/disable lifecycle state.

## Service Identity and Protocol

- AWS model slug: `rolesanywhere`
- AWS SDK for Rust slug: `rolesanywhere`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/rolesanywhere/service/2018-05-10/rolesanywhere-2018-05-10.json`
- SDK ID: `RolesAnywhere`
- Endpoint prefix: `-`
- ARN namespace: `rolesanywhere`
- CloudFormation name: `RolesAnywhere`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Delete` (4), `Get` (4), `Disable` (3), `Enable` (3), `Update` (3), `Create` (2), `Put` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateProfile`, `CreateTrustAnchor`, `DeleteAttributeMapping`, `DeleteCrl`, `DeleteProfile`, `DeleteTrustAnchor`, `DisableCrl`, `DisableProfile`, `DisableTrustAnchor`, `EnableCrl`, `EnableProfile`, `EnableTrustAnchor`, `ImportCrl`, `PutAttributeMapping`, `PutNotificationSettings`, `TagResource`, `UntagResource`, `UpdateCrl`, `UpdateProfile`, `UpdateTrustAnchor`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCrl`, `GetProfile`, `GetSubject`, `GetTrustAnchor`, `ListCrls`, `ListProfiles`, `ListSubjects`, `ListTagsForResource`, `ListTrustAnchors`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ImportCrl`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 30 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `CloudWatch`, `EventBridge`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Crl` | `crlId` | create: `ImportCrl`; read: `GetCrl`; update: `UpdateCrl`; delete: `DeleteCrl`; list: `ListCrls` | `DisableCrl`, `EnableCrl` | - |
| `Profile` | `profileId` | create: `CreateProfile`; read: `GetProfile`; update: `UpdateProfile`; delete: `DeleteProfile`; list: `ListProfiles` | `DeleteAttributeMapping`, `DisableProfile`, `EnableProfile`, `PutAttributeMapping` | - |
| `Subject` | `subjectId` | read: `GetSubject`; list: `ListSubjects` | - | - |
| `TrustAnchor` | `trustAnchorId` | create: `CreateTrustAnchor`; read: `GetTrustAnchor`; update: `UpdateTrustAnchor`; delete: `DeleteTrustAnchor`; list: `ListTrustAnchors` | `DisableTrustAnchor`, `EnableTrustAnchor` | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/rolesanywhere/latest/userguide/introduction.html
- https://docs.aws.amazon.com/rolesanywhere/latest/userguide/credential-helper.html
- https://docs.aws.amazon.com/rolesanywhere/latest/userguide/authentication-create-session.html

Research outcomes:
- IAM Roles Anywhere exchanges X.509 certificate authentication for temporary AWS credentials usable by workloads outside AWS.
- A trust anchor establishes trust between IAM Roles Anywhere and a certificate authority. It references either AWS Private CA or an external CA certificate.
- Workloads authenticate with certificates issued by a trusted CA associated with the trust anchor.
- IAM roles must trust the IAM Roles Anywhere service principal. The trust anchor is tied to the role through an `aws:SourceArn` condition using the trust anchor ARN.
- Profiles define which IAM roles can be assumed and can include one managed or inline session policy to limit permissions for returned sessions.
- A profile can include many roles, but only one session policy. Any `CreateSession` that references the profile has permissions limited by that session policy.
- IAM Roles Anywhere resources are regional and must be in the same account and Region to be used together.
- The account is the trust boundary. Certificates from any trust anchor in the account can assume any target role in the same account unless role trust policy conditions restrict them.
- The credential helper `credential-process` command returns JSON credentials with access key id, secret access key, session token, and expiration.
- Session duration can range from 900 seconds to 43200 seconds, subject to the expiration rules for the returned credentials.
- The credential helper can use plain private keys, OS certificate stores, PKCS#11 tokens, TPM keys, or a local IMDSv2-compatible credential server.

Parity implications:
- Model trust anchors, profiles, roles, subject/resource mappings, certificates, session policies, sessions, credential expiry, enabled/disabled state, and revocation/import state separately.
- CreateSession should validate same-account/same-Region resource relationships, certificate trust, role trust policy, profile role membership, and session policy intersection.
- Credential outputs should be temporary and expire according to requested duration and certificate/session constraints.

## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutNotificationSettings`
- Common required input members in this group: -

### Reset

- Operations: `ResetNotificationSettings`
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
| `ListTagsForResource` | `GET /ListTagsForResource` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags attached to the resource. Required permissions: rolesanywhere:ListTagsForResource . |
| `PutNotificationSettings` | `PATCH /put-notifications-settings` | - | `trustAnchorId`, `notificationSettings` | - | `PutNotificationSettingsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Attaches a list of notification settings to a trust anchor. A notification setting includes information such as event name, threshold, status of the notification setting, and the channel to notify. Required permissio ... |
| `ResetNotificationSettings` | `PATCH /reset-notifications-settings` | - | `trustAnchorId`, `notificationSettingKeys` | - | `ResetNotificationSettingsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Resets the custom notification setting to IAM Roles Anywhere default setting. Required permissions: rolesanywhere:ResetNotificationSettings . |
| `TagResource` | `POST /TagResource` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Attaches tags to a resource. Required permissions: rolesanywhere:TagResource . |
| `UntagResource` | `POST /UntagResource` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from the resource. Required permissions: rolesanywhere:UntagResource . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListTagsForResource` | - | `resourceArn -> resourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ResourceNotFoundException` | `structure` | message | The resource could not be found. |
| `TooManyTagsException` | `structure` | message | Too many tags. |
| `ValidationException` | `structure` | message | Validation exception error. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `PutNotificationSettingsRequest` | `structure` | trustAnchorId, notificationSettings | - |
| `PutNotificationSettingsResponse` | `structure` | trustAnchor | - |
| `ResetNotificationSettingsRequest` | `structure` | trustAnchorId, notificationSettingKeys | - |
| `ResetNotificationSettingsResponse` | `structure` | trustAnchor | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
