use aws_sdk_guardduty::config::BehaviorVersion;
use aws_sdk_guardduty::types::{
    DetectorFeatureConfiguration, FilterAction, FindingPublishingFrequency,
};
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

fn make_s3_logs_config(enable: bool) -> aws_sdk_guardduty::types::S3LogsConfiguration {
    aws_sdk_guardduty::types::S3LogsConfiguration::builder()
        .enable(enable)
        .build()
}

fn make_k8s_config(enable: bool) -> aws_sdk_guardduty::types::KubernetesConfiguration {
    aws_sdk_guardduty::types::KubernetesConfiguration::builder()
        .audit_logs(
            aws_sdk_guardduty::types::KubernetesAuditLogsConfiguration::builder()
                .enable(enable)
                .build(),
        )
        .build()
}

// =============================================================================
// test_guardduty.py translations
// =============================================================================

/// Translated from: test_guardduty.py::test_create_detector
#[tokio::test]
#[allow(deprecated)]
async fn test_create_detector() {
    let client = make_client().await;

    let resp = client
        .create_detector()
        .enable(true)
        .client_token("745645734574758463758")
        .finding_publishing_frequency(FindingPublishingFrequency::OneHour)
        .data_sources(
            aws_sdk_guardduty::types::DataSourceConfigurations::builder()
                .s3_logs(make_s3_logs_config(true))
                .build(),
        )
        .features(
            DetectorFeatureConfiguration::builder()
                .name(aws_sdk_guardduty::types::DetectorFeature::from("Test"))
                .status(aws_sdk_guardduty::types::FeatureStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert!(resp.detector_id().is_some());
    assert!(!resp.detector_id().unwrap().is_empty());
}

/// Translated from: test_guardduty.py::test_create_detector_with_minimal_params
#[tokio::test]
async fn test_create_detector_with_minimal_params() {
    let client = make_client().await;

    let resp = client.create_detector().enable(true).send().await.unwrap();

    assert!(resp.detector_id().is_some());
    assert!(!resp.detector_id().unwrap().is_empty());
}

/// Translated from: test_guardduty.py::test_get_detector_with_s3
#[tokio::test]
#[allow(deprecated)]
async fn test_get_detector_with_s3() {
    let client = make_client().await;

    let create_resp = client
        .create_detector()
        .enable(true)
        .client_token("745645734574758463758")
        .finding_publishing_frequency(FindingPublishingFrequency::OneHour)
        .data_sources(
            aws_sdk_guardduty::types::DataSourceConfigurations::builder()
                .s3_logs(make_s3_logs_config(true))
                .build(),
        )
        .send()
        .await
        .unwrap();
    let detector_id = create_resp.detector_id().unwrap();

    let resp = client
        .get_detector()
        .detector_id(detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&FindingPublishingFrequency::OneHour)
    );
    let ds = resp.data_sources().expect("data_sources should be present");
    let s3_status = ds.s3_logs().expect("s3_logs should be present").status();
    assert_eq!(s3_status.map(|s| s.as_str()), Some("ENABLED"));
    assert!(resp.created_at().is_some());
}

/// Translated from: test_guardduty.py::test_get_detector_with_all_data_sources
#[tokio::test]
#[allow(deprecated)]
async fn test_get_detector_with_all_data_sources() {
    let client = make_client().await;

    let create_resp = client
        .create_detector()
        .enable(true)
        .client_token("745645734574758463758")
        .finding_publishing_frequency(FindingPublishingFrequency::OneHour)
        .data_sources(
            aws_sdk_guardduty::types::DataSourceConfigurations::builder()
                .s3_logs(make_s3_logs_config(true))
                .kubernetes(make_k8s_config(true))
                .build(),
        )
        .send()
        .await
        .unwrap();
    let detector_id = create_resp.detector_id().unwrap();

    let resp = client
        .get_detector()
        .detector_id(detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&FindingPublishingFrequency::OneHour)
    );
    let ds = resp.data_sources().expect("data_sources should be present");
    let s3_status = ds.s3_logs().expect("s3_logs").status();
    assert_eq!(s3_status.map(|s| s.as_str()), Some("ENABLED"));
    let k8s_status = ds
        .kubernetes()
        .expect("kubernetes")
        .audit_logs()
        .expect("audit_logs")
        .status();
    assert_eq!(k8s_status.map(|s| s.as_str()), Some("ENABLED"));
    assert!(resp.created_at().is_some());
}

/// Translated from: test_guardduty.py::test_get_detector_with_features
#[tokio::test]
async fn test_get_detector_with_features() {
    let client = make_client().await;

    let create_resp = client
        .create_detector()
        .enable(true)
        .features(
            DetectorFeatureConfiguration::builder()
                .name(aws_sdk_guardduty::types::DetectorFeature::from(
                    "EKS_AUDIT_LOGS",
                ))
                .status(aws_sdk_guardduty::types::FeatureStatus::Enabled)
                .additional_configuration(
                    aws_sdk_guardduty::types::DetectorAdditionalConfiguration::builder()
                        .name(
                            aws_sdk_guardduty::types::FeatureAdditionalConfiguration::from(
                                "EKS_ADDON_MANAGEMENT",
                            ),
                        )
                        .status(aws_sdk_guardduty::types::FeatureStatus::Enabled)
                        .build(),
                )
                .build(),
        )
        .features(
            DetectorFeatureConfiguration::builder()
                .name(aws_sdk_guardduty::types::DetectorFeature::from(
                    "TS3_DATA_EVENTS",
                ))
                .status(aws_sdk_guardduty::types::FeatureStatus::Disabled)
                .build(),
        )
        .send()
        .await
        .unwrap();
    let detector_id = create_resp.detector_id().unwrap();

    let resp = client
        .get_detector()
        .detector_id(detector_id)
        .send()
        .await
        .unwrap();

    let features = resp.features();
    assert_eq!(features.len(), 2);
    assert_eq!(
        features[0].name().map(|n| n.as_str()),
        Some("EKS_AUDIT_LOGS")
    );
    assert_eq!(features[0].status().map(|s| s.as_str()), Some("ENABLED"));
    let addl = features[0].additional_configuration();
    assert_eq!(
        addl[0].name().map(|n| n.as_str()),
        Some("EKS_ADDON_MANAGEMENT")
    );
    assert_eq!(
        features[1].name().map(|n| n.as_str()),
        Some("TS3_DATA_EVENTS")
    );
    assert_eq!(features[1].status().map(|s| s.as_str()), Some("DISABLED"));
}

