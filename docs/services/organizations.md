# winterbaume-organizations

Organizations service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Organizations |
| AWS model | `organizations` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 60/63 operations (95.2%) |
| stubs (routed, returns empty/default) | 3/63 operations (4.8%) |
| moto coverage | 41/63 operations (65.1%) |
| floci coverage | 0/63 operations (0.0%) |
| kumo coverage | 11/63 operations (17.5%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws organizations list-accounts
```

## Example

```rust
use aws_sdk_organizations::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_organizations::OrganizationsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OrganizationsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_organizations::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_organizations::Client::new(&config);

    let resp = client
        .list_accounts()
        .send()
        .await
        .expect("list_accounts should succeed");
    println!("Organization accounts: {}", resp.accounts().len());
}
```

## Implemented APIs (60)

- `AcceptHandshake`
- `AttachPolicy`
- `CancelHandshake`
- `CloseAccount`
- `CreateAccount`
- `CreateGovCloudAccount`
- `CreateOrganization`
- `CreateOrganizationalUnit`
- `CreatePolicy`
- `DeclineHandshake`
- `DeleteOrganization`
- `DeleteOrganizationalUnit`
- `DeletePolicy`
- `DeleteResourcePolicy`
- `DeregisterDelegatedAdministrator`
- `DescribeAccount`
- `DescribeCreateAccountStatus`
- `DescribeHandshake`
- `DescribeOrganization`
- `DescribeOrganizationalUnit`
- `DescribePolicy`
- `DescribeResourcePolicy`
- `DescribeResponsibilityTransfer`
- `DetachPolicy`
- `DisableAWSServiceAccess`
- `DisablePolicyType`
- `EnableAWSServiceAccess`
- `EnableAllFeatures`
- `EnablePolicyType`
- `InviteAccountToOrganization`
- `InviteOrganizationToTransferResponsibility`
- `LeaveOrganization`
- `ListAWSServiceAccessForOrganization`
- `ListAccounts`
- `ListAccountsForParent`
- `ListChildren`
- `ListCreateAccountStatus`
- `ListDelegatedAdministrators`
- `ListDelegatedServicesForAccount`
- `ListHandshakesForAccount`
- `ListHandshakesForOrganization`
- `ListInboundResponsibilityTransfers`
- `ListOrganizationalUnitsForParent`
- `ListOutboundResponsibilityTransfers`
- `ListParents`
- `ListPolicies`
- `ListPoliciesForTarget`
- `ListRoots`
- `ListTagsForResource`
- `ListTargetsForPolicy`
- `MoveAccount`
- `PutResourcePolicy`
- `RegisterDelegatedAdministrator`
- `RemoveAccountFromOrganization`
- `TagResource`
- `TerminateResponsibilityTransfer`
- `UntagResource`
- `UpdateOrganizationalUnit`
- `UpdatePolicy`
- `UpdateResponsibilityTransfer`

<details><summary>Stubbed APIs (3) &mdash; routed but return an empty/default response</summary>

- `DescribeEffectivePolicy`
- `ListAccountsWithInvalidEffectivePolicy`
- `ListEffectivePolicyValidationErrors`

</details>
