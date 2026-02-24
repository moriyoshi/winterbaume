use aws_sdk_networkmanager::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_networkmanager::NetworkManagerService;

async fn make_client() -> aws_sdk_networkmanager::Client {
    let mock = MockAws::builder()
        .with_service(NetworkManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_networkmanager::Client::new(&config)
}

// ── Global Network tests ──

#[tokio::test]
async fn test_create_and_describe_global_network() {
    let client = make_client().await;

    let create_resp = client
        .create_global_network()
        .description("my test network")
        .send()
        .await
        .expect("create_global_network should succeed");

    let network = create_resp
        .global_network()
        .expect("should have GlobalNetwork");
    assert!(network.global_network_id().is_some());
    assert!(network.global_network_arn().is_some());
    assert_eq!(network.description(), Some("my test network"));

    let describe_resp = client
        .describe_global_networks()
        .send()
        .await
        .expect("describe_global_networks should succeed");

    let networks = describe_resp.global_networks();
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].global_network_id(), network.global_network_id());
}

#[tokio::test]
async fn test_delete_global_network() {
    let client = make_client().await;

    let create_resp = client
        .create_global_network()
        .description("delete-me")
        .send()
        .await
        .unwrap();

    let network_id = create_resp
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let delete_resp = client
        .delete_global_network()
        .global_network_id(&network_id)
        .send()
        .await
        .expect("delete_global_network should succeed");

    let deleted = delete_resp.global_network().unwrap();
    assert_eq!(deleted.state().map(|s| s.as_str()), Some("DELETING"));

    // Verify it's gone from list
    let describe_resp = client.describe_global_networks().send().await.unwrap();
    assert_eq!(describe_resp.global_networks().len(), 0);
}

#[tokio::test]
async fn test_update_global_network() {
    let client = make_client().await;

    let create_resp = client
        .create_global_network()
        .description("original description")
        .send()
        .await
        .unwrap();

    let network_id = create_resp
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_global_network()
        .global_network_id(&network_id)
        .description("updated description")
        .send()
        .await
        .expect("update_global_network should succeed");

    let updated = update_resp.global_network().unwrap();
    assert_eq!(updated.description(), Some("updated description"));
}

#[tokio::test]
async fn test_delete_nonexistent_global_network() {
    let client = make_client().await;

    let result = client
        .delete_global_network()
        .global_network_id("global-network-nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_nonexistent_global_network() {
    let client = make_client().await;

    let result = client
        .update_global_network()
        .global_network_id("global-network-nonexistent")
        .description("new desc")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_empty_global_networks() {
    let client = make_client().await;

    let resp = client
        .describe_global_networks()
        .send()
        .await
        .expect("describe should succeed on empty state");

    assert_eq!(resp.global_networks().len(), 0);
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    let create_resp = client
        .create_global_network()
        .description("ephemeral")
        .send()
        .await
        .unwrap();

    let network_id = create_resp
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let list_resp = client.describe_global_networks().send().await.unwrap();
    assert_eq!(list_resp.global_networks().len(), 1);

    client
        .delete_global_network()
        .global_network_id(&network_id)
        .send()
        .await
        .unwrap();

    let list_resp = client.describe_global_networks().send().await.unwrap();
    assert_eq!(list_resp.global_networks().len(), 0);
}

// ── Core Network tests ──

#[tokio::test]
async fn test_create_and_get_core_network() {
    let client = make_client().await;

    // First create a global network
    let gn = client
        .create_global_network()
        .description("gn for core network")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create core network
    let create_resp = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("my core network")
        .send()
        .await
        .expect("create_core_network should succeed");

    let cn = create_resp.core_network().expect("should have CoreNetwork");
    assert!(cn.core_network_id().is_some());
    assert!(cn.core_network_arn().is_some());
    assert_eq!(cn.description(), Some("my core network"));
    assert_eq!(cn.global_network_id(), Some(gn_id.as_str()));

    // Get core network
    let cn_id = cn.core_network_id().unwrap().to_string();
    let get_resp = client
        .get_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .expect("get_core_network should succeed");

    let fetched = get_resp.core_network().unwrap();
    assert_eq!(fetched.core_network_id(), Some(cn_id.as_str()));
    assert_eq!(fetched.description(), Some("my core network"));
}

#[tokio::test]
async fn test_delete_core_network() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let cn = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("delete-me")
        .send()
        .await
        .unwrap();
    let cn_id = cn
        .core_network()
        .unwrap()
        .core_network_id()
        .unwrap()
        .to_string();

    let delete_resp = client
        .delete_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .expect("delete_core_network should succeed");

    let deleted = delete_resp.core_network().unwrap();
    assert_eq!(deleted.state().map(|s| s.as_str()), Some("DELETING"));

    // Verify it's gone
    let get_result = client
        .get_core_network()
        .core_network_id(&cn_id)
        .send()
        .await;
    assert!(get_result.is_err());
}

#[tokio::test]
async fn test_update_core_network() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let cn = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("original")
        .send()
        .await
        .unwrap();
    let cn_id = cn
        .core_network()
        .unwrap()
        .core_network_id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_core_network()
        .core_network_id(&cn_id)
        .description("updated")
        .send()
        .await
        .expect("update_core_network should succeed");

    let updated = update_resp.core_network().unwrap();
    assert_eq!(updated.description(), Some("updated"));
}