/// Translated from: test_guardduty.py::test_update_detector
#[tokio::test]
#[allow(deprecated)]
async fn test_update_detector() {
    let client = make_client().await;

    let create_resp = client
        .create_detector()
        .enable(true)
        .client_token("745645734574758463758")
        .finding_publishing_frequency(FindingPublishingFrequency::OneHour)
        .send()
        .await
        .unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .update_detector()
        .detector_id(&detector_id)
        .enable(false)
        .finding_publishing_frequency(FindingPublishingFrequency::SixHours)
        .data_sources(
            aws_sdk_guardduty::types::DataSourceConfigurations::builder()
                .s3_logs(make_s3_logs_config(true))
                .kubernetes(make_k8s_config(false))
                .build(),
        )
        .features(
            DetectorFeatureConfiguration::builder()
                .name(aws_sdk_guardduty::types::DetectorFeature::from("Test"))
                .status(aws_sdk_guardduty::types::FeatureStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&FindingPublishingFrequency::SixHours)
    );
    let ds = resp.data_sources().expect("data_sources");
    let s3_status = ds.s3_logs().expect("s3_logs").status();
    assert_eq!(s3_status.map(|s| s.as_str()), Some("ENABLED"));
    let k8s_status = ds
        .kubernetes()
        .expect("kubernetes")
        .audit_logs()
        .expect("audit_logs")
        .status();
    assert_eq!(k8s_status.map(|s| s.as_str()), Some("DISABLED"));
    let features = resp.features();
    assert_eq!(features.len(), 1);
    assert_eq!(features[0].name().map(|n| n.as_str()), Some("Test"));
    assert_eq!(features[0].status().map(|s| s.as_str()), Some("ENABLED"));
}

/// Translated from: test_guardduty.py::test_list_detectors_initial
#[tokio::test]
async fn test_list_detectors_initial() {
    let client = make_client().await;

    let resp = client.list_detectors().send().await.unwrap();
    assert_eq!(resp.detector_ids().len(), 0);
}

/// Translated from: test_guardduty.py::test_list_detectors
#[tokio::test]
#[allow(deprecated)]
async fn test_list_detectors() {
    let client = make_client().await;

    let d1_resp = client
        .create_detector()
        .enable(true)
        .client_token("745645734574758463758")
        .finding_publishing_frequency(FindingPublishingFrequency::OneHour)
        .data_sources(
            aws_sdk_guardduty::types::DataSourceConfigurations::builder()
                .s3_logs(make_s3_logs_config(true))
                .build(),
        )
        .send()
        .await
        .unwrap();
    let d1 = d1_resp.detector_id().unwrap().to_string();

    let d2_resp = client.create_detector().enable(false).send().await.unwrap();
    let d2 = d2_resp.detector_id().unwrap().to_string();

    let resp = client.list_detectors().send().await.unwrap();
    let ids = resp.detector_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&d1));
    assert!(ids.contains(&d2));
}

/// Translated from: test_guardduty.py::test_delete_detector
#[tokio::test]
#[allow(deprecated)]
async fn test_delete_detector() {
    let client = make_client().await;

    let create_resp = client
        .create_detector()
        .enable(true)
        .client_token("745645734574758463758")
        .finding_publishing_frequency(FindingPublishingFrequency::OneHour)
        .data_sources(
            aws_sdk_guardduty::types::DataSourceConfigurations::builder()
                .s3_logs(make_s3_logs_config(true))
                .kubernetes(make_k8s_config(true))
                .build(),
        )
        .send()
        .await
        .unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    // Should succeed
    client
        .get_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    client
        .delete_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    // Get after delete should fail with BadRequestException
    let err = client
        .get_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
    assert_eq!(
        service_err.meta().message().unwrap(),
        "The request is rejected because the input detectorId is not owned by the current account."
    );

    // List should be empty
    let list_resp = client.list_detectors().send().await.unwrap();
    assert_eq!(list_resp.detector_ids().len(), 0);
}

/// Translated from: test_guardduty.py::test_get_administrator_account
#[tokio::test]
async fn test_get_administrator_account() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .enable_organization_admin_account()
        .admin_account_id("someaccount")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();

    assert_eq!(
        list_resp.admin_accounts()[0].admin_account_id(),
        Some("someaccount")
    );
    assert_eq!(
        list_resp.admin_accounts()[0].admin_status(),
        Some(&aws_sdk_guardduty::types::AdminStatus::Enabled)
    );

    // GetAdministratorAccount should succeed (200)
    let resp = client
        .get_administrator_account()
        .detector_id(&detector_id)
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Translated from: test_guardduty.py::test_no_administrator_account
#[tokio::test]
async fn test_no_administrator_account() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let list_resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();
    // No admin accounts should be returned
    assert!(list_resp.admin_accounts().is_empty());

    // GetAdministratorAccount should still succeed (200)
    let resp = client
        .get_administrator_account()
        .detector_id(&detector_id)
        .send()
        .await;
    assert!(resp.is_ok());
}

// =============================================================================
// test_guardduty_filters.py translations
// =============================================================================

/// Translated from: test_guardduty_filters.py::test_create_filter
#[tokio::test]
#[allow(deprecated)]
async fn test_create_filter() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let finding_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "x",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("y")
                .build(),
        )
        .build();

    let resp = client
        .create_filter()
        .detector_id(&detector_id)
        .name("my first filter")
        .finding_criteria(finding_criteria)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("my first filter"));
}

/// Translated from: test_guardduty_filters.py::test_create_filter__defaults
#[tokio::test]
#[allow(deprecated)]
async fn test_create_filter_defaults() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let finding_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "x",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("y")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("my first filter")
        .finding_criteria(finding_criteria)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_filter()
        .detector_id(&detector_id)
        .filter_name("my first filter")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.rank(), Some(1));
}

/// Translated from: test_guardduty_filters.py::test_get_filter
#[tokio::test]
#[allow(deprecated)]
async fn test_get_filter() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let finding_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "x",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("y")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("my first filter")
        .finding_criteria(finding_criteria)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_filter()
        .detector_id(&detector_id)
        .filter_name("my first filter")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("my first filter"));
    let fc = resp.finding_criteria().expect("finding_criteria");
    let criterion = fc.criterion().expect("criterion");
    let x_cond = criterion.get("x").expect("criterion key 'x'");
    assert_eq!(x_cond.eq(), &["y".to_string()]);
}

/// Translated from: test_guardduty_filters.py::test_update_filter
#[tokio::test]
#[allow(deprecated)]
async fn test_update_filter() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let finding_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "x",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("y")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("my first filter")
        .finding_criteria(finding_criteria)
        .send()
        .await
        .unwrap();

    let update_resp = client
        .update_filter()
        .detector_id(&detector_id)
        .filter_name("my first filter")
        .description("with desc")
        .rank(21)
        .action(FilterAction::Noop)
        .send()
        .await
        .unwrap();

    assert_eq!(update_resp.name(), Some("my first filter"));

    let resp = client
        .get_filter()
        .detector_id(&detector_id)
        .filter_name("my first filter")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("my first filter"));
    assert_eq!(resp.description(), Some("with desc"));
    assert_eq!(resp.rank(), Some(21));
    assert_eq!(resp.action(), Some(&FilterAction::Noop));
    // FindingCriteria should be unchanged
    let fc = resp.finding_criteria().expect("finding_criteria");
    let criterion = fc.criterion().expect("criterion");
    let x_cond = criterion.get("x").expect("criterion key 'x'");
    assert_eq!(x_cond.eq(), &["y".to_string()]);
}

