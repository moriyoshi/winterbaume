# winterbaume-cognitoidentityprovider

Cognito Identity Provider service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cognito Identity Provider |
| AWS model | `cognito-identity-provider` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 104/122 operations (85.2%) |
| stubs (routed, returns empty/default) | 18/122 operations (14.8%) |
| moto coverage | 62/122 operations (50.8%) |
| floci coverage | 0/122 operations (0.0%) |
| kumo coverage | 17/122 operations (13.9%) |
| fakecloud coverage | 122/122 operations (100.0%) |
| Coverage report date | 2026-07-03 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws cognito-idp list-user-pools --max-results 10
```

## Example

```rust
use aws_sdk_cognitoidentityprovider::config::BehaviorVersion;
use winterbaume_cognitoidentityprovider::CognitoIdentityProviderService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CognitoIdentityProviderService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitoidentityprovider::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_cognitoidentityprovider::Client::new(&config);

    let resp = client
        .list_user_pools()
        .max_results(60)
        .send()
        .await
        .expect("list_user_pools should succeed");
    println!("User pools: {}", resp.user_pools().len());
}
```

## Implemented APIs (104)

- `AddCustomAttributes`
- `AddUserPoolClientSecret`
- `AdminAddUserToGroup`
- `AdminConfirmSignUp`
- `AdminCreateUser`
- `AdminDeleteUser`
- `AdminDeleteUserAttributes`
- `AdminDisableProviderForUser`
- `AdminDisableUser`
- `AdminEnableUser`
- `AdminForgetDevice`
- `AdminGetDevice`
- `AdminGetUser`
- `AdminInitiateAuth`
- `AdminLinkProviderForUser`
- `AdminListDevices`
- `AdminListGroupsForUser`
- `AdminRemoveUserFromGroup`
- `AdminResetUserPassword`
- `AdminRespondToAuthChallenge`
- `AdminSetUserMFAPreference`
- `AdminSetUserPassword`
- `AdminSetUserSettings`
- `AdminUpdateDeviceStatus`
- `AdminUpdateUserAttributes`
- `AdminUserGlobalSignOut`
- `ConfirmDevice`
- `ConfirmForgotPassword`
- `ConfirmSignUp`
- `CreateGroup`
- `CreateIdentityProvider`
- `CreateManagedLoginBranding`
- `CreateResourceServer`
- `CreateTerms`
- `CreateUserImportJob`
- `CreateUserPool`
- `CreateUserPoolClient`
- `CreateUserPoolDomain`
- `DeleteGroup`
- `DeleteIdentityProvider`
- `DeleteManagedLoginBranding`
- `DeleteResourceServer`
- `DeleteTerms`
- `DeleteUserAttributes`
- `DeleteUserPool`
- `DeleteUserPoolClient`
- `DeleteUserPoolClientSecret`
- `DeleteUserPoolDomain`
- `DeleteWebAuthnCredential`
- `DescribeIdentityProvider`
- `DescribeManagedLoginBranding`
- `DescribeManagedLoginBrandingByClient`
- `DescribeResourceServer`
- `DescribeRiskConfiguration`
- `DescribeTerms`
- `DescribeUserImportJob`
- `DescribeUserPool`
- `DescribeUserPoolClient`
- `DescribeUserPoolDomain`
- `ForgetDevice`
- `ForgotPassword`
- `GetCSVHeader`
- `GetDevice`
- `GetGroup`
- `GetIdentityProviderByIdentifier`
- `GetLogDeliveryConfiguration`
- `GetSigningCertificate`
- `GetUICustomization`
- `GetUserPoolMfaConfig`
- `GlobalSignOut`
- `InitiateAuth`
- `ListDevices`
- `ListGroups`
- `ListIdentityProviders`
- `ListResourceServers`
- `ListTagsForResource`
- `ListTerms`
- `ListUserImportJobs`
- `ListUserPoolClientSecrets`
- `ListUserPoolClients`
- `ListUserPools`
- `ListUsers`
- `ListUsersInGroup`
- `ListWebAuthnCredentials`
- `ResendConfirmationCode`
- `RespondToAuthChallenge`
- `SetLogDeliveryConfiguration`
- `SetRiskConfiguration`
- `SetUICustomization`
- `SetUserPoolMfaConfig`
- `SignUp`
- `StartUserImportJob`
- `StopUserImportJob`
- `TagResource`
- `UntagResource`
- `UpdateDeviceStatus`
- `UpdateGroup`
- `UpdateIdentityProvider`
- `UpdateManagedLoginBranding`
- `UpdateResourceServer`
- `UpdateTerms`
- `UpdateUserPool`
- `UpdateUserPoolClient`
- `UpdateUserPoolDomain`

<details><summary>Stubbed APIs (18) &mdash; routed but return an empty/default response</summary>

- `AdminListUserAuthEvents`
- `AdminUpdateAuthEventFeedback`
- `AssociateSoftwareToken`
- `ChangePassword`
- `CompleteWebAuthnRegistration`
- `DeleteUser`
- `GetTokensFromRefreshToken`
- `GetUser`
- `GetUserAttributeVerificationCode`
- `GetUserAuthFactors`
- `RevokeToken`
- `SetUserMFAPreference`
- `SetUserSettings`
- `StartWebAuthnRegistration`
- `UpdateAuthEventFeedback`
- `UpdateUserAttributes`
- `VerifySoftwareToken`
- `VerifyUserAttribute`

</details>