#[tokio::test]
async fn test_list_core_networks() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Empty list
    let list_resp = client.list_core_networks().send().await.unwrap();
    assert_eq!(list_resp.core_networks().len(), 0);

    // Create two core networks
    client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("cn1")
        .send()
        .await
        .unwrap();
    client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("cn2")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_core_networks().send().await.unwrap();
    assert_eq!(list_resp.core_networks().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_core_network() {
    let client = make_client().await;

    let result = client
        .get_core_network()
        .core_network_id("core-network-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_core_network() {
    let client = make_client().await;

    let result = client
        .delete_core_network()
        .core_network_id("core-network-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Site tests ──

#[tokio::test]
async fn test_create_and_get_sites() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_site()
        .global_network_id(&gn_id)
        .description("my site")
        .send()
        .await
        .expect("create_site should succeed");

    let site = create_resp.site().expect("should have Site");
    assert!(site.site_id().is_some());
    assert!(site.site_arn().is_some());
    assert_eq!(site.description(), Some("my site"));
    assert_eq!(site.global_network_id(), Some(gn_id.as_str()));

    // Get sites
    let get_resp = client
        .get_sites()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_sites should succeed");

    let sites = get_resp.sites();
    assert_eq!(sites.len(), 1);
    assert_eq!(sites[0].site_id(), site.site_id());
}

#[tokio::test]
async fn test_delete_site() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .description("delete-me")
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    let delete_resp = client
        .delete_site()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .send()
        .await
        .expect("delete_site should succeed");

    let deleted = delete_resp.site().unwrap();
    assert_eq!(deleted.state().map(|s| s.as_str()), Some("DELETING"));

    // Verify it's gone
    let get_resp = client
        .get_sites()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.sites().len(), 0);
}

#[tokio::test]
async fn test_update_site() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .description("original")
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    let update_resp = client
        .update_site()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .description("updated site")
        .send()
        .await
        .expect("update_site should succeed");

    let updated = update_resp.site().unwrap();
    assert_eq!(updated.description(), Some("updated site"));
}

#[tokio::test]
async fn test_delete_nonexistent_site() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .delete_site()
        .global_network_id(&gn_id)
        .site_id("site-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Link tests ──

#[tokio::test]
async fn test_create_and_delete_link() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .description("site for link")
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    let bandwidth = aws_sdk_networkmanager::types::Bandwidth::builder()
        .download_speed(100)
        .upload_speed(50)
        .build();

    let create_resp = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .bandwidth(bandwidth)
        .description("my link")
        .provider("ISP")
        .r#type("broadband")
        .send()
        .await
        .expect("create_link should succeed");

    let link = create_resp.link().expect("should have Link");
    assert!(link.link_id().is_some());
    assert!(link.link_arn().is_some());
    assert_eq!(link.description(), Some("my link"));
    assert_eq!(link.site_id(), Some(site_id.as_str()));
    assert_eq!(link.provider(), Some("ISP"));

    let bw = link.bandwidth().unwrap();
    assert_eq!(bw.download_speed(), Some(100));
    assert_eq!(bw.upload_speed(), Some(50));

    // Delete
    let link_id = link.link_id().unwrap().to_string();
    let delete_resp = client
        .delete_link()
        .global_network_id(&gn_id)
        .link_id(&link_id)
        .send()
        .await
        .expect("delete_link should succeed");

    let deleted = delete_resp.link().unwrap();
    assert_eq!(deleted.state().map(|s| s.as_str()), Some("DELETING"));
}

#[tokio::test]
async fn test_delete_nonexistent_link() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .delete_link()
        .global_network_id(&gn_id)
        .link_id("link-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Tag tests ──

#[tokio::test]
async fn test_tag_list_untag_resource() {
    let client = make_client().await;

    // Create a global network to tag
    let gn = client
        .create_global_network()
        .description("gn for tagging")
        .send()
        .await
        .unwrap();
    let gn_arn = gn
        .global_network()
        .unwrap()
        .global_network_arn()
        .unwrap()
        .to_string();

    // Tag it
    let tag1 = aws_sdk_networkmanager::types::Tag::builder()
        .key("env")
        .value("prod")
        .build();
    let tag2 = aws_sdk_networkmanager::types::Tag::builder()
        .key("team")
        .value("platform")
        .build();

    client
        .tag_resource()
        .resource_arn(&gn_arn)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&gn_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tag_list();
    assert_eq!(tags.len(), 2);

    // Untag
    client
        .untag_resource()
        .resource_arn(&gn_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&gn_arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tag_list();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("team"));
    assert_eq!(tags[0].value(), Some("platform"));
}

// ── Lifecycle tests ──

#[tokio::test]
async fn test_core_network_lifecycle() {
    let client = make_client().await;

    // Create global network
    let gn = client
        .create_global_network()
        .description("lifecycle gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create core network
    let cn = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("lifecycle cn")
        .send()
        .await
        .unwrap();
    let cn_id = cn
        .core_network()
        .unwrap()
        .core_network_id()
        .unwrap()
        .to_string();

    // Get it
    let get_resp = client
        .get_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.core_network().unwrap().description(),
        Some("lifecycle cn")
    );

    // Update it
    client
        .update_core_network()
        .core_network_id(&cn_id)
        .description("updated lifecycle cn")
        .send()
        .await
        .unwrap();

    // Verify update
    let get_resp = client
        .get_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.core_network().unwrap().description(),
        Some("updated lifecycle cn")
    );

    // Delete
    client
        .delete_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_core_network()
        .core_network_id(&cn_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_site_lifecycle() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create
    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .description("lifecycle site")
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    // List (GetSites)
    let list_resp = client
        .get_sites()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.sites().len(), 1);

    // Update
    client
        .update_site()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .description("updated lifecycle site")
        .send()
        .await
        .unwrap();

    // Verify update via list
    let list_resp = client
        .get_sites()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        list_resp.sites()[0].description(),
        Some("updated lifecycle site")
    );

    // Delete
    client
        .delete_site()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    let list_resp = client
        .get_sites()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.sites().len(), 0);
}