/// Translated from: test_guardduty_filters.py::test_delete_filter
#[tokio::test]
#[allow(deprecated)]
async fn test_delete_filter() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let finding_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "x",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("y")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("my first filter")
        .finding_criteria(finding_criteria)
        .send()
        .await
        .unwrap();

    client
        .delete_filter()
        .detector_id(&detector_id)
        .filter_name("my first filter")
        .send()
        .await
        .unwrap();

    let err = client
        .get_filter()
        .detector_id(&detector_id)
        .filter_name("my first filter")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
}

// =============================================================================
// test_guardduty_organization.py translations
// =============================================================================

/// Translated from: test_guardduty_organization.py::test_enable_organization_admin_account
#[tokio::test]
async fn test_enable_organization_admin_account() {
    let client = make_client().await;

    let resp = client
        .enable_organization_admin_account()
        .admin_account_id("")
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Translated from: test_guardduty_organization.py::test_list_organization_admin_accounts
#[tokio::test]
async fn test_list_organization_admin_accounts() {
    let client = make_client().await;

    client
        .enable_organization_admin_account()
        .admin_account_id("someaccount")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();

    let accounts = resp.admin_accounts();
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].admin_account_id(), Some("someaccount"));
    assert_eq!(
        accounts[0].admin_status(),
        Some(&aws_sdk_guardduty::types::AdminStatus::Enabled)
    );
}

/// Test that creating a duplicate filter returns a BadRequestException.
#[tokio::test]
#[allow(deprecated)]
async fn test_create_filter_duplicate_name() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let finding_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "severity",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("HIGH")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("duplicate-filter")
        .finding_criteria(finding_criteria.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_filter()
        .detector_id(&detector_id)
        .name("duplicate-filter")
        .finding_criteria(finding_criteria)
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
}

/// Test that list_filters returns the correct filter names after creation.
#[tokio::test]
#[allow(deprecated)]
async fn test_list_filters() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let fc = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "type",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("Recon:EC2/PortProbeUnprotectedPort")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("filter-alpha")
        .finding_criteria(fc.clone())
        .send()
        .await
        .unwrap();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("filter-beta")
        .finding_criteria(fc)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_filters()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    let names = resp.filter_names();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"filter-alpha".to_string()));
    assert!(names.contains(&"filter-beta".to_string()));
}

/// Test that list_filters on an unknown detector returns a BadRequestException.
#[tokio::test]
async fn test_list_filters_unknown_detector() {
    let client = make_client().await;

    let err = client
        .list_filters()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
}

/// Test that get_filter on an unknown detector returns a BadRequestException.
#[tokio::test]
async fn test_get_filter_unknown_detector() {
    let client = make_client().await;

    let err = client
        .get_filter()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .filter_name("any-filter")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
}

/// Test that get_filter on a non-existent filter returns a BadRequestException.
#[tokio::test]
async fn test_get_filter_not_found() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let err = client
        .get_filter()
        .detector_id(&detector_id)
        .filter_name("nonexistent-filter")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
}

/// Test that update_detector on an unknown detector returns a BadRequestException.
#[tokio::test]
async fn test_update_detector_not_found() {
    let client = make_client().await;

    let err = client
        .update_detector()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .enable(false)
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
    assert_eq!(
        service_err.meta().message().unwrap(),
        "The request is rejected because the input detectorId is not owned by the current account."
    );
}

/// Test that a detector created with enable=false has DISABLED status.
#[tokio::test]
async fn test_create_detector_disabled() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(false).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .get_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_guardduty::types::DetectorStatus::Disabled)
    );
}

/// Test that update_detector can toggle a detector from enabled to disabled.
#[tokio::test]
async fn test_update_detector_enable_toggle() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .update_detector()
        .detector_id(&detector_id)
        .enable(false)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_guardduty::types::DetectorStatus::Disabled)
    );

    client
        .update_detector()
        .detector_id(&detector_id)
        .enable(true)
        .send()
        .await
        .unwrap();

    let resp2 = client
        .get_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp2.status(),
        Some(&aws_sdk_guardduty::types::DetectorStatus::Enabled)
    );
}

/// Test that update_filter can update finding_criteria in addition to metadata.
#[tokio::test]
#[allow(deprecated)]
async fn test_update_filter_finding_criteria() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let original_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "severity",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("LOW")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("criteria-update-filter")
        .finding_criteria(original_criteria)
        .send()
        .await
        .unwrap();

    let updated_criteria = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "severity",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("HIGH")
                .build(),
        )
        .build();

    client
        .update_filter()
        .detector_id(&detector_id)
        .filter_name("criteria-update-filter")
        .finding_criteria(updated_criteria)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_filter()
        .detector_id(&detector_id)
        .filter_name("criteria-update-filter")
        .send()
        .await
        .unwrap();

    let fc = resp.finding_criteria().expect("finding_criteria");
    let criterion = fc.criterion().expect("criterion");
    let sev_cond = criterion.get("severity").expect("severity key");
    assert_eq!(sev_cond.eq(), &["HIGH".to_string()]);
}

/// Test that enabling the same organization admin account twice returns a BadRequestException.
#[tokio::test]
async fn test_enable_organization_admin_account_duplicate() {
    let client = make_client().await;

    client
        .enable_organization_admin_account()
        .admin_account_id("account-123")
        .send()
        .await
        .unwrap();

    let err = client
        .enable_organization_admin_account()
        .admin_account_id("account-123")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_bad_request_exception());
}

/// Test that creating a filter with an ARCHIVE action stores the action correctly.
#[tokio::test]
#[allow(deprecated)]
async fn test_create_filter_with_archive_action() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let fc = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "type",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("Trojan:EC2/BlackholeTraffic")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("archive-filter")
        .action(FilterAction::Archive)
        .finding_criteria(fc)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_filter()
        .detector_id(&detector_id)
        .filter_name("archive-filter")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.action(), Some(&FilterAction::Archive));
}

/// Test that get_detector returns the default SIX_HOURS publishing frequency when none specified.
#[tokio::test]
async fn test_get_detector_default_frequency() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .get_detector()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&FindingPublishingFrequency::SixHours)
    );
}

/// Test that list_filters returns empty when no filters have been created.
#[tokio::test]
async fn test_list_filters_empty() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .list_filters()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.filter_names().len(), 0);
}

/// Test that after deleting a filter the list is updated correctly.
#[tokio::test]
#[allow(deprecated)]
async fn test_list_filters_after_delete() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let fc = aws_sdk_guardduty::types::FindingCriteria::builder()
        .criterion(
            "x",
            aws_sdk_guardduty::types::Condition::builder()
                .eq("y")
                .build(),
        )
        .build();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("to-delete")
        .finding_criteria(fc.clone())
        .send()
        .await
        .unwrap();

    client
        .create_filter()
        .detector_id(&detector_id)
        .name("to-keep")
        .finding_criteria(fc)
        .send()
        .await
        .unwrap();

    client
        .delete_filter()
        .detector_id(&detector_id)
        .filter_name("to-delete")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_filters()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    let names = resp.filter_names();
    assert_eq!(names.len(), 1);
    assert!(names.contains(&"to-keep".to_string()));
    assert!(!names.contains(&"to-delete".to_string()));
}

