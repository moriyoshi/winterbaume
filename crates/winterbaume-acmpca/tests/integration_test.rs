use aws_sdk_acmpca::config::BehaviorVersion;
use aws_sdk_acmpca::primitives::Blob;
use winterbaume_acmpca::AcmPcaService;
use winterbaume_core::MockAws;

async fn make_client(region: &str) -> aws_sdk_acmpca::Client {
    let mock = MockAws::builder()
        .with_service(AcmPcaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_acmpca::config::Region::new(region.to_owned()))
        .load()
        .await;

    aws_sdk_acmpca::Client::new(&config)
}

// --- test_create_certificate_authority ---
#[tokio::test]
async fn test_create_certificate_authority_aws_partition() {
    let client = make_client("eu-west-1").await;
    let resp = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .idempotency_token("terraform-20221125230308947400000001")
        .send()
        .await
        .expect("create_certificate_authority should succeed");

    let arn = resp.certificate_authority_arn().unwrap();
    assert!(
        arn.contains("arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/"),
        "ARN should contain correct partition, region, account: {}",
        arn
    );
}

#[tokio::test]
async fn test_create_certificate_authority_gov_partition() {
    let client = make_client("us-gov-east-1").await;
    let resp = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .idempotency_token("terraform-20221125230308947400000001")
        .send()
        .await
        .expect("create_certificate_authority should succeed");

    let arn = resp.certificate_authority_arn().unwrap();
    assert!(
        arn.contains("arn:aws-us-gov:acm-pca:us-gov-east-1:123456789012:certificate-authority/"),
        "ARN should contain gov partition: {}",
        arn
    );
}

// --- test_describe_certificate_authority ---
#[tokio::test]
async fn test_describe_certificate_authority() {
    let client = make_client("ap-southeast-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .idempotency_token("terraform-20221125230308947400000001")
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let resp = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("describe should succeed");

    let ca = resp.certificate_authority().unwrap();
    assert_eq!(ca.arn(), Some(ca_arn.as_str()));
    assert_eq!(ca.owner_account(), Some("123456789012"));
    assert!(ca.created_at().is_some());
    assert_eq!(
        ca.r#type(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
    );
    assert_eq!(
        ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::PendingCertificate)
    );

    let config = ca.certificate_authority_configuration().unwrap();
    assert_eq!(
        config.key_algorithm(),
        &aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096
    );
    assert_eq!(
        config.signing_algorithm(),
        &aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa
    );
    assert_eq!(
        config.subject().unwrap().common_name(),
        Some("yscb41lw.test")
    );
    assert_eq!(
        ca.key_storage_security_standard(),
        Some(&aws_sdk_acmpca::types::KeyStorageSecurityStandard::Fips1402Level3OrHigher)
    );
}

// --- test_describe_certificate_authority_with_security_standard ---
#[tokio::test]
async fn test_describe_certificate_authority_with_security_standard() {
    let client = make_client("ap-southeast-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .key_storage_security_standard(
            aws_sdk_acmpca::types::KeyStorageSecurityStandard::Fips1402Level2OrHigher,
        )
        .idempotency_token("terraform-20221125230308947400000001")
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let resp = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap();

    let ca = resp.certificate_authority().unwrap();
    assert_eq!(
        ca.key_storage_security_standard(),
        Some(&aws_sdk_acmpca::types::KeyStorageSecurityStandard::Fips1402Level2OrHigher)
    );
}

// --- test_describe_unknown_certificate_authority ---
#[tokio::test]
async fn test_describe_unknown_certificate_authority() {
    let client = make_client("ap-southeast-1").await;
    let result = client
        .describe_certificate_authority()
        .certificate_authority_arn("unknown")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(
        service_err.is_resource_not_found_exception(),
        "expected ResourceNotFoundException"
    );
}

