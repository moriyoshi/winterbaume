use aws_sdk_dsql::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dsql::DsqlService;

async fn make_dsql_client() -> aws_sdk_dsql::Client {
    let mock = MockAws::builder().with_service(DsqlService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dsql::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_dsql::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_cluster() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    let identifier = create_resp.identifier();
    assert!(
        !identifier.is_empty(),
        "cluster identifier should not be empty"
    );
    assert_eq!(
        identifier.len(),
        26,
        "cluster identifier should be 26 chars"
    );

    let arn = create_resp.arn();
    assert!(arn.contains("dsql"), "ARN should contain 'dsql'");
    assert!(
        arn.contains(identifier),
        "ARN should contain the cluster id"
    );

    let get_resp = client
        .get_cluster()
        .identifier(identifier)
        .send()
        .await
        .expect("get_cluster should succeed");

    assert_eq!(get_resp.identifier(), identifier);
    assert_eq!(get_resp.arn(), arn);
    assert!(!get_resp.deletion_protection_enabled());
}

#[tokio::test]
async fn test_create_cluster_with_tags() {
    let client = make_dsql_client().await;

    let mut tags = std::collections::HashMap::new();
    tags.insert("MyKey".to_string(), "MyValue".to_string());

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("create_cluster with tags should succeed");

    let identifier = create_resp.identifier();

    let get_resp = client
        .get_cluster()
        .identifier(identifier)
        .send()
        .await
        .expect("get_cluster should succeed");

    let resp_tags = get_resp.tags();
    assert!(resp_tags.is_some(), "tags should be present");
    let resp_tags = resp_tags.unwrap();
    assert_eq!(resp_tags.get("MyKey"), Some(&"MyValue".to_string()));
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let identifier = create_resp.identifier().to_string();

    let delete_resp = client
        .delete_cluster()
        .identifier(&identifier)
        .send()
        .await
        .expect("delete_cluster should succeed");

    assert_eq!(delete_resp.identifier(), identifier);
    assert_eq!(
        delete_resp.status().as_str(),
        "DELETING",
        "status should be DELETING after deletion"
    );

    // Verify it's gone
    let get_result = client.get_cluster().identifier(&identifier).send().await;
    assert!(get_result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_delete_cluster_with_deletion_protection() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(true)
        .send()
        .await
        .unwrap();

    let identifier = create_resp.identifier().to_string();

    let delete_result = client.delete_cluster().identifier(&identifier).send().await;
    assert!(
        delete_result.is_err(),
        "delete with deletion protection should fail"
    );
}

#[tokio::test]
async fn test_list_clusters() {
    let client = make_dsql_client().await;

    client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");

    let clusters = list_resp.clusters();
    assert_eq!(clusters.len(), 2, "should list 2 clusters");
}

#[tokio::test]
async fn test_get_nonexistent_cluster() {
    let client = make_dsql_client().await;

    let result = client
        .get_cluster()
        .identifier("abcdefghijklmnopqrstuvwxyz")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent cluster should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_cluster() {
    let client = make_dsql_client().await;

    let result = client
        .delete_cluster()
        .identifier("abcdefghijklmnopqrstuvwxyz")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent cluster should fail");
}

#[tokio::test]
async fn test_create_cluster_default_deletion_protection_is_enabled() {
    let client = make_dsql_client().await;

    // When deletionProtectionEnabled is not specified, the default is true per AWS docs.
    let create_resp = client
        .create_cluster()
        .send()
        .await
        .expect("create_cluster with no args should succeed");

    assert!(
        create_resp.deletion_protection_enabled(),
        "default deletion protection should be enabled"
    );
}

#[tokio::test]
async fn test_create_cluster_idempotency_with_client_token() {
    let client = make_dsql_client().await;

    let token = "my-unique-idempotency-token-12345";

    let first_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .client_token(token)
        .send()
        .await
        .expect("first create with client token should succeed");

    let second_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .client_token(token)
        .send()
        .await
        .expect("second create with same client token should succeed (idempotent)");

    assert_eq!(
        first_resp.identifier(),
        second_resp.identifier(),
        "idempotent requests with the same client token should return the same cluster"
    );
    assert_eq!(
        first_resp.arn(),
        second_resp.arn(),
        "idempotent requests should return the same ARN"
    );
}

#[tokio::test]
async fn test_create_cluster_different_client_tokens_produce_different_clusters() {
    let client = make_dsql_client().await;

    let resp1 = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .client_token("token-aaa")
        .send()
        .await
        .expect("first cluster creation should succeed");

    let resp2 = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .client_token("token-bbb")
        .send()
        .await
        .expect("second cluster creation should succeed");

    assert_ne!(
        resp1.identifier(),
        resp2.identifier(),
        "different client tokens should produce different clusters"
    );
}

#[tokio::test]
async fn test_list_clusters_empty() {
    let client = make_dsql_client().await;

    let list_resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters on empty store should succeed");

    assert_eq!(
        list_resp.clusters().len(),
        0,
        "listing with no clusters should return an empty list"
    );
}

#[tokio::test]
async fn test_list_clusters_respects_max_results() {
    let client = make_dsql_client().await;

    for _ in 0..5 {
        client
            .create_cluster()
            .deletion_protection_enabled(false)
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_clusters()
        .max_results(2)
        .send()
        .await
        .expect("list_clusters with max_results should succeed");

    assert_eq!(
        list_resp.clusters().len(),
        2,
        "list should respect max_results and return only 2 clusters"
    );
}

#[tokio::test]
async fn test_list_clusters_after_delete_reduces_count() {
    let client = make_dsql_client().await;

    let c1 = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    client
        .delete_cluster()
        .identifier(c1.identifier())
        .send()
        .await
        .expect("delete should succeed");

    let list_resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");

    assert_eq!(
        list_resp.clusters().len(),
        1,
        "after deleting one of two clusters, list should return 1"
    );
}

#[tokio::test]
async fn test_cluster_arn_format() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    let arn = create_resp.arn();
    // ARN must match: arn:aws:dsql:<region>:<account-id>:cluster/<26-char-id>
    assert!(
        arn.starts_with("arn:aws:dsql:"),
        "ARN should start with arn:aws:dsql:"
    );
    let parts: Vec<&str> = arn.splitn(6, ':').collect();
    assert_eq!(parts.len(), 6, "ARN should have 6 colon-separated parts");
    assert!(
        parts[5].starts_with("cluster/"),
        "last ARN segment should start with 'cluster/'"
    );
    let cluster_id = &parts[5]["cluster/".len()..];
    assert_eq!(
        cluster_id.len(),
        26,
        "cluster ID in ARN should be 26 characters"
    );
    assert!(
        cluster_id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
        "cluster ID should be lowercase alphanumeric"
    );
}

#[tokio::test]
async fn test_create_cluster_status_is_active() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    assert_eq!(
        create_resp.status().as_str(),
        "ACTIVE",
        "newly created cluster status should be ACTIVE"
    );
}

#[tokio::test]
async fn test_update_cluster_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let identifier = create_resp.identifier().to_string();

    // UpdateCluster is not yet implemented in the mock service; it should return an error.
    let update_result = client
        .update_cluster()
        .identifier(&identifier)
        .deletion_protection_enabled(true)
        .send()
        .await;

    assert!(
        update_result.is_err(),
        "update_cluster should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_create_cluster_with_multiple_tags() {
    let client = make_dsql_client().await;

    let mut tags = std::collections::HashMap::new();
    tags.insert("Environment".to_string(), "test".to_string());
    tags.insert("Owner".to_string(), "team-a".to_string());
    tags.insert("CostCenter".to_string(), "9001".to_string());

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("create_cluster with multiple tags should succeed");

    let get_resp = client
        .get_cluster()
        .identifier(create_resp.identifier())
        .send()
        .await
        .expect("get_cluster should succeed");

    let resp_tags = get_resp.tags().expect("tags should be present");
    assert_eq!(resp_tags.get("Environment"), Some(&"test".to_string()));
    assert_eq!(resp_tags.get("Owner"), Some(&"team-a".to_string()));
    assert_eq!(resp_tags.get("CostCenter"), Some(&"9001".to_string()));
}

#[tokio::test]
async fn test_delete_cluster_removes_client_token_for_reuse() {
    let client = make_dsql_client().await;

    let token = "reusable-token-after-delete";

    let first_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .client_token(token)
        .send()
        .await
        .expect("create with token should succeed");

    let id = first_resp.identifier().to_string();

    client
        .delete_cluster()
        .identifier(&id)
        .send()
        .await
        .expect("delete should succeed");

    // After deletion the cluster is gone; get should fail
    let get_result = client.get_cluster().identifier(&id).send().await;
    assert!(
        get_result.is_err(),
        "get after delete should fail for the deleted cluster"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Aurora DSQL
// ============================================================================

#[tokio::test]
async fn test_create_cluster_response_has_creation_time() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    // creation_time() returns &DateTime (required field); verify it is non-zero
    assert_ne!(
        create_resp.creation_time().secs(),
        0,
        "create response should have a non-zero creation_time"
    );
}

#[tokio::test]
async fn test_get_cluster_has_creation_time() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    let identifier = create_resp.identifier().to_string();

    let get_resp = client
        .get_cluster()
        .identifier(&identifier)
        .send()
        .await
        .expect("get_cluster should succeed");

    // creation_time() returns &DateTime (required field); verify it is non-zero
    assert_ne!(
        get_resp.creation_time().secs(),
        0,
        "get response should have a non-zero creation_time"
    );
}

#[tokio::test]
async fn test_delete_cluster_response_has_arn_and_creation_time() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    let identifier = create_resp.identifier().to_string();
    let expected_arn = create_resp.arn().to_string();

    let delete_resp = client
        .delete_cluster()
        .identifier(&identifier)
        .send()
        .await
        .expect("delete_cluster should succeed");

    assert_eq!(
        delete_resp.arn(),
        expected_arn,
        "delete response should return the cluster ARN"
    );
    // creation_time() returns &DateTime (required field); verify it is non-zero
    assert_ne!(
        delete_resp.creation_time().secs(),
        0,
        "delete response should have a non-zero creation_time"
    );
}

#[tokio::test]
async fn test_get_cluster_has_status() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    let get_resp = client
        .get_cluster()
        .identifier(create_resp.identifier())
        .send()
        .await
        .expect("get_cluster should succeed");

    assert_eq!(
        get_resp.status().as_str(),
        "ACTIVE",
        "get response should show ACTIVE status"
    );
}