// =============================================================================
// IP set tests
// =============================================================================

#[tokio::test]
async fn test_create_and_get_ip_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let ip_set_resp = client
        .create_ip_set()
        .detector_id(&detector_id)
        .name("my-ip-set")
        .format(aws_sdk_guardduty::types::IpSetFormat::Txt)
        .location("s3://bucket/list.txt")
        .activate(true)
        .send()
        .await
        .unwrap();

    let ip_set_id = ip_set_resp.ip_set_id;
    assert!(ip_set_id.is_some());
    let ip_set_id = ip_set_id.unwrap();
    assert!(!ip_set_id.is_empty());

    let get_resp = client
        .get_ip_set()
        .detector_id(&detector_id)
        .ip_set_id(&ip_set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.name.as_deref().unwrap_or_default(), "my-ip-set");
    assert_eq!(
        get_resp.location.as_deref().unwrap_or_default(),
        "s3://bucket/list.txt"
    );
    assert_eq!(
        get_resp
            .status
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_default(),
        "ACTIVE"
    );
}

#[tokio::test]
async fn test_list_ip_sets() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .create_ip_set()
        .detector_id(&detector_id)
        .name("set-a")
        .format(aws_sdk_guardduty::types::IpSetFormat::Txt)
        .location("s3://bucket/a.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    client
        .create_ip_set()
        .detector_id(&detector_id)
        .name("set-b")
        .format(aws_sdk_guardduty::types::IpSetFormat::Txt)
        .location("s3://bucket/b.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_ip_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.ip_set_ids().len(), 2);
}

#[tokio::test]
async fn test_update_and_delete_ip_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let ip_set_resp = client
        .create_ip_set()
        .detector_id(&detector_id)
        .name("my-ip-set")
        .format(aws_sdk_guardduty::types::IpSetFormat::Txt)
        .location("s3://bucket/list.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let ip_set_id = ip_set_resp.ip_set_id.expect("ip_set_id");

    client
        .update_ip_set()
        .detector_id(&detector_id)
        .ip_set_id(&ip_set_id)
        .activate(true)
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_ip_set()
        .detector_id(&detector_id)
        .ip_set_id(&ip_set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        get_resp
            .status
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_default(),
        "ACTIVE"
    );

    client
        .delete_ip_set()
        .detector_id(&detector_id)
        .ip_set_id(&ip_set_id)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_ip_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.ip_set_ids().len(), 0);
}

// =============================================================================
// Threat intel set tests
// =============================================================================

#[tokio::test]
async fn test_create_and_get_threat_intel_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .create_threat_intel_set()
        .detector_id(&detector_id)
        .name("my-tis")
        .format(aws_sdk_guardduty::types::ThreatIntelSetFormat::Txt)
        .location("s3://bucket/tis.txt")
        .activate(true)
        .send()
        .await
        .unwrap();

    let set_id = resp.threat_intel_set_id.expect("threat_intel_set_id");
    assert!(!set_id.is_empty());

    let get_resp = client
        .get_threat_intel_set()
        .detector_id(&detector_id)
        .threat_intel_set_id(&set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.name.as_deref().unwrap_or_default(), "my-tis");
    assert_eq!(
        get_resp
            .status
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_default(),
        "ACTIVE"
    );
}

#[tokio::test]
async fn test_list_threat_intel_sets() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .create_threat_intel_set()
        .detector_id(&detector_id)
        .name("tis-1")
        .format(aws_sdk_guardduty::types::ThreatIntelSetFormat::Txt)
        .location("s3://bucket/1.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    client
        .create_threat_intel_set()
        .detector_id(&detector_id)
        .name("tis-2")
        .format(aws_sdk_guardduty::types::ThreatIntelSetFormat::Txt)
        .location("s3://bucket/2.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_threat_intel_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.threat_intel_set_ids().len(), 2);
}

#[tokio::test]
async fn test_update_and_delete_threat_intel_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .create_threat_intel_set()
        .detector_id(&detector_id)
        .name("my-tis")
        .format(aws_sdk_guardduty::types::ThreatIntelSetFormat::Txt)
        .location("s3://bucket/tis.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let set_id = resp.threat_intel_set_id().unwrap().to_string();

    client
        .update_threat_intel_set()
        .detector_id(&detector_id)
        .threat_intel_set_id(&set_id)
        .activate(true)
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_threat_intel_set()
        .detector_id(&detector_id)
        .threat_intel_set_id(&set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.status().map(|s| s.as_str()), Some("ACTIVE"));

    client
        .delete_threat_intel_set()
        .detector_id(&detector_id)
        .threat_intel_set_id(&set_id)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_threat_intel_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.threat_intel_set_ids().len(), 0);
}

// =============================================================================
// Resource tag tests
// =============================================================================

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resource_arn = format!(
        "arn:aws:guardduty:us-east-1:123456789012:detector/{}",
        detector_id
    );

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags;
    let tags = tags.as_ref().expect("expected tags");
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resource_arn = format!(
        "arn:aws:guardduty:us-east-1:123456789012:detector/{}",
        detector_id
    );

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&resource_arn)
        .tag_keys("env")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags;
    let tags = tags.as_ref().expect("expected tags");
    assert!(!tags.contains_key("env"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

// =============================================================================
// Publishing destination CRUD tests
// =============================================================================

/// Test creating a publishing destination and retrieving it via describe.
#[tokio::test]
async fn test_create_and_describe_publishing_destination() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let dest_props = aws_sdk_guardduty::types::DestinationProperties::builder()
        .destination_arn("arn:aws:s3:::my-bucket")
        .kms_key_arn("arn:aws:kms:us-east-1:123456789012:key/abc123")
        .build();

    let resp = client
        .create_publishing_destination()
        .detector_id(&detector_id)
        .destination_type(aws_sdk_guardduty::types::DestinationType::S3)
        .destination_properties(dest_props)
        .send()
        .await
        .unwrap();

    let destination_id = resp.destination_id().unwrap().to_string();
    assert!(!destination_id.is_empty());

    let describe_resp = client
        .describe_publishing_destination()
        .detector_id(&detector_id)
        .destination_id(&destination_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        describe_resp.destination_id(),
        Some(destination_id.as_str())
    );
    assert_eq!(
        describe_resp.destination_type(),
        Some(&aws_sdk_guardduty::types::DestinationType::S3)
    );
    assert_eq!(
        describe_resp.status(),
        Some(&aws_sdk_guardduty::types::PublishingStatus::Publishing)
    );
    let props = describe_resp.destination_properties().unwrap();
    assert_eq!(props.destination_arn(), Some("arn:aws:s3:::my-bucket"));
    assert_eq!(
        props.kms_key_arn(),
        Some("arn:aws:kms:us-east-1:123456789012:key/abc123")
    );
}

/// Test listing publishing destinations.
#[tokio::test]
async fn test_list_publishing_destinations() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    // Initially empty
    let list_resp = client
        .list_publishing_destinations()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.destinations().len(), 0);

    // Create two destinations
    let dest_props = aws_sdk_guardduty::types::DestinationProperties::builder()
        .destination_arn("arn:aws:s3:::bucket-one")
        .build();
    client
        .create_publishing_destination()
        .detector_id(&detector_id)
        .destination_type(aws_sdk_guardduty::types::DestinationType::S3)
        .destination_properties(dest_props)
        .send()
        .await
        .unwrap();

    let dest_props2 = aws_sdk_guardduty::types::DestinationProperties::builder()
        .destination_arn("arn:aws:s3:::bucket-two")
        .build();
    client
        .create_publishing_destination()
        .detector_id(&detector_id)
        .destination_type(aws_sdk_guardduty::types::DestinationType::S3)
        .destination_properties(dest_props2)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_publishing_destinations()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.destinations().len(), 2);
}

