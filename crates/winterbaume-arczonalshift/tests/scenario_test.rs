/// Scenario tests for ARC Zonal Shift — end-to-end use-case chains.
///
/// These tests exercise multi-operation workflows rather than individual API calls.
use aws_sdk_arczonalshift::config::BehaviorVersion;
use aws_sdk_arczonalshift::types::{
    ControlCondition, ControlConditionType, ZonalAutoshiftStatus, ZonalShiftStatus,
};
use winterbaume_arczonalshift::ArcZonalShiftService;
use winterbaume_core::MockAws;

const ALB_ARN: &str =
    "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc";
const ALARM_ARN: &str = "arn:aws:cloudwatch:us-east-1:123456789012:alarm:practice-run-outcome";

async fn make_client() -> aws_sdk_arczonalshift::Client {
    let mock = MockAws::builder()
        .with_service(ArcZonalShiftService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_arczonalshift::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_arczonalshift::Client::new(&config)
}

fn outcome_alarm() -> ControlCondition {
    ControlCondition::builder()
        .alarm_identifier(ALARM_ARN)
        .r#type(ControlConditionType::Cloudwatch)
        .build()
        .unwrap()
}

/// Scenario: Full zonal shift lifecycle
///
/// A resource gets a zonal shift started, updated to extend its expiry and change the
/// comment, then cancelled. Verifies that listing reflects status transitions correctly.
#[tokio::test]
async fn test_zonal_shift_full_lifecycle() {
    let client = make_client().await;

    // 1. Start a zonal shift.
    let started = client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az1")
        .expires_in("1h")
        .comment("initial")
        .send()
        .await
        .expect("start_zonal_shift");

    let id = started.zonal_shift_id().to_string();
    assert_eq!(started.status(), &ZonalShiftStatus::Active);

    // 2. Update the comment and expiry.
    let updated = client
        .update_zonal_shift()
        .zonal_shift_id(&id)
        .expires_in("3h")
        .comment("extended")
        .send()
        .await
        .expect("update_zonal_shift");
    assert_eq!(updated.comment(), "extended");
    assert!(updated.expiry_time().secs() > started.expiry_time().secs());

    // 3. List active shifts — should show exactly one.
    let list_active = client
        .list_zonal_shifts()
        .status(ZonalShiftStatus::Active)
        .send()
        .await
        .expect("list_zonal_shifts active");
    assert_eq!(list_active.items().len(), 1, "expected one active shift");

    // 4. Cancel the shift.
    let cancelled = client
        .cancel_zonal_shift()
        .zonal_shift_id(&id)
        .send()
        .await
        .expect("cancel_zonal_shift");
    assert_eq!(cancelled.status(), &ZonalShiftStatus::Canceled);

    // 5. Active list is now empty; cancelled list contains one entry.
    let list_active2 = client
        .list_zonal_shifts()
        .status(ZonalShiftStatus::Active)
        .send()
        .await
        .expect("list after cancel");
    assert!(list_active2.items().is_empty());

    let list_cancelled = client
        .list_zonal_shifts()
        .status(ZonalShiftStatus::Canceled)
        .send()
        .await
        .expect("list cancelled");
    assert_eq!(list_cancelled.items().len(), 1);

    // 6. The managed resource is still visible.
    let resource = client
        .get_managed_resource()
        .resource_identifier(ALB_ARN)
        .send()
        .await
        .expect("get_managed_resource");
    assert_eq!(resource.arn(), Some(ALB_ARN));
    // No active zonal shifts on the resource.
    assert!(resource.zonal_shifts().is_empty());
}

/// Scenario: Practice run configuration enables zonal autoshift
///
/// Creates a practice run configuration, enables zonal autoshift for the resource, runs
/// a practice run, cancels it, then tears everything down.
#[tokio::test]
async fn test_practice_run_enables_autoshift_pipeline() {
    let client = make_client().await;

    // 1. Create practice run configuration.
    let created = client
        .create_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .outcome_alarms(outcome_alarm())
        .blocked_dates("2026-12-25")
        .send()
        .await
        .expect("create_practice_run_configuration");
    assert_eq!(
        created.zonal_autoshift_status(),
        &ZonalAutoshiftStatus::Disabled
    );

    // 2. Enable zonal autoshift (requires practice run config to exist).
    let enabled = client
        .update_zonal_autoshift_configuration()
        .resource_identifier(ALB_ARN)
        .zonal_autoshift_status(ZonalAutoshiftStatus::Enabled)
        .send()
        .await
        .expect("update_zonal_autoshift_configuration");
    assert_eq!(
        enabled.zonal_autoshift_status(),
        &ZonalAutoshiftStatus::Enabled
    );

    // 3. Start a practice run.
    let practice = client
        .start_practice_run()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az2")
        .comment("practice")
        .send()
        .await
        .expect("start_practice_run");
    assert!(!practice.zonal_shift_id().is_empty());

    // 4. Resource shows the practice run in progress.
    let resource = client
        .get_managed_resource()
        .resource_identifier(ALB_ARN)
        .send()
        .await
        .expect("get_managed_resource");
    assert_eq!(resource.zonal_shifts().len(), 1);

    // 5. Cancel the practice run.
    let cancelled = client
        .cancel_practice_run()
        .zonal_shift_id(practice.zonal_shift_id())
        .send()
        .await
        .expect("cancel_practice_run");
    assert_eq!(cancelled.status(), &ZonalShiftStatus::Canceled);

    // 6. Disable autoshift before deleting the configuration.
    client
        .update_zonal_autoshift_configuration()
        .resource_identifier(ALB_ARN)
        .zonal_autoshift_status(ZonalAutoshiftStatus::Disabled)
        .send()
        .await
        .expect("disable_zonal_autoshift");

    // 7. Delete the practice run configuration.
    let deleted = client
        .delete_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .send()
        .await
        .expect("delete_practice_run_configuration");
    assert_eq!(
        deleted.zonal_autoshift_status(),
        &ZonalAutoshiftStatus::Disabled
    );

    // 8. Resource managed list is still present but with no practice config.
    let list = client.list_managed_resources().send().await.expect("list");
    assert_eq!(list.items().len(), 1);
}
