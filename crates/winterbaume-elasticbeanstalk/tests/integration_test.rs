use aws_sdk_elasticbeanstalk::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_elasticbeanstalk::ElasticBeanstalkService;

async fn make_client() -> aws_sdk_elasticbeanstalk::Client {
    let mock = MockAws::builder()
        .with_service(ElasticBeanstalkService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticbeanstalk::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_elasticbeanstalk::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_delete_application() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .application_name("my-app")
        .description("A test application")
        .send()
        .await
        .expect("create_application should succeed");

    let app = resp.application().expect("should have application");
    assert_eq!(app.application_name(), Some("my-app"));
    assert_eq!(app.description(), Some("A test application"));

    client
        .delete_application()
        .application_name("my-app")
        .send()
        .await
        .expect("delete_application should succeed");
}

#[tokio::test]
async fn test_create_duplicate_application_fails() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("duplicate-app")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_application()
        .application_name("duplicate-app")
        .send()
        .await;

    assert!(result.is_err(), "duplicate application should fail");
}

#[tokio::test]
async fn test_create_environment() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("env-app")
        .send()
        .await
        .expect("create_application should succeed");

    let resp = client
        .create_environment()
        .application_name("env-app")
        .environment_name("my-env")
        .solution_stack_name("64bit Amazon Linux 2023 v4.3.0 running Python 3.11")
        .send()
        .await
        .expect("create_environment should succeed");

    assert_eq!(resp.environment_name(), Some("my-env"));
    assert_eq!(resp.application_name(), Some("env-app"));
    assert_eq!(resp.status().map(|s| s.as_str()), Some("Launching"));
}

#[tokio::test]
async fn test_describe_environments() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("desc-app")
        .send()
        .await
        .unwrap();

    client
        .create_environment()
        .application_name("desc-app")
        .environment_name("env-a")
        .send()
        .await
        .unwrap();

    client
        .create_environment()
        .application_name("desc-app")
        .environment_name("env-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_environments()
        .application_name("desc-app")
        .send()
        .await
        .expect("describe_environments should succeed");

    assert_eq!(resp.environments().len(), 2);
}

#[tokio::test]
async fn test_describe_environments_all() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("app1")
        .send()
        .await
        .unwrap();
    client
        .create_application()
        .application_name("app2")
        .send()
        .await
        .unwrap();

    client
        .create_environment()
        .application_name("app1")
        .environment_name("env1")
        .send()
        .await
        .unwrap();

    client
        .create_environment()
        .application_name("app2")
        .environment_name("env2")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_environments()
        .send()
        .await
        .expect("describe_environments (all) should succeed");

    assert_eq!(resp.environments().len(), 2);
}

#[tokio::test]
async fn test_list_available_solution_stacks() {
    let client = make_client().await;

    let resp = client
        .list_available_solution_stacks()
        .send()
        .await
        .expect("list_available_solution_stacks should succeed");

    let stacks = resp.solution_stacks();
    assert!(!stacks.is_empty(), "should have solution stacks");
    // Verify some common ones are present
    let has_python = stacks.iter().any(|s| s.contains("Python"));
    assert!(has_python, "should have Python solution stacks");
}

#[tokio::test]
async fn test_list_and_update_tags_for_resource() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("tagged-app")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_environment()
        .application_name("tagged-app")
        .environment_name("tagged-env")
        .tags(
            aws_sdk_elasticbeanstalk::types::Tag::builder()
                .key("Env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .expect("create_environment with tags should succeed");

    let env_arn = create_resp
        .environment_arn()
        .expect("should have environment ARN");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(env_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.resource_tags();
    assert!(!tags.is_empty(), "should have tags");
    let env_tag = tags.iter().find(|t| t.key() == Some("Env"));
    assert!(env_tag.is_some(), "should have Env tag");
    assert_eq!(env_tag.unwrap().value(), Some("test"));

    // Update tags
    client
        .update_tags_for_resource()
        .resource_arn(env_arn)
        .tags_to_add(
            aws_sdk_elasticbeanstalk::types::Tag::builder()
                .key("NewKey")
                .value("NewValue")
                .build(),
        )
        .tags_to_remove("Env")
        .send()
        .await
        .expect("update_tags_for_resource should succeed");

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(env_arn)
        .send()
        .await
        .expect("list_tags_for_resource after update should succeed");

    let tags2 = tags_resp2.resource_tags();
    let new_key = tags2.iter().find(|t| t.key() == Some("NewKey"));
    assert!(new_key.is_some(), "NewKey tag should be present");
    let old_key = tags2.iter().find(|t| t.key() == Some("Env"));
    assert!(old_key.is_none(), "Env tag should be removed");
}

#[tokio::test]
async fn test_create_environment_for_nonexistent_app_fails() {
    let client = make_client().await;

    let result = client
        .create_environment()
        .application_name("nonexistent-app")
        .environment_name("my-env")
        .send()
        .await;

    assert!(
        result.is_err(),
        "creating env for nonexistent app should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_application() {
    let client = make_client().await;

    let result = client
        .delete_application()
        .application_name("nonexistent-app")
        .send()
        .await;

    assert!(result.is_err(), "deleting nonexistent app should fail");
}
