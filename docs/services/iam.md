# winterbaume-iam

IAM service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | IAM |
| AWS model | `iam` |
| Protocol | awsQuery |
| winterbaume coverage | 154/176 operations (87.5%) |
| stubs (routed, returns empty/default) | 22/176 operations (12.5%) |
| moto coverage | 119/176 operations (67.6%) |
| floci coverage | 0/176 operations (0.0%) |
| kumo coverage | 39/176 operations (22.2%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws iam list-users
```

## Example

```rust
use aws_sdk_iam::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_iam::IamService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(IamService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_iam::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_iam::Client::new(&config);

    let resp = client
        .list_users()
        .send()
        .await
        .expect("list_users should succeed");
    println!("IAM users: {}", resp.users().len());
}
```

## Implemented APIs (154)

- `AddClientIDToOpenIDConnectProvider`
- `AddRoleToInstanceProfile`
- `AddUserToGroup`
- `AttachGroupPolicy`
- `AttachRolePolicy`
- `AttachUserPolicy`
- `CreateAccessKey`
- `CreateAccountAlias`
- `CreateGroup`
- `CreateInstanceProfile`
- `CreateLoginProfile`
- `CreateOpenIDConnectProvider`
- `CreatePolicy`
- `CreatePolicyVersion`
- `CreateRole`
- `CreateSAMLProvider`
- `CreateServiceLinkedRole`
- `CreateServiceSpecificCredential`
- `CreateUser`
- `CreateVirtualMFADevice`
- `DeactivateMFADevice`
- `DeleteAccessKey`
- `DeleteAccountAlias`
- `DeleteAccountPasswordPolicy`
- `DeleteGroup`
- `DeleteGroupPolicy`
- `DeleteInstanceProfile`
- `DeleteLoginProfile`
- `DeleteOpenIDConnectProvider`
- `DeletePolicy`
- `DeletePolicyVersion`
- `DeleteRole`
- `DeleteRolePermissionsBoundary`
- `DeleteRolePolicy`
- `DeleteSAMLProvider`
- `DeleteSSHPublicKey`
- `DeleteServerCertificate`
- `DeleteServiceLinkedRole`
- `DeleteServiceSpecificCredential`
- `DeleteSigningCertificate`
- `DeleteUser`
- `DeleteUserPermissionsBoundary`
- `DeleteUserPolicy`
- `DeleteVirtualMFADevice`
- `DetachGroupPolicy`
- `DetachRolePolicy`
- `DetachUserPolicy`
- `DisableOrganizationsRootCredentialsManagement`
- `DisableOrganizationsRootSessions`
- `EnableMFADevice`
- `EnableOrganizationsRootCredentialsManagement`
- `EnableOrganizationsRootSessions`
- `GenerateCredentialReport`
- `GetAccessKeyLastUsed`
- `GetAccountAuthorizationDetails`
- `GetAccountPasswordPolicy`
- `GetAccountSummary`
- `GetCredentialReport`
- `GetGroup`
- `GetGroupPolicy`
- `GetInstanceProfile`
- `GetLoginProfile`
- `GetMFADevice`
- `GetOpenIDConnectProvider`
- `GetPolicy`
- `GetPolicyVersion`
- `GetRole`
- `GetRolePolicy`
- `GetSAMLProvider`
- `GetSSHPublicKey`
- `GetServerCertificate`
- `GetServiceLinkedRoleDeletionStatus`
- `GetUser`
- `GetUserPolicy`
- `ListAccessKeys`
- `ListAccountAliases`
- `ListAttachedGroupPolicies`
- `ListAttachedRolePolicies`
- `ListAttachedUserPolicies`
- `ListEntitiesForPolicy`
- `ListGroupPolicies`
- `ListGroups`
- `ListGroupsForUser`
- `ListInstanceProfileTags`
- `ListInstanceProfiles`
- `ListInstanceProfilesForRole`
- `ListMFADeviceTags`
- `ListMFADevices`
- `ListOpenIDConnectProviderTags`
- `ListOpenIDConnectProviders`
- `ListPolicies`
- `ListPolicyTags`
- `ListPolicyVersions`
- `ListRolePolicies`
- `ListRoleTags`
- `ListRoles`
- `ListSAMLProviderTags`
- `ListSAMLProviders`
- `ListSSHPublicKeys`
- `ListServerCertificateTags`
- `ListServerCertificates`
- `ListServiceSpecificCredentials`
- `ListSigningCertificates`
- `ListUserPolicies`
- `ListUserTags`
- `ListUsers`
- `ListVirtualMFADevices`
- `PutGroupPolicy`
- `PutRolePermissionsBoundary`
- `PutRolePolicy`
- `PutUserPermissionsBoundary`
- `PutUserPolicy`
- `RemoveClientIDFromOpenIDConnectProvider`
- `RemoveRoleFromInstanceProfile`
- `RemoveUserFromGroup`
- `ResetServiceSpecificCredential`
- `ResyncMFADevice`
- `SetDefaultPolicyVersion`
- `SetSecurityTokenServicePreferences`
- `SimulateCustomPolicy`
- `SimulatePrincipalPolicy`
- `TagInstanceProfile`
- `TagMFADevice`
- `TagOpenIDConnectProvider`
- `TagPolicy`
- `TagRole`
- `TagSAMLProvider`
- `TagServerCertificate`
- `TagUser`
- `UntagInstanceProfile`
- `UntagMFADevice`
- `UntagOpenIDConnectProvider`
- `UntagPolicy`
- `UntagRole`
- `UntagSAMLProvider`
- `UntagServerCertificate`
- `UntagUser`
- `UpdateAccessKey`
- `UpdateAccountPasswordPolicy`
- `UpdateAssumeRolePolicy`
- `UpdateGroup`
- `UpdateLoginProfile`
- `UpdateOpenIDConnectProviderThumbprint`
- `UpdateRole`
- `UpdateRoleDescription`
- `UpdateSAMLProvider`
- `UpdateSSHPublicKey`
- `UpdateServerCertificate`
- `UpdateServiceSpecificCredential`
- `UpdateSigningCertificate`
- `UpdateUser`
- `UploadSSHPublicKey`
- `UploadServerCertificate`
- `UploadSigningCertificate`

<details><summary>Stubbed APIs (22) &mdash; routed but return an empty/default response</summary>

- `AcceptDelegationRequest`
- `AssociateDelegationRequest`
- `ChangePassword`
- `CreateDelegationRequest`
- `DisableOutboundWebIdentityFederation`
- `EnableOutboundWebIdentityFederation`
- `GenerateOrganizationsAccessReport`
- `GenerateServiceLastAccessedDetails`
- `GetContextKeysForCustomPolicy`
- `GetContextKeysForPrincipalPolicy`
- `GetDelegationRequest`
- `GetHumanReadableSummary`
- `GetOrganizationsAccessReport`
- `GetOutboundWebIdentityFederationInfo`
- `GetServiceLastAccessedDetails`
- `GetServiceLastAccessedDetailsWithEntities`
- `ListDelegationRequests`
- `ListOrganizationsFeatures`
- `ListPoliciesGrantingServiceAccess`
- `RejectDelegationRequest`
- `SendDelegationToken`
- `UpdateDelegationRequest`

</details>
