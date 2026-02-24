//! Integration tests for winterbaume EMR Serverless service.
//! Translated from moto's test_emrserverless Python tests.

use aws_sdk_emrserverless::config::BehaviorVersion;
use aws_sdk_emrserverless::types::{JobDriver, SparkSubmit};
use winterbaume_core::MockAws;
use winterbaume_emrserverless::EmrServerlessService;

const DEFAULT_REGION: &str = "us-east-1";
const ACCOUNT_ID: &str = "123456789012";

async fn make_emr_serverless_client() -> aws_sdk_emrserverless::Client {
    let mock = MockAws::builder()
        .with_service(EmrServerlessService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emrserverless::config::Region::new(DEFAULT_REGION))
        .load()
        .await;

    aws_sdk_emrserverless::Client::new(&config)
}

fn spark_job_driver(entry_point: &str) -> JobDriver {
    JobDriver::SparkSubmit(
        SparkSubmit::builder()
            .entry_point(entry_point)
            .entry_point_arguments("-h")
            .spark_submit_parameters("--num-executors 1")
            .build()
            .unwrap(),
    )
}

fn simple_spark_job_driver() -> JobDriver {
    JobDriver::SparkSubmit(
        SparkSubmit::builder()
            .entry_point("test.jar")
            .build()
            .unwrap(),
    )
}

fn execution_role() -> String {
    format!("arn:aws:iam::{ACCOUNT_ID}:role/emr-serverless-role")
}

// ==================== Create Application Tests ====================

/// Translated from moto TestCreateApplication.test_create_application
#[tokio::test]
async fn test_create_application() {
    let client = make_emr_serverless_client().await;

    let resp = client
        .create_application()
        .name("test-emr-serverless-application")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .expect("create_application should succeed");

    assert_eq!(resp.name(), Some("test-emr-serverless-application"));
    assert_eq!(resp.application_id().len(), 16);
    assert!(
        resp.application_id()
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    );

    let expected_arn = format!(
        "arn:aws:emr-serverless:{DEFAULT_REGION}:{ACCOUNT_ID}:/applications/{}",
        resp.application_id()
    );
    assert_eq!(resp.arn(), expected_arn);
}

/// Translated from moto TestCreateApplication.test_create_application_incorrect_type
#[tokio::test]
async fn test_create_application_incorrect_type() {
    let client = make_emr_serverless_client().await;

    let err = client
        .create_application()
        .name("test-emr-serverless-application")
        .r#type("SPARK3")
        .release_label("emr-6.6.0")
        .send()
        .await;

    assert!(err.is_err(), "creating app with incorrect type should fail");
}

/// Translated from moto TestCreateApplication.test_create_application_incorrect_release_label
#[tokio::test]
async fn test_create_application_incorrect_release_label() {
    let client = make_emr_serverless_client().await;

    let err = client
        .create_application()
        .name("test-emr-serverless-application")
        .r#type("SPARK")
        .release_label("emr-fake")
        .send()
        .await;

    assert!(
        err.is_err(),
        "creating app with incorrect release label should fail"
    );
}

// ==================== Get Application Tests ====================

/// Translated from moto TestGetApplication.test_filtering (basic)
#[tokio::test]
async fn test_get_application_basic() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-emr-serverless-application")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let app = resp.application().unwrap();
    assert_eq!(app.application_id(), &app_id);
    assert_eq!(app.name(), Some("test-emr-serverless-application"));
    assert_eq!(
        app.arn(),
        format!("arn:aws:emr-serverless:{DEFAULT_REGION}:{ACCOUNT_ID}:/applications/{app_id}")
    );
    assert_eq!(app.release_label(), "emr-6.6.0");
    assert_eq!(app.r#type(), "Spark");
    assert_eq!(
        app.state(),
        &aws_sdk_emrserverless::types::ApplicationState::Started
    );
    // Auto start should be enabled by default
    assert_eq!(
        app.auto_start_configuration().unwrap().enabled(),
        Some(true)
    );
    // Auto stop defaults
    assert_eq!(app.auto_stop_configuration().unwrap().enabled(), Some(true));
    assert_eq!(
        app.auto_stop_configuration()
            .unwrap()
            .idle_timeout_minutes(),
        Some(15)
    );
}

