use aws_sdk_guardduty::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_guardduty::GuardDutyService;

async fn make_client() -> aws_sdk_guardduty::Client {
    let mock = MockAws::builder()
        .with_service(GuardDutyService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_guardduty::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_guardduty::Client::new(&config)
}

/// Scenario: Detector lifecycle with filters and findings.
///
/// Verifies that a detector can be created, configured with a filter,
/// have sample findings generated, filtered by archive state, and then
/// cleaned up — the core GuardDuty operational workflow.
#[tokio::test]
async fn test_detector_filter_findings_lifecycle() {
    let client = make_client().await;

    // Step 1: Create a detector.
    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();
    assert!(!detector_id.is_empty());

    // Step 2: Create a filter on the detector.
    let filter_resp = client
        .create_filter()
        .detector_id(&detector_id)
        .name("high-severity-filter")
        .action(aws_sdk_guardduty::types::FilterAction::Noop)
        .rank(1)
        .finding_criteria(
            aws_sdk_guardduty::types::FindingCriteria::builder()
                .criterion(
                    "severity",
                    aws_sdk_guardduty::types::Condition::builder()
                        .greater_than_or_equal(7)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(filter_resp.name(), Some("high-severity-filter"));

    // Step 3: Create sample findings.
    client
        .create_sample_findings()
        .detector_id(&detector_id)
        .finding_types("UnauthorizedAccess:EC2/SSHBruteForce")
        .send()
        .await
        .unwrap();

    // Step 4: List findings — should include the sample finding.
    let list_resp = client
        .list_findings()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    let finding_ids = list_resp.finding_ids();
    assert!(
        !finding_ids.is_empty(),
        "expected at least one finding after CreateSampleFindings"
    );

    // Step 5: Retrieve finding details.
    let get_resp = client
        .get_findings()
        .detector_id(&detector_id)
        .set_finding_ids(Some(finding_ids.to_vec()))
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.findings().len(), finding_ids.len());

    // Step 6: Archive the findings.
    client
        .archive_findings()
        .detector_id(&detector_id)
        .set_finding_ids(Some(finding_ids.to_vec()))
        .send()
        .await
        .unwrap();

    // Step 7: Confirm archived findings no longer appear in default listing.
    let after_archive = client
        .list_findings()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert!(
        after_archive.finding_ids().is_empty(),
        "archived findings should not appear in default listing"
    );

    // Step 8: Delete the filter.
    client
        .delete_filter()
        .detector_id(&detector_id)
        .filter_name("high-severity-filter")
        .send()
        .await
        .unwrap();

    // Step 9: Delete the detector.
    client
        .delete_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    // Confirm detector is gone.
    let list_detectors = client.list_detectors().send().await.unwrap();
    assert!(
        !list_detectors
            .detector_ids()
            .iter()
            .any(|id| id == &detector_id),
        "detector should be deleted"
    );
}

/// Scenario: IP set and threat intel set management within a detector.
///
/// Verifies that a detector can manage IP sets and threat intel sets through
/// their full create/update/delete lifecycle.
#[tokio::test]
async fn test_detector_ip_and_threat_set_management() {
    let client = make_client().await;

    // Create detector.
    let detector_id = client
        .create_detector()
        .enable(true)
        .send()
        .await
        .unwrap()
        .detector_id()
        .unwrap()
        .to_string();

    // Create IP set.
    let ip_set_id = client
        .create_ip_set()
        .detector_id(&detector_id)
        .name("my-ip-set")
        .format(aws_sdk_guardduty::types::IpSetFormat::Txt)
        .location("https://example.com/ips.txt")
        .activate(true)
        .send()
        .await
        .unwrap()
        .ip_set_id()
        .unwrap()
        .to_string();

    // Create threat intel set.
    let ti_set_id = client
        .create_threat_intel_set()
        .detector_id(&detector_id)
        .name("my-ti-set")
        .format(aws_sdk_guardduty::types::ThreatIntelSetFormat::Txt)
        .location("https://example.com/threats.txt")
        .activate(true)
        .send()
        .await
        .unwrap()
        .threat_intel_set_id()
        .unwrap()
        .to_string();

    // Verify both appear in list results.
    let ip_sets = client
        .list_ip_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert!(ip_sets.ip_set_ids().iter().any(|id| id == &ip_set_id));

    let ti_sets = client
        .list_threat_intel_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert!(
        ti_sets
            .threat_intel_set_ids()
            .iter()
            .any(|id| id == &ti_set_id)
    );

    // Update the IP set.
    client
        .update_ip_set()
        .detector_id(&detector_id)
        .ip_set_id(&ip_set_id)
        .name("my-ip-set-renamed")
        .send()
        .await
        .unwrap();

    let ip_set_detail = client
        .get_ip_set()
        .detector_id(&detector_id)
        .ip_set_id(&ip_set_id)
        .send()
        .await
        .unwrap();
    assert_eq!(ip_set_detail.name(), Some("my-ip-set-renamed"));

    // Delete both sets and the detector.
    client
        .delete_ip_set()
        .detector_id(&detector_id)
        .ip_set_id(&ip_set_id)
        .send()
        .await
        .unwrap();

    client
        .delete_threat_intel_set()
        .detector_id(&detector_id)
        .threat_intel_set_id(&ti_set_id)
        .send()
        .await
        .unwrap();

    client
        .delete_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
}
