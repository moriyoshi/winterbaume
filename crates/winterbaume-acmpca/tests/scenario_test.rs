//! Scenario tests for ACM PCA: end-to-end workflows that chain 3+ operations
//! and assert business outcomes rather than per-API return shapes.

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

/// Scenario: Full root CA lifecycle — create, self-sign, activate, then disable and restore.
///
/// Steps: CreateCA → GetCSR → IssueCert (RootCA template) → GetCert → ImportCert →
///        DescribeCA (assert ACTIVE) → DeleteCA (assert DELETED) → RestoreCA (assert DISABLED)
#[tokio::test]
async fn test_root_ca_full_lifecycle() {
    let client = make_client("eu-west-1").await;

    // Step 1: Create a root CA.
    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("root-lifecycle.example.com")
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .certificate_authority_type(aws_sdk_acmpca::types::CertificateAuthorityType::Root)
        .tags(
            aws_sdk_acmpca::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create CA should succeed")
        .certificate_authority_arn()
        .unwrap()
        .to_string();

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
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::PendingCertificate),
        "newly created CA should be PENDING_CERTIFICATE"
    );

    // Step 2: Get the CSR.
    let csr = client
        .get_certificate_authority_csr()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("get CSR should succeed")
        .csr()
        .unwrap()
        .to_string();
    assert!(
        csr.contains("-----BEGIN CERTIFICATE REQUEST-----"),
        "CSR should be PEM"
    );

    // Step 3: Issue a self-signed root CA certificate.
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
        .expect("issue cert should succeed")
        .certificate_arn()
        .unwrap()
        .to_string();

    // Step 4: Retrieve the signed certificate.
    let cert_pem = client
        .get_certificate()
        .certificate_authority_arn(&ca_arn)
        .certificate_arn(&cert_arn)
        .send()
        .await
        .expect("get cert should succeed")
        .certificate()
        .unwrap()
        .to_string();
    assert!(
        cert_pem.contains("-----BEGIN CERTIFICATE-----"),
        "certificate should be PEM"
    );

    // Step 5: Import the certificate to activate the CA.
    client
        .import_certificate_authority_certificate()
        .certificate_authority_arn(&ca_arn)
        .certificate(Blob::new(cert_pem.as_bytes()))
        .send()
        .await
        .expect("import cert should succeed");

    // CA should now be ACTIVE with validity dates set.
    let active_ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();
    assert_eq!(
        active_ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::Active),
        "CA should be ACTIVE after import"
    );
    assert!(active_ca.not_before().is_some(), "not_before should be set");
    assert!(active_ca.not_after().is_some(), "not_after should be set");

    // Verify tag is still present after import.
    let tags = client
        .list_tags()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 1);
    assert_eq!(tags.tags()[0].key(), "env");

    // Step 6: Delete the CA (moves to DELETED).
    client
        .delete_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("delete CA should succeed");

    let deleted_ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();
    assert_eq!(
        deleted_ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::Deleted),
        "CA should be DELETED after delete"
    );

    // Step 7: Restore from DELETED state.
    client
        .restore_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("restore CA should succeed");

    let restored_ca = client
        .describe_certificate_authority()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .unwrap()
        .certificate_authority()
        .unwrap()
        .clone();
    assert_eq!(
        restored_ca.status(),
        Some(&aws_sdk_acmpca::types::CertificateAuthorityStatus::Disabled),
        "CA should be DISABLED after restore"
    );
}

/// Scenario: Permission delegation lifecycle — grant, verify, then revoke permissions.
///
/// Steps: CreateCA → CreatePermission → ListPermissions (assert present) →
///        DeletePermission → ListPermissions (assert empty)
#[tokio::test]
async fn test_permission_delegation_lifecycle() {
    let client = make_client("us-east-1").await;

    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("perm-scenario.example.com")
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

    // Grant ACM service access.
    client
        .create_permission()
        .certificate_authority_arn(&ca_arn)
        .principal("acm.amazonaws.com")
        .actions(aws_sdk_acmpca::types::ActionType::IssueCertificate)
        .actions(aws_sdk_acmpca::types::ActionType::GetCertificate)
        .actions(aws_sdk_acmpca::types::ActionType::ListPermissions)
        .send()
        .await
        .expect("create permission should succeed");

    // Verify permission is visible.
    let perms = client
        .list_permissions()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("list permissions should succeed");
    assert_eq!(perms.permissions().len(), 1, "one permission should exist");
    assert_eq!(
        perms.permissions()[0].principal(),
        Some("acm.amazonaws.com"),
        "correct principal"
    );
    assert_eq!(
        perms.permissions()[0].actions().len(),
        3,
        "all 3 actions should be listed"
    );

    // Revoke the permission.
    client
        .delete_permission()
        .certificate_authority_arn(&ca_arn)
        .principal("acm.amazonaws.com")
        .send()
        .await
        .expect("delete permission should succeed");

    // Verify permission is gone.
    let perms_after = client
        .list_permissions()
        .certificate_authority_arn(&ca_arn)
        .send()
        .await
        .expect("list permissions should succeed");
    assert_eq!(
        perms_after.permissions().len(),
        0,
        "no permissions should remain"
    );

    // Attempting to delete the same permission again should fail.
    let result = client
        .delete_permission()
        .certificate_authority_arn(&ca_arn)
        .principal("acm.amazonaws.com")
        .send()
        .await;
    assert!(
        result.is_err(),
        "deleting non-existent permission should fail"
    );
}

/// Scenario: Resource policy lifecycle — attach, read, then remove a CA policy.
///
/// Steps: CreateCA → PutPolicy → GetPolicy (assert content) → DeletePolicy →
///        GetPolicy (assert not found)
#[tokio::test]
async fn test_resource_policy_lifecycle() {
    let client = make_client("ap-southeast-1").await;

    let ca_arn = client
        .create_certificate_authority()
        .certificate_authority_configuration(
            aws_sdk_acmpca::types::CertificateAuthorityConfiguration::builder()
                .key_algorithm(aws_sdk_acmpca::types::KeyAlgorithm::Rsa2048)
                .signing_algorithm(aws_sdk_acmpca::types::SigningAlgorithm::Sha256Withrsa)
                .subject(
                    aws_sdk_acmpca::types::Asn1Subject::builder()
                        .common_name("policy-scenario.example.com")
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

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"arn:aws:iam::123456789012:root"},"Action":["acm-pca:IssueCertificate","acm-pca:GetCertificate"],"Resource":"*"}]}"#;

    // Attach policy.
    client
        .put_policy()
        .resource_arn(&ca_arn)
        .policy(policy_doc)
        .send()
        .await
        .expect("put_policy should succeed");

    // Read back and verify content.
    let resp = client
        .get_policy()
        .resource_arn(&ca_arn)
        .send()
        .await
        .expect("get_policy should succeed");
    assert_eq!(
        resp.policy(),
        Some(policy_doc),
        "retrieved policy should match attached policy"
    );

    // Remove policy.
    client
        .delete_policy()
        .resource_arn(&ca_arn)
        .send()
        .await
        .expect("delete_policy should succeed");

    // Subsequent get should return ResourceNotFoundException.
    let result = client.get_policy().resource_arn(&ca_arn).send().await;
    let err = result.unwrap_err().into_service_error();
    assert!(
        err.is_resource_not_found_exception(),
        "expected ResourceNotFoundException after policy deletion, got {:?}",
        err
    );
}