/// Translated from moto TestGetApplication.test_invalid_application_id
#[tokio::test]
async fn test_get_application_invalid_id() {
    let client = make_emr_serverless_client().await;

    let err = client
        .get_application()
        .application_id("fake_application_id")
        .send()
        .await;

    assert!(err.is_err(), "get non-existent application should fail");
}

/// Translated from moto TestGetApplication.test_filtering (with initialCapacity)
#[tokio::test]
async fn test_get_application_with_initial_capacity() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-emr-serverless-application")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .initial_capacity(
            "Driver",
            aws_sdk_emrserverless::types::InitialCapacityConfig::builder()
                .worker_count(1)
                .worker_configuration(
                    aws_sdk_emrserverless::types::WorkerResourceConfig::builder()
                        .cpu("2 vCPU")
                        .memory("4 GB")
                        .disk("20 GB")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let app = resp.application().unwrap();
    let ic = app.initial_capacity().unwrap();
    let driver = ic.get("Driver").unwrap();
    assert_eq!(driver.worker_count(), 1);
    let wc = driver.worker_configuration().unwrap();
    assert_eq!(wc.cpu(), "2 vCPU");
    assert_eq!(wc.memory(), "4 GB");
    assert_eq!(wc.disk(), Some("20 GB"));
}

/// Translated from moto TestGetApplication.test_filtering (with maximumCapacity)
#[tokio::test]
async fn test_get_application_with_maximum_capacity() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-emr-serverless-application")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .maximum_capacity(
            aws_sdk_emrserverless::types::MaximumAllowedResources::builder()
                .cpu("400 vCPU")
                .memory("1024 GB")
                .disk("1000 GB")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let app = resp.application().unwrap();
    let mc = app.maximum_capacity().unwrap();
    assert_eq!(mc.cpu(), "400 vCPU");
    assert_eq!(mc.memory(), "1024 GB");
    assert_eq!(mc.disk(), Some("1000 GB"));
}

/// Translated from moto TestGetApplication.test_filtering (with networkConfiguration)
#[tokio::test]
async fn test_get_application_with_network_configuration() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-emr-serverless-application")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .network_configuration(
            aws_sdk_emrserverless::types::NetworkConfiguration::builder()
                .subnet_ids("subnet-0123456789abcdefg")
                .security_group_ids("sg-0123456789abcdefg")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let app = resp.application().unwrap();
    let nc = app.network_configuration().unwrap();
    assert_eq!(nc.subnet_ids(), &["subnet-0123456789abcdefg"]);
    assert_eq!(nc.security_group_ids(), &["sg-0123456789abcdefg"]);
}

// ==================== Delete Application Tests ====================

/// Translated from moto TestDeleteApplication.test_invalid_application_id
#[tokio::test]
async fn test_delete_application_invalid_id() {
    let client = make_emr_serverless_client().await;

    let err = client
        .delete_application()
        .application_id("fake_application_id")
        .send()
        .await;

    assert!(err.is_err(), "delete non-existent application should fail");
}

/// Translated from moto TestDeleteApplication - STOPPED application can be deleted
#[tokio::test]
async fn test_delete_application_stopped() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Stop it first
    client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    // Now delete should succeed
    let resp = client
        .delete_application()
        .application_id(&app_id)
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Translated from moto TestDeleteApplication - STARTED application cannot be deleted
#[tokio::test]
async fn test_delete_application_started_fails() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Application is in STARTED state by default, so delete should fail
    let err = client
        .delete_application()
        .application_id(&app_id)
        .send()
        .await;
    assert!(err.is_err(), "deleting STARTED application should fail");
}

// ==================== List Applications Tests ====================

/// Translated from moto TestListApplication.test_response_context
#[tokio::test]
async fn test_list_applications() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-emr-serverless-application-STARTED")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client.list_applications().send().await.unwrap();
    let apps = resp.applications();
    assert!(!apps.is_empty());

    let app = apps.iter().find(|a| a.id() == app_id).unwrap();
    assert_eq!(app.name(), Some("test-emr-serverless-application-STARTED"));
    let expected_arn =
        format!("arn:aws:emr-serverless:{DEFAULT_REGION}:{ACCOUNT_ID}:/applications/{app_id}");
    assert_eq!(app.arn(), expected_arn);
    assert_eq!(app.release_label(), "emr-6.6.0");
    assert_eq!(app.r#type(), "Spark");
    assert_eq!(
        app.state(),
        &aws_sdk_emrserverless::types::ApplicationState::Started
    );
}

