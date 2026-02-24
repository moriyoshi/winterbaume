use std::sync::{Arc, Mutex};

use aws_sdk_arczonalshift::config::BehaviorVersion;
use aws_sdk_arczonalshift::types::{
    ControlCondition, ControlConditionType, ZonalAutoshiftStatus, ZonalShiftStatus,
};
use winterbaume_arczonalshift::ArcZonalShiftService;
use winterbaume_core::{MockAws, StatefulService};

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

#[tokio::test]
async fn test_start_and_cancel_zonal_shift() {
    let client = make_client().await;

    let start = client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az1")
        .expires_in("1h")
        .comment("rotating az1")
        .send()
        .await
        .expect("start_zonal_shift should succeed");

    let id = start.zonal_shift_id().to_string();
    assert!(!id.is_empty());
    assert_eq!(start.away_from(), "use1-az1");
    assert_eq!(start.status(), &ZonalShiftStatus::Active);

    let cancel = client
        .cancel_zonal_shift()
        .zonal_shift_id(&id)
        .send()
        .await
        .expect("cancel_zonal_shift should succeed");
    assert_eq!(cancel.status(), &ZonalShiftStatus::Canceled);
}

#[tokio::test]
async fn test_start_zonal_shift_validation() {
    let client = make_client().await;
    let err = client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az1")
        .expires_in("bogus")
        .comment("c")
        .send()
        .await
        .expect_err("invalid expiresIn should fail");
    let msg = format!("{err:?}");
    assert!(msg.contains("ValidationException"), "got: {msg}");
}

#[tokio::test]
async fn test_two_active_zonal_shifts_conflict() {
    let client = make_client().await;
    client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az1")
        .expires_in("1h")
        .comment("first")
        .send()
        .await
        .expect("first shift should succeed");

    let err = client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az2")
        .expires_in("1h")
        .comment("second")
        .send()
        .await
        .expect_err("second shift should conflict");
    let msg = format!("{err:?}");
    assert!(msg.contains("ConflictException"), "got: {msg}");
}

#[tokio::test]
async fn test_update_zonal_shift_extends_expiry() {
    let client = make_client().await;
    let start = client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az1")
        .expires_in("30m")
        .comment("orig")
        .send()
        .await
        .expect("start should succeed");

    let updated = client
        .update_zonal_shift()
        .zonal_shift_id(start.zonal_shift_id())
        .expires_in("3h")
        .comment("extended")
        .send()
        .await
        .expect("update should succeed");

    assert_eq!(updated.comment(), "extended");
    assert!(updated.expiry_time().secs() > start.expiry_time().secs());
}

#[tokio::test]
async fn test_cancel_zonal_shift_not_found() {
    let client = make_client().await;
    let err = client
        .cancel_zonal_shift()
        .zonal_shift_id("missing-id")
        .send()
        .await
        .expect_err("cancel of missing id should fail");
    let msg = format!("{err:?}");
    assert!(msg.contains("ResourceNotFoundException"), "got: {msg}");
}

#[tokio::test]
async fn test_list_zonal_shifts_filters_by_status() {
    let client = make_client().await;
    let s1 = client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az1")
        .expires_in("1h")
        .comment("c")
        .send()
        .await
        .expect("start should succeed");
    client
        .cancel_zonal_shift()
        .zonal_shift_id(s1.zonal_shift_id())
        .send()
        .await
        .expect("cancel should succeed");

    let active = client
        .list_zonal_shifts()
        .status(ZonalShiftStatus::Active)
        .send()
        .await
        .expect("list active should succeed");
    assert!(active.items().is_empty());

    let canceled = client
        .list_zonal_shifts()
        .status(ZonalShiftStatus::Canceled)
        .send()
        .await
        .expect("list canceled should succeed");
    assert_eq!(canceled.items().len(), 1);
}

