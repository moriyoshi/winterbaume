# AWS SSO OIDC

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IAM Identity Center OpenID Connect (OIDC) is a web service that enables a client (such as CLI or a native application) to register with IAM Identity Center. The service also enables the client to fetch the user’s access token upon successful authentication and authorization with IAM Identity Center. API namespaces IAM Identity Center uses the `sso` and `identitystore` API namespaces. IAM Identity Center OpenID Connect uses the `sso-oauth` namespace. Considerations for using this guide Before you begin using this guide, we recommend that you first review the following important information about how the IAM Identity Center OIDC service works.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS SSO OIDC workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Create`, `Register`, `Start` operation families, including `CreateToken`, `CreateTokenWithIAM`, `RegisterClient`, `StartDeviceAuthorization`.

## Service Identity and Protocol

- AWS model slug: `sso-oidc`
- AWS SDK for Rust slug: `ssooidc`
- Model version: `2019-06-10`
- Model file: `vendor/api-models-aws/models/sso-oidc/service/2019-06-10/sso-oidc-2019-06-10.json`
- SDK ID: `SSO OIDC`
- Endpoint prefix: `oidc`
- ARN namespace: `sso-oauth`
- CloudFormation name: `SSOOIDC`
- CloudTrail event source: `ssooidc.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (2), `Register` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateToken`, `CreateTokenWithIAM`, `RegisterClient`, `StartDeviceAuthorization`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDeviceAuthorization`.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_RegisterClient.html
- https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_StartDeviceAuthorization.html
- https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html

Research outcomes:
- `RegisterClient` registers public OIDC clients for IAM Identity Center. Supported flows include authorisation code with PKCE and device code.
- `grantTypes` can restrict clients to `authorization_code`, `urn:ietf:params:oauth:grant-type:device_code`, and `refresh_token` flows.
- `redirectUris` constrain callback destinations after authorisation, and scopes constrain what the access token can request at registration time.
- `RegisterClient` returns endpoints, client id, client secret, issue time, and client secret expiry.
- `StartDeviceAuthorization` requires client id, client secret, and start URL, and returns device code, user code, verification URI, complete verification URI, expiry, and polling interval.
- The device code is short-lived. Clients poll `CreateToken` according to the returned interval until authorised or expired.
- `CreateToken` supports authorisation code, device code, and refresh token grants.
- `CreateToken` access tokens can be used to fetch short-lived credentials for assigned AWS accounts or access application APIs as bearer tokens.
- The `scope` parameter on `CreateToken` has no effect; the access token includes the scopes configured during client registration. The `idToken` response field is documented as unsupported.

Parity implications:
- Model registered public clients, client secret expiry, allowed grant types, redirect URIs, scopes, device authorisation records, polling interval, token expiry, and refresh tokens.
- Device-code polling should enforce pending, authorised, denied, expired, and polling-too-fast behaviours.
- Token creation should use the client's registered scopes rather than request-time scope values.

## Operation Groups

### Create

- Operations: `CreateToken`, `CreateTokenWithIAM`
- Common required input members in this group: `clientId`, `grantType`

### Register

- Operations: `RegisterClient`
- Common required input members in this group: -

### Start

