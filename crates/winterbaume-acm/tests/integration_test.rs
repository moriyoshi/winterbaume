use aws_sdk_acm::config::BehaviorVersion;
use aws_sdk_acm::types::{CertificateStatus, CertificateType, KeyAlgorithm, RenewalEligibility};
use winterbaume_acm::AcmService;
use winterbaume_core::MockAws;

const ACCOUNT_ID: &str = "123456789012";

// Test certificate resources (from moto test resources)
const RSA_2048_CRT: &[u8] = include_bytes!("resources/star_moto_com.pem");
const RSA_2048_KEY: &[u8] = include_bytes!("resources/star_moto_com.key");
const CA_CRT: &[u8] = include_bytes!("resources/ca.pem");
const EC_PRIME256V1_CRT: &[u8] = include_bytes!("resources/star_moto_com_ec_prime256v1.pem");
const EC_PRIME256V1_KEY: &[u8] = include_bytes!("resources/star_moto_com_ec_prime256v1.key");

const SERVER_COMMON_NAME: &str = "*.moto.com";
const CERT_AUTH_ARN: &str = "arn:aws:acm-pca:eu-central-1:123456789012:certificate-authority/12345678-1234-1234-1234-123456789012";

async fn make_acm_client() -> aws_sdk_acm::Client {
    make_acm_client_with_region("eu-central-1").await
}

async fn make_acm_client_with_region(region: &str) -> aws_sdk_acm::Client {
    let mock = MockAws::builder().with_service(AcmService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_acm::config::Region::new(region.to_owned()))
        .load()
        .await;

    aws_sdk_acm::Client::new(&config)
}

async fn import_cert(client: &aws_sdk_acm::Client) -> String {
    let resp = client
        .import_certificate()
        .certificate(aws_sdk_acm::primitives::Blob::new(RSA_2048_CRT))
        .private_key(aws_sdk_acm::primitives::Blob::new(RSA_2048_KEY))
        .certificate_chain(aws_sdk_acm::primitives::Blob::new(CA_CRT))
        .send()
        .await
        .expect("import_certificate should succeed");
    resp.certificate_arn().unwrap().to_string()
}

async fn pca_cert(client: &aws_sdk_acm::Client) -> String {
    let resp = client
        .request_certificate()
        .domain_name(SERVER_COMMON_NAME)
        .certificate_authority_arn(CERT_AUTH_ARN)
        .send()
        .await
        .expect("request_certificate with PCA should succeed");
    resp.certificate_arn().unwrap().to_string()
}

// ===== test_import_certificate (also tests GetCertificate) =====
#[tokio::test]
async fn test_import_certificate() {
    let client = make_acm_client().await;

    let resp = client
        .import_certificate()
        .certificate(aws_sdk_acm::primitives::Blob::new(RSA_2048_CRT))
        .private_key(aws_sdk_acm::primitives::Blob::new(RSA_2048_KEY))
        .certificate_chain(aws_sdk_acm::primitives::Blob::new(CA_CRT))
        .send()
        .await
        .expect("import_certificate should succeed");

    let arn = resp.certificate_arn().unwrap().to_string();

    let resp = client
        .get_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("get_certificate should succeed");

    let cert_pem = resp.certificate().expect("should have certificate body");
    assert_eq!(cert_pem, std::str::from_utf8(RSA_2048_CRT).unwrap());
    assert!(resp.certificate_chain().is_some());
}

// ===== test_import_certificate_with_tags =====
#[tokio::test]
async fn test_import_certificate_with_tags() {
    let client = make_acm_client().await;

    let resp = client
        .import_certificate()
        .certificate(aws_sdk_acm::primitives::Blob::new(RSA_2048_CRT))
        .private_key(aws_sdk_acm::primitives::Blob::new(RSA_2048_KEY))
        .certificate_chain(aws_sdk_acm::primitives::Blob::new(CA_CRT))
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("Environment")
                .value("QA")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("KeyOnly")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("import_certificate with tags should succeed");

    let arn = resp.certificate_arn().unwrap().to_string();

    let resp = client
        .get_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("get_certificate should succeed");
    assert_eq!(
        resp.certificate().unwrap(),
        std::str::from_utf8(RSA_2048_CRT).unwrap()
    );
    assert!(resp.certificate_chain().is_some());

    let resp = client
        .list_tags_for_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_certificate should succeed");

    let tags = resp.tags();
    let env_tag = tags.iter().find(|t| t.key() == "Environment");
    assert!(env_tag.is_some());
    assert_eq!(env_tag.unwrap().value(), Some("QA"));

    let key_only_tag = tags.iter().find(|t| t.key() == "KeyOnly");
    assert!(key_only_tag.is_some());
    assert_eq!(key_only_tag.unwrap().value(), None);
}