// ── Additional coverage tests ──

#[tokio::test]
async fn test_create_core_network_nonexistent_global_network() {
    let client = make_client().await;

    let result = client
        .create_core_network()
        .global_network_id("global-network-nonexistent")
        .description("should fail")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_nonexistent_site() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .update_site()
        .global_network_id(&gn_id)
        .site_id("site-nonexistent")
        .description("new desc")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_link_with_tags() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    let create_resp = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(100)
                .upload_speed(50)
                .build(),
        )
        .tags(
            aws_sdk_networkmanager::types::Tag::builder()
                .key("Name")
                .value("TestLink")
                .build(),
        )
        .send()
        .await
        .expect("create_link with tags should succeed");

    let link = create_resp.link().expect("should have Link");
    let tags = link.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Name"));
    assert_eq!(tags[0].value(), Some("TestLink"));
}

#[tokio::test]
async fn test_create_link_nonexistent_site() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id("site-nonexistent")
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(10)
                .build(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_device() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .delete_device()
        .global_network_id(&gn_id)
        .device_id("device-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_device_with_tags() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_device()
        .global_network_id(&gn_id)
        .description("tagged device")
        .tags(
            aws_sdk_networkmanager::types::Tag::builder()
                .key("Env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .expect("create_device with tags should succeed");

    let device = create_resp.device().expect("should have Device");
    assert_eq!(device.description(), Some("tagged device"));
    let tags = device.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Env"));
    assert_eq!(tags[0].value(), Some("test"));
}

#[tokio::test]
async fn test_global_network_state_on_create() {
    let client = make_client().await;

    let create_resp = client
        .create_global_network()
        .description("state check")
        .send()
        .await
        .unwrap();

    let network = create_resp.global_network().unwrap();
    assert_eq!(network.state().map(|s| s.as_str()), Some("AVAILABLE"));
    assert!(network.created_at().is_some());
}

#[tokio::test]
async fn test_core_network_state_on_create() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_core_network()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();

    let cn = create_resp.core_network().unwrap();
    assert_eq!(cn.state().map(|s| s.as_str()), Some("AVAILABLE"));
    assert!(cn.created_at().is_some());
}

#[tokio::test]
async fn test_tag_resource_on_link() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    let link = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(10)
                .build(),
        )
        .send()
        .await
        .unwrap();
    let link_arn = link.link().unwrap().link_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&link_arn)
        .tags(
            aws_sdk_networkmanager::types::Tag::builder()
                .key("Purpose")
                .value("wan")
                .build(),
        )
        .send()
        .await
        .expect("tag_resource on link should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&link_arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tag_list();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Purpose"));
    assert_eq!(tags[0].value(), Some("wan"));
}

// ── Additional tests ported from moto test_networkmanager.py ──

// test_create_global_network with tags
#[tokio::test]
async fn test_create_global_network_with_tags() {
    let client = make_client().await;

    let tag = aws_sdk_networkmanager::types::Tag::builder()
        .key("Name")
        .value("TestNetwork")
        .build();

    let create_resp = client
        .create_global_network()
        .description("Test global network")
        .tags(tag)
        .send()
        .await
        .expect("create_global_network should succeed");

    let network = create_resp.global_network().unwrap();
    assert_eq!(network.description(), Some("Test global network"));

    // Tags should be returned
    let tags = network.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Name"));
    assert_eq!(tags[0].value(), Some("TestNetwork"));
}

// test_create_core_network with tags
#[tokio::test]
async fn test_create_core_network_with_tags() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("Test global network")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let tag = aws_sdk_networkmanager::types::Tag::builder()
        .key("Name")
        .value("TestNetwork")
        .build();

    let create_resp = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("Test core network")
        .tags(tag)
        .send()
        .await
        .expect("create_core_network should succeed");

    let cn = create_resp.core_network().unwrap();
    assert_eq!(cn.description(), Some("Test core network"));
    assert_eq!(cn.global_network_id(), Some(gn_id.as_str()));

    // Tags should be returned
    let tags = cn.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Name"));
    assert_eq!(tags[0].value(), Some("TestNetwork"));
}

// test_list_core_networks - checking OwnerAccountId
#[tokio::test]
async fn test_list_core_networks_owner_account_id() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    client
        .create_core_network()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();

    let list_resp = client.list_core_networks().send().await.unwrap();
    assert_eq!(list_resp.core_networks().len(), 1);

    let network = &list_resp.core_networks()[0];
    assert!(network.core_network_id().is_some());
    assert!(network.core_network_arn().is_some());
    assert!(network.global_network_id().is_some());
    assert!(network.state().is_some());
    // OwnerAccountId should be populated
    assert!(network.owner_account_id().is_some());
}