/// Test updating a publishing destination.
#[tokio::test]
async fn test_update_publishing_destination() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let dest_props = aws_sdk_guardduty::types::DestinationProperties::builder()
        .destination_arn("arn:aws:s3:::original-bucket")
        .build();
    let create_dest_resp = client
        .create_publishing_destination()
        .detector_id(&detector_id)
        .destination_type(aws_sdk_guardduty::types::DestinationType::S3)
        .destination_properties(dest_props)
        .send()
        .await
        .unwrap();
    let destination_id = create_dest_resp.destination_id().unwrap().to_string();

    let new_props = aws_sdk_guardduty::types::DestinationProperties::builder()
        .destination_arn("arn:aws:s3:::updated-bucket")
        .kms_key_arn("arn:aws:kms:us-east-1:123456789012:key/new-key")
        .build();
    client
        .update_publishing_destination()
        .detector_id(&detector_id)
        .destination_id(&destination_id)
        .destination_properties(new_props)
        .send()
        .await
        .unwrap();

    let describe_resp = client
        .describe_publishing_destination()
        .detector_id(&detector_id)
        .destination_id(&destination_id)
        .send()
        .await
        .unwrap();

    let props = describe_resp.destination_properties().unwrap();
    assert_eq!(props.destination_arn(), Some("arn:aws:s3:::updated-bucket"));
    assert_eq!(
        props.kms_key_arn(),
        Some("arn:aws:kms:us-east-1:123456789012:key/new-key")
    );
}

/// Test deleting a publishing destination.
#[tokio::test]
async fn test_delete_publishing_destination() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let dest_props = aws_sdk_guardduty::types::DestinationProperties::builder()
        .destination_arn("arn:aws:s3:::to-be-deleted")
        .build();
    let create_dest_resp = client
        .create_publishing_destination()
        .detector_id(&detector_id)
        .destination_type(aws_sdk_guardduty::types::DestinationType::S3)
        .destination_properties(dest_props)
        .send()
        .await
        .unwrap();
    let destination_id = create_dest_resp.destination_id().unwrap().to_string();

    // Should exist in list
    let list_resp = client
        .list_publishing_destinations()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.destinations().len(), 1);

    // Delete it
    client
        .delete_publishing_destination()
        .detector_id(&detector_id)
        .destination_id(&destination_id)
        .send()
        .await
        .unwrap();

    // Should be gone
    let list_resp = client
        .list_publishing_destinations()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.destinations().len(), 0);
}

// =============================================================================
// Member management tests
// =============================================================================

/// Test create_members, list_members, get_members lifecycle.
#[tokio::test]
async fn test_create_list_get_members() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let account_detail = aws_sdk_guardduty::types::AccountDetail::builder()
        .account_id("111111111111")
        .email("member@example.com")
        .build();

    client
        .create_members()
        .detector_id(&detector_id)
        .account_details(account_detail)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_members()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.members().len(), 1);
    assert_eq!(
        list_resp.members()[0].account_id().unwrap_or_default(),
        "111111111111"
    );

    let get_resp = client
        .get_members()
        .detector_id(&detector_id)
        .account_ids("111111111111")
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.members().len(), 1);
    assert_eq!(
        get_resp.members()[0].account_id().unwrap_or_default(),
        "111111111111"
    );
}

/// Test invite_members, disassociate_members, stop_monitoring_members, start_monitoring_members.
#[tokio::test]
async fn test_member_status_transitions() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let account_detail = aws_sdk_guardduty::types::AccountDetail::builder()
        .account_id("222222222222")
        .email("member2@example.com")
        .build();

    client
        .create_members()
        .detector_id(&detector_id)
        .account_details(account_detail)
        .send()
        .await
        .unwrap();

    client
        .invite_members()
        .detector_id(&detector_id)
        .account_ids("222222222222")
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_members()
        .detector_id(&detector_id)
        .account_ids("222222222222")
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.members()[0]
            .relationship_status()
            .unwrap_or_default(),
        "Invited"
    );

    client
        .start_monitoring_members()
        .detector_id(&detector_id)
        .account_ids("222222222222")
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_members()
        .detector_id(&detector_id)
        .account_ids("222222222222")
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.members()[0]
            .relationship_status()
            .unwrap_or_default(),
        "Enabled"
    );

    client
        .stop_monitoring_members()
        .detector_id(&detector_id)
        .account_ids("222222222222")
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_members()
        .detector_id(&detector_id)
        .account_ids("222222222222")
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.members()[0]
            .relationship_status()
            .unwrap_or_default(),
        "Disabled"
    );
}

/// Test delete_members removes members from the list.
#[tokio::test]
async fn test_delete_members() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let account_detail = aws_sdk_guardduty::types::AccountDetail::builder()
        .account_id("333333333333")
        .email("deleteme@example.com")
        .build();

    client
        .create_members()
        .detector_id(&detector_id)
        .account_details(account_detail)
        .send()
        .await
        .unwrap();

    client
        .delete_members()
        .detector_id(&detector_id)
        .account_ids("333333333333")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_members()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.members().len(), 0);
}

// =============================================================================
// Malware protection plan tests
// =============================================================================

