# winterbaume-acm

ACM service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ACM |
| AWS model | `acm` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 16/17 operations (94.1%) |
| stubs (routed, returns empty/default) | 0/17 operations (0.0%) |
| moto coverage | 11/17 operations (64.7%) |
| floci coverage | 0/17 operations (0.0%) |
| kumo coverage | 6/17 operations (35.3%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws acm list-certificates
```

## Example

```rust
use aws_sdk_acm::config::BehaviorVersion;
use winterbaume_acm::AcmService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(AcmService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_acm::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_acm::Client::new(&config);

    let resp = client
        .list_certificates()
        .send()
        .await
        .expect("list_certificates should succeed");
    println!("Certificates: {}", resp.certificate_summary_list().len());
}
```

## Implemented APIs (16)

- `AddTagsToCertificate`
- `DeleteCertificate`
- `DescribeCertificate`
- `ExportCertificate`
- `GetAccountConfiguration`
- `GetCertificate`
- `ImportCertificate`
- `ListCertificates`
- `ListTagsForCertificate`
- `PutAccountConfiguration`
- `RemoveTagsFromCertificate`
- `RenewCertificate`
- `RequestCertificate`
- `ResendValidationEmail`
- `RevokeCertificate`
- `UpdateCertificateOptions`

<details><summary>Not yet implemented APIs (1)</summary>

- `SearchCertificates`

</details>
