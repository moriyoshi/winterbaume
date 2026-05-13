# winterbaume-wafv2

WAFv2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | WAFv2 |
| AWS model | `wafv2` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 38/55 operations (69.1%) |
| stubs (routed, returns empty/default) | 0/55 operations (0.0%) |
| moto coverage | 29/55 operations (52.7%) |
| floci coverage | 0/55 operations (0.0%) |
| kumo coverage | 0/55 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws wafv2 list-web-acls --scope REGIONAL
```

## Example

```rust
use aws_sdk_wafv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_wafv2::WafV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(WafV2Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_wafv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_wafv2::Client::new(&config);

    use aws_sdk_wafv2::types::Scope;
    let resp = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list_web_acls should succeed");
    println!("WAFv2 web ACLs: {}", resp.web_acls().len());
}
```

## Implemented APIs (38)

- `AssociateWebACL`
- `CheckCapacity`
- `CreateAPIKey`
- `CreateIPSet`
- `CreateRegexPatternSet`
- `CreateRuleGroup`
- `CreateWebACL`
- `DeleteAPIKey`
- `DeleteIPSet`
- `DeleteLoggingConfiguration`
- `DeletePermissionPolicy`
- `DeleteRegexPatternSet`
- `DeleteRuleGroup`
- `DeleteWebACL`
- `DisassociateWebACL`
- `GetIPSet`
- `GetLoggingConfiguration`
- `GetPermissionPolicy`
- `GetRegexPatternSet`
- `GetRuleGroup`
- `GetWebACL`
- `GetWebACLForResource`
- `ListAPIKeys`
- `ListIPSets`
- `ListLoggingConfigurations`
- `ListRegexPatternSets`
- `ListResourcesForWebACL`
- `ListRuleGroups`
- `ListTagsForResource`
- `ListWebACLs`
- `PutLoggingConfiguration`
- `PutPermissionPolicy`
- `TagResource`
- `UntagResource`
- `UpdateIPSet`
- `UpdateRegexPatternSet`
- `UpdateRuleGroup`
- `UpdateWebACL`

<details><summary>Not yet implemented APIs (17)</summary>

- `DeleteFirewallManagerRuleGroups`
- `DescribeAllManagedProducts`
- `DescribeManagedProductsByVendor`
- `DescribeManagedRuleGroup`
- `GenerateMobileSdkReleaseUrl`
- `GetDecryptedAPIKey`
- `GetManagedRuleSet`
- `GetMobileSdkRelease`
- `GetRateBasedStatementManagedKeys`
- `GetSampledRequests`
- `GetTopPathStatisticsByTraffic`
- `ListAvailableManagedRuleGroupVersions`
- `ListAvailableManagedRuleGroups`
- `ListManagedRuleSets`
- `ListMobileSdkReleases`
- `PutManagedRuleSetVersions`
- `UpdateManagedRuleSetVersionExpiryDate`

</details>