/// Translated from moto TestListApplication.test_filtering (maxResults)
#[tokio::test]
async fn test_list_applications_max_results() {
    let client = make_emr_serverless_client().await;

    // Create 2 applications
    client
        .create_application()
        .name("app1")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();
    client
        .create_application()
        .name("app2")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_applications()
        .max_results(1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.applications().len(), 1);
}

// ==================== Start/Stop Application Tests ====================

/// Translated from moto TestStartApplication.test_valid_application_id
#[tokio::test]
async fn test_start_application() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Stop it first, then start
    client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .start_application()
        .application_id(&app_id)
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Translated from moto TestStartApplication.test_invalid_application_id
#[tokio::test]
async fn test_start_application_invalid_id() {
    let client = make_emr_serverless_client().await;

    let err = client
        .start_application()
        .application_id("fake_application_id")
        .send()
        .await;
    assert!(err.is_err(), "start non-existent application should fail");
}

/// Translated from moto TestStopApplication.test_valid_application_id
#[tokio::test]
async fn test_stop_application() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Translated from moto TestStopApplication.test_invalid_application_id
#[tokio::test]
async fn test_stop_application_invalid_id() {
    let client = make_emr_serverless_client().await;

    let err = client
        .stop_application()
        .application_id("fake_application_id")
        .send()
        .await;
    assert!(err.is_err(), "stop non-existent application should fail");
}

// ==================== Update Application Tests ====================

/// Translated from moto TestUpdateApplication.test_invalid_application_id
#[tokio::test]
async fn test_update_application_invalid_id() {
    let client = make_emr_serverless_client().await;

    let err = client
        .update_application()
        .application_id("fake_application_id")
        .send()
        .await;
    assert!(err.is_err(), "update non-existent application should fail");
}

/// Translated from moto TestUpdateApplication - STARTED cannot be updated
#[tokio::test]
async fn test_update_application_started_fails() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Application is STARTED, update should fail
    let err = client
        .update_application()
        .application_id(&app_id)
        .send()
        .await;
    assert!(err.is_err(), "update STARTED application should fail");
}

/// Translated from moto TestUpdateApplication - STOPPED can be updated
#[tokio::test]
async fn test_update_application_stopped() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Stop first
    client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    // Update should succeed
    let resp = client
        .update_application()
        .application_id(&app_id)
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Translated from moto TestUpdateApplication.test_valid_update (with initialCapacity)
#[tokio::test]
async fn test_update_application_with_initial_capacity() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-emr-serverless-application-STOPPED")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Stop first
    client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_application()
        .application_id(&app_id)
        .initial_capacity(
            "Driver",
            aws_sdk_emrserverless::types::InitialCapacityConfig::builder()
                .worker_count(1)
                .worker_configuration(
                    aws_sdk_emrserverless::types::WorkerResourceConfig::builder()
                        .cpu("2 vCPU")
                        .memory("4 GB")
                        .disk("20 GB")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let app = resp.application().unwrap();
    assert_eq!(app.application_id(), app_id);
    assert_eq!(
        app.state(),
        &aws_sdk_emrserverless::types::ApplicationState::Stopped
    );
    let ic = app.initial_capacity().unwrap();
    let driver = ic.get("Driver").unwrap();
    assert_eq!(driver.worker_count(), 1);
    let wc = driver.worker_configuration().unwrap();
    assert_eq!(wc.cpu(), "2 vCPU");
    assert_eq!(wc.memory(), "4 GB");
    assert_eq!(wc.disk(), Some("20 GB"));
}

// ==================== Job Run Tests ====================

/// Translated from moto TestStartJobRun.test_start_job_run
#[tokio::test]
async fn test_start_job_run() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .name("Test Job Run")
        .job_driver(spark_job_driver("test.jar"))
        .send()
        .await
        .expect("start_job_run should succeed");

    assert_eq!(resp.application_id(), app_id);
    assert_eq!(resp.job_run_id().len(), 16);
    assert!(
        resp.job_run_id()
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    );
    assert!(resp.arn().starts_with(&format!(
        "arn:aws:emr-serverless:{DEFAULT_REGION}:{ACCOUNT_ID}:/applications/{app_id}/jobruns/"
    )));
}

/// Translated from moto TestStartJobRun.test_cross_account_role
#[tokio::test]
async fn test_start_job_run_cross_account() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let err = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn("arn:aws:iam::999999999999:role/emr-serverless-role")
        .job_driver(spark_job_driver("test.jar"))
        .send()
        .await;

    assert!(err.is_err(), "cross-account role should fail");
}

