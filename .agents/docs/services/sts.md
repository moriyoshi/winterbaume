# AWS Security Token Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Security Token Service Security Token Service (STS) enables you to request temporary, limited-privilege credentials for users. This guide provides descriptions of the STS API. For more information about using this service, see Temporary Security Credentials.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Security Token Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `Assume`, `Decode` operation families, including `GetAccessKeyInfo`, `GetCallerIdentity`, `GetDelegatedAccessToken`, `GetFederationToken`, `AssumeRole`, `AssumeRoleWithSAML`.

## Service Identity and Protocol

- AWS model slug: `sts`
- AWS SDK for Rust slug: `sts`
- Model version: `2011-06-15`
- Model file: `vendor/api-models-aws/models/sts/service/2011-06-15/sts-2011-06-15.json`
- SDK ID: `STS`
- Endpoint prefix: `sts`
- ARN namespace: `sts`
- CloudFormation name: `STS`
- CloudTrail event source: `sts.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`, `UseGlobalEndpoint`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (6), `Assume` (4), `Decode` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccessKeyInfo`, `GetCallerIdentity`, `GetDelegatedAccessToken`, `GetFederationToken`, `GetSessionToken`, `GetWebIdentityToken`.
- 9 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_request.html

Research outcomes:
- AssumeRole returns temporary credentials containing access key ID, secret access key, and session token.
- Temporary credentials from AssumeRole can call AWS services except STS GetFederationToken and GetSessionToken.
- Session policies can be inline or up to 10 managed policy ARNs. Resulting permissions are the intersection of the role identity policy and session policies.
- Session policies cannot grant permissions beyond those already allowed by the assumed role.
- Cross-account AssumeRole requires both the role trust policy and caller-side permission to call AssumeRole for that role ARN.
- Same-account trust policies can grant assumption directly as a resource policy without a separate identity policy in some cases.
- DurationSeconds ranges from 900 seconds to the role maximum session duration, up to 43200 seconds. Role chaining limits sessions to one hour.
- MFA-protected role assumption requires SerialNumber and TokenCode when the trust policy tests MFA presence. ExternalId can be required for third-party cross-account access.

Parity implications:
- Model role sessions, temporary credentials, expiry, session policies, session tags, source identity, MFA context, ExternalId, and role chaining.
- AssumeRole authorisation needs trust-policy evaluation, caller identity permission, explicit deny handling, and policy intersection with session policies.
- Generated credentials should carry session principal identity and expiry into later service authorisation checks.

## Operation Groups

### Get

- Operations: `GetAccessKeyInfo`, `GetCallerIdentity`, `GetDelegatedAccessToken`, `GetFederationToken`, `GetSessionToken`, `GetWebIdentityToken`
- Common required input members in this group: `AccessKeyId`, `Audience`, `Name`, `SigningAlgorithm`, `TradeInToken`

### Assume

- Operations: `AssumeRole`, `AssumeRoleWithSAML`, `AssumeRoleWithWebIdentity`, `AssumeRoot`
- Common required input members in this group: `PrincipalArn`, `RoleArn`, `RoleSessionName`, `SAMLAssertion`, `TargetPrincipal`, `TaskPolicyArn`, `WebIdentityToken`

### Decode

- Operations: `DecodeAuthorizationMessage`
- Common required input members in this group: `EncodedMessage`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssumeRole` | - | - | `RoleArn`, `RoleSessionName` | - | `AssumeRoleResponse` | `ExpiredTokenException`, `MalformedPolicyDocumentException`, `PackedPolicyTooLargeException`, `RegionDisabledException` | Returns a set of temporary security credentials that you can use to access Amazon Web Services resources. These temporary credentials consist of an access key ID, a secret access key, and a security token. |
| `AssumeRoleWithSAML` | - | - | `PrincipalArn`, `RoleArn`, `SAMLAssertion` | - | `AssumeRoleWithSAMLResponse` | `ExpiredTokenException`, `IDPRejectedClaimException`, `InvalidIdentityTokenException`, `MalformedPolicyDocumentException`, `PackedPolicyTooLargeException`, `RegionDisabledException` | Returns a set of temporary security credentials for users who have been authenticated via a SAML authentication response. This operation provides a mechanism for tying an enterprise identity store or directory to role-based Amazon Web Services access without... |
| `AssumeRoleWithWebIdentity` | - | - | `RoleArn`, `RoleSessionName`, `WebIdentityToken` | - | `AssumeRoleWithWebIdentityResponse` | `ExpiredTokenException`, `IDPCommunicationErrorException`, `IDPRejectedClaimException`, `InvalidIdentityTokenException`, `MalformedPolicyDocumentException`, `PackedPolicyTooLargeException`, `RegionDisabledException` | Returns a set of temporary security credentials for users who have been authenticated in a mobile or web application with a web identity provider. Example providers include the OAuth 2.0 providers Login with Amazon and Facebook, or any OpenID... |
| `AssumeRoot` | - | - | `TargetPrincipal`, `TaskPolicyArn` | - | `AssumeRootResponse` | `ExpiredTokenException`, `RegionDisabledException` | Returns a set of short term credentials you can use to perform privileged tasks on a member account in your organization. You must use credentials from an Organizations management account or a delegated administrator account for IAM to call `AssumeRoot`. |
| `DecodeAuthorizationMessage` | - | - | `EncodedMessage` | - | `DecodeAuthorizationMessageResponse` | `InvalidAuthorizationMessageException` | Decodes additional information about the authorization status of a request from an encoded message returned in response to an Amazon Web Services request. For example, if a user is not authorized to perform an operation that he or she has requested, the... |
| `GetAccessKeyInfo` | - | - | `AccessKeyId` | - | `GetAccessKeyInfoResponse` | - | Returns the account identifier for the specified access key ID. Access keys consist of two parts: an access key ID (for example, `AKIAIOSFODNN7EXAMPLE`) and a secret access key (for example, `wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY`). |
| `GetCallerIdentity` | - | - | - | - | `GetCallerIdentityResponse` | - | Returns details about the IAM user or role whose credentials are used to call the operation. No permissions are required to perform this operation. |
| `GetDelegatedAccessToken` | - | - | `TradeInToken` | - | `GetDelegatedAccessTokenResponse` | `ExpiredTradeInTokenException`, `PackedPolicyTooLargeException`, `RegionDisabledException` | Exchanges a trade-in token for temporary Amazon Web Services credentials with the permissions associated with the assumed principal. This operation allows you to obtain credentials for a specific principal based on a trade-in token, enabling delegation of... |
| `GetFederationToken` | - | - | `Name` | - | `GetFederationTokenResponse` | `MalformedPolicyDocumentException`, `PackedPolicyTooLargeException`, `RegionDisabledException` | Returns a set of temporary security credentials (consisting of an access key ID, a secret access key, and a security token) for a user. A typical use is in a proxy application that gets temporary security credentials on behalf of distributed applications... |
| `GetSessionToken` | - | - | - | - | `GetSessionTokenResponse` | `RegionDisabledException` | Returns a set of temporary credentials for an Amazon Web Services account or IAM user. The credentials consist of an access key ID, a secret access key, and a security token. |
| `GetWebIdentityToken` | - | - | `Audience`, `SigningAlgorithm` | - | `GetWebIdentityTokenResponse` | `JWTPayloadSizeExceededException`, `OutboundWebIdentityFederationDisabledException`, `SessionDurationEscalationException` | Returns a signed JSON Web Token (JWT) that represents the calling Amazon Web Services identity. The returned JWT can be used to authenticate with external services that support OIDC discovery. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `RegionDisabledException` | `structure` | `message` | STS is not activated in the requested region for the account that is being asked to generate credentials. |
| `PackedPolicyTooLargeException` | `structure` | `message` | The request was rejected because the total packed size of the session policies and session tags combined was too large. |
| `ExpiredTokenException` | `structure` | `message` | The web identity token that was passed is expired or is not valid. |
| `MalformedPolicyDocumentException` | `structure` | `message` | The request was rejected because the policy document was malformed. |
| `IDPRejectedClaimException` | `structure` | `message` | The identity provider (IdP) reported that authentication failed. |
| `InvalidIdentityTokenException` | `structure` | `message` | The web identity token that was passed could not be validated by Amazon Web Services. |
| `AssumeRoleRequest` | `structure` | `DurationSeconds`, `ExternalId`, `Policy`, `PolicyArns`, `ProvidedContexts`, `RoleArn`, `RoleSessionName`, `SerialNumber`, `SourceIdentity`, `Tags`, `TokenCode`, `TransitiveTagKeys` | - |
| `AssumeRoleResponse` | `structure` | `AssumedRoleUser`, `Credentials`, `PackedPolicySize`, `SourceIdentity` | Contains the response to a successful AssumeRole request, including temporary Amazon Web Services credentials that can be used to make Amazon Web Services requests. |
| `AssumeRoleWithSAMLRequest` | `structure` | `DurationSeconds`, `Policy`, `PolicyArns`, `PrincipalArn`, `RoleArn`, `SAMLAssertion` | - |
| `AssumeRoleWithSAMLResponse` | `structure` | `AssumedRoleUser`, `Audience`, `Credentials`, `Issuer`, `NameQualifier`, `PackedPolicySize`, `SourceIdentity`, `Subject`, `SubjectType` | Contains the response to a successful AssumeRoleWithSAML request, including temporary Amazon Web Services credentials that can be used to make Amazon Web Services requests. |
| `AssumeRoleWithWebIdentityRequest` | `structure` | `DurationSeconds`, `Policy`, `PolicyArns`, `ProviderId`, `RoleArn`, `RoleSessionName`, `WebIdentityToken` | - |
| `AssumeRoleWithWebIdentityResponse` | `structure` | `AssumedRoleUser`, `Audience`, `Credentials`, `PackedPolicySize`, `Provider`, `SourceIdentity`, `SubjectFromWebIdentityToken` | Contains the response to a successful AssumeRoleWithWebIdentity request, including temporary Amazon Web Services credentials that can be used to make Amazon Web Services requests. |
| `IDPCommunicationErrorException` | `structure` | `message` | The request could not be fulfilled because the identity provider (IDP) that was asked to verify the incoming identity token could not be reached. |
| `AssumeRootRequest` | `structure` | `DurationSeconds`, `TargetPrincipal`, `TaskPolicyArn` | - |
| `AssumeRootResponse` | `structure` | `Credentials`, `SourceIdentity` | - |
| `DecodeAuthorizationMessageRequest` | `structure` | `EncodedMessage` | - |
| `DecodeAuthorizationMessageResponse` | `structure` | `DecodedMessage` | A document that contains additional information about the authorization status of a request from an encoded message that is returned in response to an Amazon Web Services request. |
| `InvalidAuthorizationMessageException` | `structure` | `message` | The error returned if the message passed to `DecodeAuthorizationMessage` was invalid. |
| `GetAccessKeyInfoRequest` | `structure` | `AccessKeyId` | - |
| `GetAccessKeyInfoResponse` | `structure` | `Account` | - |
| `GetCallerIdentityRequest` | `structure` | - | - |
| `GetCallerIdentityResponse` | `structure` | `Account`, `Arn`, `UserId` | Contains the response to a successful GetCallerIdentity request, including information about the entity making the request. |
| `GetDelegatedAccessTokenRequest` | `structure` | `TradeInToken` | - |
| `GetDelegatedAccessTokenResponse` | `structure` | `AssumedPrincipal`, `Credentials`, `PackedPolicySize` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