// test_tag_resource on core_network and site
#[tokio::test]
async fn test_tag_resource_on_core_network() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let cn = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("cn")
        .send()
        .await
        .unwrap();
    let cn = cn.core_network().unwrap();
    let cn_arn = cn.core_network_arn().unwrap().to_string();
    let cn_id = cn.core_network_id().unwrap().to_string();

    // Tag the core network
    let test_tags = vec![
        aws_sdk_networkmanager::types::Tag::builder()
            .key("Moto")
            .value("TestTag")
            .build(),
        aws_sdk_networkmanager::types::Tag::builder()
            .key("Owner")
            .value("Alice")
            .build(),
    ];

    client
        .tag_resource()
        .resource_arn(&cn_arn)
        .set_tags(Some(test_tags))
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify tags via get_core_network
    let get_resp = client
        .get_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .unwrap();

    let tags = get_resp.core_network().unwrap().tags();
    assert_eq!(tags.len(), 2);
}

// test_untag_resource
#[tokio::test]
async fn test_untag_resource_on_core_network() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create core network with 2 tags
    let cn = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("cn")
        .tags(
            aws_sdk_networkmanager::types::Tag::builder()
                .key("Name")
                .value("TestNetwork")
                .build(),
        )
        .tags(
            aws_sdk_networkmanager::types::Tag::builder()
                .key("DeleteMe")
                .value("DeleteThisTag!")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let cn = cn.core_network().unwrap();
    let cn_arn = cn.core_network_arn().unwrap().to_string();
    let cn_id = cn.core_network_id().unwrap().to_string();

    // Untag
    client
        .untag_resource()
        .resource_arn(&cn_arn)
        .tag_keys("DeleteMe")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify only 1 tag remains
    let get_resp = client
        .get_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .unwrap();

    let tags = get_resp.core_network().unwrap().tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Name"));
    assert_eq!(tags[0].value(), Some("TestNetwork"));
}

// test_create_site with tags
#[tokio::test]
async fn test_create_site_with_tags() {
    let client = make_client().await;

    let gn = client
        .create_global_network()
        .description("gn")
        .send()
        .await
        .unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let tag = aws_sdk_networkmanager::types::Tag::builder()
        .key("Name")
        .value("TestSite")
        .build();

    let create_resp = client
        .create_site()
        .global_network_id(&gn_id)
        .description("Test site")
        .tags(tag)
        .send()
        .await
        .expect("create_site should succeed");

    let site = create_resp.site().unwrap();
    assert_eq!(site.description(), Some("Test site"));
    assert_eq!(site.global_network_id(), Some(gn_id.as_str()));

    let tags = site.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Name"));
    assert_eq!(tags[0].value(), Some("TestSite"));
}

// ── Device tests ──

#[tokio::test]
async fn test_create_and_get_devices() {
    let client = make_client().await;

    // Create a global network first
    let gn = client
        .create_global_network()
        .send()
        .await
        .expect("create_global_network should succeed");
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create a device
    let create_resp = client
        .create_device()
        .global_network_id(&gn_id)
        .description("my-device")
        .send()
        .await
        .expect("create_device should succeed");

    let device = create_resp.device().expect("should have device");
    assert_eq!(device.description(), Some("my-device"));
    assert_eq!(device.global_network_id(), Some(gn_id.as_str()));
    let device_id = device.device_id().unwrap().to_string();

    // Get devices
    let get_resp = client
        .get_devices()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_devices should succeed");

    let devices = get_resp.devices();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_id(), Some(device_id.as_str()));
}

#[tokio::test]
async fn test_delete_device() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_device()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let device_id = create_resp
        .device()
        .unwrap()
        .device_id()
        .unwrap()
        .to_string();

    let del_resp = client
        .delete_device()
        .global_network_id(&gn_id)
        .device_id(&device_id)
        .send()
        .await
        .expect("delete_device should succeed");

    assert_eq!(
        del_resp.device().unwrap().device_id(),
        Some(device_id.as_str())
    );
}

#[tokio::test]
async fn test_get_links() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create a site for the link
    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    // Create a link
    client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(100)
                .upload_speed(50)
                .build(),
        )
        .send()
        .await
        .expect("create_link should succeed");

    // GetLinks
    let get_resp = client
        .get_links()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_links should succeed");

    let links = get_resp.links();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].global_network_id(), Some(gn_id.as_str()));
}

// ── UpdateDevice tests ──

#[tokio::test]
async fn test_update_device() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_device()
        .global_network_id(&gn_id)
        .description("original device")
        .send()
        .await
        .unwrap();
    let device_id = create_resp
        .device()
        .unwrap()
        .device_id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_device()
        .global_network_id(&gn_id)
        .device_id(&device_id)
        .description("updated device")
        .vendor("Cisco")
        .model("ISR4451")
        .send()
        .await
        .expect("update_device should succeed");

    let updated = update_resp.device().unwrap();
    assert_eq!(updated.description(), Some("updated device"));
    assert_eq!(updated.vendor(), Some("Cisco"));
    assert_eq!(updated.model(), Some("ISR4451"));
}

#[tokio::test]
async fn test_update_nonexistent_device() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .update_device()
        .global_network_id(&gn_id)
        .device_id("device-nonexistent")
        .description("nope")
        .send()
        .await;
    assert!(result.is_err());
}