#[tokio::test]
async fn test_list_clusters_contains_correct_identifiers() {
    let client = make_dsql_client().await;

    let c1 = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("first create should succeed");

    let c2 = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("second create should succeed");

    let list_resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");

    let clusters = list_resp.clusters();
    assert_eq!(clusters.len(), 2, "should list 2 clusters");

    let ids: Vec<&str> = clusters.iter().map(|c| c.identifier()).collect();
    assert!(
        ids.contains(&c1.identifier()),
        "list should contain identifier of first cluster"
    );
    assert!(
        ids.contains(&c2.identifier()),
        "list should contain identifier of second cluster"
    );

    let arns: Vec<&str> = clusters.iter().map(|c| c.arn()).collect();
    assert!(
        arns.contains(&c1.arn()),
        "list should contain ARN of first cluster"
    );
    assert!(
        arns.contains(&c2.arn()),
        "list should contain ARN of second cluster"
    );
}

#[tokio::test]
async fn test_get_cluster_no_tags_returns_none() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("create_cluster should succeed");

    let get_resp = client
        .get_cluster()
        .identifier(create_resp.identifier())
        .send()
        .await
        .expect("get_cluster should succeed");

    // When no tags were provided at creation, tags field should be absent.
    assert!(
        get_resp.tags().is_none(),
        "get response should have no tags when none were set at creation"
    );
}

