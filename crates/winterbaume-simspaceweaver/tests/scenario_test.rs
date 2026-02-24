/// Scenario: Full simulation pipeline
///
/// Demonstrates a complete SimSpace Weaver workflow:
/// start a simulation → start the clock → deploy an app → stop the app →
/// stop the clock → stop the simulation → delete the simulation.
/// Asserts business outcomes (statuses, list membership) rather than
/// per-API return shapes.
use aws_sdk_simspaceweaver::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_simspaceweaver::SimSpaceWeaverService;

async fn make_client() -> aws_sdk_simspaceweaver::Client {
    let mock = MockAws::builder()
        .with_service(SimSpaceWeaverService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simspaceweaver::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_simspaceweaver::Client::new(&config)
}

#[tokio::test]
async fn test_full_simulation_pipeline() {
    let client = make_client().await;

    // 1. Start the simulation.
    let started = client
        .start_simulation()
        .name("pipeline-sim")
        .role_arn("arn:aws:iam::123456789012:role/SimRole")
        .description("Integration scenario")
        .send()
        .await
        .expect("start_simulation");
    assert!(started.arn().is_some(), "ARN should be present");

    // 2. Confirm it appears in the list with STARTING status.
    let listed = client
        .list_simulations()
        .send()
        .await
        .expect("list_simulations");
    let sim_meta = listed
        .simulations()
        .iter()
        .find(|s| s.name() == Some("pipeline-sim"))
        .expect("simulation should appear in list");
    assert_eq!(
        sim_meta.status().map(|s| s.as_str()),
        Some("STARTING"),
        "simulation should be STARTING after creation"
    );

    // 3. Start the simulation clock.
    client
        .start_clock()
        .simulation("pipeline-sim")
        .send()
        .await
        .expect("start_clock");

    // 4. Verify clock status via describe.
    let after_clock = client
        .describe_simulation()
        .simulation("pipeline-sim")
        .send()
        .await
        .expect("describe after start_clock");
    let clock_status = after_clock
        .live_simulation_state()
        .and_then(|ls| ls.clocks().first())
        .and_then(|c| c.status())
        .map(|s| s.as_str());
    assert_eq!(clock_status, Some("STARTED"), "clock should be STARTED");

    // 5. Deploy an app.
    client
        .start_app()
        .simulation("pipeline-sim")
        .domain("Workers")
        .name("cruncher")
        .send()
        .await
        .expect("start_app");

    // 6. Verify app appears in list.
    let apps = client
        .list_apps()
        .simulation("pipeline-sim")
        .send()
        .await
        .expect("list_apps");
    assert_eq!(apps.apps().len(), 1, "one app should be running");
    assert_eq!(
        apps.apps()[0].status().map(|s| s.as_str()),
        Some("STARTING"),
        "app should be STARTING"
    );

    // 7. Stop the app.
    client
        .stop_app()
        .simulation("pipeline-sim")
        .domain("Workers")
        .app("cruncher")
        .send()
        .await
        .expect("stop_app");

    let described_app = client
        .describe_app()
        .simulation("pipeline-sim")
        .domain("Workers")
        .app("cruncher")
        .send()
        .await
        .expect("describe_app after stop");
    assert_eq!(
        described_app.status().map(|s| s.as_str()),
        Some("STOPPING"),
        "app should be STOPPING after stop"
    );

    // 8. Stop the clock.
    client
        .stop_clock()
        .simulation("pipeline-sim")
        .send()
        .await
        .expect("stop_clock");

    // 9. Stop and then delete the simulation.
    client
        .stop_simulation()
        .simulation("pipeline-sim")
        .send()
        .await
        .expect("stop_simulation");

    client
        .delete_simulation()
        .simulation("pipeline-sim")
        .send()
        .await
        .expect("delete_simulation");

    // 10. Confirm the simulation no longer appears in the list.
    let final_list = client
        .list_simulations()
        .send()
        .await
        .expect("final list_simulations");
    assert!(
        !final_list
            .simulations()
            .iter()
            .any(|s| s.name() == Some("pipeline-sim")),
        "deleted simulation must not appear in list"
    );
}

#[tokio::test]
async fn test_tagging_workflow() {
    let client = make_client().await;

    // Start a simulation to get an ARN.
    let started = client
        .start_simulation()
        .name("tagged-sim")
        .role_arn("arn:aws:iam::123456789012:role/SimRole")
        .send()
        .await
        .expect("start_simulation");
    let arn = started.arn().expect("ARN").to_string();

    // Tag the resource.
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("Env", "test")
        .tags("Project", "winterbaume")
        .send()
        .await
        .expect("tag_resource");

    // Verify tags are returned.
    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags");
    let tag_map = tags.tags().expect("tags should be present");
    assert_eq!(tag_map.get("Env").map(|s| s.as_str()), Some("test"));
    assert_eq!(
        tag_map.get("Project").map(|s| s.as_str()),
        Some("winterbaume")
    );

    // Remove one tag and verify.
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag_resource");

    let after = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags after untag");
    let empty = std::collections::HashMap::new();
    let after_tags = after.tags().unwrap_or(&empty);
    assert!(!after_tags.contains_key("Env"), "Env tag should be removed");
    assert!(
        after_tags.contains_key("Project"),
        "Project tag should remain"
    );
}
