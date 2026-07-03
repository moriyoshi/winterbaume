# winterbaume-acmpca

ACM PCA service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ACM PCA |
| AWS model | `acm-pca` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 23/23 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/23 operations (0.0%) |
| moto coverage | 17/23 operations (73.9%) |
| floci coverage | 0/23 operations (0.0%) |
| kumo coverage | 0/23 operations (0.0%) |
| fakecloud coverage | 0/23 operations (0.0%) |
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
aws acm-pca list-certificate-authorities
```

## Example

```rust
use aws_sdk_acmpca::config::BehaviorVersion;
use winterbaume_acmpca::AcmPcaService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AcmPcaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_acmpca::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_acmpca::Client::new(&config);

    let resp = client
        .list_certificate_authorities()
        .send()
        .await
        .expect("list_certificate_authorities should succeed");
    println!(
        "Certificate authorities: {}",
        resp.certificate_authorities().len()
    );
}
```

## Implemented APIs (23)

- `CreateCertificateAuthority`
- `CreateCertificateAuthorityAuditReport`
- `CreatePermission`
- `DeleteCertificateAuthority`
- `DeletePermission`
- `DeletePolicy`
- `DescribeCertificateAuthority`
- `DescribeCertificateAuthorityAuditReport`
- `GetCertificate`
- `GetCertificateAuthorityCertificate`
- `GetCertificateAuthorityCsr`
- `GetPolicy`
- `ImportCertificateAuthorityCertificate`
- `IssueCertificate`
- `ListCertificateAuthorities`
- `ListPermissions`
- `ListTags`
- `PutPolicy`
- `RestoreCertificateAuthority`
- `RevokeCertificate`
- `TagCertificateAuthority`
- `UntagCertificateAuthority`
- `UpdateCertificateAuthority`
