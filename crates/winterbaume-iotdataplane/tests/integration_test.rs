use aws_sdk_iotdataplane::config::BehaviorVersion;
use aws_sdk_iotdataplane::primitives::Blob;
use winterbaume_core::MockAws;
use winterbaume_iotdataplane::IotDataPlaneService;

async fn make_iot_data_client() -> aws_sdk_iotdataplane::Client {
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

#[tokio::test]
async fn test_update_and_get_thing_shadow() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"temperature":25}}}"#;

    client
        .update_thing_shadow()
        .thing_name("my-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .expect("update_thing_shadow should succeed");

    let get_resp = client
        .get_thing_shadow()
        .thing_name("my-thing")
        .send()
        .await
        .expect("get_thing_shadow should succeed");

    let body = get_resp.payload().unwrap().as_ref();
    let parsed: serde_json::Value = serde_json::from_slice(body).unwrap();
    let state_obj = parsed.get("state").and_then(|s| s.get("desired"));
    assert!(state_obj.is_some());
}

#[tokio::test]
async fn test_get_nonexistent_shadow_fails() {
    let client = make_iot_data_client().await;

    let result = client
        .get_thing_shadow()
        .thing_name("nonexistent-thing")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_thing_shadow() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"led":"on"}}}"#;

    client
        .update_thing_shadow()
        .thing_name("del-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .unwrap();

    client
        .delete_thing_shadow()
        .thing_name("del-thing")
        .send()
        .await
        .expect("delete_thing_shadow should succeed");

    let result = client
        .get_thing_shadow()
        .thing_name("del-thing")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_shadow_fails() {
    let client = make_iot_data_client().await;

    let result = client
        .delete_thing_shadow()
        .thing_name("nonexistent-thing")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_named_shadows() {
    let client = make_iot_data_client().await;

    let payload1 = br#"{"state":{"desired":{"color":"red"}}}"#;
    let payload2 = br#"{"state":{"desired":{"color":"blue"}}}"#;

    client
        .update_thing_shadow()
        .thing_name("multi-thing")
        .shadow_name("shadow-a")
        .payload(Blob::new(payload1.to_vec()))
        .send()
        .await
        .unwrap();

    client
        .update_thing_shadow()
        .thing_name("multi-thing")
        .shadow_name("shadow-b")
        .payload(Blob::new(payload2.to_vec()))
        .send()
        .await
        .unwrap();

    let get_a = client
        .get_thing_shadow()
        .thing_name("multi-thing")
        .shadow_name("shadow-a")
        .send()
        .await
        .expect("get named shadow-a should succeed");

    let body_a = get_a.payload().unwrap().as_ref();
    let parsed_a: serde_json::Value = serde_json::from_slice(body_a).unwrap();
    assert!(
        parsed_a
            .get("state")
            .and_then(|s| s.get("desired"))
            .and_then(|d| d.get("color"))
            .and_then(|c| c.as_str())
            .map(|c| c == "red")
            .unwrap_or(false)
    );
}

#[tokio::test]
async fn test_list_named_shadows_for_thing() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"x":1}}}"#;

    // Create two named shadows
    for name in ["shadow-x", "shadow-y"] {
        client
            .update_thing_shadow()
            .thing_name("list-thing")
            .shadow_name(name)
            .payload(Blob::new(payload.to_vec()))
            .send()
            .await
            .unwrap();
    }

    // Also create a classic shadow (should not appear in named list)
    client
        .update_thing_shadow()
        .thing_name("list-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_named_shadows_for_thing()
        .thing_name("list-thing")
        .send()
        .await
        .expect("list_named_shadows should succeed");

    let results = resp.results();
    assert_eq!(results.len(), 2);
    assert!(results.contains(&"shadow-x".to_string()));
    assert!(results.contains(&"shadow-y".to_string()));
}

#[tokio::test]
async fn test_publish() {
    let client = make_iot_data_client().await;

    let payload = br#"{"msg":"hello"}"#;

    client
        .publish()
        .topic("test/topic")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .expect("publish should succeed");
}

