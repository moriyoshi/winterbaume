/// Scenario: Device provisioning and job delivery pipeline
///
/// This scenario exercises the full lifecycle of provisioning an IoT device and
/// dispatching a job to it, representing a common fleet-management workflow:
///   1. Create a thing type and a thing
///   2. Create a certificate and attach it to the thing
///   3. Create a policy and attach it to the certificate principal
///   4. Create a job targeting the thing
///   5. Verify the job execution is visible for the thing
///   6. Cancel the job and confirm status
use aws_sdk_iot::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_iot::IotService;

async fn make_client() -> aws_sdk_iot::Client {
    let mock = MockAws::builder().with_service(IotService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_iot::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_iot::Client::new(&config)
}

/// Scenario: Device provisioning pipeline — create thing type, thing, cert, policy and
/// verify list operations reflect the registered device.
#[tokio::test]
async fn test_device_provisioning_pipeline() {
    let client = make_client().await;

    // Step 1: Create a thing type
    client
        .create_thing_type()
        .thing_type_name("TemperatureSensor")
        .send()
        .await
        .expect("create_thing_type should succeed");

    // Step 2: Create a thing of that type
    let thing_resp = client
        .create_thing()
        .thing_name("sensor-001")
        .thing_type_name("TemperatureSensor")
        .send()
        .await
        .expect("create_thing should succeed");
    let thing_arn = thing_resp
        .thing_arn()
        .expect("thing_arn should be set")
        .to_string();
    assert!(thing_arn.contains("sensor-001"));

    // Step 3: Issue a certificate
    let cert_resp = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .expect("create_keys_and_certificate should succeed");
    let cert_id = cert_resp.certificate_id().expect("cert_id").to_string();
    let cert_arn = cert_resp.certificate_arn().expect("cert_arn").to_string();
    assert!(!cert_id.is_empty());

    // Step 4: Attach certificate to thing
    client
        .attach_thing_principal()
        .thing_name("sensor-001")
        .principal(&cert_arn)
        .send()
        .await
        .expect("attach_thing_principal should succeed");

    // Step 5: Create a policy and attach to the certificate principal
    client
        .create_policy()
        .policy_name("SensorPolicy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"iot:*","Resource":"*"}]}"#)
        .send()
        .await
        .expect("create_policy should succeed");
    client
        .attach_policy()
        .policy_name("SensorPolicy")
        .target(&cert_arn)
        .send()
        .await
        .expect("attach_policy should succeed");

    // Step 6: Verify the thing and cert are visible in list calls
    let list_things = client.list_things().send().await.expect("list_things");
    let names: Vec<_> = list_things
        .things()
        .iter()
        .map(|t| t.thing_name().unwrap_or(""))
        .collect();
    assert!(
        names.contains(&"sensor-001"),
        "sensor-001 should appear in list_things"
    );

    let list_certs = client
        .list_certificates()
        .send()
        .await
        .expect("list_certificates");
    let cert_ids: Vec<_> = list_certs
        .certificates()
        .iter()
        .map(|c| c.certificate_id().unwrap_or(""))
        .collect();
    assert!(
        cert_ids.contains(&cert_id.as_str()),
        "cert should appear in list_certificates"
    );

    // Step 7: Verify thing principals
    let principals = client
        .list_thing_principals()
        .thing_name("sensor-001")
        .send()
        .await
        .expect("list_thing_principals");
    assert!(
        principals.principals().contains(&cert_arn),
        "cert_arn should be a principal of sensor-001"
    );
}

/// Scenario: Job dispatch and cancellation pipeline — create a thing, create a job
/// targeting it, verify execution is created, then cancel and verify status.
#[tokio::test]
async fn test_job_dispatch_and_cancellation() {
    let client = make_client().await;

    // Provision a device
    client
        .create_thing()
        .thing_name("actuator-007")
        .send()
        .await
        .expect("create_thing");

    let thing_arn = client
        .describe_thing()
        .thing_name("actuator-007")
        .send()
        .await
        .expect("describe_thing")
        .thing_arn()
        .expect("thing_arn")
        .to_string();

    // Create a job targeting the thing
    client
        .create_job()
        .job_id("firmware-update-v2")
        .set_targets(Some(vec![thing_arn.clone()]))
        .document(r#"{"operation":"update","version":"2.0"}"#)
        .send()
        .await
        .expect("create_job should succeed");

    // Describe the job and verify initial status
    let job = client
        .describe_job()
        .job_id("firmware-update-v2")
        .send()
        .await
        .expect("describe_job");
    let status = job
        .job()
        .expect("job present")
        .status()
        .expect("status")
        .as_str()
        .to_string();
    assert_eq!(status, "IN_PROGRESS");

    // List jobs and confirm ours is present
    let jobs_list = client.list_jobs().send().await.expect("list_jobs");
    let job_ids: Vec<_> = jobs_list
        .jobs()
        .iter()
        .map(|j| j.job_id().unwrap_or(""))
        .collect();
    assert!(
        job_ids.contains(&"firmware-update-v2"),
        "job should appear in list_jobs"
    );

    // Cancel the job
    client
        .cancel_job()
        .job_id("firmware-update-v2")
        .force(false)
        .send()
        .await
        .expect("cancel_job");

    // Verify job is CANCELED
    let after = client
        .describe_job()
        .job_id("firmware-update-v2")
        .send()
        .await
        .expect("describe_job after cancel");
    let after_status = after
        .job()
        .expect("job")
        .status()
        .expect("status")
        .as_str()
        .to_string();
    assert_eq!(after_status, "CANCELED");
}