// ── UpdateLink tests ──

#[tokio::test]
async fn test_update_link() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    let link = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(100)
                .upload_speed(50)
                .build(),
        )
        .description("original link")
        .send()
        .await
        .unwrap();
    let link_id = link.link().unwrap().link_id().unwrap().to_string();

    let update_resp = client
        .update_link()
        .global_network_id(&gn_id)
        .link_id(&link_id)
        .description("updated link")
        .provider("NewISP")
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(200)
                .upload_speed(100)
                .build(),
        )
        .send()
        .await
        .expect("update_link should succeed");

    let updated = update_resp.link().unwrap();
    assert_eq!(updated.description(), Some("updated link"));
    assert_eq!(updated.provider(), Some("NewISP"));
    let bw = updated.bandwidth().unwrap();
    assert_eq!(bw.download_speed(), Some(200));
    assert_eq!(bw.upload_speed(), Some(100));
}

// ── Connection tests ──

#[tokio::test]
async fn test_create_get_update_delete_connection() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create two devices for a connection
    let dev1 = client
        .create_device()
        .global_network_id(&gn_id)
        .description("device-a")
        .send()
        .await
        .unwrap();
    let dev1_id = dev1.device().unwrap().device_id().unwrap().to_string();

    let dev2 = client
        .create_device()
        .global_network_id(&gn_id)
        .description("device-b")
        .send()
        .await
        .unwrap();
    let dev2_id = dev2.device().unwrap().device_id().unwrap().to_string();

    // CreateConnection
    let create_resp = client
        .create_connection()
        .global_network_id(&gn_id)
        .device_id(&dev1_id)
        .connected_device_id(&dev2_id)
        .description("my connection")
        .send()
        .await
        .expect("create_connection should succeed");

    let conn = create_resp.connection().expect("should have Connection");
    assert!(conn.connection_id().is_some());
    assert!(conn.connection_arn().is_some());
    assert_eq!(conn.description(), Some("my connection"));
    assert_eq!(conn.device_id(), Some(dev1_id.as_str()));
    assert_eq!(conn.connected_device_id(), Some(dev2_id.as_str()));
    let conn_id = conn.connection_id().unwrap().to_string();

    // GetConnections
    let get_resp = client
        .get_connections()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_connections should succeed");
    let connections = get_resp.connections();
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].connection_id(), Some(conn_id.as_str()));

    // UpdateConnection
    let update_resp = client
        .update_connection()
        .global_network_id(&gn_id)
        .connection_id(&conn_id)
        .description("updated connection")
        .send()
        .await
        .expect("update_connection should succeed");
    assert_eq!(
        update_resp.connection().unwrap().description(),
        Some("updated connection")
    );

    // DeleteConnection
    let delete_resp = client
        .delete_connection()
        .global_network_id(&gn_id)
        .connection_id(&conn_id)
        .send()
        .await
        .expect("delete_connection should succeed");
    let deleted = delete_resp.connection().unwrap();
    assert_eq!(deleted.state().map(|s| s.as_str()), Some("DELETING"));

    // Verify it's gone
    let get_resp = client
        .get_connections()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.connections().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_connection() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .delete_connection()
        .global_network_id(&gn_id)
        .connection_id("connection-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Transit Gateway Registration tests ──

#[tokio::test]
async fn test_register_get_deregister_transit_gateway() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let tgw_arn = "arn:aws:ec2:us-east-1:123456789012:transit-gateway/tgw-0123456789abcdef0";

    // Register
    let reg_resp = client
        .register_transit_gateway()
        .global_network_id(&gn_id)
        .transit_gateway_arn(tgw_arn)
        .send()
        .await
        .expect("register_transit_gateway should succeed");

    let reg = reg_resp
        .transit_gateway_registration()
        .expect("should have registration");
    assert_eq!(reg.transit_gateway_arn(), Some(tgw_arn));
    assert_eq!(reg.global_network_id(), Some(gn_id.as_str()));

    // GetTransitGatewayRegistrations
    let get_resp = client
        .get_transit_gateway_registrations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_transit_gateway_registrations should succeed");
    let regs = get_resp.transit_gateway_registrations();
    assert_eq!(regs.len(), 1);
    assert_eq!(regs[0].transit_gateway_arn(), Some(tgw_arn));

    // Deregister
    let dereg_resp = client
        .deregister_transit_gateway()
        .global_network_id(&gn_id)
        .transit_gateway_arn(tgw_arn)
        .send()
        .await
        .expect("deregister_transit_gateway should succeed");

    let dereg = dereg_resp.transit_gateway_registration().unwrap();
    assert_eq!(
        dereg.state().and_then(|s| s.code()).map(|c| c.as_str()),
        Some("DELETING")
    );

    // Verify gone
    let get_resp = client
        .get_transit_gateway_registrations()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.transit_gateway_registrations().len(), 0);
}

// ── Link Association tests ──