#[tokio::test]
async fn test_practice_run_configuration_lifecycle() {
    let client = make_client().await;

    let create = client
        .create_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .outcome_alarms(outcome_alarm())
        .blocked_dates("2026-12-25")
        .send()
        .await
        .expect("create_practice_run_configuration should succeed");
    assert_eq!(
        create.zonal_autoshift_status(),
        &ZonalAutoshiftStatus::Disabled,
    );
    let cfg = create
        .practice_run_configuration()
        .expect("practice_run_configuration must be present");
    assert_eq!(cfg.outcome_alarms().len(), 1);

    let updated = client
        .update_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .outcome_alarms(outcome_alarm())
        .blocked_windows("MON:18:30-MON:19:30")
        .send()
        .await
        .expect("update should succeed");
    assert_eq!(
        updated
            .practice_run_configuration()
            .unwrap()
            .blocked_windows()
            .len(),
        1
    );

    let deleted = client
        .delete_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .send()
        .await
        .expect("delete should succeed");
    assert_eq!(
        deleted.zonal_autoshift_status(),
        &ZonalAutoshiftStatus::Disabled
    );
}

#[tokio::test]
async fn test_create_practice_run_requires_outcome_alarms() {
    let client = make_client().await;
    let err = client
        .create_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .send()
        .await
        .expect_err("missing outcome alarms should fail");
    let msg = format!("{err:?}");
    assert!(msg.contains("ValidationException"), "got: {msg}");
}

#[tokio::test]
async fn test_update_zonal_autoshift_requires_practice_run() {
    let client = make_client().await;
    let err = client
        .update_zonal_autoshift_configuration()
        .resource_identifier(ALB_ARN)
        .zonal_autoshift_status(ZonalAutoshiftStatus::Enabled)
        .send()
        .await
        .expect_err("enable without practice run config should fail");
    let msg = format!("{err:?}");
    assert!(msg.contains("ConflictException"), "got: {msg}");

    client
        .create_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .outcome_alarms(outcome_alarm())
        .send()
        .await
        .expect("create practice run config should succeed");
    let resp = client
        .update_zonal_autoshift_configuration()
        .resource_identifier(ALB_ARN)
        .zonal_autoshift_status(ZonalAutoshiftStatus::Enabled)
        .send()
        .await
        .expect("now enable should succeed");
    assert_eq!(
        resp.zonal_autoshift_status(),
        &ZonalAutoshiftStatus::Enabled
    );
}

#[tokio::test]
async fn test_get_managed_resource_after_shift() {
    let client = make_client().await;
    let s = client
        .start_zonal_shift()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az2")
        .expires_in("1h")
        .comment("c")
        .send()
        .await
        .expect("start");
    let resp = client
        .get_managed_resource()
        .resource_identifier(ALB_ARN)
        .send()
        .await
        .expect("get_managed_resource");
    assert_eq!(resp.arn(), Some(ALB_ARN));
    assert_eq!(resp.zonal_shifts().len(), 1);
    assert_eq!(resp.zonal_shifts()[0].zonal_shift_id(), s.zonal_shift_id());
}

#[tokio::test]
async fn test_list_managed_resources_after_practice_config() {
    let client = make_client().await;
    client
        .create_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .outcome_alarms(outcome_alarm())
        .send()
        .await
        .expect("create");
    let resp = client.list_managed_resources().send().await.expect("list");
    assert_eq!(resp.items().len(), 1);
    assert_eq!(resp.items()[0].arn(), Some(ALB_ARN));
}

#[tokio::test]
async fn test_get_managed_resource_not_found() {
    let client = make_client().await;
    let err = client
        .get_managed_resource()
        .resource_identifier("arn:aws:elasticloadbalancing:us-east-1:1:loadbalancer/x")
        .send()
        .await
        .expect_err("missing resource");
    let msg = format!("{err:?}");
    assert!(msg.contains("ResourceNotFoundException"), "got: {msg}");
}

