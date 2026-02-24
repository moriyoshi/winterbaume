use aws_sdk_ram::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ram::RamService;

const ACCOUNT_ID: &str = "123456789012";

async fn make_client() -> aws_sdk_ram::Client {
    let mock = MockAws::builder().with_service(RamService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ram::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_ram::Client::new(&config)
}

/// Helper to assert that an SDK error result failed (regardless of specific error variant).
fn assert_error_contains(err: &dyn std::fmt::Debug, substring: &str) {
    let debug_str = format!("{:?}", err);
    assert!(
        debug_str.contains(substring),
        "expected error to contain '{}', got: {}",
        substring,
        debug_str
    );
}

// ==================== test_create_resource_share ====================

#[tokio::test]
async fn test_create_resource_share() {
    let client = make_client().await;

    // Create a simple resource share
    let response = client
        .create_resource_share()
        .name("test")
        .send()
        .await
        .expect("create_resource_share should succeed");

    let resource = response
        .resource_share()
        .expect("should have resource_share");
    assert_eq!(resource.allow_external_principals(), Some(true));
    assert!(resource.creation_time().is_some());
    assert!(resource.last_updated_time().is_some());
    assert_eq!(resource.name(), Some("test"));
    assert_eq!(resource.owning_account_id(), Some(ACCOUNT_ID));
    let arn = resource.resource_share_arn().unwrap();
    assert!(
        arn.starts_with("arn:aws:ram:us-east-1:123456789012:resource-share/"),
        "ARN should match expected pattern, got: {}",
        arn
    );
    assert_eq!(resource.status().map(|s| s.as_str()), Some("ACTIVE"));
    // create_resource_share response should NOT include featureSet
    assert!(resource.feature_set().is_none());

    // Creating a resource share with the same name should result in a second one
    let response2 = client
        .create_resource_share()
        .name("test")
        .allow_external_principals(false)
        .resource_arns(format!(
            "arn:aws:ec2:us-east-1:{}:transit-gateway/tgw-123456789",
            ACCOUNT_ID
        ))
        .send()
        .await
        .expect("second create_resource_share should succeed");

    let resource2 = response2.resource_share().unwrap();
    assert_eq!(resource2.allow_external_principals(), Some(false));
    assert!(resource2.creation_time().is_some());
    assert!(resource2.last_updated_time().is_some());
    assert_eq!(resource2.name(), Some("test"));
    assert_eq!(resource2.owning_account_id(), Some(ACCOUNT_ID));
    assert_eq!(resource2.status().map(|s| s.as_str()), Some("ACTIVE"));

    // Both shares should exist
    let get_resp = client
        .get_resource_shares()
        .resource_owner("SELF".into())
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.resource_shares().len(), 2);
}

// ==================== test_create_resource_share_errors ====================

#[tokio::test]
async fn test_create_resource_share_error_invalid_arn() {
    let client = make_client().await;

    let err = client
        .create_resource_share()
        .name("test")
        .resource_arns("inalid-arn")
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(
        &err,
        "The specified resource ARN inalid-arn is not valid. Verify the ARN and try again.",
    );
}

#[tokio::test]
async fn test_create_resource_share_error_non_shareable_type() {
    let client = make_client().await;

    let err = client
        .create_resource_share()
        .name("test")
        .resource_arns(format!("arn:aws:iam::{}:role/test", ACCOUNT_ID))
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(&err, "You cannot share the selected resource type.");
}

#[tokio::test]
async fn test_create_resource_share_error_invalid_principal() {
    let client = make_client().await;

    let err = client
        .create_resource_share()
        .name("test")
        .principals("invalid")
        .resource_arns(format!(
            "arn:aws:ec2:us-east-1:{}:transit-gateway/tgw-123456789",
            ACCOUNT_ID
        ))
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(
        &err,
        "Principal ID invalid is malformed. Verify the ID and try again.",
    );
}

// ==================== test_get_resource_shares ====================

#[tokio::test]
async fn test_get_resource_shares() {
    let client = make_client().await;
    client
        .create_resource_share()
        .name("test")
        .send()
        .await
        .unwrap();

    let response = client
        .get_resource_shares()
        .resource_owner("SELF".into())
        .send()
        .await
        .unwrap();

    assert_eq!(response.resource_shares().len(), 1);
    let resource = &response.resource_shares()[0];
    assert_eq!(resource.allow_external_principals(), Some(true));
    assert!(resource.creation_time().is_some());
    // get_resource_shares should include featureSet = STANDARD
    assert_eq!(resource.feature_set().map(|f| f.as_str()), Some("STANDARD"));
    assert!(resource.last_updated_time().is_some());
    assert_eq!(resource.name(), Some("test"));
    assert_eq!(resource.owning_account_id(), Some(ACCOUNT_ID));
    let arn = resource.resource_share_arn().unwrap();
    assert!(arn.starts_with("arn:aws:ram:us-east-1:123456789012:resource-share/"));
    assert_eq!(resource.status().map(|s| s.as_str()), Some("ACTIVE"));
}