// --- test_get_certificate_authority_certificate (before import should fail) ---
#[tokio::test]
async fn test_get_certificate_authority_certificate_before_import() {
    let client = make_client("ap-southeast-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let result = client
        .get_certificate_authority_certificate()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(
        service_err.is_invalid_state_exception(),
        "expected InvalidStateException, got {:?}",
        service_err
    );
}

// --- test_get_certificate_authority_csr ---
#[tokio::test]
async fn test_get_certificate_authority_csr() {
    let client = make_client("us-east-2").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let resp = client
        .get_certificate_authority_csr()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("get_csr should succeed");

    let csr = resp.csr().unwrap();
    assert!(
        csr.contains("-----BEGIN CERTIFICATE REQUEST-----"),
        "CSR should be PEM format, got: {}",
        &csr[..50.min(csr.len())]
    );
}

// --- test_list_tags_when_ca_has_no_tags ---
#[tokio::test]
async fn test_list_tags_when_ca_has_no_tags() {
    let client = make_client("us-east-2").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap();

    assert!(resp.tags().is_empty(), "tags should be empty");
}

// --- test_list_tags ---
#[tokio::test]
async fn test_list_tags() {
    let client = make_client("us-east-2").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("t1")
                .value("v1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("t2")
                .value("v2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].key(), "t1");
    assert_eq!(tags[0].value(), Some("v1"));
    assert_eq!(tags[1].key(), "t2");
    assert_eq!(tags[1].value(), Some("v2"));
}

// --- test_update_certificate_authority ---
#[tokio::test]
async fn test_update_certificate_authority_status() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    client
        .update_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .status(aws_sdk_acmpca::types::CertificateAuthorityStatus::Disabled)
        .send()
        .await
        .expect("update should succeed");

    let ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();

    assert_eq!(
        ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::Disabled)
    );
    assert!(ca.last_state_change_at().is_some());
}

// --- test_update_certificate_authority with revocation config ---
#[tokio::test]
async fn test_update_certificate_authority_revocation_config() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    // Set revocation config with CRL enabled (default S3ObjectAcl = PUBLIC_READ)
    client
        .update_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .revocation_configuration(
            aws_sdk_acmpca::types::RevocationConfiguration::builder()
                .crl_configuration(
                    aws_sdk_acmpca::types::CrlConfiguration::builder()
                        .enabled(true)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();

    let crl = ca
        .revocation_configuration()
        .unwrap()
        .crl_configuration()
        .unwrap();
    assert!(crl.enabled());
    assert_eq!(
        crl.s3_object_acl(),
        Some(&aws_sdk_acmpca::types::S3ObjectAcl::PublicRead)
    );

    // Update with BUCKET_OWNER_FULL_CONTROL
    client
        .update_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .revocation_configuration(
            aws_sdk_acmpca::types::RevocationConfiguration::builder()
                .crl_configuration(
                    aws_sdk_acmpca::types::CrlConfiguration::builder()
                        .enabled(true)
                        .s3_object_acl(aws_sdk_acmpca::types::S3ObjectAcl::BucketOwnerFullControl)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ca2 = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();

    let crl2 = ca2
        .revocation_configuration()
        .unwrap()
        .crl_configuration()
        .unwrap();
    assert_eq!(
        crl2.s3_object_acl(),
        Some(&aws_sdk_acmpca::types::S3ObjectAcl::BucketOwnerFullControl)
    );
}

// --- test_delete_certificate_authority ---
#[tokio::test]
async fn test_delete_certificate_authority() {
    let client = make_client("ap-southeast-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    client
        .delete_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("delete should succeed");

    let ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();

    assert_eq!(
        ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::Deleted)
    );
}

// --- test_issue_certificate ---
#[tokio::test]
async fn test_issue_certificate() {
    let client = make_client("ap-southeast-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("t8fzth32.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let csr = client
        .get_certificate_authority_csr()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .csr()
        .unwrap()
        .to_string();

    let resp = client
        .issue_certificate()
        .certificate_authority_arn(&ca_arn)
        .csr(Blob::new(csr.as_bytes()))
        .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
        .validity(
            aws_sdk_acmpca::types::Validity::builder()
                .r#type(aws_sdk_acmpca::types::ValidityPeriodType::Years)
                .value(10)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("issue_certificate should succeed");

    let cert_arn = resp.certificate_arn().unwrap();
    assert!(
        cert_arn.starts_with("arn:aws:acm-pca:ap-southeast-1:123456789012:certificate-authority"),
        "cert ARN should have correct prefix: {}",
        cert_arn
    );
}

// --- test_get_certificate ---
#[tokio::test]
async fn test_get_certificate() {
    let client = make_client("us-east-2").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("t8fzth32.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let csr = client
        .get_certificate_authority_csr()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .csr()
        .unwrap()
        .to_string();

    let cert_arn = client
        .issue_certificate()
        .certificate_authority_arn(&ca_arn)
        .csr(Blob::new(csr.as_bytes()))
        .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
        .validity(
            aws_sdk_acmpca::types::Validity::builder()
                .r#type(aws_sdk_acmpca::types::ValidityPeriodType::Years)
                .value(10)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    let resp = client
        .get_certificate()
        .certificate_authority_arn(&ca_arn)
        .certificate_arn(&cert_arn)
        .send()
        .await
        .expect("get_certificate should succeed");

    let cert = resp.certificate().unwrap();
    assert!(
        cert.contains("-----BEGIN CERTIFICATE-----"),
        "should be PEM format"
    );
}

// --- test_tag_certificate_authority ---
#[tokio::test]
async fn test_tag_certificate_authority() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    client
        .tag_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("t1")
                .value("v1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("t2")
                .value("v2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].key(), "t1");
    assert_eq!(tags[0].value(), Some("v1"));
    assert_eq!(tags[1].key(), "t2");
    assert_eq!(tags[1].value(), Some("v2"));
}

// --- test_untag_certificate_authority ---
#[tokio::test]
async fn test_untag_certificate_authority() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    client
        .tag_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("t1")
                .value("v1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("t2")
                .value("v2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("t1")
                .value("v1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "t2");
    assert_eq!(tags[0].value(), Some("v2"));
}

// --- test_import_certificate_authority_certificate ---
#[tokio::test]
async fn test_import_certificate_authority_certificate_invalid() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let result = client
        .import_certificate_authority_certificate()
        .certificate_authority_arn(&ca_arn)
        .certificate(Blob::new(b"invalid certificate pem"))
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(
        service_err.is_malformed_certificate_exception(),
        "expected MalformedCertificateException, got {:?}",
        service_err
    );
}

// --- test_import and then get_certificate_authority_certificate ---
#[tokio::test]
async fn test_import_and_get_certificate_authority_certificate() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("yscb41lw.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    // Issue a root CA certificate
    let csr = client
        .get_certificate_authority_csr()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .csr()
        .unwrap()
        .to_string();

    let cert_arn = client
        .issue_certificate()
        .certificate_authority_arn(&ca_arn)
        .csr(Blob::new(csr.as_bytes()))
        .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
        .template_arn("arn:aws:acm-pca:::template/RootCACertificate/V1")
        .validity(
            aws_sdk_acmpca::types::Validity::builder()
                .r#type(aws_sdk_acmpca::types::ValidityPeriodType::Years)
                .value(10)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    let ca_cert = client
        .get_certificate()
        .certificate_authority_arn(&ca_arn)
        .certificate_arn(&cert_arn)
        .send()
        .await
        .unwrap()
        .certificate()
        .unwrap()
        .to_string();

    // Import the certificate
    client
        .import_certificate_authority_certificate()
        .certificate_authority_arn(&ca_arn)
        .certificate(Blob::new(ca_cert.as_bytes()))
        .send()
        .await
        .expect("import should succeed");

    // Describe should show ACTIVE
    let ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();

    assert_eq!(
        ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::Active)
    );
    assert!(ca.not_before().is_some());
    assert!(ca.not_after().is_some());

    // Get CA certificate
    let resp = client
        .get_certificate_authority_certificate()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("get_ca_cert should succeed");

    let cert = resp.certificate().unwrap();
    assert!(cert.contains("-----BEGIN CERTIFICATE-----"));
    assert!(cert.contains("-----END CERTIFICATE-----"));
}

// --- test_root_certificate_has_no_chain ---
#[tokio::test]
async fn test_root_certificate_has_no_chain() {
    let client = make_client("us-east-2").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("root.test.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let csr = client
        .get_certificate_authority_csr()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .csr()
        .unwrap()
        .to_string();

    let cert_arn = client
        .issue_certificate()
        .certificate_authority_arn(&ca_arn)
        .csr(Blob::new(csr.as_bytes()))
        .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
        .template_arn("arn:aws:acm-pca:::template/RootCACertificate/V1")
        .validity(
            aws_sdk_acmpca::types::Validity::builder()
                .r#type(aws_sdk_acmpca::types::ValidityPeriodType::Years)
                .value(10)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    let resp = client
        .get_certificate()
        .certificate_authority_arn(&ca_arn)
        .certificate_arn(&cert_arn)
        .send()
        .await
        .unwrap();

    assert!(resp.certificate().is_some());
    assert!(
        resp.certificate_chain().is_none(),
        "root CA cert should have no chain"
    );
    assert!(
        resp.certificate()
            .unwrap()
            .contains("-----BEGIN CERTIFICATE-----")
    );
}

// --- test_list_certificate_authorities ---
#[tokio::test]
async fn test_list_certificate_authorities() {
    let client = make_client("us-east-1").await;

    let ca1_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("ca1.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
        .idempotency_token("token1")
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let ca2_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("ca2.test")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .idempotency_token("token2")
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let resp = client.list_certificate_authorities().send().await.unwrap();

    let cas = resp.certificate_authorities();
    assert_eq!(cas.len(), 2);

    let arns: Vec<&str> = cas.iter().filter_map(|ca| ca.arn()).collect();
    assert!(arns.contains(&ca1_arn.as_str()));
    assert!(arns.contains(&ca2_arn.as_str()));

    for ca in cas {
        assert!(ca.arn().is_some());
        assert!(ca.created_at().is_some());
        assert!(ca.status().is_some());
        assert!(ca.r#type().is_some());

        if ca.arn() == Some(ca1_arn.as_str()) {
            assert_eq!(
                ca.r#type(),
                Some(&aws_sdk_acmpca::types::CertificateAuthorityType::Subordinate)
            );
            assert_eq!(
                ca.status(),
                Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::PendingCertificate)
            );
            assert_eq!(
                ca.certificate_authority_configuration()
                    .unwrap()
                    .subject()
                    .unwrap()
                    .common_name(),
                Some("ca1.test")
            );
        } else {
            assert_eq!(
                ca.r#type(),
                Some(&aws_sdk_acmpca::types::CertificateAuthorityType::Root)
            );
            assert_eq!(
                ca.status(),
                Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::PendingCertificate)
            );
            assert_eq!(
                ca.certificate_authority_configuration()
                    .unwrap()
                    .subject()
                    .unwrap()
                    .common_name(),
                Some("ca2.test")
            );
        }
    }
}

// --- test_policy_operations ---
#[tokio::test]
async fn test_policy_operations() {
    let client = make_client("us-east-1").await;

    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa4096)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha512Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("test.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"012345678901"},"Action":["acm-pca:IssueCertificate"],"Resource":"*"}]}"#;

    // Put policy
    client
        .put_policy()
        .resource_arn(&ca_arn)
        .policy(policy)
        .send()
        .await
        .expect("put_policy should succeed");

    // Get policy
    let resp = client
        .get_policy()
        .resource_arn(&ca_arn)
        .send()
        .await
        .expect("get_policy should succeed");

    assert_eq!(resp.policy(), Some(policy));

    // Delete policy
    client
        .delete_policy()
        .resource_arn(&ca_arn)
        .send()
        .await
        .expect("delete_policy should succeed");

    // Get policy should fail
    let result = client.get_policy().resource_arn(&ca_arn).send().await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(
        service_err.is_resource_not_found_exception(),
        "expected ResourceNotFoundException"
    );
}

// ============================================================================
// Tests derived from AWS documentation: AWS Private CA (ACM PCA)
// ============================================================================

#[tokio::test]
async fn test_delete_certificate_authority_unknown() {
    let client = make_client("eu-west-1").await;

    let result = client
        .delete_certificate_authority()
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent CA should fail");
}

#[tokio::test]
async fn test_update_certificate_authority_unknown() {
    let client = make_client("eu-west-1").await;

    let result = client
        .update_certificate_authority()
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent",
        )
        .status(aws_sdk_acmpca::types::CertificateAuthorityStatus::Disabled)
        .send()
        .await;
    assert!(result.is_err(), "update nonexistent CA should fail");
}

#[tokio::test]
async fn test_get_certificate_authority_csr_unknown() {
    let client = make_client("eu-west-1").await;

    let result = client
        .get_certificate_authority_csr()
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err(), "get CSR for nonexistent CA should fail");
}

#[tokio::test]
async fn test_get_certificate_unknown_ca() {
    let client = make_client("eu-west-1").await;

    let result = client
        .get_certificate()
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent",
        )
        .certificate_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent/certificate/abc",
        )
        .send()
        .await;
    assert!(
        result.is_err(),
        "get certificate for nonexistent CA should fail"
    );
}

#[tokio::test]
async fn test_list_certificate_authorities_is_list() {
    let client = make_client("eu-west-1").await;

    let resp = client
        .list_certificate_authorities()
        .send()
        .await
        .expect("list_certificate_authorities should succeed");

    // Result is a list (may be empty or contain CAs created in other tests - just verify structure)
    let _ = resp.certificate_authorities();
}

#[tokio::test]
async fn test_put_policy_unknown_ca() {
    let client = make_client("eu-west-1").await;

    let result = client
        .put_policy()
        .resource_arn("arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await;
    assert!(result.is_err(), "put policy for nonexistent CA should fail");
}

#[tokio::test]
async fn test_get_policy_unknown_ca() {
    let client = make_client("eu-west-1").await;

    let result = client
        .get_policy()
        .resource_arn("arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "get policy for nonexistent CA should fail");
}

#[tokio::test]
async fn test_delete_policy_unknown_ca() {
    let client = make_client("eu-west-1").await;

    let result = client
        .delete_policy()
        .resource_arn("arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete policy for nonexistent CA should fail"
    );
}

// --- RestoreCertificateAuthority ---
#[tokio::test]
async fn test_restore_certificate_authority() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("restore-test.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    // Delete first to put into DELETED state
    client
        .delete_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("delete should succeed");

    // Restore from DELETED state
    client
        .restore_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("restore should succeed");

    // Verify status is DISABLED after restore
    let ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();

    assert_eq!(
        ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::Disabled)
    );
}

#[tokio::test]
async fn test_restore_certificate_authority_not_deleted() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("restore-test2.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    // Restore should fail because CA is not DELETED
    let result = client
        .restore_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await;
    assert!(result.is_err(), "restore on non-DELETED CA should fail");
}

#[tokio::test]
async fn test_restore_certificate_authority_unknown_ca() {
    let client = make_client("eu-west-1").await;
    let result = client
        .restore_certificate_authority()
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err(), "restore on unknown CA should fail");
}

// --- CreatePermission / DeletePermission / ListPermissions ---
#[tokio::test]
async fn test_create_and_list_permissions() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("perm-test.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    // Create a permission
    client
        .create_permission()
        .certificate_authority_arn(&ca_arn)
        .principal("acm.amazonaws.com")
        .actions(aws_sdk_acmpca::types::ActionType::IssueCertificate)
        .actions(aws_sdk_acmpca::types::ActionType::GetCertificate)
        .actions(aws_sdk_acmpca::types::ActionType::ListPermissions)
        .send()
        .await
        .expect("create_permission should succeed");

    // List permissions
    let resp = client
        .list_permissions()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("list_permissions should succeed");

    let perms = resp.permissions();
    assert_eq!(perms.len(), 1, "should have 1 permission");
    assert_eq!(perms[0].principal(), Some("acm.amazonaws.com"));
    assert_eq!(perms[0].actions().len(), 3);
}

#[tokio::test]
async fn test_delete_permission() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("del-perm-test.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    // Create a permission
    client
        .create_permission()
        .certificate_authority_arn(&ca_arn)
        .principal("acm.amazonaws.com")
        .actions(aws_sdk_acmpca::types::ActionType::IssueCertificate)
        .send()
        .await
        .expect("create_permission should succeed");

    // Delete the permission
    client
        .delete_permission()
        .certificate_authority_arn(&ca_arn)
        .principal("acm.amazonaws.com")
        .send()
        .await
        .expect("delete_permission should succeed");

    // Verify it's gone
    let resp = client
        .list_permissions()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("list_permissions should succeed");
    assert_eq!(
        resp.permissions().len(),
        0,
        "permissions should be empty after delete"
    );
}

#[tokio::test]
async fn test_delete_permission_nonexistent() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("no-perm-test.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let result = client
        .delete_permission()
        .certificate_authority_arn(&ca_arn)
        .principal("acm.amazonaws.com")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent permission should fail");
}

#[tokio::test]
async fn test_list_permissions_unknown_ca() {
    let client = make_client("eu-west-1").await;
    let result = client
        .list_permissions()
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent",
        )
        .send()
        .await;
    assert!(
        result.is_err(),
        "list_permissions on unknown CA should fail"
    );
}

// --- CreateCertificateAuthorityAuditReport / DescribeCertificateAuthorityAuditReport ---
#[tokio::test]
async fn test_create_and_describe_audit_report() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("audit-test.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    // Create an audit report
    let create_resp = client
        .create_certificate_authority_audit_report()
        .certificate_authority_arn(&ca_arn)
        .s3_bucket_name("my-audit-bucket")
        .audit_report_response_format(aws_sdk_acmpca::types::AuditReportResponseFormat::Json)
        .send()
        .await
        .expect("create audit report should succeed");

    let audit_report_id = create_resp.audit_report_id().unwrap().to_string();
    let s3_key = create_resp.s3_key().unwrap().to_string();
    assert!(!audit_report_id.is_empty());
    assert!(!s3_key.is_empty());

    // Describe the audit report
    let describe_resp = client
        .describe_certificate_authority_audit_report()
        .certificate_authority_arn(&ca_arn)
        .audit_report_id(&audit_report_id)
        .send()
        .await
        .expect("describe audit report should succeed");

    assert_eq!(
        describe_resp.audit_report_status(),
        Some(&aws_sdk_acmpca::types::AuditReportStatus::Success)
    );
    assert_eq!(describe_resp.s3_bucket_name(), Some("my-audit-bucket"));
    assert_eq!(describe_resp.s3_key(), Some(s3_key.as_str()));
    assert!(describe_resp.created_at().is_some());
}

#[tokio::test]
async fn test_describe_audit_report_nonexistent() {
    let client = make_client("eu-west-1").await;
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("audit-fail-test.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .send()
        .await
        .unwrap()
        .certificate_authority_arn()
        .unwrap()
        .to_string();

    let result = client
        .describe_certificate_authority_audit_report()
        .certificate_authority_arn(&ca_arn)
        .audit_report_id("nonexistent-report-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe nonexistent audit report should fail"
    );
}

#[tokio::test]
async fn test_create_audit_report_unknown_ca() {
    let client = make_client("eu-west-1").await;
    let result = client
        .create_certificate_authority_audit_report()
        .certificate_authority_arn(
            "arn:aws:acm-pca:eu-west-1:123456789012:certificate-authority/nonexistent",
        )
        .s3_bucket_name("my-audit-bucket")
        .audit_report_response_format(aws_sdk_acmpca::types::AuditReportResponseFormat::Json)
        .send()
        .await;
    assert!(
        result.is_err(),
        "create audit report for unknown CA should fail"
    );
}