/// Translated from moto TestStartJobRun.test_run_timeout
#[tokio::test]
async fn test_start_job_run_timeout_too_short() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let err = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(spark_job_driver("test.jar"))
        .execution_timeout_minutes(4)
        .send()
        .await;

    assert!(err.is_err(), "timeout < 5 minutes should fail");
}

// ==================== Get Job Run Tests ====================

/// Translated from moto TestGetJobRun.test_get_job_run
#[tokio::test]
async fn test_get_job_run() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let job_resp = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .name("Test Job")
        .job_driver(spark_job_driver("test.jar"))
        .send()
        .await
        .unwrap();

    let job_run_id = job_resp.job_run_id().to_string();

    let resp = client
        .get_job_run()
        .application_id(&app_id)
        .job_run_id(&job_run_id)
        .send()
        .await
        .unwrap();

    let run = resp.job_run().unwrap();
    assert_eq!(run.application_id(), app_id);
    assert_eq!(run.job_run_id(), job_run_id);
}

/// Translated from moto TestGetJobRun.test_invalid_application_id
#[tokio::test]
async fn test_get_job_run_invalid_app() {
    let client = make_emr_serverless_client().await;

    let err = client
        .get_job_run()
        .application_id("fakeapp")
        .job_run_id("fakejob")
        .send()
        .await;
    assert!(err.is_err(), "get job run with invalid app should fail");
}

/// Translated from moto TestGetJobRun.test_invalid_job_run_id
#[tokio::test]
async fn test_get_job_run_invalid_job() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let err = client
        .get_job_run()
        .application_id(&app_id)
        .job_run_id("fakejobrun")
        .send()
        .await;
    assert!(err.is_err(), "get job run with invalid job should fail");
}

/// Translated from moto TestGetJobRun.test_job_not_belongs_to_other_application
#[tokio::test]
async fn test_get_job_run_wrong_application() {
    let client = make_emr_serverless_client().await;

    let app1 = client
        .create_application()
        .name("app1")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();
    let app1_id = app1.application_id().to_string();

    let app2 = client
        .create_application()
        .name("app2")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();
    let app2_id = app2.application_id().to_string();

    let job = client
        .start_job_run()
        .application_id(&app2_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    // Try to get job using app1 ID - should fail
    let err = client
        .get_job_run()
        .application_id(&app1_id)
        .job_run_id(job.job_run_id())
        .send()
        .await;
    assert!(err.is_err(), "get job run from wrong app should fail");
}

// ==================== List Job Runs Tests ====================

/// Translated from moto TestListJobRun.test_list_job_runs
#[tokio::test]
async fn test_list_job_runs() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let job1 = client
        .start_job_run()
        .application_id(&app_id)
        .name("Job 1")
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    let job2 = client
        .start_job_run()
        .application_id(&app_id)
        .name("Job 2")
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_job_runs()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.job_runs().len(), 2);
    let ids: Vec<&str> = resp.job_runs().iter().map(|r| r.id()).collect();
    assert!(ids.contains(&job1.job_run_id()));
    assert!(ids.contains(&job2.job_run_id()));
    assert!(resp.job_runs().iter().all(|r| r.application_id() == app_id));
}

/// Translated from moto TestListJobRun.test_invalid_application_id
#[tokio::test]
async fn test_list_job_runs_invalid_app() {
    let client = make_emr_serverless_client().await;

    let err = client
        .list_job_runs()
        .application_id("fakeapp")
        .send()
        .await;
    assert!(err.is_err(), "list job runs with invalid app should fail");
}

/// Translated from moto TestListJobRun.test_max_results
#[tokio::test]
async fn test_list_job_runs_max_results() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    for i in 0..2 {
        client
            .start_job_run()
            .application_id(&app_id)
            .name(format!("Job {i}"))
            .execution_role_arn(execution_role())
            .job_driver(simple_spark_job_driver())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_job_runs()
        .application_id(&app_id)
        .max_results(1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.job_runs().len(), 1);
}

// ==================== Cancel Job Run Tests ====================