// ==================== test_get_resource_shares_errors ====================

#[tokio::test]
async fn test_get_resource_shares_error_invalid_owner() {
    let client = make_client().await;

    let err = client
        .get_resource_shares()
        .resource_owner("invalid".into())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(
        &err,
        "invalid is not a valid resource owner. Specify either SELF or OTHER-ACCOUNTS and try again.",
    );
}

// ==================== test_update_resource_share ====================

#[tokio::test]
async fn test_update_resource_share() {
    let client = make_client().await;

    let create_resp = client
        .create_resource_share()
        .name("test")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    // Small delay to ensure lastUpdatedTime > creationTime
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let response = client
        .update_resource_share()
        .resource_share_arn(&arn)
        .name("test-update")
        .send()
        .await
        .unwrap();

    let resource = response.resource_share().unwrap();
    assert_eq!(resource.allow_external_principals(), Some(true));
    assert_eq!(resource.name(), Some("test-update"));
    assert_eq!(resource.owning_account_id(), Some(ACCOUNT_ID));
    assert!(
        resource
            .resource_share_arn()
            .unwrap()
            .starts_with("arn:aws:ram:us-east-1:")
    );
    assert_eq!(resource.status().map(|s| s.as_str()), Some("ACTIVE"));
    // update response should NOT include featureSet
    assert!(resource.feature_set().is_none());

    // Verify only one share exists
    let get_resp = client
        .get_resource_shares()
        .resource_owner("SELF".into())
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.resource_shares().len(), 1);
}

// ==================== test_update_resource_share_errors ====================

#[tokio::test]
async fn test_update_resource_share_error_unknown() {
    let client = make_client().await;

    let arn = format!(
        "arn:aws:ram:us-east-1:{}:resource-share/not-existing",
        ACCOUNT_ID
    );

    let err = client
        .update_resource_share()
        .resource_share_arn(&arn)
        .name("test-update")
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(&err, &format!("ResourceShare {} could not be found.", arn));
}

// ==================== test_delete_resource_share ====================

#[tokio::test]
async fn test_delete_resource_share() {
    let client = make_client().await;

    let create_resp = client
        .create_resource_share()
        .name("test")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    let response = client
        .delete_resource_share()
        .resource_share_arn(&arn)
        .send()
        .await
        .unwrap();

    assert_eq!(response.return_value(), Some(true));

    // The share should still exist but with DELETED status
    let get_resp = client
        .get_resource_shares()
        .resource_owner("SELF".into())
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.resource_shares().len(), 1);
    let resource = &get_resp.resource_shares()[0];
    assert_eq!(resource.status().map(|s| s.as_str()), Some("DELETED"));
}

// ==================== test_delete_resource_share_errors ====================

#[tokio::test]
async fn test_delete_resource_share_error_unknown() {
    let client = make_client().await;

    let arn = format!(
        "arn:aws:ram:us-east-1:{}:resource-share/not-existing",
        ACCOUNT_ID
    );

    let err = client
        .delete_resource_share()
        .resource_share_arn(&arn)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(&err, &format!("ResourceShare {} could not be found.", arn));
}

// ==================== test_enable_sharing_with_aws_organization ====================

#[tokio::test]
async fn test_enable_sharing_with_aws_organization() {
    let client = make_client().await;

    let response = client
        .enable_sharing_with_aws_organization()
        .send()
        .await
        .expect("enable_sharing_with_aws_organization should succeed");

    assert_eq!(response.return_value(), Some(true));
}

// ==================== test_get_resource_share_associations_with_principals ====================

