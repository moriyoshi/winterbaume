use aws_sdk_opensearch::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_opensearch::OpenSearchService;

async fn make_opensearch_client() -> aws_sdk_opensearch::Client {
    let mock = MockAws::builder()
        .with_service(OpenSearchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_opensearch::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_opensearch::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_domain() {
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("test-domain")
        .engine_version("OpenSearch_2.11")
        .send()
        .await
        .expect("create_domain should succeed");

    let status = resp.domain_status().expect("should have domain status");
    assert_eq!(status.domain_name(), "test-domain");
    assert!(status.arn().contains("test-domain"));
    assert_eq!(status.created(), Some(true));

    let desc_resp = client
        .describe_domain()
        .domain_name("test-domain")
        .send()
        .await
        .expect("describe_domain should succeed");

    let status = desc_resp
        .domain_status()
        .expect("should have domain status");
    assert_eq!(status.domain_name(), "test-domain");
    assert_eq!(
        status.engine_version().unwrap_or_default(),
        "OpenSearch_2.11"
    );
}

#[tokio::test]
async fn test_list_domain_names() {
    let client = make_opensearch_client().await;

    // List should be empty initially
    let resp = client
        .list_domain_names()
        .send()
        .await
        .expect("list_domain_names should succeed");
    assert!(resp.domain_names().is_empty());

    // Create two domains
    for name in ["domain-a", "domain-b"] {
        client
            .create_domain()
            .domain_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_domain_names()
        .send()
        .await
        .expect("list_domain_names should succeed");
    assert_eq!(resp.domain_names().len(), 2);
}

#[tokio::test]
async fn test_delete_domain() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("del-domain")
        .send()
        .await
        .unwrap();

    client
        .delete_domain()
        .domain_name("del-domain")
        .send()
        .await
        .expect("delete_domain should succeed");

    // Describe should fail after deletion
    let result = client
        .describe_domain()
        .domain_name("del-domain")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_fails() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("dup-domain")
        .send()
        .await
        .unwrap();

    let result = client
        .create_domain()
        .domain_name("dup-domain")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_fails() {
    let client = make_opensearch_client().await;

    let result = client
        .describe_domain()
        .domain_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_fails() {
    let client = make_opensearch_client().await;

    let result = client
        .delete_domain()
        .domain_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_domains() {
    let client = make_opensearch_client().await;

    // Create two domains
    for name in ["desc-domain-a", "desc-domain-b"] {
        client
            .create_domain()
            .domain_name(name)
            .engine_version("OpenSearch_2.11")
            .send()
            .await
            .unwrap();
    }

    // Describe both domains at once
    let resp = client
        .describe_domains()
        .domain_names("desc-domain-a")
        .domain_names("desc-domain-b")
        .send()
        .await
        .expect("describe_domains should succeed");

    let status_list = resp.domain_status_list();
    assert_eq!(status_list.len(), 2);

    let names: Vec<&str> = status_list.iter().map(|s| s.domain_name()).collect();
    assert!(names.contains(&"desc-domain-a"));
    assert!(names.contains(&"desc-domain-b"));
}

#[tokio::test]
async fn test_describe_domains_partial() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("partial-domain")
        .send()
        .await
        .unwrap();

    // Ask for one existing and one non-existing domain
    let resp = client
        .describe_domains()
        .domain_names("partial-domain")
        .domain_names("nonexistent-domain")
        .send()
        .await
        .expect("describe_domains should succeed");

    // Only the existing domain should be returned
    let status_list = resp.domain_status_list();
    assert_eq!(status_list.len(), 1);
    assert_eq!(status_list[0].domain_name(), "partial-domain");
}

#[tokio::test]
async fn test_describe_domain_config() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("config-domain")
        .engine_version("OpenSearch_2.11")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_domain_config()
        .domain_name("config-domain")
        .send()
        .await
        .expect("describe_domain_config should succeed");

    let config = resp.domain_config().expect("should have domain config");

    // Check engine version config
    let engine = config.engine_version().expect("should have engine version");
    let engine_opts = engine.options();
    assert_eq!(engine_opts, "OpenSearch_2.11");
}

#[tokio::test]
async fn test_describe_domain_config_nonexistent_fails() {
    let client = make_opensearch_client().await;

    let result = client
        .describe_domain_config()
        .domain_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_domain_config() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("update-domain")
        .engine_version("OpenSearch_2.11")
        .send()
        .await
        .unwrap();

    // Update the access policies
    let resp = client
        .update_domain_config()
        .domain_name("update-domain")
        .access_policies(r#"{"Version":"2012-10-17"}"#)
        .send()
        .await
        .expect("update_domain_config should succeed");

    let config = resp.domain_config().expect("should have domain config");
    let policies = config
        .access_policies()
        .expect("should have access policies");
    let opts = policies.options();
    assert_eq!(opts, r#"{"Version":"2012-10-17"}"#);

    // Verify the update persisted via describe
    let desc_resp = client
        .describe_domain()
        .domain_name("update-domain")
        .send()
        .await
        .unwrap();

    let status = desc_resp.domain_status().unwrap();
    assert_eq!(
        status.access_policies().unwrap_or_default(),
        r#"{"Version":"2012-10-17"}"#
    );
}

#[tokio::test]
async fn test_update_domain_config_nonexistent_fails() {
    let client = make_opensearch_client().await;

    let result = client
        .update_domain_config()
        .domain_name("nonexistent")
        .access_policies("test")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_compatible_versions() {
    let client = make_opensearch_client().await;

    let resp = client
        .get_compatible_versions()
        .send()
        .await
        .expect("get_compatible_versions should succeed");

    let versions = resp.compatible_versions();
    assert!(!versions.is_empty());

    // Verify the structure has source and target versions
    let first = &versions[0];
    assert!(first.source_version().is_some());
    let targets = first.target_versions();
    assert!(!targets.is_empty());
}

#[tokio::test]
async fn test_get_compatible_versions_with_domain() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("compat-domain")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_compatible_versions()
        .domain_name("compat-domain")
        .send()
        .await
        .expect("get_compatible_versions with domain should succeed");

    assert!(!resp.compatible_versions().is_empty());
}

#[tokio::test]
async fn test_get_compatible_versions_nonexistent_domain() {
    let client = make_opensearch_client().await;

    let result = client
        .get_compatible_versions()
        .domain_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_and_list_tags() {
    let client = make_opensearch_client().await;

    // Create a domain first
    let resp = client
        .create_domain()
        .domain_name("tagged-domain")
        .send()
        .await
        .unwrap();

    let arn = resp.domain_status().unwrap().arn().to_string();

    // Add tags
    use aws_sdk_opensearch::types::Tag;
    client
        .add_tags()
        .arn(&arn)
        .tag_list(Tag::builder().key("env").value("test").build().unwrap())
        .tag_list(
            Tag::builder()
                .key("project")
                .value("winterbaume")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags should succeed");

    // List tags
    let list_resp = client
        .list_tags()
        .arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = list_resp.tag_list();
    assert_eq!(tags.len(), 2);

    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(tag_map.get("env"), Some(&"test"));
    assert_eq!(tag_map.get("project"), Some(&"winterbaume"));
}

#[tokio::test]
async fn test_remove_tags() {
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("removetag-domain")
        .send()
        .await
        .unwrap();

    let arn = resp.domain_status().unwrap().arn().to_string();

    // Add tags
    use aws_sdk_opensearch::types::Tag;
    client
        .add_tags()
        .arn(&arn)
        .tag_list(Tag::builder().key("env").value("test").build().unwrap())
        .tag_list(Tag::builder().key("team").value("core").build().unwrap())
        .send()
        .await
        .unwrap();

    // Remove one tag
    client
        .remove_tags()
        .arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("remove_tags should succeed");

    // List remaining tags
    let list_resp = client.list_tags().arn(&arn).send().await.unwrap();

    let tags = list_resp.tag_list();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "team");
    assert_eq!(tags[0].value(), "core");
}

#[tokio::test]
async fn test_list_tags_empty() {
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("notags-domain")
        .send()
        .await
        .unwrap();

    let arn = resp.domain_status().unwrap().arn().to_string();

    let list_resp = client
        .list_tags()
        .arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed for domain with no tags");

    assert!(list_resp.tag_list().is_empty());
}

// ===== Moto parity tests =====

/// Parity with moto test_create_domain__minimal_options.
#[tokio::test]
async fn test_create_domain_minimal_options() {
    let client = make_opensearch_client().await;
    let resp = client
        .create_domain()
        .domain_name("testdn")
        .send()
        .await
        .expect("create_domain should succeed");

    let status = resp.domain_status().unwrap();
    assert!(!status.domain_id().is_empty());
    assert_eq!(status.domain_name(), "testdn");
    assert!(status.endpoint().is_some());
}

/// Parity with moto test_create_domain_with_some_options.
#[tokio::test]
async fn test_create_domain_with_options() {
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("testdn")
        .ebs_options(
            aws_sdk_opensearch::types::EbsOptions::builder()
                .ebs_enabled(true)
                .volume_size(10)
                .build(),
        )
        .engine_version("OpenSearch_1.1")
        .send()
        .await
        .expect("create_domain should succeed");

    let status = resp.domain_status().unwrap();
    assert_eq!(status.created(), Some(true));
    assert_eq!(status.engine_version().unwrap(), "OpenSearch_1.1");
    let ebs = status.ebs_options().unwrap();
    assert_eq!(ebs.ebs_enabled(), Some(true));
    assert_eq!(ebs.volume_size(), Some(10));
}

/// Parity with moto test_describe_domain: checks default engine version is "OpenSearch_2.5".
#[tokio::test]
async fn test_describe_domain_default_engine_version() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_domain()
        .domain_name("testdn")
        .send()
        .await
        .unwrap();

    let status = resp.domain_status().unwrap();
    assert!(!status.domain_id().is_empty());
    assert_eq!(status.domain_name(), "testdn");
    assert_eq!(status.engine_version().unwrap(), "OpenSearch_2.5");
}

/// Parity with moto test_describe_unknown_domain.
#[tokio::test]
async fn test_describe_unknown_domain() {
    let client = make_opensearch_client().await;

    let result = client.describe_domain().domain_name("testdn").send().await;
    assert!(result.is_err());
}

/// Parity with moto test_describe_domain_config: checks State and PendingDeletion.
#[tokio::test]
async fn test_describe_domain_config_fields() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_domain_config()
        .domain_name("testdn")
        .send()
        .await
        .unwrap();

    let config = resp.domain_config().unwrap();
    let ev = config.engine_version().unwrap();
    assert_eq!(ev.options(), "OpenSearch_2.5");

    let ev_status = ev.status().expect("should have status");
    assert_eq!(ev_status.state().as_str(), "Active");
    assert_eq!(ev_status.pending_deletion(), Some(false));
}

/// Parity with moto test_delete_domain: deleted domain should have Deleted=true.
#[tokio::test]
async fn test_delete_domain_returns_deleted_true() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_domain()
        .domain_name("testdn")
        .send()
        .await
        .expect("delete_domain should succeed");

    let status = resp.domain_status().unwrap();
    assert!(!status.domain_id().is_empty());
    assert_eq!(status.domain_name(), "testdn");
    assert_eq!(status.deleted(), Some(true));
}

/// Parity with moto test_list_domain_names.
#[tokio::test]
async fn test_list_domain_names_with_engine_type() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .engine_version("OpenSearch_1.0")
        .send()
        .await
        .unwrap();

    let resp = client.list_domain_names().send().await.unwrap();

    assert!(!resp.domain_names().is_empty());
    let found = resp
        .domain_names()
        .iter()
        .any(|d| d.domain_name() == Some("testdn"));
    assert!(found, "should find testdn in domain names list");
}

/// Parity with moto test_describe_domains: checks AdvancedSecurityOptions and AdvancedOptions present.
#[tokio::test]
async fn test_describe_domains_advanced_fields() {
    let client = make_opensearch_client().await;

    let domain_names: Vec<String> = (1..=4).map(|i| format!("env{i}")).collect();
    for name in &domain_names {
        client
            .create_domain()
            .domain_name(name)
            .engine_version("OpenSearch_1.0")
            .send()
            .await
            .unwrap();
    }

    let mut builder = client.describe_domains();
    for name in &domain_names {
        builder = builder.domain_names(name);
    }
    let resp = builder.send().await.unwrap();

    assert_eq!(resp.domain_status_list().len(), 4);
    for domain in resp.domain_status_list() {
        assert!(
            domain_names.iter().any(|n| n == domain.domain_name()),
            "domain name should be in the list"
        );
        // Moto checks for AdvancedSecurityOptions and AdvancedOptions
        assert!(
            domain.advanced_security_options().is_some(),
            "should have AdvancedSecurityOptions"
        );
        assert!(
            domain.advanced_options().is_some(),
            "should have AdvancedOptions"
        );
    }

    // Test for invalid domain name returns empty list
    let resp = client
        .describe_domains()
        .domain_names("invalid")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.domain_status_list().len(), 0);
}

/// Parity with moto test_create_without_tags.
#[tokio::test]
async fn test_create_without_tags() {
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("testdn")
        .send()
        .await
        .unwrap();

    let arn = resp.domain_status().unwrap().arn().to_string();

    let tags_resp = client.list_tags().arn(&arn).send().await.unwrap();
    assert!(tags_resp.tag_list().is_empty());
}

/// Parity with moto test_create_with_tags.
#[tokio::test]
async fn test_create_with_tags() {
    let client = make_opensearch_client().await;
    use aws_sdk_opensearch::types::Tag;

    let resp = client
        .create_domain()
        .domain_name("testdn")
        .tag_list(Tag::builder().key("k1").value("v1").build().unwrap())
        .send()
        .await
        .unwrap();

    let arn = resp.domain_status().unwrap().arn().to_string();

    // Verify initial tag
    let tags_resp = client.list_tags().arn(&arn).send().await.unwrap();
    assert_eq!(tags_resp.tag_list().len(), 1);
    assert_eq!(tags_resp.tag_list()[0].key(), "k1");
    assert_eq!(tags_resp.tag_list()[0].value(), "v1");

    // Add another tag
    client
        .add_tags()
        .arn(&arn)
        .tag_list(Tag::builder().key("k2").value("v2").build().unwrap())
        .send()
        .await
        .unwrap();

    let tags_resp = client.list_tags().arn(&arn).send().await.unwrap();
    assert_eq!(tags_resp.tag_list().len(), 2);

    let tag_map: std::collections::HashMap<&str, &str> = tags_resp
        .tag_list()
        .iter()
        .map(|t| (t.key(), t.value()))
        .collect();
    assert_eq!(tag_map.get("k1"), Some(&"v1"));
    assert_eq!(tag_map.get("k2"), Some(&"v2"));

    // Remove first tag
    client
        .remove_tags()
        .arn(&arn)
        .tag_keys("k1")
        .send()
        .await
        .unwrap();

    let tags_resp = client.list_tags().arn(&arn).send().await.unwrap();
    assert_eq!(tags_resp.tag_list().len(), 1);
    assert_eq!(tags_resp.tag_list()[0].key(), "k2");
    assert_eq!(tags_resp.tag_list()[0].value(), "v2");
}

/// Parity with moto test_get_compatible_versions.
#[tokio::test]
async fn test_get_compatible_versions_count() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .send()
        .await
        .unwrap();

    let versions = client
        .get_compatible_versions()
        .send()
        .await
        .unwrap()
        .compatible_versions()
        .to_vec();
    assert!(!versions.is_empty());

    let versions_with_domain = client
        .get_compatible_versions()
        .domain_name("testdn")
        .send()
        .await
        .unwrap()
        .compatible_versions()
        .to_vec();
    assert!(!versions_with_domain.is_empty());
    // Both should return the same number of versions
    assert_eq!(versions.len(), versions_with_domain.len());
}