#[tokio::test]
async fn test_update_shadow_version_increments() {
    let client = make_iot_data_client().await;

    let payload1 = br#"{"state":{"desired":{"v":1}}}"#;
    let payload2 = br#"{"state":{"desired":{"v":2}}}"#;

    let resp1 = client
        .update_thing_shadow()
        .thing_name("ver-thing")
        .payload(Blob::new(payload1.to_vec()))
        .send()
        .await
        .expect("first update should succeed");

    let body1: serde_json::Value =
        serde_json::from_slice(resp1.payload().unwrap().as_ref()).unwrap();
    let version1 = body1.get("version").and_then(|v| v.as_i64()).unwrap();

    let resp2 = client
        .update_thing_shadow()
        .thing_name("ver-thing")
        .payload(Blob::new(payload2.to_vec()))
        .send()
        .await
        .expect("second update should succeed");

    let body2: serde_json::Value =
        serde_json::from_slice(resp2.payload().unwrap().as_ref()).unwrap();
    let version2 = body2.get("version").and_then(|v| v.as_i64()).unwrap();

    assert_eq!(version1, 1);
    assert_eq!(version2, 2);
}

// ============================================================================
// Additional tests derived from AWS documentation
// ============================================================================

#[tokio::test]
async fn test_update_shadow_reported_state() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"reported":{"temperature":22,"humidity":60}}}"#;

    let resp = client
        .update_thing_shadow()
        .thing_name("reported-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .expect("update with reported state should succeed");

    let body: serde_json::Value = serde_json::from_slice(resp.payload().unwrap().as_ref()).unwrap();
    assert!(body.get("state").is_some());
    assert!(body.get("version").is_some());
}

#[tokio::test]
async fn test_named_shadow_independent_from_classic() {
    let client = make_iot_data_client().await;

    let classic_payload = br#"{"state":{"desired":{"mode":"classic"}}}"#;
    let named_payload = br#"{"state":{"desired":{"mode":"named"}}}"#;

    client
        .update_thing_shadow()
        .thing_name("independent-thing")
        .payload(Blob::new(classic_payload.to_vec()))
        .send()
        .await
        .unwrap();

    client
        .update_thing_shadow()
        .thing_name("independent-thing")
        .shadow_name("my-shadow")
        .payload(Blob::new(named_payload.to_vec()))
        .send()
        .await
        .unwrap();

    let classic_resp = client
        .get_thing_shadow()
        .thing_name("independent-thing")
        .send()
        .await
        .expect("classic shadow should be retrievable");

    let named_resp = client
        .get_thing_shadow()
        .thing_name("independent-thing")
        .shadow_name("my-shadow")
        .send()
        .await
        .expect("named shadow should be retrievable");

    let classic_body: serde_json::Value =
        serde_json::from_slice(classic_resp.payload().unwrap().as_ref()).unwrap();
    let named_body: serde_json::Value =
        serde_json::from_slice(named_resp.payload().unwrap().as_ref()).unwrap();

    let classic_mode = classic_body
        .pointer("/state/desired/mode")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let named_mode = named_body
        .pointer("/state/desired/mode")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    assert_eq!(classic_mode, "classic");
    assert_eq!(named_mode, "named");
}