#[tokio::test]
async fn test_get_resource_share_associations_with_principals() {
    let client = make_client().await;

    let response = client
        .create_resource_share()
        .name("test")
        .principals("123456789012")
        .resource_arns(format!(
            "arn:aws:ec2:us-east-1:{}:transit-gateway/tgw-123456789",
            ACCOUNT_ID
        ))
        .send()
        .await
        .unwrap();
    let resource_share_arn = response
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    let assoc_resp = client
        .get_resource_share_associations()
        .association_type("PRINCIPAL".into())
        .resource_share_arns(&resource_share_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(assoc_resp.resource_share_associations().len(), 1);
    let association = &assoc_resp.resource_share_associations()[0];
    assert_eq!(
        association.resource_share_arn(),
        Some(resource_share_arn.as_str())
    );
    assert_eq!(association.resource_share_name(), Some("test"));
    assert_eq!(association.associated_entity(), Some("123456789012"));
    assert_eq!(
        association.association_type().map(|t| t.as_str()),
        Some("PRINCIPAL")
    );
    assert_eq!(association.status().map(|s| s.as_str()), Some("ASSOCIATED"));
    assert!(association.creation_time().is_some());
    assert!(association.last_updated_time().is_some());
    assert_eq!(association.external(), Some(false));
}

// ==================== test_get_resource_share_associations_with_resources ====================

#[tokio::test]
async fn test_get_resource_share_associations_with_resources() {
    let client = make_client().await;

    let resource_arn = format!(
        "arn:aws:ec2:us-east-1:{}:transit-gateway/tgw-123456789",
        ACCOUNT_ID
    );
    let response = client
        .create_resource_share()
        .name("test")
        .principals("123456789012")
        .resource_arns(&resource_arn)
        .send()
        .await
        .unwrap();
    let resource_share_arn = response
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    let assoc_resp = client
        .get_resource_share_associations()
        .association_type("RESOURCE".into())
        .resource_share_arns(&resource_share_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(assoc_resp.resource_share_associations().len(), 1);
    let association = &assoc_resp.resource_share_associations()[0];
    assert_eq!(
        association.resource_share_arn(),
        Some(resource_share_arn.as_str())
    );
    assert_eq!(association.resource_share_name(), Some("test"));
    assert_eq!(association.associated_entity(), Some(resource_arn.as_str()));
    assert_eq!(
        association.association_type().map(|t| t.as_str()),
        Some("RESOURCE")
    );
    assert_eq!(association.status().map(|s| s.as_str()), Some("ASSOCIATED"));
    assert!(association.creation_time().is_some());
    assert!(association.last_updated_time().is_some());
    assert_eq!(association.external(), Some(false));
}

// ==================== test_get_resource_share_associations_with_filters ====================

#[tokio::test]
async fn test_get_resource_share_associations_with_filters() {
    let client = make_client().await;

    let resource_arn = format!(
        "arn:aws:ec2:us-east-1:{}:transit-gateway/tgw-123456789",
        ACCOUNT_ID
    );
    client
        .create_resource_share()
        .name("test")
        .principals("123456789012")
        .resource_arns(&resource_arn)
        .send()
        .await
        .unwrap();

    // Filter by principal
    let resp = client
        .get_resource_share_associations()
        .association_type("PRINCIPAL".into())
        .principal("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.resource_share_associations().len(), 1);
    assert_eq!(
        resp.resource_share_associations()[0].associated_entity(),
        Some("123456789012")
    );

    // Filter by resource ARN
    let resp = client
        .get_resource_share_associations()
        .association_type("RESOURCE".into())
        .resource_arn(&resource_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.resource_share_associations().len(), 1);
    assert_eq!(
        resp.resource_share_associations()[0].associated_entity(),
        Some(resource_arn.as_str())
    );
}

// ==================== test_get_resource_share_associations_errors ====================

#[tokio::test]
async fn test_get_resource_share_associations_error_invalid_type() {
    let client = make_client().await;

    let err = client
        .get_resource_share_associations()
        .association_type("INVALID".into())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(&err, "is not a valid association type");
}

#[tokio::test]
async fn test_get_resource_share_associations_error_invalid_status() {
    let client = make_client().await;

    let err = client
        .get_resource_share_associations()
        .association_type("PRINCIPAL".into())
        .association_status("INVALID".into())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(&err, "is not a valid association status");
}

#[tokio::test]
async fn test_get_resource_share_associations_error_resource_arn_with_principal_type() {
    let client = make_client().await;

    let resource_arn = format!(
        "arn:aws:ec2:us-east-1:{}:transit-gateway/tgw-123456789",
        ACCOUNT_ID
    );

    let err = client
        .get_resource_share_associations()
        .association_type("PRINCIPAL".into())
        .resource_arn(&resource_arn)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(
        &err,
        "You cannot specify a resource ARN when the association type is PRINCIPAL",
    );
}

#[tokio::test]
async fn test_get_resource_share_associations_error_principal_with_resource_type() {
    let client = make_client().await;

    let err = client
        .get_resource_share_associations()
        .association_type("RESOURCE".into())
        .principal("123456789012")
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(
        &err,
        "You cannot specify a principal when the association type is RESOURCE",
    );
}

// ==================== test_list_resource_types ====================

#[tokio::test]
async fn test_list_resource_types_default() {
    let client = make_client().await;

    let response = client.list_resource_types().send().await.unwrap();

    let resource_types = response.resource_types();
    assert!(!resource_types.is_empty(), "should have resource types");
    // Check that known types are present
    let has_subnet = resource_types
        .iter()
        .any(|rt| rt.resource_type() == Some("ec2:Subnet"));
    assert!(has_subnet, "should include ec2:Subnet resource type");

    let has_transit_gw_multicast = resource_types
        .iter()
        .any(|rt| rt.resource_type() == Some("ec2:TransitGatewayMulticastDomain"));
    assert!(
        has_transit_gw_multicast,
        "should include ec2:TransitGatewayMulticastDomain"
    );
}

#[tokio::test]
async fn test_list_resource_types_all_scope() {
    let client = make_client().await;

    let response = client
        .list_resource_types()
        .resource_region_scope("ALL".into())
        .send()
        .await
        .unwrap();

    // ALL should return the same count as default
    let default_resp = client.list_resource_types().send().await.unwrap();
    assert_eq!(
        response.resource_types().len(),
        default_resp.resource_types().len()
    );
}

#[tokio::test]
async fn test_list_resource_types_global_scope() {
    let client = make_client().await;

    let response = client
        .list_resource_types()
        .resource_region_scope("GLOBAL".into())
        .send()
        .await
        .unwrap();

    let resource_types = response.resource_types();
    // All returned types should have GLOBAL scope
    for rt in resource_types {
        assert_eq!(
            rt.resource_region_scope().map(|s| s.as_str()),
            Some("GLOBAL"),
            "expected GLOBAL scope for {:?}",
            rt.resource_type()
        );
    }
}

#[tokio::test]
async fn test_list_resource_types_regional_scope() {
    let client = make_client().await;

    let response = client
        .list_resource_types()
        .resource_region_scope("REGIONAL".into())
        .send()
        .await
        .unwrap();

    let resource_types = response.resource_types();
    // All returned types should have REGIONAL scope
    for rt in resource_types {
        assert_eq!(
            rt.resource_region_scope().map(|s| s.as_str()),
            Some("REGIONAL"),
            "expected REGIONAL scope for {:?}",
            rt.resource_type()
        );
    }
}

#[tokio::test]
async fn test_list_resource_types_invalid_scope() {
    let client = make_client().await;

    let err = client
        .list_resource_types()
        .resource_region_scope("INVALID".into())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(
        &err,
        "INVALID is not a valid resource region scope value. Specify a valid value and try again.",
    );
}

// ==================== test_list_permissions ====================

#[tokio::test]
async fn test_list_permissions_default() {
    let client = make_client().await;

    let response = client.list_permissions().send().await.unwrap();

    let permissions = response.permissions();
    assert!(
        !permissions.is_empty(),
        "should have at least one permission"
    );

    // Check that each permission has expected fields
    let first = &permissions[0];
    assert!(first.arn().is_some());
    assert!(first.name().is_some());
    assert!(first.resource_type().is_some());
}

#[tokio::test]
async fn test_list_permissions_by_resource_type() {
    let client = make_client().await;

    let response = client
        .list_permissions()
        .resource_type("glue:Catalog")
        .send()
        .await
        .unwrap();

    let permissions = response.permissions();
    assert!(
        !permissions.is_empty(),
        "should have glue:Catalog permissions"
    );
    // All returned permissions should be for glue:Catalog
    for perm in permissions {
        assert_eq!(
            perm.resource_type(),
            Some("glue:Catalog"),
            "expected glue:Catalog resource type"
        );
    }
}

#[tokio::test]
async fn test_list_permissions_by_permission_type_all() {
    let client = make_client().await;

    let response = client
        .list_permissions()
        .permission_type("ALL".into())
        .send()
        .await
        .unwrap();

    let permissions = response.permissions();
    assert!(
        !permissions.is_empty(),
        "ALL permission type should return permissions"
    );
}

#[tokio::test]
async fn test_list_permissions_by_resource_type_and_permission_type() {
    let client = make_client().await;

    let response = client
        .list_permissions()
        .resource_type("glue:Catalog")
        .permission_type("AWS".into())
        .send()
        .await
        .unwrap();

    let permissions = response.permissions();
    for perm in permissions {
        assert_eq!(
            perm.resource_type(),
            Some("glue:Catalog"),
            "expected glue:Catalog resource type"
        );
    }
}

#[tokio::test]
async fn test_list_permissions_error_invalid_resource_type() {
    let client = make_client().await;

    let err = client
        .list_permissions()
        .resource_type("gluE:catalog")
        .permission_type("AWS".into())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(&err, "Invalid resource type: gluE:catalog");
}

#[tokio::test]
async fn test_list_permissions_error_invalid_permission_type() {
    let client = make_client().await;

    let err = client
        .list_permissions()
        .resource_type("glue:Catalog")
        .permission_type("INVALID".into())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert_error_contains(
        &err,
        "INVALID is not a valid scope. Must be one of: ALL, AWS, LOCAL.",
    );
}

// ==================== Additional basic tests ====================

#[tokio::test]
async fn test_create_resource_share_with_resources() {
    let client = make_client().await;

    let resource_arn = format!("arn:aws:ec2:us-east-1:{}:subnet/subnet-12345", ACCOUNT_ID);

    let resp = client
        .create_resource_share()
        .name("share-with-resources")
        .resource_arns(&resource_arn)
        .send()
        .await
        .expect("create_resource_share should succeed");

    let rs = resp.resource_share().unwrap();
    let share_arn = rs.resource_share_arn().unwrap().to_string();

    let list_resp = client
        .list_resources()
        .resource_owner("SELF".into())
        .resource_share_arns(&share_arn)
        .send()
        .await
        .expect("list_resources should succeed");

    let resources = list_resp.resources();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].arn(), Some(resource_arn.as_str()));
}

#[tokio::test]
async fn test_list_resources_empty() {
    let client = make_client().await;

    let resp = client
        .list_resources()
        .resource_owner("SELF".into())
        .send()
        .await
        .expect("list_resources should succeed");

    assert!(resp.resources().is_empty());
}

// ==================== test_tag_resource / test_untag_resource ====================

#[tokio::test]
async fn test_tag_and_untag_resource() {
    let client = make_client().await;

    let create_resp = client
        .create_resource_share()
        .name("tagged-share")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    // TagResource
    client
        .tag_resource()
        .resource_share_arn(&arn)
        .tags(
            aws_sdk_ram::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify tag is present via GetResourceShares
    let get_resp = client
        .get_resource_shares()
        .resource_owner("SELF".into())
        .send()
        .await
        .unwrap();
    let share = &get_resp.resource_shares()[0];
    assert!(
        share
            .tags()
            .iter()
            .any(|t| t.key() == Some("env") && t.value() == Some("test")),
        "expected env=test tag"
    );

    // UntagResource
    client
        .untag_resource()
        .resource_share_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tag removed
    let get_resp2 = client
        .get_resource_shares()
        .resource_owner("SELF".into())
        .send()
        .await
        .unwrap();
    let share2 = &get_resp2.resource_shares()[0];
    assert!(
        !share2.tags().iter().any(|t| t.key() == Some("env")),
        "expected env tag to be removed"
    );
}

// ==================== test_associate_disassociate_resource_share ====================

#[tokio::test]
async fn test_associate_and_disassociate_resource_share() {
    let client = make_client().await;

    let create_resp = client
        .create_resource_share()
        .name("assoc-share")
        .send()
        .await
        .unwrap();
    let share_arn = create_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();
    let resource_arn = format!("arn:aws:ec2:us-east-1:{}:subnet/subnet-abcdef", ACCOUNT_ID);

    // AssociateResourceShare
    let assoc_resp = client
        .associate_resource_share()
        .resource_share_arn(&share_arn)
        .resource_arns(&resource_arn)
        .send()
        .await
        .expect("associate_resource_share should succeed");

    let assocs = assoc_resp.resource_share_associations();
    assert_eq!(assocs.len(), 1);
    assert_eq!(assocs[0].associated_entity(), Some(resource_arn.as_str()));
    assert_eq!(assocs[0].status().map(|s| s.as_str()), Some("ASSOCIATED"));

    // Verify via ListResources
    let list_resp = client
        .list_resources()
        .resource_owner("SELF".into())
        .resource_share_arns(&share_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.resources().len(), 1);

    // DisassociateResourceShare
    let disassoc_resp = client
        .disassociate_resource_share()
        .resource_share_arn(&share_arn)
        .resource_arns(&resource_arn)
        .send()
        .await
        .expect("disassociate_resource_share should succeed");

    let disassoc_assocs = disassoc_resp.resource_share_associations();
    assert_eq!(disassoc_assocs.len(), 1);
    assert_eq!(
        disassoc_assocs[0].status().map(|s| s.as_str()),
        Some("DISASSOCIATED")
    );
}

#[tokio::test]
async fn test_associate_resource_share_error_unknown() {
    let client = make_client().await;
    let err = client
        .associate_resource_share()
        .resource_share_arn(format!(
            "arn:aws:ram:us-east-1:{}:resource-share/nonexistent",
            ACCOUNT_ID
        ))
        .resource_arns(format!(
            "arn:aws:ec2:us-east-1:{}:subnet/subnet-abcdef",
            ACCOUNT_ID
        ))
        .send()
        .await;
    assert!(err.is_err());
    assert_error_contains(&err.unwrap_err(), "could not be found");
}

// ==================== test_create_and_manage_permission ====================

#[tokio::test]
async fn test_create_permission() {
    let client = make_client().await;

    let resp = client
        .create_permission()
        .name("TestPermission")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("create_permission should succeed");

    let perm = resp.permission().expect("permission should be present");
    assert!(perm.arn().is_some());
    assert_eq!(perm.name(), Some("TestPermission"));
    assert_eq!(perm.resource_type(), Some("ec2:Subnet"));
    assert_eq!(
        perm.permission_type().map(|s| s.as_str()),
        Some("CUSTOMER_MANAGED")
    );
}

#[tokio::test]
async fn test_get_permission() {
    let client = make_client().await;

    // Get a built-in permission
    let list_resp = client.list_permissions().send().await.unwrap();
    let first_perm = &list_resp.permissions()[0];
    let arn = first_perm.arn().unwrap().to_string();

    let get_resp = client
        .get_permission()
        .permission_arn(&arn)
        .send()
        .await
        .expect("get_permission should succeed");

    let detail = get_resp
        .permission()
        .expect("permission detail should be present");
    assert_eq!(detail.arn(), Some(arn.as_str()));
    assert!(detail.name().is_some());
    assert!(detail.resource_type().is_some());
}

#[tokio::test]
async fn test_get_permission_error_unknown() {
    let client = make_client().await;
    let err = client
        .get_permission()
        .permission_arn("arn:aws:ram::aws:permission/NonExistent")
        .send()
        .await;
    assert!(err.is_err());
    assert_error_contains(&err.unwrap_err(), "could not be found");
}

#[tokio::test]
async fn test_list_permission_versions() {
    let client = make_client().await;

    // Create a customer-managed permission
    let create_resp = client
        .create_permission()
        .name("VersionedPermission")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let arn = create_resp.permission().unwrap().arn().unwrap().to_string();

    let resp = client
        .list_permission_versions()
        .permission_arn(&arn)
        .send()
        .await
        .expect("list_permission_versions should succeed");

    assert!(!resp.permissions().is_empty());
    assert_eq!(resp.permissions()[0].arn(), Some(arn.as_str()));
}

#[tokio::test]
async fn test_delete_permission() {
    let client = make_client().await;

    let create_resp = client
        .create_permission()
        .name("DeleteMe")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let arn = create_resp.permission().unwrap().arn().unwrap().to_string();

    let del_resp = client
        .delete_permission()
        .permission_arn(&arn)
        .send()
        .await
        .expect("delete_permission should succeed");
    assert_eq!(del_resp.return_value(), Some(true));

    // Should be gone now
    let err = client.get_permission().permission_arn(&arn).send().await;
    assert!(err.is_err());
    assert_error_contains(&err.unwrap_err(), "could not be found");
}

#[tokio::test]
async fn test_create_permission_version() {
    let client = make_client().await;

    let create_resp = client
        .create_permission()
        .name("MultiVersionPerm")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let arn = create_resp.permission().unwrap().arn().unwrap().to_string();

    let v2_resp = client
        .create_permission_version()
        .permission_arn(&arn)
        .policy_template(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"ec2:Describe*","Resource":"*"}]}"#)
        .send()
        .await
        .expect("create_permission_version should succeed");

    let detail = v2_resp
        .permission()
        .expect("permission detail should be present");
    assert_eq!(detail.arn(), Some(arn.as_str()));
    assert_eq!(detail.version(), Some("2"));
}

