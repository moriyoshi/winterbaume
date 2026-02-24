use aws_sdk_outposts::config::BehaviorVersion;
use winterbaume_core::{MockAws, MockService};
use winterbaume_outposts::{OutpostsService, OutpostsStateView};

async fn make_client() -> aws_sdk_outposts::Client {
    let mock = MockAws::builder()
        .with_service(OutpostsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_outposts::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_outposts::Client::new(&config)
}

// -------------------------------------------------------------------------
// Site tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_site() {
    let client = make_client().await;

    let create_resp = client
        .create_site()
        .name("test-site")
        .description("A test site")
        .send()
        .await
        .expect("create_site should succeed");

    let site = create_resp.site().expect("site should be present");
    assert_eq!(site.name(), Some("test-site"));
    assert_eq!(site.description(), Some("A test site"));
    assert!(site.site_id().unwrap_or("").starts_with("os-"));

    // Get
    let site_id = site.site_id().unwrap();
    let get_resp = client
        .get_site()
        .site_id(site_id)
        .send()
        .await
        .expect("get_site should succeed");

    let got_site = get_resp.site().expect("site should be present");
    assert_eq!(got_site.name(), site.name());
    assert_eq!(got_site.site_id(), site.site_id());
}

#[tokio::test]
async fn test_list_sites() {
    let client = make_client().await;

    client
        .create_site()
        .name("site-1")
        .send()
        .await
        .expect("create_site should succeed");
    client
        .create_site()
        .name("site-2")
        .send()
        .await
        .expect("create_site should succeed");

    let list_resp = client
        .list_sites()
        .send()
        .await
        .expect("list_sites should succeed");

    let sites = list_resp.sites();
    assert_eq!(sites.len(), 2);
}

#[tokio::test]
async fn test_update_site() {
    let client = make_client().await;

    let create_resp = client
        .create_site()
        .name("original-name")
        .send()
        .await
        .unwrap();
    let site_id = create_resp.site().unwrap().site_id().unwrap().to_string();

    let update_resp = client
        .update_site()
        .site_id(&site_id)
        .name("updated-name")
        .description("new description")
        .send()
        .await
        .expect("update_site should succeed");

    let updated = update_resp.site().unwrap();
    assert_eq!(updated.name(), Some("updated-name"));
    assert_eq!(updated.description(), Some("new description"));
}