/// Translated from moto TestCancelJobRun.test_cancel_job_run
#[tokio::test]
async fn test_cancel_job_run() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let job_resp = client
        .start_job_run()
        .application_id(&app_id)
        .name("Test Job")
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    let job_run_id = job_resp.job_run_id().to_string();

    let resp = client
        .cancel_job_run()
        .application_id(&app_id)
        .job_run_id(&job_run_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.application_id(), app_id);
    assert_eq!(resp.job_run_id(), job_run_id);
}

/// Translated from moto TestCancelJobRun.test_invalid_job_run_id
#[tokio::test]
async fn test_cancel_job_run_invalid_job() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let err = client
        .cancel_job_run()
        .application_id(&app_id)
        .job_run_id("fakejobrun")
        .send()
        .await;
    assert!(err.is_err(), "cancel invalid job run should fail");
}

// ==================== Additional Tests from Moto ====================

/// Translated from moto TestListApplication.test_filtering - filter by states
#[tokio::test]
async fn test_list_applications_state_filter() {
    let client = make_emr_serverless_client().await;

    // Create one STARTED and one STOPPED application
    let app1 = client
        .create_application()
        .name("app-started")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();
    let app1_id = app1.application_id().to_string();

    let app2 = client
        .create_application()
        .name("app-stopped")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();
    let app2_id = app2.application_id().to_string();

    // Stop app2
    client
        .stop_application()
        .application_id(&app2_id)
        .send()
        .await
        .unwrap();

    // Filter by STARTED: only app1
    let resp_started = client
        .list_applications()
        .states(aws_sdk_emrserverless::types::ApplicationState::Started)
        .send()
        .await
        .unwrap();
    let started_ids: Vec<&str> = resp_started.applications().iter().map(|a| a.id()).collect();
    assert!(
        started_ids.contains(&app1_id.as_str()),
        "STARTED filter should include app1"
    );
    assert!(
        !started_ids.contains(&app2_id.as_str()),
        "STARTED filter should exclude app2 (STOPPED)"
    );

    // Filter by STOPPED: only app2
    let resp_stopped = client
        .list_applications()
        .states(aws_sdk_emrserverless::types::ApplicationState::Stopped)
        .send()
        .await
        .unwrap();
    let stopped_ids: Vec<&str> = resp_stopped.applications().iter().map(|a| a.id()).collect();
    assert!(
        stopped_ids.contains(&app2_id.as_str()),
        "STOPPED filter should include app2"
    );
    assert!(
        !stopped_ids.contains(&app1_id.as_str()),
        "STOPPED filter should exclude app1 (STARTED)"
    );

    // Filter by unknown state: empty
    let resp_none = client
        .list_applications()
        .states(aws_sdk_emrserverless::types::ApplicationState::Terminated)
        .send()
        .await
        .unwrap();
    assert!(
        resp_none.applications().is_empty(),
        "filter by unused state should return empty list"
    );
}

/// Translated from moto TestListApplication.test_next_token - pagination
#[tokio::test]
async fn test_list_applications_next_token() {
    let client = make_emr_serverless_client().await;

    // Create 3 applications
    for i in 0..3 {
        client
            .create_application()
            .name(format!("paginated-app-{i}"))
            .r#type("SPARK")
            .release_label("emr-6.6.0")
            .send()
            .await
            .unwrap();
    }

    // First page: 2 items
    let page1 = client
        .list_applications()
        .max_results(2)
        .send()
        .await
        .unwrap();
    assert_eq!(page1.applications().len(), 2);
    let next_token = page1
        .next_token()
        .expect("should have next_token for first page");

    // Second page: remaining items
    let page2 = client
        .list_applications()
        .next_token(next_token)
        .send()
        .await
        .unwrap();
    assert!(
        !page2.applications().is_empty(),
        "second page should have items"
    );
    assert_eq!(
        page2.next_token(),
        None,
        "second page should have no next_token"
    );
}

/// Translated from moto TestStartJobRun.test_invalid_application_id
#[tokio::test]
async fn test_start_job_run_invalid_app() {
    let client = make_emr_serverless_client().await;

    let err = client
        .start_job_run()
        .application_id("fake_application_id")
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await;
    assert!(err.is_err(), "start_job_run with invalid app should fail");
}