// ==================== test_associate_permission_with_share ====================

#[tokio::test]
async fn test_associate_and_disassociate_resource_share_permission() {
    let client = make_client().await;

    // Create a resource share
    let share_resp = client
        .create_resource_share()
        .name("perm-share")
        .send()
        .await
        .unwrap();
    let share_arn = share_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    // Get a built-in permission ARN
    let list_resp = client.list_permissions().send().await.unwrap();
    let perm_arn = list_resp.permissions()[0].arn().unwrap().to_string();

    // AssociateResourceSharePermission
    let assoc_resp = client
        .associate_resource_share_permission()
        .resource_share_arn(&share_arn)
        .permission_arn(&perm_arn)
        .send()
        .await
        .expect("associate_resource_share_permission should succeed");
    assert_eq!(assoc_resp.return_value(), Some(true));

    // ListResourceSharePermissions
    let list_resp = client
        .list_resource_share_permissions()
        .resource_share_arn(&share_arn)
        .send()
        .await
        .expect("list_resource_share_permissions should succeed");
    assert_eq!(list_resp.permissions().len(), 1);
    assert_eq!(list_resp.permissions()[0].arn(), Some(perm_arn.as_str()));

    // DisassociateResourceSharePermission
    let disassoc_resp = client
        .disassociate_resource_share_permission()
        .resource_share_arn(&share_arn)
        .permission_arn(&perm_arn)
        .send()
        .await
        .expect("disassociate_resource_share_permission should succeed");
    assert_eq!(disassoc_resp.return_value(), Some(true));

    // ListResourceSharePermissions should now be empty
    let list_resp2 = client
        .list_resource_share_permissions()
        .resource_share_arn(&share_arn)
        .send()
        .await
        .unwrap();
    assert!(list_resp2.permissions().is_empty());
}