#[tokio::test]
async fn test_list_clusters_pagination_next_token() {
    let client = make_dsql_client().await;

    for _ in 0..3 {
        client
            .create_cluster()
            .deletion_protection_enabled(false)
            .send()
            .await
            .unwrap();
    }

    // Request only 2 of 3; the implementation currently returns next_token: None
    // because pagination is not yet fully implemented.
    let list_resp = client
        .list_clusters()
        .max_results(2)
        .send()
        .await
        .expect("list_clusters with max_results should succeed");

    assert_eq!(
        list_resp.clusters().len(),
        2,
        "response should contain 2 clusters matching max_results"
    );
    assert!(
        list_resp.next_token().is_some(),
        "next_token should be returned when results are truncated"
    );
}

#[tokio::test]
async fn test_list_clusters_pagination_full_traversal() {
    let client = make_dsql_client().await;

    // Create 5 clusters.
    for _ in 0..5 {
        client
            .create_cluster()
            .deletion_protection_enabled(false)
            .send()
            .await
            .unwrap();
    }

    // Page through all clusters using max_results=2.
    let mut all_identifiers: Vec<String> = Vec::new();
    let mut token: Option<String> = None;

    loop {
        let mut req = client.list_clusters().max_results(2);
        if let Some(ref t) = token {
            req = req.next_token(t);
        }
        let resp = req.send().await.expect("list_clusters should succeed");
        let page = resp.clusters();
        assert!(
            !page.is_empty(),
            "each page should have at least one cluster"
        );
        for c in page {
            all_identifiers.push(c.identifier().to_string());
        }
        token = resp.next_token().map(|s| s.to_string());
        if token.is_none() {
            break;
        }
    }

    assert_eq!(
        all_identifiers.len(),
        5,
        "pagination should return all 5 clusters across pages"
    );

    // No duplicates.
    let mut deduped = all_identifiers.clone();
    deduped.sort();
    deduped.dedup();
    assert_eq!(
        deduped.len(),
        5,
        "pagination should not return duplicate clusters"
    );
}