#[tokio::test]
async fn test_list_autoshifts_returns_empty() {
    let client = make_client().await;
    let resp = client.list_autoshifts().send().await.expect("list");
    assert!(resp.items().is_empty());
}

#[tokio::test]
async fn test_practice_run_lifecycle() {
    let client = make_client().await;
    client
        .create_practice_run_configuration()
        .resource_identifier(ALB_ARN)
        .outcome_alarms(outcome_alarm())
        .send()
        .await
        .expect("create config");

    let started = client
        .start_practice_run()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az3")
        .comment("p1")
        .send()
        .await
        .expect("start practice run");
    assert!(!started.zonal_shift_id().is_empty());

    let canceled = client
        .cancel_practice_run()
        .zonal_shift_id(started.zonal_shift_id())
        .send()
        .await
        .expect("cancel practice run");
    assert_eq!(canceled.status(), &ZonalShiftStatus::Canceled);
}

#[tokio::test]
async fn test_start_practice_run_without_config_fails() {
    let client = make_client().await;
    let err = client
        .start_practice_run()
        .resource_identifier(ALB_ARN)
        .away_from("use1-az3")
        .comment("p1")
        .send()
        .await
        .expect_err("missing practice config");
    let msg = format!("{err:?}");
    assert!(msg.contains("ResourceNotFoundException"), "got: {msg}");
}

#[tokio::test]
async fn test_observer_notification_status_round_trip() {
    let client = make_client().await;
    let initial = client
        .get_autoshift_observer_notification_status()
        .send()
        .await
        .expect("get");
    use aws_sdk_arczonalshift::types::AutoshiftObserverNotificationStatus;
    assert_eq!(
        initial.status(),
        &AutoshiftObserverNotificationStatus::Disabled
    );

    let resp = client
        .update_autoshift_observer_notification_status()
        .status(AutoshiftObserverNotificationStatus::Enabled)
        .send()
        .await
        .expect("update");
    assert_eq!(resp.status(), &AutoshiftObserverNotificationStatus::Enabled);

    let final_state = client
        .get_autoshift_observer_notification_status()
        .send()
        .await
        .expect("get after update");
    assert_eq!(
        final_state.status(),
        &AutoshiftObserverNotificationStatus::Enabled
    );
}

#[tokio::test]
async fn test_state_view_snapshot_restore_round_trip() {
    use winterbaume_arczonalshift::views::{
        ArcZonalShiftStateView, ControlConditionView, ManagedResourceView,
        PracticeRunConfigurationView,
    };

    let svc = ArcZonalShiftService::new();

    let mut view = ArcZonalShiftStateView::default();
    view.practice_run_configurations.insert(
        ALB_ARN.to_string(),
        PracticeRunConfigurationView {
            resource_identifier: ALB_ARN.to_string(),
            blocking_alarms: vec![],
            outcome_alarms: vec![ControlConditionView {
                alarm_identifier: ALARM_ARN.to_string(),
                r#type: "CLOUDWATCH".to_string(),
            }],
            blocked_windows: vec![],
            allowed_windows: vec![],
            blocked_dates: vec![],
        },
    );
    view.managed_resources.insert(
        ALB_ARN.to_string(),
        ManagedResourceView {
            resource_identifier: ALB_ARN.to_string(),
            name: "my-alb".to_string(),
            arn: ALB_ARN.to_string(),
            availability_zones: vec!["use1-az1".to_string()],
            applied_weights: Default::default(),
            zonal_autoshift_status: "DISABLED".to_string(),
        },
    );

    svc.restore("123456789012", "us-east-1", view.clone())
        .await
        .expect("restore");

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.practice_run_configurations.len(), 1);
    assert!(snapshot.practice_run_configurations.contains_key(ALB_ARN));
    assert_eq!(snapshot.managed_resources.len(), 1);
}

#[tokio::test]
async fn test_state_change_listener_fires_on_mutation() {
    let svc = ArcZonalShiftService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore should succeed");

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}