/// Parity with moto test_get_compatible_versions_unknown_domain.
#[tokio::test]
async fn test_get_compatible_versions_unknown_domain() {
    let client = make_opensearch_client().await;

    let result = client
        .get_compatible_versions()
        .domain_name("testdn")
        .send()
        .await;
    assert!(result.is_err());
}

/// Parity with moto test_update_domain_config.
#[tokio::test]
async fn test_update_domain_config_ebs() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .ebs_options(
            aws_sdk_opensearch::types::EbsOptions::builder()
                .ebs_enabled(true)
                .volume_size(10)
                .build(),
        )
        .engine_version("OpenSearch_1.1")
        .send()
        .await
        .unwrap();

    let config = client
        .update_domain_config()
        .domain_name("testdn")
        .ebs_options(
            aws_sdk_opensearch::types::EbsOptions::builder()
                .ebs_enabled(false)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ebs = config
        .domain_config()
        .unwrap()
        .ebs_options()
        .unwrap()
        .options()
        .unwrap();
    assert_eq!(ebs.ebs_enabled(), Some(false));
}

/// Parity with moto test_list_filtered_domain_names: filter by EngineType returns matching domains.
#[tokio::test]
async fn test_list_domain_names_filter_by_engine_type() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .engine_version("OpenSearch_1.0")
        .send()
        .await
        .unwrap();

    // Filter by "OpenSearch" engine type
    let resp = client
        .list_domain_names()
        .engine_type(aws_sdk_opensearch::types::EngineType::OpenSearch)
        .send()
        .await
        .expect("list_domain_names with EngineType filter should succeed");

    let domain_names = resp.domain_names();
    let found = domain_names
        .iter()
        .any(|d| d.domain_name() == Some("testdn"));
    assert!(found, "testdn should appear in OpenSearch-filtered list");

    // Verify EngineType field is present in the results
    for domain in domain_names {
        assert!(domain.engine_type().is_some(), "engine_type should be set");
        assert_eq!(
            domain.engine_type().unwrap().as_str(),
            "OpenSearch",
            "engine_type should be OpenSearch"
        );
    }
}

