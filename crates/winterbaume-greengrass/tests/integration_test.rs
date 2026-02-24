use aws_sdk_greengrass::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_greengrass::GreengrassService;

async fn make_client() -> aws_sdk_greengrass::Client {
    let mock = MockAws::builder()
        .with_service(GreengrassService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_greengrass::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_greengrass::Client::new(&config)
}

// ---- Group tests ----

#[tokio::test]
async fn test_create_and_get_group() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("my-test-group")
        .send()
        .await
        .expect("create_group should succeed");

    let group_id = resp.id().expect("should have group id");
    assert!(!group_id.is_empty());
    assert_eq!(resp.name().unwrap(), "my-test-group");
    assert!(resp.arn().is_some());
    assert!(resp.creation_timestamp().is_some());

    let get_resp = client
        .get_group()
        .group_id(group_id)
        .send()
        .await
        .expect("get_group should succeed");

    assert_eq!(get_resp.id().unwrap(), group_id);
    assert_eq!(get_resp.name().unwrap(), "my-test-group");
    assert!(get_resp.arn().is_some());
}

#[tokio::test]
async fn test_list_groups_empty() {
    let client = make_client().await;

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");

    assert!(resp.groups().is_empty());
}

#[tokio::test]
async fn test_list_groups_with_items() {
    let client = make_client().await;

    client.create_group().name("group-a").send().await.unwrap();
    client.create_group().name("group-b").send().await.unwrap();

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");

    assert_eq!(resp.groups().len(), 2);
}

#[tokio::test]
async fn test_delete_group() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("to-delete")
        .send()
        .await
        .unwrap();
    let group_id = resp.id().unwrap().to_string();

    client
        .delete_group()
        .group_id(&group_id)
        .send()
        .await
        .expect("delete_group should succeed");

    // Verify group is gone
    let result = client.get_group().group_id(&group_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("temp-group")
        .send()
        .await
        .unwrap();
    let group_id = resp.id().unwrap().to_string();

    let list = client.list_groups().send().await.unwrap();
    assert_eq!(list.groups().len(), 1);

    client
        .delete_group()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();

    let list = client.list_groups().send().await.unwrap();
    assert_eq!(list.groups().len(), 0);
}

