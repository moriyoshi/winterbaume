# winterbaume-taxsettings

AWS Tax Settings service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Tax Settings |
| AWS model | `taxsettings` |
| Protocol | restJson1 |
| winterbaume coverage | 0/16 operations (0.0%) |
| stubs (routed, returns empty/default) | 0/16 operations (0.0%) |
| moto coverage | 0/16 operations (0.0%) |
| floci coverage | 0/16 operations (0.0%) |
| kumo coverage | 0/16 operations (0.0%) |
| Coverage report date | 2026-05-16 |

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
aws taxsettings help
```

## Example

```rust
use aws_sdk_taxsettings::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_taxsettings::TaxSettingsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TaxSettingsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_taxsettings::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_taxsettings::Client::new(&config);
    let resp = client
        .get_tax_inheritance()
        .send()
        .await
        .expect("get_tax_inheritance should succeed");
    println!("Heritage status: {:?}", resp.heritage_status());
}
```

## Implemented APIs (0)

No modeled operations are currently detected as implemented.

<details><summary>Not yet implemented APIs (16)</summary>

- `BatchDeleteTaxRegistration`
- `BatchGetTaxExemptions`
- `BatchPutTaxRegistration`
- `DeleteSupplementalTaxRegistration`
- `DeleteTaxRegistration`
- `GetTaxExemptionTypes`
- `GetTaxInheritance`
- `GetTaxRegistration`
- `GetTaxRegistrationDocument`
- `ListSupplementalTaxRegistrations`
- `ListTaxExemptions`
- `ListTaxRegistrations`
- `PutSupplementalTaxRegistration`
- `PutTaxExemption`
- `PutTaxInheritance`
- `PutTaxRegistration`

</details>