#[tokio::test]
async fn test_associate_get_disassociate_link() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    let device = client
        .create_device()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let device_id = device.device().unwrap().device_id().unwrap().to_string();

    let link = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(100)
                .build(),
        )
        .send()
        .await
        .unwrap();
    let link_id = link.link().unwrap().link_id().unwrap().to_string();

    // AssociateLink
    let assoc_resp = client
        .associate_link()
        .global_network_id(&gn_id)
        .device_id(&device_id)
        .link_id(&link_id)
        .send()
        .await
        .expect("associate_link should succeed");

    let assoc = assoc_resp
        .link_association()
        .expect("should have LinkAssociation");
    assert_eq!(assoc.global_network_id(), Some(gn_id.as_str()));
    assert_eq!(assoc.device_id(), Some(device_id.as_str()));
    assert_eq!(assoc.link_id(), Some(link_id.as_str()));

    // GetLinkAssociations
    let get_resp = client
        .get_link_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_link_associations should succeed");
    let assocs = get_resp.link_associations();
    assert_eq!(assocs.len(), 1);

    // DisassociateLink
    let disassoc_resp = client
        .disassociate_link()
        .global_network_id(&gn_id)
        .device_id(&device_id)
        .link_id(&link_id)
        .send()
        .await
        .expect("disassociate_link should succeed");

    let disassoc = disassoc_resp.link_association().unwrap();
    assert_eq!(
        disassoc.link_association_state().map(|s| s.as_str()),
        Some("DELETED")
    );

    // Verify gone
    let get_resp = client
        .get_link_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.link_associations().len(), 0);
}

// ── Customer Gateway Association tests ──

#[tokio::test]
async fn test_associate_get_disassociate_customer_gateway() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let device = client
        .create_device()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let device_id = device.device().unwrap().device_id().unwrap().to_string();

    let cgw_arn = "arn:aws:ec2:us-east-1:123456789012:customer-gateway/cgw-0123456789abcdef0";

    // AssociateCustomerGateway
    let assoc_resp = client
        .associate_customer_gateway()
        .global_network_id(&gn_id)
        .customer_gateway_arn(cgw_arn)
        .device_id(&device_id)
        .send()
        .await
        .expect("associate_customer_gateway should succeed");

    let assoc = assoc_resp
        .customer_gateway_association()
        .expect("should have CustomerGatewayAssociation");
    assert_eq!(assoc.customer_gateway_arn(), Some(cgw_arn));
    assert_eq!(assoc.global_network_id(), Some(gn_id.as_str()));
    assert_eq!(assoc.device_id(), Some(device_id.as_str()));

    // GetCustomerGatewayAssociations
    let get_resp = client
        .get_customer_gateway_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_customer_gateway_associations should succeed");
    let assocs = get_resp.customer_gateway_associations();
    assert_eq!(assocs.len(), 1);
    assert_eq!(assocs[0].customer_gateway_arn(), Some(cgw_arn));

    // DisassociateCustomerGateway
    let disassoc_resp = client
        .disassociate_customer_gateway()
        .global_network_id(&gn_id)
        .customer_gateway_arn(cgw_arn)
        .send()
        .await
        .expect("disassociate_customer_gateway should succeed");

    let disassoc = disassoc_resp.customer_gateway_association().unwrap();
    assert_eq!(disassoc.state().map(|s| s.as_str()), Some("DELETED"));

    // Verify gone
    let get_resp = client
        .get_customer_gateway_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.customer_gateway_associations().len(), 0);
}

// ── Transit Gateway Connect Peer Association tests ──

#[tokio::test]
async fn test_associate_get_disassociate_transit_gateway_connect_peer() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let device = client
        .create_device()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let device_id = device.device().unwrap().device_id().unwrap().to_string();

    let tgw_cp_arn = "arn:aws:ec2:us-east-1:123456789012:transit-gateway-connect-peer/tgw-connect-peer-0123456789abcdef0";

    // AssociateTransitGatewayConnectPeer
    let assoc_resp = client
        .associate_transit_gateway_connect_peer()
        .global_network_id(&gn_id)
        .transit_gateway_connect_peer_arn(tgw_cp_arn)
        .device_id(&device_id)
        .send()
        .await
        .expect("associate_transit_gateway_connect_peer should succeed");

    let assoc = assoc_resp
        .transit_gateway_connect_peer_association()
        .expect("should have association");
    assert_eq!(assoc.transit_gateway_connect_peer_arn(), Some(tgw_cp_arn));
    assert_eq!(assoc.global_network_id(), Some(gn_id.as_str()));
    assert_eq!(assoc.device_id(), Some(device_id.as_str()));

    // GetTransitGatewayConnectPeerAssociations
    let get_resp = client
        .get_transit_gateway_connect_peer_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_transit_gateway_connect_peer_associations should succeed");
    let assocs = get_resp.transit_gateway_connect_peer_associations();
    assert_eq!(assocs.len(), 1);

    // DisassociateTransitGatewayConnectPeer
    let disassoc_resp = client
        .disassociate_transit_gateway_connect_peer()
        .global_network_id(&gn_id)
        .transit_gateway_connect_peer_arn(tgw_cp_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let disassoc = disassoc_resp
        .transit_gateway_connect_peer_association()
        .unwrap();
    assert_eq!(disassoc.state().map(|s| s.as_str()), Some("DELETED"));

    // Verify gone
    let get_resp = client
        .get_transit_gateway_connect_peer_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.transit_gateway_connect_peer_associations().len(),
        0
    );
}

// ── Route Analysis tests ──