// ==================== test_list_permission_associations ====================

#[tokio::test]
async fn test_list_permission_associations() {
    let client = make_client().await;

    // Create resource share and associate a permission
    let share_resp = client
        .create_resource_share()
        .name("assoc-perm-share")
        .send()
        .await
        .unwrap();
    let share_arn = share_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    let list_resp = client.list_permissions().send().await.unwrap();
    let perm_arn = list_resp.permissions()[0].arn().unwrap().to_string();

    client
        .associate_resource_share_permission()
        .resource_share_arn(&share_arn)
        .permission_arn(&perm_arn)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_permission_associations()
        .permission_arn(&perm_arn)
        .send()
        .await
        .expect("list_permission_associations should succeed");

    assert_eq!(resp.permissions().len(), 1);
    assert_eq!(resp.permissions()[0].arn(), Some(perm_arn.as_str()));
}

// ==================== test_list_principals ====================

#[tokio::test]
async fn test_list_principals() {
    let client = make_client().await;

    client
        .create_resource_share()
        .name("principals-share")
        .principals("123456789012")
        .resource_arns(format!(
            "arn:aws:ec2:us-east-1:{}:subnet/subnet-12345",
            ACCOUNT_ID
        ))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_principals()
        .resource_owner("SELF".into())
        .send()
        .await
        .expect("list_principals should succeed");

    assert_eq!(resp.principals().len(), 1);
    assert_eq!(resp.principals()[0].id(), Some("123456789012"));
}

