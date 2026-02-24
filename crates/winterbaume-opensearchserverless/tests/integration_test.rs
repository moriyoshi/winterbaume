use aws_sdk_opensearchserverless::config::BehaviorVersion;
use aws_sdk_opensearchserverless::types::{CollectionType, SecurityPolicyType};
use winterbaume_core::MockAws;
use winterbaume_opensearchserverless::OpenSearchServerlessService;

async fn make_client() -> aws_sdk_opensearchserverless::Client {
    let mock = MockAws::builder()
        .with_service(OpenSearchServerlessService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_opensearchserverless::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_opensearchserverless::Client::new(&config)
}

// ---- Collection tests ----

#[tokio::test]
async fn test_create_collection() {
    let client = make_client().await;

    let resp = client
        .create_collection()
        .name("my-collection")
        .r#type(CollectionType::Search)
        .description("Test collection")
        .send()
        .await
        .expect("create_collection should succeed");

    let detail = resp.create_collection_detail().expect("should have detail");
    assert_eq!(detail.name(), Some("my-collection"));
    assert_eq!(detail.status().map(|s| s.as_str()), Some("ACTIVE"));
    assert!(detail.id().is_some());
    assert!(detail.arn().is_some());
}

#[tokio::test]
async fn test_list_collections() {
    let client = make_client().await;

    for name in ["col-a", "col-b", "col-c"] {
        client.create_collection().name(name).send().await.unwrap();
    }

    let resp = client
        .list_collections()
        .send()
        .await
        .expect("list_collections should succeed");

    assert_eq!(resp.collection_summaries().len(), 3);
}

#[tokio::test]
async fn test_delete_collection() {
    let client = make_client().await;

    let create_resp = client
        .create_collection()
        .name("to-delete")
        .send()
        .await
        .unwrap();
    let id = create_resp
        .create_collection_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_collection()
        .id(&id)
        .send()
        .await
        .expect("delete_collection should succeed");

    // List should be empty now
    let list_resp = client.list_collections().send().await.unwrap();
    assert_eq!(list_resp.collection_summaries().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_collection() {
    let client = make_client().await;

    let result = client.delete_collection().id("nonexistent").send().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_batch_get_collection_by_ids() {
    let client = make_client().await;

    let resp1 = client
        .create_collection()
        .name("batch-col-1")
        .send()
        .await
        .unwrap();
    let id1 = resp1
        .create_collection_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp2 = client
        .create_collection()
        .name("batch-col-2")
        .send()
        .await
        .unwrap();
    let id2 = resp2
        .create_collection_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let batch_resp = client
        .batch_get_collection()
        .ids(&id1)
        .ids(&id2)
        .send()
        .await
        .expect("batch_get_collection should succeed");

    assert_eq!(batch_resp.collection_details().len(), 2);
}

#[tokio::test]
async fn test_batch_get_collection_by_names() {
    let client = make_client().await;

    client
        .create_collection()
        .name("named-col-1")
        .send()
        .await
        .unwrap();
    client
        .create_collection()
        .name("named-col-2")
        .send()
        .await
        .unwrap();

    let batch_resp = client
        .batch_get_collection()
        .names("named-col-1")
        .names("named-col-2")
        .send()
        .await
        .expect("batch_get_collection by names should succeed");

    assert_eq!(batch_resp.collection_details().len(), 2);
}

// ---- Collection lifecycle test ----

#[tokio::test]
async fn test_collection_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_collection()
        .name("lifecycle-col")
        .r#type(CollectionType::Vectorsearch)
        .description("Lifecycle test")
        .send()
        .await
        .expect("create_collection should succeed");

    let id = create_resp
        .create_collection_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // List - should contain our collection
    let list_resp = client.list_collections().send().await.unwrap();
    assert!(
        list_resp
            .collection_summaries()
            .iter()
            .any(|c| c.id() == Some(id.as_str()))
    );

    // BatchGet by id
    let batch_resp = client.batch_get_collection().ids(&id).send().await.unwrap();
    assert_eq!(batch_resp.collection_details().len(), 1);
    assert_eq!(batch_resp.collection_details()[0].id(), Some(id.as_str()));

    // Delete
    client
        .delete_collection()
        .id(&id)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let list_after = client.list_collections().send().await.unwrap();
    assert!(
        !list_after
            .collection_summaries()
            .iter()
            .any(|c| c.id() == Some(id.as_str()))
    );
}

// ---- SecurityPolicy tests ----

#[tokio::test]
async fn test_create_security_policy() {
    let client = make_client().await;

    let resp = client
        .create_security_policy()
        .name("my-enc-policy")
        .r#type(SecurityPolicyType::Encryption)
        .policy(
            r#"{"Rules":[{"ResourceType":"collection","Resource":["collection/*"]}],"AWSOwnedKey":true}"#,
        )
        .description("Test encryption policy")
        .send()
        .await
        .expect("create_security_policy should succeed");

    let detail = resp.security_policy_detail().expect("should have detail");
    assert_eq!(detail.name(), Some("my-enc-policy"));
    assert_eq!(detail.r#type().map(|t| t.as_str()), Some("encryption"));
    assert!(detail.policy_version().is_some());
}

#[tokio::test]
async fn test_get_security_policy() {
    let client = make_client().await;

    client
        .create_security_policy()
        .name("get-test-policy")
        .r#type(SecurityPolicyType::Network)
        .policy(
            r#"[{"Rules":[{"ResourceType":"collection","Resource":["collection/get-test"]}],"AllowFromPublic":true}]"#,
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_security_policy()
        .name("get-test-policy")
        .r#type(SecurityPolicyType::Network)
        .send()
        .await
        .expect("get_security_policy should succeed");

    let detail = resp.security_policy_detail().expect("should have detail");
    assert_eq!(detail.name(), Some("get-test-policy"));
}

#[tokio::test]
async fn test_get_nonexistent_security_policy() {
    let client = make_client().await;

    let result = client
        .get_security_policy()
        .name("nonexistent")
        .r#type(SecurityPolicyType::Encryption)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_security_policy() {
    let client = make_client().await;

    let create_resp = client
        .create_security_policy()
        .name("upd-policy")
        .r#type(SecurityPolicyType::Encryption)
        .policy(r#"{"AWSOwnedKey":true}"#)
        .send()
        .await
        .unwrap();
    let version = create_resp
        .security_policy_detail()
        .unwrap()
        .policy_version()
        .unwrap()
        .to_string();

    let upd_resp = client
        .update_security_policy()
        .name("upd-policy")
        .r#type(SecurityPolicyType::Encryption)
        .policy_version(&version)
        .description("Updated description")
        .send()
        .await
        .expect("update_security_policy should succeed");

    let detail = upd_resp
        .security_policy_detail()
        .expect("should have detail");
    assert_eq!(detail.description(), Some("Updated description"));
    // Version should be bumped
    assert_ne!(detail.policy_version(), Some(version.as_str()));
}

#[tokio::test]
async fn test_list_security_policies() {
    let client = make_client().await;

    for i in 0..3 {
        client
            .create_security_policy()
            .name(format!("list-enc-{i}"))
            .r#type(SecurityPolicyType::Encryption)
            .policy(r#"{"AWSOwnedKey":true}"#)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_security_policies()
        .r#type(SecurityPolicyType::Encryption)
        .send()
        .await
        .expect("list_security_policies should succeed");

    assert_eq!(resp.security_policy_summaries().len(), 3);
}

// ---- VpcEndpoint tests ----

#[tokio::test]
async fn test_create_vpc_endpoint() {
    let client = make_client().await;

    let resp = client
        .create_vpc_endpoint()
        .name("my-vpc-ep")
        .vpc_id("vpc-12345678")
        .subnet_ids("subnet-aabbccdd")
        .send()
        .await
        .expect("create_vpc_endpoint should succeed");

    let detail = resp
        .create_vpc_endpoint_detail()
        .expect("should have detail");
    assert_eq!(detail.name(), Some("my-vpc-ep"));
    assert_eq!(detail.status().map(|s| s.as_str()), Some("ACTIVE"));
    assert!(detail.id().is_some());
}

// ---- Tag tests ----

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;

    // Create a collection to tag
    let create_resp = client
        .create_collection()
        .name("tag-col")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .create_collection_detail()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    // Tag it
    let tag = aws_sdk_opensearchserverless::types::Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(tag)
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert_eq!(list_resp.tags().len(), 1);
    assert_eq!(list_resp.tags()[0].key(), "env");
    assert_eq!(list_resp.tags()[0].value(), "test");

    // Untag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tags removed
    let list_after = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(list_after.tags().len(), 0);
}

#[tokio::test]
async fn test_tag_nonexistent_resource() {
    let client = make_client().await;

    let tag = aws_sdk_opensearchserverless::types::Tag::builder()
        .key("key")
        .value("val")
        .build()
        .unwrap();
    let result = client
        .tag_resource()
        .resource_arn("arn:aws:aoss:us-east-1:123456789012:collection/nonexistent")
        .tags(tag)
        .send()
        .await;

    assert!(result.is_err());
}

// ---- State view tests ----

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use winterbaume_core::StatefulService;
    use winterbaume_opensearchserverless::OpenSearchServerlessStateView;
    use winterbaume_opensearchserverless::views::CollectionView;

    let svc = OpenSearchServerlessService::new();

    // Seed via restore
    let mut view = OpenSearchServerlessStateView::default();
    view.collections.insert(
        "abc123".to_string(),
        CollectionView {
            id: "abc123".to_string(),
            name: "snap-col".to_string(),
            arn: "arn:aws:aoss:us-east-1:123456789012:collection/abc123".to_string(),
            type_: "SEARCH".to_string(),
            status: "ACTIVE".to_string(),
            description: None,
            kms_key_arn: None,
            created_date: 0,
            last_modified_date: 0,
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Snapshot should contain the collection
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.collections.len(), 1);
    assert!(snap.collections.contains_key("abc123"));

    // Restore to empty
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();

    // Verify empty
    let snap2 = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap2.collections.len(), 0);
}

#[tokio::test]
async fn test_state_view_merge() {
    use winterbaume_core::StatefulService;
    use winterbaume_opensearchserverless::OpenSearchServerlessStateView;
    use winterbaume_opensearchserverless::views::CollectionView;

    let svc = OpenSearchServerlessService::new();

    let mut view = OpenSearchServerlessStateView::default();
    view.collections.insert(
        "abc123".to_string(),
        CollectionView {
            id: "abc123".to_string(),
            name: "merged-col".to_string(),
            arn: "arn:aws:aoss:us-east-1:123456789012:collection/abc123".to_string(),
            type_: "SEARCH".to_string(),
            status: "ACTIVE".to_string(),
            description: None,
            kms_key_arn: None,
            created_date: 0,
            last_modified_date: 0,
        },
    );
    svc.merge("123456789012", "us-east-1", view).await.unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.collections.contains_key("abc123"));

    // Merge again with a different collection - should accumulate
    let mut view2 = OpenSearchServerlessStateView::default();
    view2.collections.insert(
        "def456".to_string(),
        CollectionView {
            id: "def456".to_string(),
            name: "merged-col-2".to_string(),
            arn: "arn:aws:aoss:us-east-1:123456789012:collection/def456".to_string(),
            type_: "SEARCH".to_string(),
            status: "ACTIVE".to_string(),
            description: None,
            kms_key_arn: None,
            created_date: 0,
            last_modified_date: 0,
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let snap2 = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap2.collections.len(), 2);
    assert!(snap2.collections.contains_key("abc123"));
    assert!(snap2.collections.contains_key("def456"));
}

// ---- State change notification tests ----

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = OpenSearchServerlessService::new();
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
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "123456789012");
    assert_eq!(got[0].1, "us-east-1");
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_opensearchserverless::OpenSearchServerlessStateView;
    use winterbaume_opensearchserverless::views::CollectionView;

    let svc = OpenSearchServerlessService::new();

    // Pre-seed
    let mut view = OpenSearchServerlessStateView::default();
    view.collections.insert(
        "seed1".to_string(),
        CollectionView {
            id: "seed1".to_string(),
            name: "seed-col".to_string(),
            arn: "arn:aws:aoss:us-east-1:123456789012:collection/seed1".to_string(),
            type_: "SEARCH".to_string(),
            status: "ACTIVE".to_string(),
            description: None,
            kms_key_arn: None,
            created_date: 0,
            last_modified_date: 0,
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Register listener after initial restore
    let snapshots: Arc<Mutex<Vec<OpenSearchServerlessStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Trigger another restore with a different collection
    let mut view2 = OpenSearchServerlessStateView::default();
    view2.collections.insert(
        "new1".to_string(),
        CollectionView {
            id: "new1".to_string(),
            name: "new-col".to_string(),
            arn: "arn:aws:aoss:us-east-1:123456789012:collection/new1".to_string(),
            type_: "SEARCH".to_string(),
            status: "ACTIVE".to_string(),
            description: None,
            kms_key_arn: None,
            created_date: 0,
            last_modified_date: 0,
        },
    );
    svc.restore("123456789012", "us-east-1", view2)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].collections.contains_key("new1"));
    assert!(!got[0].collections.contains_key("seed1"));
}
