use std::sync::{Arc, Mutex};

use aws_sdk_autoscalingplans::config::BehaviorVersion;
use aws_sdk_autoscalingplans::types::{
    ApplicationSource, ScalableDimension, ScalingInstruction, ServiceNamespace,
    TargetTrackingConfiguration,
};
use winterbaume_autoscalingplans::AutoScalingPlansService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_autoscalingplans::Client {
    let mock = MockAws::builder()
        .with_service(AutoScalingPlansService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_autoscalingplans::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_autoscalingplans::Client::new(&config)
}

fn minimal_instruction() -> ScalingInstruction {
    let target = TargetTrackingConfiguration::builder()
        .target_value(50.0)
        .predefined_scaling_metric_specification(
            aws_sdk_autoscalingplans::types::PredefinedScalingMetricSpecification::builder()
                .predefined_scaling_metric_type(
                    aws_sdk_autoscalingplans::types::ScalingMetricType::AsgAverageCpuUtilization,
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    ScalingInstruction::builder()
        .service_namespace(ServiceNamespace::Autoscaling)
        .resource_id("autoScalingGroup/test-asg")
        .scalable_dimension(ScalableDimension::AutoScalingGroupDesiredCapacity)
        .min_capacity(1)
        .max_capacity(10)
        .target_tracking_configurations(target)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_and_describe_scaling_plan() {
    let client = make_client().await;
    let app_source = ApplicationSource::builder()
        .cloud_formation_stack_arn(
            "arn:aws:cloudformation:us-east-1:123456789012:stack/my-stack/abc-1234",
        )
        .build();
    let resp = client
        .create_scaling_plan()
        .scaling_plan_name("test-plan")
        .application_source(app_source)
        .scaling_instructions(minimal_instruction())
        .send()
        .await
        .expect("create");
    assert_eq!(resp.scaling_plan_version(), 1);

    let describe = client
        .describe_scaling_plans()
        .send()
        .await
        .expect("describe");
    let plans = describe.scaling_plans();
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].scaling_plan_name(), "test-plan");
}

#[tokio::test]
async fn test_create_duplicate_plan_fails() {
    let client = make_client().await;
    let app_source = ApplicationSource::builder()
        .cloud_formation_stack_arn(
            "arn:aws:cloudformation:us-east-1:123456789012:stack/dup/abc-1234",
        )
        .build();
    client
        .create_scaling_plan()
        .scaling_plan_name("dup-plan")
        .application_source(app_source.clone())
        .scaling_instructions(minimal_instruction())
        .send()
        .await
        .expect("first create");
    let err = client
        .create_scaling_plan()
        .scaling_plan_name("dup-plan")
        .application_source(app_source)
        .scaling_instructions(minimal_instruction())
        .send()
        .await
        .expect_err("dup should fail");
    assert!(format!("{:?}", err).contains("ConcurrentUpdateException"));
}

#[tokio::test]
async fn test_delete_scaling_plan() {
    let client = make_client().await;
    let app_source = ApplicationSource::builder()
        .cloud_formation_stack_arn(
            "arn:aws:cloudformation:us-east-1:123456789012:stack/del/abc-1234",
        )
        .build();
    client
        .create_scaling_plan()
        .scaling_plan_name("del-plan")
        .application_source(app_source)
        .scaling_instructions(minimal_instruction())
        .send()
        .await
        .expect("create");

    client
        .delete_scaling_plan()
        .scaling_plan_name("del-plan")
        .scaling_plan_version(1)
        .send()
        .await
        .expect("delete");

    let err = client
        .delete_scaling_plan()
        .scaling_plan_name("del-plan")
        .scaling_plan_version(1)
        .send()
        .await
        .expect_err("second delete should fail");
    assert!(format!("{:?}", err).contains("ObjectNotFoundException"));
}

#[tokio::test]
async fn test_describe_scaling_plan_resources() {
    let client = make_client().await;
    let app_source = ApplicationSource::builder()
        .cloud_formation_stack_arn(
            "arn:aws:cloudformation:us-east-1:123456789012:stack/res/abc-1234",
        )
        .build();
    client
        .create_scaling_plan()
        .scaling_plan_name("res-plan")
        .application_source(app_source)
        .scaling_instructions(minimal_instruction())
        .send()
        .await
        .expect("create");

    let resp = client
        .describe_scaling_plan_resources()
        .scaling_plan_name("res-plan")
        .scaling_plan_version(1)
        .send()
        .await
        .expect("describe resources");
    assert_eq!(resp.scaling_plan_resources().len(), 0);
}

#[tokio::test]
async fn test_get_forecast_data_returns_empty() {
    let client = make_client().await;
    let app_source = ApplicationSource::builder()
        .cloud_formation_stack_arn(
            "arn:aws:cloudformation:us-east-1:123456789012:stack/fc/abc-1234",
        )
        .build();
    client
        .create_scaling_plan()
        .scaling_plan_name("fc-plan")
        .application_source(app_source)
        .scaling_instructions(minimal_instruction())
        .send()
        .await
        .expect("create");

    let resp = client
        .get_scaling_plan_resource_forecast_data()
        .scaling_plan_name("fc-plan")
        .scaling_plan_version(1)
        .service_namespace(ServiceNamespace::Autoscaling)
        .resource_id("autoScalingGroup/test-asg")
        .scalable_dimension(ScalableDimension::AutoScalingGroupDesiredCapacity)
        .forecast_data_type(aws_sdk_autoscalingplans::types::ForecastDataType::CapacityForecast)
        .start_time(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .end_time(aws_smithy_types::DateTime::from_secs(1_700_086_400))
        .send()
        .await
        .expect("get_forecast");
    assert_eq!(resp.datapoints().len(), 0);
}

#[tokio::test]
async fn test_describe_missing_plan_resources() {
    let client = make_client().await;
    let err = client
        .describe_scaling_plan_resources()
        .scaling_plan_name("missing")
        .scaling_plan_version(1)
        .send()
        .await
        .expect_err("missing should fail");
    assert!(format!("{:?}", err).contains("ObjectNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AutoScalingPlansService::new();
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
        .expect("restore");
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
}
