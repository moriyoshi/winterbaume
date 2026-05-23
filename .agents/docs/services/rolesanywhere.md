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

- Operations: `ListCrls`, `ListProfiles`, `ListSubjects`, `ListTagsForResource`, `ListTrustAnchors`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `resourceArn`

### Delete

- Operations: `DeleteAttributeMapping`, `DeleteCrl`, `DeleteProfile`, `DeleteTrustAnchor`
- Traits: `idempotent` (4)
- Common required input members in this group: `certificateField`, `crlId`, `profileId`, `trustAnchorId`

### Get

- Operations: `GetCrl`, `GetProfile`, `GetSubject`, `GetTrustAnchor`
- Traits: `readonly` (4)
- Common required input members in this group: `crlId`, `profileId`, `subjectId`, `trustAnchorId`

### Disable

- Operations: `DisableCrl`, `DisableProfile`, `DisableTrustAnchor`
- Common required input members in this group: `crlId`, `profileId`, `trustAnchorId`

### Enable

- Operations: `EnableCrl`, `EnableProfile`, `EnableTrustAnchor`
- Common required input members in this group: `crlId`, `profileId`, `trustAnchorId`

### Update

- Operations: `UpdateCrl`, `UpdateProfile`, `UpdateTrustAnchor`
- Traits: `idempotent` (2)
- Common required input members in this group: `crlId`, `profileId`, `trustAnchorId`

### Create

- Operations: `CreateProfile`, `CreateTrustAnchor`
- Common required input members in this group: `name`, `roleArns`, `source`

### Put

- Operations: `PutAttributeMapping`, `PutNotificationSettings`
- Traits: `idempotent` (1)
- Common required input members in this group: `certificateField`, `mappingRules`, `notificationSettings`, `profileId`, `trustAnchorId`

### Import

- Operations: `ImportCrl`
- Common required input members in this group: `crlData`, `name`, `trustAnchorArn`

### Reset