#[tokio::test]
async fn test_list_principals_error_invalid_owner() {
    let client = make_client().await;
    let err = client
        .list_principals()
        .resource_owner("INVALID".into())
        .send()
        .await;
    assert!(err.is_err());
    assert_error_contains(&err.unwrap_err(), "is not a valid resource owner");
}

// ==================== test_get_resource_policies ====================

#[tokio::test]
async fn test_get_resource_policies() {
    let client = make_client().await;

    let resource_arn = format!("arn:aws:ec2:us-east-1:{}:subnet/subnet-77777", ACCOUNT_ID);

    // Create a share with a resource so it shows up in policies
    client
        .create_resource_share()
        .name("policy-share")
        .resource_arns(&resource_arn)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_resource_policies()
        .resource_arns(&resource_arn)
        .send()
        .await
        .expect("get_resource_policies should succeed");

    assert!(!resp.policies().is_empty());
}

// ==================== test_get_resource_share_invitations ====================

#[tokio::test]
async fn test_get_resource_share_invitations_empty() {
    let client = make_client().await;

    let resp = client
        .get_resource_share_invitations()
        .send()
        .await
        .expect("get_resource_share_invitations should succeed");

    // No invitations set up, should be empty
    assert!(resp.resource_share_invitations().is_empty());
}