/// Translated from moto TestListJobRun.test_application_states - filter by states
#[tokio::test]
async fn test_list_job_runs_state_filter() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-job-filter")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Start a job run
    let job = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    let job_run_id = job.job_run_id().to_string();

    // Filter by RUNNING: should include the job
    let resp_running = client
        .list_job_runs()
        .application_id(&app_id)
        .states(aws_sdk_emrserverless::types::JobRunState::Running)
        .send()
        .await
        .unwrap();
    let running_ids: Vec<&str> = resp_running.job_runs().iter().map(|r| r.id()).collect();
    assert!(
        running_ids.contains(&job_run_id.as_str()),
        "RUNNING filter should include the job"
    );

    // Filter by CANCELLED: should be empty (job was not cancelled)
    let resp_cancelled = client
        .list_job_runs()
        .application_id(&app_id)
        .states(aws_sdk_emrserverless::types::JobRunState::Cancelled)
        .send()
        .await
        .unwrap();
    assert!(
        resp_cancelled.job_runs().is_empty(),
        "CANCELLED filter should return empty list"
    );
}

/// Translated from moto TestUpdateApplication.test_valid_update (autoStart/autoStop config)
#[tokio::test]
async fn test_update_application_auto_configs() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-auto-config-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Stop first
    client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    // Update with custom autoStart/autoStop configs
    let resp = client
        .update_application()
        .application_id(&app_id)
        .auto_start_configuration(
            aws_sdk_emrserverless::types::AutoStartConfig::builder()
                .enabled(false)
                .build(),
        )
        .auto_stop_configuration(
            aws_sdk_emrserverless::types::AutoStopConfig::builder()
                .enabled(false)
                .idle_timeout_minutes(5)
                .build(),
        )
        .send()
        .await
        .expect("update with autoStart/autoStop should succeed");

    let app = resp.application().unwrap();
    assert_eq!(
        app.auto_start_configuration().unwrap().enabled(),
        Some(false)
    );
    assert_eq!(
        app.auto_stop_configuration().unwrap().enabled(),
        Some(false)
    );
    assert_eq!(
        app.auto_stop_configuration()
            .unwrap()
            .idle_timeout_minutes(),
        Some(5)
    );
}

// ==================== New Tests: Uncovered Scenarios ====================

/// HIVE is a valid application type — verify it is accepted and normalized
#[tokio::test]
async fn test_create_hive_application() {
    let client = make_emr_serverless_client().await;

    let resp = client
        .create_application()
        .name("test-hive-application")
        .r#type("HIVE")
        .release_label("emr-6.9.0")
        .send()
        .await
        .expect("create HIVE application should succeed");

    assert_eq!(resp.name(), Some("test-hive-application"));
    assert_eq!(resp.application_id().len(), 16);

    let get_resp = client
        .get_application()
        .application_id(resp.application_id())
        .send()
        .await
        .unwrap();

    let app = get_resp.application().unwrap();
    assert_eq!(app.r#type(), "Hive");
}

/// Create application with tags, verify they are persisted via get_application
#[tokio::test]
async fn test_create_application_with_tags() {
    let client = make_emr_serverless_client().await;

    let resp = client
        .create_application()
        .name("tagged-application")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .tags("env", "test")
        .tags("team", "data-eng")
        .send()
        .await
        .expect("create application with tags should succeed");

    let app_id = resp.application_id().to_string();

    let get_resp = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let app = get_resp.application().unwrap();
    let tags = app.tags();
    assert_eq!(
        tags.as_ref().and_then(|t| t.get("env")).map(|s| s.as_str()),
        Some("test")
    );
    assert_eq!(
        tags.as_ref()
            .and_then(|t| t.get("team"))
            .map(|s| s.as_str()),
        Some("data-eng")
    );
}

/// Execution timeout exactly at the minimum (5 minutes) should succeed
#[tokio::test]
async fn test_start_job_run_timeout_at_minimum() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-timeout-min")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let resp = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .execution_timeout_minutes(5)
        .send()
        .await
        .expect("execution timeout of exactly 5 minutes should succeed");

    assert_eq!(resp.application_id(), app_id);
}

/// Cancel a job run and then verify its state is CANCELLED via get_job_run
#[tokio::test]
async fn test_cancel_job_run_state_reflects_cancelled() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-cancel-verify")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let job_resp = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    let job_run_id = job_resp.job_run_id().to_string();

    client
        .cancel_job_run()
        .application_id(&app_id)
        .job_run_id(&job_run_id)
        .send()
        .await
        .expect("cancel_job_run should succeed");

    let get_resp = client
        .get_job_run()
        .application_id(&app_id)
        .job_run_id(&job_run_id)
        .send()
        .await
        .unwrap();

    let run = get_resp.job_run().unwrap();
    assert_eq!(
        run.state(),
        &aws_sdk_emrserverless::types::JobRunState::Cancelled
    );
}

