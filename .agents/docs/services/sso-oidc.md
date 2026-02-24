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
- Common required input members in this group: `clientId`, `clientSecret`, `grantType`

### Register

- Operations: `RegisterClient`
- Common required input members in this group: `clientName`, `clientType`

### Start

- Operations: `StartDeviceAuthorization`
- Common required input members in this group: `clientId`, `clientSecret`, `startUrl`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateToken` | `POST /token` | - | `clientId`, `clientSecret`, `grantType` | - | `CreateTokenResponse` | `AccessDeniedException`, `AuthorizationPendingException`, `ExpiredTokenException`, `InternalServerException`, `InvalidClientException`, `InvalidGrantException`, `InvalidRequestException`, `InvalidScopeException`, ... (+3) | Creates and returns access and refresh tokens for clients that are authenticated using client secrets. The access token can be used to fetch short-lived credentials for the assigned AWS accounts or to access application APIs using `bearer` authentication. |
| `CreateTokenWithIAM` | `POST /token?aws_iam=t` | - | `clientId`, `grantType` | - | `CreateTokenWithIAMResponse` | `AccessDeniedException`, `AuthorizationPendingException`, `ExpiredTokenException`, `InternalServerException`, `InvalidClientException`, `InvalidGrantException`, `InvalidRequestException`, `InvalidRequestRegionException`, ... (+4) | Creates and returns access and refresh tokens for authorized client applications that are authenticated using any IAM entity, such as a service role or user. These tokens might contain defined scopes that specify permissions such as `read:profile` or... |
| `RegisterClient` | `POST /client/register` | - | `clientName`, `clientType` | - | `RegisterClientResponse` | `InternalServerException`, `InvalidClientMetadataException`, `InvalidRedirectUriException`, `InvalidRequestException`, `InvalidScopeException`, `SlowDownException`, `UnsupportedGrantTypeException` | Registers a public client with IAM Identity Center. This allows clients to perform authorization using the authorization code grant with Proof Key for Code Exchange (PKCE) or the device code grant. |
| `StartDeviceAuthorization` | `POST /device_authorization` | - | `clientId`, `clientSecret`, `startUrl` | - | `StartDeviceAuthorizationResponse` | `InternalServerException`, `InvalidClientException`, `InvalidRequestException`, `SlowDownException`, `UnauthorizedClientException` | Initiates device authorization by requesting a pair of verification codes from the authorization service. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `error`, `error_description` | Indicates that an error from the service occurred while trying to process a request. |
| `InvalidRequestException` | `structure` | `error`, `error_description`, `reason` | Indicates that something is wrong with the input to the request. |
| `SlowDownException` | `structure` | `error`, `error_description` | Indicates that the client is making the request too frequently and is more than the service can handle. |
| `InvalidClientException` | `structure` | `error`, `error_description` | Indicates that the `clientId` or `clientSecret` in the request is invalid. |
| `InvalidScopeException` | `structure` | `error`, `error_description` | Indicates that the scope provided in the request is invalid. |
| `UnauthorizedClientException` | `structure` | `error`, `error_description` | Indicates that the client is not currently authorized to make the request. |
| `UnsupportedGrantTypeException` | `structure` | `error`, `error_description` | Indicates that the grant type in the request is not supported by the service. |
| `AccessDeniedException` | `structure` | `error`, `error_description`, `reason` | You do not have sufficient access to perform this action. |
| `AuthorizationPendingException` | `structure` | `error`, `error_description` | Indicates that a request to authorize a client with an access user session token is pending. |
| `ExpiredTokenException` | `structure` | `error`, `error_description` | Indicates that the token issued by the service is expired and is no longer valid. |
| `InvalidGrantException` | `structure` | `error`, `error_description` | Indicates that a request contains an invalid grant. |
| `CreateTokenRequest` | `structure` | `clientId`, `clientSecret`, `code`, `codeVerifier`, `deviceCode`, `grantType`, `redirectUri`, `refreshToken`, `scope` | - |
| `CreateTokenResponse` | `structure` | `accessToken`, `expiresIn`, `idToken`, `refreshToken`, `tokenType` | - |
| `CreateTokenWithIAMRequest` | `structure` | `assertion`, `clientId`, `code`, `codeVerifier`, `grantType`, `redirectUri`, `refreshToken`, `requestedTokenType`, `scope`, `subjectToken`, `subjectTokenType` | - |
| `CreateTokenWithIAMResponse` | `structure` | `accessToken`, `awsAdditionalDetails`, `expiresIn`, `idToken`, `issuedTokenType`, `refreshToken`, `scope`, `tokenType` | - |
| `InvalidRequestRegionException` | `structure` | `endpoint`, `error`, `error_description`, `region` | Indicates that a token provided as input to the request was issued by and is only usable by calling IAM Identity Center endpoints in another region. |
| `RegisterClientRequest` | `structure` | `clientName`, `clientType`, `entitledApplicationArn`, `grantTypes`, `issuerUrl`, `redirectUris`, `scopes` | - |
| `RegisterClientResponse` | `structure` | `authorizationEndpoint`, `clientId`, `clientIdIssuedAt`, `clientSecret`, `clientSecretExpiresAt`, `tokenEndpoint` | - |
| `InvalidClientMetadataException` | `structure` | `error`, `error_description` | Indicates that the client information sent in the request during registration is invalid. |
| `InvalidRedirectUriException` | `structure` | `error`, `error_description` | Indicates that one or more redirect URI in the request is not supported for this operation. |
| `StartDeviceAuthorizationRequest` | `structure` | `clientId`, `clientSecret`, `startUrl` | - |
| `StartDeviceAuthorizationResponse` | `structure` | `deviceCode`, `expiresIn`, `interval`, `userCode`, `verificationUri`, `verificationUriComplete` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