#[tokio::test]
async fn test_start_and_get_route_analysis() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let source =
        aws_sdk_networkmanager::types::RouteAnalysisEndpointOptionsSpecification::builder()
            .transit_gateway_attachment_arn(
                "arn:aws:ec2:us-east-1:123456789012:transit-gateway-attachment/tgw-attach-src",
            )
            .ip_address("10.0.0.1")
            .build();

    let destination =
        aws_sdk_networkmanager::types::RouteAnalysisEndpointOptionsSpecification::builder()
            .transit_gateway_attachment_arn(
                "arn:aws:ec2:us-east-1:123456789012:transit-gateway-attachment/tgw-attach-dst",
            )
            .ip_address("10.0.1.1")
            .build();

    let start_resp = client
        .start_route_analysis()
        .global_network_id(&gn_id)
        .source(source)
        .destination(destination)
        .include_return_path(false)
        .use_middleboxes(false)
        .send()
        .await
        .expect("start_route_analysis should succeed");

    let ra = start_resp
        .route_analysis()
        .expect("should have RouteAnalysis");
    assert!(ra.route_analysis_id().is_some());
    assert_eq!(ra.global_network_id(), Some(gn_id.as_str()));
    let ra_id = ra.route_analysis_id().unwrap().to_string();

    // GetRouteAnalysis
    let get_resp = client
        .get_route_analysis()
        .global_network_id(&gn_id)
        .route_analysis_id(&ra_id)
        .send()
        .await
        .expect("get_route_analysis should succeed");

    let fetched = get_resp.route_analysis().unwrap();
    assert_eq!(fetched.route_analysis_id(), Some(ra_id.as_str()));
    assert_eq!(fetched.global_network_id(), Some(gn_id.as_str()));
}

#[tokio::test]
async fn test_get_nonexistent_route_analysis() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let result = client
        .get_route_analysis()
        .global_network_id(&gn_id)
        .route_analysis_id("ra-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── ListAttachments test ──

#[tokio::test]
async fn test_list_attachments_empty() {
    let client = make_client().await;

    let resp = client
        .list_attachments()
        .send()
        .await
        .expect("list_attachments should succeed on empty state");

    assert_eq!(resp.attachments().len(), 0);
}

// ── VPC Attachment tests ──

#[tokio::test]
async fn test_create_get_delete_vpc_attachment() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let cn = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("cn for vpc attachment")
        .send()
        .await
        .unwrap();
    let cn_id = cn
        .core_network()
        .unwrap()
        .core_network_id()
        .unwrap()
        .to_string();

    let vpc_arn = "arn:aws:ec2:us-east-1:123456789012:vpc/vpc-0123456789abcdef0";

    // CreateVpcAttachment
    let create_resp = client
        .create_vpc_attachment()
        .core_network_id(&cn_id)
        .vpc_arn(vpc_arn)
        .subnet_arns("arn:aws:ec2:us-east-1:123456789012:subnet/subnet-aaa")
        .subnet_arns("arn:aws:ec2:us-east-1:123456789012:subnet/subnet-bbb")
        .send()
        .await
        .expect("create_vpc_attachment should succeed");

    let att = create_resp
        .vpc_attachment()
        .expect("should have VpcAttachment");
    let attachment = att.attachment().expect("should have nested Attachment");
    assert!(attachment.attachment_id().is_some());
    assert_eq!(attachment.core_network_id(), Some(cn_id.as_str()));
    let att_id = attachment.attachment_id().unwrap().to_string();

    // GetVpcAttachment
    let get_resp = client
        .get_vpc_attachment()
        .attachment_id(&att_id)
        .send()
        .await
        .expect("get_vpc_attachment should succeed");
    let fetched = get_resp.vpc_attachment().unwrap();
    assert_eq!(
        fetched.attachment().unwrap().attachment_id(),
        Some(att_id.as_str())
    );

    // ListAttachments should show it
    let list_resp = client.list_attachments().send().await.unwrap();
    assert_eq!(list_resp.attachments().len(), 1);

    // DeleteAttachment
    let del_resp = client
        .delete_attachment()
        .attachment_id(&att_id)
        .send()
        .await
        .expect("delete_attachment should succeed");
    let deleted = del_resp.attachment().unwrap();
    assert_eq!(deleted.state().map(|s| s.as_str()), Some("DELETING"));

    // Verify gone from list
    let list_resp = client.list_attachments().send().await.unwrap();
    assert_eq!(list_resp.attachments().len(), 0);
}

// ── Connection with tags test ──

#[tokio::test]
async fn test_create_connection_with_tags() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let dev1 = client
        .create_device()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let dev1_id = dev1.device().unwrap().device_id().unwrap().to_string();

    let dev2 = client
        .create_device()
        .global_network_id(&gn_id)
        .send()
        .await
        .unwrap();
    let dev2_id = dev2.device().unwrap().device_id().unwrap().to_string();

    let tag = aws_sdk_networkmanager::types::Tag::builder()
        .key("Env")
        .value("staging")
        .build();

    let create_resp = client
        .create_connection()
        .global_network_id(&gn_id)
        .device_id(&dev1_id)
        .connected_device_id(&dev2_id)
        .description("tagged connection")
        .tags(tag)
        .send()
        .await
        .expect("create_connection with tags should succeed");

    let conn = create_resp.connection().unwrap();
    let tags = conn.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("Env"));
    assert_eq!(tags[0].value(), Some("staging"));
}

// ── GetConnections on empty state ──