/// Cancel job run with an invalid application ID returns an error
#[tokio::test]
async fn test_cancel_job_run_invalid_app() {
    let client = make_emr_serverless_client().await;

    let err = client
        .cancel_job_run()
        .application_id("nonexistent_app_id")
        .job_run_id("nonexistent_job_id")
        .send()
        .await;

    assert!(
        err.is_err(),
        "cancel job run with invalid application ID should fail"
    );
}

/// List job runs with next_token pagination returns correct pages
#[tokio::test]
async fn test_list_job_runs_next_token() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-job-pagination")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    for i in 0..3 {
        client
            .start_job_run()
            .application_id(&app_id)
            .name(format!("Paginated Job {i}"))
            .execution_role_arn(execution_role())
            .job_driver(simple_spark_job_driver())
            .send()
            .await
            .unwrap();
    }

    let page1 = client
        .list_job_runs()
        .application_id(&app_id)
        .max_results(2)
        .send()
        .await
        .unwrap();

    assert_eq!(page1.job_runs().len(), 2);
    let next_token = page1
        .next_token()
        .expect("first page should have a next_token");

    let page2 = client
        .list_job_runs()
        .application_id(&app_id)
        .next_token(next_token)
        .send()
        .await
        .unwrap();

    assert!(
        !page2.job_runs().is_empty(),
        "second page should have items"
    );
    assert_eq!(
        page2.next_token(),
        None,
        "second page should have no next_token"
    );

    // Verify no overlap
    let page1_ids: Vec<&str> = page1.job_runs().iter().map(|r| r.id()).collect();
    let page2_ids: Vec<&str> = page2.job_runs().iter().map(|r| r.id()).collect();
    for id in &page2_ids {
        assert!(
            !page1_ids.contains(id),
            "pages should not overlap on job run IDs"
        );
    }
}

/// Start job run with S3 monitoring configuration overrides — verify via get_job_run
#[tokio::test]
async fn test_start_job_run_with_s3_monitoring() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-s3-monitoring")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let job_resp = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .configuration_overrides(
            aws_sdk_emrserverless::types::ConfigurationOverrides::builder()
                .monitoring_configuration(
                    aws_sdk_emrserverless::types::MonitoringConfiguration::builder()
                        .s3_monitoring_configuration(
                            aws_sdk_emrserverless::types::S3MonitoringConfiguration::builder()
                                .log_uri("s3://my-bucket/logs/")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("start_job_run with S3 monitoring config should succeed");

    let job_run_id = job_resp.job_run_id().to_string();

    let get_resp = client
        .get_job_run()
        .application_id(&app_id)
        .job_run_id(&job_run_id)
        .send()
        .await
        .unwrap();

    let run = get_resp.job_run().unwrap();
    let overrides = run
        .configuration_overrides()
        .expect("configuration_overrides should be present");
    let monitoring = overrides
        .monitoring_configuration()
        .expect("monitoring_configuration should be present");
    let s3 = monitoring
        .s3_monitoring_configuration()
        .expect("s3_monitoring_configuration should be present");
    assert_eq!(s3.log_uri(), Some("s3://my-bucket/logs/"));
}

/// Update application maximum capacity on a STOPPED application — verify the new values
#[tokio::test]
async fn test_update_application_maximum_capacity() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-update-max-cap")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let update_resp = client
        .update_application()
        .application_id(&app_id)
        .maximum_capacity(
            aws_sdk_emrserverless::types::MaximumAllowedResources::builder()
                .cpu("200 vCPU")
                .memory("512 GB")
                .disk("500 GB")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update maximum capacity on STOPPED app should succeed");

    let app = update_resp.application().unwrap();
    let mc = app
        .maximum_capacity()
        .expect("maximum_capacity should be set");
    assert_eq!(mc.cpu(), "200 vCPU");
    assert_eq!(mc.memory(), "512 GB");
    assert_eq!(mc.disk(), Some("500 GB"));
}

/// After deleting an application, get_application should return an error
#[tokio::test]
async fn test_get_application_after_delete_fails() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-delete-then-get")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    client
        .stop_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    client
        .delete_application()
        .application_id(&app_id)
        .send()
        .await
        .expect("delete stopped application should succeed");

    let err = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await;

    assert!(
        err.is_err(),
        "get_application after deletion should fail with ResourceNotFoundException"
    );
}

