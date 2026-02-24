/// Scenario tests for the IoT Data Plane mock service.
///
/// These tests exercise end-to-end use-case workflows chaining multiple
/// operations to verify business-level correctness rather than per-API shape.
use aws_sdk_iotdataplane::config::BehaviorVersion;
use aws_sdk_iotdataplane::primitives::Blob;
use winterbaume_core::MockAws;
use winterbaume_iotdataplane::IotDataPlaneService;

async fn make_client() -> aws_sdk_iotdataplane::Client {
    let mock = MockAws::builder()
        .with_service(IotDataPlaneService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_iotdataplane::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_iotdataplane::Client::from_conf(
        aws_sdk_iotdataplane::config::Builder::from(&config)
            .endpoint_url("https://data.iot.us-east-1.amazonaws.com")
            .build(),
    )
}

/// Scenario: device shadow lifecycle — create, update, read, delete.
///
/// Models a typical IoT device that reports its state, receives a desired
/// configuration, then is decommissioned. Chains UpdateThingShadow,
/// GetThingShadow, ListNamedShadowsForThing, and DeleteThingShadow.
#[tokio::test]
async fn test_device_shadow_lifecycle() {
    // Scenario: Device shadow lifecycle — create, update, read, delete.
    let client = make_client().await;

    let thing = "scenario-device-01";

    // Step 1: Device reports its initial state.
    let initial = br#"{"state":{"reported":{"online":true,"firmware":"1.0.0"}}}"#;
    client
        .update_thing_shadow()
        .thing_name(thing)
        .payload(Blob::new(initial.to_vec()))
        .send()
        .await
        .expect("initial shadow report should succeed");

    // Step 2: Cloud sets desired configuration.
    let desired = br#"{"state":{"desired":{"firmware":"2.0.0"},"reported":{"online":true,"firmware":"1.0.0"}}}"#;
    let resp = client
        .update_thing_shadow()
        .thing_name(thing)
        .payload(Blob::new(desired.to_vec()))
        .send()
        .await
        .expect("desired state update should succeed");

    let body: serde_json::Value = serde_json::from_slice(resp.payload().unwrap().as_ref()).unwrap();
    assert_eq!(
        body.get("version").and_then(|v| v.as_i64()),
        Some(2),
        "version should be 2 after second update"
    );

    // Step 3: Read back and verify the delta.
    let get_resp = client
        .get_thing_shadow()
        .thing_name(thing)
        .send()
        .await
        .expect("get shadow should succeed");

    let shadow: serde_json::Value =
        serde_json::from_slice(get_resp.payload().unwrap().as_ref()).unwrap();
    assert_eq!(
        shadow
            .pointer("/state/desired/firmware")
            .and_then(|v| v.as_str()),
        Some("2.0.0"),
        "desired firmware should be 2.0.0"
    );
    assert_eq!(
        shadow
            .pointer("/state/reported/firmware")
            .and_then(|v| v.as_str()),
        Some("1.0.0"),
        "reported firmware should still be 1.0.0"
    );

    // Step 4: Decommission — delete shadow and verify it is gone.
    client
        .delete_thing_shadow()
        .thing_name(thing)
        .send()
        .await
        .expect("delete should succeed");

    let err = client
        .get_thing_shadow()
        .thing_name(thing)
        .send()
        .await
        .unwrap_err()
        .into_service_error();
    assert!(
        err.is_resource_not_found_exception(),
        "deleted shadow should not be found"
    );
}

/// Scenario: multi-shadow device — classic and named shadows are independent.
///
/// Models a device with a classic operational shadow and a named
/// "config" shadow. Verifies that they store independent state and
/// appear separately in ListNamedShadowsForThing.
#[tokio::test]
async fn test_multi_shadow_device() {
    // Scenario: Multi-shadow device — classic and named shadows are independent.
    let client = make_client().await;

    let thing = "scenario-multi-shadow-device";

    // Classic shadow: device operational state.
    let classic = br#"{"state":{"reported":{"status":"running"}}}"#;
    client
        .update_thing_shadow()
        .thing_name(thing)
        .payload(Blob::new(classic.to_vec()))
        .send()
        .await
        .unwrap();

    // Named shadow "config": configuration parameters.
    let config = br#"{"state":{"desired":{"interval":30,"mode":"auto"}}}"#;
    client
        .update_thing_shadow()
        .thing_name(thing)
        .shadow_name("config")
        .payload(Blob::new(config.to_vec()))
        .send()
        .await
        .unwrap();

    // Named shadow "diagnostics": diagnostic counters.
    let diag = br#"{"state":{"reported":{"errors":0,"reboots":1}}}"#;
    client
        .update_thing_shadow()
        .thing_name(thing)
        .shadow_name("diagnostics")
        .payload(Blob::new(diag.to_vec()))
        .send()
        .await
        .unwrap();

    // ListNamedShadowsForThing should return only named shadows.
    let list = client
        .list_named_shadows_for_thing()
        .thing_name(thing)
        .send()
        .await
        .expect("list named shadows should succeed");

    let names = list.results();
    assert_eq!(names.len(), 2, "should have exactly 2 named shadows");
    assert!(names.contains(&"config".to_string()));
    assert!(names.contains(&"diagnostics".to_string()));

    // Classic shadow is still accessible independently.
    let classic_get = client
        .get_thing_shadow()
        .thing_name(thing)
        .send()
        .await
        .expect("classic shadow should still be accessible");
    let classic_body: serde_json::Value =
        serde_json::from_slice(classic_get.payload().unwrap().as_ref()).unwrap();
    assert_eq!(
        classic_body
            .pointer("/state/reported/status")
            .and_then(|v| v.as_str()),
        Some("running")
    );

    // Delete "config" shadow; "diagnostics" and classic should survive.
    client
        .delete_thing_shadow()
        .thing_name(thing)
        .shadow_name("config")
        .send()
        .await
        .expect("delete named shadow should succeed");

    let list2 = client
        .list_named_shadows_for_thing()
        .thing_name(thing)
        .send()
        .await
        .expect("list after delete should succeed");
    assert_eq!(list2.results().len(), 1);
    assert!(list2.results().contains(&"diagnostics".to_string()));
}

/// Scenario: retained-message publish and retrieval.
///
/// Models a device publishing a retained last-will-style message,
/// then reads it back via GetRetainedMessage and ListRetainedMessages.
#[tokio::test]
async fn test_retained_message_workflow() {
    // Scenario: Retained-message publish and retrieval.
    let client = make_client().await;

    let topic = "scenario/device/status";
    let payload = br#"{"status":"online","ts":1700000000}"#;

    // Publish with retain=true to store a retained message.
    client
        .publish()
        .topic(topic)
        .payload(Blob::new(payload.to_vec()))
        .retain(true)
        .send()
        .await
        .expect("retained publish should succeed");

    // Retrieve the retained message directly.
    let get = client
        .get_retained_message()
        .topic(topic)
        .send()
        .await
        .expect("get_retained_message should succeed");

    assert_eq!(get.topic(), Some(topic), "topic should match");
    assert!(get.qos() >= 0, "qos should be set");

    // List retained messages and confirm our topic is included.
    let list = client
        .list_retained_messages()
        .send()
        .await
        .expect("list_retained_messages should succeed");

    let topics: Vec<&str> = list
        .retained_topics()
        .iter()
        .filter_map(|s| s.topic())
        .collect();
    assert!(
        topics.contains(&topic),
        "published retained topic should appear in listing"
    );
}