#[tokio::test]
async fn test_get_connections_empty() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let resp = client
        .get_connections()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_connections should succeed on empty state");

    assert_eq!(resp.connections().len(), 0);
}

// ── GetTransitGatewayRegistrations on empty state ──

#[tokio::test]
async fn test_get_transit_gateway_registrations_empty() {
    let client = make_client().await;

    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    let resp = client
        .get_transit_gateway_registrations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_transit_gateway_registrations should succeed on empty state");

    assert_eq!(resp.transit_gateway_registrations().len(), 0);
}

// ── Coverage for FIX(terraform-e2e) handler fixes ──

/// FIX(terraform-e2e): Device polling with specific device ID filter.
///
/// When the Terraform provider polls for a specific device after creation it
/// sends deviceIds=[id] and expects EXACTLY one result.  Before the fix,
/// returning all devices in the global network caused FindOneBy to fail with
/// "too many results".  This test creates two devices and verifies that
/// filtering by a single device_id returns only the matching device.
#[tokio::test]
async fn test_get_devices_filtered_by_device_id() {
    let client = make_client().await;

    // Create a global network
    let gn = client.create_global_network().send().await.unwrap();
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create two devices in the same global network
    let dev1 = client
        .create_device()
        .global_network_id(&gn_id)
        .description("device-alpha")
        .send()
        .await
        .expect("create first device should succeed");
    let dev1_id = dev1.device().unwrap().device_id().unwrap().to_string();

    let dev2 = client
        .create_device()
        .global_network_id(&gn_id)
        .description("device-beta")
        .send()
        .await
        .expect("create second device should succeed");
    let dev2_id = dev2.device().unwrap().device_id().unwrap().to_string();

    // Without filter: both devices are returned
    let all_resp = client
        .get_devices()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_devices (unfiltered) should succeed");
    assert_eq!(all_resp.devices().len(), 2);

    // Filter by first device ID only
    let filtered_resp = client
        .get_devices()
        .global_network_id(&gn_id)
        .device_ids(&dev1_id)
        .send()
        .await
        .expect("get_devices (filtered) should succeed");
    let filtered = filtered_resp.devices();
    assert_eq!(filtered.len(), 1, "should return exactly one device");
    assert_eq!(filtered[0].device_id(), Some(dev1_id.as_str()));
    assert_eq!(filtered[0].description(), Some("device-alpha"));

    // Filter by second device ID only
    let filtered_resp2 = client
        .get_devices()
        .global_network_id(&gn_id)
        .device_ids(&dev2_id)
        .send()
        .await
        .expect("get_devices (filtered by dev2) should succeed");
    let filtered2 = filtered_resp2.devices();
    assert_eq!(filtered2.len(), 1, "should return exactly one device");
    assert_eq!(filtered2[0].device_id(), Some(dev2_id.as_str()));
    assert_eq!(filtered2[0].description(), Some("device-beta"));

    // Filter by a non-existent device ID: expect empty
    let empty_resp = client
        .get_devices()
        .global_network_id(&gn_id)
        .device_ids("device-does-not-exist")
        .send()
        .await
        .expect("get_devices (non-existent filter) should succeed");
    assert_eq!(empty_resp.devices().len(), 0);
}

/// FIX(terraform-e2e): URI handling for localhost requests.
///
/// The old implementation only handled amazonaws.com URIs; localhost test URIs
/// produced path segments like ["http:", "", "127.0.0.1:PORT", ...], causing
/// every route match to fall through to the 404 catch-all.  Because the mock
/// HTTP client routes requests via localhost, every successful integration test
/// implicitly validates this fix.  This test explicitly confirms that a
/// multi-step workflow (create global network, create device, query device)
/// succeeds end-to-end through the localhost-routed mock.
#[tokio::test]
async fn test_localhost_uri_routing_end_to_end() {
    let client = make_client().await;

    // Create a global network via localhost
    let gn = client
        .create_global_network()
        .description("localhost-routing-test")
        .send()
        .await
        .expect("create_global_network via localhost should succeed");
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Create a site (exercises a different route path)
    let site = client
        .create_site()
        .global_network_id(&gn_id)
        .description("site-via-localhost")
        .send()
        .await
        .expect("create_site via localhost should succeed");
    let site_id = site.site().unwrap().site_id().unwrap().to_string();

    // Create a device (another route path)
    let dev = client
        .create_device()
        .global_network_id(&gn_id)
        .site_id(&site_id)
        .description("device-via-localhost")
        .send()
        .await
        .expect("create_device via localhost should succeed");
    let dev_id = dev.device().unwrap().device_id().unwrap().to_string();

    // Query devices with filter (yet another route + query params)
    let get_resp = client
        .get_devices()
        .global_network_id(&gn_id)
        .device_ids(&dev_id)
        .send()
        .await
        .expect("get_devices via localhost should succeed");
    assert_eq!(get_resp.devices().len(), 1);
    assert_eq!(
        get_resp.devices()[0].site_id(),
        Some(site_id.as_str()),
        "device should be associated with the site created via localhost routing"
    );

    // Delete device (DELETE method route)
    client
        .delete_device()
        .global_network_id(&gn_id)
        .device_id(&dev_id)
        .send()
        .await
        .expect("delete_device via localhost should succeed");

    // Delete global network (cleanup route)
    client
        .delete_global_network()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("delete_global_network via localhost should succeed");
}