// ===== test_list_certificates =====
#[tokio::test]
async fn test_list_certificates() {
    let client = make_acm_client().await;
    let issued_arn = import_cert(&client).await;
    // FIX(terraform-e2e): request_certificate now returns ISSUED immediately, not PENDING_VALIDATION.
    let requested_arn = client
        .request_certificate()
        .domain_name("google.com")
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    // List all certificates
    let certs = client
        .list_certificates()
        .send()
        .await
        .expect("list_certificates should succeed")
        .certificate_summary_list()
        .to_vec();

    let arns: Vec<&str> = certs.iter().map(|c| c.certificate_arn().unwrap()).collect();
    assert!(arns.contains(&issued_arn.as_str()));
    assert!(arns.contains(&requested_arn.as_str()));

    for cert in &certs {
        assert!(cert.certificate_arn().is_some());
        assert!(cert.domain_name().is_some());
        assert!(cert.status().is_some());
        assert!(cert.r#type().is_some());
        assert!(cert.key_algorithm().is_some());
        assert!(cert.renewal_eligibility().is_some());
    }

    // Filter: EXPIRED and INACTIVE should return empty
    let certs = client
        .list_certificates()
        .certificate_statuses(CertificateStatus::Expired)
        .certificate_statuses(CertificateStatus::Inactive)
        .send()
        .await
        .unwrap()
        .certificate_summary_list()
        .to_vec();
    assert_eq!(certs.len(), 0);

    // Filter: PENDING_VALIDATION — mock immediately issues all certs, so this should be empty.
    let certs = client
        .list_certificates()
        .certificate_statuses(CertificateStatus::PendingValidation)
        .send()
        .await
        .unwrap()
        .certificate_summary_list()
        .to_vec();
    assert_eq!(certs.len(), 0);

    // Filter: ISSUED — both the imported cert and the requested cert should appear.
    let certs = client
        .list_certificates()
        .certificate_statuses(CertificateStatus::Issued)
        .send()
        .await
        .unwrap()
        .certificate_summary_list()
        .to_vec();
    let arns: Vec<&str> = certs.iter().map(|c| c.certificate_arn().unwrap()).collect();
    assert!(arns.contains(&issued_arn.as_str()));
    assert!(arns.contains(&requested_arn.as_str()));

    // Filter: ISSUED + PENDING_VALIDATION — same as ISSUED-only since mock has no pending certs.
    let certs = client
        .list_certificates()
        .certificate_statuses(CertificateStatus::Issued)
        .certificate_statuses(CertificateStatus::PendingValidation)
        .send()
        .await
        .unwrap()
        .certificate_summary_list()
        .to_vec();
    let arns: Vec<&str> = certs.iter().map(|c| c.certificate_arn().unwrap()).collect();
    assert!(arns.contains(&issued_arn.as_str()));
    assert!(arns.contains(&requested_arn.as_str()));
}

// ===== test_get_invalid_certificate =====
#[tokio::test]
async fn test_get_invalid_certificate() {
    let client = make_acm_client().await;
    let bad_arn = format!(
        "arn:aws:acm:us-east-2:{ACCOUNT_ID}:certificate/_0000000-0000-0000-0000-000000000000"
    );

    let result = client
        .get_certificate()
        .certificate_arn(&bad_arn)
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_delete_certificate (also tests deleting invalid certificate) =====
#[tokio::test]
async fn test_delete_certificate() {
    let client = make_acm_client().await;
    let arn = import_cert(&client).await;

    // Delete should succeed
    client
        .delete_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("delete_certificate should succeed");

    // Deleting again should fail with ResourceNotFoundException
    let result = client
        .delete_certificate()
        .certificate_arn(&arn)
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_describe_certificate =====
#[tokio::test]
async fn test_describe_certificate() {
    let client = make_acm_client().await;

    let resp = client
        .request_certificate()
        .domain_name(SERVER_COMMON_NAME)
        .send()
        .await
        .unwrap();
    let arn = resp.certificate_arn().unwrap().to_string();

    let resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("describe_certificate should succeed");

    let cert = resp.certificate().expect("should have certificate");
    assert_eq!(cert.certificate_arn(), Some(arn.as_str()));
    assert_eq!(cert.domain_name(), Some(SERVER_COMMON_NAME));
    assert_eq!(cert.issuer(), Some("Amazon"));
    assert_eq!(cert.key_algorithm(), Some(&KeyAlgorithm::Rsa2048));
    // FIX(terraform-e2e): mock immediately issues certs; expect ISSUED not PENDING_VALIDATION.
    assert_eq!(cert.status(), Some(&CertificateStatus::Issued));
    assert_eq!(cert.r#type(), Some(&CertificateType::AmazonIssued));
    assert_eq!(
        cert.renewal_eligibility(),
        Some(&RenewalEligibility::Ineligible)
    );
    assert!(cert.options().is_some());

    let dvos = cert.domain_validation_options();
    assert_eq!(dvos.len(), 1);
    assert_eq!(dvos[0].domain_name(), SERVER_COMMON_NAME);
}

// ===== test_describe_certificate_with_bad_arn =====
#[tokio::test]
async fn test_describe_certificate_with_bad_arn() {
    let client = make_acm_client().await;
    let bad_arn = format!(
        "arn:aws:acm:us-east-2:{ACCOUNT_ID}:certificate/_0000000-0000-0000-0000-000000000000"
    );

    let result = client
        .describe_certificate()
        .certificate_arn(&bad_arn)
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_describe_certificate_with_imported_cert =====
#[tokio::test]
async fn test_describe_certificate_with_imported_cert() {
    let client = make_acm_client().await;
    let arn = import_cert(&client).await;

    let resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("describe_certificate should succeed");

    let cert = resp.certificate().expect("should have certificate");
    assert_eq!(cert.certificate_arn(), Some(arn.as_str()));
    assert_eq!(cert.domain_name(), Some(SERVER_COMMON_NAME));
    assert_eq!(cert.issuer(), Some("Moto"));
    assert_eq!(cert.key_algorithm(), Some(&KeyAlgorithm::Rsa2048));
    assert_eq!(cert.status(), Some(&CertificateStatus::Issued));
    assert_eq!(cert.r#type(), Some(&CertificateType::Imported));
    assert_eq!(
        cert.renewal_eligibility(),
        Some(&RenewalEligibility::Ineligible)
    );
    assert!(cert.options().is_some());

    // Imported certs should NOT have DomainValidationOptions
    assert!(cert.domain_validation_options().is_empty());
}

// ===== test_describe_certificate_with_pca_cert =====
#[tokio::test]
async fn test_describe_certificate_with_pca_cert() {
    let client = make_acm_client().await;
    let arn = pca_cert(&client).await;

    let resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("describe_certificate should succeed");

    let cert = resp.certificate().expect("should have certificate");
    assert_eq!(cert.certificate_arn(), Some(arn.as_str()));
    assert_eq!(cert.domain_name(), Some(SERVER_COMMON_NAME));
    assert_eq!(cert.issuer(), Some("Amazon"));
    assert_eq!(cert.key_algorithm(), Some(&KeyAlgorithm::Rsa2048));
    assert_eq!(cert.status(), Some(&CertificateStatus::Issued));
    assert_eq!(cert.r#type(), Some(&CertificateType::Private));
    assert_eq!(
        cert.renewal_eligibility(),
        Some(&RenewalEligibility::Ineligible)
    );
    assert!(cert.options().is_some());
    assert_eq!(cert.certificate_authority_arn(), Some(CERT_AUTH_ARN));

    // PCA certs should NOT have DomainValidationOptions
    assert!(cert.domain_validation_options().is_empty());
}

// ===== test_export_certificate_with_pca_cert =====
#[tokio::test]
async fn test_export_certificate_with_pca_cert() {
    let client = make_acm_client().await;
    let arn = pca_cert(&client).await;

    let resp = client
        .export_certificate()
        .certificate_arn(&arn)
        .passphrase(aws_sdk_acm::primitives::Blob::new("passphrase"))
        .send()
        .await
        .expect("export_certificate should succeed");

    assert!(resp.certificate().is_some());
    assert!(resp.certificate_chain().is_some());
    assert!(resp.private_key().is_some());
}

// ===== test_export_certificate_with_imported_cert =====
#[tokio::test]
async fn test_export_certificate_with_imported_cert() {
    let client = make_acm_client().await;
    let arn = import_cert(&client).await;

    let result = client
        .export_certificate()
        .certificate_arn(&arn)
        .passphrase(aws_sdk_acm::primitives::Blob::new("passphrase"))
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err
        .as_service_error()
        .expect("should be service error")
        .meta();
    assert_eq!(meta.code(), Some("ValidationException"));
    assert!(
        meta.message()
            .unwrap_or("")
            .contains("is not a private certificate")
    );
}

// ===== test_export_certificate_with_bad_arn =====
#[tokio::test]
async fn test_export_certificate_with_bad_arn() {
    let client = make_acm_client().await;
    let bad_arn = format!(
        "arn:aws:acm:us-east-2:{ACCOUNT_ID}:certificate/_0000000-0000-0000-0000-000000000000"
    );

    let result = client
        .export_certificate()
        .certificate_arn(&bad_arn)
        .passphrase(aws_sdk_acm::primitives::Blob::new("pass"))
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_add_tags_to_certificate (also tests ListTagsForCertificate) =====
#[tokio::test]
async fn test_add_tags_to_certificate() {
    let client = make_acm_client().await;
    let arn = import_cert(&client).await;

    client
        .add_tags_to_certificate()
        .certificate_arn(&arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key1")
                .value("value1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags_to_certificate should succeed");

    let resp = client
        .list_tags_for_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_certificate should succeed");

    let tags = resp.tags();
    let key1_tag = tags.iter().find(|t| t.key() == "key1");
    assert!(key1_tag.is_some());
    assert_eq!(key1_tag.unwrap().value(), Some("value1"));

    let key2_tag = tags.iter().find(|t| t.key() == "key2");
    assert!(key2_tag.is_some());
    // Key-only tag should not have a value
    assert_eq!(key2_tag.unwrap().value(), None);
}

// ===== test_add_tags_to_invalid_certificate =====
#[tokio::test]
async fn test_add_tags_to_invalid_certificate() {
    let client = make_acm_client().await;
    let bad_arn = format!(
        "arn:aws:acm:us-east-2:{ACCOUNT_ID}:certificate/_0000000-0000-0000-0000-000000000000"
    );

    let result = client
        .add_tags_to_certificate()
        .certificate_arn(&bad_arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key1")
                .value("value1")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_list_tags_for_invalid_certificate =====
#[tokio::test]
async fn test_list_tags_for_invalid_certificate() {
    let client = make_acm_client().await;
    let bad_arn = format!(
        "arn:aws:acm:us-east-2:{ACCOUNT_ID}:certificate/_0000000-0000-0000-0000-000000000000"
    );

    let result = client
        .list_tags_for_certificate()
        .certificate_arn(&bad_arn)
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_remove_tags_from_certificate =====
#[tokio::test]
async fn test_remove_tags_from_certificate() {
    let client = make_acm_client().await;
    let arn = import_cert(&client).await;

    // Add tags
    client
        .add_tags_to_certificate()
        .certificate_arn(&arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key1")
                .value("value1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key2")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key3")
                .value("value3")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key4")
                .value("value4")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Remove tags with various matching scenarios
    client
        .remove_tags_from_certificate()
        .certificate_arn(&arn)
        .tags(
            // key1 with wrong value should NOT remove
            aws_sdk_acm::types::Tag::builder()
                .key("key1")
                .value("value2")
                .build()
                .unwrap(),
        )
        .tags(
            // key2 single key removal (no value)
            aws_sdk_acm::types::Tag::builder()
                .key("key2")
                .build()
                .unwrap(),
        )
        .tags(
            // key3 exact match removal
            aws_sdk_acm::types::Tag::builder()
                .key("key3")
                .value("value3")
                .build()
                .unwrap(),
        )
        .tags(
            // key4 partial match removal (key only)
            aws_sdk_acm::types::Tag::builder()
                .key("key4")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("remove_tags_from_certificate should succeed");

    let resp = client
        .list_tags_for_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    let keys: Vec<&str> = tags.iter().map(|t| t.key.as_str()).collect();

    // key2, key3, key4 should be removed
    assert!(!keys.contains(&"key2"));
    assert!(!keys.contains(&"key3"));
    assert!(!keys.contains(&"key4"));

    // key1 should remain (value didn't match)
    assert!(keys.contains(&"key1"));
}

// ===== test_remove_tags_from_invalid_certificate =====
#[tokio::test]
async fn test_remove_tags_from_invalid_certificate() {
    let client = make_acm_client().await;
    let bad_arn = format!(
        "arn:aws:acm:us-east-2:{ACCOUNT_ID}:certificate/_0000000-0000-0000-0000-000000000000"
    );

    let result = client
        .remove_tags_from_certificate()
        .certificate_arn(&bad_arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("key1")
                .value("value1")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_request_certificate =====
#[tokio::test]
async fn test_request_certificate() {
    let client = make_acm_client().await;

    let resp = client
        .request_certificate()
        .domain_name("google.com")
        .subject_alternative_names("google.com")
        .subject_alternative_names("www.google.com")
        .subject_alternative_names("mail.google.com")
        .send()
        .await
        .expect("request_certificate should succeed");

    let arn = resp.certificate_arn().expect("should have certificate ARN");
    assert!(arn.contains(&format!(
        "arn:aws:acm:eu-central-1:{ACCOUNT_ID}:certificate/"
    )));
}

// ===== test_request_certificate_no_san =====
#[tokio::test]
async fn test_request_certificate_no_san() {
    let client = make_acm_client().await;

    let resp = client
        .request_certificate()
        .domain_name("google.com")
        .send()
        .await
        .expect("request_certificate should succeed");

    let arn = resp.certificate_arn().unwrap().to_string();

    let resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();

    let cert = resp.certificate().unwrap();
    assert_eq!(
        cert.renewal_eligibility(),
        Some(&RenewalEligibility::Ineligible)
    );
    assert!(cert.options().is_some());

    let dvos = cert.domain_validation_options();
    assert_eq!(dvos.len(), 1);
    assert_eq!(dvos[0].domain_name(), "google.com");
    assert_eq!(dvos[0].validation_domain(), Some("google.com"));
}

// ===== test_request_certificate_with_sans =====
#[tokio::test]
async fn test_request_certificate_with_sans() {
    let client = make_acm_client().await;

    let resp = client
        .request_certificate()
        .domain_name("google.com")
        .subject_alternative_names("google.com")
        .subject_alternative_names("www.google.com")
        .subject_alternative_names("mail.google.com")
        .send()
        .await
        .expect("request_certificate with SANs should succeed");

    let arn = resp.certificate_arn().unwrap().to_string();

    let desc = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();

    let cert = desc.certificate().unwrap();
    assert_eq!(cert.subject_alternative_names().len(), 3);

    let dvos = cert.domain_validation_options();
    assert_eq!(dvos.len(), 3);
}

// ===== test_request_certificate_with_certificate_authority =====
#[tokio::test]
async fn test_request_certificate_with_certificate_authority() {
    let client = make_acm_client().await;

    let resp = client
        .request_certificate()
        .domain_name("example.com")
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-central-1:123456789012:certificate-authority/12345678-1234-1234-1234-123456789012",
        )
        .send()
        .await
        .expect("request_certificate with CA ARN should succeed");

    assert!(resp.certificate_arn().is_some());
}

// ===== test_delete_nonexistent_certificate =====
#[tokio::test]
async fn test_delete_nonexistent_certificate() {
    let client = make_acm_client().await;

    let result = client
        .delete_certificate()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/nonexistent")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(service_err.is_resource_not_found_exception());
}

// ===== test_list_certificates_with_key_types_filter =====
#[tokio::test]
async fn test_list_certificates_with_key_types_filter() {
    let client = make_acm_client_with_region("us-east-1").await;
    let arn = import_cert(&client).await;

    // Should find the RSA_2048 cert when filtering for RSA_2048
    let certs = client
        .list_certificates()
        .includes(
            aws_sdk_acm::types::Filters::builder()
                .key_types(KeyAlgorithm::Rsa2048)
                .key_types(KeyAlgorithm::Rsa4096)
                .key_types(KeyAlgorithm::EcPrime256v1)
                .build(),
        )
        .send()
        .await
        .unwrap()
        .certificate_summary_list()
        .to_vec();

    assert_eq!(certs.len(), 1);
    assert_eq!(certs[0].certificate_arn().unwrap(), arn);

    // Should NOT find the RSA_2048 cert when filtering for RSA_1024
    let certs = client
        .list_certificates()
        .includes(
            aws_sdk_acm::types::Filters::builder()
                .key_types(KeyAlgorithm::Rsa1024)
                .build(),
        )
        .send()
        .await
        .unwrap()
        .certificate_summary_list()
        .to_vec();

    assert_eq!(certs.len(), 0);
}

// ===== test_import_ec_certificate =====
#[tokio::test]
async fn test_import_ec_certificate() {
    let client = make_acm_client_with_region("ca-central-1").await;

    let resp = client
        .import_certificate()
        .certificate(aws_sdk_acm::primitives::Blob::new(EC_PRIME256V1_CRT))
        .private_key(aws_sdk_acm::primitives::Blob::new(EC_PRIME256V1_KEY))
        .certificate_chain(aws_sdk_acm::primitives::Blob::new(CA_CRT))
        .send()
        .await
        .expect("import EC certificate should succeed");

    let arn = resp.certificate_arn().unwrap().to_string();

    let resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();

    let cert = resp.certificate().unwrap();
    assert_eq!(cert.key_algorithm(), Some(&KeyAlgorithm::EcPrime256v1));
    assert_eq!(cert.domain_name(), Some(SERVER_COMMON_NAME));
}

// ===== test_operations_with_invalid_tags =====
#[tokio::test]
async fn test_operations_with_invalid_tags() {
    let client = make_acm_client().await;

    // request certificate with too-long key
    let result = client
        .request_certificate()
        .domain_name("example.com")
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("X".repeat(200))
                .value("Valid")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err
        .as_service_error()
        .expect("should be service error")
        .meta();
    assert_eq!(meta.code(), Some("ValidationException"));
    assert!(
        meta.message()
            .unwrap_or("")
            .contains("Member must have length less than or equal to 128")
    );

    // add tags with aws: prefix
    let arn = import_cert(&client).await;
    let result = client
        .add_tags_to_certificate()
        .certificate_arn(&arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("aws:xxx")
                .value("Valid")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err
        .as_service_error()
        .expect("should be service error")
        .meta();
    assert_eq!(meta.code(), Some("ValidationException"));
    assert!(
        meta.message()
            .unwrap_or("")
            .contains("AWS internal tags cannot be changed with this API")
    );

    // remove tags with aws: prefix
    let result = client
        .remove_tags_from_certificate()
        .certificate_arn(&arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("aws:xxx")
                .value("Valid")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err
        .as_service_error()
        .expect("should be service error")
        .meta();
    assert_eq!(meta.code(), Some("ValidationException"));
    assert!(
        meta.message()
            .unwrap_or("")
            .contains("AWS internal tags cannot be changed with this API")
    );
}

// ===== test_add_too_many_tags =====
#[tokio::test]
async fn test_add_too_many_tags() {
    let client = make_acm_client().await;
    let arn = import_cert(&client).await;

    // Try to add 51 tags at once
    let mut req = client.add_tags_to_certificate().certificate_arn(&arn);
    for i in 1..=51 {
        req = req.tags(
            aws_sdk_acm::types::Tag::builder()
                .key(format!("a-{i}"))
                .value("abcd")
                .build()
                .unwrap(),
        );
    }
    let result = req.send().await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err
        .as_service_error()
        .expect("should be service error")
        .meta();
    assert_eq!(meta.code(), Some("TooManyTagsException"));

    // Verify no tags were added
    let tags = client
        .list_tags_for_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 0);
}

#[tokio::test]
async fn test_get_account_configuration_default() {
    let client = make_acm_client().await;

    let resp = client
        .get_account_configuration()
        .send()
        .await
        .expect("get_account_configuration should succeed");

    let expiry_events = resp.expiry_events().expect("should have expiry events");
    assert_eq!(expiry_events.days_before_expiry(), Some(45));
}

#[tokio::test]
async fn test_put_and_get_account_configuration() {
    let client = make_acm_client().await;

    client
        .put_account_configuration()
        .expiry_events(
            aws_sdk_acm::types::ExpiryEventsConfiguration::builder()
                .days_before_expiry(30)
                .build(),
        )
        .idempotency_token("test-token")
        .send()
        .await
        .expect("put_account_configuration should succeed");

    let resp = client
        .get_account_configuration()
        .send()
        .await
        .expect("get_account_configuration should succeed");

    let expiry_events = resp.expiry_events().expect("should have expiry events");
    assert_eq!(expiry_events.days_before_expiry(), Some(30));
}

// ============================================================================
// Ported from moto: test_acm.py
// ============================================================================

// Ported from moto: test_acm.py::test_export_certificate_with_short_passphrase
#[tokio::test]
async fn test_export_certificate_with_short_passphrase() {
    let client = make_acm_client().await;
    let arn = pca_cert(&client).await;

    let result = client
        .export_certificate()
        .certificate_arn(&arn)
        .passphrase(aws_sdk_acm::primitives::Blob::new(""))
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err
        .as_service_error()
        .expect("should be service error")
        .meta();
    assert_eq!(meta.code(), Some("ValidationException"));
    assert!(
        meta.message().unwrap_or("").contains("passphrase"),
        "Error message should mention passphrase, got: {:?}",
        meta.message()
    );
}

// Ported from moto: test_acm.py::test_import_certificate_with_invalid_value_tags
#[tokio::test]
async fn test_import_certificate_with_too_long_value_tags() {
    let client = make_acm_client().await;

    let result = client
        .import_certificate()
        .certificate(aws_sdk_acm::primitives::Blob::new(RSA_2048_CRT))
        .private_key(aws_sdk_acm::primitives::Blob::new(RSA_2048_KEY))
        .certificate_chain(aws_sdk_acm::primitives::Blob::new(CA_CRT))
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("Valid")
                .value("X".repeat(300))
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("aws:xx")
                .value("Valid")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err
        .as_service_error()
        .expect("should be service error")
        .meta();
    assert_eq!(meta.code(), Some("ValidationException"));
    assert!(
        meta.message()
            .unwrap_or("")
            .contains("Member must have length less than or equal to 256"),
        "Error message should mention value length constraint, got: {:?}",
        meta.message()
    );
}

// Ported from moto: test_acm.py::test_request_certificate
// Verifies idempotency token returns same ARN for identical requests
#[tokio::test]
async fn test_request_certificate_idempotency() {
    let client = make_acm_client().await;

    let resp1 = client
        .request_certificate()
        .domain_name("google.com")
        .idempotency_token("test-idemp-token")
        .subject_alternative_names("google.com")
        .subject_alternative_names("www.google.com")
        .subject_alternative_names("mail.google.com")
        .send()
        .await
        .expect("first request_certificate should succeed");

    let arn1 = resp1.certificate_arn().unwrap().to_string();
    assert!(arn1.contains("arn:aws:acm:eu-central-1:"));

    let resp2 = client
        .request_certificate()
        .domain_name("google.com")
        .idempotency_token("test-idemp-token")
        .subject_alternative_names("google.com")
        .subject_alternative_names("www.google.com")
        .subject_alternative_names("mail.google.com")
        .send()
        .await
        .expect("second request_certificate should succeed (idempotent)");

    let arn2 = resp2.certificate_arn().unwrap().to_string();
    assert_eq!(arn1, arn2, "Same idempotency token should return same ARN");
}

// Ported from moto: test_acm.py::test_request_certificate_with_sans
// Verifies SubjectAlternativeNames in describe output
#[tokio::test]
async fn test_request_certificate_with_sans_describe() {
    let client = make_acm_client().await;

    let resp = client
        .request_certificate()
        .domain_name("google.com")
        .subject_alternative_names("google.com")
        .subject_alternative_names("www.google.com")
        .subject_alternative_names("mail.google.com")
        .send()
        .await
        .unwrap();

    let arn = resp.certificate_arn().unwrap();
    let desc = client
        .describe_certificate()
        .certificate_arn(arn)
        .send()
        .await
        .unwrap();

    let cert = desc.certificate().unwrap();
    assert_eq!(cert.subject_alternative_names().len(), 3);
    let san_names: Vec<&str> = cert
        .subject_alternative_names()
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert!(san_names.contains(&"google.com"));
    assert!(san_names.contains(&"www.google.com"));
    assert!(san_names.contains(&"mail.google.com"));

    let validation_opts = cert.domain_validation_options();
    assert_eq!(validation_opts.len(), 3);
}

// Ported from moto: test_acm.py::test_describe_certificate
// Verifies full describe output fields for a requested certificate
#[tokio::test]
async fn test_describe_certificate_request_fields() {
    let client = make_acm_client().await;

    let resp = client
        .request_certificate()
        .domain_name(SERVER_COMMON_NAME)
        .send()
        .await
        .unwrap();
    let arn = resp.certificate_arn().unwrap();

    let desc = client
        .describe_certificate()
        .certificate_arn(arn)
        .send()
        .await
        .unwrap();
    let cert = desc.certificate().unwrap();

    assert_eq!(cert.certificate_arn(), Some(arn));
    assert_eq!(cert.domain_name(), Some(SERVER_COMMON_NAME));
    assert_eq!(cert.issuer(), Some("Amazon"));
    assert_eq!(cert.key_algorithm(), Some(&KeyAlgorithm::Rsa2048));
    // FIX(terraform-e2e): mock immediately issues certs; expect ISSUED not PENDING_VALIDATION.
    assert_eq!(cert.status(), Some(&CertificateStatus::Issued));
    assert_eq!(cert.r#type(), Some(&CertificateType::AmazonIssued));
    assert_eq!(
        cert.renewal_eligibility(),
        Some(&RenewalEligibility::Ineligible)
    );
    assert!(cert.options().is_some());

    let validation_opts = cert.domain_validation_options();
    assert_eq!(validation_opts.len(), 1);
    assert_eq!(validation_opts[0].domain_name(), SERVER_COMMON_NAME);
}

// ============================================================================
// Tests derived from AWS documentation: AWS Certificate Manager (ACM)
// ============================================================================

#[tokio::test]
async fn test_list_certificates_empty() {
    let client = make_acm_client().await;

    let resp = client
        .list_certificates()
        .send()
        .await
        .expect("list_certificates on empty store should succeed");

    assert_eq!(resp.certificate_summary_list().len(), 0);
}

#[tokio::test]
async fn test_get_certificate_pending_validation_returns_error() {
    // A certificate that was requested (not imported) and is PENDING_VALIDATION
    // should return RequestInProgressException when GetCertificate is called.
    let client = make_acm_client().await;

    let arn = client
        .request_certificate()
        .domain_name("pending.example.com")
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    let err = client
        .get_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect_err("GetCertificate on a requested-but-unvalidated cert must error");
    let svc = err.into_service_error();
    assert!(
        svc.is_request_in_progress_exception(),
        "expected RequestInProgressException, got {svc:?}",
    );
}

#[tokio::test]
async fn test_describe_certificate_timing_fields() {
    let client = make_acm_client().await;

    let arn = import_cert(&client).await;

    let resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();

    let cert = resp.certificate().unwrap();
    assert!(
        cert.not_before().is_some(),
        "not_before should be set for imported cert"
    );
    assert!(
        cert.not_after().is_some(),
        "not_after should be set for imported cert"
    );
}

#[tokio::test]
async fn test_put_account_configuration_invalid_days() {
    let client = make_acm_client().await;

    let result = client
        .put_account_configuration()
        .expiry_events(
            aws_sdk_acm::types::ExpiryEventsConfiguration::builder()
                .days_before_expiry(0)
                .build(),
        )
        .idempotency_token("test-token")
        .send()
        .await
        .expect_err("DaysBeforeExpiry=0 must reject as AWS does");
    let svc = result.into_service_error();
    assert!(
        svc.is_validation_exception(),
        "expected ValidationException, got {svc:?}",
    );
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): RequestCertificate returns certificate with ISSUED status
// (state.rs:63) — terraform waits for certificates to reach ISSUED status.
// Return ISSUED immediately since real DNS/email validation is not possible.
#[tokio::test]
async fn test_certificate_immediately_issued() {
    let client = make_acm_client().await;

    let req_resp = client
        .request_certificate()
        .domain_name("test.example.com")
        .send()
        .await
        .expect("request_certificate should succeed");

    let arn = req_resp.certificate_arn().unwrap().to_string();

    let desc_resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("describe_certificate should succeed");

    let cert = desc_resp.certificate().expect("should have certificate");
    assert_eq!(
        cert.status(),
        Some(&CertificateStatus::Issued),
        "certificate should be ISSUED immediately"
    );
    assert_eq!(
        cert.r#type(),
        Some(&CertificateType::AmazonIssued),
        "certificate type should be AMAZON_ISSUED"
    );
}

// Covers FIX(terraform-e2e): PCA certificate also gets ISSUED status immediately
#[tokio::test]
async fn test_pca_certificate_immediately_issued() {
    let client = make_acm_client().await;

    let arn = pca_cert(&client).await;

    let desc_resp = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("describe_certificate should succeed");

    let cert = desc_resp.certificate().expect("should have certificate");
    assert_eq!(
        cert.status(),
        Some(&CertificateStatus::Issued),
        "PCA certificate should be ISSUED immediately"
    );
    assert_eq!(
        cert.r#type(),
        Some(&CertificateType::Private),
        "PCA certificate type should be PRIVATE"
    );
}
