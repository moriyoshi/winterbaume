# winterbaume-route53domains

Route 53 Domains service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Route 53 Domains |
| AWS model | `route-53-domains` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 5/34 operations (14.7%) |
| stubs (routed, returns empty/default) | 0/34 operations (0.0%) |
| moto coverage | 0/34 operations (0.0%) |
| floci coverage | 0/34 operations (0.0%) |
| kumo coverage | 0/34 operations (0.0%) |
| fakecloud coverage | 0/34 operations (0.0%) |
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
aws route53domains list-domains
```

## Example

```rust
use aws_sdk_route53domains::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53domains::Route53DomainsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Route53DomainsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53domains::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_route53domains::Client::new(&config);

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");
    println!("Route 53 domains: {}", resp.domains().len());
}
```

## Implemented APIs (5)

- `CheckDomainAvailability`
- `DeleteDomain`
- `GetDomainDetail`
- `ListDomains`
- `RegisterDomain`

<details><summary>Not yet implemented APIs (29)</summary>

- `AcceptDomainTransferFromAnotherAwsAccount`
- `AssociateDelegationSignerToDomain`
- `CancelDomainTransferToAnotherAwsAccount`
- `CheckDomainTransferability`
- `DeleteTagsForDomain`
- `DisableDomainAutoRenew`
- `DisableDomainTransferLock`
- `DisassociateDelegationSignerFromDomain`
- `EnableDomainAutoRenew`
- `EnableDomainTransferLock`
- `GetContactReachabilityStatus`
- `GetDomainSuggestions`
- `GetOperationDetail`
- `ListOperations`
- `ListPrices`
- `ListTagsForDomain`
- `PushDomain`
- `RejectDomainTransferFromAnotherAwsAccount`
- `RenewDomain`
- `ResendContactReachabilityEmail`
- `ResendOperationAuthorization`
- `RetrieveDomainAuthCode`
- `TransferDomain`
- `TransferDomainToAnotherAwsAccount`
- `UpdateDomainContact`
- `UpdateDomainContactPrivacy`
- `UpdateDomainNameservers`
- `UpdateTagsForDomain`
- `ViewBilling`

</details>
