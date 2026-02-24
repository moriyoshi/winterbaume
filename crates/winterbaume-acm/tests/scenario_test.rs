//! End-to-end scenario tests for the ACM service.
//!
//! Each scenario chains 3+ operations and asserts business outcomes
//! rather than per-API return shapes.

use aws_sdk_acm::config::BehaviorVersion;
use aws_sdk_acm::types::{CertificateStatus, CertificateType, RenewalEligibility};
use winterbaume_acm::AcmService;
use winterbaume_core::MockAws;

const RSA_2048_CRT: &[u8] = include_bytes!("resources/star_moto_com.pem");
const RSA_2048_KEY: &[u8] = include_bytes!("resources/star_moto_com.key");
const CA_CRT: &[u8] = include_bytes!("resources/ca.pem");

async fn make_client() -> aws_sdk_acm::Client {
    let mock = MockAws::builder().with_service(AcmService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_acm::config::Region::new("eu-central-1"))
        .load()
        .await;
    aws_sdk_acm::Client::new(&config)
}

/// Scenario: Certificate tagging lifecycle.
///
/// A certificate is requested, tagged on creation, additional tags are added,
/// some are removed, and the final tag set is verified. Covers the full
/// tag-management surface area that Terraform uses to manage `aws_acm_certificate`.
#[tokio::test]
async fn test_certificate_tagging_lifecycle() {
    // Scenario: tag a certificate, add more tags, remove some, verify final state.
    let client = make_client().await;

    // Step 1: Request a certificate with an initial tag.
    let arn = client
        .request_certificate()
        .domain_name("tagging.example.com")
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("env")
                .value("staging")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("request_certificate should succeed")
        .certificate_arn()
        .unwrap()
        .to_string();

    // Step 2: Verify the certificate is immediately ISSUED.
    let desc = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();
    let cert = desc.certificate().unwrap();
    assert_eq!(cert.status(), Some(&CertificateStatus::Issued));

    // Step 3: Add more tags.
    client
        .add_tags_to_certificate()
        .certificate_arn(&arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("owner")
                .value("alice")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags_to_certificate should succeed");

    // Step 4: Remove one tag.
    client
        .remove_tags_from_certificate()
        .certificate_arn(&arn)
        .tags(
            aws_sdk_acm::types::Tag::builder()
                .key("owner")
                .value("alice")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("remove_tags_from_certificate should succeed");

    // Step 5: Assert final tag set — env + team remain; owner is gone.
    let tags = client
        .list_tags_for_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();
    let tag_map: std::collections::HashMap<String, Option<String>> = tags
        .tags()
        .iter()
        .map(|t| (t.key().to_string(), t.value().map(str::to_string)))
        .collect();

    assert_eq!(
        tag_map.get("env").and_then(|v| v.as_deref()),
        Some("staging")
    );
    assert_eq!(
        tag_map.get("team").and_then(|v| v.as_deref()),
        Some("platform")
    );
    assert!(
        !tag_map.contains_key("owner"),
        "removed tag should not appear"
    );
}

/// Scenario: Certificate fleet management.
///
/// Multiple certificates are created (requested and imported), listed with
/// status filters, updated, and finally cleaned up. Covers typical fleet
/// management workflows used by Terraform plan/apply/destroy cycles.
#[tokio::test]
async fn test_certificate_fleet_management() {
    // Scenario: build a fleet of certs, filter, update options, delete.
    let client = make_client().await;

    // Step 1: Request two DNS-validated certificates.
    let arn1 = client
        .request_certificate()
        .domain_name("fleet-a.example.com")
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    let arn2 = client
        .request_certificate()
        .domain_name("fleet-b.example.com")
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    // Step 2: Import a third certificate.
    let arn3 = client
        .import_certificate()
        .certificate(aws_sdk_acm::primitives::Blob::new(RSA_2048_CRT))
        .private_key(aws_sdk_acm::primitives::Blob::new(RSA_2048_KEY))
        .certificate_chain(aws_sdk_acm::primitives::Blob::new(CA_CRT))
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    // Step 3: All three should appear in the ISSUED list.
    let issued = client
        .list_certificates()
        .certificate_statuses(CertificateStatus::Issued)
        .send()
        .await
        .unwrap();
    let issued_arns: Vec<&str> = issued
        .certificate_summary_list()
        .iter()
        .map(|c| c.certificate_arn().unwrap())
        .collect();
    assert!(issued_arns.contains(&arn1.as_str()));
    assert!(issued_arns.contains(&arn2.as_str()));
    assert!(issued_arns.contains(&arn3.as_str()));

    // Step 4: Update transparency logging preference on arn1.
    client
        .update_certificate_options()
        .certificate_arn(&arn1)
        .options(
            aws_sdk_acm::types::CertificateOptions::builder()
                .certificate_transparency_logging_preference(
                    aws_sdk_acm::types::CertificateTransparencyLoggingPreference::Disabled,
                )
                .build(),
        )
        .send()
        .await
        .expect("update_certificate_options should succeed");

    // Verify the update is reflected in describe.
    let desc = client
        .describe_certificate()
        .certificate_arn(&arn1)
        .send()
        .await
        .unwrap();
    let opts = desc.certificate().unwrap().options().unwrap();
    assert_eq!(
        opts.certificate_transparency_logging_preference(),
        Some(&aws_sdk_acm::types::CertificateTransparencyLoggingPreference::Disabled)
    );

    // Step 5: Delete arn2; verify it disappears from the list.
    client
        .delete_certificate()
        .certificate_arn(&arn2)
        .send()
        .await
        .expect("delete_certificate should succeed");

    let after_delete = client.list_certificates().send().await.unwrap();
    let remaining_arns: Vec<&str> = after_delete
        .certificate_summary_list()
        .iter()
        .map(|c| c.certificate_arn().unwrap())
        .collect();
    assert!(
        !remaining_arns.contains(&arn2.as_str()),
        "deleted cert should not appear"
    );
    assert!(remaining_arns.contains(&arn1.as_str()));
    assert!(remaining_arns.contains(&arn3.as_str()));
}

/// Scenario: Imported certificate full lifecycle.
///
/// An external certificate is imported, inspected for type and timing fields,
/// then exported (which should fail for IMPORTED type — only PRIVATE certs
/// are exportable). This validates the IMPORTED type gate check.
#[tokio::test]
async fn test_imported_certificate_inspect_and_export_gate() {
    // Scenario: import → describe → attempt export (should fail) → delete.
    let client = make_client().await;

    // Step 1: Import.
    let arn = client
        .import_certificate()
        .certificate(aws_sdk_acm::primitives::Blob::new(RSA_2048_CRT))
        .private_key(aws_sdk_acm::primitives::Blob::new(RSA_2048_KEY))
        .certificate_chain(aws_sdk_acm::primitives::Blob::new(CA_CRT))
        .send()
        .await
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    // Step 2: Describe — type must be IMPORTED, timing fields must be populated.
    let desc = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .unwrap();
    let cert = desc.certificate().unwrap();
    assert_eq!(cert.r#type(), Some(&CertificateType::Imported));
    assert_eq!(
        cert.renewal_eligibility(),
        Some(&RenewalEligibility::Ineligible)
    );
    assert!(
        cert.not_before().is_some(),
        "not_before must be set for imported cert"
    );
    assert!(
        cert.not_after().is_some(),
        "not_after must be set for imported cert"
    );

    // Step 3: Export should fail — only PRIVATE certs are exportable.
    let export_result = client
        .export_certificate()
        .certificate_arn(&arn)
        .passphrase(aws_sdk_acm::primitives::Blob::new("securepassphrase"))
        .send()
        .await;
    assert!(export_result.is_err(), "IMPORTED cert cannot be exported");
    let meta = export_result
        .unwrap_err()
        .as_service_error()
        .unwrap()
        .meta()
        .clone();
    assert_eq!(meta.code(), Some("ValidationException"));
    assert!(
        meta.message()
            .unwrap_or("")
            .contains("not a private certificate")
    );

    // Step 4: Delete and confirm removal.
    client
        .delete_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_certificate()
        .certificate_arn(&arn)
        .send()
        .await;
    assert!(result.is_err(), "describing deleted cert should fail");
    assert!(
        result
            .unwrap_err()
            .as_service_error()
            .unwrap()
            .is_resource_not_found_exception()
    );
}