#[tokio::test]
async fn test_delete_named_shadow_only_removes_named() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"on":true}}}"#;

    client
        .update_thing_shadow()
        .thing_name("selective-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .unwrap();

    client
        .update_thing_shadow()
        .thing_name("selective-thing")
        .shadow_name("extra")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .unwrap();

    client
        .delete_thing_shadow()
        .thing_name("selective-thing")
        .shadow_name("extra")
        .send()
        .await
        .expect("delete named shadow should succeed");

    // Classic shadow should still be accessible
    client
        .get_thing_shadow()
        .thing_name("selective-thing")
        .send()
        .await
        .expect("classic shadow should still exist after named shadow deletion");

    // Named shadow should be gone
    let result = client
        .get_thing_shadow()
        .thing_name("selective-thing")
        .shadow_name("extra")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_named_shadows_empty_for_thing_with_no_named_shadows() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"x":1}}}"#;

    // Create only a classic shadow
    client
        .update_thing_shadow()
        .thing_name("classic-only-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_named_shadows_for_thing()
        .thing_name("classic-only-thing")
        .send()
        .await
        .expect("list_named_shadows should succeed even with no named shadows");

    let results = resp.results();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_list_named_shadows_empty_for_unknown_thing() {
    let client = make_iot_data_client().await;

    let resp = client
        .list_named_shadows_for_thing()
        .thing_name("completely-unknown-thing")
        .send()
        .await
        .expect("list_named_shadows should succeed for unknown thing");

    let results = resp.results();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_publish_to_hierarchical_topic() {
    let client = make_iot_data_client().await;

    let payload = br#"{"sensor":"temp","value":23.5}"#;

    client
        .publish()
        .topic("home/living-room/temperature")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .expect("publish to hierarchical topic should succeed");
}

#[tokio::test]
async fn test_publish_without_payload() {
    let client = make_iot_data_client().await;

    client
        .publish()
        .topic("alerts/ping")
        .send()
        .await
        .expect("publish without payload should succeed");
}

#[tokio::test]
async fn test_update_named_shadow_version_increments() {
    let client = make_iot_data_client().await;

    let payload1 = br#"{"state":{"desired":{"step":1}}}"#;
    let payload2 = br#"{"state":{"desired":{"step":2}}}"#;
    let payload3 = br#"{"state":{"desired":{"step":3}}}"#;

    let resp1 = client
        .update_thing_shadow()
        .thing_name("named-ver-thing")
        .shadow_name("versioned")
        .payload(Blob::new(payload1.to_vec()))
        .send()
        .await
        .expect("first update should succeed");

    let resp2 = client
        .update_thing_shadow()
        .thing_name("named-ver-thing")
        .shadow_name("versioned")
        .payload(Blob::new(payload2.to_vec()))
        .send()
        .await
        .expect("second update should succeed");

    let resp3 = client
        .update_thing_shadow()
        .thing_name("named-ver-thing")
        .shadow_name("versioned")
        .payload(Blob::new(payload3.to_vec()))
        .send()
        .await
        .expect("third update should succeed");

    let body1: serde_json::Value =
        serde_json::from_slice(resp1.payload().unwrap().as_ref()).unwrap();
    let body2: serde_json::Value =
        serde_json::from_slice(resp2.payload().unwrap().as_ref()).unwrap();
    let body3: serde_json::Value =
        serde_json::from_slice(resp3.payload().unwrap().as_ref()).unwrap();

    assert_eq!(body1.get("version").and_then(|v| v.as_i64()), Some(1));
    assert_eq!(body2.get("version").and_then(|v| v.as_i64()), Some(2));
    assert_eq!(body3.get("version").and_then(|v| v.as_i64()), Some(3));
}

#[tokio::test]
async fn test_named_shadow_version_independent_from_classic() {
    let client = make_iot_data_client().await;

    let classic_payload = br#"{"state":{"desired":{"c":1}}}"#;
    let named_payload = br#"{"state":{"desired":{"n":1}}}"#;

    // Update classic shadow twice
    for _ in 0..2 {
        client
            .update_thing_shadow()
            .thing_name("ver-isolation-thing")
            .payload(Blob::new(classic_payload.to_vec()))
            .send()
            .await
            .unwrap();
    }

    // Update named shadow once
    let named_resp = client
        .update_thing_shadow()
        .thing_name("ver-isolation-thing")
        .shadow_name("isolated")
        .payload(Blob::new(named_payload.to_vec()))
        .send()
        .await
        .expect("named shadow update should succeed");

    let named_body: serde_json::Value =
        serde_json::from_slice(named_resp.payload().unwrap().as_ref()).unwrap();

    // Named shadow version should start at 1, unaffected by classic shadow updates
    assert_eq!(named_body.get("version").and_then(|v| v.as_i64()), Some(1));
}

#[tokio::test]
async fn test_get_shadow_after_multiple_updates_returns_latest() {
    let client = make_iot_data_client().await;

    for i in 1..=5 {
        let payload = format!(r#"{{"state":{{"desired":{{"counter":{}}}}}}}"#, i);
        client
            .update_thing_shadow()
            .thing_name("latest-thing")
            .payload(Blob::new(payload.into_bytes()))
            .send()
            .await
            .unwrap();
    }

    let get_resp = client
        .get_thing_shadow()
        .thing_name("latest-thing")
        .send()
        .await
        .expect("get_thing_shadow should succeed");

    let body: serde_json::Value =
        serde_json::from_slice(get_resp.payload().unwrap().as_ref()).unwrap();

    let counter = body
        .pointer("/state/desired/counter")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    assert_eq!(counter, 5);
}

// ============================================================================
// Tests derived from AWS documentation: IoT Data Plane
// ============================================================================

#[tokio::test]
async fn test_update_shadow_invalid_json() {
    let client = make_iot_data_client().await;

    let err = client
        .update_thing_shadow()
        .thing_name("invalid-json-thing")
        .payload(Blob::new(b"not-valid-json".to_vec()))
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidRequestException"),
        "Expected InvalidRequestException for malformed JSON, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_shadow_response_has_version_and_timestamp() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"key":"value"}}}"#;

    let resp = client
        .update_thing_shadow()
        .thing_name("resp-fields-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .expect("update_thing_shadow should succeed");

    let body: serde_json::Value = serde_json::from_slice(resp.payload().unwrap().as_ref()).unwrap();

    assert_eq!(
        body.get("version").and_then(|v| v.as_i64()),
        Some(1),
        "version should be 1 for first update"
    );
    assert!(
        body.get("timestamp").and_then(|v| v.as_i64()).is_some(),
        "timestamp field should be present"
    );
    assert!(
        body.get("state").is_some(),
        "state field should be present in update response"
    );
}

#[tokio::test]
async fn test_delete_shadow_response_has_version_and_timestamp() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"k":1}}}"#;

    // Update twice so version becomes 2
    for _ in 0..2 {
        client
            .update_thing_shadow()
            .thing_name("del-resp-thing")
            .payload(Blob::new(payload.to_vec()))
            .send()
            .await
            .unwrap();
    }

    let del_resp = client
        .delete_thing_shadow()
        .thing_name("del-resp-thing")
        .send()
        .await
        .expect("delete_thing_shadow should succeed");

    let body: serde_json::Value = serde_json::from_slice(del_resp.payload().as_ref()).unwrap();

    assert!(
        body.get("version").and_then(|v| v.as_i64()).is_some(),
        "delete response should include version"
    );
    assert!(
        body.get("timestamp").and_then(|v| v.as_i64()).is_some(),
        "delete response should include timestamp"
    );
    assert_eq!(
        body.get("version").and_then(|v| v.as_i64()),
        Some(2),
        "delete response version should match last update version"
    );
}