#[tokio::test]
async fn test_delete_site() {
    let client = make_client().await;

    let create_resp = client.create_site().name("to-delete").send().await.unwrap();
    let site_id = create_resp.site().unwrap().site_id().unwrap().to_string();

    client
        .delete_site()
        .site_id(&site_id)
        .send()
        .await
        .expect("delete_site should succeed");

    // Get should fail
    let result = client.get_site().site_id(&site_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_site() {
    let client = make_client().await;
    let result = client.get_site().site_id("os-nonexistent").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_site() {
    let client = make_client().await;
    let result = client.delete_site().site_id("os-nonexistent").send().await;
    assert!(result.is_err());
}

// -------------------------------------------------------------------------
// Outpost tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_outpost() {
    let client = make_client().await;

    // Create a site first (required)
    let site_resp = client.create_site().name("my-site").send().await.unwrap();
    let site_id = site_resp.site().unwrap().site_id().unwrap().to_string();

    let create_resp = client
        .create_outpost()
        .name("test-outpost")
        .site_id(&site_id)
        .description("A test outpost")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create_outpost should succeed");

    let outpost = create_resp.outpost().expect("outpost should be present");
    assert_eq!(outpost.name(), Some("test-outpost"));
    assert_eq!(outpost.description(), Some("A test outpost"));
    assert_eq!(outpost.site_id(), Some(site_id.as_str()));
    assert!(outpost.outpost_id().unwrap_or("").starts_with("op-"));

    // Get
    let outpost_id = outpost.outpost_id().unwrap();
    let get_resp = client
        .get_outpost()
        .outpost_id(outpost_id)
        .send()
        .await
        .expect("get_outpost should succeed");

    let got = get_resp.outpost().expect("outpost should be present");
    assert_eq!(got.name(), outpost.name());
    assert_eq!(got.outpost_id(), outpost.outpost_id());
}

#[tokio::test]
async fn test_create_outpost_with_nonexistent_site() {
    let client = make_client().await;

    let result = client
        .create_outpost()
        .name("test-outpost")
        .site_id("os-nonexistent99999999")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_outposts() {
    let client = make_client().await;

    let site_resp = client.create_site().name("my-site").send().await.unwrap();
    let site_id = site_resp.site().unwrap().site_id().unwrap().to_string();

    client
        .create_outpost()
        .name("outpost-1")
        .site_id(&site_id)
        .send()
        .await
        .unwrap();
    client
        .create_outpost()
        .name("outpost-2")
        .site_id(&site_id)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_outposts()
        .send()
        .await
        .expect("list_outposts should succeed");

    assert_eq!(list_resp.outposts().len(), 2);
}

#[tokio::test]
async fn test_update_outpost() {
    let client = make_client().await;

    let site_resp = client.create_site().name("my-site").send().await.unwrap();
    let site_id = site_resp.site().unwrap().site_id().unwrap().to_string();

    let create_resp = client
        .create_outpost()
        .name("original")
        .site_id(&site_id)
        .send()
        .await
        .unwrap();
    let outpost_id = create_resp
        .outpost()
        .unwrap()
        .outpost_id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_outpost()
        .outpost_id(&outpost_id)
        .name("renamed")
        .description("updated desc")
        .send()
        .await
        .expect("update_outpost should succeed");

    let updated = update_resp.outpost().unwrap();
    assert_eq!(updated.name(), Some("renamed"));
    assert_eq!(updated.description(), Some("updated desc"));
}

#[tokio::test]
async fn test_delete_outpost() {
    let client = make_client().await;

    let site_resp = client.create_site().name("my-site").send().await.unwrap();
    let site_id = site_resp.site().unwrap().site_id().unwrap().to_string();

    let create_resp = client
        .create_outpost()
        .name("to-delete")
        .site_id(&site_id)
        .send()
        .await
        .unwrap();
    let outpost_id = create_resp
        .outpost()
        .unwrap()
        .outpost_id()
        .unwrap()
        .to_string();

    client
        .delete_outpost()
        .outpost_id(&outpost_id)
        .send()
        .await
        .expect("delete_outpost should succeed");

    let result = client.get_outpost().outpost_id(&outpost_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_outpost() {
    let client = make_client().await;
    let result = client
        .get_outpost()
        .outpost_id("op-nonexistent99999999")
        .send()
        .await;
    assert!(result.is_err());
}

// -------------------------------------------------------------------------
// Lifecycle test
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_outpost_lifecycle() {
    let client = make_client().await;

    // Create site
    let site = client
        .create_site()
        .name("lifecycle-site")
        .send()
        .await
        .unwrap()
        .site()
        .unwrap()
        .clone();
    let site_id = site.site_id().unwrap();

    // Create outpost
    let outpost = client
        .create_outpost()
        .name("lifecycle-outpost")
        .site_id(site_id)
        .send()
        .await
        .unwrap()
        .outpost()
        .unwrap()
        .clone();
    let outpost_id = outpost.outpost_id().unwrap();

    // Describe
    let got = client
        .get_outpost()
        .outpost_id(outpost_id)
        .send()
        .await
        .unwrap()
        .outpost()
        .unwrap()
        .clone();
    assert_eq!(got.name(), Some("lifecycle-outpost"));

    // Update
    client
        .update_outpost()
        .outpost_id(outpost_id)
        .name("renamed")
        .send()
        .await
        .unwrap();

    // Verify update
    let got2 = client
        .get_outpost()
        .outpost_id(outpost_id)
        .send()
        .await
        .unwrap()
        .outpost()
        .unwrap()
        .clone();
    assert_eq!(got2.name(), Some("renamed"));

    // Delete
    client
        .delete_outpost()
        .outpost_id(outpost_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    assert!(
        client
            .get_outpost()
            .outpost_id(outpost_id)
            .send()
            .await
            .is_err()
    );

    // Delete site
    client.delete_site().site_id(site_id).send().await.unwrap();

    assert!(client.get_site().site_id(site_id).send().await.is_err());
}

// -------------------------------------------------------------------------
// Tag tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_tag_operations() {
    let client = make_client().await;

    let site = client
        .create_site()
        .name("tag-test-site")
        .send()
        .await
        .unwrap()
        .site()
        .unwrap()
        .clone();

    let site_arn = site.site_arn().unwrap().to_string();

    // Tag
    client
        .tag_resource()
        .resource_arn(&site_arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&site_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().cloned().unwrap_or_default();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));

    // Untag
    client
        .untag_resource()
        .resource_arn(&site_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&site_arn)
        .send()
        .await
        .unwrap();

    let tags2 = tags_resp2.tags().cloned().unwrap_or_default();
    assert!(!tags2.contains_key("env"));
    assert_eq!(tags2.get("team").map(|s| s.as_str()), Some("platform"));
}

// -------------------------------------------------------------------------
// State view tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_snapshot_and_restore() {
    use winterbaume_core::StatefulService;

    let svc = OutpostsService::new();

    // Create resources via raw requests
    let site_req = winterbaume_core::MockRequest {
        method: "POST".to_string(),
        uri: "https://outposts.us-east-1.amazonaws.com/sites".to_string(),
        headers: http::HeaderMap::new(),
        body: bytes::Bytes::from(r#"{"Name":"snap-site"}"#),
    };
    let site_resp = svc.handle(site_req).await;
    assert_eq!(site_resp.status, 200);
    let site_body: serde_json::Value = serde_json::from_slice(&site_resp.body).unwrap();
    let site_id = site_body["Site"]["SiteId"].as_str().unwrap().to_string();

    let outpost_req = winterbaume_core::MockRequest {
        method: "POST".to_string(),
        uri: "https://outposts.us-east-1.amazonaws.com/outposts".to_string(),
        headers: http::HeaderMap::new(),
        body: bytes::Bytes::from(format!(
            r#"{{"Name":"snap-outpost","SiteId":"{}"}}"#,
            site_id
        )),
    };
    let outpost_resp = svc.handle(outpost_req).await;
    assert_eq!(outpost_resp.status, 200);

    // Snapshot
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.sites.len(), 1);
    assert_eq!(snapshot.outposts.len(), 1);

    // Restore into fresh service
    let svc2 = OutpostsService::new();
    svc2.restore("123456789012", "us-east-1", snapshot.clone())
        .await
        .unwrap();

    let snapshot2 = svc2.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot2.sites.len(), 1);
    assert_eq!(snapshot2.outposts.len(), 1);
}

#[tokio::test]
async fn test_merge_is_additive() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_outposts::views::SiteView;

    let svc = OutpostsService::new();

    // Pre-seed with one site
    let view1 = OutpostsStateView {
        sites: {
            let mut m = HashMap::new();
            m.insert(
                "os-aaa".to_string(),
                SiteView {
                    site_id: "os-aaa".to_string(),
                    site_arn: "arn:aws:outposts:us-east-1:123456789012:site/os-aaa".to_string(),
                    account_id: "123456789012".to_string(),
                    name: "site-a".to_string(),
                    description: None,
                    notes: None,
                    operating_address_country_code: None,
                    operating_address_state_or_region: None,
                    operating_address_city: None,
                    tags: HashMap::new(),
                },
            );
            m
        },
        outposts: HashMap::new(),
    };
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap();

    // Merge with another site
    let view2 = OutpostsStateView {
        sites: {
            let mut m = HashMap::new();
            m.insert(
                "os-bbb".to_string(),
                SiteView {
                    site_id: "os-bbb".to_string(),
                    site_arn: "arn:aws:outposts:us-east-1:123456789012:site/os-bbb".to_string(),
                    account_id: "123456789012".to_string(),
                    name: "site-b".to_string(),
                    description: None,
                    notes: None,
                    operating_address_country_code: None,
                    operating_address_state_or_region: None,
                    operating_address_city: None,
                    tags: HashMap::new(),
                },
            );
            m
        },
        outposts: HashMap::new(),
    };
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.sites.len(), 2);
    assert!(snapshot.sites.contains_key("os-aaa"));
    assert!(snapshot.sites.contains_key("os-bbb"));
}