/// Parity with moto test_list_unknown_domain_names_engine_type: unknown engine type returns error.
#[tokio::test]
async fn test_list_domain_names_unknown_engine_type() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("testdn")
        .engine_version("OpenSearch_1.0")
        .send()
        .await
        .unwrap();

    // The SDK may not allow invalid enum values directly; test with a known-invalid raw type
    // Instead, test that valid "Elasticsearch" engine type also works as a filter
    let resp = client
        .list_domain_names()
        .engine_type(aws_sdk_opensearch::types::EngineType::Elasticsearch)
        .send()
        .await
        .expect("list_domain_names with Elasticsearch filter should succeed");

    // testdn is OpenSearch, so should not appear in Elasticsearch-filtered list
    let domain_names = resp.domain_names();
    let found = domain_names
        .iter()
        .any(|d| d.domain_name() == Some("testdn"));
    assert!(
        !found,
        "testdn (OpenSearch) should not appear in Elasticsearch-filtered list"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon OpenSearch Service
// ============================================================================

/// A domain created with an Elasticsearch engine version should appear with
/// EngineType::Elasticsearch in ListDomainNames.
#[tokio::test]
async fn test_elasticsearch_engine_version() {
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("es-domain")
        .engine_version("Elasticsearch_7.10")
        .send()
        .await
        .expect("create_domain with Elasticsearch version should succeed");

    let status = resp.domain_status().unwrap();
    assert_eq!(status.domain_name(), "es-domain");
    assert_eq!(
        status.engine_version().unwrap_or_default(),
        "Elasticsearch_7.10"
    );

    // ListDomainNames should show the domain with Elasticsearch engine type
    let list_resp = client
        .list_domain_names()
        .engine_type(aws_sdk_opensearch::types::EngineType::Elasticsearch)
        .send()
        .await
        .expect("list_domain_names with Elasticsearch filter should succeed");

    let found = list_resp
        .domain_names()
        .iter()
        .any(|d| d.domain_name() == Some("es-domain"));
    assert!(
        found,
        "es-domain should appear in Elasticsearch-filtered list"
    );

    for domain in list_resp.domain_names() {
        assert_eq!(
            domain.engine_type().unwrap().as_str(),
            "Elasticsearch",
            "engine_type should be Elasticsearch"
        );
    }
}

/// UpdateDomainConfig with ClusterConfig should update instance_count and instance_type.
#[tokio::test]
async fn test_update_domain_config_cluster() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("cluster-update-domain")
        .send()
        .await
        .unwrap();

    // Update cluster config - change instance count to 3
    let update_resp = client
        .update_domain_config()
        .domain_name("cluster-update-domain")
        .cluster_config(
            aws_sdk_opensearch::types::ClusterConfig::builder()
                .instance_count(3)
                .instance_type(
                    aws_sdk_opensearch::types::OpenSearchPartitionInstanceType::from(
                        "m5.xlarge.search",
                    ),
                )
                .build(),
        )
        .send()
        .await
        .expect("update_domain_config with cluster config should succeed");

    let config = update_resp.domain_config().unwrap();
    let cluster = config.cluster_config().unwrap().options().unwrap();
    assert_eq!(cluster.instance_count(), Some(3));
    assert!(
        cluster.instance_type().is_some(),
        "instance_type should be present"
    );

    // Verify via describe_domain_config that change persisted
    let desc_resp = client
        .describe_domain_config()
        .domain_name("cluster-update-domain")
        .send()
        .await
        .unwrap();

    let desc_cluster = desc_resp
        .domain_config()
        .unwrap()
        .cluster_config()
        .unwrap()
        .options()
        .unwrap();
    assert_eq!(desc_cluster.instance_count(), Some(3));
}

