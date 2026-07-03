# winterbaume-servicequotas

Service Quotas service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Service Quotas |
| AWS model | `service-quotas` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 5/26 operations (19.2%) |
| stubs (routed, returns empty/default) | 0/26 operations (0.0%) |
| moto coverage | 2/26 operations (7.7%) |
| floci coverage | 0/26 operations (0.0%) |
| kumo coverage | 8/26 operations (30.8%) |
| fakecloud coverage | 0/26 operations (0.0%) |
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
aws service-quotas list-services
```

## Example

```rust
use aws_sdk_servicequotas::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicequotas::ServiceQuotasService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ServiceQuotasService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicequotas::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_servicequotas::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("Service Quotas services: {}", resp.services().len());
}
```

## Implemented APIs (5)

- `GetAWSDefaultServiceQuota`
- `GetServiceQuota`
- `ListAWSDefaultServiceQuotas`
- `ListServiceQuotas`
- `ListServices`

<details><summary>Not yet implemented APIs (21)</summary>

- `AssociateServiceQuotaTemplate`
- `CreateSupportCase`
- `DeleteServiceQuotaIncreaseRequestFromTemplate`
- `DisassociateServiceQuotaTemplate`
- `GetAssociationForServiceQuotaTemplate`
- `GetAutoManagementConfiguration`
- `GetQuotaUtilizationReport`
- `GetRequestedServiceQuotaChange` (implemented by kumo)
- `GetServiceQuotaIncreaseRequestFromTemplate`
- `ListRequestedServiceQuotaChangeHistory` (implemented by kumo)
- `ListRequestedServiceQuotaChangeHistoryByQuota`
- `ListServiceQuotaIncreaseRequestsInTemplate`
- `ListTagsForResource`
- `PutServiceQuotaIncreaseRequestIntoTemplate`
- `RequestServiceQuotaIncrease` (implemented by kumo)
- `StartAutoManagement`
- `StartQuotaUtilizationReport`
- `StopAutoManagement`
- `TagResource`
- `UntagResource`
- `UpdateAutoManagement`

</details>