/// Test create, get, list, update, delete malware protection plan lifecycle.
#[tokio::test]
async fn test_malware_protection_plan_lifecycle() {
    let client = make_client().await;

    let s3_bucket = aws_sdk_guardduty::types::CreateS3BucketResource::builder()
        .bucket_name("my-bucket")
        .build();
    let protected_resource = aws_sdk_guardduty::types::CreateProtectedResource::builder()
        .s3_bucket(s3_bucket)
        .build();

    let create_resp = client
        .create_malware_protection_plan()
        .role("arn:aws:iam::123456789012:role/GuardDutyRole")
        .protected_resource(protected_resource)
        .send()
        .await
        .unwrap();

    let plan_id = create_resp
        .malware_protection_plan_id()
        .unwrap()
        .to_string();
    assert!(!plan_id.is_empty());

    // List plans
    let list_resp = client.list_malware_protection_plans().send().await.unwrap();
    assert_eq!(list_resp.malware_protection_plans().len(), 1);
    assert_eq!(
        list_resp.malware_protection_plans()[0]
            .malware_protection_plan_id()
            .unwrap_or_default(),
        plan_id
    );

    // Get plan
    let get_resp = client
        .get_malware_protection_plan()
        .malware_protection_plan_id(&plan_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.role().unwrap_or_default(),
        "arn:aws:iam::123456789012:role/GuardDutyRole"
    );
    assert_eq!(
        get_resp.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE"
    );

    // Update plan
    client
        .update_malware_protection_plan()
        .malware_protection_plan_id(&plan_id)
        .role("arn:aws:iam::123456789012:role/UpdatedRole")
        .send()
        .await
        .unwrap();

    let get_resp2 = client
        .get_malware_protection_plan()
        .malware_protection_plan_id(&plan_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp2.role().unwrap_or_default(),
        "arn:aws:iam::123456789012:role/UpdatedRole"
    );

    // Delete plan
    client
        .delete_malware_protection_plan()
        .malware_protection_plan_id(&plan_id)
        .send()
        .await
        .unwrap();

    // Should no longer be listed
    let list_resp2 = client.list_malware_protection_plans().send().await.unwrap();
    assert_eq!(list_resp2.malware_protection_plans().len(), 0);
}

/// Test get_malware_protection_plan returns not found for missing plan.
#[tokio::test]
async fn test_get_malware_protection_plan_not_found() {
    let client = make_client().await;

    let result = client
        .get_malware_protection_plan()
        .malware_protection_plan_id("nonexistent-plan-id")
        .send()
        .await;

    assert!(result.is_err());
}

// =============================================================================
// Malware scan settings tests
// =============================================================================

/// Test get and update malware scan settings.
#[tokio::test]
async fn test_malware_scan_settings() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    // Default settings
    let get_resp = client
        .get_malware_scan_settings()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert!(get_resp.ebs_snapshot_preservation().is_none());

    // Update settings
    client
        .update_malware_scan_settings()
        .detector_id(&detector_id)
        .ebs_snapshot_preservation(aws_sdk_guardduty::types::EbsSnapshotPreservation::NoRetention)
        .send()
        .await
        .unwrap();

    let get_resp2 = client
        .get_malware_scan_settings()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp2
            .ebs_snapshot_preservation()
            .map(|s| s.as_str())
            .unwrap_or_default(),
        "NO_RETENTION"
    );
}

// =============================================================================
// Malware scan tests
// =============================================================================

/// Test start_malware_scan and get_malware_scan lifecycle.
#[tokio::test]
async fn test_start_and_get_malware_scan() {
    let client = make_client().await;

    client.create_detector().enable(true).send().await.unwrap();

    let start_resp = client
        .start_malware_scan()
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:instance/i-12345")
        .send()
        .await
        .unwrap();

    let scan_id = start_resp.scan_id().unwrap().to_string();
    assert!(!scan_id.is_empty());

    let get_resp = client
        .get_malware_scan()
        .scan_id(&scan_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.scan_id().unwrap_or_default(), scan_id);
    assert_eq!(
        get_resp
            .scan_status()
            .map(|s| s.as_str())
            .unwrap_or_default(),
        "RUNNING"
    );
}

/// Test describe_malware_scans returns scans for a detector.
#[tokio::test]
async fn test_describe_malware_scans() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let _detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .start_malware_scan()
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:instance/i-abc")
        .send()
        .await
        .unwrap();

    let describe_resp = client
        .describe_malware_scans()
        .detector_id(&_detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(describe_resp.scans().len(), 1);
}

// =============================================================================
// Invitation tests
// =============================================================================

/// Test get_invitations_count and list_invitations with no invitations.
#[tokio::test]
async fn test_invitations_empty() {
    let client = make_client().await;

    let count_resp = client.get_invitations_count().send().await.unwrap();
    assert_eq!(count_resp.invitations_count(), Some(0));

    let list_resp = client.list_invitations().send().await.unwrap();
    assert_eq!(list_resp.invitations().len(), 0);
}

// =============================================================================
// Organisation configuration tests
// =============================================================================

/// Test update and describe organisation configuration.
#[tokio::test]
async fn test_organization_configuration() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .update_organization_configuration()
        .detector_id(&detector_id)
        .auto_enable_organization_members(aws_sdk_guardduty::types::AutoEnableMembers::New)
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_organization_configuration()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        desc_resp
            .auto_enable_organization_members()
            .map(|s| s.as_str())
            .unwrap_or_default(),
        "NEW"
    );
}

// =============================================================================
// Disable organisation admin account
// =============================================================================

/// Test enable then disable organisation admin account.
#[tokio::test]
async fn test_disable_organization_admin_account() {
    let client = make_client().await;

    client
        .enable_organization_admin_account()
        .admin_account_id("444444444444")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.admin_accounts().len(), 1);

    client
        .disable_organization_admin_account()
        .admin_account_id("444444444444")
        .send()
        .await
        .unwrap();

    let list_resp2 = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp2.admin_accounts().len(), 0);
}

// =============================================================================
// Tests for newly implemented handlers (formerly STUB)
// =============================================================================

/// Test accept_administrator_invitation sets the administrator on the detector.
#[tokio::test]
async fn test_accept_administrator_invitation() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    // Accept administrator invitation
    let resp = client
        .accept_administrator_invitation()
        .detector_id(&detector_id)
        .administrator_id("admin-account-123")
        .invitation_id("inv-001")
        .send()
        .await;
    assert!(resp.is_ok());

    // GetAdministratorAccount should reflect the accepted admin
    let admin_resp = client
        .get_administrator_account()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    // Verify it returns successfully (the administrator field is populated)
    assert!(admin_resp.administrator().is_some());
}

/// Test accept_administrator_invitation on a non-existent detector returns error.
#[tokio::test]
async fn test_accept_administrator_invitation_bad_detector() {
    let client = make_client().await;

    let resp = client
        .accept_administrator_invitation()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .administrator_id("admin-account-123")
        .invitation_id("inv-001")
        .send()
        .await;
    assert!(resp.is_err());
}

/// Test accept_invitation (legacy) sets the master account on the detector.
#[tokio::test]
#[allow(deprecated)]
async fn test_accept_invitation() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .accept_invitation()
        .detector_id(&detector_id)
        .master_id("master-account-456")
        .invitation_id("inv-002")
        .send()
        .await;
    assert!(resp.is_ok());

    // GetMasterAccount should return the master account
    let master_resp = client
        .get_master_account()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    let master = master_resp.master().expect("master should be present");
    assert_eq!(master.account_id(), Some("master-account-456"));
}