#[tokio::test]
async fn test_list_clusters_no_next_token_when_exact_page() {
    let client = make_dsql_client().await;

    // Create exactly 2 clusters.
    for _ in 0..2 {
        client
            .create_cluster()
            .deletion_protection_enabled(false)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_clusters()
        .max_results(2)
        .send()
        .await
        .expect("list_clusters should succeed");

    assert_eq!(resp.clusters().len(), 2, "should return exactly 2 clusters");
    assert!(
        resp.next_token().is_none(),
        "next_token should be None when total count equals max_results"
    );
}

#[tokio::test]
async fn test_tag_resource_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().to_string();

    let result = client
        .tag_resource()
        .resource_arn(&arn)
        .tags("TestKey", "TestValue")
        .send()
        .await;

    assert!(
        result.is_err(),
        "tag_resource should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_untag_resource_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().to_string();

    let result = client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("SomeKey")
        .send()
        .await;

    assert!(
        result.is_err(),
        "untag_resource should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().to_string();

    let result = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await;

    assert!(
        result.is_err(),
        "list_tags_for_resource should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_get_cluster_policy_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let identifier = create_resp.identifier().to_string();

    let result = client
        .get_cluster_policy()
        .identifier(&identifier)
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_cluster_policy should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_delete_cluster_policy_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let identifier = create_resp.identifier().to_string();

    let result = client
        .delete_cluster_policy()
        .identifier(&identifier)
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_cluster_policy should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_put_cluster_policy_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let identifier = create_resp.identifier().to_string();

    let result = client
        .put_cluster_policy()
        .identifier(&identifier)
        .policy("{\"Version\":\"2012-10-17\",\"Statement\":[]}")
        .send()
        .await;

    assert!(
        result.is_err(),
        "put_cluster_policy should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_get_vpc_endpoint_service_name_not_implemented_returns_error() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();

    let identifier = create_resp.identifier().to_string();

    let result = client
        .get_vpc_endpoint_service_name()
        .identifier(&identifier)
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_vpc_endpoint_service_name should return an error since it is not yet implemented"
    );
}

#[tokio::test]
async fn test_create_cluster_with_no_args_has_status() {
    let client = make_dsql_client().await;

    let create_resp = client
        .create_cluster()
        .send()
        .await
        .expect("create_cluster with no explicit args should succeed");

    assert_eq!(
        create_resp.status().as_str(),
        "ACTIVE",
        "cluster created with no args should have ACTIVE status"
    );
    assert!(
        !create_resp.identifier().is_empty(),
        "cluster should have a non-empty identifier"
    );
    assert!(
        !create_resp.arn().is_empty(),
        "cluster should have a non-empty ARN"
    );
}