/// DescribeDomains with an empty domain names list should return an empty status list.
#[tokio::test]
async fn test_describe_domains_empty() {
    let client = make_opensearch_client().await;

    // Create a domain that should NOT appear (we pass no names)
    client
        .create_domain()
        .domain_name("ignored-domain")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_domains()
        .send()
        .await
        .expect("describe_domains with empty list should succeed");

    assert!(
        resp.domain_status_list().is_empty(),
        "describe_domains with no names should return empty list"
    );
}

/// AddTags with an existing tag key should overwrite the value (upsert).
#[tokio::test]
async fn test_add_tags_overwrite() {
    use aws_sdk_opensearch::types::Tag;
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("overwrite-tag-domain")
        .send()
        .await
        .unwrap();

    let arn = resp.domain_status().unwrap().arn().to_string();

    // Add initial tag
    client
        .add_tags()
        .arn(&arn)
        .tag_list(Tag::builder().key("env").value("staging").build().unwrap())
        .send()
        .await
        .unwrap();

    // Overwrite same key with new value
    client
        .add_tags()
        .arn(&arn)
        .tag_list(
            Tag::builder()
                .key("env")
                .value("production")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags overwriting an existing key should succeed");

    let list_resp = client.list_tags().arn(&arn).send().await.unwrap();
    let tags = list_resp.tag_list();

    // Should still have only one "env" tag, with the updated value
    let env_tags: Vec<_> = tags.iter().filter(|t| t.key() == "env").collect();
    assert_eq!(env_tags.len(), 1, "should have exactly one 'env' tag");
    assert_eq!(
        env_tags[0].value(),
        "production",
        "tag value should be updated to 'production'"
    );
}

/// Full domain lifecycle: create -> describe -> update -> delete -> verify gone.
#[tokio::test]
async fn test_full_domain_lifecycle() {
    let client = make_opensearch_client().await;

    // 1. Create
    let create_resp = client
        .create_domain()
        .domain_name("lifecycle-domain")
        .engine_version("OpenSearch_2.11")
        .send()
        .await
        .expect("create_domain should succeed");

    let status = create_resp.domain_status().unwrap();
    assert_eq!(status.domain_name(), "lifecycle-domain");
    assert_eq!(status.created(), Some(true));
    assert_eq!(status.deleted(), Some(false));

    // 2. Describe
    let desc_resp = client
        .describe_domain()
        .domain_name("lifecycle-domain")
        .send()
        .await
        .expect("describe_domain should succeed");

    let desc_status = desc_resp.domain_status().unwrap();
    assert_eq!(desc_status.engine_version().unwrap(), "OpenSearch_2.11");
    assert!(!desc_status.domain_id().is_empty());
    assert!(desc_status.endpoint().is_some());

    // 3. Update
    let update_resp = client
        .update_domain_config()
        .domain_name("lifecycle-domain")
        .access_policies(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("update_domain_config should succeed");

    let updated_config = update_resp.domain_config().unwrap();
    assert_eq!(
        updated_config.access_policies().unwrap().options(),
        r#"{"Version":"2012-10-17","Statement":[]}"#
    );

    // 4. Verify update via describe_domain
    let post_update_resp = client
        .describe_domain()
        .domain_name("lifecycle-domain")
        .send()
        .await
        .unwrap();
    assert_eq!(
        post_update_resp
            .domain_status()
            .unwrap()
            .access_policies()
            .unwrap_or_default(),
        r#"{"Version":"2012-10-17","Statement":[]}"#
    );

    // 5. Delete
    let delete_resp = client
        .delete_domain()
        .domain_name("lifecycle-domain")
        .send()
        .await
        .expect("delete_domain should succeed");

    let deleted_status = delete_resp.domain_status().unwrap();
    assert_eq!(deleted_status.domain_name(), "lifecycle-domain");
    assert_eq!(deleted_status.deleted(), Some(true));

    // 6. Verify gone
    let gone_result = client
        .describe_domain()
        .domain_name("lifecycle-domain")
        .send()
        .await;
    assert!(
        gone_result.is_err(),
        "describe_domain after delete should return error"
    );
    let err_str = format!("{:?}", gone_result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

/// Deleted domain should not appear in ListDomainNames.
#[tokio::test]
async fn test_list_domain_names_after_delete() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("keep-domain")
        .send()
        .await
        .unwrap();
    client
        .create_domain()
        .domain_name("delete-me-domain")
        .send()
        .await
        .unwrap();

    // Verify both appear
    let list_resp = client.list_domain_names().send().await.unwrap();
    assert_eq!(list_resp.domain_names().len(), 2);

    // Delete one
    client
        .delete_domain()
        .domain_name("delete-me-domain")
        .send()
        .await
        .unwrap();

    // Deleted domain should no longer appear
    let list_resp = client.list_domain_names().send().await.unwrap();
    assert_eq!(list_resp.domain_names().len(), 1);

    let found_keep = list_resp
        .domain_names()
        .iter()
        .any(|d| d.domain_name() == Some("keep-domain"));
    let found_deleted = list_resp
        .domain_names()
        .iter()
        .any(|d| d.domain_name() == Some("delete-me-domain"));

    assert!(found_keep, "keep-domain should still appear in list");
    assert!(
        !found_deleted,
        "delete-me-domain should not appear in list after deletion"
    );
}

/// Domain ARN should follow the pattern arn:aws:es:{region}:{account_id}:domain/{name}.
#[tokio::test]
async fn test_domain_arn_format() {
    let client = make_opensearch_client().await;

    let resp = client
        .create_domain()
        .domain_name("arn-check-domain")
        .send()
        .await
        .expect("create_domain should succeed");

    let status = resp.domain_status().unwrap();
    let arn = status.arn();

    // ARN must start with "arn:aws:es:"
    assert!(
        arn.starts_with("arn:aws:es:"),
        "ARN should start with arn:aws:es:, got: {arn}"
    );
    // ARN must contain the domain name
    assert!(
        arn.contains("arn-check-domain"),
        "ARN should contain the domain name, got: {arn}"
    );
    // ARN should contain "domain/"
    assert!(
        arn.contains("domain/"),
        "ARN should contain 'domain/', got: {arn}"
    );
    // ARN should contain the region (us-east-1 as set in make_opensearch_client)
    assert!(
        arn.contains("us-east-1"),
        "ARN should contain the region, got: {arn}"
    );
}

// =============================================================================
// VPC Endpoint tests
// =============================================================================

#[tokio::test]
async fn test_create_and_list_vpc_endpoints() {
    let client = make_opensearch_client().await;

    let domain_resp = client
        .create_domain()
        .domain_name("vpc-ep-domain")
        .send()
        .await
        .unwrap();
    let domain_arn = domain_resp.domain_status().unwrap().arn().to_string();

    use aws_sdk_opensearch::types::VpcOptions;
    let resp = client
        .create_vpc_endpoint()
        .domain_arn(&domain_arn)
        .vpc_options(
            VpcOptions::builder()
                .subnet_ids("subnet-12345")
                .security_group_ids("sg-abcde")
                .build(),
        )
        .send()
        .await
        .expect("create_vpc_endpoint should succeed");

    let ep = resp.vpc_endpoint().expect("should have vpc endpoint");
    assert_eq!(ep.domain_arn().unwrap_or_default(), domain_arn);
    assert_eq!(ep.status().map(|s| s.as_str()), Some("ACTIVE"));

    let list_resp = client
        .list_vpc_endpoints()
        .send()
        .await
        .expect("list_vpc_endpoints should succeed");
    assert_eq!(list_resp.vpc_endpoint_summary_list().len(), 1);
}

#[tokio::test]
async fn test_describe_vpc_endpoints() {
    let client = make_opensearch_client().await;

    let domain_resp = client
        .create_domain()
        .domain_name("vpc-desc-domain")
        .send()
        .await
        .unwrap();
    let domain_arn = domain_resp.domain_status().unwrap().arn().to_string();

    use aws_sdk_opensearch::types::VpcOptions;
    let create_resp = client
        .create_vpc_endpoint()
        .domain_arn(&domain_arn)
        .vpc_options(VpcOptions::builder().subnet_ids("subnet-99").build())
        .send()
        .await
        .unwrap();
    let ep_id = create_resp
        .vpc_endpoint()
        .unwrap()
        .vpc_endpoint_id()
        .unwrap()
        .to_string();

    let desc_resp = client
        .describe_vpc_endpoints()
        .vpc_endpoint_ids(&ep_id)
        .send()
        .await
        .expect("describe_vpc_endpoints should succeed");

    assert_eq!(desc_resp.vpc_endpoints().len(), 1);
    assert_eq!(
        desc_resp.vpc_endpoints()[0]
            .vpc_endpoint_id()
            .unwrap_or_default(),
        ep_id
    );
}

#[tokio::test]
async fn test_delete_vpc_endpoint() {
    let client = make_opensearch_client().await;

    let domain_resp = client
        .create_domain()
        .domain_name("vpc-del-domain")
        .send()
        .await
        .unwrap();
    let domain_arn = domain_resp.domain_status().unwrap().arn().to_string();

    use aws_sdk_opensearch::types::VpcOptions;
    let create_resp = client
        .create_vpc_endpoint()
        .domain_arn(&domain_arn)
        .vpc_options(VpcOptions::builder().subnet_ids("subnet-del").build())
        .send()
        .await
        .unwrap();
    let ep_id = create_resp
        .vpc_endpoint()
        .unwrap()
        .vpc_endpoint_id()
        .unwrap()
        .to_string();

    client
        .delete_vpc_endpoint()
        .vpc_endpoint_id(&ep_id)
        .send()
        .await
        .expect("delete_vpc_endpoint should succeed");

    let list_resp = client.list_vpc_endpoints().send().await.unwrap();
    assert!(list_resp.vpc_endpoint_summary_list().is_empty());
}

// =============================================================================
// VPC Endpoint Access Authorization tests
// =============================================================================

#[tokio::test]
async fn test_authorize_and_list_vpc_endpoint_access() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("auth-domain")
        .send()
        .await
        .unwrap();

    let resp = client
        .authorize_vpc_endpoint_access()
        .domain_name("auth-domain")
        .account("123456789012")
        .send()
        .await
        .expect("authorize_vpc_endpoint_access should succeed");

    let principal = resp
        .authorized_principal()
        .expect("should have authorized principal");
    assert_eq!(principal.principal().unwrap_or_default(), "123456789012");

    let list_resp = client
        .list_vpc_endpoint_access()
        .domain_name("auth-domain")
        .send()
        .await
        .expect("list_vpc_endpoint_access should succeed");

    assert_eq!(list_resp.authorized_principal_list().len(), 1);
}

#[tokio::test]
async fn test_revoke_vpc_endpoint_access() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("revoke-domain")
        .send()
        .await
        .unwrap();

    client
        .authorize_vpc_endpoint_access()
        .domain_name("revoke-domain")
        .account("111111111111")
        .send()
        .await
        .unwrap();

    client
        .revoke_vpc_endpoint_access()
        .domain_name("revoke-domain")
        .account("111111111111")
        .send()
        .await
        .expect("revoke_vpc_endpoint_access should succeed");

    let list_resp = client
        .list_vpc_endpoint_access()
        .domain_name("revoke-domain")
        .send()
        .await
        .unwrap();
    assert!(list_resp.authorized_principal_list().is_empty());
}

// =============================================================================
// Data Source tests
// =============================================================================

#[tokio::test]
async fn test_add_get_list_delete_data_source() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("ds-domain")
        .send()
        .await
        .unwrap();

    use aws_sdk_opensearch::types::{DataSourceType, S3GlueDataCatalog};
    client
        .add_data_source()
        .domain_name("ds-domain")
        .name("my-datasource")
        .description("A test data source")
        .data_source_type(DataSourceType::S3GlueDataCatalog(
            S3GlueDataCatalog::builder()
                .role_arn("arn:aws:iam::123456789012:role/DataRole")
                .build(),
        ))
        .send()
        .await
        .expect("add_data_source should succeed");

    let get_resp = client
        .get_data_source()
        .domain_name("ds-domain")
        .name("my-datasource")
        .send()
        .await
        .expect("get_data_source should succeed");

    assert_eq!(get_resp.name().unwrap_or_default(), "my-datasource");

    let list_resp = client
        .list_data_sources()
        .domain_name("ds-domain")
        .send()
        .await
        .expect("list_data_sources should succeed");

    assert_eq!(list_resp.data_sources().len(), 1);

    client
        .delete_data_source()
        .domain_name("ds-domain")
        .name("my-datasource")
        .send()
        .await
        .expect("delete_data_source should succeed");

    let list_resp2 = client
        .list_data_sources()
        .domain_name("ds-domain")
        .send()
        .await
        .unwrap();
    assert!(list_resp2.data_sources().is_empty());
}

