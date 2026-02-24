use aws_sdk_apprunner::config::BehaviorVersion;
use winterbaume_apprunner::AppRunnerService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_apprunner::Client {
    let mock = MockAws::builder()
        .with_service(AppRunnerService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apprunner::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_apprunner::Client::new(&config)
}

#[tokio::test]
async fn test_list_services_empty() {
    let client = make_client().await;
    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    assert_eq!(resp.service_summary_list().len(), 0);
}

#[tokio::test]
async fn test_create_and_describe_service() {
    let client = make_client().await;

    // Create via describe - service doesn't exist, expect error
    let err = client
        .describe_service()
        .service_arn("arn:aws:apprunner:us-east-1:000000000000:service/nonexistent/abc")
        .send()
        .await;
    assert!(err.is_err(), "describe of nonexistent service should fail");

    // Create a service
    use aws_sdk_apprunner::types::{
        CodeRepository, SourceCodeVersion, SourceCodeVersionType, SourceConfiguration,
    };
    let source_config = SourceConfiguration::builder()
        .code_repository(
            CodeRepository::builder()
                .repository_url("https://github.com/example/repo")
                .source_code_version(
                    SourceCodeVersion::builder()
                        .r#type(SourceCodeVersionType::Branch)
                        .value("main")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build();
    let resp = client
        .create_service()
        .service_name("my-service")
        .source_configuration(source_config)
        .send()
        .await
        .expect("create_service should succeed");

    let svc = resp.service().expect("should have service");
    assert_eq!(svc.service_name(), "my-service");
    assert_eq!(svc.status().as_str(), "RUNNING");

    // Describe the created service
    let arn = svc.service_arn().to_string();
    let describe_resp = client
        .describe_service()
        .service_arn(&arn)
        .send()
        .await
        .expect("describe_service should succeed");
    let described = describe_resp.service().expect("should have service");
    assert_eq!(described.service_arn(), arn);
}

#[tokio::test]
async fn test_list_services() {
    let client = make_client().await;

    use aws_sdk_apprunner::types::{
        CodeRepository, SourceCodeVersion, SourceCodeVersionType, SourceConfiguration,
    };

    for name in ["svc-a", "svc-b", "svc-c"] {
        let source_config = SourceConfiguration::builder()
            .code_repository(
                CodeRepository::builder()
                    .repository_url("https://github.com/example/repo")
                    .source_code_version(
                        SourceCodeVersion::builder()
                            .r#type(SourceCodeVersionType::Branch)
                            .value("main")
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build();
        client
            .create_service()
            .service_name(name)
            .source_configuration(source_config)
            .send()
            .await
            .expect("create_service should succeed");
    }

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    assert_eq!(resp.service_summary_list().len(), 3);
}

#[tokio::test]
async fn test_delete_service() {
    let client = make_client().await;

    use aws_sdk_apprunner::types::{
        CodeRepository, SourceCodeVersion, SourceCodeVersionType, SourceConfiguration,
    };
    let source_config = SourceConfiguration::builder()
        .code_repository(
            CodeRepository::builder()
                .repository_url("https://github.com/example/repo")
                .source_code_version(
                    SourceCodeVersion::builder()
                        .r#type(SourceCodeVersionType::Branch)
                        .value("main")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build();
    let create_resp = client
        .create_service()
        .service_name("to-delete")
        .source_configuration(source_config)
        .send()
        .await
        .expect("create_service should succeed");
    let arn = create_resp.service().unwrap().service_arn().to_string();

    let delete_resp = client
        .delete_service()
        .service_arn(&arn)
        .send()
        .await
        .expect("delete_service should succeed");
    assert_eq!(delete_resp.service().unwrap().status().as_str(), "DELETED");

    // Should be gone now
    let err = client.describe_service().service_arn(&arn).send().await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_pause_and_resume_service() {
    let client = make_client().await;

    use aws_sdk_apprunner::types::{
        CodeRepository, SourceCodeVersion, SourceCodeVersionType, SourceConfiguration,
    };
    let source_config = SourceConfiguration::builder()
        .code_repository(
            CodeRepository::builder()
                .repository_url("https://github.com/example/repo")
                .source_code_version(
                    SourceCodeVersion::builder()
                        .r#type(SourceCodeVersionType::Branch)
                        .value("main")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build();
    let create_resp = client
        .create_service()
        .service_name("pause-resume-svc")
        .source_configuration(source_config)
        .send()
        .await
        .unwrap();
    let arn = create_resp.service().unwrap().service_arn().to_string();

    let pause_resp = client
        .pause_service()
        .service_arn(&arn)
        .send()
        .await
        .expect("pause_service should succeed");
    assert_eq!(pause_resp.service().unwrap().status().as_str(), "PAUSED");

    let resume_resp = client
        .resume_service()
        .service_arn(&arn)
        .send()
        .await
        .expect("resume_service should succeed");
    assert_eq!(resume_resp.service().unwrap().status().as_str(), "RUNNING");
}

#[tokio::test]
async fn test_create_and_list_connections() {
    let client = make_client().await;

    use aws_sdk_apprunner::types::ProviderType;
    let resp = client
        .create_connection()
        .connection_name("my-conn")
        .provider_type(ProviderType::Github)
        .send()
        .await
        .expect("create_connection should succeed");
    let conn = resp.connection().expect("should have connection");
    assert_eq!(conn.connection_name(), Some("my-conn"));
    assert_eq!(conn.status().unwrap().as_str(), "PENDING_HANDSHAKE");

    let list_resp = client
        .list_connections()
        .send()
        .await
        .expect("list_connections should succeed");
    assert_eq!(list_resp.connection_summary_list().len(), 1);
}

#[tokio::test]
async fn test_delete_connection() {
    let client = make_client().await;

    use aws_sdk_apprunner::types::ProviderType;
    let create_resp = client
        .create_connection()
        .connection_name("conn-to-delete")
        .provider_type(ProviderType::Github)
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .connection()
        .unwrap()
        .connection_arn()
        .unwrap()
        .to_string();

    client
        .delete_connection()
        .connection_arn(&arn)
        .send()
        .await
        .expect("delete_connection should succeed");

    let list_resp = client.list_connections().send().await.unwrap();
    assert_eq!(list_resp.connection_summary_list().len(), 0);
}

#[tokio::test]
async fn test_create_and_describe_auto_scaling_configuration() {
    let client = make_client().await;

    let create_resp = client
        .create_auto_scaling_configuration()
        .auto_scaling_configuration_name("my-asc")
        .min_size(2)
        .max_size(10)
        .max_concurrency(50)
        .send()
        .await
        .expect("create_auto_scaling_configuration should succeed");

    let cfg = create_resp
        .auto_scaling_configuration()
        .expect("should have config");
    assert_eq!(cfg.auto_scaling_configuration_name().unwrap(), "my-asc");
    assert_eq!(cfg.min_size(), Some(2));
    assert_eq!(cfg.max_size(), Some(10));
    assert_eq!(cfg.max_concurrency(), Some(50));
    assert_eq!(cfg.status().unwrap().as_str(), "ACTIVE");

    let arn = cfg.auto_scaling_configuration_arn().unwrap().to_string();
    let describe_resp = client
        .describe_auto_scaling_configuration()
        .auto_scaling_configuration_arn(&arn)
        .send()
        .await
        .expect("describe_auto_scaling_configuration should succeed");
    let described = describe_resp
        .auto_scaling_configuration()
        .expect("should have config");
    assert_eq!(described.auto_scaling_configuration_arn().unwrap(), arn);
}

#[tokio::test]
async fn test_list_auto_scaling_configurations() {
    let client = make_client().await;

    for name in ["asc-1", "asc-2"] {
        client
            .create_auto_scaling_configuration()
            .auto_scaling_configuration_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_auto_scaling_configurations()
        .send()
        .await
        .expect("list_auto_scaling_configurations should succeed");
    assert_eq!(resp.auto_scaling_configuration_summary_list().len(), 2);
}

#[tokio::test]
async fn test_delete_auto_scaling_configuration() {
    let client = make_client().await;

    let create_resp = client
        .create_auto_scaling_configuration()
        .auto_scaling_configuration_name("asc-to-delete")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .auto_scaling_configuration()
        .unwrap()
        .auto_scaling_configuration_arn()
        .unwrap()
        .to_string();

    client
        .delete_auto_scaling_configuration()
        .auto_scaling_configuration_arn(&arn)
        .send()
        .await
        .expect("delete_auto_scaling_configuration should succeed");

    let resp = client
        .list_auto_scaling_configurations()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.auto_scaling_configuration_summary_list().len(), 0);
}

#[tokio::test]
async fn test_tag_and_untag_resource() {
    let client = make_client().await;

    use aws_sdk_apprunner::types::{
        CodeRepository, SourceCodeVersion, SourceCodeVersionType, SourceConfiguration, Tag,
    };
    let source_config = SourceConfiguration::builder()
        .code_repository(
            CodeRepository::builder()
                .repository_url("https://github.com/example/repo")
                .source_code_version(
                    SourceCodeVersion::builder()
                        .r#type(SourceCodeVersionType::Branch)
                        .value("main")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build();
    let create_resp = client
        .create_service()
        .service_name("tagged-svc")
        .source_configuration(source_config)
        .send()
        .await
        .unwrap();
    let arn = create_resp.service().unwrap().service_arn().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("env").value("test").build())
        .tags(Tag::builder().key("team").value("platform").build())
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    assert_eq!(list_resp.tags().len(), 2);

    // Untag one
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp2.tags().len(), 1);
    assert_eq!(list_resp2.tags()[0].key().unwrap(), "team");
}

#[tokio::test]
async fn test_state_snapshot_restore() {
    use std::collections::HashMap;

    use winterbaume_apprunner::AppRunnerStateView;
    use winterbaume_apprunner::views::AppRunnerServiceView;
    use winterbaume_core::StatefulService;

    let svc = AppRunnerService::new();
    let account_id = "123456789012";
    let region = "us-east-1";

    let mut services = HashMap::new();
    services.insert(
        "test-svc".to_string(),
        AppRunnerServiceView {
            service_id: "id-123".to_string(),
            service_name: "test-svc".to_string(),
            service_arn: "arn:aws:apprunner:us-east-1:123456789012:service/test-svc/id-123"
                .to_string(),
            service_url: "test-svc.us-east-1.awsapprunner.com".to_string(),
            status: "RUNNING".to_string(),
            created_at: 1000.0,
            updated_at: 1000.0,
            tags: vec![],
            encryption_configuration: None,
            health_check_configuration: None,
            instance_configuration: None,
            network_configuration: None,
            observability_configuration: None,
            source_configuration: None,
        },
    );

    let view = AppRunnerStateView {
        services,
        ..Default::default()
    };

    svc.restore(account_id, region, view)
        .await
        .expect("restore should succeed");

    let snapshot = svc.snapshot(account_id, region).await;
    assert!(snapshot.services.contains_key("test-svc"));
    assert_eq!(snapshot.services["test-svc"].service_id, "id-123");
}

#[tokio::test]
async fn test_state_merge() {
    use std::collections::HashMap;

    use winterbaume_apprunner::AppRunnerStateView;
    use winterbaume_apprunner::views::AppRunnerServiceView;
    use winterbaume_core::StatefulService;

    let svc = AppRunnerService::new();
    let account_id = "123456789012";
    let region = "us-east-1";

    // First restore with service A
    let mut services_a = HashMap::new();
    services_a.insert(
        "svc-a".to_string(),
        AppRunnerServiceView {
            service_id: "id-a".to_string(),
            service_name: "svc-a".to_string(),
            service_arn: "arn:aws:apprunner:us-east-1:123456789012:service/svc-a/id-a".to_string(),
            service_url: "svc-a.us-east-1.awsapprunner.com".to_string(),
            status: "RUNNING".to_string(),
            created_at: 1000.0,
            updated_at: 1000.0,
            tags: vec![],
            encryption_configuration: None,
            health_check_configuration: None,
            instance_configuration: None,
            network_configuration: None,
            observability_configuration: None,
            source_configuration: None,
        },
    );
    svc.restore(
        account_id,
        region,
        AppRunnerStateView {
            services: services_a,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    // Merge with service B
    let mut services_b = HashMap::new();
    services_b.insert(
        "svc-b".to_string(),
        AppRunnerServiceView {
            service_id: "id-b".to_string(),
            service_name: "svc-b".to_string(),
            service_arn: "arn:aws:apprunner:us-east-1:123456789012:service/svc-b/id-b".to_string(),
            service_url: "svc-b.us-east-1.awsapprunner.com".to_string(),
            status: "RUNNING".to_string(),
            created_at: 2000.0,
            updated_at: 2000.0,
            tags: vec![],
            encryption_configuration: None,
            health_check_configuration: None,
            instance_configuration: None,
            network_configuration: None,
            observability_configuration: None,
            source_configuration: None,
        },
    );
    svc.merge(
        account_id,
        region,
        AppRunnerStateView {
            services: services_b,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let snapshot = svc.snapshot(account_id, region).await;
    // Both should be present
    assert!(snapshot.services.contains_key("svc-a"));
    assert!(snapshot.services.contains_key("svc-b"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = AppRunnerService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        winterbaume_apprunner::AppRunnerStateView::default(),
    )
    .await
    .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use winterbaume_apprunner::AppRunnerStateView;
    use winterbaume_apprunner::views::AppRunnerServiceView;
    use winterbaume_core::StatefulService;

    let svc = AppRunnerService::new();

    // Pre-seed to get first event out of the way
    svc.restore("123456789012", "us-east-1", AppRunnerStateView::default())
        .await
        .unwrap();

    // Register listener after initial restore
    let snapshots: Arc<Mutex<Vec<AppRunnerStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let mut services = HashMap::new();
    services.insert(
        "listener-svc".to_string(),
        AppRunnerServiceView {
            service_id: "id-listener".to_string(),
            service_name: "listener-svc".to_string(),
            service_arn:
                "arn:aws:apprunner:us-east-1:123456789012:service/listener-svc/id-listener"
                    .to_string(),
            service_url: "listener-svc.us-east-1.awsapprunner.com".to_string(),
            status: "RUNNING".to_string(),
            created_at: 3000.0,
            updated_at: 3000.0,
            tags: vec![],
            encryption_configuration: None,
            health_check_configuration: None,
            instance_configuration: None,
            network_configuration: None,
            observability_configuration: None,
            source_configuration: None,
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        AppRunnerStateView {
            services,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].services.contains_key("listener-svc"));
}

// ============================================================================
// Additional operation coverage
// ============================================================================

fn make_source_config() -> aws_sdk_apprunner::types::SourceConfiguration {
    use aws_sdk_apprunner::types::{
        CodeRepository, SourceCodeVersion, SourceCodeVersionType, SourceConfiguration,
    };
    SourceConfiguration::builder()
        .code_repository(
            CodeRepository::builder()
                .repository_url("https://github.com/example/repo")
                .source_code_version(
                    SourceCodeVersion::builder()
                        .r#type(SourceCodeVersionType::Branch)
                        .value("main")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
}

#[tokio::test]
async fn test_update_service() {
    let client = make_client().await;
    let create_resp = client
        .create_service()
        .service_name("update-svc")
        .source_configuration(make_source_config())
        .send()
        .await
        .unwrap();
    let arn = create_resp.service().unwrap().service_arn().to_string();

    let update_resp = client
        .update_service()
        .service_arn(&arn)
        .source_configuration(make_source_config())
        .send()
        .await
        .expect("update_service should succeed");
    assert!(update_resp.service().is_some());
}

#[tokio::test]
async fn test_start_deployment() {
    let client = make_client().await;
    let create_resp = client
        .create_service()
        .service_name("deploy-svc")
        .source_configuration(make_source_config())
        .send()
        .await
        .unwrap();
    let arn = create_resp.service().unwrap().service_arn().to_string();

    let deploy_resp = client
        .start_deployment()
        .service_arn(&arn)
        .send()
        .await
        .expect("start_deployment should succeed");
    assert!(!deploy_resp.operation_id().is_empty());
}

#[tokio::test]
async fn test_update_default_auto_scaling_configuration() {
    let client = make_client().await;
    let create_resp = client
        .create_auto_scaling_configuration()
        .auto_scaling_configuration_name("default-asc")
        .min_size(1)
        .max_size(5)
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .auto_scaling_configuration()
        .unwrap()
        .auto_scaling_configuration_arn()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_default_auto_scaling_configuration()
        .auto_scaling_configuration_arn(&arn)
        .send()
        .await
        .expect("update_default_auto_scaling_configuration should succeed");
    assert!(update_resp.auto_scaling_configuration().is_some());
}

#[tokio::test]
async fn test_describe_nonexistent_service() {
    let client = make_client().await;
    let err = client
        .describe_service()
        .service_arn("arn:aws:apprunner:us-east-1:000000000000:service/nope/nope")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_service() {
    let client = make_client().await;
    let err = client
        .delete_service()
        .service_arn("arn:aws:apprunner:us-east-1:000000000000:service/nope/nope")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_service_full_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_service()
        .service_name("lifecycle-svc")
        .source_configuration(make_source_config())
        .send()
        .await
        .unwrap();
    let arn = create_resp.service().unwrap().service_arn().to_string();

    // Describe
    let desc = client
        .describe_service()
        .service_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.service().unwrap().service_name(), "lifecycle-svc");

    // List
    let list = client.list_services().send().await.unwrap();
    assert_eq!(list.service_summary_list().len(), 1);

    // Pause
    client
        .pause_service()
        .service_arn(&arn)
        .send()
        .await
        .unwrap();
    let desc2 = client
        .describe_service()
        .service_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.service().unwrap().status().as_str(), "PAUSED");

    // Resume
    client
        .resume_service()
        .service_arn(&arn)
        .send()
        .await
        .unwrap();

    // Delete
    let del = client
        .delete_service()
        .service_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(del.service().unwrap().status().as_str(), "DELETED");

    // Verify gone
    assert!(
        client
            .describe_service()
            .service_arn(&arn)
            .send()
            .await
            .is_err()
    );
}