- Operations: `StartDeviceAuthorization`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateToken` | `POST /token` | - | `clientId`, `clientSecret`, `grantType` | - | `CreateTokenResponse` | `AccessDeniedException`, `AuthorizationPendingException`, `ExpiredTokenException`, `InternalServerException`, `InvalidClientException`, `InvalidGrantException`, `InvalidRequestException`, `InvalidScopeException`, `SlowDownException`, `UnauthorizedClientException`, `UnsupportedGrantTypeException` | Creates and returns access and refresh tokens for clients that are authenticated using client secrets. The access token can be used to fetch short-lived credentials for the assigned AWS accounts or to access applicat ... |
| `CreateTokenWithIAM` | `POST /token?aws_iam=t` | - | `clientId`, `grantType` | - | `CreateTokenWithIAMResponse` | `AccessDeniedException`, `AuthorizationPendingException`, `ExpiredTokenException`, `InternalServerException`, `InvalidClientException`, `InvalidGrantException`, `InvalidRequestException`, `InvalidRequestRegionException`, `InvalidScopeException`, `SlowDownException`, `UnauthorizedClientException`, `UnsupportedGrantTypeException` | Creates and returns access and refresh tokens for authorized client applications that are authenticated using any IAM entity, such as a service role or user. These tokens might contain defined scopes that specify per ... |
| `RegisterClient` | `POST /client/register` | - | `clientName`, `clientType` | - | `RegisterClientResponse` | `InternalServerException`, `InvalidClientMetadataException`, `InvalidRedirectUriException`, `InvalidRequestException`, `InvalidScopeException`, `SlowDownException`, `UnsupportedGrantTypeException` | Registers a public client with IAM Identity Center. This allows clients to perform authorization using the authorization code grant with Proof Key for Code Exchange (PKCE) or the device code grant. |
| `StartDeviceAuthorization` | `POST /device_authorization` | - | `clientId`, `clientSecret`, `startUrl` | - | `StartDeviceAuthorizationResponse` | `InternalServerException`, `InvalidClientException`, `InvalidRequestException`, `SlowDownException`, `UnauthorizedClientException` | Initiates device authorization by requesting a pair of verification codes from the authorization service. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | error, reason, error_description | You do not have sufficient access to perform this action. |
| `AuthorizationPendingException` | `structure` | error, error_description | Indicates that a request to authorize a client with an access user session token is pending. |
| `ExpiredTokenException` | `structure` | error, error_description | Indicates that the token issued by the service is expired and is no longer valid. |
| `InternalServerException` | `structure` | error, error_description | Indicates that an error from the service occurred while trying to process a request. |
| `InvalidClientException` | `structure` | error, error_description | Indicates that the clientId or clientSecret in the request is invalid. For example, this can occur when a client sends an incorrect clientId or an expired c ... |
| `InvalidClientMetadataException` | `structure` | error, error_description | Indicates that the client information sent in the request during registration is invalid. |
| `InvalidGrantException` | `structure` | error, error_description | Indicates that a request contains an invalid grant. This can occur if a client makes a CreateToken request with an invalid grant type. |
| `InvalidRedirectUriException` | `structure` | error, error_description | Indicates that one or more redirect URI in the request is not supported for this operation. |
| `InvalidRequestException` | `structure` | error, reason, error_description | Indicates that something is wrong with the input to the request. For example, a required parameter might be missing or out of range. |
| `InvalidRequestRegionException` | `structure` | error, error_description, endpoint, region | Indicates that a token provided as input to the request was issued by and is only usable by calling IAM Identity Center endpoints in another region. |
| `InvalidScopeException` | `structure` | error, error_description | Indicates that the scope provided in the request is invalid. |
| `SlowDownException` | `structure` | error, error_description | Indicates that the client is making the request too frequently and is more than the service can handle. |
| `UnauthorizedClientException` | `structure` | error, error_description | Indicates that the client is not currently authorized to make the request. This can happen when a clientId is not issued for a public client. |
| `UnsupportedGrantTypeException` | `structure` | error, error_description | Indicates that the grant type in the request is not supported by the service. |
| `CreateTokenRequest` | `structure` | clientId, clientSecret, grantType, deviceCode, code, refreshToken, scope, redirectUri, codeVerifier | - |
| `CreateTokenResponse` | `structure` | accessToken, tokenType, expiresIn, refreshToken, idToken | - |
| `CreateTokenWithIAMRequest` | `structure` | clientId, grantType, code, refreshToken, assertion, scope, redirectUri, subjectToken, subjectTokenType, requestedTokenType, codeVerifier | - |
| `CreateTokenWithIAMResponse` | `structure` | accessToken, tokenType, expiresIn, refreshToken, idToken, issuedTokenType, scope, awsAdditionalDetails | - |
| `RegisterClientRequest` | `structure` | clientName, clientType, scopes, redirectUris, grantTypes, issuerUrl, entitledApplicationArn | - |
| `RegisterClientResponse` | `structure` | clientId, clientSecret, clientIdIssuedAt, clientSecretExpiresAt, authorizationEndpoint, tokenEndpoint | - |
| `StartDeviceAuthorizationRequest` | `structure` | clientId, clientSecret, startUrl | - |
| `StartDeviceAuthorizationResponse` | `structure` | deviceCode, userCode, verificationUri, verificationUriComplete, expiresIn, interval | - |
| `AccessDeniedExceptionReason` | `enum` | KMS_ACCESS_DENIED | - |
| `InvalidRequestExceptionReason` | `enum` | KMS_KEY_NOT_FOUND, KMS_INVALID_KEY_USAGE, KMS_INVALID_STATE, KMS_DISABLED_KEY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