#[tokio::test]
async fn test_list_named_shadows_response_has_timestamp() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"x":1}}}"#;

    client
        .update_thing_shadow()
        .thing_name("ts-thing")
        .shadow_name("ts-shadow")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_named_shadows_for_thing()
        .thing_name("ts-thing")
        .send()
        .await
        .expect("list_named_shadows_for_thing should succeed");

    assert!(
        resp.timestamp() > 0,
        "list response timestamp should be a positive epoch value"
    );
}

#[tokio::test]
async fn test_publish_with_qos1() {
    let client = make_iot_data_client().await;

    let payload = br#"{"event":"motion_detected"}"#;

    client
        .publish()
        .topic("sensors/motion")
        .payload(Blob::new(payload.to_vec()))
        .qos(1)
        .send()
        .await
        .expect("publish with QoS=1 should succeed");
}

#[tokio::test]
async fn test_publish_with_retain() {
    let client = make_iot_data_client().await;

    let payload = br#"{"status":"online"}"#;

    client
        .publish()
        .topic("devices/status")
        .payload(Blob::new(payload.to_vec()))
        .retain(true)
        .send()
        .await
        .expect("publish with retain=true should succeed");
}

#[tokio::test]
async fn test_update_shadow_both_desired_and_reported() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"target":22},"reported":{"current":20}}}"#;

    client
        .update_thing_shadow()
        .thing_name("both-states-thing")
        .payload(Blob::new(payload.to_vec()))
        .send()
        .await
        .expect("update with both desired and reported should succeed");

    let get_resp = client
        .get_thing_shadow()
        .thing_name("both-states-thing")
        .send()
        .await
        .expect("get_thing_shadow should succeed");

    let body: serde_json::Value =
        serde_json::from_slice(get_resp.payload().unwrap().as_ref()).unwrap();

    let desired = body
        .pointer("/state/desired/target")
        .and_then(|v| v.as_i64());
    let reported = body
        .pointer("/state/reported/current")
        .and_then(|v| v.as_i64());

    assert_eq!(desired, Some(22), "desired state should be stored");
    assert_eq!(reported, Some(20), "reported state should be stored");
}

#[tokio::test]
async fn test_get_shadow_not_found_error_code() {
    let client = make_iot_data_client().await;

    let err = client
        .get_thing_shadow()
        .thing_name("does-not-exist-thing")
        .send()
        .await
        .unwrap_err();

    let service_err = err.into_service_error();
    assert!(
        service_err.is_resource_not_found_exception(),
        "Expected ResourceNotFoundException for nonexistent shadow"
    );
}

