use std::sync::{Arc, Mutex};

use aws_sdk_ssmquicksetup::config::BehaviorVersion;
use aws_sdk_ssmquicksetup::types::ConfigurationDefinitionInput;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_ssmquicksetup::SsmQuickSetupService;

async fn make_client() -> aws_sdk_ssmquicksetup::Client {
    let mock = MockAws::builder()
        .with_service(SsmQuickSetupService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssmquicksetup::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_ssmquicksetup::Client::new(&config)
}

fn sample_definition() -> ConfigurationDefinitionInput {
    ConfigurationDefinitionInput::builder()
        .r#type("AWSConfigSetup")
        .type_version("1.0")
        .parameters("ResourceTypes", "AWS::EC2::Instance")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_manager_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_configuration_manager()
        .name("config-mgr")
        .description("test mgr")
        .configuration_definitions(sample_definition())
        .send()
        .await
        .expect("create");
    let arn = create.manager_arn().to_string();

    let get = client
        .get_configuration_manager()
        .manager_arn(&arn)
        .send()
        .await
        .expect("get");
    assert_eq!(get.name(), Some("config-mgr"));
    assert_eq!(get.configuration_definitions().len(), 1);

    let list = client
        .list_configuration_managers()
        .send()
        .await
        .expect("list");
    assert_eq!(list.configuration_managers_list().len(), 1);

    client
        .update_configuration_manager()
        .manager_arn(&arn)
        .description("updated")
        .send()
        .await
        .expect("update");

    let after = client
        .get_configuration_manager()
        .manager_arn(&arn)
        .send()
        .await
        .expect("after");
    assert_eq!(after.description(), Some("updated"));

    client
        .delete_configuration_manager()
        .manager_arn(&arn)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_get_manager_not_found() {
    let client = make_client().await;
    let err = client
        .get_configuration_manager()
        .manager_arn("arn:aws:ssm-quicksetup:us-east-1:123:configuration-manager/missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_quick_setup_types() {
    let client = make_client().await;
    let resp = client
        .list_quick_setup_types()
        .send()
        .await
        .expect("list_types");
    assert!(!resp.quick_setup_type_list().is_empty());
}

#[tokio::test]
async fn test_service_settings_round_trip() {
    let client = make_client().await;
    client
        .update_service_settings()
        .explorer_enabling_role_arn("arn:aws:iam::123:role/Explorer")
        .send()
        .await
        .expect("update");
    let resp = client.get_service_settings().send().await.expect("get");
    let settings = resp.service_settings().expect("settings");
    assert_eq!(
        settings.explorer_enabling_role_arn(),
        Some("arn:aws:iam::123:role/Explorer")
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = SsmQuickSetupService::new();
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