/// Test disassociate_from_administrator_account clears the admin relationship.
#[tokio::test]
async fn test_disassociate_from_administrator_account() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    // First accept an administrator invitation
    client
        .accept_administrator_invitation()
        .detector_id(&detector_id)
        .administrator_id("admin-account-789")
        .invitation_id("inv-003")
        .send()
        .await
        .unwrap();

    // Now disassociate
    let resp = client
        .disassociate_from_administrator_account()
        .detector_id(&detector_id)
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Test disassociate_from_administrator_account on a bad detector returns error.
#[tokio::test]
async fn test_disassociate_from_administrator_account_bad_detector() {
    let client = make_client().await;

    let resp = client
        .disassociate_from_administrator_account()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await;
    assert!(resp.is_err());
}

/// Test get_master_account returns empty master when none is set.
#[tokio::test]
#[allow(deprecated)]
async fn test_get_master_account_empty() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .get_master_account()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    // Should succeed and return a master with no account_id
    let master = resp.master().expect("master field should be present");
    assert!(master.account_id().is_none() || master.account_id() == Some(""));
}

/// Test get_master_account on a bad detector returns error.
#[tokio::test]
#[allow(deprecated)]
async fn test_get_master_account_bad_detector() {
    let client = make_client().await;

    let resp = client
        .get_master_account()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await;
    assert!(resp.is_err());
}

/// Test disassociate_from_master_account succeeds.
#[tokio::test]
#[allow(deprecated)]
async fn test_disassociate_from_master_account() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    // First accept a legacy invitation
    client
        .accept_invitation()
        .detector_id(&detector_id)
        .master_id("master-account-101")
        .invitation_id("inv-004")
        .send()
        .await
        .unwrap();

    // Disassociate from master
    let resp = client
        .disassociate_from_master_account()
        .detector_id(&detector_id)
        .send()
        .await;
    assert!(resp.is_ok());

    // GetMasterAccount should now return empty
    let master_resp = client
        .get_master_account()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();
    let master = master_resp.master().expect("master should be present");
    assert!(master.account_id().is_none() || master.account_id() == Some(""));
}

/// Test get_coverage_statistics returns valid response for a detector.
#[tokio::test]
async fn test_get_coverage_statistics() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .get_coverage_statistics()
        .detector_id(&detector_id)
        .statistics_type(aws_sdk_guardduty::types::CoverageStatisticsType::CountByCoverageStatus)
        .send()
        .await
        .unwrap();

    // Should return valid (possibly empty) coverage statistics
    assert!(resp.coverage_statistics().is_some());
}

/// Test get_coverage_statistics on a bad detector returns error.
#[tokio::test]
async fn test_get_coverage_statistics_bad_detector() {
    let client = make_client().await;

    let resp = client
        .get_coverage_statistics()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .statistics_type(aws_sdk_guardduty::types::CoverageStatisticsType::CountByCoverageStatus)
        .send()
        .await;
    assert!(resp.is_err());
}

/// Test list_coverage returns valid response for a detector.
#[tokio::test]
async fn test_list_coverage() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .list_coverage()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    // Empty detector should have no coverage resources
    assert!(resp.resources().is_empty());
}

/// Test list_coverage on a bad detector returns error.
#[tokio::test]
async fn test_list_coverage_bad_detector() {
    let client = make_client().await;

    let resp = client
        .list_coverage()
        .detector_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await;
    assert!(resp.is_err());
}

/// Test send_object_malware_scan returns 200.
#[tokio::test]
async fn test_send_object_malware_scan() {
    let client = make_client().await;

    let resp = client.send_object_malware_scan().send().await;
    assert!(resp.is_ok());
}

/// Test get_organization_statistics returns sensible defaults.
#[tokio::test]
async fn test_get_organization_statistics() {
    let client = make_client().await;

    let resp = client.get_organization_statistics().send().await.unwrap();

    let details = resp
        .organization_details()
        .expect("organization_details should be present");
    assert!(details.organization_statistics().is_some());
}

/// Test get_organization_statistics reflects enabled admin accounts.
#[tokio::test]
async fn test_get_organization_statistics_with_admin() {
    let client = make_client().await;

    client
        .enable_organization_admin_account()
        .admin_account_id("org-admin-123")
        .send()
        .await
        .unwrap();

    let resp = client.get_organization_statistics().send().await.unwrap();

    let details = resp.organization_details().expect("organization_details");
    let stats = details
        .organization_statistics()
        .expect("organization_statistics");
    assert!(stats.enabled_accounts_count() >= Some(1));
}

// =============================================================================
// Threat entity set tests
// =============================================================================

/// Test create and get threat entity set.
#[tokio::test]
async fn test_create_and_get_threat_entity_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .create_threat_entity_set()
        .detector_id(&detector_id)
        .name("my-tes")
        .format(aws_sdk_guardduty::types::ThreatEntitySetFormat::Txt)
        .location("s3://bucket/tes.txt")
        .activate(true)
        .send()
        .await
        .unwrap();

    let set_id = resp.threat_entity_set_id().unwrap().to_string();
    assert!(!set_id.is_empty());

    let get_resp = client
        .get_threat_entity_set()
        .detector_id(&detector_id)
        .threat_entity_set_id(&set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.name().unwrap_or_default(), "my-tes");
    assert_eq!(
        get_resp.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE"
    );
    assert_eq!(
        get_resp.location().unwrap_or_default(),
        "s3://bucket/tes.txt"
    );
}

/// Test list threat entity sets.
#[tokio::test]
async fn test_list_threat_entity_sets() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .create_threat_entity_set()
        .detector_id(&detector_id)
        .name("tes-1")
        .format(aws_sdk_guardduty::types::ThreatEntitySetFormat::Txt)
        .location("s3://bucket/1.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    client
        .create_threat_entity_set()
        .detector_id(&detector_id)
        .name("tes-2")
        .format(aws_sdk_guardduty::types::ThreatEntitySetFormat::Txt)
        .location("s3://bucket/2.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_threat_entity_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.threat_entity_set_ids().len(), 2);
}

/// Test update and delete threat entity set.
#[tokio::test]
async fn test_update_and_delete_threat_entity_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .create_threat_entity_set()
        .detector_id(&detector_id)
        .name("tes-update")
        .format(aws_sdk_guardduty::types::ThreatEntitySetFormat::Txt)
        .location("s3://bucket/tes.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let set_id = resp.threat_entity_set_id().unwrap().to_string();

    client
        .update_threat_entity_set()
        .detector_id(&detector_id)
        .threat_entity_set_id(&set_id)
        .name("tes-updated")
        .activate(true)
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_threat_entity_set()
        .detector_id(&detector_id)
        .threat_entity_set_id(&set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.name().unwrap_or_default(), "tes-updated");
    assert_eq!(
        get_resp.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE"
    );

    client
        .delete_threat_entity_set()
        .detector_id(&detector_id)
        .threat_entity_set_id(&set_id)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_threat_entity_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.threat_entity_set_ids().len(), 0);
}