#[tokio::test]
async fn test_delete_shadow_not_found_error_code() {
    let client = make_iot_data_client().await;

    let err = client
        .delete_thing_shadow()
        .thing_name("does-not-exist-del-thing")
        .send()
        .await
        .unwrap_err();

    let service_err = err.into_service_error();
    assert!(
        service_err.is_resource_not_found_exception(),
        "Expected ResourceNotFoundException for nonexistent shadow delete"
    );
}

#[tokio::test]
async fn test_full_shadow_lifecycle() {
    let client = make_iot_data_client().await;

    let payload_v1 = br#"{"state":{"desired":{"mode":"off"}}}"#;
    let payload_v2 = br#"{"state":{"desired":{"mode":"on"},"reported":{"mode":"off"}}}"#;

    // Create (v1)
    let update_resp = client
        .update_thing_shadow()
        .thing_name("lifecycle-thing")
        .shadow_name("lifecycle-shadow")
        .payload(Blob::new(payload_v1.to_vec()))
        .send()
        .await
        .expect("first update should succeed");

    let body1: serde_json::Value =
        serde_json::from_slice(update_resp.payload().unwrap().as_ref()).unwrap();
    assert_eq!(body1.get("version").and_then(|v| v.as_i64()), Some(1));

    // Get (v1)
    let get_resp = client
        .get_thing_shadow()
        .thing_name("lifecycle-thing")
        .shadow_name("lifecycle-shadow")
        .send()
        .await
        .expect("get after first update should succeed");

    let stored: serde_json::Value =
        serde_json::from_slice(get_resp.payload().unwrap().as_ref()).unwrap();
    assert_eq!(
        stored
            .pointer("/state/desired/mode")
            .and_then(|v| v.as_str()),
        Some("off")
    );

    // Update (v2)
    let update_resp2 = client
        .update_thing_shadow()
        .thing_name("lifecycle-thing")
        .shadow_name("lifecycle-shadow")
        .payload(Blob::new(payload_v2.to_vec()))
        .send()
        .await
        .expect("second update should succeed");

    let body2: serde_json::Value =
        serde_json::from_slice(update_resp2.payload().unwrap().as_ref()).unwrap();
    assert_eq!(body2.get("version").and_then(|v| v.as_i64()), Some(2));

    // Verify list includes this shadow
    let list_resp = client
        .list_named_shadows_for_thing()
        .thing_name("lifecycle-thing")
        .send()
        .await
        .expect("list should succeed");
    assert!(
        list_resp
            .results()
            .contains(&"lifecycle-shadow".to_string())
    );

    // Delete
    let del_resp = client
        .delete_thing_shadow()
        .thing_name("lifecycle-thing")
        .shadow_name("lifecycle-shadow")
        .send()
        .await
        .expect("delete should succeed");

    let del_body: serde_json::Value = serde_json::from_slice(del_resp.payload().as_ref()).unwrap();
    assert_eq!(del_body.get("version").and_then(|v| v.as_i64()), Some(2));

    // Verify gone
    let get_err = client
        .get_thing_shadow()
        .thing_name("lifecycle-thing")
        .shadow_name("lifecycle-shadow")
        .send()
        .await
        .unwrap_err()
        .into_service_error();
    assert!(get_err.is_resource_not_found_exception());

    // Verify removed from list
    let list_resp2 = client
        .list_named_shadows_for_thing()
        .thing_name("lifecycle-thing")
        .send()
        .await
        .expect("list after delete should succeed");
    assert!(
        !list_resp2
            .results()
            .contains(&"lifecycle-shadow".to_string())
    );
}

#[tokio::test]
async fn test_list_named_shadows_after_delete_removes_shadow_name() {
    let client = make_iot_data_client().await;

    let payload = br#"{"state":{"desired":{"k":1}}}"#;

    for name in ["keep-shadow", "delete-shadow"] {
        client
            .update_thing_shadow()
            .thing_name("del-list-thing")
            .shadow_name(name)
            .payload(Blob::new(payload.to_vec()))
            .send()
            .await
            .unwrap();
    }

    client
        .delete_thing_shadow()
        .thing_name("del-list-thing")
        .shadow_name("delete-shadow")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_named_shadows_for_thing()
        .thing_name("del-list-thing")
        .send()
        .await
        .expect("list should succeed after deletion");

    let results = resp.results();
    assert_eq!(results.len(), 1);
    assert!(results.contains(&"keep-shadow".to_string()));
    assert!(!results.contains(&"delete-shadow".to_string()));
}
