use std::sync::{Arc, Mutex};

use aws_sdk_backupgateway::config::BehaviorVersion;
use aws_sdk_backupgateway::types::{Tag, VmwareToAwsTagMapping};
use winterbaume_backupgateway::BackupGatewayService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_backupgateway::Client {
    let mock = MockAws::builder()
        .with_service(BackupGatewayService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backupgateway::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_backupgateway::Client::new(&config)
}

#[tokio::test]
async fn test_gateway_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_gateway()
        .activation_key("activation-xyz")
        .gateway_display_name("MyGW")
        .gateway_type("BACKUP_VM".into())
        .send()
        .await
        .expect("create");
    let arn = create.gateway_arn().expect("arn").to_string();

    let got = client
        .get_gateway()
        .gateway_arn(&arn)
        .send()
        .await
        .expect("get");
    assert_eq!(got.gateway().unwrap().gateway_display_name(), Some("MyGW"));

    let list = client.list_gateways().send().await.expect("list");
    assert_eq!(list.gateways().len(), 1);

    client
        .update_gateway_information()
        .gateway_arn(&arn)
        .gateway_display_name("Renamed")
        .send()
        .await
        .expect("update");

    let got2 = client
        .get_gateway()
        .gateway_arn(&arn)
        .send()
        .await
        .expect("get2");
    assert_eq!(
        got2.gateway().unwrap().gateway_display_name(),
        Some("Renamed")
    );

    client
        .delete_gateway()
        .gateway_arn(&arn)
        .send()
        .await
        .expect("delete");
    let err = client
        .get_gateway()
        .gateway_arn(&arn)
        .send()
        .await
        .expect_err("gone");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_hypervisor_lifecycle() {
    let client = make_client().await;
    let create = client
        .import_hypervisor_configuration()
        .host("vc.example.com")
        .name("MyHV")
        .send()
        .await
        .expect("import");
    let arn = create.hypervisor_arn().expect("arn").to_string();

    let got = client
        .get_hypervisor()
        .hypervisor_arn(&arn)
        .send()
        .await
        .expect("get");
    assert_eq!(got.hypervisor().unwrap().name(), Some("MyHV"));

    client
        .update_hypervisor()
        .hypervisor_arn(&arn)
        .name("RenamedHV")
        .send()
        .await
        .expect("update");

    let list = client.list_hypervisors().send().await.expect("list");
    assert_eq!(list.hypervisors().len(), 1);

    client
        .delete_hypervisor()
        .hypervisor_arn(&arn)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_associate_disassociate() {
    let client = make_client().await;
    let create = client
        .create_gateway()
        .activation_key("k")
        .gateway_display_name("AssocGW")
        .gateway_type("BACKUP_VM".into())
        .send()
        .await
        .expect("create");
    let arn = create.gateway_arn().unwrap().to_string();

    client
        .associate_gateway_to_server()
        .gateway_arn(&arn)
        .server_arn("arn:aws:vmware:us-east-1:123:server/test")
        .send()
        .await
        .expect("associate");

    let got = client
        .get_gateway()
        .gateway_arn(&arn)
        .send()
        .await
        .expect("get");
    assert!(got.gateway().unwrap().hypervisor_id().is_some());

    client
        .disassociate_gateway_from_server()
        .gateway_arn(&arn)
        .send()
        .await
        .expect("disassociate");
    let got2 = client
        .get_gateway()
        .gateway_arn(&arn)
        .send()
        .await
        .expect("get2");
    assert!(got2.gateway().unwrap().hypervisor_id().is_none());
}

#[tokio::test]
async fn test_maintenance_start_time() {
    let client = make_client().await;
    let create = client
        .create_gateway()
        .activation_key("k")
        .gateway_display_name("MaintGW")
        .gateway_type("BACKUP_VM".into())
        .send()
        .await
        .expect("create");
    let arn = create.gateway_arn().unwrap().to_string();

    client
        .put_maintenance_start_time()
        .gateway_arn(&arn)
        .hour_of_day(3)
        .minute_of_hour(30)
        .day_of_week(2)
        .send()
        .await
        .expect("put maint");

    let got = client
        .get_gateway()
        .gateway_arn(&arn)
        .send()
        .await
        .expect("get");
    let mst = got.gateway().unwrap().maintenance_start_time().unwrap();
    assert_eq!(mst.hour_of_day(), 3);
}

#[tokio::test]
async fn test_hypervisor_property_mappings() {
    let client = make_client().await;
    let create = client
        .import_hypervisor_configuration()
        .host("vc")
        .name("MapHV")
        .send()
        .await
        .expect("import");
    let arn = create.hypervisor_arn().unwrap().to_string();

    client
        .put_hypervisor_property_mappings()
        .hypervisor_arn(&arn)
        .iam_role_arn("arn:aws:iam::123:role/r")
        .vmware_to_aws_tag_mappings(
            VmwareToAwsTagMapping::builder()
                .aws_tag_key("Env")
                .aws_tag_value("Prod")
                .vmware_category("Cat")
                .vmware_tag_name("Tag")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put");

    let got = client
        .get_hypervisor_property_mappings()
        .hypervisor_arn(&arn)
        .send()
        .await
        .expect("get");
    assert_eq!(got.vmware_to_aws_tag_mappings().len(), 1);
}

#[tokio::test]
async fn test_test_hypervisor_configuration() {
    let client = make_client().await;
    let create = client
        .create_gateway()
        .activation_key("k")
        .gateway_display_name("TestHV")
        .gateway_type("BACKUP_VM".into())
        .send()
        .await
        .expect("create");
    let arn = create.gateway_arn().unwrap().to_string();
    client
        .test_hypervisor_configuration()
        .gateway_arn(&arn)
        .host("vc")
        .send()
        .await
        .expect("test");
}

#[tokio::test]
async fn test_list_virtual_machines_empty() {
    let client = make_client().await;
    let resp = client.list_virtual_machines().send().await.expect("list");
    assert!(resp.virtual_machines().is_empty());
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_gateway()
        .activation_key("k")
        .gateway_display_name("Tagged")
        .gateway_type("BACKUP_VM".into())
        .send()
        .await
        .expect("create");
    let arn = create.gateway_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("Env").value("prod").build().unwrap())
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(tags.tags().len(), 1);

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = BackupGatewayService::new();
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

#[tokio::test]
async fn test_state_view_round_trip() {
    use winterbaume_backupgateway::views::{BackupGatewayStateView, GatewayView};
    let svc = BackupGatewayService::new();
    let mut view = BackupGatewayStateView::default();
    view.gateways.insert(
        "arn:aws:backup-gateway:us-east-1:123:gateway/seed".to_string(),
        GatewayView {
            arn: "arn:aws:backup-gateway:us-east-1:123:gateway/seed".to_string(),
            display_name: "Seed".to_string(),
            gateway_type: "BACKUP_VM".to_string(),
            software_version: "1.0".to_string(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.gateways.len(), 1);
}