// -------------------------------------------------------------------------
// State change notification tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = OutpostsService::new();
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
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_outposts::views::SiteView;

    let svc = OutpostsService::new();

    // Pre-seed
    let view = OutpostsStateView {
        sites: {
            let mut m = HashMap::new();
            m.insert(
                "os-aaa".to_string(),
                SiteView {
                    site_id: "os-aaa".to_string(),
                    site_arn: "arn:aws:outposts:us-east-1:123456789012:site/os-aaa".to_string(),
                    account_id: "123456789012".to_string(),
                    name: "site-a".to_string(),
                    description: None,
                    notes: None,
                    operating_address_country_code: None,
                    operating_address_state_or_region: None,
                    operating_address_city: None,
                    tags: HashMap::new(),
                },
            );
            m
        },
        outposts: HashMap::new(),
    };
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Register listener after first event
    let snapshots: Arc<Mutex<Vec<OutpostsStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Merge another site
    let view2 = OutpostsStateView {
        sites: {
            let mut m = HashMap::new();
            m.insert(
                "os-bbb".to_string(),
                SiteView {
                    site_id: "os-bbb".to_string(),
                    site_arn: "arn:aws:outposts:us-east-1:123456789012:site/os-bbb".to_string(),
                    account_id: "123456789012".to_string(),
                    name: "site-b".to_string(),
                    description: None,
                    notes: None,
                    operating_address_country_code: None,
                    operating_address_state_or_region: None,
                    operating_address_city: None,
                    tags: HashMap::new(),
                },
            );
            m
        },
        outposts: HashMap::new(),
    };
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].sites.len(), 2);
    assert!(got[0].sites.contains_key("os-aaa"));
    assert!(got[0].sites.contains_key("os-bbb"));
}