#[tokio::test]
async fn test_get_nonexistent_group() {
    let client = make_client().await;

    let result = client
        .get_group()
        .group_id("nonexistent-group-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_group() {
    let client = make_client().await;

    let result = client
        .delete_group()
        .group_id("nonexistent-group-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- UpdateGroup test ----

#[tokio::test]
async fn test_update_group() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("original-name")
        .send()
        .await
        .unwrap();
    let group_id = resp.id().unwrap().to_string();

    client
        .update_group()
        .group_id(&group_id)
        .name("updated-name")
        .send()
        .await
        .expect("update_group should succeed");

    let get_resp = client.get_group().group_id(&group_id).send().await.unwrap();
    assert_eq!(get_resp.name().unwrap(), "updated-name");
}

// ---- GroupVersion tests ----

#[tokio::test]
async fn test_create_and_get_group_version() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("version-test-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    let gv = client
        .create_group_version()
        .group_id(&group_id)
        .send()
        .await
        .expect("create_group_version should succeed");

    assert!(gv.arn().is_some());
    assert!(gv.version().is_some());
    let version_id = gv.version().unwrap().to_string();

    let get_gv = client
        .get_group_version()
        .group_id(&group_id)
        .group_version_id(&version_id)
        .send()
        .await
        .expect("get_group_version should succeed");

    assert_eq!(get_gv.version().unwrap(), &version_id);
    assert!(get_gv.arn().is_some());
}

#[tokio::test]
async fn test_list_group_versions() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("list-versions-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    client
        .create_group_version()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();
    client
        .create_group_version()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_group_versions()
        .group_id(&group_id)
        .send()
        .await
        .expect("list_group_versions should succeed");

    // create_group creates an initial version, plus 2 explicit creates = 3
    assert_eq!(resp.versions().len(), 3);
}

// ---- Role Association tests ----

#[tokio::test]
async fn test_associate_and_get_role() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("role-test-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    let assoc = client
        .associate_role_to_group()
        .group_id(&group_id)
        .role_arn("arn:aws:iam::123456789012:role/MyRole")
        .send()
        .await
        .expect("associate_role_to_group should succeed");

    assert!(assoc.associated_at().is_some());

    let get_role = client
        .get_associated_role()
        .group_id(&group_id)
        .send()
        .await
        .expect("get_associated_role should succeed");

    assert_eq!(
        get_role.role_arn().unwrap(),
        "arn:aws:iam::123456789012:role/MyRole"
    );
}

#[tokio::test]
async fn test_disassociate_role_from_group() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("disassoc-role-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    client
        .associate_role_to_group()
        .group_id(&group_id)
        .role_arn("arn:aws:iam::123456789012:role/MyRole")
        .send()
        .await
        .unwrap();

    let resp = client
        .disassociate_role_from_group()
        .group_id(&group_id)
        .send()
        .await
        .expect("disassociate_role_from_group should succeed");

    assert!(resp.disassociated_at().is_some());

    // Verify role is gone
    let result = client
        .get_associated_role()
        .group_id(&group_id)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Deployment tests ----

#[tokio::test]
async fn test_create_and_get_deployment() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("deploy-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    let deploy = client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .expect("create_deployment should succeed");

    assert!(deploy.deployment_id().is_some());
    assert!(deploy.deployment_arn().is_some());

    let deployment_id = deploy.deployment_id().unwrap().to_string();

    let status = client
        .get_deployment_status()
        .group_id(&group_id)
        .deployment_id(&deployment_id)
        .send()
        .await
        .expect("get_deployment_status should succeed");

    assert!(status.deployment_status().is_some());
}

#[tokio::test]
async fn test_list_deployments() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("list-deploy-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_deployments()
        .group_id(&group_id)
        .send()
        .await
        .expect("list_deployments should succeed");

    assert_eq!(resp.deployments().len(), 1);
}

#[tokio::test]
async fn test_reset_deployments() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("reset-deploy-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .unwrap();

    let resp = client
        .reset_deployments()
        .group_id(&group_id)
        .send()
        .await
        .expect("reset_deployments should succeed");

    assert!(resp.deployment_arn().is_some());
    assert!(resp.deployment_id().is_some());
}

// ---- Core Definition tests ----

#[tokio::test]
async fn test_core_definition_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_core_definition()
        .name("my-core-def")
        .send()
        .await
        .expect("create_core_definition should succeed");

    let def_id = resp.id().unwrap().to_string();
    assert_eq!(resp.name().unwrap(), "my-core-def");
    assert!(resp.arn().is_some());
    assert!(resp.latest_version().is_some());

    // Get
    let get_resp = client
        .get_core_definition()
        .core_definition_id(&def_id)
        .send()
        .await
        .expect("get_core_definition should succeed");

    assert_eq!(get_resp.id().unwrap(), &def_id);
    assert_eq!(get_resp.name().unwrap(), "my-core-def");

    // Update
    client
        .update_core_definition()
        .core_definition_id(&def_id)
        .name("updated-core-def")
        .send()
        .await
        .expect("update_core_definition should succeed");

    let get_resp = client
        .get_core_definition()
        .core_definition_id(&def_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.name().unwrap(), "updated-core-def");

    // List
    let list_resp = client
        .list_core_definitions()
        .send()
        .await
        .expect("list_core_definitions should succeed");

    assert_eq!(list_resp.definitions().len(), 1);

    // Delete
    client
        .delete_core_definition()
        .core_definition_id(&def_id)
        .send()
        .await
        .expect("delete_core_definition should succeed");

    let result = client
        .get_core_definition()
        .core_definition_id(&def_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_core_definition_version_lifecycle() {
    let client = make_client().await;

    let def = client
        .create_core_definition()
        .name("core-ver-test")
        .send()
        .await
        .unwrap();
    let def_id = def.id().unwrap().to_string();
    let initial_version = def.latest_version().unwrap().to_string();

    // Create version
    let ver = client
        .create_core_definition_version()
        .core_definition_id(&def_id)
        .send()
        .await
        .expect("create_core_definition_version should succeed");

    assert!(ver.arn().is_some());
    let version_id = ver.version().unwrap().to_string();

    // Get version
    let get_ver = client
        .get_core_definition_version()
        .core_definition_id(&def_id)
        .core_definition_version_id(&version_id)
        .send()
        .await
        .expect("get_core_definition_version should succeed");

    assert_eq!(get_ver.version().unwrap(), &version_id);

    // List versions (should have initial + new = 2)
    let list_ver = client
        .list_core_definition_versions()
        .core_definition_id(&def_id)
        .send()
        .await
        .expect("list_core_definition_versions should succeed");

    assert_eq!(list_ver.versions().len(), 2);
}

// ---- Device Definition tests ----

#[tokio::test]
async fn test_device_definition_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_device_definition()
        .name("my-device-def")
        .send()
        .await
        .expect("create_device_definition should succeed");

    let def_id = resp.id().unwrap().to_string();
    assert_eq!(resp.name().unwrap(), "my-device-def");

    let get_resp = client
        .get_device_definition()
        .device_definition_id(&def_id)
        .send()
        .await
        .expect("get_device_definition should succeed");

    assert_eq!(get_resp.id().unwrap(), &def_id);

    client
        .update_device_definition()
        .device_definition_id(&def_id)
        .name("updated-device-def")
        .send()
        .await
        .expect("update_device_definition should succeed");

    let list = client
        .list_device_definitions()
        .send()
        .await
        .expect("list_device_definitions should succeed");

    assert_eq!(list.definitions().len(), 1);

    client
        .delete_device_definition()
        .device_definition_id(&def_id)
        .send()
        .await
        .expect("delete_device_definition should succeed");

    let result = client
        .get_device_definition()
        .device_definition_id(&def_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_device_definition_version() {
    let client = make_client().await;

    let def = client
        .create_device_definition()
        .name("dev-ver-test")
        .send()
        .await
        .unwrap();
    let def_id = def.id().unwrap().to_string();

    let ver = client
        .create_device_definition_version()
        .device_definition_id(&def_id)
        .send()
        .await
        .expect("create_device_definition_version should succeed");

    let version_id = ver.version().unwrap().to_string();

    let get_ver = client
        .get_device_definition_version()
        .device_definition_id(&def_id)
        .device_definition_version_id(&version_id)
        .send()
        .await
        .expect("get_device_definition_version should succeed");

    assert_eq!(get_ver.version().unwrap(), &version_id);

    let list_ver = client
        .list_device_definition_versions()
        .device_definition_id(&def_id)
        .send()
        .await
        .expect("list_device_definition_versions should succeed");

    assert_eq!(list_ver.versions().len(), 2);
}

// ---- Function Definition tests ----

#[tokio::test]
async fn test_function_definition_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_function_definition()
        .name("my-func-def")
        .send()
        .await
        .expect("create_function_definition should succeed");

    let def_id = resp.id().unwrap().to_string();
    assert_eq!(resp.name().unwrap(), "my-func-def");

    let get_resp = client
        .get_function_definition()
        .function_definition_id(&def_id)
        .send()
        .await
        .expect("get_function_definition should succeed");

    assert_eq!(get_resp.id().unwrap(), &def_id);

    client
        .update_function_definition()
        .function_definition_id(&def_id)
        .name("updated-func-def")
        .send()
        .await
        .expect("update_function_definition should succeed");

    let list = client
        .list_function_definitions()
        .send()
        .await
        .expect("list_function_definitions should succeed");

    assert_eq!(list.definitions().len(), 1);

    client
        .delete_function_definition()
        .function_definition_id(&def_id)
        .send()
        .await
        .expect("delete_function_definition should succeed");

    let result = client
        .get_function_definition()
        .function_definition_id(&def_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_function_definition_version() {
    let client = make_client().await;

    let def = client
        .create_function_definition()
        .name("func-ver-test")
        .send()
        .await
        .unwrap();
    let def_id = def.id().unwrap().to_string();

    let ver = client
        .create_function_definition_version()
        .function_definition_id(&def_id)
        .send()
        .await
        .expect("create_function_definition_version should succeed");

    let version_id = ver.version().unwrap().to_string();

    let get_ver = client
        .get_function_definition_version()
        .function_definition_id(&def_id)
        .function_definition_version_id(&version_id)
        .send()
        .await
        .expect("get_function_definition_version should succeed");

    assert_eq!(get_ver.version().unwrap(), &version_id);

    let list_ver = client
        .list_function_definition_versions()
        .function_definition_id(&def_id)
        .send()
        .await
        .expect("list_function_definition_versions should succeed");

    assert_eq!(list_ver.versions().len(), 2);
}

// ---- Resource Definition tests ----

#[tokio::test]
async fn test_resource_definition_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_resource_definition()
        .name("my-resource-def")
        .send()
        .await
        .expect("create_resource_definition should succeed");

    let def_id = resp.id().unwrap().to_string();
    assert_eq!(resp.name().unwrap(), "my-resource-def");

    let get_resp = client
        .get_resource_definition()
        .resource_definition_id(&def_id)
        .send()
        .await
        .expect("get_resource_definition should succeed");

    assert_eq!(get_resp.id().unwrap(), &def_id);

    client
        .update_resource_definition()
        .resource_definition_id(&def_id)
        .name("updated-resource-def")
        .send()
        .await
        .expect("update_resource_definition should succeed");

    let list = client
        .list_resource_definitions()
        .send()
        .await
        .expect("list_resource_definitions should succeed");

    assert_eq!(list.definitions().len(), 1);

    client
        .delete_resource_definition()
        .resource_definition_id(&def_id)
        .send()
        .await
        .expect("delete_resource_definition should succeed");

    let result = client
        .get_resource_definition()
        .resource_definition_id(&def_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resource_definition_version() {
    let client = make_client().await;

    let def = client
        .create_resource_definition()
        .name("res-ver-test")
        .send()
        .await
        .unwrap();
    let def_id = def.id().unwrap().to_string();

    let ver = client
        .create_resource_definition_version()
        .resource_definition_id(&def_id)
        .send()
        .await
        .expect("create_resource_definition_version should succeed");

    let version_id = ver.version().unwrap().to_string();

    let get_ver = client
        .get_resource_definition_version()
        .resource_definition_id(&def_id)
        .resource_definition_version_id(&version_id)
        .send()
        .await
        .expect("get_resource_definition_version should succeed");

    assert_eq!(get_ver.version().unwrap(), &version_id);

    let list_ver = client
        .list_resource_definition_versions()
        .resource_definition_id(&def_id)
        .send()
        .await
        .expect("list_resource_definition_versions should succeed");

    assert_eq!(list_ver.versions().len(), 2);
}

// ---- Subscription Definition tests ----

#[tokio::test]
async fn test_subscription_definition_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_subscription_definition()
        .name("my-sub-def")
        .send()
        .await
        .expect("create_subscription_definition should succeed");

    let def_id = resp.id().unwrap().to_string();
    assert_eq!(resp.name().unwrap(), "my-sub-def");

    let get_resp = client
        .get_subscription_definition()
        .subscription_definition_id(&def_id)
        .send()
        .await
        .expect("get_subscription_definition should succeed");

    assert_eq!(get_resp.id().unwrap(), &def_id);

    client
        .update_subscription_definition()
        .subscription_definition_id(&def_id)
        .name("updated-sub-def")
        .send()
        .await
        .expect("update_subscription_definition should succeed");

    let list = client
        .list_subscription_definitions()
        .send()
        .await
        .expect("list_subscription_definitions should succeed");

    assert_eq!(list.definitions().len(), 1);

    client
        .delete_subscription_definition()
        .subscription_definition_id(&def_id)
        .send()
        .await
        .expect("delete_subscription_definition should succeed");

    let result = client
        .get_subscription_definition()
        .subscription_definition_id(&def_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_subscription_definition_version() {
    let client = make_client().await;

    let def = client
        .create_subscription_definition()
        .name("sub-ver-test")
        .send()
        .await
        .unwrap();
    let def_id = def.id().unwrap().to_string();

    let ver = client
        .create_subscription_definition_version()
        .subscription_definition_id(&def_id)
        .send()
        .await
        .expect("create_subscription_definition_version should succeed");

    let version_id = ver.version().unwrap().to_string();

    let get_ver = client
        .get_subscription_definition_version()
        .subscription_definition_id(&def_id)
        .subscription_definition_version_id(&version_id)
        .send()
        .await
        .expect("get_subscription_definition_version should succeed");

    assert_eq!(get_ver.version().unwrap(), &version_id);

    let list_ver = client
        .list_subscription_definition_versions()
        .subscription_definition_id(&def_id)
        .send()
        .await
        .expect("list_subscription_definition_versions should succeed");

    assert_eq!(list_ver.versions().len(), 2);
}

// ---- Error path tests ----

#[tokio::test]
async fn test_delete_nonexistent_core_definition() {
    let client = make_client().await;

    let result = client
        .delete_core_definition()
        .core_definition_id("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_deployment_status() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("deploy-err-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    let result = client
        .get_deployment_status()
        .group_id(&group_id)
        .deployment_id("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_nonexistent_group() {
    let client = make_client().await;

    let result = client
        .update_group()
        .group_id("nonexistent")
        .name("test")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_greengrass_groups.py
// ============================================================================

// Ported from moto: test_greengrass_groups.py::test_get_group_with_invalid_id
#[tokio::test]
async fn test_moto_get_group_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_group()
        .group_id("b552443b-1888-469b-81f8-0ebc5ca92949")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException")
            || err_str.contains("That Group Definition does not exist."),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_delete_group_with_invalid_id
#[tokio::test]
async fn test_moto_delete_group_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .delete_group()
        .group_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_update_group_with_empty_name
#[tokio::test]
async fn test_moto_update_group_with_empty_name() {
    let client = make_client().await;
    let create_res = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = create_res.id().unwrap();

    let err = client
        .update_group()
        .group_id(group_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException")
            || err_str.contains("Input does not contain any attributes to be updated"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_update_group_with_invalid_id
#[tokio::test]
async fn test_moto_update_group_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .update_group()
        .group_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("123")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_create_group_version
#[tokio::test]
async fn test_moto_create_group_version() {
    let client = make_client().await;
    let create_res = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = create_res.id().unwrap().to_string();

    let gv_res = client
        .create_group_version()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();

    assert!(gv_res.arn().is_some());
    assert!(gv_res.creation_timestamp().is_some());
    assert_eq!(gv_res.id().unwrap(), &group_id);
    assert!(gv_res.version().is_some());
}

// Ported from moto: test_greengrass_groups.py::test_create_group_version_with_invalid_id
#[tokio::test]
async fn test_moto_create_group_version_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .create_group_version()
        .group_id("cd2ea6dc-6634-4e89-8441-8003500435f9")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_list_group_versions (initial version from create_group)
#[tokio::test]
async fn test_moto_list_group_versions_initial() {
    let client = make_client().await;
    let create_res = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = create_res.id().unwrap().to_string();

    let list_res = client
        .list_group_versions()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();

    // create_group creates an initial group version
    assert_eq!(list_res.versions().len(), 1);
    let gv = &list_res.versions()[0];
    assert!(gv.arn().is_some());
    assert!(gv.creation_timestamp().is_some());
    assert_eq!(gv.id().unwrap(), &group_id);
    assert!(gv.version().is_some());
}

// Ported from moto: test_greengrass_groups.py::test_list_group_versions_with_invalid_id
#[tokio::test]
async fn test_moto_list_group_versions_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .list_group_versions()
        .group_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_get_group_version_with_invalid_id
#[tokio::test]
async fn test_moto_get_group_version_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_group_version()
        .group_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .group_version_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_get_group_version_with_invalid_version_id
#[tokio::test]
async fn test_moto_get_group_version_with_invalid_version_id() {
    let client = make_client().await;
    let create_res = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = create_res.id().unwrap().to_string();
    let invalid_ver_id = "7b0bdeae-54c7-47cf-9f93-561e672efd9c";

    let err = client
        .get_group_version()
        .group_id(&group_id)
        .group_version_id(invalid_ver_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionNotFoundException"),
        "Expected VersionNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_associate_role_to_group
#[tokio::test]
async fn test_moto_associate_role_to_group() {
    let client = make_client().await;
    // Moto does not require the group to exist
    let res = client
        .associate_role_to_group()
        .group_id("abc002c8-1093-485e-9324-3baadf38e582")
        .role_arn("arn:aws:iam::123456789012:role/greengrass-role")
        .send()
        .await
        .unwrap();
    assert!(res.associated_at().is_some());
}

// Ported from moto: test_greengrass_groups.py::test_get_associated_role
#[tokio::test]
async fn test_moto_get_associated_role() {
    let client = make_client().await;
    let group_id = "abc002c8-1093-485e-9324-3baadf38e582";
    let role_arn = "arn:aws:iam::123456789012:role/greengrass-role";
    client
        .associate_role_to_group()
        .group_id(group_id)
        .role_arn(role_arn)
        .send()
        .await
        .unwrap();

    let res = client
        .get_associated_role()
        .group_id(group_id)
        .send()
        .await
        .unwrap();
    assert!(res.associated_at().is_some());
    assert_eq!(res.role_arn().unwrap(), role_arn);
}

// Ported from moto: test_greengrass_groups.py::test_get_associated_role_with_invalid_id
#[tokio::test]
async fn test_moto_get_associated_role_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_associated_role()
        .group_id("abc002c8-1093-485e-9324-3baadf38e582")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("You need to attach an IAM role to this deployment group"),
        "Expected role not found error, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_disassociate_role_from_group
#[tokio::test]
async fn test_moto_disassociate_role_from_group() {
    let client = make_client().await;
    let group_id = "abc002c8-1093-485e-9324-3baadf38e582";
    let role_arn = "arn:aws:iam::123456789012:role/greengrass-role";
    client
        .associate_role_to_group()
        .group_id(group_id)
        .role_arn(role_arn)
        .send()
        .await
        .unwrap();
    client
        .get_associated_role()
        .group_id(group_id)
        .send()
        .await
        .unwrap();

    let res = client
        .disassociate_role_from_group()
        .group_id(group_id)
        .send()
        .await
        .unwrap();
    assert!(res.disassociated_at().is_some());

    // After disassociation, get_associated_role should fail
    let err = client
        .get_associated_role()
        .group_id(group_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("You need to attach an IAM role to this deployment group"),
        "Expected role not found error, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_groups.py::test_disassociate_role_from_group_with_none_exists_group_id
#[tokio::test]
async fn test_moto_disassociate_role_from_nonexistent_group() {
    let client = make_client().await;
    // Moto returns success even for non-existent group
    let res = client
        .disassociate_role_from_group()
        .group_id("abc002c8-1093-485e-9324-3baadf38e582")
        .send()
        .await
        .unwrap();
    assert!(res.disassociated_at().is_some());
}

// ============================================================================
// Ported from moto: test_greengrass_core.py
// ============================================================================

// Ported from moto: test_greengrass_core.py::test_delete_core_definition (double delete)
#[tokio::test]
async fn test_moto_delete_core_definition_double_delete() {
    let client = make_client().await;
    let create_res = client
        .create_core_definition()
        .name("TestCore")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    client
        .delete_core_definition()
        .core_definition_id(&def_id)
        .send()
        .await
        .unwrap();

    let err = client
        .delete_core_definition()
        .core_definition_id(&def_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_core.py::test_update_core_definition_with_empty_name
#[tokio::test]
async fn test_moto_update_core_definition_with_empty_name() {
    let client = make_client().await;
    let create_res = client
        .create_core_definition()
        .name("TestCore")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    let err = client
        .update_core_definition()
        .core_definition_id(&def_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException")
            || err_str.contains("Input does not contain any attributes to be updated"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_core.py::test_update_core_definition_with_invalid_id
#[tokio::test]
async fn test_moto_update_core_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .update_core_definition()
        .core_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("abc")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_core.py::test_get_core_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_get_core_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_core_definition_version()
        .core_definition_id("fe2392e9-e67f-4308-af1b-ff94a128b231")
        .core_definition_version_id("cd2ea6dc-6634-4e89-8441-8003500435f9")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_core.py::test_get_core_definition_version_with_invalid_version_id
#[tokio::test]
async fn test_moto_get_core_def_ver_with_invalid_version_id() {
    let client = make_client().await;
    let create_res = client
        .create_core_definition()
        .name("TestCore")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();
    let invalid_version_id = "cd2ea6dc-6634-4e89-8441-8003500435f9";

    let err = client
        .get_core_definition_version()
        .core_definition_id(&def_id)
        .core_definition_version_id(invalid_version_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionNotFoundException"),
        "Expected VersionNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_core.py::test_list_core_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_list_core_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .list_core_definition_versions()
        .core_definition_id("cd2ea6dc-6634-4e89-8441-8003500435f9")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_greengrass_device.py
// ============================================================================

// Ported from moto: test_greengrass_device.py::test_get_device_definition_with_invalid_id
#[tokio::test]
async fn test_moto_get_device_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_device_definition()
        .device_definition_id("b552443b-1888-469b-81f8-0ebc5ca92949")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_device.py::test_delete_device_definition_with_invalid_id
#[tokio::test]
async fn test_moto_delete_device_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .delete_device_definition()
        .device_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_device.py::test_update_device_definition_with_empty_name
#[tokio::test]
async fn test_moto_update_device_definition_with_empty_name() {
    let client = make_client().await;
    let create_res = client
        .create_device_definition()
        .name("TestDevice")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    let err = client
        .update_device_definition()
        .device_definition_id(&def_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_device.py::test_update_device_definition_with_invalid_id
#[tokio::test]
async fn test_moto_update_device_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .update_device_definition()
        .device_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("123")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_device.py::test_create_device_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_create_device_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .create_device_definition_version()
        .device_definition_id("123")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_device.py::test_get_device_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_get_device_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_device_definition_version()
        .device_definition_id("fe2392e9-e67f-4308-af1b-ff94a128b231")
        .device_definition_version_id("cd2ea6dc-6634-4e89-8441-8003500435f9")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_device.py::test_get_device_definition_version_with_invalid_version_id
#[tokio::test]
async fn test_moto_get_device_def_ver_with_invalid_version_id() {
    let client = make_client().await;
    let create_res = client
        .create_device_definition()
        .name("TestDevice")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();
    let invalid_ver_id = "6fbffc21-989e-4d29-a793-a42f450a78c6";

    let err = client
        .get_device_definition_version()
        .device_definition_id(&def_id)
        .device_definition_version_id(invalid_ver_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionNotFoundException"),
        "Expected VersionNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_greengrass_functions.py
// ============================================================================

// Ported from moto: test_greengrass_functions.py::test_get_function_definition_with_invalid_id
#[tokio::test]
async fn test_moto_get_function_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_function_definition()
        .function_definition_id("b552443b-1888-469b-81f8-0ebc5ca92949")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_functions.py::test_delete_function_definition_with_invalid_id
#[tokio::test]
async fn test_moto_delete_function_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .delete_function_definition()
        .function_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_functions.py::test_update_function_definition_with_empty_name
#[tokio::test]
async fn test_moto_update_function_definition_with_empty_name() {
    let client = make_client().await;
    let create_res = client
        .create_function_definition()
        .name("TestFunc")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    let err = client
        .update_function_definition()
        .function_definition_id(&def_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_functions.py::test_update_function_definition_with_invalid_id
#[tokio::test]
async fn test_moto_update_function_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .update_function_definition()
        .function_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("123")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_functions.py::test_create_function_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_create_func_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .create_function_definition_version()
        .function_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_functions.py::test_list_function_definition_versions_with_invalid_id
#[tokio::test]
async fn test_moto_list_func_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .list_function_definition_versions()
        .function_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_functions.py::test_get_function_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_get_func_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_function_definition_version()
        .function_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .function_definition_version_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_functions.py::test_get_function_definition_version_with_invalid_version_id
#[tokio::test]
async fn test_moto_get_func_def_ver_with_invalid_version_id() {
    let client = make_client().await;
    let create_res = client
        .create_function_definition()
        .name("TestFunction")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();
    let invalid_ver_id = "7b0bdeae-54c7-47cf-9f93-561e672efd9c";

    let err = client
        .get_function_definition_version()
        .function_definition_id(&def_id)
        .function_definition_version_id(invalid_ver_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    // Moto uses "IdNotFoundException" for this case, but some use VersionNotFoundException
    assert!(
        err_str.contains("VersionNotFoundException") || err_str.contains("IdNotFoundException"),
        "Expected version not found error, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_greengrass_resource.py
// ============================================================================

// Ported from moto: test_greengrass_resource.py::test_get_resource_definition_with_invalid_id
#[tokio::test]
async fn test_moto_get_resource_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_resource_definition()
        .resource_definition_id("76f22a66-176a-4474-b450-04099dc4b069")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_resource.py::test_delete_resource_definition_with_invalid_id
#[tokio::test]
async fn test_moto_delete_resource_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .delete_resource_definition()
        .resource_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_resource.py::test_update_resource_definition_with_invalid_id
#[tokio::test]
async fn test_moto_update_resource_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .update_resource_definition()
        .resource_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("123")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_resource.py::test_update_device_definition_with_empty_name (resource)
#[tokio::test]
async fn test_moto_update_resource_definition_with_empty_name() {
    let client = make_client().await;
    let create_res = client
        .create_resource_definition()
        .name("TestResource")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    let err = client
        .update_resource_definition()
        .resource_definition_id(&def_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_resource.py::test_create_resources_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_create_resource_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .create_resource_definition_version()
        .resource_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_resource.py::test_list_resource_definition_versions_with_invalid_id
#[tokio::test]
async fn test_moto_list_resource_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .list_resource_definition_versions()
        .resource_definition_id("fe2392e9-e67f-4308-af1b-ff94a128b231")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_resource.py::test_get_resource_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_get_resource_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_resource_definition_version()
        .resource_definition_id("fe2392e9-e67f-4308-af1b-ff94a128b231")
        .resource_definition_version_id("cd2ea6dc-6634-4e89-8441-8003500435f9")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_resource.py::test_get_resource_definition_version_with_invalid_version_id
#[tokio::test]
async fn test_moto_get_resource_def_ver_with_invalid_version_id() {
    let client = make_client().await;
    let create_res = client
        .create_resource_definition()
        .name("TestResource")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();
    let invalid_ver_id = "6fbffc21-989e-4d29-a793-a42f450a78c6";

    let err = client
        .get_resource_definition_version()
        .resource_definition_id(&def_id)
        .resource_definition_version_id(invalid_ver_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionNotFoundException"),
        "Expected VersionNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_greengrass_subscriptions.py
// ============================================================================

// Ported from moto: test_greengrass_subscriptions.py::test_get_subscription_definition_with_invalid_id
#[tokio::test]
async fn test_moto_get_subscription_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_subscription_definition()
        .subscription_definition_id("b552443b-1888-469b-81f8-0ebc5ca92949")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_subscriptions.py::test_delete_subscription_definition_with_invalid_id
#[tokio::test]
async fn test_moto_delete_subscription_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .delete_subscription_definition()
        .subscription_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_subscriptions.py::test_update_subscription_definition_with_empty_name
#[tokio::test]
async fn test_moto_update_subscription_definition_with_empty_name() {
    let client = make_client().await;
    let create_res = client
        .create_subscription_definition()
        .name("TestSubscription")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    let err = client
        .update_subscription_definition()
        .subscription_definition_id(&def_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_subscriptions.py::test_update_subscription_definition_with_invalid_id
#[tokio::test]
async fn test_moto_update_subscription_definition_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .update_subscription_definition()
        .subscription_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("123")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_subscriptions.py::test_create_subscription_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_create_subscription_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .create_subscription_definition_version()
        .subscription_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_subscriptions.py::test_list_subscription_definition_versions_with_invalid_id
#[tokio::test]
async fn test_moto_list_subscription_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .list_subscription_definition_versions()
        .subscription_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_subscriptions.py::test_get_subscription_definition_version_with_invalid_id
#[tokio::test]
async fn test_moto_get_subscription_def_ver_with_invalid_id() {
    let client = make_client().await;
    let err = client
        .get_subscription_definition_version()
        .subscription_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .subscription_definition_version_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_subscriptions.py::test_get_subscription_definition_version_with_invalid_version_id
#[tokio::test]
async fn test_moto_get_subscription_def_ver_with_invalid_version_id() {
    let client = make_client().await;
    let create_res = client
        .create_subscription_definition()
        .name("TestSubscription")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();
    let invalid_ver_id = "7b0bdeae-54c7-47cf-9f93-561e672efd9c";

    let err = client
        .get_subscription_definition_version()
        .subscription_definition_id(&def_id)
        .subscription_definition_version_id(invalid_ver_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionNotFoundException"),
        "Expected VersionNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_greengrass_deployment.py
// ============================================================================

// Ported from moto: test_greengrass_deployment.py::test_get_deployment_status
#[tokio::test]
async fn test_moto_get_deployment_status() {
    let client = make_client().await;
    let group = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    let deploy = client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .unwrap();

    let deployment_id = deploy.deployment_id().unwrap().to_string();
    let status = client
        .get_deployment_status()
        .group_id(&group_id)
        .deployment_id(&deployment_id)
        .send()
        .await
        .unwrap();

    assert_eq!(status.deployment_status().unwrap(), "InProgress");
    assert_eq!(status.deployment_type().unwrap().as_str(), "NewDeployment");
    assert!(status.updated_at().is_some());
}

// Ported from moto: test_greengrass_deployment.py::test_get_deployment_status_with_invalid_deployment_id
#[tokio::test]
async fn test_moto_get_deployment_status_with_invalid_deployment_id() {
    let client = make_client().await;
    let group = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();
    let deployment_id = "7b0bdeae-54c7-47cf-9f93-561e672efd9c";

    let err = client
        .get_deployment_status()
        .group_id(&group_id)
        .deployment_id(deployment_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidInputException") || err_str.contains("does not exist"),
        "Expected InvalidInputException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_deployment.py::test_create_deployment_with_invalid_group_id
#[tokio::test]
async fn test_moto_create_deployment_with_invalid_group_id() {
    let client = make_client().await;
    let err = client
        .create_deployment()
        .group_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .group_version_id("dfd06e54-6531-4a9b-9505-3c1036b6906a")
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_deployment.py::test_reset_deployments_with_invalid_group_id
#[tokio::test]
async fn test_moto_reset_deployments_with_invalid_group_id() {
    let client = make_client().await;
    let err = client
        .reset_deployments()
        .group_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_deployment.py::test_reset_deployments_with_never_deployed_group
#[tokio::test]
async fn test_moto_reset_deployments_with_never_deployed_group() {
    let client = make_client().await;
    let group = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    let err = client
        .reset_deployments()
        .group_id(&group_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("has not been deployed or has already been reset"),
        "Expected not deployed error, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_deployment.py::test_reset_deployments_with_already_reset_group
#[tokio::test]
async fn test_moto_reset_deployments_with_already_reset_group() {
    let client = make_client().await;
    let group = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    // Create a deployment first
    client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .unwrap();

    // Reset it
    client
        .reset_deployments()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();

    // Second reset should fail
    let err = client
        .reset_deployments()
        .group_id(&group_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("has not been deployed or has already been reset"),
        "Expected already reset error, got: {err_str}"
    );
}

// Ported from moto: test_greengrass_deployment.py::test_list_deployments (with deployment type and group_arn)
#[tokio::test]
async fn test_moto_list_deployments_with_type() {
    let client = make_client().await;
    let group = client
        .create_group()
        .name("TestGroup")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .unwrap();

    let list_res = client
        .list_deployments()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_res.deployments().len(), 1);
    let deployment = &list_res.deployments()[0];
    assert!(deployment.created_at().is_some());
    assert!(deployment.deployment_arn().is_some());
    assert!(deployment.deployment_id().is_some());
    assert_eq!(
        deployment.deployment_type().unwrap().as_str(),
        "NewDeployment"
    );
    assert!(deployment.group_arn().is_some());
}

// ---- Connector Definition tests ----

#[tokio::test]
async fn test_connector_definition_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_connector_definition()
        .name("my-connector-def")
        .send()
        .await
        .expect("create_connector_definition should succeed");

    let def_id = resp.id().unwrap().to_string();
    assert_eq!(resp.name().unwrap(), "my-connector-def");
    assert!(resp.arn().is_some());
    assert!(resp.latest_version().is_some());

    // Get
    let get_resp = client
        .get_connector_definition()
        .connector_definition_id(&def_id)
        .send()
        .await
        .expect("get_connector_definition should succeed");

    assert_eq!(get_resp.id().unwrap(), &def_id);
    assert_eq!(get_resp.name().unwrap(), "my-connector-def");

    // Update
    client
        .update_connector_definition()
        .connector_definition_id(&def_id)
        .name("updated-connector-def")
        .send()
        .await
        .expect("update_connector_definition should succeed");

    let get_resp2 = client
        .get_connector_definition()
        .connector_definition_id(&def_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp2.name().unwrap(), "updated-connector-def");

    // List
    let list_resp = client
        .list_connector_definitions()
        .send()
        .await
        .expect("list_connector_definitions should succeed");

    assert_eq!(list_resp.definitions().len(), 1);

    // Delete
    client
        .delete_connector_definition()
        .connector_definition_id(&def_id)
        .send()
        .await
        .expect("delete_connector_definition should succeed");

    let result = client
        .get_connector_definition()
        .connector_definition_id(&def_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_connector_definition_version() {
    let client = make_client().await;

    let def = client
        .create_connector_definition()
        .name("conn-ver-test")
        .send()
        .await
        .unwrap();
    let def_id = def.id().unwrap().to_string();

    // Create version
    let ver = client
        .create_connector_definition_version()
        .connector_definition_id(&def_id)
        .send()
        .await
        .expect("create_connector_definition_version should succeed");

    assert!(ver.arn().is_some());
    let version_id = ver.version().unwrap().to_string();

    // Get version
    let get_ver = client
        .get_connector_definition_version()
        .connector_definition_id(&def_id)
        .connector_definition_version_id(&version_id)
        .send()
        .await
        .expect("get_connector_definition_version should succeed");

    assert_eq!(get_ver.version().unwrap(), &version_id);

    // List versions (initial + new = 2)
    let list_ver = client
        .list_connector_definition_versions()
        .connector_definition_id(&def_id)
        .send()
        .await
        .expect("list_connector_definition_versions should succeed");

    assert_eq!(list_ver.versions().len(), 2);
}

#[tokio::test]
async fn test_delete_nonexistent_connector_definition() {
    let client = make_client().await;

    let result = client
        .delete_connector_definition()
        .connector_definition_id("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Logger Definition tests ----

#[tokio::test]
async fn test_logger_definition_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_logger_definition()
        .name("my-logger-def")
        .send()
        .await
        .expect("create_logger_definition should succeed");

    let def_id = resp.id().unwrap().to_string();
    assert_eq!(resp.name().unwrap(), "my-logger-def");
    assert!(resp.arn().is_some());
    assert!(resp.latest_version().is_some());

    // Get
    let get_resp = client
        .get_logger_definition()
        .logger_definition_id(&def_id)
        .send()
        .await
        .expect("get_logger_definition should succeed");

    assert_eq!(get_resp.id().unwrap(), &def_id);
    assert_eq!(get_resp.name().unwrap(), "my-logger-def");

    // Update
    client
        .update_logger_definition()
        .logger_definition_id(&def_id)
        .name("updated-logger-def")
        .send()
        .await
        .expect("update_logger_definition should succeed");

    let get_resp2 = client
        .get_logger_definition()
        .logger_definition_id(&def_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp2.name().unwrap(), "updated-logger-def");

    // List
    let list_resp = client
        .list_logger_definitions()
        .send()
        .await
        .expect("list_logger_definitions should succeed");

    assert_eq!(list_resp.definitions().len(), 1);

    // Delete
    client
        .delete_logger_definition()
        .logger_definition_id(&def_id)
        .send()
        .await
        .expect("delete_logger_definition should succeed");

    let result = client
        .get_logger_definition()
        .logger_definition_id(&def_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_logger_definition_version() {
    let client = make_client().await;

    let def = client
        .create_logger_definition()
        .name("log-ver-test")
        .send()
        .await
        .unwrap();
    let def_id = def.id().unwrap().to_string();

    // Create version
    let ver = client
        .create_logger_definition_version()
        .logger_definition_id(&def_id)
        .send()
        .await
        .expect("create_logger_definition_version should succeed");

    assert!(ver.arn().is_some());
    let version_id = ver.version().unwrap().to_string();

    // Get version
    let get_ver = client
        .get_logger_definition_version()
        .logger_definition_id(&def_id)
        .logger_definition_version_id(&version_id)
        .send()
        .await
        .expect("get_logger_definition_version should succeed");

    assert_eq!(get_ver.version().unwrap(), &version_id);

    // List versions (initial + new = 2)
    let list_ver = client
        .list_logger_definition_versions()
        .logger_definition_id(&def_id)
        .send()
        .await
        .expect("list_logger_definition_versions should succeed");

    assert_eq!(list_ver.versions().len(), 2);
}

#[tokio::test]
async fn test_delete_nonexistent_logger_definition() {
    let client = make_client().await;

    let result = client
        .delete_logger_definition()
        .logger_definition_id("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Connector Definition error path tests
// ============================================================================

#[tokio::test]
async fn test_get_nonexistent_connector_definition() {
    let client = make_client().await;

    let err = client
        .get_connector_definition()
        .connector_definition_id("b552443b-1888-469b-81f8-0ebc5ca92949")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_connector_definition_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .update_connector_definition()
        .connector_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("something")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_connector_definition_with_empty_name() {
    let client = make_client().await;

    let create_res = client
        .create_connector_definition()
        .name("TestConnector")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    let err = client
        .update_connector_definition()
        .connector_definition_id(&def_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException")
            || err_str.contains("Input does not contain any attributes to be updated"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_connector_definition_version_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .create_connector_definition_version()
        .connector_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_connector_definition_versions_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .list_connector_definition_versions()
        .connector_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_connector_definition_version_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .get_connector_definition_version()
        .connector_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .connector_definition_version_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_connector_definition_version_with_invalid_version_id() {
    let client = make_client().await;

    let create_res = client
        .create_connector_definition()
        .name("TestConnector")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();
    let invalid_ver_id = "6fbffc21-989e-4d29-a793-a42f450a78c6";

    let err = client
        .get_connector_definition_version()
        .connector_definition_id(&def_id)
        .connector_definition_version_id(invalid_ver_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionNotFoundException") || err_str.contains("IdNotFoundException"),
        "Expected version not found error, got: {err_str}"
    );
}

// ============================================================================
// Logger Definition error path tests
// ============================================================================

#[tokio::test]
async fn test_get_nonexistent_logger_definition() {
    let client = make_client().await;

    let err = client
        .get_logger_definition()
        .logger_definition_id("b552443b-1888-469b-81f8-0ebc5ca92949")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_logger_definition_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .update_logger_definition()
        .logger_definition_id("6fbffc21-989e-4d29-a793-a42f450a78c6")
        .name("something")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_logger_definition_with_empty_name() {
    let client = make_client().await;

    let create_res = client
        .create_logger_definition()
        .name("TestLogger")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();

    let err = client
        .update_logger_definition()
        .logger_definition_id(&def_id)
        .name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidContainerDefinitionException")
            || err_str.contains("Input does not contain any attributes to be updated"),
        "Expected InvalidContainerDefinitionException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_logger_definition_version_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .create_logger_definition_version()
        .logger_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_logger_definition_versions_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .list_logger_definition_versions()
        .logger_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_logger_definition_version_with_invalid_id() {
    let client = make_client().await;

    let err = client
        .get_logger_definition_version()
        .logger_definition_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .logger_definition_version_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException"),
        "Expected IdNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_logger_definition_version_with_invalid_version_id() {
    let client = make_client().await;

    let create_res = client
        .create_logger_definition()
        .name("TestLogger")
        .send()
        .await
        .unwrap();
    let def_id = create_res.id().unwrap().to_string();
    let invalid_ver_id = "6fbffc21-989e-4d29-a793-a42f450a78c6";

    let err = client
        .get_logger_definition_version()
        .logger_definition_id(&def_id)
        .logger_definition_version_id(invalid_ver_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionNotFoundException") || err_str.contains("IdNotFoundException"),
        "Expected version not found error, got: {err_str}"
    );
}

// ============================================================================
// Group version with definition ARN bindings
// ============================================================================

#[tokio::test]
async fn test_create_group_version_with_definition_arns() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("group-with-defs")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    let core_def = client
        .create_core_definition()
        .name("core-for-group")
        .send()
        .await
        .unwrap();
    let core_def_version_arn = core_def.latest_version_arn().unwrap().to_string();

    let func_def = client
        .create_function_definition()
        .name("func-for-group")
        .send()
        .await
        .unwrap();
    let func_def_version_arn = func_def.latest_version_arn().unwrap().to_string();

    let gv = client
        .create_group_version()
        .group_id(&group_id)
        .core_definition_version_arn(&core_def_version_arn)
        .function_definition_version_arn(&func_def_version_arn)
        .send()
        .await
        .expect("create_group_version with ARNs should succeed");

    assert!(gv.arn().is_some());
    assert!(gv.version().is_some());

    let version_id = gv.version().unwrap().to_string();
    let get_gv = client
        .get_group_version()
        .group_id(&group_id)
        .group_version_id(&version_id)
        .send()
        .await
        .expect("get_group_version should succeed");

    let def = get_gv.definition().expect("definition should be present");
    assert_eq!(
        def.core_definition_version_arn().unwrap(),
        &core_def_version_arn
    );
    assert_eq!(
        def.function_definition_version_arn().unwrap(),
        &func_def_version_arn
    );
}

// ============================================================================
// Deployment: multiple deployments and listing
// ============================================================================

#[tokio::test]
async fn test_list_deployments_multiple() {
    let client = make_client().await;

    let group = client
        .create_group()
        .name("multi-deploy-group")
        .send()
        .await
        .unwrap();
    let group_id = group.id().unwrap().to_string();

    client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::NewDeployment)
        .send()
        .await
        .unwrap();

    client
        .create_deployment()
        .group_id(&group_id)
        .deployment_type(aws_sdk_greengrass::types::DeploymentType::Redeployment)
        .send()
        .await
        .unwrap();

    let list_res = client
        .list_deployments()
        .group_id(&group_id)
        .send()
        .await
        .expect("list_deployments should succeed");

    assert_eq!(list_res.deployments().len(), 2);
}

#[tokio::test]
async fn test_list_deployments_nonexistent_group() {
    let client = make_client().await;

    let err = client
        .list_deployments()
        .group_id("7b0bdeae-54c7-47cf-9f93-561e672efd9c")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IdNotFoundException") || err_str.contains("ResourceNotFoundException"),
        "Expected not found error, got: {err_str}"
    );
}

// ============================================================================
// Group: ARN and timestamp fields
// ============================================================================

#[tokio::test]
async fn test_group_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("arn-test-group")
        .send()
        .await
        .unwrap();

    let group_id = resp.id().unwrap();
    let arn = resp.arn().expect("arn should be present");
    assert!(
        arn.contains("arn:aws:greengrass:"),
        "ARN should start with arn:aws:greengrass:"
    );
    assert!(arn.contains(group_id), "ARN should contain the group ID");
    assert!(
        resp.creation_timestamp().is_some(),
        "creation_timestamp should be present"
    );
    assert!(
        resp.last_updated_timestamp().is_some(),
        "last_updated_timestamp should be present"
    );
    assert!(
        resp.latest_version().is_some(),
        "latest_version should be present"
    );
    assert!(
        resp.latest_version_arn().is_some(),
        "latest_version_arn should be present"
    );
}

// ============================================================================
// Cross-definition: multiple definition types in parallel
// ============================================================================

#[tokio::test]
async fn test_multiple_definition_types_independent() {
    let client = make_client().await;

    // Create one of each definition type
    let core = client
        .create_core_definition()
        .name("core-a")
        .send()
        .await
        .unwrap();
    let device = client
        .create_device_definition()
        .name("device-a")
        .send()
        .await
        .unwrap();
    let func = client
        .create_function_definition()
        .name("func-a")
        .send()
        .await
        .unwrap();
    let resource = client
        .create_resource_definition()
        .name("resource-a")
        .send()
        .await
        .unwrap();
    let subscription = client
        .create_subscription_definition()
        .name("sub-a")
        .send()
        .await
        .unwrap();
    let connector = client
        .create_connector_definition()
        .name("connector-a")
        .send()
        .await
        .unwrap();
    let logger = client
        .create_logger_definition()
        .name("logger-a")
        .send()
        .await
        .unwrap();

    // Each list should contain exactly 1 definition (state is isolated per client)
    assert_eq!(
        client
            .list_core_definitions()
            .send()
            .await
            .unwrap()
            .definitions()
            .len(),
        1
    );
    assert_eq!(
        client
            .list_device_definitions()
            .send()
            .await
            .unwrap()
            .definitions()
            .len(),
        1
    );
    assert_eq!(
        client
            .list_function_definitions()
            .send()
            .await
            .unwrap()
            .definitions()
            .len(),
        1
    );
    assert_eq!(
        client
            .list_resource_definitions()
            .send()
            .await
            .unwrap()
            .definitions()
            .len(),
        1
    );
    assert_eq!(
        client
            .list_subscription_definitions()
            .send()
            .await
            .unwrap()
            .definitions()
            .len(),
        1
    );
    assert_eq!(
        client
            .list_connector_definitions()
            .send()
            .await
            .unwrap()
            .definitions()
            .len(),
        1
    );
    assert_eq!(
        client
            .list_logger_definitions()
            .send()
            .await
            .unwrap()
            .definitions()
            .len(),
        1
    );

    // IDs are all distinct
    let ids = [
        core.id().unwrap(),
        device.id().unwrap(),
        func.id().unwrap(),
        resource.id().unwrap(),
        subscription.id().unwrap(),
        connector.id().unwrap(),
        logger.id().unwrap(),
    ];
    let unique_ids: std::collections::HashSet<&str> = ids.iter().copied().collect();
    assert_eq!(unique_ids.len(), 7, "all definition IDs must be unique");
}
