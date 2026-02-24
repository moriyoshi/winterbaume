use aws_sdk_servicediscovery::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicediscovery::ServiceDiscoveryService;

async fn make_sd_client() -> aws_sdk_servicediscovery::Client {
    let mock = MockAws::builder()
        .with_service(ServiceDiscoveryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicediscovery::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_servicediscovery::Client::new(&config)
}

// ========== Namespace tests ==========

#[tokio::test]
async fn test_create_private_dns_namespace() {
    let client = make_sd_client().await;

    let resp = client
        .create_private_dns_namespace()
        .name("example.local")
        .vpc("vpc-12345678")
        .description("Test namespace")
        .send()
        .await
        .expect("create_private_dns_namespace should succeed");

    let op_id = resp.operation_id().expect("should have operation ID");
    assert!(!op_id.is_empty());
}

#[tokio::test]
async fn test_create_http_namespace() {
    let client = make_sd_client().await;

    let resp = client
        .create_http_namespace()
        .name("example-http")
        .description("HTTP namespace")
        .send()
        .await
        .expect("create_http_namespace should succeed");

    let op_id = resp.operation_id().expect("should have operation ID");
    assert!(!op_id.is_empty());
}

#[tokio::test]
async fn test_create_public_dns_namespace() {
    let client = make_sd_client().await;

    let resp = client
        .create_public_dns_namespace()
        .name("example.com")
        .description("Public DNS namespace")
        .send()
        .await
        .expect("create_public_dns_namespace should succeed");

    let op_id = resp.operation_id().expect("should have operation ID");
    assert!(!op_id.is_empty());
}

#[tokio::test]
async fn test_get_namespace() {
    let client = make_sd_client().await;

    client
        .create_private_dns_namespace()
        .name("get-test.local")
        .vpc("vpc-12345678")
        .description("Namespace for get test")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns = list_resp
        .namespaces()
        .first()
        .expect("should have namespace");
    let ns_id = ns.id().expect("namespace should have id");

    let get_resp = client
        .get_namespace()
        .id(ns_id)
        .send()
        .await
        .expect("get_namespace should succeed");

    let namespace = get_resp.namespace().expect("should have namespace");
    assert_eq!(namespace.name(), Some("get-test.local"));
    assert_eq!(
        namespace.r#type(),
        Some(&aws_sdk_servicediscovery::types::NamespaceType::DnsPrivate)
    );
}

#[tokio::test]
async fn test_delete_namespace() {
    let client = make_sd_client().await;

    client
        .create_private_dns_namespace()
        .name("delete-test.local")
        .vpc("vpc-12345678")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns = list_resp.namespaces().first().unwrap();
    let ns_id = ns.id().unwrap().to_string();

    let del_resp = client
        .delete_namespace()
        .id(&ns_id)
        .send()
        .await
        .expect("delete_namespace should succeed");

    assert!(del_resp.operation_id().is_some());

    let result = client.get_namespace().id(&ns_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_namespaces() {
    let client = make_sd_client().await;

    for (i, name) in ["ns1.local", "ns2.local", "ns3.local"].iter().enumerate() {
        client
            .create_private_dns_namespace()
            .name(*name)
            .vpc(format!("vpc-{i:08}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_namespaces()
        .send()
        .await
        .expect("list_namespaces should succeed");

    assert_eq!(resp.namespaces().len(), 3);
}

#[tokio::test]
async fn test_delete_nonexistent_namespace() {
    let client = make_sd_client().await;
    let result = client.delete_namespace().id("ns-nonexistent").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_namespace() {
    let client = make_sd_client().await;
    let result = client.get_namespace().id("ns-nonexistent").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_http_namespace() {
    let client = make_sd_client().await;

    client
        .create_http_namespace()
        .name("update-http.local")
        .description("Original")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns_id = list_resp
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .update_http_namespace()
        .id(&ns_id)
        .namespace(
            aws_sdk_servicediscovery::types::HttpNamespaceChange::builder()
                .description("Updated")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_http_namespace should succeed");

    assert!(resp.operation_id().is_some());

    let get_resp = client.get_namespace().id(&ns_id).send().await.unwrap();
    let ns = get_resp.namespace().unwrap();
    assert_eq!(ns.description(), Some("Updated"));
}

#[tokio::test]
async fn test_update_private_dns_namespace() {
    let client = make_sd_client().await;

    client
        .create_private_dns_namespace()
        .name("update-priv.local")
        .vpc("vpc-12345678")
        .description("Original")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns_id = list_resp
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .update_private_dns_namespace()
        .id(&ns_id)
        .namespace(
            aws_sdk_servicediscovery::types::PrivateDnsNamespaceChange::builder()
                .description("Updated private")
                .build(),
        )
        .send()
        .await
        .expect("update_private_dns_namespace should succeed");

    assert!(resp.operation_id().is_some());
}

#[tokio::test]
async fn test_update_public_dns_namespace() {
    let client = make_sd_client().await;

    client
        .create_public_dns_namespace()
        .name("update-pub.example.com")
        .description("Original")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns_id = list_resp
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .update_public_dns_namespace()
        .id(&ns_id)
        .namespace(
            aws_sdk_servicediscovery::types::PublicDnsNamespaceChange::builder()
                .description("Updated public")
                .build(),
        )
        .send()
        .await
        .expect("update_public_dns_namespace should succeed");

    assert!(resp.operation_id().is_some());
}

// ========== Service tests ==========

async fn create_namespace_and_get_id(client: &aws_sdk_servicediscovery::Client) -> String {
    client
        .create_http_namespace()
        .name("svc-test.local")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    list_resp
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_create_service() {
    let client = make_sd_client().await;
    let ns_id = create_namespace_and_get_id(&client).await;

    let resp = client
        .create_service()
        .name("my-service")
        .namespace_id(&ns_id)
        .description("Test service")
        .send()
        .await
        .expect("create_service should succeed");

    let svc = resp.service().expect("should have service");
    assert_eq!(svc.name(), Some("my-service"));
    assert_eq!(svc.namespace_id(), Some(ns_id.as_str()));
    assert!(svc.id().is_some());
    assert!(svc.arn().is_some());
}

#[tokio::test]
async fn test_get_service() {
    let client = make_sd_client().await;
    let ns_id = create_namespace_and_get_id(&client).await;

    let create_resp = client
        .create_service()
        .name("get-svc")
        .namespace_id(&ns_id)
        .send()
        .await
        .unwrap();

    let svc_id = create_resp.service().unwrap().id().unwrap().to_string();

    let get_resp = client
        .get_service()
        .id(&svc_id)
        .send()
        .await
        .expect("get_service should succeed");

    let svc = get_resp.service().expect("should have service");
    assert_eq!(svc.name(), Some("get-svc"));
}

#[tokio::test]
async fn test_delete_service() {
    let client = make_sd_client().await;
    let ns_id = create_namespace_and_get_id(&client).await;

    let create_resp = client
        .create_service()
        .name("del-svc")
        .namespace_id(&ns_id)
        .send()
        .await
        .unwrap();

    let svc_id = create_resp.service().unwrap().id().unwrap().to_string();

    client
        .delete_service()
        .id(&svc_id)
        .send()
        .await
        .expect("delete_service should succeed");

    let result = client.get_service().id(&svc_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_services() {
    let client = make_sd_client().await;
    let ns_id = create_namespace_and_get_id(&client).await;

    for name in ["svc-a", "svc-b", "svc-c"] {
        client
            .create_service()
            .name(name)
            .namespace_id(&ns_id)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");

    assert_eq!(resp.services().len(), 3);
}

#[tokio::test]
async fn test_update_service() {
    let client = make_sd_client().await;
    let ns_id = create_namespace_and_get_id(&client).await;

    let create_resp = client
        .create_service()
        .name("upd-svc")
        .namespace_id(&ns_id)
        .description("Original")
        .send()
        .await
        .unwrap();

    let svc_id = create_resp.service().unwrap().id().unwrap().to_string();

    let resp = client
        .update_service()
        .id(&svc_id)
        .service(
            aws_sdk_servicediscovery::types::ServiceChange::builder()
                .description("Updated")
                .build(),
        )
        .send()
        .await
        .expect("update_service should succeed");

    assert!(resp.operation_id().is_some());

    let get_resp = client.get_service().id(&svc_id).send().await.unwrap();
    let svc = get_resp.service().unwrap();
    assert_eq!(svc.description(), Some("Updated"));
}

#[tokio::test]
async fn test_delete_nonexistent_service() {
    let client = make_sd_client().await;
    let result = client.delete_service().id("srv-nonexistent").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_service() {
    let client = make_sd_client().await;
    let result = client.get_service().id("srv-nonexistent").send().await;
    assert!(result.is_err());
}

// ========== Instance tests ==========

async fn create_ns_and_svc(client: &aws_sdk_servicediscovery::Client) -> (String, String) {
    let ns_id = create_namespace_and_get_id(client).await;

    let create_resp = client
        .create_service()
        .name("inst-svc")
        .namespace_id(&ns_id)
        .send()
        .await
        .unwrap();

    let svc_id = create_resp.service().unwrap().id().unwrap().to_string();
    (ns_id, svc_id)
}

#[tokio::test]
async fn test_register_instance() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_ns_and_svc(&client).await;

    let resp = client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.0.1")
        .attributes("AWS_INSTANCE_PORT", "8080")
        .send()
        .await
        .expect("register_instance should succeed");

    assert!(resp.operation_id().is_some());
}

#[tokio::test]
async fn test_get_instance() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_ns_and_svc(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-get-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.0.2")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_instance()
        .service_id(&svc_id)
        .instance_id("i-get-001")
        .send()
        .await
        .expect("get_instance should succeed");

    let inst = resp.instance().expect("should have instance");
    assert_eq!(inst.id(), "i-get-001");
    assert_eq!(
        inst.attributes().unwrap().get("AWS_INSTANCE_IPV4"),
        Some(&"10.0.0.2".to_string())
    );
}

#[tokio::test]
async fn test_deregister_instance() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_ns_and_svc(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-dereg-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.0.3")
        .send()
        .await
        .unwrap();

    let resp = client
        .deregister_instance()
        .service_id(&svc_id)
        .instance_id("i-dereg-001")
        .send()
        .await
        .expect("deregister_instance should succeed");

    assert!(resp.operation_id().is_some());

    // Verify instance is gone
    let result = client
        .get_instance()
        .service_id(&svc_id)
        .instance_id("i-dereg-001")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_instances() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_ns_and_svc(&client).await;

    for i in 0..3 {
        client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(format!("i-list-{i:03}"))
            .attributes("AWS_INSTANCE_IPV4", format!("10.0.0.{i}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_instances()
        .service_id(&svc_id)
        .send()
        .await
        .expect("list_instances should succeed");

    assert_eq!(resp.instances().len(), 3);
}

#[tokio::test]
async fn test_get_instances_health_status() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_ns_and_svc(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-health-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.0.10")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_instances_health_status()
        .service_id(&svc_id)
        .send()
        .await
        .expect("get_instances_health_status should succeed");

    let status = resp.status().unwrap();
    assert_eq!(
        status.get("i-health-001").map(|s| s.as_str()),
        Some("HEALTHY")
    );
}

#[tokio::test]
async fn test_update_instance_custom_health_status() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_ns_and_svc(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-hs-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.0.20")
        .send()
        .await
        .unwrap();

    client
        .update_instance_custom_health_status()
        .service_id(&svc_id)
        .instance_id("i-hs-001")
        .status(aws_sdk_servicediscovery::types::CustomHealthStatus::Unhealthy)
        .send()
        .await
        .expect("update_instance_custom_health_status should succeed");

    let resp = client
        .get_instances_health_status()
        .service_id(&svc_id)
        .send()
        .await
        .unwrap();

    let status = resp.status().unwrap();
    assert_eq!(
        status.get("i-hs-001").map(|s| s.as_str()),
        Some("UNHEALTHY")
    );
}

// ========== Discover tests ==========

#[tokio::test]
async fn test_discover_instances() {
    let client = make_sd_client().await;

    client
        .create_http_namespace()
        .name("discover.local")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns_id = list_resp
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_service()
        .name("disc-svc")
        .namespace_id(&ns_id)
        .send()
        .await
        .unwrap();

    let svc_id = create_resp.service().unwrap().id().unwrap().to_string();

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-disc-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.1.1")
        .send()
        .await
        .unwrap();

    let resp = client
        .discover_instances()
        .namespace_name("discover.local")
        .service_name("disc-svc")
        .send()
        .await
        .expect("discover_instances should succeed");

    let instances = resp.instances();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].instance_id(), Some("i-disc-001"));
}

#[tokio::test]
async fn test_discover_instances_revision() {
    let client = make_sd_client().await;

    client
        .create_http_namespace()
        .name("rev.local")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns_id = list_resp
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_service()
        .name("rev-svc")
        .namespace_id(&ns_id)
        .send()
        .await
        .unwrap();

    let svc_id = create_resp.service().unwrap().id().unwrap().to_string();

    // Revision before any instance registrations
    let resp = client
        .discover_instances_revision()
        .namespace_name("rev.local")
        .service_name("rev-svc")
        .send()
        .await
        .expect("discover_instances_revision should succeed");

    let rev_before = resp.instances_revision();

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-rev-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.2.1")
        .send()
        .await
        .unwrap();

    let resp2 = client
        .discover_instances_revision()
        .namespace_name("rev.local")
        .service_name("rev-svc")
        .send()
        .await
        .unwrap();

    assert!(resp2.instances_revision() > rev_before);
}

// ========== Operation tests ==========

#[tokio::test]
async fn test_get_operation() {
    let client = make_sd_client().await;

    let create_resp = client
        .create_http_namespace()
        .name("op-test.local")
        .send()
        .await
        .unwrap();

    let op_id = create_resp.operation_id().unwrap();

    let resp = client
        .get_operation()
        .operation_id(op_id)
        .send()
        .await
        .expect("get_operation should succeed");

    let op = resp.operation().expect("should have operation");
    assert_eq!(op.id(), Some(op_id));
    assert_eq!(
        op.status(),
        Some(&aws_sdk_servicediscovery::types::OperationStatus::Success)
    );
}

#[tokio::test]
async fn test_list_operations() {
    let client = make_sd_client().await;

    client
        .create_http_namespace()
        .name("ops1.local")
        .send()
        .await
        .unwrap();

    client
        .create_http_namespace()
        .name("ops2.local")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_operations()
        .send()
        .await
        .expect("list_operations should succeed");

    assert!(resp.operations().len() >= 2);
}

#[tokio::test]
async fn test_get_nonexistent_operation() {
    let client = make_sd_client().await;
    let result = client
        .get_operation()
        .operation_id("non-existent-op-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ========== Tag tests ==========

#[tokio::test]
async fn test_tag_and_list_tags_namespace() {
    let client = make_sd_client().await;

    client
        .create_http_namespace()
        .name("tag-test.local")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns = list_resp.namespaces().first().unwrap();
    let ns_arn = ns.arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&ns_arn)
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&ns_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_sd_client().await;

    client
        .create_http_namespace()
        .name("untag-test.local")
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns = list_resp.namespaces().first().unwrap();
    let ns_arn = ns.arn().unwrap().to_string();

    client
        .untag_resource()
        .resource_arn(&ns_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&ns_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "team");
    assert_eq!(tags[0].value(), "platform");
}

// ========== Lifecycle tests ==========

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_sd_client().await;

    // 1. Create namespace
    client
        .create_http_namespace()
        .name("lifecycle.local")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_namespaces().send().await.unwrap();
    let ns_id = list_resp
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // 2. Create service
    let svc_resp = client
        .create_service()
        .name("lifecycle-svc")
        .namespace_id(&ns_id)
        .send()
        .await
        .unwrap();
    let svc_id = svc_resp.service().unwrap().id().unwrap().to_string();

    // 3. Register instance
    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-lc-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.5.1")
        .send()
        .await
        .unwrap();

    // 4. Discover instance
    let disc_resp = client
        .discover_instances()
        .namespace_name("lifecycle.local")
        .service_name("lifecycle-svc")
        .send()
        .await
        .unwrap();
    assert_eq!(disc_resp.instances().len(), 1);

    // 5. Deregister instance
    client
        .deregister_instance()
        .service_id(&svc_id)
        .instance_id("i-lc-001")
        .send()
        .await
        .unwrap();

    // 6. Delete service (should succeed since no instances)
    client
        .delete_service()
        .id(&svc_id)
        .send()
        .await
        .expect("delete_service should succeed after deregistering instances");

    // 7. Delete namespace
    client
        .delete_namespace()
        .id(&ns_id)
        .send()
        .await
        .expect("delete_namespace should succeed after deleting services");

    // 8. Verify namespace is gone
    let result = client.get_namespace().id(&ns_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_cannot_delete_service_with_instances() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_ns_and_svc(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-nodelete-001")
        .attributes("AWS_INSTANCE_IPV4", "10.0.6.1")
        .send()
        .await
        .unwrap();

    let result = client.delete_service().id(&svc_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_cannot_delete_namespace_with_services() {
    let client = make_sd_client().await;
    let ns_id = create_namespace_and_get_id(&client).await;

    client
        .create_service()
        .name("blocking-svc")
        .namespace_id(&ns_id)
        .send()
        .await
        .unwrap();

    let result = client.delete_namespace().id(&ns_id).send().await;
    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_servicediscovery_httpnamespaces.py
// ============================================================================

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_create_http_namespace
#[tokio::test]
async fn test_moto_create_http_namespace() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .send()
        .await
        .unwrap();

    let resp = client.list_namespaces().send().await.unwrap();
    let namespaces = resp.namespaces();
    assert_eq!(namespaces.len(), 1);

    let namespace = &namespaces[0];
    let ns_id = namespace.id().unwrap();
    // Verify ns-XXXX format (16 hex chars)
    assert!(
        ns_id.starts_with("ns-"),
        "ID should start with ns-: {ns_id}"
    );

    let arn = namespace.arn().unwrap();
    assert!(
        arn.contains(&format!("namespace/{ns_id}")),
        "ARN should contain namespace ID: {arn}"
    );
    assert_eq!(namespace.name(), Some("mynamespace"));
    assert_eq!(
        namespace.r#type(),
        Some(&aws_sdk_servicediscovery::types::NamespaceType::Http)
    );
    assert!(namespace.create_date().is_some());

    // Check Properties
    let props = namespace.properties().unwrap();
    let dns_props = props.dns_properties().unwrap();
    let soa = dns_props.soa().unwrap();
    // HTTP namespace: DnsProperties has empty SOA
    assert_eq!(soa.ttl(), 0);

    let http_props = props.http_properties().unwrap();
    assert_eq!(http_props.http_name(), Some("mynamespace"));
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_get_http_namespace_minimal
#[tokio::test]
async fn test_moto_get_http_namespace_minimal() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client.get_namespace().id(&ns_id).send().await.unwrap();
    let namespace = resp.namespace().unwrap();

    assert_eq!(namespace.id(), Some(ns_id.as_str()));
    assert!(namespace.arn().is_some());
    assert_eq!(namespace.name(), Some("mynamespace"));
    assert_eq!(
        namespace.r#type(),
        Some(&aws_sdk_servicediscovery::types::NamespaceType::Http)
    );
    assert!(namespace.create_date().is_some());
    assert!(namespace.creator_request_id().is_some());

    let props = namespace.properties().unwrap();
    let dns_props = props.dns_properties().unwrap();
    let soa = dns_props.soa().unwrap();
    assert_eq!(soa.ttl(), 0);
    let http_props = props.http_properties().unwrap();
    assert_eq!(http_props.http_name(), Some("mynamespace"));

    // No description set
    assert!(namespace.description().is_none());
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_get_http_namespace
#[tokio::test]
async fn test_moto_get_http_namespace_full() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .creator_request_id("crid")
        .description("mu fancy namespace")
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client.get_namespace().id(&ns_id).send().await.unwrap();
    let namespace = resp.namespace().unwrap();

    assert_eq!(namespace.name(), Some("mynamespace"));
    assert_eq!(namespace.creator_request_id(), Some("crid"));
    assert_eq!(namespace.description(), Some("mu fancy namespace"));
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_delete_namespace
#[tokio::test]
async fn test_moto_delete_namespace_clears_operations() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let del_resp = client.delete_namespace().id(&ns_id).send().await.unwrap();
    assert!(del_resp.operation_id().is_some());

    assert!(
        client
            .list_namespaces()
            .send()
            .await
            .unwrap()
            .namespaces()
            .is_empty()
    );
    // Operations should be cleared after namespace deletion
    assert!(
        client
            .list_operations()
            .send()
            .await
            .unwrap()
            .operations()
            .is_empty()
    );
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_delete_unknown_namespace
#[tokio::test]
async fn test_moto_delete_unknown_namespace() {
    let client = make_sd_client().await;
    let err = client
        .delete_namespace()
        .id("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NamespaceNotFound"),
        "Expected NamespaceNotFound, got: {err_str}"
    );
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_get_unknown_namespace
#[tokio::test]
async fn test_moto_get_unknown_namespace() {
    let client = make_sd_client().await;
    let err = client
        .get_namespace()
        .id("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NamespaceNotFound"),
        "Expected NamespaceNotFound, got: {err_str}"
    );
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_create_private_dns_namespace_minimal
#[tokio::test]
async fn test_moto_create_private_dns_namespace_minimal() {
    let client = make_sd_client().await;
    client
        .create_private_dns_namespace()
        .name("dns_ns")
        .vpc("vpc_id")
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client.get_namespace().id(&ns_id).send().await.unwrap();
    let namespace = resp.namespace().unwrap();

    assert_eq!(namespace.name(), Some("dns_ns"));
    assert_eq!(
        namespace.r#type(),
        Some(&aws_sdk_servicediscovery::types::NamespaceType::DnsPrivate)
    );

    let props = namespace.properties().unwrap();
    let dns_props = props.dns_properties().unwrap();
    assert!(dns_props.hosted_zone_id().is_some());
    // Minimal: no SOA provided
    assert!(dns_props.soa().is_none());
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_create_private_dns_namespace
#[tokio::test]
async fn test_moto_create_private_dns_namespace_with_soa() {
    let client = make_sd_client().await;
    client
        .create_private_dns_namespace()
        .name("dns_ns")
        .vpc("vpc_id")
        .description("my private dns")
        .properties(
            aws_sdk_servicediscovery::types::PrivateDnsNamespaceProperties::builder()
                .dns_properties(
                    aws_sdk_servicediscovery::types::PrivateDnsPropertiesMutable::builder()
                        .soa(
                            aws_sdk_servicediscovery::types::Soa::builder()
                                .ttl(123)
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client.get_namespace().id(&ns_id).send().await.unwrap();
    let namespace = resp.namespace().unwrap();

    assert_eq!(namespace.name(), Some("dns_ns"));
    assert_eq!(namespace.description(), Some("my private dns"));
    assert_eq!(
        namespace.r#type(),
        Some(&aws_sdk_servicediscovery::types::NamespaceType::DnsPrivate)
    );

    let props = namespace.properties().unwrap();
    let dns_props = props.dns_properties().unwrap();
    assert!(dns_props.hosted_zone_id().is_some());
    assert_eq!(dns_props.soa().unwrap().ttl(), 123);
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_update_private_dns_namespace
#[tokio::test]
async fn test_moto_update_private_dns_namespace_with_soa() {
    let client = make_sd_client().await;
    client
        .create_private_dns_namespace()
        .name("dns_ns")
        .vpc("vpc_id")
        .description("my private dns")
        .properties(
            aws_sdk_servicediscovery::types::PrivateDnsNamespaceProperties::builder()
                .dns_properties(
                    aws_sdk_servicediscovery::types::PrivateDnsPropertiesMutable::builder()
                        .soa(
                            aws_sdk_servicediscovery::types::Soa::builder()
                                .ttl(123)
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .update_private_dns_namespace()
        .id(&ns_id)
        .namespace(
            aws_sdk_servicediscovery::types::PrivateDnsNamespaceChange::builder()
                .description("updated dns")
                .properties(
                    aws_sdk_servicediscovery::types::PrivateDnsNamespacePropertiesChange::builder()
                        .dns_properties(
                            aws_sdk_servicediscovery::types::PrivateDnsPropertiesMutableChange::builder()
                                .soa(
                                    aws_sdk_servicediscovery::types::SoaChange::builder()
                                        .ttl(654)
                                        .build()
                                        .unwrap(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let namespace = client
        .get_namespace()
        .id(&ns_id)
        .send()
        .await
        .unwrap()
        .namespace()
        .unwrap()
        .clone();
    assert_eq!(namespace.description(), Some("updated dns"));
    assert_eq!(
        namespace
            .properties()
            .unwrap()
            .dns_properties()
            .unwrap()
            .soa()
            .unwrap()
            .ttl(),
        654
    );
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_create_private_dns_namespace_with_duplicate_vpc
#[tokio::test]
async fn test_moto_create_private_dns_namespace_with_duplicate_vpc() {
    let client = make_sd_client().await;
    client
        .create_private_dns_namespace()
        .name("dns_ns")
        .vpc("vpc_id")
        .send()
        .await
        .unwrap();

    let err = client
        .create_private_dns_namespace()
        .name("sth else")
        .vpc("vpc_id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictingDomainExists"),
        "Expected ConflictingDomainExists, got: {err_str}"
    );
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_create_public_dns_namespace_minimal
#[tokio::test]
async fn test_moto_create_public_dns_namespace_minimal() {
    let client = make_sd_client().await;
    client
        .create_public_dns_namespace()
        .name("public_dns_ns")
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client.get_namespace().id(&ns_id).send().await.unwrap();
    let namespace = resp.namespace().unwrap();

    assert_eq!(namespace.name(), Some("public_dns_ns"));
    assert_eq!(
        namespace.r#type(),
        Some(&aws_sdk_servicediscovery::types::NamespaceType::DnsPublic)
    );
}

// Ported from moto: test_servicediscovery_httpnamespaces.py::test_update_http_namespace
#[tokio::test]
async fn test_moto_update_http_namespace() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .creator_request_id("crid")
        .description("mu fancy namespace")
        .send()
        .await
        .unwrap();

    let ns_id = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .update_http_namespace()
        .id(&ns_id)
        .namespace(
            aws_sdk_servicediscovery::types::HttpNamespaceChange::builder()
                .description("updated http")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let namespace = client
        .get_namespace()
        .id(&ns_id)
        .send()
        .await
        .unwrap()
        .namespace()
        .unwrap()
        .clone();
    assert_eq!(namespace.description(), Some("updated http"));
}

// ============================================================================
// Ported from moto: test_servicediscovery_service.py
// ============================================================================

async fn create_ns_and_get_id_via_operation(client: &aws_sdk_servicediscovery::Client) -> String {
    let operation_id = client
        .create_http_namespace()
        .name("mynamespace")
        .send()
        .await
        .unwrap()
        .operation_id()
        .unwrap()
        .to_string();

    let op = client
        .get_operation()
        .operation_id(&operation_id)
        .send()
        .await
        .unwrap();
    op.operation()
        .unwrap()
        .targets()
        .unwrap()
        .get(&aws_sdk_servicediscovery::types::OperationTargetType::Namespace)
        .unwrap()
        .clone()
}

// Ported from moto: test_servicediscovery_service.py::test_create_service_minimal
#[tokio::test]
async fn test_moto_create_service_minimal() {
    let client = make_sd_client().await;
    let namespace_id = create_ns_and_get_id_via_operation(&client).await;

    let resp = client
        .create_service()
        .name("my service")
        .namespace_id(&namespace_id)
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert!(svc.id().is_some());
    assert!(svc.arn().is_some());
    assert_eq!(svc.name(), Some("my service"));
    assert_eq!(svc.namespace_id(), Some(namespace_id.as_str()));
    assert!(svc.create_date().is_some());
}

// Ported from moto: test_servicediscovery_service.py::test_create_service
#[tokio::test]
#[allow(deprecated)]
async fn test_moto_create_service_full() {
    let client = make_sd_client().await;
    let namespace_id = create_ns_and_get_id_via_operation(&client).await;

    let resp = client
        .create_service()
        .name("my service")
        .creator_request_id("crid")
        .description("my service")
        .dns_config(
            aws_sdk_servicediscovery::types::DnsConfig::builder()
                .namespace_id(&namespace_id)
                .routing_policy(aws_sdk_servicediscovery::types::RoutingPolicy::Weighted)
                .dns_records(
                    aws_sdk_servicediscovery::types::DnsRecord::builder()
                        .r#type(aws_sdk_servicediscovery::types::RecordType::Srv)
                        .ttl(0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .health_check_config(
            aws_sdk_servicediscovery::types::HealthCheckConfig::builder()
                .r#type(aws_sdk_servicediscovery::types::HealthCheckType::Tcp)
                .resource_path("/sth")
                .build()
                .unwrap(),
        )
        .health_check_custom_config(
            aws_sdk_servicediscovery::types::HealthCheckCustomConfig::builder()
                .failure_threshold(125)
                .build(),
        )
        .r#type(aws_sdk_servicediscovery::types::ServiceTypeOption::Http)
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert!(svc.id().is_some());
    assert!(svc.arn().is_some());
    assert_eq!(svc.name(), Some("my service"));
    // When DnsConfig is provided, NamespaceId is NOT in the response (per moto)
    assert!(svc.namespace_id().is_none());
    assert_eq!(svc.description(), Some("my service"));

    // DnsConfig
    let dc = svc.dns_config().unwrap();
    assert_eq!(dc.namespace_id(), Some(namespace_id.as_str()));
    assert_eq!(
        dc.routing_policy(),
        Some(&aws_sdk_servicediscovery::types::RoutingPolicy::Weighted)
    );
    assert_eq!(dc.dns_records().len(), 1);
    assert_eq!(
        dc.dns_records()[0].r#type(),
        &aws_sdk_servicediscovery::types::RecordType::Srv
    );
    assert_eq!(dc.dns_records()[0].ttl(), 0);

    // HealthCheckConfig
    let hcc = svc.health_check_config().unwrap();
    assert_eq!(
        hcc.r#type(),
        &aws_sdk_servicediscovery::types::HealthCheckType::Tcp
    );
    assert_eq!(hcc.resource_path(), Some("/sth"));

    // HealthCheckCustomConfig
    let hccc = svc.health_check_custom_config().unwrap();
    assert_eq!(hccc.failure_threshold(), Some(125));

    // Type
    assert_eq!(
        svc.r#type(),
        Some(&aws_sdk_servicediscovery::types::ServiceType::Http)
    );
    assert_eq!(svc.creator_request_id(), Some("crid"));
}

// Ported from moto: test_servicediscovery_service.py::test_get_unknown_service
#[tokio::test]
async fn test_moto_get_unknown_service() {
    let client = make_sd_client().await;
    let err = client.get_service().id("unknown").send().await.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ServiceNotFound"),
        "Expected ServiceNotFound, got: {err_str}"
    );
}

// Ported from moto: test_servicediscovery_service.py::test_delete_service
#[tokio::test]
async fn test_moto_delete_service() {
    let client = make_sd_client().await;
    let namespace_id = create_ns_and_get_id_via_operation(&client).await;

    let service_id = client
        .create_service()
        .name("my service")
        .namespace_id(&namespace_id)
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_service()
        .id(&service_id)
        .send()
        .await
        .unwrap();

    let err = client
        .get_service()
        .id(&service_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ServiceNotFound"),
        "Expected ServiceNotFound, got: {err_str}"
    );
}

// Ported from moto: test_servicediscovery_service.py::test_update_service_description
#[tokio::test]
#[allow(deprecated)]
async fn test_moto_update_service_description() {
    let client = make_sd_client().await;
    let namespace_id = create_ns_and_get_id_via_operation(&client).await;

    let service_id = client
        .create_service()
        .name("my service")
        .namespace_id(&namespace_id)
        .description("first desc")
        .dns_config(
            aws_sdk_servicediscovery::types::DnsConfig::builder()
                .namespace_id(&namespace_id)
                .routing_policy(aws_sdk_servicediscovery::types::RoutingPolicy::Weighted)
                .dns_records(
                    aws_sdk_servicediscovery::types::DnsRecord::builder()
                        .r#type(aws_sdk_servicediscovery::types::RecordType::Srv)
                        .ttl(0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .health_check_config(
            aws_sdk_servicediscovery::types::HealthCheckConfig::builder()
                .r#type(aws_sdk_servicediscovery::types::HealthCheckType::Tcp)
                .resource_path("/sth")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .update_service()
        .id(&service_id)
        .service(
            aws_sdk_servicediscovery::types::ServiceChange::builder()
                .description("updated desc")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let svc = client
        .get_service()
        .id(&service_id)
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .clone();

    assert_eq!(svc.id(), Some(service_id.as_str()));
    assert_eq!(svc.name(), Some("my service"));
    assert_eq!(svc.namespace_id(), Some(namespace_id.as_str()));
    assert_eq!(svc.description(), Some("updated desc"));
    // DnsConfig should be removed when not included in update
    assert!(svc.dns_config().is_none());
    // HealthCheckConfig is preserved
    let hcc = svc.health_check_config().unwrap();
    assert_eq!(
        hcc.r#type(),
        &aws_sdk_servicediscovery::types::HealthCheckType::Tcp
    );
    assert_eq!(hcc.resource_path(), Some("/sth"));
}

// Ported from moto: test_servicediscovery_service.py::test_update_service_others
#[tokio::test]
async fn test_moto_update_service_others() {
    let client = make_sd_client().await;
    let namespace_id = create_ns_and_get_id_via_operation(&client).await;

    let service_id = client
        .create_service()
        .name("my service")
        .namespace_id(&namespace_id)
        .description("first desc")
        .dns_config(
            aws_sdk_servicediscovery::types::DnsConfig::builder()
                .routing_policy(aws_sdk_servicediscovery::types::RoutingPolicy::Weighted)
                .dns_records(
                    aws_sdk_servicediscovery::types::DnsRecord::builder()
                        .r#type(aws_sdk_servicediscovery::types::RecordType::Srv)
                        .ttl(0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .update_service()
        .id(&service_id)
        .service(
            aws_sdk_servicediscovery::types::ServiceChange::builder()
                .dns_config(
                    aws_sdk_servicediscovery::types::DnsConfigChange::builder()
                        .dns_records(
                            aws_sdk_servicediscovery::types::DnsRecord::builder()
                                .r#type(aws_sdk_servicediscovery::types::RecordType::Srv)
                                .ttl(12)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .health_check_config(
                    aws_sdk_servicediscovery::types::HealthCheckConfig::builder()
                        .r#type(aws_sdk_servicediscovery::types::HealthCheckType::Tcp)
                        .resource_path("/sth")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let svc = client
        .get_service()
        .id(&service_id)
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .clone();

    assert_eq!(svc.name(), Some("my service"));
    assert_eq!(svc.namespace_id(), Some(namespace_id.as_str()));
    assert_eq!(svc.description(), Some("first desc"));

    // DnsConfig updated - routing policy preserved, records updated
    let dc = svc.dns_config().unwrap();
    assert_eq!(
        dc.routing_policy(),
        Some(&aws_sdk_servicediscovery::types::RoutingPolicy::Weighted)
    );
    assert_eq!(dc.dns_records().len(), 1);
    assert_eq!(dc.dns_records()[0].ttl(), 12);

    // HealthCheckConfig added
    let hcc = svc.health_check_config().unwrap();
    assert_eq!(
        hcc.r#type(),
        &aws_sdk_servicediscovery::types::HealthCheckType::Tcp
    );
    assert_eq!(hcc.resource_path(), Some("/sth"));
}

// ============================================================================
// Ported from moto: test_servicediscovery_operations.py
// ============================================================================

// Ported from moto: test_servicediscovery_operations.py::test_list_operations_initial
#[tokio::test]
async fn test_moto_list_operations_initial() {
    let client = make_sd_client().await;
    let resp = client.list_operations().send().await.unwrap();
    assert!(resp.operations().is_empty());
}

// Ported from moto: test_servicediscovery_operations.py::test_list_operations
#[tokio::test]
async fn test_moto_list_operations_after_create() {
    let client = make_sd_client().await;
    let op_id = client
        .create_http_namespace()
        .name("n/a")
        .send()
        .await
        .unwrap()
        .operation_id()
        .unwrap()
        .to_string();

    let resp = client.list_operations().send().await.unwrap();
    assert_eq!(resp.operations().len(), 1);
    let op = &resp.operations()[0];
    assert_eq!(op.id(), Some(op_id.as_str()));
    assert_eq!(
        op.status(),
        Some(&aws_sdk_servicediscovery::types::OperationStatus::Success)
    );
}

// Ported from moto: test_servicediscovery_operations.py::test_get_create_http_namespace_operation
#[tokio::test]
async fn test_moto_get_create_http_namespace_operation() {
    let client = make_sd_client().await;
    let op_id = client
        .create_http_namespace()
        .name("mynamespace")
        .send()
        .await
        .unwrap()
        .operation_id()
        .unwrap()
        .to_string();

    let resp = client
        .get_operation()
        .operation_id(&op_id)
        .send()
        .await
        .unwrap();
    let operation = resp.operation().unwrap();

    assert_eq!(operation.id(), Some(op_id.as_str()));
    assert_eq!(
        operation.r#type(),
        Some(&aws_sdk_servicediscovery::types::OperationType::CreateNamespace)
    );
    assert_eq!(
        operation.status(),
        Some(&aws_sdk_servicediscovery::types::OperationStatus::Success)
    );
    assert!(operation.create_date().is_some());
    assert!(operation.update_date().is_some());

    let targets = operation.targets().unwrap();
    assert!(targets.contains_key(&aws_sdk_servicediscovery::types::OperationTargetType::Namespace));

    // Verify the target namespace exists
    let ns_id = targets
        .get(&aws_sdk_servicediscovery::types::OperationTargetType::Namespace)
        .unwrap();
    let namespaces = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .iter()
        .map(|ns| ns.id().unwrap().to_string())
        .collect::<Vec<_>>();
    assert!(namespaces.contains(ns_id));
}

// Ported from moto: test_servicediscovery_operations.py::test_get_update_service_operation
#[tokio::test]
async fn test_moto_get_update_service_operation() {
    let client = make_sd_client().await;

    let service_id = client
        .create_service()
        .name("my service")
        .description("first desc")
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .update_service()
        .id(&service_id)
        .service(
            aws_sdk_servicediscovery::types::ServiceChange::builder()
                .description("updated desc")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let op_id = resp.operation_id().unwrap().to_string();

    let op_resp = client
        .get_operation()
        .operation_id(&op_id)
        .send()
        .await
        .unwrap();
    let operation = op_resp.operation().unwrap();

    assert_eq!(operation.id(), Some(op_id.as_str()));
    assert_eq!(
        operation.r#type(),
        Some(&aws_sdk_servicediscovery::types::OperationType::UpdateService)
    );
    assert_eq!(
        operation.status(),
        Some(&aws_sdk_servicediscovery::types::OperationStatus::Success)
    );
    assert!(operation.create_date().is_some());
    assert!(operation.update_date().is_some());
    assert!(operation.targets().is_some());
}

// Ported from moto: test_servicediscovery_operations.py::test_get_unknown_operation
#[tokio::test]
async fn test_moto_get_unknown_operation() {
    let client = make_sd_client().await;
    let err = client
        .get_operation()
        .operation_id("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("OperationNotFound"),
        "Expected OperationNotFound, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_servicediscovery_instance.py
// ============================================================================

async fn create_moto_test_fixtures(client: &aws_sdk_servicediscovery::Client) -> (String, String) {
    client
        .create_http_namespace()
        .name("mynamespace")
        .send()
        .await
        .unwrap();

    let namespaces = client.list_namespaces().send().await.unwrap();
    let ns = namespaces
        .namespaces()
        .iter()
        .find(|ns| ns.name() == Some("mynamespace"))
        .unwrap();
    let ns_id = ns.id().unwrap().to_string();

    let srv_resp = client
        .create_service()
        .name("myservice")
        .namespace_id(&ns_id)
        .dns_config(
            aws_sdk_servicediscovery::types::DnsConfig::builder()
                .dns_records(
                    aws_sdk_servicediscovery::types::DnsRecord::builder()
                        .r#type(aws_sdk_servicediscovery::types::RecordType::A)
                        .ttl(60)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let svc_id = srv_resp.service().unwrap().id().unwrap().to_string();
    (ns_id, svc_id)
}

// Ported from moto: test_servicediscovery_instance.py::test_register_instance
#[tokio::test]
async fn test_moto_register_instance() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let inst_resp = client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .creator_request_id("crid")
        .attributes("attr1", "value1")
        .send()
        .await
        .unwrap();

    assert!(inst_resp.operation_id().is_some());

    // Verify operation targets INSTANCE with instance_id
    let operation = client
        .get_operation()
        .operation_id(inst_resp.operation_id().unwrap())
        .send()
        .await
        .unwrap();
    let targets = operation.operation().unwrap().targets().unwrap();
    assert_eq!(
        targets.get(&aws_sdk_servicediscovery::types::OperationTargetType::Instance),
        Some(&"i-123".to_string())
    );

    // Verify instance data
    let instance = client
        .get_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .send()
        .await
        .unwrap();
    let inst = instance.instance().unwrap();
    assert_eq!(inst.creator_request_id(), Some("crid"));
    assert_eq!(
        inst.attributes().unwrap().get("attr1"),
        Some(&"value1".to_string())
    );
    assert_eq!(inst.id(), "i-123");
}

// Ported from moto: test_servicediscovery_instance.py::test_deregister_instance
#[tokio::test]
async fn test_moto_deregister_instance() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .creator_request_id("crid")
        .attributes("attr1", "value1")
        .send()
        .await
        .unwrap();

    let dereg_resp = client
        .deregister_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .send()
        .await
        .unwrap();
    assert!(dereg_resp.operation_id().is_some());

    // Verify operation targets INSTANCE with instance_id
    let operation = client
        .get_operation()
        .operation_id(dereg_resp.operation_id().unwrap())
        .send()
        .await
        .unwrap();
    let targets = operation.operation().unwrap().targets().unwrap();
    assert_eq!(
        targets.get(&aws_sdk_servicediscovery::types::OperationTargetType::Instance),
        Some(&"i-123".to_string())
    );

    // Verify instance is gone
    let err = client
        .get_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InstanceNotFound"),
        "Expected InstanceNotFound, got: {err_str}"
    );
}

// Ported from moto: test_servicediscovery_instance.py::test_get_unknown_instance
#[tokio::test]
async fn test_moto_get_unknown_instance() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let err = client
        .get_instance()
        .service_id(&svc_id)
        .instance_id("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InstanceNotFound"),
        "Expected InstanceNotFound, got: {err_str}"
    );
}

// Ported from moto: test_servicediscovery_instance.py::test_list_instances
#[tokio::test]
async fn test_moto_list_instances() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let instance_ids = ["i-123", "i-456", "i-789", "i-012"];
    for instance_id in &instance_ids {
        client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(*instance_id)
            .send()
            .await
            .unwrap();
    }

    let instances = client
        .list_instances()
        .service_id(&svc_id)
        .send()
        .await
        .unwrap();
    assert_eq!(instances.instances().len(), 4);

    let ids: std::collections::HashSet<String> = instances
        .instances()
        .iter()
        .map(|i| i.id().unwrap().to_string())
        .collect();
    for id in &instance_ids {
        assert!(ids.contains(*id), "Missing instance {id}");
    }
}

// Ported from moto: test_servicediscovery_instance.py::test_paginate_list_instances
#[tokio::test]
async fn test_moto_paginate_list_instances() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let instance_ids = ["i-012", "i-123", "i-456", "i-789"];
    for instance_id in &instance_ids {
        client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(*instance_id)
            .send()
            .await
            .unwrap();
    }

    let page1 = client
        .list_instances()
        .service_id(&svc_id)
        .max_results(2)
        .send()
        .await
        .unwrap();
    assert_eq!(page1.instances().len(), 2);
    assert!(page1.next_token().is_some());

    let page2 = client
        .list_instances()
        .service_id(&svc_id)
        .next_token(page1.next_token().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(page2.instances().len(), 2);
    assert!(page2.next_token().is_none());

    // All 4 instances returned across both pages
    let mut all_ids: Vec<String> = page1
        .instances()
        .iter()
        .chain(page2.instances().iter())
        .map(|i| i.id().unwrap().to_string())
        .collect();
    all_ids.sort();
    let mut expected: Vec<String> = instance_ids.iter().map(|s| s.to_string()).collect();
    expected.sort();
    assert_eq!(all_ids, expected);
}

// Ported from moto: test_servicediscovery_instance.py::test_get_instances_health_status
#[tokio::test]
async fn test_moto_get_instances_health_status() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let instance_ids = ["i-123", "i-456", "i-789", "i-012"];
    for instance_id in &instance_ids {
        client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(*instance_id)
            .send()
            .await
            .unwrap();
    }

    let health = client
        .get_instances_health_status()
        .service_id(&svc_id)
        .send()
        .await
        .unwrap();
    let status = health.status().unwrap();
    assert_eq!(status.len(), 4);
    for id in &instance_ids {
        assert_eq!(
            status.get(*id).map(|s| s.as_str()),
            Some("HEALTHY"),
            "Instance {id} should be HEALTHY"
        );
    }
}

// Ported from moto: test_servicediscovery_instance.py::test_update_instance_custom_health_status
#[tokio::test]
async fn test_moto_update_instance_custom_health_status() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .send()
        .await
        .unwrap();

    client
        .update_instance_custom_health_status()
        .service_id(&svc_id)
        .instance_id("i-123")
        .status(aws_sdk_servicediscovery::types::CustomHealthStatus::Unhealthy)
        .send()
        .await
        .unwrap();

    let health = client
        .get_instances_health_status()
        .service_id(&svc_id)
        .send()
        .await
        .unwrap();
    let status = health.status().unwrap();
    assert_eq!(status.get("i-123").map(|s| s.as_str()), Some("UNHEALTHY"));
}

// Ported from moto: test_servicediscovery_instance.py::test_discover_instances_formatting
#[tokio::test]
async fn test_moto_discover_instances_formatting() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .attributes("attr1", "value1")
        .attributes("attr2", "value1")
        .send()
        .await
        .unwrap();

    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .max_results(2)
        .send()
        .await
        .unwrap();

    let instances = resp.instances();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].instance_id(), Some("i-123"));
    assert_eq!(instances[0].namespace_name(), Some("mynamespace"));
    assert_eq!(instances[0].service_name(), Some("myservice"));

    let attrs = instances[0].attributes().unwrap();
    assert_eq!(attrs.get("attr1"), Some(&"value1".to_string()));
    assert_eq!(attrs.get("attr2"), Some(&"value1".to_string()));

    assert_eq!(
        instances[0].health_status(),
        Some(&aws_sdk_servicediscovery::types::HealthStatus::Healthy)
    );

    assert_eq!(resp.instances_revision(), Some(1));
}

// Ported from moto: test_servicediscovery_instance.py::test_discover_instances_attr_filters
#[tokio::test]
async fn test_moto_discover_instances_attr_filters() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let instance_dicts = [
        ("i-123", vec![("attr1", "value1"), ("attr2", "value1")]),
        ("i-456", vec![("attr1", "value2")]),
        ("i-789", vec![("attr1", "value3"), ("attr2", "value1")]),
        ("i-012", vec![("attr1", "value3")]),
    ];

    for (id, attrs) in &instance_dicts {
        let mut req = client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(*id);
        for (k, v) in attrs {
            req = req.attributes(*k, *v);
        }
        req.send().await.unwrap();
    }

    // No filters - all 4
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 4);

    // QueryParameters: attr1=value3 -> 2 instances
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .query_parameters("attr1", "value3")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 2);
    let ids: std::collections::HashSet<String> = resp
        .instances()
        .iter()
        .filter_map(|i| i.instance_id().map(|s| s.to_string()))
        .collect();
    assert!(ids.contains("i-789") && ids.contains("i-012"));

    // QueryParameters + OptionalParameters: attr1=value3, attr2=value1 -> 1 instance
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .query_parameters("attr1", "value3")
        .optional_parameters("attr2", "value1")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 1);
    assert_eq!(resp.instances()[0].instance_id(), Some("i-789"));

    // QueryParameters + OptionalParameters where optional doesn't match anyone
    // -> returns all query-matched (2)
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .query_parameters("attr1", "value3")
        .optional_parameters("attr2", "value2")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 2);
}

// Ported from moto: test_servicediscovery_instance.py::test_discover_instances_health_filters
#[tokio::test]
async fn test_moto_discover_instances_health_filters() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let instance_dicts = [
        ("i-123", false),
        ("i-456", false),
        ("i-789", true),
        ("i-012", true),
    ];

    for (id, unhealthy) in &instance_dicts {
        client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(*id)
            .send()
            .await
            .unwrap();
        if *unhealthy {
            client
                .update_instance_custom_health_status()
                .service_id(&svc_id)
                .instance_id(*id)
                .status(aws_sdk_servicediscovery::types::CustomHealthStatus::Unhealthy)
                .send()
                .await
                .unwrap();
        }
    }

    // ALL -> 4
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .health_status(aws_sdk_servicediscovery::types::HealthStatusFilter::All)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 4);

    // UNHEALTHY -> 2
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .health_status(aws_sdk_servicediscovery::types::HealthStatusFilter::Unhealthy)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 2);
    let ids: std::collections::HashSet<String> = resp
        .instances()
        .iter()
        .filter_map(|i| i.instance_id().map(|s| s.to_string()))
        .collect();
    assert!(ids.contains("i-789") && ids.contains("i-012"));

    // HEALTHY -> 2
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .health_status(aws_sdk_servicediscovery::types::HealthStatusFilter::Healthy)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 2);
    let ids: std::collections::HashSet<String> = resp
        .instances()
        .iter()
        .filter_map(|i| i.instance_id().map(|s| s.to_string()))
        .collect();
    assert!(ids.contains("i-123") && ids.contains("i-456"));

    // HEALTHY_OR_ELSE_ALL -> 2 (healthy instances exist)
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .health_status(aws_sdk_servicediscovery::types::HealthStatusFilter::HealthyOrElseAll)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 2);
    let ids: std::collections::HashSet<String> = resp
        .instances()
        .iter()
        .filter_map(|i| i.instance_id().map(|s| s.to_string()))
        .collect();
    assert!(ids.contains("i-123") && ids.contains("i-456"));

    // Now make all instances unhealthy
    client
        .update_instance_custom_health_status()
        .service_id(&svc_id)
        .instance_id("i-123")
        .status(aws_sdk_servicediscovery::types::CustomHealthStatus::Unhealthy)
        .send()
        .await
        .unwrap();
    client
        .update_instance_custom_health_status()
        .service_id(&svc_id)
        .instance_id("i-456")
        .status(aws_sdk_servicediscovery::types::CustomHealthStatus::Unhealthy)
        .send()
        .await
        .unwrap();

    // HEALTHY_OR_ELSE_ALL -> 4 (no healthy, so return all)
    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .health_status(aws_sdk_servicediscovery::types::HealthStatusFilter::HealthyOrElseAll)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 4);
}

// Ported from moto: test_servicediscovery_instance.py::test_max_results_discover_instances
#[tokio::test]
async fn test_moto_max_results_discover_instances() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let instance_ids = ["i-123", "i-456", "i-789", "i-012"];
    for instance_id in &instance_ids {
        client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(*instance_id)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .discover_instances()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .max_results(2)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.instances().len(), 2);
}

// Ported from moto: test_servicediscovery_instance.py::test_discover_instances_revision
#[tokio::test]
async fn test_moto_discover_instances_revision() {
    let client = make_sd_client().await;
    let (_ns_id, svc_id) = create_moto_test_fixtures(&client).await;

    let instance_ids = ["i-123", "i-456", "i-789", "i-012"];
    for instance_id in &instance_ids {
        client
            .register_instance()
            .service_id(&svc_id)
            .instance_id(*instance_id)
            .send()
            .await
            .unwrap();
    }

    let revisions = client
        .discover_instances_revision()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .send()
        .await
        .unwrap();
    assert_eq!(revisions.instances_revision(), Some(4));

    // Deregister and re-register to increase revision
    client
        .deregister_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .send()
        .await
        .unwrap();
    client
        .register_instance()
        .service_id(&svc_id)
        .instance_id("i-123")
        .send()
        .await
        .unwrap();

    let revisions = client
        .discover_instances_revision()
        .namespace_name("mynamespace")
        .service_name("myservice")
        .send()
        .await
        .unwrap();
    assert_eq!(revisions.instances_revision(), Some(6));
}

// ============================================================================
// Ported from moto: test_servicediscovery_tags.py
// ============================================================================

// Ported from moto: test_servicediscovery_tags.py::test_create_http_namespace_with_tags
#[tokio::test]
async fn test_moto_create_http_namespace_with_tags() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let ns_arn = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&ns_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "key1");
    assert_eq!(tags[0].value(), "val1");
}

// Ported from moto: test_servicediscovery_tags.py::test_create_public_dns_namespace_with_tags
#[tokio::test]
async fn test_moto_create_public_dns_namespace_with_tags() {
    let client = make_sd_client().await;
    client
        .create_public_dns_namespace()
        .name("mynamespace")
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let ns_arn = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&ns_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "key1");
    assert_eq!(tags[0].value(), "val1");
}

// Ported from moto: test_servicediscovery_tags.py::test_create_private_dns_namespace_with_tags
#[tokio::test]
async fn test_moto_create_private_dns_namespace_with_tags() {
    let client = make_sd_client().await;
    client
        .create_private_dns_namespace()
        .name("mynamespace")
        .vpc("vpc")
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let ns_arn = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&ns_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "key1");
    assert_eq!(tags[0].value(), "val1");
}

// Ported from moto: test_servicediscovery_tags.py::test_create_service_with_tags
#[tokio::test]
async fn test_moto_create_service_with_tags() {
    let client = make_sd_client().await;
    client
        .create_service()
        .name("myservice")
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let svc_arn = client
        .list_services()
        .send()
        .await
        .unwrap()
        .services()
        .first()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&svc_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "key1");
    assert_eq!(tags[0].value(), "val1");
}

// Ported from moto: test_servicediscovery_tags.py::test_tag_resource
#[tokio::test]
async fn test_moto_tag_resource() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let ns_arn = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&ns_arn)
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key2")
                .value("val2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&ns_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
}

// Ported from moto: test_servicediscovery_tags.py::test_untag_resource
#[tokio::test]
async fn test_moto_untag_resource() {
    let client = make_sd_client().await;
    client
        .create_http_namespace()
        .name("mynamespace")
        .send()
        .await
        .unwrap();

    let ns_arn = client
        .list_namespaces()
        .send()
        .await
        .unwrap()
        .namespaces()
        .first()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&ns_arn)
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_servicediscovery::types::Tag::builder()
                .key("key2")
                .value("val2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&ns_arn)
        .tag_keys("key1")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&ns_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "key2");
    assert_eq!(tags[0].value(), "val2");
}
