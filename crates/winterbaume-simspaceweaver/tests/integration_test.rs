use std::sync::{Arc, Mutex};

use aws_sdk_simspaceweaver::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
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
async fn test_simulation_lifecycle() {
    let client = make_client().await;
    client
        .start_simulation()
        .name("sim-test")
        .role_arn("arn:aws:iam::123:role/SimRole")
        .description("test sim")
        .send()
        .await
        .expect("start_simulation");

    let list = client.list_simulations().send().await.expect("list");
    assert_eq!(list.simulations().len(), 1);

    let desc = client
        .describe_simulation()
        .simulation("sim-test")
        .send()
        .await
        .expect("describe");
    assert_eq!(desc.name(), Some("sim-test"));
    assert_eq!(desc.status().map(|s| s.as_str()), Some("STARTING"));

    client
        .stop_simulation()
        .simulation("sim-test")
        .send()
        .await
        .expect("stop");
    let after = client
        .describe_simulation()
        .simulation("sim-test")
        .send()
        .await
        .expect("describe2");
    assert_eq!(after.status().map(|s| s.as_str()), Some("STOPPING"));

    client
        .delete_simulation()
        .simulation("sim-test")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_app_lifecycle_within_simulation() {
    let client = make_client().await;
    client
        .start_simulation()
        .name("sim-app")
        .role_arn("arn:aws:iam::123:role/SimRole")
        .send()
        .await
        .expect("start sim");

    client
        .start_app()
        .simulation("sim-app")
        .domain("Workers")
        .name("worker-1")
        .send()
        .await
        .expect("start app");

    let apps = client
        .list_apps()
        .simulation("sim-app")
        .send()
        .await
        .expect("list apps");
    assert_eq!(apps.apps().len(), 1);

    client
        .stop_app()
        .simulation("sim-app")
        .domain("Workers")
        .app("worker-1")
        .send()
        .await
        .expect("stop app");

    client
        .delete_app()
        .simulation("sim-app")
        .domain("Workers")
        .app("worker-1")
        .send()
        .await
        .expect("delete app");
}

#[tokio::test]
async fn test_clock_state() {
    let client = make_client().await;
    client
        .start_simulation()
        .name("sim-clock")
        .role_arn("arn:aws:iam::123:role/SimRole")
        .send()
        .await
        .expect("start sim");
    client
        .start_clock()
        .simulation("sim-clock")
        .send()
        .await
        .expect("start clock");
    client
        .stop_clock()
        .simulation("sim-clock")
        .send()
        .await
        .expect("stop clock");
}

#[tokio::test]
async fn test_describe_simulation_not_found() {
    let client = make_client().await;
    let err = client
        .describe_simulation()
        .simulation("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = SimSpaceWeaverService::new();
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
    assert_eq!(events.lock().unwrap().len(), 1);
}
