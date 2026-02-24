use crate::harness::*;

// ACM terraform resources tested here:
//   aws_acm_certificate (via ImportCertificate — avoids DNS/email validation wait)
//
// Note: aws_acm_certificate with validation_method = "DNS" or "EMAIL" causes
// terraform to wait for the certificate to reach ISSUED status. The winterbaume
// ACM handler returns ISSUED immediately for RequestCertificate, but the terraform
// provider may still poll DescribeCertificate waiting for the validation records
// to be populated. To avoid this, we use imported certificates which start as
// ISSUED immediately.
//
// NOTE: aws_acm_certificate with validation_method coverage is deferred until
// the terraform provider polling behaviour is confirmed stable against the
// winterbaume mock; ImportCertificate covers the same lifecycle today.

// ---------------------------------------------------------------------------
// Import certificate tests
// ---------------------------------------------------------------------------

// Self-signed test certificate (RSA 2048, expired — only for mock testing)
// Generated with: openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem
// -days 1 -nodes -subj "/CN=acm-test.example.com"
const TEST_CERT_PEM: &str = r#"-----BEGIN CERTIFICATE-----
MIICpDCCAYwCCQDU9pQ4pHnyrDANBgkqhkiG9w0BAQsFADAUMRIwEAYDVQQDDAls
b2NhbGhvc3QwHhcNMjMwMTAxMDAwMDAwWhcNMjMwMTAyMDAwMDAwWjAUMRIwEAYD
VQQDDAlsb2NhbGhvc3QwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQC7
o4qne60TB3wolLCJG5L7a/v9l9xMBYHBPKNaVHBMaFEZPUBWJtM1Q9kZSr/Jrmq
NTEXaPlhpCyKK7XBcI2VcxRUfRCMCDJaOzFw2nCbMOFlPRD0SMKQ6MBWs11MQME
aJv2X7kX0pRALTl6T7xVWzf/DUoGSEIuBQxGbHfMX3EO0VqGMu/dGTkGXvJnBwY
p5h2V1FPHsT5pRFkHMvyFbzOc7iB/d7Q0JIYpT4/MXvDTrxKiqFZZGJzb3n3Bsf
Yo8l2mhDtHkNSXjEGX8nWBHdvBR9KPSY8A0pFYwLfCrxOxeJVYcjxMFf3aXlxYQ
Q9xDKLqUlBr1GiLVGr7HAgMBAAEwDQYJKoZIhvcNAQELBQADggEBAA==
-----END CERTIFICATE-----"#;

const TEST_KEY_PEM: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAu6OKp3utEwd8KJSwiRuS+2v7/ZfcTAWBwTyjWlRwTGhRGT1A
VibTNUPZGUq/ya5qjUxF2j5YaQsiiu1wXCNlXMUVH0QjAgyWjsxcNpwmzDhZT0Q9
EjCkOjAVrNdTEDBGib9l+5F9KUQC05ek+8VVs3/w1KBkhCLgUMRmx3zF9xDtFahj
Lv3Rk5Bl7yZwcGKeYdldRTx7E+aURZBzL8hW8znO4gf3e0NCSGKU+PzF7w068Soq
hWWRic29594bH2KPJdpoQ7R5DUl4xBl/J1gR3bwUfSj0mPANKRWMC3wq8TsXiVWH
I8TBX92l5cWEEPcQyi6lJQa9Roi1Rq+xwIDAQABAoIBAGvr7VUVkFBBsXJHwHFi
mHmb7LrQpYnPTqt6wG8uTcS7HJKd2UWX3E9iSt5fQFzAL/EVqx5kJYMmBmjWHfE
ubkFHanQ0F0c9NAfWS2YkSbM8bBwrHN+6i0FN+SJWGJnCT0VqCGQc8x3w+bQmDi7
VMLKpRPNMJp0E6I8ZjlNwREAA3y8jR5eFh3x7C7XTnW0VeqBxsP4NvDZ3sDd2JXH
NUpj5LdOLt6Fy/1P5X7wSd8xyE0aRp1JMPIY5GqmFRoLPQKBgQDvdGHe6T1aJHOl
5p2BMeXnFGaI2mVkS8L/nNiIZTX5h4KE3nMKJYrGTJoMZ6UdVSJ8sQ4KFWWH3bpg
YtKzMFUC7eMK3EHa5S0M3HNf4JGxfyVE7p1QEeIg5J5KDqjnFuPnVwKBgQDI0rTi
qQY5VBMGhEFqkN9A0XxLQ3TkT/4RxH0G+k+pJ+XCAlbziFLkXHqUo0fR2mIy4PVg
b6hfzX9RJQF3LdvKCNzRe7pqIKz3V7JaZ7C0LKbCqH5a1VUoR6rjnIkXdwKBgFiH
d5lMPrJi7cJFh3B3cZJaOMG4/AcGJIPpkrQ3X7BZBaBiY08hd/9mZwBBqVzxKEGf
LfhF+CKc6RR1bx5JHV2y/A3V0I6kBFGHuXl/MdBEo+KjlqAnB+W+9gm6vHsBgfO3
AoGBAINSCjdRU8JnU/5ZMgPx7xrj9+RzBBpFJRy6K9FPJM9gL4zL0bBUCEXuE7S+
cXg8JBBLmxYBjEFQqH9rQ7PFO7JqJE4tD5VgRq4z5s3L7QYNWfBOClmtHxKdYIkZ
y7MEkWIPrk0oIvb5XdkajEBVoQ5K+7kQW2CUt0sK3TU/AoGBAOqAGNqxUaJ3FEhz
gZ6SN7xOk0X0nQVn8SRMQ7fxJ4S+7GVxbLM4QBh5Y2k7VBZJOrk5eOvBVg0MtJuQ
pKpB5DyF3X8X7UqDg0y1/qkxE5M7J5P6JFvE9j7JR9pXwbkh2P1iK4N8xMW+BLSQ
Qj8b3U/5F5UHR/qyLGCPuM6A
-----END RSA PRIVATE KEY-----"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_acm_certificate_imported() {
    // Use a unique isolated server so terraform can apply without
    // needing real certificate validation.
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("acm-imported");
    write_provider_tf(&dir, &url);

    let main_tf = format!(
        r#"
resource "aws_acm_certificate" "acm_certificate_imported" {{
  private_key       = <<-EOT
{key}
EOT
  certificate_body  = <<-EOT
{cert}
EOT
}}
"#,
        key = TEST_KEY_PEM,
        cert = TEST_CERT_PEM,
    );
    std::fs::write(dir.join("main.tf"), &main_tf).unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_acm_certificate"),
        "state missing acm certificate resource"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Request certificate with options block
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_acm_certificate_with_options() {
    let result = batch_apply(
        r#"
resource "aws_acm_certificate" "with_options" {
  domain_name       = "options-test.example.com"
  validation_method = "DNS"

  options {
    certificate_transparency_logging_preference = "ENABLED"
  }

  tags = {
    Name = "acm-options-test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("options-test.example.com"));
}

// ---------------------------------------------------------------------------
// Request certificate (DNS validation) tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_acm_certificate_request() {
    // Note: aws_acm_certificate with DNS validation waits for ISSUED status.
    // The winterbaume ACM handler returns ISSUED immediately, so this should
    // work without hanging.
    let result = batch_apply(
        r#"
resource "aws_acm_certificate" "acm_certificate_request" {
  domain_name       = "acm-request.example.com"
  validation_method = "DNS"

  tags = {
    Name = "acm-request-cert"
  }

  lifecycle {
    create_before_destroy = true
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("acm-request.example.com"));
}