- Operations: `ResetNotificationSettings`
- Common required input members in this group: `notificationSettingKeys`, `trustAnchorId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateProfile` | `POST /profiles` | - | `name`, `roleArns` | - | `ProfileDetailResponse` | `AccessDeniedException`, `ValidationException` | Creates a profile , a list of the roles that Roles Anywhere service is trusted to assume. You use profiles to intersect permissions with IAM managed policies. |
| `CreateTrustAnchor` | `POST /trustanchors` | - | `name`, `source` | - | `TrustAnchorDetailResponse` | `AccessDeniedException`, `ValidationException` | Creates a trust anchor to establish trust between IAM Roles Anywhere and your certificate authority (CA). You can define a trust anchor as a reference to an Private Certificate Authority (Private CA) or by uploading a CA certificate. |
| `DeleteAttributeMapping` | `DELETE /profiles/{profileId}/mappings` | `idempotent` | `certificateField`, `profileId` | - | `DeleteAttributeMappingResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Delete an entry from the attribute mapping rules enforced by a given profile. |
| `DeleteCrl` | `DELETE /crl/{crlId}` | `idempotent` | `crlId` | - | `CrlDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Deletes a certificate revocation list (CRL). Required permissions: `rolesanywhere:DeleteCrl`. |
| `DeleteProfile` | `DELETE /profile/{profileId}` | `idempotent` | `profileId` | - | `ProfileDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Deletes a profile. Required permissions: `rolesanywhere:DeleteProfile`. |
| `DeleteTrustAnchor` | `DELETE /trustanchor/{trustAnchorId}` | `idempotent` | `trustAnchorId` | - | `TrustAnchorDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Deletes a trust anchor. Required permissions: `rolesanywhere:DeleteTrustAnchor`. |
| `DisableCrl` | `POST /crl/{crlId}/disable` | - | `crlId` | - | `CrlDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Disables a certificate revocation list (CRL). Required permissions: `rolesanywhere:DisableCrl`. |
| `DisableProfile` | `POST /profile/{profileId}/disable` | - | `profileId` | - | `ProfileDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Disables a profile. When disabled, temporary credential requests with this profile fail. |
| `DisableTrustAnchor` | `POST /trustanchor/{trustAnchorId}/disable` | - | `trustAnchorId` | - | `TrustAnchorDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Disables a trust anchor. When disabled, temporary credential requests specifying this trust anchor are unauthorized. |
| `EnableCrl` | `POST /crl/{crlId}/enable` | - | `crlId` | - | `CrlDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Enables a certificate revocation list (CRL). When enabled, certificates stored in the CRL are unauthorized to receive session credentials. |
| `EnableProfile` | `POST /profile/{profileId}/enable` | - | `profileId` | - | `ProfileDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Enables temporary credential requests for a profile. Required permissions: `rolesanywhere:EnableProfile`. |
| `EnableTrustAnchor` | `POST /trustanchor/{trustAnchorId}/enable` | - | `trustAnchorId` | - | `TrustAnchorDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Enables a trust anchor. When enabled, certificates in the trust anchor chain are authorized for trust validation. |
| `GetCrl` | `GET /crl/{crlId}` | `readonly` | `crlId` | - | `CrlDetailResponse` | `ResourceNotFoundException` | Gets a certificate revocation list (CRL). Required permissions: `rolesanywhere:GetCrl`. |
| `GetProfile` | `GET /profile/{profileId}` | `readonly` | `profileId` | - | `ProfileDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Gets a profile. Required permissions: `rolesanywhere:GetProfile`. |
| `GetSubject` | `GET /subject/{subjectId}` | `readonly` | `subjectId` | - | `SubjectDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Gets a subject , which associates a certificate identity with authentication attempts. The subject stores auditing information such as the status of the last authentication attempt, the certificate data used in the attempt, and the last time the associated... |
| `GetTrustAnchor` | `GET /trustanchor/{trustAnchorId}` | `readonly` | `trustAnchorId` | - | `TrustAnchorDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets a trust anchor. Required permissions: `rolesanywhere:GetTrustAnchor`. |
| `ImportCrl` | `POST /crls` | - | `crlData`, `name`, `trustAnchorArn` | - | `CrlDetailResponse` | `AccessDeniedException`, `ValidationException` | Imports the certificate revocation list (CRL). A CRL is a list of certificates that have been revoked by the issuing certificate Authority (CA).In order to be properly imported, a CRL must be in PEM format. |
| `ListCrls` | `GET /crls` | `readonly`, `paginated` | - | - | `ListCrlsResponse` | `AccessDeniedException`, `ValidationException` | Lists all certificate revocation lists (CRL) in the authenticated account and Amazon Web Services Region. Required permissions: `rolesanywhere:ListCrls`. |
| `ListProfiles` | `GET /profiles` | `readonly`, `paginated` | - | - | `ListProfilesResponse` | `AccessDeniedException`, `ValidationException` | Lists all profiles in the authenticated account and Amazon Web Services Region. Required permissions: `rolesanywhere:ListProfiles`. |
| `ListSubjects` | `GET /subjects` | `readonly`, `paginated` | - | - | `ListSubjectsResponse` | `AccessDeniedException`, `ValidationException` | Lists the subjects in the authenticated account and Amazon Web Services Region. Required permissions: `rolesanywhere:ListSubjects`. |
| `ListTagsForResource` | `GET /ListTagsForResource` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags attached to the resource. Required permissions: `rolesanywhere:ListTagsForResource`. |
| `ListTrustAnchors` | `GET /trustanchors` | `readonly`, `paginated` | - | - | `ListTrustAnchorsResponse` | `AccessDeniedException`, `ValidationException` | Lists the trust anchors in the authenticated account and Amazon Web Services Region. Required permissions: `rolesanywhere:ListTrustAnchors`. |
| `PutAttributeMapping` | `PUT /profiles/{profileId}/mappings` | `idempotent` | `certificateField`, `mappingRules`, `profileId` | - | `PutAttributeMappingResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Put an entry in the attribute mapping rules that will be enforced by a given profile. A mapping specifies a certificate field and one or more specifiers that have contextual meanings. |
| `PutNotificationSettings` | `PATCH /put-notifications-settings` | - | `notificationSettings`, `trustAnchorId` | - | `PutNotificationSettingsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Attaches a list of notification settings to a trust anchor. A notification setting includes information such as event name, threshold, status of the notification setting, and the channel to notify. |
| `ResetNotificationSettings` | `PATCH /reset-notifications-settings` | - | `notificationSettingKeys`, `trustAnchorId` | - | `ResetNotificationSettingsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Resets the custom notification setting to IAM Roles Anywhere default setting. Required permissions: `rolesanywhere:ResetNotificationSettings`. |
| `TagResource` | `POST /TagResource` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Attaches tags to a resource. Required permissions: `rolesanywhere:TagResource`. |
| `UntagResource` | `POST /UntagResource` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from the resource. Required permissions: `rolesanywhere:UntagResource`. |
| `UpdateCrl` | `PATCH /crl/{crlId}` | - | `crlId` | - | `CrlDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Updates the certificate revocation list (CRL). A CRL is a list of certificates that have been revoked by the issuing certificate authority (CA). |
| `UpdateProfile` | `PATCH /profile/{profileId}` | `idempotent` | `profileId` | - | `ProfileDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Updates a profile , a list of the roles that IAM Roles Anywhere service is trusted to assume. You use profiles to intersect permissions with IAM managed policies. |
| `UpdateTrustAnchor` | `PATCH /trustanchor/{trustAnchorId}` | `idempotent` | `trustAnchorId` | - | `TrustAnchorDetailResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Updates a trust anchor. You establish trust between IAM Roles Anywhere and your certificate authority (CA) by configuring a trust anchor. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListTagsForResource` | - | `resourceArn -> resourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `ResourceNotFoundException` | `structure` | `message` | The resource could not be found. |
| `ValidationException` | `structure` | `message` | Validation exception error. |
| `ProfileDetailResponse` | `structure` | `profile` | - |
| `TrustAnchorDetailResponse` | `structure` | `trustAnchor` | - |
| `CrlDetailResponse` | `structure` | `crl` | - |
| `ScalarCrlRequest` | `structure` | `crlId` | - |
| `ScalarProfileRequest` | `structure` | `profileId` | - |
| `ScalarTrustAnchorRequest` | `structure` | `trustAnchorId` | - |
| `ListRequest` | `structure` | `nextToken`, `pageSize` | - |
| `CreateProfileRequest` | `structure` | `acceptRoleSessionName`, `durationSeconds`, `enabled`, `managedPolicyArns`, `name`, `requireInstanceProperties`, `roleArns`, `sessionPolicy`, `tags` | - |
| `CreateTrustAnchorRequest` | `structure` | `enabled`, `name`, `notificationSettings`, `source`, `tags` | - |
| `DeleteAttributeMappingRequest` | `structure` | `certificateField`, `profileId`, `specifiers` | - |
| `DeleteAttributeMappingResponse` | `structure` | `profile` | - |
| `ScalarSubjectRequest` | `structure` | `subjectId` | - |
| `SubjectDetailResponse` | `structure` | `subject` | - |
| `ImportCrlRequest` | `structure` | `crlData`, `enabled`, `name`, `tags`, `trustAnchorArn` | - |
| `ListCrlsResponse` | `structure` | `crls`, `nextToken` | - |
| `ListProfilesResponse` | `structure` | `nextToken`, `profiles` | - |
| `ListSubjectsResponse` | `structure` | `nextToken`, `subjects` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `ListTrustAnchorsResponse` | `structure` | `nextToken`, `trustAnchors` | - |
| `PutAttributeMappingRequest` | `structure` | `certificateField`, `mappingRules`, `profileId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