// ==================== test_promote_resource_share_created_from_policy ====================

#[tokio::test]
async fn test_promote_resource_share_created_from_policy() {
    let client = make_client().await;

    let create_resp = client
        .create_resource_share()
        .name("promote-share")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    let resp = client
        .promote_resource_share_created_from_policy()
        .resource_share_arn(&arn)
        .send()
        .await
        .expect("promote_resource_share_created_from_policy should succeed");

    assert_eq!(resp.return_value(), Some(true));
}

// ==================== test_replace_permission_associations ====================

#[tokio::test]
async fn test_replace_permission_associations() {
    let client = make_client().await;

    // Create two permissions
    let from_resp = client
        .create_permission()
        .name("FromPerm")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let from_arn = from_resp.permission().unwrap().arn().unwrap().to_string();

    let to_resp = client
        .create_permission()
        .name("ToPerm")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let to_arn = to_resp.permission().unwrap().arn().unwrap().to_string();

    let resp = client
        .replace_permission_associations()
        .from_permission_arn(&from_arn)
        .to_permission_arn(&to_arn)
        .send()
        .await
        .expect("replace_permission_associations should succeed");

    let work = resp
        .replace_permission_associations_work()
        .expect("work item should be present");
    assert!(work.id().is_some());
    assert_eq!(work.from_permission_arn(), Some(from_arn.as_str()));
    assert_eq!(work.to_permission_arn(), Some(to_arn.as_str()));
    assert_eq!(work.status().map(|s| s.as_str()), Some("IN_PROGRESS"));

    // ListReplacePermissionAssociationsWork
    let list_resp = client
        .list_replace_permission_associations_work()
        .send()
        .await
        .expect("list_replace_permission_associations_work should succeed");
    assert_eq!(list_resp.replace_permission_associations_works().len(), 1);
}

