use aws_sdk_codedeploy::config::BehaviorVersion;
use winterbaume_codedeploy::CodeDeployService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_codedeploy::Client {
    let mock = MockAws::builder()
        .with_service(CodeDeployService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codedeploy::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_codedeploy::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_application() {
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .application_name("my-app")
        .compute_platform(aws_sdk_codedeploy::types::ComputePlatform::Server)
        .send()
        .await
        .expect("create_application should succeed");

    assert!(create_resp.application_id().is_some());

    let get_resp = client
        .get_application()
        .application_name("my-app")
        .send()
        .await
        .expect("get_application should succeed");

    let app = get_resp.application().expect("should have application");
    assert_eq!(app.application_name(), Some("my-app"));
    assert_eq!(
        app.compute_platform(),
        Some(&aws_sdk_codedeploy::types::ComputePlatform::Server)
    );
}

#[tokio::test]
async fn test_list_applications() {
    let client = make_client().await;

    for name in ["app-a", "app-b"] {
        client
            .create_application()
            .application_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    assert_eq!(resp.applications().len(), 2);
}

#[tokio::test]
async fn test_delete_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("delete-me")
        .send()
        .await
        .unwrap();

    client
        .delete_application()
        .application_name("delete-me")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_application()
        .application_name("delete-me")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("temp-app")
        .send()
        .await
        .unwrap();

    let list = client.list_applications().send().await.unwrap();
    assert_eq!(list.applications().len(), 1);

    client
        .delete_application()
        .application_name("temp-app")
        .send()
        .await
        .unwrap();

    let list = client.list_applications().send().await.unwrap();
    assert_eq!(list.applications().len(), 0);
}

#[tokio::test]
async fn test_get_nonexistent_application() {
    let client = make_client().await;

    let result = client
        .get_application()
        .application_name("no-such-app")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_batch_get_applications() {
    let client = make_client().await;

    for name in ["batch-app-1", "batch-app-2", "batch-app-3"] {
        client
            .create_application()
            .application_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .batch_get_applications()
        .application_names("batch-app-1")
        .application_names("batch-app-3")
        .application_names("nonexistent-app")
        .send()
        .await
        .expect("batch_get_applications should succeed");

    // Should return only the 2 that exist
    let infos = resp.applications_info();
    assert_eq!(infos.len(), 2);
}

#[tokio::test]
async fn test_create_and_get_deployment_group() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("dg-app")
        .compute_platform(aws_sdk_codedeploy::types::ComputePlatform::Server)
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_deployment_group()
        .application_name("dg-app")
        .deployment_group_name("my-dg")
        .service_role_arn("arn:aws:iam::123456789012:role/CodeDeployRole")
        .send()
        .await
        .expect("create_deployment_group should succeed");

    assert!(create_resp.deployment_group_id().is_some());

    let get_resp = client
        .get_deployment_group()
        .application_name("dg-app")
        .deployment_group_name("my-dg")
        .send()
        .await
        .expect("get_deployment_group should succeed");

    let info = get_resp
        .deployment_group_info()
        .expect("should have deployment group info");
    assert_eq!(info.deployment_group_name(), Some("my-dg"));
    assert_eq!(info.application_name(), Some("dg-app"));
    assert_eq!(
        info.service_role_arn(),
        Some("arn:aws:iam::123456789012:role/CodeDeployRole")
    );
}

#[tokio::test]
async fn test_list_deployment_groups() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("ldg-app")
        .send()
        .await
        .unwrap();

    for name in ["dg-1", "dg-2"] {
        client
            .create_deployment_group()
            .application_name("ldg-app")
            .deployment_group_name(name)
            .service_role_arn("arn:aws:iam::123456789012:role/Role")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_deployment_groups()
        .application_name("ldg-app")
        .send()
        .await
        .expect("list_deployment_groups should succeed");

    assert_eq!(resp.deployment_groups().len(), 2);
    assert_eq!(resp.application_name(), Some("ldg-app"));
}

#[tokio::test]
async fn test_create_deployment_group_nonexistent_app() {
    let client = make_client().await;

    let result = client
        .create_deployment_group()
        .application_name("no-such-app")
        .deployment_group_name("dg")
        .service_role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_and_get_deployment() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("deploy-app")
        .send()
        .await
        .unwrap();

    client
        .create_deployment_group()
        .application_name("deploy-app")
        .deployment_group_name("deploy-dg")
        .service_role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_deployment()
        .application_name("deploy-app")
        .deployment_group_name("deploy-dg")
        .description("test deployment")
        .revision(
            aws_sdk_codedeploy::types::RevisionLocation::builder()
                .revision_type(aws_sdk_codedeploy::types::RevisionLocationType::S3)
                .s3_location(
                    aws_sdk_codedeploy::types::S3Location::builder()
                        .bucket("my-bucket")
                        .key("my-key.zip")
                        .bundle_type(aws_sdk_codedeploy::types::BundleType::Zip)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("create_deployment should succeed");

    let deployment_id = create_resp
        .deployment_id()
        .expect("should have deployment id");
    assert!(deployment_id.starts_with("d-"));

    let get_resp = client
        .get_deployment()
        .deployment_id(deployment_id)
        .send()
        .await
        .expect("get_deployment should succeed");

    let info = get_resp
        .deployment_info()
        .expect("should have deployment info");
    assert_eq!(info.application_name(), Some("deploy-app"));
    assert_eq!(info.deployment_group_name(), Some("deploy-dg"));
    assert_eq!(info.description(), Some("test deployment"));
    assert_eq!(
        info.status(),
        Some(&aws_sdk_codedeploy::types::DeploymentStatus::Created)
    );
}

#[tokio::test]
async fn test_batch_get_deployments() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("batch-deploy-app")
        .send()
        .await
        .unwrap();

    client
        .create_deployment_group()
        .application_name("batch-deploy-app")
        .deployment_group_name("batch-deploy-dg")
        .service_role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();

    let mut deployment_ids = vec![];
    for _ in 0..3 {
        let resp = client
            .create_deployment()
            .application_name("batch-deploy-app")
            .deployment_group_name("batch-deploy-dg")
            .send()
            .await
            .unwrap();
        deployment_ids.push(resp.deployment_id().unwrap().to_string());
    }

    let resp = client
        .batch_get_deployments()
        .deployment_ids(&deployment_ids[0])
        .deployment_ids(&deployment_ids[2])
        .deployment_ids("d-NONEXISTENT")
        .send()
        .await
        .expect("batch_get_deployments should succeed");

    // Should return only the 2 that exist
    assert_eq!(resp.deployments_info().len(), 2);
}

#[tokio::test]
async fn test_list_deployments() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("list-deploy-app")
        .send()
        .await
        .unwrap();

    client
        .create_deployment_group()
        .application_name("list-deploy-app")
        .deployment_group_name("list-deploy-dg")
        .service_role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();

    for _ in 0..2 {
        client
            .create_deployment()
            .application_name("list-deploy-app")
            .deployment_group_name("list-deploy-dg")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_deployments()
        .application_name("list-deploy-app")
        .deployment_group_name("list-deploy-dg")
        .send()
        .await
        .expect("list_deployments should succeed");

    assert_eq!(resp.deployments().len(), 2);
}

#[tokio::test]
async fn test_tag_resource_and_list_tags() {
    let client = make_client().await;

    let arn = "arn:aws:codedeploy:us-east-1:123456789012:application:my-tagged-app";

    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_codedeploy::types::Tag::builder()
                .key("env")
                .value("prod")
                .build(),
        )
        .tags(
            aws_sdk_codedeploy::types::Tag::builder()
                .key("team")
                .value("platform")
                .build(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);

    let tag_map: std::collections::HashMap<&str, &str> = tags
        .iter()
        .map(|t| (t.key().unwrap_or(""), t.value().unwrap_or("")))
        .collect();
    assert_eq!(tag_map.get("env"), Some(&"prod"));
    assert_eq!(tag_map.get("team"), Some(&"platform"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let arn = "arn:aws:codedeploy:us-east-1:123456789012:application:untag-app";

    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_codedeploy::types::Tag::builder()
                .key("env")
                .value("prod")
                .build(),
        )
        .tags(
            aws_sdk_codedeploy::types::Tag::builder()
                .key("team")
                .value("platform")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("team"));
    assert_eq!(tags[0].value(), Some("platform"));
}

#[tokio::test]
async fn test_get_nonexistent_deployment() {
    let client = make_client().await;

    let result = client
        .get_deployment()
        .deployment_id("d-NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_deployment_group() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("exists-app")
        .send()
        .await
        .unwrap();

    let result = client
        .get_deployment_group()
        .application_name("exists-app")
        .deployment_group_name("no-such-dg")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS CodeDeploy
// ============================================================================

#[tokio::test]
async fn test_create_duplicate_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("dup-app")
        .send()
        .await
        .unwrap();

    let result = client
        .create_application()
        .application_name("dup-app")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_application() {
    let client = make_client().await;

    let result = client
        .delete_application()
        .application_name("no-such-app")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_applications_empty() {
    let client = make_client().await;

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    assert_eq!(resp.applications().len(), 0);
}

#[tokio::test]
async fn test_create_duplicate_deployment_group() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("dup-dg-app")
        .send()
        .await
        .unwrap();

    client
        .create_deployment_group()
        .application_name("dup-dg-app")
        .deployment_group_name("dup-dg")
        .service_role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();

    let result = client
        .create_deployment_group()
        .application_name("dup-dg-app")
        .deployment_group_name("dup-dg")
        .service_role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_deployment_groups_nonexistent_app() {
    let client = make_client().await;

    let result = client
        .list_deployment_groups()
        .application_name("no-such-app")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_deployment_groups_empty() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("empty-dg-app")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_deployment_groups()
        .application_name("empty-dg-app")
        .send()
        .await
        .expect("list_deployment_groups should succeed");

    assert_eq!(resp.deployment_groups().len(), 0);
}

#[tokio::test]
async fn test_create_deployment_nonexistent_deployment_group() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("no-dg-app")
        .send()
        .await
        .unwrap();

    let result = client
        .create_deployment()
        .application_name("no-dg-app")
        .deployment_group_name("no-such-dg")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_deployments_empty() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("empty-deploy-app")
        .send()
        .await
        .unwrap();

    client
        .create_deployment_group()
        .application_name("empty-deploy-app")
        .deployment_group_name("empty-deploy-dg")
        .service_role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_deployments()
        .application_name("empty-deploy-app")
        .deployment_group_name("empty-deploy-dg")
        .send()
        .await
        .expect("list_deployments should succeed");

    assert_eq!(resp.deployments().len(), 0);
}

#[tokio::test]
async fn test_batch_get_deployments_all_nonexistent() {
    let client = make_client().await;

    let resp = client
        .batch_get_deployments()
        .deployment_ids("d-NONEXISTENT1")
        .deployment_ids("d-NONEXISTENT2")
        .send()
        .await
        .expect("batch_get_deployments should succeed even if all are nonexistent");

    assert_eq!(resp.deployments_info().len(), 0);
}

#[tokio::test]
async fn test_untag_resource_all_tags() {
    let client = make_client().await;

    let arn = "arn:aws:codedeploy:us-east-1:123456789012:application:untag-all-app";

    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_codedeploy::types::Tag::builder()
                .key("key1")
                .value("value1")
                .build(),
        )
        .tags(
            aws_sdk_codedeploy::types::Tag::builder()
                .key("key2")
                .value("value2")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("key1")
        .tag_keys("key2")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 0);
}