// =============================================================================
// Trusted entity set tests
// =============================================================================

/// Test create and get trusted entity set.
#[tokio::test]
async fn test_create_and_get_trusted_entity_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .create_trusted_entity_set()
        .detector_id(&detector_id)
        .name("my-trs")
        .format(aws_sdk_guardduty::types::TrustedEntitySetFormat::Txt)
        .location("s3://bucket/trs.txt")
        .activate(true)
        .send()
        .await
        .unwrap();

    let set_id = resp.trusted_entity_set_id().unwrap().to_string();
    assert!(!set_id.is_empty());

    let get_resp = client
        .get_trusted_entity_set()
        .detector_id(&detector_id)
        .trusted_entity_set_id(&set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.name().unwrap_or_default(), "my-trs");
    assert_eq!(
        get_resp.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE"
    );
    assert_eq!(
        get_resp.location().unwrap_or_default(),
        "s3://bucket/trs.txt"
    );
}

/// Test list trusted entity sets.
#[tokio::test]
async fn test_list_trusted_entity_sets() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    client
        .create_trusted_entity_set()
        .detector_id(&detector_id)
        .name("trs-1")
        .format(aws_sdk_guardduty::types::TrustedEntitySetFormat::Txt)
        .location("s3://bucket/1.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    client
        .create_trusted_entity_set()
        .detector_id(&detector_id)
        .name("trs-2")
        .format(aws_sdk_guardduty::types::TrustedEntitySetFormat::Txt)
        .location("s3://bucket/2.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_trusted_entity_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.trusted_entity_set_ids().len(), 2);
}

/// Test update and delete trusted entity set.
#[tokio::test]
async fn test_update_and_delete_trusted_entity_set() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let resp = client
        .create_trusted_entity_set()
        .detector_id(&detector_id)
        .name("trs-update")
        .format(aws_sdk_guardduty::types::TrustedEntitySetFormat::Txt)
        .location("s3://bucket/trs.txt")
        .activate(false)
        .send()
        .await
        .unwrap();

    let set_id = resp.trusted_entity_set_id().unwrap().to_string();

    client
        .update_trusted_entity_set()
        .detector_id(&detector_id)
        .trusted_entity_set_id(&set_id)
        .name("trs-updated")
        .activate(true)
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_trusted_entity_set()
        .detector_id(&detector_id)
        .trusted_entity_set_id(&set_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.name().unwrap_or_default(), "trs-updated");
    assert_eq!(
        get_resp.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE"
    );

    client
        .delete_trusted_entity_set()
        .detector_id(&detector_id)
        .trusted_entity_set_id(&set_id)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_trusted_entity_sets()
        .detector_id(&detector_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.trusted_entity_set_ids().len(), 0);
}

// =============================================================================
// Member detector tests
// =============================================================================

/// Test get_member_detectors returns member detector configurations.
#[tokio::test]
async fn test_get_member_detectors() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let account_detail = aws_sdk_guardduty::types::AccountDetail::builder()
        .account_id("444444444444")
        .email("member-det@example.com")
        .build();

    client
        .create_members()
        .detector_id(&detector_id)
        .account_details(account_detail)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_member_detectors()
        .detector_id(&detector_id)
        .account_ids("444444444444")
        .send()
        .await
        .unwrap();

    let configs = resp.member_data_source_configurations();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].account_id().unwrap_or_default(), "444444444444");
}

/// Test update_member_detectors updates member feature configuration.
#[tokio::test]
async fn test_update_member_detectors() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let account_detail = aws_sdk_guardduty::types::AccountDetail::builder()
        .account_id("555555555555")
        .email("member-upd@example.com")
        .build();

    client
        .create_members()
        .detector_id(&detector_id)
        .account_details(account_detail)
        .send()
        .await
        .unwrap();

    let feature = aws_sdk_guardduty::types::MemberFeaturesConfiguration::builder()
        .name(aws_sdk_guardduty::types::OrgFeature::from("EKS_AUDIT_LOGS"))
        .status(aws_sdk_guardduty::types::FeatureStatus::Enabled)
        .build();

    let resp = client
        .update_member_detectors()
        .detector_id(&detector_id)
        .account_ids("555555555555")
        .features(feature)
        .send()
        .await;

    assert!(resp.is_ok());
}

/// Test disassociate_members removes monitoring relationship.
#[tokio::test]
async fn test_disassociate_members() {
    let client = make_client().await;

    let create_resp = client.create_detector().enable(true).send().await.unwrap();
    let detector_id = create_resp.detector_id().unwrap().to_string();

    let account_detail = aws_sdk_guardduty::types::AccountDetail::builder()
        .account_id("666666666666")
        .email("member-dis@example.com")
        .build();

    client
        .create_members()
        .detector_id(&detector_id)
        .account_details(account_detail)
        .send()
        .await
        .unwrap();

    // Invite and start monitoring, then disassociate.
    client
        .invite_members()
        .detector_id(&detector_id)
        .account_ids("666666666666")
        .send()
        .await
        .unwrap();

    client
        .start_monitoring_members()
        .detector_id(&detector_id)
        .account_ids("666666666666")
        .send()
        .await
        .unwrap();

    let resp = client
        .disassociate_members()
        .detector_id(&detector_id)
        .account_ids("666666666666")
        .send()
        .await;

    assert!(resp.is_ok());

    // After disassociation the member relationship status should reflect it.
    let get_resp = client
        .get_members()
        .detector_id(&detector_id)
        .account_ids("666666666666")
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.members().len(), 1);
    // Status should have changed from Enabled to something else (Disassociated).
    let status = get_resp.members()[0]
        .relationship_status()
        .unwrap_or_default();
    assert_ne!(status, "Enabled");
}

// =============================================================================
// Invitation decline / delete tests
// =============================================================================

/// Test decline_invitations succeeds (no-op when there are no pending invitations).
#[tokio::test]
async fn test_decline_invitations() {
    let client = make_client().await;

    let resp = client
        .decline_invitations()
        .account_ids("111111111111")
        .send()
        .await;

    assert!(resp.is_ok());
}

/// Test delete_invitations succeeds.
#[tokio::test]
async fn test_delete_invitations() {
    let client = make_client().await;

    let resp = client
        .delete_invitations()
        .account_ids("111111111111")
        .send()
        .await;

    assert!(resp.is_ok());
}

// =============================================================================
// List malware scans test
// =============================================================================

/// Test list_malware_scans returns scans across all detectors.
#[tokio::test]
async fn test_list_malware_scans() {
    let client = make_client().await;

    client.create_detector().enable(true).send().await.unwrap();

    client
        .start_malware_scan()
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:instance/i-list-test")
        .send()
        .await
        .unwrap();

    let resp = client.list_malware_scans().send().await.unwrap();

    assert!(!resp.scans().is_empty());
}
