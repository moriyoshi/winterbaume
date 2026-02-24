/// End-to-end scenario tests for OpenSearch Serverless.
///
/// Each test exercises a complete use-case workflow chaining 3 or more operations
/// and asserts business outcomes rather than individual API return shapes.
use aws_sdk_opensearchserverless::config::BehaviorVersion;
use aws_sdk_opensearchserverless::types::{CollectionType, SecurityPolicyType, Tag};
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

/// Scenario: A team provisions a vector-search collection protected by an encryption
/// policy, tags it for cost allocation, and then verifies discovery via batch-get.
///
/// Operations: CreateSecurityPolicy → CreateCollection (with tags) → BatchGetCollection
/// → ListTagsForResource → UntagResource → DeleteCollection
#[tokio::test]
async fn test_secure_collection_provisioning_workflow() {
    // Scenario: team provisioning a secured, tagged collection end-to-end
    let client = make_client().await;

    // Step 1: Create an encryption security policy for the collection
    let policy_resp = client
        .create_security_policy()
        .name("team-enc-policy")
        .r#type(SecurityPolicyType::Encryption)
        .policy(
            r#"{"Rules":[{"ResourceType":"collection","Resource":["collection/team-*"]}],"AWSOwnedKey":true}"#,
        )
        .description("Team encryption policy")
        .send()
        .await
        .expect("create_security_policy should succeed");

    let policy_detail = policy_resp
        .security_policy_detail()
        .expect("should have policy detail");
    assert_eq!(policy_detail.name(), Some("team-enc-policy"));
    let _policy_version = policy_detail
        .policy_version()
        .expect("should have policy version")
        .to_string();

    // Step 2: Create a vector-search collection
    let create_resp = client
        .create_collection()
        .name("team-vectors")
        .r#type(CollectionType::Vectorsearch)
        .description("Team vector search collection")
        .tags(
            Tag::builder()
                .key("team")
                .value("ml-platform")
                .build()
                .unwrap(),
        )
        .tags(
            Tag::builder()
                .key("cost-centre")
                .value("ai-research")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_collection should succeed");

    let detail = create_resp
        .create_collection_detail()
        .expect("should have detail");
    assert_eq!(detail.name(), Some("team-vectors"));
    assert_eq!(
        detail.r#type().map(|t| t.as_str()),
        Some("VECTORSEARCH"),
        "collection type should be VECTORSEARCH"
    );
    let collection_id = detail.id().expect("should have id").to_string();
    let collection_arn = detail.arn().expect("should have ARN").to_string();

    // Step 3: Verify the collection is discoverable via batch-get
    let batch_resp = client
        .batch_get_collection()
        .ids(&collection_id)
        .send()
        .await
        .expect("batch_get_collection should succeed");

    assert_eq!(
        batch_resp.collection_details().len(),
        1,
        "batch-get should return the created collection"
    );
    assert_eq!(
        batch_resp.collection_details()[0].id(),
        Some(collection_id.as_str())
    );

    // Step 4: Verify tags were persisted (tags passed to create_collection
    // are stored in TagStore by ARN)
    let tag_resp = client
        .list_tags_for_resource()
        .resource_arn(&collection_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tag_resp.tags();
    assert_eq!(tags.len(), 2, "should have 2 cost-allocation tags");
    assert!(
        tags.iter()
            .any(|t| t.key() == "team" && t.value() == "ml-platform"),
        "team tag should be present"
    );
    assert!(
        tags.iter()
            .any(|t| t.key() == "cost-centre" && t.value() == "ai-research"),
        "cost-centre tag should be present"
    );

    // Step 5: Remove the cost-centre tag (policy change — no longer tracked)
    client
        .untag_resource()
        .resource_arn(&collection_arn)
        .tag_keys("cost-centre")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tag_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&collection_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tag_resp2.tags().len(), 1, "only team tag should remain");

    // Step 6: Decommission the collection
    client
        .delete_collection()
        .id(&collection_id)
        .send()
        .await
        .expect("delete_collection should succeed");

    // Business outcome: collection is gone, list returns empty
    let list_resp = client.list_collections().send().await.unwrap();
    assert!(
        !list_resp
            .collection_summaries()
            .iter()
            .any(|c| c.id() == Some(collection_id.as_str())),
        "deleted collection must not appear in list"
    );
}

/// Scenario: A developer creates multiple collections of different types and applies
/// both encryption and network security policies, then verifies policy versioning
/// is enforced on update.
///
/// Operations: CreateCollection (×2) → CreateSecurityPolicy (×2) → GetSecurityPolicy
/// → UpdateSecurityPolicy (version check) → ListSecurityPolicies → BatchGetCollection
#[tokio::test]
async fn test_multi_collection_policy_management_workflow() {
    // Scenario: managing multiple collections with independent security policies
    let client = make_client().await;

    // Step 1: Create two collections of different types
    let search_resp = client
        .create_collection()
        .name("search-store")
        .r#type(CollectionType::Search)
        .send()
        .await
        .expect("create search collection should succeed");
    let search_id = search_resp
        .create_collection_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let timeseries_resp = client
        .create_collection()
        .name("metrics-store")
        .r#type(CollectionType::Timeseries)
        .send()
        .await
        .expect("create timeseries collection should succeed");
    let ts_id = timeseries_resp
        .create_collection_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Step 2: Create encryption and network policies for both collections
    client
        .create_security_policy()
        .name("stores-enc")
        .r#type(SecurityPolicyType::Encryption)
        .policy(r#"{"Rules":[{"ResourceType":"collection","Resource":["collection/*-store"]}],"AWSOwnedKey":true}"#)
        .send()
        .await
        .expect("create encryption policy should succeed");

    let net_resp = client
        .create_security_policy()
        .name("stores-net")
        .r#type(SecurityPolicyType::Network)
        .policy(
            r#"[{"Rules":[{"ResourceType":"collection","Resource":["collection/*-store"]}],"AllowFromPublic":true}]"#,
        )
        .send()
        .await
        .expect("create network policy should succeed");
    let net_version = net_resp
        .security_policy_detail()
        .unwrap()
        .policy_version()
        .unwrap()
        .to_string();

    // Step 3: Retrieve a policy and verify its content
    let get_resp = client
        .get_security_policy()
        .name("stores-enc")
        .r#type(SecurityPolicyType::Encryption)
        .send()
        .await
        .expect("get_security_policy should succeed");
    let enc_detail = get_resp.security_policy_detail().unwrap();
    assert_eq!(enc_detail.name(), Some("stores-enc"));

    // Step 4: Update the network policy description (version must match)
    let upd_resp = client
        .update_security_policy()
        .name("stores-net")
        .r#type(SecurityPolicyType::Network)
        .policy_version(&net_version)
        .description("Restricted to internal VPC only")
        .send()
        .await
        .expect("update_security_policy should succeed");
    let new_version = upd_resp
        .security_policy_detail()
        .unwrap()
        .policy_version()
        .unwrap()
        .to_string();
    assert_ne!(
        new_version, net_version,
        "policy version must be bumped after update"
    );

    // Step 5: Verify stale version is rejected
    let stale_result = client
        .update_security_policy()
        .name("stores-net")
        .r#type(SecurityPolicyType::Network)
        .policy_version(&net_version) // stale version
        .description("This should fail")
        .send()
        .await;
    assert!(
        stale_result.is_err(),
        "update with stale version must be rejected"
    );

    // Step 6: List policies and confirm both exist
    let enc_list = client
        .list_security_policies()
        .r#type(SecurityPolicyType::Encryption)
        .send()
        .await
        .unwrap();
    assert_eq!(enc_list.security_policy_summaries().len(), 1);

    let net_list = client
        .list_security_policies()
        .r#type(SecurityPolicyType::Network)
        .send()
        .await
        .unwrap();
    assert_eq!(net_list.security_policy_summaries().len(), 1);

    // Step 7: Batch-get both collections to confirm both are alive
    let batch_resp = client
        .batch_get_collection()
        .ids(&search_id)
        .ids(&ts_id)
        .send()
        .await
        .expect("batch_get_collection should succeed");
    assert_eq!(
        batch_resp.collection_details().len(),
        2,
        "both collections should be retrievable"
    );
}