/// List job runs filtered by CANCELLED state includes only cancelled runs
#[tokio::test]
async fn test_list_job_runs_cancelled_state_filter() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-cancel-filter")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    // Start two jobs
    let job1 = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    let job2 = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .send()
        .await
        .unwrap();

    // Cancel only job1
    client
        .cancel_job_run()
        .application_id(&app_id)
        .job_run_id(job1.job_run_id())
        .send()
        .await
        .unwrap();

    // Filter by CANCELLED
    let resp = client
        .list_job_runs()
        .application_id(&app_id)
        .states(aws_sdk_emrserverless::types::JobRunState::Cancelled)
        .send()
        .await
        .unwrap();

    let ids: Vec<&str> = resp.job_runs().iter().map(|r| r.id()).collect();
    assert!(
        ids.contains(&job1.job_run_id()),
        "CANCELLED filter should include job1"
    );
    assert!(
        !ids.contains(&job2.job_run_id()),
        "CANCELLED filter should exclude job2 (still RUNNING)"
    );

    // Filter by RUNNING should include job2 but not job1
    let resp_running = client
        .list_job_runs()
        .application_id(&app_id)
        .states(aws_sdk_emrserverless::types::JobRunState::Running)
        .send()
        .await
        .unwrap();

    let running_ids: Vec<&str> = resp_running.job_runs().iter().map(|r| r.id()).collect();
    assert!(
        running_ids.contains(&job2.job_run_id()),
        "RUNNING filter should include job2"
    );
    assert!(
        !running_ids.contains(&job1.job_run_id()),
        "RUNNING filter should exclude cancelled job1"
    );
}

/// Start job run with tags — verify tags are stored and retrievable via get_job_run
#[tokio::test]
async fn test_start_job_run_with_tags() {
    let client = make_emr_serverless_client().await;

    let create_resp = client
        .create_application()
        .name("test-app-job-tags")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application_id().to_string();

    let job_resp = client
        .start_job_run()
        .application_id(&app_id)
        .execution_role_arn(execution_role())
        .job_driver(simple_spark_job_driver())
        .tags("job-env", "staging")
        .tags("owner", "team-spark")
        .send()
        .await
        .expect("start_job_run with tags should succeed");

    let job_run_id = job_resp.job_run_id().to_string();

    let get_resp = client
        .get_job_run()
        .application_id(&app_id)
        .job_run_id(&job_run_id)
        .send()
        .await
        .unwrap();

    let run = get_resp.job_run().unwrap();
    let tags = run.tags();
    assert_eq!(
        tags.as_ref()
            .and_then(|t| t.get("job-env"))
            .map(|s| s.as_str()),
        Some("staging")
    );
    assert_eq!(
        tags.as_ref()
            .and_then(|t| t.get("owner"))
            .map(|s| s.as_str()),
        Some("team-spark")
    );
}

/// Deleting an application that does not exist returns an error
#[tokio::test]
async fn test_delete_application_nonexistent() {
    let client = make_emr_serverless_client().await;

    let err = client
        .delete_application()
        .application_id("completely_nonexistent_id")
        .send()
        .await;

    assert!(
        err.is_err(),
        "deleting a non-existent application should return ResourceNotFoundException"
    );
}

// ==================== Tag URL-encoding regression test ====================

/// Regression: the SDK URL-encodes colons in /tags/{resourceArn+} path
/// parameters, so the handler must decode the segments before using them
/// as a state-map key. Tags would otherwise round-trip to a different key.
#[tokio::test]
async fn test_tag_resource_url_encoded_arn_roundtrip() {
    let client = make_emr_serverless_client().await;

    let resp = client
        .create_application()
        .name("tag-roundtrip-app")
        .r#type("SPARK")
        .release_label("emr-6.6.0")
        .send()
        .await
        .expect("create_application should succeed");

    let arn = resp.arn().to_string();
    assert!(arn.contains(':'));

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag_resource should succeed despite URL-encoded path ARN");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tag_map = tags.tags().expect("tags must be present");
    assert_eq!(tag_map.get("env"), Some(&"prod".to_string()));

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed despite URL-encoded path ARN");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed after untag");
    assert!(tags.tags().map(|t| t.is_empty()).unwrap_or(true));
}