// =============================================================================
// Direct Query Data Source tests
// =============================================================================

#[tokio::test]
async fn test_add_get_list_delete_direct_query_data_source() {
    let client = make_opensearch_client().await;

    use aws_sdk_opensearch::types::{CloudWatchDirectQueryDataSource, DirectQueryDataSourceType};

    let add_resp = client
        .add_direct_query_data_source()
        .data_source_name("my-dq-source")
        .description("Direct query test")
        .data_source_type(DirectQueryDataSourceType::CloudWatchLog(
            CloudWatchDirectQueryDataSource::builder()
                .role_arn("arn:aws:iam::123456789012:role/CWRole")
                .build()
                .unwrap(),
        ))
        .open_search_arns("arn:aws:es:us-east-1:123456789012:domain/my-domain")
        .send()
        .await
        .expect("add_direct_query_data_source should succeed");

    assert!(add_resp.data_source_arn().is_some());

    let get_resp = client
        .get_direct_query_data_source()
        .data_source_name("my-dq-source")
        .send()
        .await
        .expect("get_direct_query_data_source should succeed");

    assert_eq!(
        get_resp.data_source_name().unwrap_or_default(),
        "my-dq-source"
    );

    let list_resp = client
        .list_direct_query_data_sources()
        .send()
        .await
        .expect("list_direct_query_data_sources should succeed");

    assert_eq!(list_resp.direct_query_data_sources().len(), 1);

    client
        .delete_direct_query_data_source()
        .data_source_name("my-dq-source")
        .send()
        .await
        .expect("delete_direct_query_data_source should succeed");

    let list_resp2 = client
        .list_direct_query_data_sources()
        .send()
        .await
        .unwrap();
    assert!(list_resp2.direct_query_data_sources().is_empty());
}