// ==================== test_set_default_permission_version ====================

#[tokio::test]
async fn test_set_default_permission_version() {
    let client = make_client().await;

    let create_resp = client
        .create_permission()
        .name("SetDefaultPerm")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let arn = create_resp.permission().unwrap().arn().unwrap().to_string();

    let resp = client
        .set_default_permission_version()
        .permission_arn(&arn)
        .permission_version(1)
        .send()
        .await
        .expect("set_default_permission_version should succeed");

    assert_eq!(resp.return_value(), Some(true));
}

// ==================== test_list_source_associations ====================

#[tokio::test]
async fn test_list_source_associations() {
    let client = make_client().await;

    // Create a share first
    let share_resp = client
        .create_resource_share()
        .name("source-assoc-share")
        .send()
        .await
        .unwrap();
    let share_arn = share_resp
        .resource_share()
        .unwrap()
        .resource_share_arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_source_associations()
        .resource_share_arns(share_arn)
        .send()
        .await
        .expect("list_source_associations should succeed");

    // Expected to be empty (simplified implementation)
    assert!(resp.source_associations().is_empty());
}

// ==================== test_list_pending_invitation_resources ====================

#[tokio::test]
async fn test_list_pending_invitation_resources_not_found() {
    let client = make_client().await;

    let err = client
        .list_pending_invitation_resources()
        .resource_share_invitation_arn(format!(
            "arn:aws:ram:us-east-1:{}:resource-share-invitation/nonexistent",
            ACCOUNT_ID
        ))
        .send()
        .await;

    assert!(err.is_err());
    assert_error_contains(&err.unwrap_err(), "could not be found");
}

// ==================== test_delete_permission_version ====================

#[tokio::test]
async fn test_delete_permission_version() {
    let client = make_client().await;

    let create_resp = client
        .create_permission()
        .name("DelVersionPerm")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let arn = create_resp.permission().unwrap().arn().unwrap().to_string();

    let del_resp = client
        .delete_permission_version()
        .permission_arn(&arn)
        .permission_version(1)
        .send()
        .await
        .expect("delete_permission_version should succeed");

    assert_eq!(del_resp.return_value(), Some(true));
}

// ==================== test_promote_permission_created_from_policy ====================

#[tokio::test]
async fn test_promote_permission_created_from_policy() {
    let client = make_client().await;

    // Create original permission
    let create_resp = client
        .create_permission()
        .name("OriginalPerm")
        .resource_type("ec2:Subnet")
        .policy_template(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();
    let orig_arn = create_resp.permission().unwrap().arn().unwrap().to_string();

    let resp = client
        .promote_permission_created_from_policy()
        .permission_arn(&orig_arn)
        .name("PromotedPerm")
        .send()
        .await
        .expect("promote_permission_created_from_policy should succeed");

    let promoted = resp.permission().expect("permission should be present");
    assert_eq!(promoted.name(), Some("PromotedPerm"));
    assert!(promoted.arn().unwrap().contains("PromotedPerm"));
}
