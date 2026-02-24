use std::sync::{Arc, Mutex};

use aws_sdk_amplifybackend::config::BehaviorVersion;
use winterbaume_amplifybackend::AmplifyBackendService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_amplifybackend::Client {
    let mock = MockAws::builder()
        .with_service(AmplifyBackendService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplifybackend::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_amplifybackend::Client::new(&config)
}

#[tokio::test]
async fn test_create_backend_lifecycle() {
    let client = make_client().await;
    let resp = client
        .create_backend()
        .app_id("d1abc123")
        .app_name("MyApp")
        .backend_environment_name("dev")
        .send()
        .await
        .expect("create_backend");
    assert_eq!(resp.app_id(), Some("d1abc123"));
    assert_eq!(resp.backend_environment_name(), Some("dev"));
    assert_eq!(resp.status(), Some("COMPLETED"));

    let get = client
        .get_backend()
        .app_id("d1abc123")
        .backend_environment_name("dev")
        .send()
        .await
        .expect("get_backend");
    assert_eq!(get.app_id(), Some("d1abc123"));
    assert_eq!(get.app_name(), Some("MyApp"));
    assert_eq!(get.backend_environment_name(), Some("dev"));

    client
        .delete_backend()
        .app_id("d1abc123")
        .backend_environment_name("dev")
        .send()
        .await
        .expect("delete_backend");

    let err = client
        .get_backend()
        .app_id("d1abc123")
        .backend_environment_name("dev")
        .send()
        .await
        .expect_err("get after delete should fail");
    assert!(format!("{:?}", err).contains("NotFoundException"));
}

#[tokio::test]
async fn test_create_duplicate_backend_fails() {
    let client = make_client().await;
    client
        .create_backend()
        .app_id("d1xyz")
        .app_name("App")
        .backend_environment_name("dev")
        .send()
        .await
        .expect("first create");

    let err = client
        .create_backend()
        .app_id("d1xyz")
        .app_name("App")
        .backend_environment_name("dev")
        .send()
        .await
        .expect_err("dup create should fail");
    assert!(format!("{:?}", err).contains("BadRequestException"));
}

#[tokio::test]
async fn test_get_backend_not_found() {
    let client = make_client().await;
    let err = client
        .get_backend()
        .app_id("missing-app")
        .send()
        .await
        .expect_err("missing app should fail");
    assert!(format!("{:?}", err).contains("NotFoundException"));
}

#[tokio::test]
async fn test_clone_backend() {
    let client = make_client().await;
    client
        .create_backend()
        .app_id("d1clone")
        .app_name("CloneApp")
        .backend_environment_name("dev")
        .send()
        .await
        .expect("create");

    let resp = client
        .clone_backend()
        .app_id("d1clone")
        .backend_environment_name("dev")
        .target_environment_name("staging")
        .send()
        .await
        .expect("clone");
    assert_eq!(resp.backend_environment_name(), Some("staging"));

    let get = client
        .get_backend()
        .app_id("d1clone")
        .backend_environment_name("staging")
        .send()
        .await
        .expect("get cloned env");
    assert_eq!(get.app_name(), Some("CloneApp"));
}

#[tokio::test]
async fn test_get_backend_lists_environments() {
    let client = make_client().await;
    for env in ["dev", "staging", "prod"] {
        client
            .create_backend()
            .app_id("d1multi")
            .app_name("MultiApp")
            .backend_environment_name(env)
            .send()
            .await
            .expect("create");
    }

    let resp = client
        .get_backend()
        .app_id("d1multi")
        .send()
        .await
        .expect("get");
    let envs = resp.backend_environment_list();
    assert_eq!(envs.len(), 3);
    assert!(envs.iter().any(|e| e == "dev"));
    assert!(envs.iter().any(|e| e == "staging"));
    assert!(envs.iter().any(|e| e == "prod"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AmplifyBackendService::new();
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
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}