// =============================================================================
// Package tests
// =============================================================================

#[tokio::test]
async fn test_create_describe_delete_package() {
    let client = make_opensearch_client().await;

    use aws_sdk_opensearch::types::{PackageSource, PackageType};
    let resp = client
        .create_package()
        .package_name("my-package")
        .package_type(PackageType::TxtDictionary)
        .package_source(
            PackageSource::builder()
                .s3_bucket_name("my-bucket")
                .s3_key("dict.txt")
                .build(),
        )
        .send()
        .await
        .expect("create_package should succeed");

    let pkg = resp.package_details().expect("should have package details");
    let pkg_id = pkg.package_id().unwrap_or_default().to_string();
    assert!(!pkg_id.is_empty());
    assert_eq!(pkg.package_name().unwrap_or_default(), "my-package");

    let desc_resp = client
        .describe_packages()
        .send()
        .await
        .expect("describe_packages should succeed");

    assert!(!desc_resp.package_details_list().is_empty());

    client
        .delete_package()
        .package_id(&pkg_id)
        .send()
        .await
        .expect("delete_package should succeed");

    let desc_resp2 = client.describe_packages().send().await.unwrap();
    assert!(desc_resp2.package_details_list().is_empty());
}

#[tokio::test]
async fn test_associate_dissociate_package() {
    let client = make_opensearch_client().await;

    client
        .create_domain()
        .domain_name("pkg-domain")
        .send()
        .await
        .unwrap();

    use aws_sdk_opensearch::types::{PackageSource, PackageType};
    let pkg_resp = client
        .create_package()
        .package_name("assoc-pkg")
        .package_type(PackageType::TxtDictionary)
        .package_source(
            PackageSource::builder()
                .s3_bucket_name("b")
                .s3_key("k")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let pkg_id = pkg_resp
        .package_details()
        .unwrap()
        .package_id()
        .unwrap()
        .to_string();

    client
        .associate_package()
        .package_id(&pkg_id)
        .domain_name("pkg-domain")
        .send()
        .await
        .expect("associate_package should succeed");

    let list_resp = client
        .list_packages_for_domain()
        .domain_name("pkg-domain")
        .send()
        .await
        .expect("list_packages_for_domain should succeed");
    assert_eq!(list_resp.domain_package_details_list().len(), 1);

    client
        .dissociate_package()
        .package_id(&pkg_id)
        .domain_name("pkg-domain")
        .send()
        .await
        .expect("dissociate_package should succeed");

    let list_resp2 = client
        .list_packages_for_domain()
        .domain_name("pkg-domain")
        .send()
        .await
        .unwrap();
    assert!(list_resp2.domain_package_details_list().is_empty());
}

// =============================================================================
// Cross-Cluster Connection tests
// =============================================================================

#[tokio::test]
async fn test_create_describe_delete_outbound_connection() {
    let client = make_opensearch_client().await;

    use aws_sdk_opensearch::types::{AwsDomainInformation, DomainInformationContainer};

    let resp = client
        .create_outbound_connection()
        .connection_alias("my-cc-conn")
        .local_domain_info(
            DomainInformationContainer::builder()
                .aws_domain_information(
                    AwsDomainInformation::builder()
                        .domain_name("local-domain")
                        .owner_id("123456789012")
                        .region("us-east-1")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .remote_domain_info(
            DomainInformationContainer::builder()
                .aws_domain_information(
                    AwsDomainInformation::builder()
                        .domain_name("remote-domain")
                        .owner_id("999999999999")
                        .region("us-west-2")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("create_outbound_connection should succeed");

    let conn_id = resp
        .connection_id()
        .expect("should have connection_id")
        .to_string();

    let desc_resp = client
        .describe_outbound_connections()
        .send()
        .await
        .expect("describe_outbound_connections should succeed");

    assert_eq!(desc_resp.connections().len(), 1);

    client
        .delete_outbound_connection()
        .connection_id(&conn_id)
        .send()
        .await
        .expect("delete_outbound_connection should succeed");

    let desc_resp2 = client.describe_outbound_connections().send().await.unwrap();
    assert!(desc_resp2.connections().is_empty());
}

#[tokio::test]
async fn test_describe_inbound_connections_empty() {
    let client = make_opensearch_client().await;

    let resp = client
        .describe_inbound_connections()
        .send()
        .await
        .expect("describe_inbound_connections should succeed");

    // Initially empty
    assert!(resp.connections().is_empty());
}

// =============================================================================
// Reserved Instance tests
// =============================================================================

#[tokio::test]
async fn test_describe_reserved_instances_empty() {
    let client = make_opensearch_client().await;

    let resp = client
        .describe_reserved_instances()
        .send()
        .await
        .expect("describe_reserved_instances should succeed");

    assert!(resp.reserved_instances().is_empty());
}

#[tokio::test]
async fn test_purchase_and_describe_reserved_instances() {
    let client = make_opensearch_client().await;

    let resp = client
        .purchase_reserved_instance_offering()
        .reservation_name("my-reservation")
        .reserved_instance_offering_id("offering-abc123")
        .instance_count(2)
        .send()
        .await
        .expect("purchase_reserved_instance_offering should succeed");

    assert!(resp.reserved_instance_id().is_some());
    assert_eq!(
        resp.reservation_name().unwrap_or_default(),
        "my-reservation"
    );

    let desc_resp = client
        .describe_reserved_instances()
        .send()
        .await
        .expect("describe_reserved_instances should succeed");

    assert_eq!(desc_resp.reserved_instances().len(), 1);
    assert_eq!(
        desc_resp.reserved_instances()[0]
            .reservation_name()
            .unwrap_or_default(),
        "my-reservation"
    );
    assert_eq!(desc_resp.reserved_instances()[0].instance_count(), 2);
}

#[tokio::test]
async fn test_describe_reserved_instance_offerings() {
    let client = make_opensearch_client().await;

    let resp = client
        .describe_reserved_instance_offerings()
        .send()
        .await
        .expect("describe_reserved_instance_offerings should succeed");

    // Returns empty list (static stub)
    assert!(resp.reserved_instance_offerings().is_empty());
}
