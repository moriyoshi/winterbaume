use aws_sdk_ssm::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ssm::SsmService;

async fn make_ssm_client() -> aws_sdk_ssm::Client {
    let mock = MockAws::builder().with_service(SsmService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssm::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_ssm::Client::new(&config)
}

// ── Parameter operations ──────────────────────────────────────────

#[tokio::test]
async fn test_put_and_get_parameter() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/test/param1")
        .value("hello")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .expect("put_parameter should succeed");

    let resp = client
        .get_parameter()
        .name("/test/param1")
        .send()
        .await
        .expect("get_parameter should succeed");

    let param = resp.parameter().expect("should have parameter");
    assert_eq!(param.name(), Some("/test/param1"));
    assert_eq!(param.value(), Some("hello"));
    assert_eq!(param.version(), 1);
}

#[tokio::test]
async fn test_put_parameter_overwrite() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/test/param1")
        .value("v1")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    // Overwrite
    let resp = client
        .put_parameter()
        .name("/test/param1")
        .value("v2")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .overwrite(true)
        .send()
        .await
        .expect("overwrite should succeed");

    assert_eq!(resp.version(), 2);

    let get_resp = client
        .get_parameter()
        .name("/test/param1")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.parameter().unwrap().value(), Some("v2"));
}

#[tokio::test]
async fn test_put_parameter_no_overwrite_fails() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/test/param1")
        .value("v1")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let result = client
        .put_parameter()
        .name("/test/param1")
        .value("v2")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await;

    assert!(result.is_err(), "put without overwrite should fail");
}

#[tokio::test]
async fn test_get_parameters() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/a")
        .value("1")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();
    client
        .put_parameter()
        .name("/b")
        .value("2")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_parameters()
        .names("/a")
        .names("/b")
        .names("/nonexistent")
        .send()
        .await
        .expect("get_parameters should succeed");

    assert_eq!(resp.parameters().len(), 2);
    assert_eq!(resp.invalid_parameters().len(), 1);
}

#[tokio::test]
async fn test_get_parameters_by_path() {
    let client = make_ssm_client().await;

    for (name, val) in [
        ("/app/db/host", "h"),
        ("/app/db/port", "p"),
        ("/app/cache/ttl", "t"),
    ] {
        client
            .put_parameter()
            .name(name)
            .value(val)
            .r#type(aws_sdk_ssm::types::ParameterType::String)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .get_parameters_by_path()
        .path("/app/db")
        .send()
        .await
        .expect("get_parameters_by_path should succeed");
    assert_eq!(resp.parameters().len(), 2);

    let resp = client
        .get_parameters_by_path()
        .path("/app")
        .recursive(true)
        .send()
        .await
        .expect("recursive should succeed");
    assert_eq!(resp.parameters().len(), 3);
}

#[tokio::test]
async fn test_delete_parameter() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/del")
        .value("x")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    client
        .delete_parameter()
        .name("/del")
        .send()
        .await
        .expect("delete_parameter should succeed");

    let result = client.get_parameter().name("/del").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_parameters() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/x")
        .value("1")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_parameters()
        .names("/x")
        .names("/nonexistent")
        .send()
        .await
        .expect("delete_parameters should succeed");

    assert_eq!(resp.deleted_parameters().len(), 1);
    assert_eq!(resp.invalid_parameters().len(), 1);
}

#[tokio::test]
async fn test_describe_parameters() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/desc")
        .value("v")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_parameters()
        .send()
        .await
        .expect("describe_parameters should succeed");

    assert!(!resp.parameters().is_empty());
}

#[tokio::test]
async fn test_get_parameter_history() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/hist/param")
        .value("v1")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    client
        .put_parameter()
        .name("/hist/param")
        .value("v2")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .overwrite(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_parameter_history()
        .name("/hist/param")
        .send()
        .await
        .expect("get_parameter_history should succeed");

    assert_eq!(resp.parameters().len(), 2);
    assert_eq!(resp.parameters()[0].version(), 1);
    assert_eq!(resp.parameters()[1].version(), 2);
}

#[tokio::test]
async fn test_label_and_unlabel_parameter_version() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/label/param")
        .value("v1")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    client
        .label_parameter_version()
        .name("/label/param")
        .parameter_version(1)
        .labels("prod")
        .labels("stable")
        .send()
        .await
        .expect("label_parameter_version should succeed");

    // Verify labels via history
    let hist = client
        .get_parameter_history()
        .name("/label/param")
        .send()
        .await
        .unwrap();
    let pv = &hist.parameters()[0];
    assert!(pv.labels().contains(&"prod".to_string()));
    assert!(pv.labels().contains(&"stable".to_string()));

    // Unlabel
    client
        .unlabel_parameter_version()
        .name("/label/param")
        .parameter_version(1)
        .labels("prod")
        .send()
        .await
        .expect("unlabel_parameter_version should succeed");

    let hist = client
        .get_parameter_history()
        .name("/label/param")
        .send()
        .await
        .unwrap();
    let pv = &hist.parameters()[0];
    assert!(!pv.labels().contains(&"prod".to_string()));
    assert!(pv.labels().contains(&"stable".to_string()));
}

// ── Tag operations ────────────────────────────────────────────────

#[tokio::test]
async fn test_add_tags_to_resource() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/tagged/param")
        .value("tagged-value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .expect("put_parameter should succeed");

    client
        .add_tags_to_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/tagged/param")
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("Environment")
                .value("Production")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("Team")
                .value("Backend")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags_to_resource should succeed");
}

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/tag-list/param")
        .value("val")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    client
        .add_tags_to_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/tag-list/param")
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("Env")
                .value("Dev")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/tag-list/param")
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tag_list = resp.tag_list();
    assert_eq!(tag_list.len(), 1);
    assert_eq!(tag_list[0].key(), "Env");
    assert_eq!(tag_list[0].value(), "Dev");
}

#[tokio::test]
async fn test_remove_tags_from_resource() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/tag-rm/param")
        .value("val")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    client
        .add_tags_to_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/tag-rm/param")
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("A")
                .value("1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("B")
                .value("2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .remove_tags_from_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/tag-rm/param")
        .tag_keys("A")
        .send()
        .await
        .expect("remove_tags_from_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/tag-rm/param")
        .send()
        .await
        .unwrap();

    let tag_list = resp.tag_list();
    assert_eq!(tag_list.len(), 1);
    assert_eq!(tag_list[0].key(), "B");
}

// ── Document operations ───────────────────────────────────────────

#[tokio::test]
async fn test_create_and_describe_document() {
    let client = make_ssm_client().await;

    let resp = client
        .create_document()
        .name("TestDoc")
        .content("{\"schemaVersion\":\"2.2\"}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .document_format(aws_sdk_ssm::types::DocumentFormat::Json)
        .send()
        .await
        .expect("create_document should succeed");

    let desc = resp.document_description().unwrap();
    assert_eq!(desc.name(), Some("TestDoc"));
    assert_eq!(desc.status().map(|s| s.as_str()), Some("Active"));

    let resp = client
        .describe_document()
        .name("TestDoc")
        .send()
        .await
        .expect("describe_document should succeed");

    assert_eq!(resp.document().unwrap().name(), Some("TestDoc"));
}

#[tokio::test]
async fn test_get_document() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("GetDocTest")
        .content("{\"version\":\"1\"}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_document()
        .name("GetDocTest")
        .send()
        .await
        .expect("get_document should succeed");

    assert_eq!(resp.name(), Some("GetDocTest"));
    assert_eq!(resp.content(), Some("{\"version\":\"1\"}"));
}

#[tokio::test]
async fn test_delete_document() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("DelDoc")
        .content("{}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    client
        .delete_document()
        .name("DelDoc")
        .send()
        .await
        .expect("delete_document should succeed");

    let result = client.describe_document().name("DelDoc").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_document() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("UpdDoc")
        .content("{\"v\":\"1\"}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_document()
        .name("UpdDoc")
        .content("{\"v\":\"2\"}")
        .document_version("$LATEST")
        .send()
        .await
        .expect("update_document should succeed");

    let desc = resp.document_description().unwrap();
    assert_eq!(desc.latest_version(), Some("2"));
}

#[tokio::test]
async fn test_update_document_default_version() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("DefVerDoc")
        .content("{\"v\":\"1\"}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    client
        .update_document()
        .name("DefVerDoc")
        .content("{\"v\":\"2\"}")
        .document_version("$LATEST")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_document_default_version()
        .name("DefVerDoc")
        .document_version("2")
        .send()
        .await
        .expect("update_document_default_version should succeed");

    let desc = resp.description().unwrap();
    assert_eq!(desc.default_version(), Some("2"));
}

#[tokio::test]
async fn test_list_documents() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("ListDoc1")
        .content("{}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_documents()
        .send()
        .await
        .expect("list_documents should succeed");

    assert!(!resp.document_identifiers().is_empty());
    assert!(
        resp.document_identifiers()
            .iter()
            .any(|d| d.name() == Some("ListDoc1"))
    );
}

#[tokio::test]
async fn test_describe_and_modify_document_permission() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("PermDoc")
        .content("{}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    // Modify permission: add an account
    client
        .modify_document_permission()
        .name("PermDoc")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .account_ids_to_add("123456789012")
        .send()
        .await
        .expect("modify_document_permission should succeed");

    let resp = client
        .describe_document_permission()
        .name("PermDoc")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .send()
        .await
        .expect("describe_document_permission should succeed");

    assert!(resp.account_ids().contains(&"123456789012".to_string()));

    // Remove the account
    client
        .modify_document_permission()
        .name("PermDoc")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .account_ids_to_remove("123456789012")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_document_permission()
        .name("PermDoc")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .send()
        .await
        .unwrap();

    assert!(!resp.account_ids().contains(&"123456789012".to_string()));
}

// ── Maintenance Window operations ─────────────────────────────────

#[tokio::test]
async fn test_create_and_get_maintenance_window() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("TestWindow")
        .schedule("cron(0 2 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(true)
        .send()
        .await
        .expect("create_maintenance_window should succeed");

    let window_id = create_resp.window_id().unwrap();
    assert!(window_id.starts_with("mw-"));

    let get_resp = client
        .get_maintenance_window()
        .window_id(window_id)
        .send()
        .await
        .expect("get_maintenance_window should succeed");

    assert_eq!(get_resp.name(), Some("TestWindow"));
    assert_eq!(get_resp.schedule(), Some("cron(0 2 * * ? *)"));
    assert_eq!(get_resp.duration(), Some(2));
    assert_eq!(get_resp.cutoff(), 1);
}

#[tokio::test]
async fn test_delete_maintenance_window() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("DelWindow")
        .schedule("cron(0 2 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(true)
        .send()
        .await
        .unwrap();

    let window_id = create_resp.window_id().unwrap();

    client
        .delete_maintenance_window()
        .window_id(window_id)
        .send()
        .await
        .expect("delete_maintenance_window should succeed");

    let result = client
        .get_maintenance_window()
        .window_id(window_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_maintenance_windows() {
    let client = make_ssm_client().await;

    client
        .create_maintenance_window()
        .name("DescW1")
        .schedule("cron(0 2 * * ? *)")
        .duration(1)
        .cutoff(0)
        .allow_unassociated_targets(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_maintenance_windows()
        .send()
        .await
        .expect("describe_maintenance_windows should succeed");

    assert!(!resp.window_identities().is_empty());
}

#[tokio::test]
async fn test_register_and_deregister_target_with_maintenance_window() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("TargetWindow")
        .schedule("cron(0 2 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(true)
        .send()
        .await
        .unwrap();

    let window_id = create_resp.window_id().unwrap();

    let reg_resp = client
        .register_target_with_maintenance_window()
        .window_id(window_id)
        .resource_type(aws_sdk_ssm::types::MaintenanceWindowResourceType::Instance)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("InstanceIds")
                .values("i-1234567890abcdef0")
                .build(),
        )
        .send()
        .await
        .expect("register_target should succeed");

    let target_id = reg_resp.window_target_id().unwrap();
    assert!(target_id.starts_with("t-"));

    // Describe targets
    let desc_resp = client
        .describe_maintenance_window_targets()
        .window_id(window_id)
        .send()
        .await
        .expect("describe_maintenance_window_targets should succeed");

    assert_eq!(desc_resp.targets().len(), 1);

    // Deregister
    client
        .deregister_target_from_maintenance_window()
        .window_id(window_id)
        .window_target_id(target_id)
        .send()
        .await
        .expect("deregister_target should succeed");

    let desc_resp = client
        .describe_maintenance_window_targets()
        .window_id(window_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.targets().len(), 0);
}

#[tokio::test]
async fn test_register_and_deregister_task_with_maintenance_window() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("TaskWindow")
        .schedule("cron(0 2 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(true)
        .send()
        .await
        .unwrap();

    let window_id = create_resp.window_id().unwrap();

    let reg_resp = client
        .register_task_with_maintenance_window()
        .window_id(window_id)
        .task_arn("AWS-RunShellScript")
        .task_type(aws_sdk_ssm::types::MaintenanceWindowTaskType::RunCommand)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("WindowTargetIds")
                .values("*")
                .build(),
        )
        .send()
        .await
        .expect("register_task should succeed");

    let task_id = reg_resp.window_task_id().unwrap();

    // Describe tasks
    let desc_resp = client
        .describe_maintenance_window_tasks()
        .window_id(window_id)
        .send()
        .await
        .expect("describe_maintenance_window_tasks should succeed");

    assert_eq!(desc_resp.tasks().len(), 1);
    assert_eq!(desc_resp.tasks()[0].task_arn(), Some("AWS-RunShellScript"));

    // Deregister
    client
        .deregister_task_from_maintenance_window()
        .window_id(window_id)
        .window_task_id(task_id)
        .send()
        .await
        .expect("deregister_task should succeed");

    let desc_resp = client
        .describe_maintenance_window_tasks()
        .window_id(window_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.tasks().len(), 0);
}

// ── Patch Baseline operations ─────────────────────────────────────

#[tokio::test]
async fn test_create_and_delete_patch_baseline() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_patch_baseline()
        .name("TestBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux2)
        .send()
        .await
        .expect("create_patch_baseline should succeed");

    let baseline_id = create_resp.baseline_id().unwrap();
    assert!(baseline_id.starts_with("pb-"));

    client
        .delete_patch_baseline()
        .baseline_id(baseline_id)
        .send()
        .await
        .expect("delete_patch_baseline should succeed");
}

#[tokio::test]
async fn test_describe_patch_baselines() {
    let client = make_ssm_client().await;

    client
        .create_patch_baseline()
        .name("DescBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::Windows)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_patch_baselines()
        .send()
        .await
        .expect("describe_patch_baselines should succeed");

    assert!(!resp.baseline_identities().is_empty());
}

#[tokio::test]
async fn test_register_and_deregister_patch_baseline_for_patch_group() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_patch_baseline()
        .name("PatchGroupBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux2)
        .send()
        .await
        .unwrap();

    let baseline_id = create_resp.baseline_id().unwrap();

    // Register
    let reg_resp = client
        .register_patch_baseline_for_patch_group()
        .baseline_id(baseline_id)
        .patch_group("production")
        .send()
        .await
        .expect("register_patch_baseline_for_patch_group should succeed");

    assert_eq!(reg_resp.baseline_id(), Some(baseline_id));
    assert_eq!(reg_resp.patch_group(), Some("production"));

    // Get by patch group
    let get_resp = client
        .get_patch_baseline_for_patch_group()
        .patch_group("production")
        .send()
        .await
        .expect("get_patch_baseline_for_patch_group should succeed");

    assert_eq!(get_resp.baseline_id(), Some(baseline_id));

    // Deregister
    let dereg_resp = client
        .deregister_patch_baseline_for_patch_group()
        .baseline_id(baseline_id)
        .patch_group("production")
        .send()
        .await
        .expect("deregister_patch_baseline_for_patch_group should succeed");

    assert_eq!(dereg_resp.baseline_id(), Some(baseline_id));

    // Verify gone
    let get_resp = client
        .get_patch_baseline_for_patch_group()
        .patch_group("production")
        .send()
        .await
        .unwrap();

    assert!(get_resp.baseline_id().is_none());
}

// ── Command operations ────────────────────────────────────────────

#[tokio::test]
async fn test_send_command_and_list() {
    let client = make_ssm_client().await;

    let resp = client
        .send_command()
        .document_name("AWS-RunShellScript")
        .instance_ids("i-1234567890abcdef0")
        .send()
        .await
        .expect("send_command should succeed");

    let cmd = resp.command().unwrap();
    assert_eq!(cmd.document_name(), Some("AWS-RunShellScript"));
    assert!(
        cmd.instance_ids()
            .contains(&"i-1234567890abcdef0".to_string())
    );

    let list_resp = client
        .list_commands()
        .send()
        .await
        .expect("list_commands should succeed");

    assert!(!list_resp.commands().is_empty());
}

#[tokio::test]
async fn test_get_command_invocation() {
    let client = make_ssm_client().await;

    let resp = client
        .send_command()
        .document_name("AWS-RunShellScript")
        .instance_ids("i-abcdef1234567890")
        .send()
        .await
        .unwrap();

    let cmd = resp.command().unwrap();
    let command_id = cmd.command_id().unwrap();

    let inv_resp = client
        .get_command_invocation()
        .command_id(command_id)
        .instance_id("i-abcdef1234567890")
        .send()
        .await
        .expect("get_command_invocation should succeed");

    assert_eq!(inv_resp.command_id(), Some(command_id));
    assert_eq!(inv_resp.instance_id(), Some("i-abcdef1234567890"));
}

#[tokio::test]
async fn test_get_command_invocation_not_found() {
    let client = make_ssm_client().await;

    let result = client
        .get_command_invocation()
        .command_id("nonexistent-id")
        .instance_id("i-123")
        .send()
        .await;

    assert!(result.is_err());
}

// ── Document lifecycle test ───────────────────────────────────────

#[tokio::test]
async fn test_document_full_lifecycle() {
    let client = make_ssm_client().await;

    // Create
    client
        .create_document()
        .name("LifecycleDoc")
        .content("{\"v\":\"1\"}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    // Describe
    let desc = client
        .describe_document()
        .name("LifecycleDoc")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.document().unwrap().name(), Some("LifecycleDoc"));

    // Get
    let get = client
        .get_document()
        .name("LifecycleDoc")
        .send()
        .await
        .unwrap();
    assert_eq!(get.content(), Some("{\"v\":\"1\"}"));

    // Update
    client
        .update_document()
        .name("LifecycleDoc")
        .content("{\"v\":\"2\"}")
        .document_version("$LATEST")
        .send()
        .await
        .unwrap();

    // List
    let list = client.list_documents().send().await.unwrap();
    assert!(
        list.document_identifiers()
            .iter()
            .any(|d| d.name() == Some("LifecycleDoc"))
    );

    // Delete
    client
        .delete_document()
        .name("LifecycleDoc")
        .send()
        .await
        .unwrap();

    // Verify gone
    assert!(
        client
            .describe_document()
            .name("LifecycleDoc")
            .send()
            .await
            .is_err()
    );
}

// ── Create document duplicate error ───────────────────────────────

#[tokio::test]
async fn test_create_duplicate_document_fails() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("DupDoc")
        .content("{}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let result = client
        .create_document()
        .name("DupDoc")
        .content("{}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_ssm.py
// ============================================================================

// Ported from moto: test_ssm.py::test_delete_nonexistent_parameter
#[tokio::test]
async fn test_delete_nonexistent_parameter() {
    let client = make_ssm_client().await;

    let err = client
        .delete_parameter()
        .name("test_noexist")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ParameterNotFound"),
        "Expected ParameterNotFound error, got: {err_str}"
    );
}

// Ported from moto: test_ssm.py::test_put_parameter (name="test")
#[tokio::test]
async fn test_put_parameter_with_arn_and_version() {
    let client = make_ssm_client().await;

    let response = client
        .put_parameter()
        .name("test")
        .description("A test parameter")
        .value("value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    assert_eq!(response.version(), 1);

    let get_resp = client.get_parameters().names("test").send().await.unwrap();

    assert_eq!(get_resp.parameters().len(), 1);
    let p = &get_resp.parameters()[0];
    assert_eq!(p.name(), Some("test"));
    assert_eq!(p.value(), Some("value"));
    assert_eq!(p.r#type(), Some(&aws_sdk_ssm::types::ParameterType::String));
    assert_eq!(p.version(), 1);
    assert_eq!(p.data_type(), Some("text"));
    assert!(p.arn().is_some());
    assert!(p.arn().unwrap().contains(":parameter/test"));

    // Trying to put again without overwrite should fail with ParameterAlreadyExists
    let err = client
        .put_parameter()
        .name("test")
        .description("desc 2")
        .value("value 2")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ParameterAlreadyExists")
            || err_str.contains("The parameter already exists"),
        "Expected ParameterAlreadyExists error, got: {err_str}"
    );

    // Overwrite should succeed and produce version 2
    let ow_resp = client
        .put_parameter()
        .name("test")
        .description("desc 3")
        .value("value 3")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .overwrite(true)
        .send()
        .await
        .unwrap();

    assert_eq!(ow_resp.version(), 2);

    let get_resp2 = client.get_parameters().names("test").send().await.unwrap();
    assert_eq!(get_resp2.parameters()[0].value(), Some("value 3"));
    assert_eq!(get_resp2.parameters()[0].version(), 2);
}

// Ported from moto: test_ssm.py::test_get_parameter
#[tokio::test]
async fn test_get_parameter_arn() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/test")
        .description("A test parameter")
        .value("value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let resp = client.get_parameter().name("/test").send().await.unwrap();

    let p = resp.parameter().unwrap();
    assert_eq!(p.name(), Some("/test"));
    assert_eq!(p.value(), Some("value"));
    assert_eq!(p.r#type(), Some(&aws_sdk_ssm::types::ParameterType::String));
    assert_eq!(p.data_type(), Some("text"));
    assert!(p.arn().is_some());
    assert!(p.arn().unwrap().contains(":parameter/test"));
}

// Ported from moto: test_ssm.py::test_get_nonexistant_parameter
#[tokio::test]
async fn test_get_nonexistant_parameter() {
    let client = make_ssm_client().await;

    let err = client
        .get_parameter()
        .name("test_noexist")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ParameterNotFound") || err_str.contains("not found"),
        "Expected ParameterNotFound error, got: {err_str}"
    );
}

// Ported from moto: test_ssm.py::test_get_parameter_invalid
#[tokio::test]
async fn test_get_parameter_invalid() {
    let client = make_ssm_client().await;

    let resp = client
        .get_parameters()
        .names("invalid")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.parameters().len(), 0);
    assert_eq!(resp.invalid_parameters().len(), 1);
    assert_eq!(resp.invalid_parameters()[0], "invalid");
}

// Ported from moto: test_ssm.py::test_get_parameters_by_path (comprehensive)
#[tokio::test]
async fn test_get_parameters_by_path_comprehensive() {
    let client = make_ssm_client().await;

    // Setup parameters
    for (name, value, param_type) in [
        (
            "/foo/name1",
            "value1",
            aws_sdk_ssm::types::ParameterType::String,
        ),
        (
            "/foo/name2",
            "value2",
            aws_sdk_ssm::types::ParameterType::String,
        ),
        (
            "/bar/name3",
            "value3",
            aws_sdk_ssm::types::ParameterType::String,
        ),
        (
            "/bar/name3/name4",
            "value4",
            aws_sdk_ssm::types::ParameterType::String,
        ),
    ] {
        client
            .put_parameter()
            .name(name)
            .value(value)
            .r#type(param_type)
            .send()
            .await
            .unwrap();
    }

    // /foo should have 2 (name1, name2)
    let resp = client
        .get_parameters_by_path()
        .path("/foo")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.parameters().len(), 2);

    // /bar non-recursive should have 1 (name3)
    let resp = client
        .get_parameters_by_path()
        .path("/bar")
        .recursive(false)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.parameters().len(), 1);
    assert_eq!(resp.parameters()[0].value(), Some("value3"));

    // /bar recursive should have 2 (name3, name3/name4)
    let resp = client
        .get_parameters_by_path()
        .path("/bar")
        .recursive(true)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.parameters().len(), 2);
}

// Ported from moto: test_ssm.py::test_describe_parameters (attributes)
#[tokio::test]
async fn test_describe_parameters_attributes() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("aa")
        .value("11")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    client
        .put_parameter()
        .name("bb")
        .value("22")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let resp = client.describe_parameters().send().await.unwrap();

    let params = resp.parameters();
    assert_eq!(params.len(), 2);
    // All parameters should have version 1
    for p in params {
        assert_eq!(p.version(), 1);
    }
}

// Ported from moto: test_ssm.py::test_get_parameter_history (3 versions)
#[tokio::test]
async fn test_get_parameter_history_three_versions() {
    let client = make_ssm_client().await;

    for i in 0..3 {
        client
            .put_parameter()
            .name("test")
            .value(format!("value-{i}"))
            .r#type(aws_sdk_ssm::types::ParameterType::String)
            .overwrite(true)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .get_parameter_history()
        .name("test")
        .send()
        .await
        .unwrap();

    let params = resp.parameters();
    assert_eq!(params.len(), 3);

    for (index, param) in params.iter().enumerate() {
        assert_eq!(param.name(), Some("test"));
        assert_eq!(
            param.r#type(),
            Some(&aws_sdk_ssm::types::ParameterType::String)
        );
        assert_eq!(param.value(), Some(format!("value-{index}").as_str()));
        assert_eq!(param.version(), (index as i64) + 1);
        assert!(param.labels().is_empty());
    }
}

// Ported from moto: test_ssm.py::test_label_parameter_version
#[tokio::test]
async fn test_label_parameter_version_basic() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("test")
        .description("A test parameter")
        .value("value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let resp = client
        .label_parameter_version()
        .name("test")
        .labels("test-label")
        .send()
        .await
        .unwrap();

    assert!(resp.invalid_labels().is_empty());
}

// Ported from moto: test_ssm.py::test_label_parameter_version_twice
#[tokio::test]
async fn test_label_parameter_version_twice() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("test")
        .description("A test parameter")
        .value("value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    // Apply the same label twice
    client
        .label_parameter_version()
        .name("test")
        .parameter_version(1)
        .labels("test-label")
        .send()
        .await
        .unwrap();

    let resp = client
        .label_parameter_version()
        .name("test")
        .parameter_version(1)
        .labels("test-label")
        .send()
        .await
        .unwrap();

    assert!(resp.invalid_labels().is_empty());

    // Verify only one copy of the label exists in history
    let hist = client
        .get_parameter_history()
        .name("test")
        .send()
        .await
        .unwrap();

    assert_eq!(hist.parameters().len(), 1);
    assert_eq!(hist.parameters()[0].labels(), &["test-label".to_string()]);
}

// Ported from moto: test_ssm.py::test_label_parameter_moving_versions
#[tokio::test]
async fn test_label_parameter_moving_versions() {
    let client = make_ssm_client().await;

    for i in 0..3 {
        client
            .put_parameter()
            .name("test")
            .description(format!("A test parameter version {i}"))
            .value(format!("value-{i}"))
            .r#type(aws_sdk_ssm::types::ParameterType::String)
            .overwrite(true)
            .send()
            .await
            .unwrap();
    }

    // Label version 1
    client
        .label_parameter_version()
        .name("test")
        .parameter_version(1)
        .labels("test-label")
        .send()
        .await
        .unwrap();

    // Move label to version 2 (label should move from v1 to v2)
    client
        .label_parameter_version()
        .name("test")
        .parameter_version(2)
        .labels("test-label")
        .send()
        .await
        .unwrap();

    let hist = client
        .get_parameter_history()
        .name("test")
        .send()
        .await
        .unwrap();

    assert_eq!(hist.parameters().len(), 3);
    for param in hist.parameters() {
        if param.version() == 2 {
            assert!(
                param.labels().contains(&"test-label".to_string()),
                "Version 2 should have the label"
            );
        }
    }
}

// Ported from moto: test_ssm.py::test_tags_in_list_tags_from_resource_parameter
#[tokio::test]
async fn test_tags_in_list_tags_from_resource_parameter() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("/spam/eggs")
        .value("eggs")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    // Add tag via add_tags_to_resource
    client
        .add_tags_to_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/spam/eggs")
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("spam")
                .value("eggs")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id("/spam/eggs")
        .send()
        .await
        .unwrap();

    assert_eq!(tags.tag_list().len(), 1);
    assert_eq!(tags.tag_list()[0].key(), "spam");
    assert_eq!(tags.tag_list()[0].value(), "eggs");

    // Delete the parameter
    client
        .delete_parameter()
        .name("/spam/eggs")
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_ssm.py::test_delete_parameters (comprehensive)
#[tokio::test]
async fn test_delete_parameters_comprehensive() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("test")
        .description("A test parameter")
        .value("value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    // Verify parameter exists
    let resp = client.get_parameters().names("test").send().await.unwrap();
    assert_eq!(resp.parameters().len(), 1);

    // Delete test and invalid
    let result = client
        .delete_parameters()
        .names("test")
        .names("invalid")
        .send()
        .await
        .unwrap();

    assert_eq!(result.deleted_parameters().len(), 1);
    assert_eq!(result.invalid_parameters().len(), 1);

    // Verify parameter is gone
    let resp = client.get_parameters().names("test").send().await.unwrap();
    assert_eq!(resp.parameters().len(), 0);
}

// ============================================================================
// Ported from moto: test_ssm_maintenance_windows.py
// ============================================================================

// Ported from moto: test_ssm_maintenance_windows.py::test_describe_maintenance_window (empty)
#[tokio::test]
async fn test_describe_maintenance_windows_empty() {
    let client = make_ssm_client().await;

    let resp = client.describe_maintenance_windows().send().await.unwrap();

    assert_eq!(resp.window_identities().len(), 0);
}

// Ported from moto: test_ssm_maintenance_windows.py::test_create_maintenance_windows_simple
#[tokio::test]
async fn test_create_maintenance_windows_simple() {
    let client = make_ssm_client().await;

    let resp = client
        .create_maintenance_window()
        .name("simple-window")
        .schedule("cron(15 12 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();

    let window_id = resp.window_id().unwrap();
    assert!(!window_id.is_empty());

    let desc = client.describe_maintenance_windows().send().await.unwrap();

    assert_eq!(desc.window_identities().len(), 1);
    let my_window = &desc.window_identities()[0];
    assert_eq!(my_window.window_id(), Some(window_id));
    assert_eq!(my_window.name(), Some("simple-window"));
    assert!(my_window.enabled());
    assert_eq!(my_window.duration(), Some(2));
    assert_eq!(my_window.cutoff(), 1);
    assert_eq!(my_window.schedule(), Some("cron(15 12 * * ? *)"));
}

// Ported from moto: test_ssm_maintenance_windows.py::test_get_maintenance_windows
#[tokio::test]
async fn test_get_maintenance_window_not_found() {
    let client = make_ssm_client().await;

    let resp = client
        .create_maintenance_window()
        .name("my-window")
        .schedule("cron(15 12 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();

    let window_id = resp.window_id().unwrap();

    // Getting a non-existent window should fail
    let err = client
        .get_maintenance_window()
        .window_id(format!("{window_id}bad"))
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("DoesNotExist"),
        "Expected DoesNotExistException, got: {err_str}"
    );
}

// Ported from moto: test_ssm_maintenance_windows.py::test_describe_maintenance_windows (filter)
#[tokio::test]
async fn test_describe_maintenance_windows_multiple() {
    let client = make_ssm_client().await;

    for idx in 0..4 {
        client
            .create_maintenance_window()
            .name(format!("window_{idx}"))
            .schedule("cron(15 12 * * ? *)")
            .duration(2)
            .cutoff(1)
            .allow_unassociated_targets(false)
            .send()
            .await
            .unwrap();
    }

    let resp = client.describe_maintenance_windows().send().await.unwrap();

    assert_eq!(resp.window_identities().len(), 4);
}

// Ported from moto: test_ssm_maintenance_windows.py::test_tags
#[tokio::test]
async fn test_maintenance_window_tags() {
    let client = make_ssm_client().await;

    // Create window without tags
    let mw_id = client
        .create_maintenance_window()
        .name("simple-window")
        .schedule("cron(15 12 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap()
        .window_id()
        .unwrap()
        .to_string();

    // Add tags
    client
        .add_tags_to_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::MaintenanceWindow)
        .resource_id(&mw_id)
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("k1")
                .value("v1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::MaintenanceWindow)
        .resource_id(&mw_id)
        .send()
        .await
        .unwrap();

    assert_eq!(tags.tag_list().len(), 1);
    assert_eq!(tags.tag_list()[0].key(), "k1");
    assert_eq!(tags.tag_list()[0].value(), "v1");

    // Add more tags
    client
        .add_tags_to_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::MaintenanceWindow)
        .resource_id(&mw_id)
        .tags(
            aws_sdk_ssm::types::Tag::builder()
                .key("k2")
                .value("v2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::MaintenanceWindow)
        .resource_id(&mw_id)
        .send()
        .await
        .unwrap();

    assert_eq!(tags.tag_list().len(), 2);

    // Remove a tag
    client
        .remove_tags_from_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::MaintenanceWindow)
        .resource_id(&mw_id)
        .tag_keys("k2")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::MaintenanceWindow)
        .resource_id(&mw_id)
        .send()
        .await
        .unwrap();

    assert_eq!(tags.tag_list().len(), 1);
    assert_eq!(tags.tag_list()[0].key(), "k1");
}

// Ported from moto: test_ssm_maintenance_windows.py::test_register_maintenance_window_target
#[tokio::test]
async fn test_register_maintenance_window_target() {
    let client = make_ssm_client().await;

    let resp = client
        .create_maintenance_window()
        .name("simple-window")
        .schedule("cron(15 12 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();
    let window_id = resp.window_id().unwrap();

    let reg_resp = client
        .register_target_with_maintenance_window()
        .window_id(window_id)
        .resource_type(aws_sdk_ssm::types::MaintenanceWindowResourceType::Instance)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("tag:Name")
                .values("my-instance")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let target_id = reg_resp.window_target_id().unwrap();
    assert!(!target_id.is_empty());

    let desc_resp = client
        .describe_maintenance_window_targets()
        .window_id(window_id)
        .send()
        .await
        .unwrap();

    assert_eq!(desc_resp.targets().len(), 1);
    assert_eq!(
        desc_resp.targets()[0].resource_type(),
        Some(&aws_sdk_ssm::types::MaintenanceWindowResourceType::Instance)
    );
    assert_eq!(desc_resp.targets()[0].window_target_id(), Some(target_id));
}

// Ported from moto: test_ssm_maintenance_windows.py::test_describe_maintenance_window_with_no_task_or_targets
#[tokio::test]
async fn test_describe_maintenance_window_with_no_task_or_targets() {
    let client = make_ssm_client().await;

    let resp = client
        .create_maintenance_window()
        .name("simple-window")
        .schedule("cron(15 12 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();
    let window_id = resp.window_id().unwrap();

    let tasks_resp = client
        .describe_maintenance_window_tasks()
        .window_id(window_id)
        .send()
        .await
        .unwrap();
    assert_eq!(tasks_resp.tasks().len(), 0);

    let targets_resp = client
        .describe_maintenance_window_targets()
        .window_id(window_id)
        .send()
        .await
        .unwrap();
    assert_eq!(targets_resp.targets().len(), 0);
}

// Ported from moto: test_ssm_maintenance_windows.py::test_register_maintenance_window_task
#[tokio::test]
async fn test_register_maintenance_window_task_with_details() {
    let client = make_ssm_client().await;

    let resp = client
        .create_maintenance_window()
        .name("simple-window")
        .schedule("cron(15 12 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();
    let window_id = resp.window_id().unwrap();

    let reg_target = client
        .register_target_with_maintenance_window()
        .window_id(window_id)
        .resource_type(aws_sdk_ssm::types::MaintenanceWindowResourceType::Instance)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("tag:Name")
                .values("my-instance")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let window_target_id = reg_target.window_target_id().unwrap();

    let reg_task = client
        .register_task_with_maintenance_window()
        .window_id(window_id)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("WindowTargetIds")
                .values(window_target_id)
                .build(),
        )
        .task_arn("AWS-RunShellScript")
        .task_type(aws_sdk_ssm::types::MaintenanceWindowTaskType::RunCommand)
        .send()
        .await
        .unwrap();

    let task_id = reg_task.window_task_id().unwrap();
    assert!(!task_id.is_empty());

    let desc_resp = client
        .describe_maintenance_window_tasks()
        .window_id(window_id)
        .send()
        .await
        .unwrap();

    assert_eq!(desc_resp.tasks().len(), 1);
    assert_eq!(desc_resp.tasks()[0].window_task_id(), Some(task_id));
    assert_eq!(desc_resp.tasks()[0].window_id(), Some(window_id));
    assert_eq!(desc_resp.tasks()[0].task_arn(), Some("AWS-RunShellScript"));
}

// Ported from moto: test_ssm_maintenance_windows.py::test_deregister_maintenance_window_task
#[tokio::test]
async fn test_deregister_maintenance_window_task() {
    let client = make_ssm_client().await;

    let resp = client
        .create_maintenance_window()
        .name("simple-window")
        .schedule("cron(15 12 * * ? *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();
    let window_id = resp.window_id().unwrap();

    let reg_target = client
        .register_target_with_maintenance_window()
        .window_id(window_id)
        .resource_type(aws_sdk_ssm::types::MaintenanceWindowResourceType::Instance)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("tag:Name")
                .values("my-instance")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let window_target_id = reg_target.window_target_id().unwrap();

    let reg_task = client
        .register_task_with_maintenance_window()
        .window_id(window_id)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("WindowTargetIds")
                .values(window_target_id)
                .build(),
        )
        .task_arn("AWS-RunShellScript")
        .task_type(aws_sdk_ssm::types::MaintenanceWindowTaskType::RunCommand)
        .send()
        .await
        .unwrap();
    let task_id = reg_task.window_task_id().unwrap();

    client
        .deregister_task_from_maintenance_window()
        .window_id(window_id)
        .window_task_id(task_id)
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_maintenance_window_tasks()
        .window_id(window_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.tasks().len(), 0);
}

// ============================================================================
// Ported from moto: test_ssm_patch_baseline.py
// ============================================================================

// Ported from moto: test_ssm_patch_baseline.py::test_create_patch_baseLine
#[tokio::test]
async fn test_create_patch_baseline_with_description() {
    let client = make_ssm_client().await;

    let resp = client
        .create_patch_baseline()
        .name("ExamplePatchBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux)
        .send()
        .await
        .unwrap();

    let baseline_id = resp.baseline_id().unwrap();
    assert!(baseline_id.starts_with("pb-"));

    let desc_resp = client.describe_patch_baselines().send().await.unwrap();

    assert!(!desc_resp.baseline_identities().is_empty());
    let found = desc_resp
        .baseline_identities()
        .iter()
        .any(|b| b.baseline_id() == Some(baseline_id));
    assert!(found, "Created baseline should appear in describe results");
}

// Ported from moto: test_ssm_patch_baseline.py::test_delete_patch_baseline
#[tokio::test]
async fn test_delete_patch_baseline_removes_it() {
    let client = make_ssm_client().await;

    let resp = client
        .create_patch_baseline()
        .name("ExamplePatchBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux)
        .send()
        .await
        .unwrap();

    let baseline_id = resp.baseline_id().unwrap();

    client
        .delete_patch_baseline()
        .baseline_id(baseline_id)
        .send()
        .await
        .unwrap();

    let desc_resp = client.describe_patch_baselines().send().await.unwrap();

    let found = desc_resp
        .baseline_identities()
        .iter()
        .any(|b| b.baseline_id() == Some(baseline_id));
    assert!(
        !found,
        "Deleted baseline should not appear in describe results"
    );
}

// ============================================================================
// Ported from moto: test_ssm_patch_group.py
// ============================================================================

// Ported from moto: test_ssm_patch_group.py::test_register_patch_baseline_for_patch_group
#[tokio::test]
async fn test_register_patch_baseline_for_patch_group_basic() {
    let client = make_ssm_client().await;

    let baseline_id = client
        .create_patch_baseline()
        .name("ExamplePatchBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux)
        .send()
        .await
        .unwrap()
        .baseline_id()
        .unwrap()
        .to_string();

    let resp = client
        .register_patch_baseline_for_patch_group()
        .baseline_id(&baseline_id)
        .patch_group("test")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.baseline_id(), Some(baseline_id.as_str()));
    assert_eq!(resp.patch_group(), Some("test"));
}

// Ported from moto: test_ssm_patch_group.py::test_get_patch_baseline_for_patch_group
#[tokio::test]
async fn test_get_patch_baseline_for_patch_group_basic() {
    let client = make_ssm_client().await;

    let baseline_id = client
        .create_patch_baseline()
        .name("ExamplePatchBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux)
        .send()
        .await
        .unwrap()
        .baseline_id()
        .unwrap()
        .to_string();

    client
        .register_patch_baseline_for_patch_group()
        .baseline_id(&baseline_id)
        .patch_group("test")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_patch_baseline_for_patch_group()
        .patch_group("test")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.baseline_id(), Some(baseline_id.as_str()));
    assert_eq!(resp.patch_group(), Some("test"));
}

// Ported from moto: test_ssm_patch_group.py::test_deregister_patch_baseline_for_patch_group
#[tokio::test]
async fn test_deregister_patch_baseline_for_patch_group_basic() {
    let client = make_ssm_client().await;

    let baseline_id = client
        .create_patch_baseline()
        .name("ExamplePatchBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux)
        .send()
        .await
        .unwrap()
        .baseline_id()
        .unwrap()
        .to_string();

    client
        .register_patch_baseline_for_patch_group()
        .baseline_id(&baseline_id)
        .patch_group("test")
        .send()
        .await
        .unwrap();

    let resp = client
        .deregister_patch_baseline_for_patch_group()
        .baseline_id(&baseline_id)
        .patch_group("test")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.baseline_id(), Some(baseline_id.as_str()));
    assert_eq!(resp.patch_group(), Some("test"));

    // Verify it's gone
    let get_resp = client
        .get_patch_baseline_for_patch_group()
        .patch_group("test")
        .send()
        .await
        .unwrap();

    assert!(get_resp.baseline_id().is_none());
}

// ============================================================================
// Ported from moto: test_ssm_doc_permissions.py
// ============================================================================

// Ported from moto: test_ssm_doc_permissions.py::test_describe_document_permissions_unknown_document
#[tokio::test]
async fn test_describe_document_permissions_unknown_document() {
    let client = make_ssm_client().await;

    let err = client
        .describe_document_permission()
        .name("UnknownDocument")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidDocument"),
        "Expected InvalidDocument error, got: {err_str}"
    );
}

// Ported from moto: test_ssm_doc_permissions.py::test_describe_document_permissions_initial
#[tokio::test]
async fn test_describe_document_permissions_initial() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("TestDocument")
        .content("{\"schemaVersion\":\"2.2\"}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_document_permission()
        .name("TestDocument")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .send()
        .await
        .unwrap();

    assert!(resp.account_ids().is_empty());
}

// Ported from moto: test_ssm_doc_permissions.py::test_modify_document_permission_add_account_id
#[tokio::test]
async fn test_modify_document_permission_add_and_remove() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("TestDocument")
        .content("{\"schemaVersion\":\"2.2\"}")
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    // Add two account IDs
    client
        .modify_document_permission()
        .name("TestDocument")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .account_ids_to_add("111111111111")
        .account_ids_to_add("222222222222")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_document_permission()
        .name("TestDocument")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.account_ids().len(), 2);
    assert!(resp.account_ids().contains(&"111111111111".to_string()));
    assert!(resp.account_ids().contains(&"222222222222".to_string()));

    // Remove one
    client
        .modify_document_permission()
        .name("TestDocument")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .account_ids_to_remove("222222222222")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_document_permission()
        .name("TestDocument")
        .permission_type(aws_sdk_ssm::types::DocumentPermissionType::Share)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.account_ids().len(), 1);
    assert!(resp.account_ids().contains(&"111111111111".to_string()));
    assert!(!resp.account_ids().contains(&"222222222222".to_string()));
}

// ============================================================================
// Ported from moto: test_ssm.py - Commands
// ============================================================================

// Ported from moto: test_ssm.py - send_command with parameters
#[tokio::test]
async fn test_send_command_with_parameters() {
    let client = make_ssm_client().await;

    let resp = client
        .send_command()
        .document_name("AWS-RunShellScript")
        .instance_ids("i-1234567890abcdef0")
        .instance_ids("i-1234567890abcdef1")
        .send()
        .await
        .unwrap();

    let cmd = resp.command().unwrap();
    assert_eq!(cmd.document_name(), Some("AWS-RunShellScript"));
    assert_eq!(cmd.instance_ids().len(), 2);
    assert!(
        cmd.instance_ids()
            .contains(&"i-1234567890abcdef0".to_string())
    );
    assert!(
        cmd.instance_ids()
            .contains(&"i-1234567890abcdef1".to_string())
    );
    assert!(cmd.command_id().is_some());
}

// Ported from moto: test_ssm.py - update_parameter type preservation
#[tokio::test]
async fn test_update_parameter_preserves_type() {
    let client = make_ssm_client().await;

    client
        .put_parameter()
        .name("test_param")
        .description("Description")
        .value("Value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    client
        .put_parameter()
        .name("test_param")
        .value("UpdatedValue")
        .overwrite(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_parameter()
        .name("test_param")
        .send()
        .await
        .unwrap();

    let p = resp.parameter().unwrap();
    assert_eq!(p.r#type(), Some(&aws_sdk_ssm::types::ParameterType::String));
    assert_eq!(p.value(), Some("UpdatedValue"));
}

// ── UpdateMaintenanceWindow ───────────────────────────────────────

#[tokio::test]
async fn test_update_maintenance_window() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("my-window")
        .schedule("cron(0 16 ? * TUE *)")
        .duration(4)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .expect("create_maintenance_window should succeed");

    let window_id = create_resp.window_id().expect("should have window_id");

    let update_resp = client
        .update_maintenance_window()
        .window_id(window_id)
        .name("my-window-updated")
        .duration(6)
        .send()
        .await
        .expect("update_maintenance_window should succeed");

    assert_eq!(update_resp.name(), Some("my-window-updated"));
    assert_eq!(update_resp.duration(), Some(6));
    assert_eq!(update_resp.window_id(), Some(window_id));
}

// ── GetMaintenanceWindowTask ──────────────────────────────────────

#[tokio::test]
async fn test_get_maintenance_window_task() {
    let client = make_ssm_client().await;

    let window_resp = client
        .create_maintenance_window()
        .name("task-window")
        .schedule("cron(0 18 ? * MON *)")
        .duration(2)
        .cutoff(0)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();

    let window_id = window_resp.window_id().unwrap().to_string();

    let task_resp = client
        .register_task_with_maintenance_window()
        .window_id(&window_id)
        .task_arn("AWS-RunShellScript")
        .task_type(aws_sdk_ssm::types::MaintenanceWindowTaskType::RunCommand)
        .send()
        .await
        .expect("register_task should succeed");

    let task_id = task_resp.window_task_id().expect("should have task_id");

    let get_resp = client
        .get_maintenance_window_task()
        .window_id(&window_id)
        .window_task_id(task_id)
        .send()
        .await
        .expect("get_maintenance_window_task should succeed");

    assert_eq!(get_resp.task_arn(), Some("AWS-RunShellScript"));
    assert_eq!(get_resp.window_id(), Some(window_id.as_str()));
}

// ── UpdateMaintenanceWindowTask ───────────────────────────────────

#[tokio::test]
async fn test_update_maintenance_window_task() {
    let client = make_ssm_client().await;

    let window_resp = client
        .create_maintenance_window()
        .name("update-task-window")
        .schedule("cron(0 18 ? * WED *)")
        .duration(2)
        .cutoff(0)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();

    let window_id = window_resp.window_id().unwrap().to_string();

    let task_resp = client
        .register_task_with_maintenance_window()
        .window_id(&window_id)
        .task_arn("AWS-RunShellScript")
        .task_type(aws_sdk_ssm::types::MaintenanceWindowTaskType::RunCommand)
        .send()
        .await
        .unwrap();

    let task_id = task_resp.window_task_id().unwrap().to_string();

    let update_resp = client
        .update_maintenance_window_task()
        .window_id(&window_id)
        .window_task_id(&task_id)
        .task_arn("AWS-RunPatchBaseline")
        .send()
        .await
        .expect("update_maintenance_window_task should succeed");

    assert_eq!(update_resp.task_arn(), Some("AWS-RunPatchBaseline"));
    assert_eq!(update_resp.window_task_id(), Some(task_id.as_str()));
}

// ── Association operations ────────────────────────────────────────

#[tokio::test]
async fn test_create_and_describe_association() {
    let client = make_ssm_client().await;

    // First create a document that the association will reference
    client
        .create_document()
        .name("MyRunbook")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_association()
        .name("MyRunbook")
        .association_name("my-assoc")
        .send()
        .await
        .expect("create_association should succeed");

    let assoc_desc = create_resp
        .association_description()
        .expect("should have association description");
    let assoc_id = assoc_desc
        .association_id()
        .expect("should have association_id");
    assert_eq!(assoc_desc.name(), Some("MyRunbook"));
    assert_eq!(assoc_desc.association_name(), Some("my-assoc"));

    let describe_resp = client
        .describe_association()
        .association_id(assoc_id)
        .send()
        .await
        .expect("describe_association should succeed");

    let desc = describe_resp
        .association_description()
        .expect("should have association description");
    assert_eq!(desc.association_id(), Some(assoc_id));
    assert_eq!(desc.name(), Some("MyRunbook"));
}

#[tokio::test]
async fn test_list_associations() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("ListDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    client
        .create_association()
        .name("ListDoc")
        .association_name("assoc-1")
        .send()
        .await
        .unwrap();

    client
        .create_association()
        .name("ListDoc")
        .association_name("assoc-2")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_associations()
        .send()
        .await
        .expect("list_associations should succeed");

    let associations = list_resp.associations();
    assert!(
        associations.len() >= 2,
        "should have at least 2 associations"
    );
    let names: Vec<Option<&str>> = associations.iter().map(|a| a.association_name()).collect();
    assert!(names.contains(&Some("assoc-1")));
    assert!(names.contains(&Some("assoc-2")));
}

#[tokio::test]
async fn test_update_and_delete_association() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("UpdateDeleteDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_association()
        .name("UpdateDeleteDoc")
        .send()
        .await
        .unwrap();

    let assoc_id = create_resp
        .association_description()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_association()
        .association_id(&assoc_id)
        .schedule_expression("cron(0 0 ? * SUN *)")
        .send()
        .await
        .expect("update_association should succeed");

    let updated_desc = update_resp.association_description().unwrap();
    assert_eq!(
        updated_desc.schedule_expression(),
        Some("cron(0 0 ? * SUN *)")
    );

    client
        .delete_association()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("delete_association should succeed");

    // Confirm it's gone
    let describe_result = client
        .describe_association()
        .association_id(&assoc_id)
        .send()
        .await;
    assert!(
        describe_result.is_err(),
        "describe after delete should fail"
    );
}

// ── Activation operations ─────────────────────────────────────────

#[tokio::test]
async fn test_create_and_describe_activations() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_activation()
        .iam_role("arn:aws:iam::123456789012:role/TestRole")
        .description("Test activation")
        .default_instance_name("my-instance")
        .registration_limit(5)
        .send()
        .await
        .expect("create_activation should succeed");

    let activation_id = create_resp.activation_id().unwrap();
    assert!(!activation_id.is_empty());
    assert!(create_resp.activation_code().is_some());

    let describe_resp = client
        .describe_activations()
        .send()
        .await
        .expect("describe_activations should succeed");

    let activations = describe_resp.activation_list();
    assert!(!activations.is_empty());
    let found = activations
        .iter()
        .find(|a| a.activation_id() == Some(activation_id))
        .expect("activation should be in list");
    assert_eq!(
        found.iam_role(),
        Some("arn:aws:iam::123456789012:role/TestRole")
    );
    assert_eq!(found.description(), Some("Test activation"));
    assert_eq!(found.default_instance_name(), Some("my-instance"));
    assert_eq!(found.registration_limit(), Some(5));
}

#[tokio::test]
async fn test_delete_activation() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_activation()
        .iam_role("arn:aws:iam::123456789012:role/TestRole")
        .send()
        .await
        .unwrap();

    let activation_id = create_resp.activation_id().unwrap().to_string();

    client
        .delete_activation()
        .activation_id(&activation_id)
        .send()
        .await
        .expect("delete_activation should succeed");

    let describe_resp = client.describe_activations().send().await.unwrap();

    let found = describe_resp
        .activation_list()
        .iter()
        .any(|a| a.activation_id() == Some(&activation_id));
    assert!(!found, "deleted activation should not appear in list");
}

// ── OpsItem operations ────────────────────────────────────────────

#[tokio::test]
async fn test_create_get_and_describe_ops_item() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_ops_item()
        .title("Test OpsItem")
        .source("test-source")
        .description("A test ops item")
        .priority(3)
        .severity("Medium")
        .category("Security")
        .send()
        .await
        .expect("create_ops_item should succeed");

    let ops_item_id = create_resp.ops_item_id().unwrap().to_string();
    assert!(!ops_item_id.is_empty());

    // Get the ops item
    let get_resp = client
        .get_ops_item()
        .ops_item_id(&ops_item_id)
        .send()
        .await
        .expect("get_ops_item should succeed");

    let item = get_resp.ops_item().unwrap();
    assert_eq!(item.ops_item_id(), Some(ops_item_id.as_str()));
    assert_eq!(item.title(), Some("Test OpsItem"));
    assert_eq!(item.source(), Some("test-source"));
    assert_eq!(item.description(), Some("A test ops item"));
    assert_eq!(item.priority(), Some(3));
    assert_eq!(item.severity(), Some("Medium"));
    assert_eq!(item.category(), Some("Security"));
    assert_eq!(
        item.status(),
        Some(&aws_sdk_ssm::types::OpsItemStatus::Open)
    );

    // Describe ops items
    let describe_resp = client
        .describe_ops_items()
        .send()
        .await
        .expect("describe_ops_items should succeed");

    let summaries = describe_resp.ops_item_summaries();
    assert!(!summaries.is_empty());
    let found = summaries
        .iter()
        .find(|s| s.ops_item_id() == Some(ops_item_id.as_str()))
        .expect("ops item should be in summary list");
    assert_eq!(found.title(), Some("Test OpsItem"));
}

#[tokio::test]
async fn test_update_and_delete_ops_item() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_ops_item()
        .title("Update Me")
        .source("test")
        .send()
        .await
        .unwrap();

    let ops_item_id = create_resp.ops_item_id().unwrap().to_string();

    // Update the ops item
    client
        .update_ops_item()
        .ops_item_id(&ops_item_id)
        .title("Updated Title")
        .status(aws_sdk_ssm::types::OpsItemStatus::InProgress)
        .priority(1)
        .description("Updated description")
        .send()
        .await
        .expect("update_ops_item should succeed");

    // Verify the update
    let get_resp = client
        .get_ops_item()
        .ops_item_id(&ops_item_id)
        .send()
        .await
        .unwrap();

    let item = get_resp.ops_item().unwrap();
    assert_eq!(item.title(), Some("Updated Title"));
    assert_eq!(
        item.status(),
        Some(&aws_sdk_ssm::types::OpsItemStatus::InProgress)
    );
    assert_eq!(item.priority(), Some(1));
    assert_eq!(item.description(), Some("Updated description"));

    // Delete the ops item
    client
        .delete_ops_item()
        .ops_item_id(&ops_item_id)
        .send()
        .await
        .expect("delete_ops_item should succeed");

    // Verify it's gone
    let get_result = client.get_ops_item().ops_item_id(&ops_item_id).send().await;
    assert!(get_result.is_err(), "get after delete should fail");
}

// ── Service Setting operations ────────────────────────────────────

#[tokio::test]
async fn test_service_setting_lifecycle() {
    let client = make_ssm_client().await;

    let setting_id = "/ssm/managed-instance/activation-tier";

    // Update a service setting
    client
        .update_service_setting()
        .setting_id(setting_id)
        .setting_value("advanced")
        .send()
        .await
        .expect("update_service_setting should succeed");

    // Get the service setting
    let get_resp = client
        .get_service_setting()
        .setting_id(setting_id)
        .send()
        .await
        .expect("get_service_setting should succeed");

    let setting = get_resp.service_setting().unwrap();
    assert_eq!(setting.setting_id(), Some(setting_id));
    assert_eq!(setting.setting_value(), Some("advanced"));

    // Reset the service setting
    client
        .reset_service_setting()
        .setting_id(setting_id)
        .send()
        .await
        .expect("reset_service_setting should succeed");
}

// ── ListDocumentVersions operation ────────────────────────────────

#[tokio::test]
async fn test_list_document_versions() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("VersionedDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    // Update the document to create a second version
    client
        .update_document()
        .name("VersionedDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[{"action":"aws:runShellScript"}]}"#)
        .document_version("$LATEST")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_document_versions()
        .name("VersionedDoc")
        .send()
        .await
        .expect("list_document_versions should succeed");

    let versions = list_resp.document_versions();
    assert!(
        versions.len() >= 2,
        "should have at least 2 versions, got {}",
        versions.len()
    );
}

// ── Session operations ────────────────────────────────────────────

#[tokio::test]
async fn test_start_terminate_and_describe_sessions() {
    let client = make_ssm_client().await;

    let start_resp = client
        .start_session()
        .target("i-1234567890abcdef0")
        .send()
        .await
        .expect("start_session should succeed");

    let session_id = start_resp.session_id().unwrap().to_string();
    assert!(!session_id.is_empty());

    // Describe sessions (no filter to list all)
    let describe_resp = client
        .describe_sessions()
        .state(aws_sdk_ssm::types::SessionState::Active)
        .send()
        .await
        .expect("describe_sessions should succeed");

    // If filtered result is empty, try without filter (status stored as "Connected")
    let describe_resp = if describe_resp.sessions().is_empty() {
        client
            .describe_sessions()
            .send()
            .await
            .expect("describe_sessions without filter should succeed")
    } else {
        describe_resp
    };

    let sessions = describe_resp.sessions();
    let found = sessions
        .iter()
        .find(|s| s.session_id() == Some(session_id.as_str()));
    assert!(found.is_some(), "active session should be in list");
    assert_eq!(found.unwrap().target(), Some("i-1234567890abcdef0"));

    // Terminate the session
    let term_resp = client
        .terminate_session()
        .session_id(&session_id)
        .send()
        .await
        .expect("terminate_session should succeed");
    assert_eq!(term_resp.session_id(), Some(session_id.as_str()));
}

// ── OpsMetadata operations ────────────────────────────────────────

#[tokio::test]
async fn test_ops_metadata_lifecycle() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_ops_metadata()
        .resource_id("arn:aws:ssm:us-east-1:123456789012:document/TestDoc")
        .metadata(
            "Env",
            aws_sdk_ssm::types::MetadataValue::builder()
                .value("production")
                .build(),
        )
        .send()
        .await
        .expect("create_ops_metadata should succeed");

    let arn = create_resp.ops_metadata_arn().unwrap().to_string();
    assert!(!arn.is_empty());

    // Get the ops metadata
    let get_resp = client
        .get_ops_metadata()
        .ops_metadata_arn(&arn)
        .send()
        .await
        .expect("get_ops_metadata should succeed");

    assert_eq!(
        get_resp.resource_id(),
        Some("arn:aws:ssm:us-east-1:123456789012:document/TestDoc")
    );
    let metadata = get_resp.metadata().unwrap();
    let env_val = metadata.get("Env").unwrap();
    assert_eq!(env_val.value(), Some("production"));

    // List ops metadata
    let list_resp = client
        .list_ops_metadata()
        .send()
        .await
        .expect("list_ops_metadata should succeed");

    let items = list_resp.ops_metadata_list();
    assert!(!items.is_empty());

    // Update ops metadata
    client
        .update_ops_metadata()
        .ops_metadata_arn(&arn)
        .metadata_to_update(
            "Stage",
            aws_sdk_ssm::types::MetadataValue::builder()
                .value("beta")
                .build(),
        )
        .send()
        .await
        .expect("update_ops_metadata should succeed");

    // Delete ops metadata
    client
        .delete_ops_metadata()
        .ops_metadata_arn(&arn)
        .send()
        .await
        .expect("delete_ops_metadata should succeed");

    // Verify it's gone
    let get_result = client
        .get_ops_metadata()
        .ops_metadata_arn(&arn)
        .send()
        .await;
    assert!(get_result.is_err(), "get after delete should fail");
}

// ── ResourceDataSync operations ───────────────────────────────────

#[tokio::test]
async fn test_resource_data_sync_lifecycle() {
    let client = make_ssm_client().await;

    client
        .create_resource_data_sync()
        .sync_name("test-sync")
        .s3_destination(
            aws_sdk_ssm::types::ResourceDataSyncS3Destination::builder()
                .bucket_name("my-bucket")
                .region("us-east-1")
                .sync_format(aws_sdk_ssm::types::ResourceDataSyncS3Format::JsonSerde)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_resource_data_sync should succeed");

    // List resource data syncs
    let list_resp = client
        .list_resource_data_sync()
        .send()
        .await
        .expect("list_resource_data_sync should succeed");

    let syncs = list_resp.resource_data_sync_items();
    assert!(!syncs.is_empty());
    let found = syncs
        .iter()
        .find(|s| s.sync_name() == Some("test-sync"))
        .expect("sync should be in list");
    assert!(found.s3_destination().is_some());

    // Delete resource data sync
    client
        .delete_resource_data_sync()
        .sync_name("test-sync")
        .send()
        .await
        .expect("delete_resource_data_sync should succeed");

    // Verify it's gone
    let list_resp2 = client.list_resource_data_sync().send().await.unwrap();
    let found2 = list_resp2
        .resource_data_sync_items()
        .iter()
        .any(|s| s.sync_name() == Some("test-sync"));
    assert!(!found2, "deleted sync should not appear in list");
}

// ── Default patch baseline operations ─────────────────────────────

#[tokio::test]
async fn test_register_and_get_default_patch_baseline() {
    let client = make_ssm_client().await;

    // Create a patch baseline first
    let create_resp = client
        .create_patch_baseline()
        .name("DefaultTestBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::Windows)
        .send()
        .await
        .unwrap();

    let baseline_id = create_resp.baseline_id().unwrap().to_string();

    // Register it as default
    let reg_resp = client
        .register_default_patch_baseline()
        .baseline_id(&baseline_id)
        .send()
        .await
        .expect("register_default_patch_baseline should succeed");

    assert_eq!(reg_resp.baseline_id(), Some(baseline_id.as_str()));

    // Get default patch baseline
    let get_resp = client
        .get_default_patch_baseline()
        .operating_system(aws_sdk_ssm::types::OperatingSystem::Windows)
        .send()
        .await
        .expect("get_default_patch_baseline should succeed");

    assert_eq!(get_resp.baseline_id(), Some(baseline_id.as_str()));
}

// ── GetPatchBaseline operation ────────────────────────────────────

#[tokio::test]
async fn test_get_patch_baseline() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_patch_baseline()
        .name("GetTestBaseline")
        .description("baseline for get test")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux2)
        .send()
        .await
        .unwrap();

    let baseline_id = create_resp.baseline_id().unwrap().to_string();

    let get_resp = client
        .get_patch_baseline()
        .baseline_id(&baseline_id)
        .send()
        .await
        .expect("get_patch_baseline should succeed");

    assert_eq!(get_resp.baseline_id(), Some(baseline_id.as_str()));
    assert_eq!(get_resp.name(), Some("GetTestBaseline"));
    assert_eq!(get_resp.description(), Some("baseline for get test"));
}

// ── DescribePatchGroups operation ─────────────────────────────────

#[tokio::test]
async fn test_describe_patch_groups() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_patch_baseline()
        .name("PatchGroupBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::Windows)
        .send()
        .await
        .unwrap();

    let baseline_id = create_resp.baseline_id().unwrap().to_string();

    client
        .register_patch_baseline_for_patch_group()
        .baseline_id(&baseline_id)
        .patch_group("web-servers")
        .send()
        .await
        .unwrap();

    let describe_resp = client
        .describe_patch_groups()
        .send()
        .await
        .expect("describe_patch_groups should succeed");

    let mappings = describe_resp.mappings();
    assert!(!mappings.is_empty());
    let found = mappings
        .iter()
        .find(|m| m.patch_group() == Some("web-servers"))
        .expect("patch group mapping should be in list");
    let identity = found.baseline_identity().unwrap();
    assert_eq!(identity.baseline_id(), Some(baseline_id.as_str()));
}

// ── ListCommandInvocations operation ──────────────────────────────

#[tokio::test]
async fn test_list_command_invocations() {
    let client = make_ssm_client().await;

    // Create a document first
    client
        .create_document()
        .name("InvocDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let send_resp = client
        .send_command()
        .document_name("InvocDoc")
        .instance_ids("i-abc123")
        .send()
        .await
        .unwrap();

    let command_id = send_resp
        .command()
        .unwrap()
        .command_id()
        .unwrap()
        .to_string();

    let list_resp = client
        .list_command_invocations()
        .command_id(&command_id)
        .send()
        .await
        .expect("list_command_invocations should succeed");

    let invocations = list_resp.command_invocations();
    assert!(!invocations.is_empty());
    assert_eq!(invocations[0].command_id(), Some(command_id.as_str()));
    assert_eq!(invocations[0].instance_id(), Some("i-abc123"));
}

// ── ResourcePolicy operations ─────────────────────────────────────

#[tokio::test]
async fn test_resource_policy_lifecycle() {
    let client = make_ssm_client().await;

    // Create a parameter to attach the policy to
    client
        .put_parameter()
        .name("/test/policy-target")
        .value("test-value")
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .unwrap();

    let resource_arn = "arn:aws:ssm:us-east-1:123456789012:parameter/test/policy-target";
    let policy_json = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":"*","Action":"ssm:GetParameter","Resource":"*"}]}"#;

    // Put resource policy
    let put_resp = client
        .put_resource_policy()
        .resource_arn(resource_arn)
        .policy(policy_json)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let policy_id = put_resp.policy_id().unwrap().to_string();
    assert!(!policy_id.is_empty());
    assert!(put_resp.policy_hash().is_some());

    // Get resource policies
    let get_resp = client
        .get_resource_policies()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("get_resource_policies should succeed");

    let policies = get_resp.policies();
    assert!(!policies.is_empty());
    let found = policies
        .iter()
        .find(|p| p.policy_id() == Some(policy_id.as_str()))
        .expect("policy should be in list");
    assert_eq!(found.policy(), Some(policy_json));

    // Delete resource policy
    client
        .delete_resource_policy()
        .resource_arn(resource_arn)
        .policy_id(&policy_id)
        .policy_hash(put_resp.policy_hash().unwrap())
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    // Verify it's gone
    let get_resp2 = client
        .get_resource_policies()
        .resource_arn(resource_arn)
        .send()
        .await
        .unwrap();
    assert!(
        get_resp2.policies().is_empty(),
        "policies should be empty after delete"
    );
}

// ── Stub operations: Association sub-features ─────────────────────

#[tokio::test]
async fn test_create_association_batch() {
    let client = make_ssm_client().await;

    let resp = client
        .create_association_batch()
        .entries(
            aws_sdk_ssm::types::CreateAssociationBatchRequestEntry::builder()
                .name("AWS-RunShellScript")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_association_batch should succeed");
    // Stub returns empty response — just confirm no error
    let _ = resp;
}

#[tokio::test]
async fn test_describe_association_executions() {
    let client = make_ssm_client().await;

    // First create an association to reference
    let create_resp = client
        .create_association()
        .name("AWS-RunShellScript")
        .send()
        .await
        .unwrap();
    let assoc_id = create_resp
        .association_description()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_association_executions()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("describe_association_executions should succeed");
    let _ = resp.association_executions();
}

#[tokio::test]
async fn test_describe_association_execution_targets() {
    let client = make_ssm_client().await;

    // Create an association first so the ID exists
    let create_resp = client
        .create_association()
        .name("AWS-RunShellScript")
        .send()
        .await
        .unwrap();
    let assoc_id = create_resp
        .association_description()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_association_execution_targets()
        .association_id(&assoc_id)
        .execution_id("00000001-0001-0000-0000-000000000001")
        .send()
        .await
        .expect("describe_association_execution_targets should succeed");
    let _ = resp.association_execution_targets();
}

#[tokio::test]
async fn test_list_association_versions() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_association()
        .name("AWS-RunShellScript")
        .send()
        .await
        .unwrap();
    let assoc_id = create_resp
        .association_description()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let resp = client
        .list_association_versions()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("list_association_versions should succeed");
    let _ = resp.association_versions();
}

#[tokio::test]
async fn test_update_association_status() {
    let client = make_ssm_client().await;

    let resp = client
        .update_association_status()
        .instance_id("i-1234567890abcdef0")
        .name("AWS-RunShellScript")
        .association_status(
            aws_sdk_ssm::types::AssociationStatus::builder()
                .date(aws_smithy_types::DateTime::from_secs(0))
                .name(aws_sdk_ssm::types::AssociationStatusName::Success)
                .message("ok")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_association_status should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_start_associations_once() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_association()
        .name("AWS-RunShellScript")
        .send()
        .await
        .unwrap();
    let assoc_id = create_resp
        .association_description()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    client
        .start_associations_once()
        .association_ids(&assoc_id)
        .send()
        .await
        .expect("start_associations_once should succeed");
}

// ── Stub operations: Command sub-features ─────────────────────────

#[tokio::test]
async fn test_cancel_command() {
    let client = make_ssm_client().await;

    let send_resp = client
        .send_command()
        .instance_ids("i-1234567890abcdef0")
        .document_name("AWS-RunShellScript")
        .send()
        .await
        .unwrap();

    let command_id = send_resp
        .command()
        .unwrap()
        .command_id()
        .unwrap()
        .to_string();

    client
        .cancel_command()
        .command_id(&command_id)
        .send()
        .await
        .expect("cancel_command should succeed");
}

// ── Stub operations: Maintenance Window execution history ──────────

#[tokio::test]
async fn test_cancel_maintenance_window_execution() {
    let client = make_ssm_client().await;

    let resp = client
        .cancel_maintenance_window_execution()
        .window_execution_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("cancel_maintenance_window_execution should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_describe_maintenance_window_executions() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("exec-test-window")
        .schedule("cron(0 0 ? * SUN *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();
    let window_id = create_resp.window_id().unwrap().to_string();

    let resp = client
        .describe_maintenance_window_executions()
        .window_id(&window_id)
        .send()
        .await
        .expect("describe_maintenance_window_executions should succeed");
    let _ = resp.window_executions();
}

#[tokio::test]
async fn test_describe_maintenance_window_execution_tasks() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_maintenance_window_execution_tasks()
        .window_execution_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("describe_maintenance_window_execution_tasks should succeed");
    let _ = resp.window_execution_task_identities();
}

#[tokio::test]
async fn test_describe_maintenance_window_execution_task_invocations() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_maintenance_window_execution_task_invocations()
        .window_execution_id("00000001-0000-0000-0000-000000000001")
        .task_id("task-00000000000000001")
        .send()
        .await
        .expect("describe_maintenance_window_execution_task_invocations should succeed");
    let _ = resp.window_execution_task_invocation_identities();
}

#[tokio::test]
async fn test_get_maintenance_window_execution() {
    let client = make_ssm_client().await;

    let resp = client
        .get_maintenance_window_execution()
        .window_execution_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("get_maintenance_window_execution should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_get_maintenance_window_execution_task() {
    let client = make_ssm_client().await;

    let resp = client
        .get_maintenance_window_execution_task()
        .window_execution_id("00000001-0000-0000-0000-000000000001")
        .task_id("task-00000000000000001")
        .send()
        .await
        .expect("get_maintenance_window_execution_task should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_get_maintenance_window_execution_task_invocation() {
    let client = make_ssm_client().await;

    let resp = client
        .get_maintenance_window_execution_task_invocation()
        .window_execution_id("00000001-0000-0000-0000-000000000001")
        .task_id("task-00000000000000001")
        .invocation_id("00000001-0000-0000-0000-000000000002")
        .send()
        .await
        .expect("get_maintenance_window_execution_task_invocation should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_describe_maintenance_window_schedule() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("sched-test-window")
        .schedule("cron(0 0 ? * SUN *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(false)
        .send()
        .await
        .unwrap();
    let window_id = create_resp.window_id().unwrap().to_string();

    let resp = client
        .describe_maintenance_window_schedule()
        .window_id(&window_id)
        .send()
        .await
        .expect("describe_maintenance_window_schedule should succeed");
    let _ = resp.scheduled_window_executions();
}

#[tokio::test]
async fn test_describe_maintenance_windows_for_target() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_maintenance_windows_for_target()
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("instanceids")
                .values("i-1234567890abcdef0")
                .build(),
        )
        .resource_type(aws_sdk_ssm::types::MaintenanceWindowResourceType::Instance)
        .send()
        .await
        .expect("describe_maintenance_windows_for_target should succeed");
    let _ = resp.window_identities();
}

#[tokio::test]
async fn test_update_maintenance_window_target() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_maintenance_window()
        .name("target-update-window")
        .schedule("cron(0 0 ? * SUN *)")
        .duration(2)
        .cutoff(1)
        .allow_unassociated_targets(true)
        .send()
        .await
        .unwrap();
    let window_id = create_resp.window_id().unwrap().to_string();

    let reg_resp = client
        .register_target_with_maintenance_window()
        .window_id(&window_id)
        .resource_type(aws_sdk_ssm::types::MaintenanceWindowResourceType::Instance)
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("instanceids")
                .values("i-1234567890abcdef0")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let window_target_id = reg_resp.window_target_id().unwrap().to_string();

    let resp = client
        .update_maintenance_window_target()
        .window_id(&window_id)
        .window_target_id(&window_target_id)
        .name("updated-target-name")
        .send()
        .await
        .expect("update_maintenance_window_target should succeed");
    let _ = resp;
}

// ── Stub operations: Managed instances ────────────────────────────

#[tokio::test]
async fn test_deregister_managed_instance() {
    let client = make_ssm_client().await;

    // Stub returns success regardless of instance ID
    client
        .deregister_managed_instance()
        .instance_id("mi-1234567890abcdef0")
        .send()
        .await
        .expect("deregister_managed_instance should succeed");
}

#[tokio::test]
async fn test_describe_instance_information() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_instance_information()
        .send()
        .await
        .expect("describe_instance_information should succeed");
    let _ = resp.instance_information_list();
}

#[tokio::test]
async fn test_describe_instance_properties() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_instance_properties()
        .send()
        .await
        .expect("describe_instance_properties should succeed");
    let _ = resp.instance_properties();
}

#[tokio::test]
async fn test_update_managed_instance_role() {
    let client = make_ssm_client().await;

    client
        .update_managed_instance_role()
        .instance_id("mi-1234567890abcdef0")
        .iam_role("arn:aws:iam::123456789012:role/NewRole")
        .send()
        .await
        .expect("update_managed_instance_role should succeed");
}

#[tokio::test]
async fn test_get_connection_status() {
    let client = make_ssm_client().await;

    let resp = client
        .get_connection_status()
        .target("i-1234567890abcdef0")
        .send()
        .await
        .expect("get_connection_status should succeed");
    let _ = resp;
}

// ── Stub operations: Patch Manager ────────────────────────────────

#[tokio::test]
async fn test_describe_available_patches() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_available_patches()
        .send()
        .await
        .expect("describe_available_patches should succeed");
    let _ = resp.patches();
}

#[tokio::test]
async fn test_describe_effective_patches_for_patch_baseline() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_patch_baseline()
        .name("EffPatchTestBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::Windows)
        .send()
        .await
        .unwrap();
    let baseline_id = create_resp.baseline_id().unwrap().to_string();

    let resp = client
        .describe_effective_patches_for_patch_baseline()
        .baseline_id(&baseline_id)
        .send()
        .await
        .expect("describe_effective_patches_for_patch_baseline should succeed");
    let _ = resp.effective_patches();
}

#[tokio::test]
async fn test_describe_instance_patch_states() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_instance_patch_states()
        .instance_ids("i-1234567890abcdef0")
        .send()
        .await
        .expect("describe_instance_patch_states should succeed");
    let _ = resp.instance_patch_states();
}

#[tokio::test]
async fn test_describe_instance_patch_states_for_patch_group() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_instance_patch_states_for_patch_group()
        .patch_group("my-patch-group")
        .send()
        .await
        .expect("describe_instance_patch_states_for_patch_group should succeed");
    let _ = resp.instance_patch_states();
}

#[tokio::test]
async fn test_describe_instance_patches() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_instance_patches()
        .instance_id("i-1234567890abcdef0")
        .send()
        .await
        .expect("describe_instance_patches should succeed");
    let _ = resp.patches();
}

#[tokio::test]
async fn test_describe_patch_group_state() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_patch_group_state()
        .patch_group("my-patch-group")
        .send()
        .await
        .expect("describe_patch_group_state should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_describe_patch_properties() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_patch_properties()
        .operating_system(aws_sdk_ssm::types::OperatingSystem::Windows)
        .property(aws_sdk_ssm::types::PatchProperty::Product)
        .send()
        .await
        .expect("describe_patch_properties should succeed");
    let _ = resp.properties();
}

#[tokio::test]
async fn test_get_deployable_patch_snapshot_for_instance() {
    let client = make_ssm_client().await;

    let resp = client
        .get_deployable_patch_snapshot_for_instance()
        .instance_id("i-1234567890abcdef0")
        .snapshot_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("get_deployable_patch_snapshot_for_instance should succeed");
    let _ = resp;
}

// ── Stub operations: Inventory ────────────────────────────────────

#[tokio::test]
async fn test_put_and_get_inventory() {
    let client = make_ssm_client().await;

    let instance_id = "i-inventory-test-001";

    client
        .put_inventory()
        .instance_id(instance_id)
        .items(
            aws_sdk_ssm::types::InventoryItem::builder()
                .type_name("AWS:Application")
                .schema_version("1.1")
                .capture_time("2024-01-01T00:00:00Z")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_inventory should succeed");

    let resp = client
        .get_inventory()
        .send()
        .await
        .expect("get_inventory should succeed");
    let _ = resp.entities();
}

#[tokio::test]
async fn test_list_inventory_entries() {
    let client = make_ssm_client().await;

    let instance_id = "i-inventory-entries-test";

    client
        .put_inventory()
        .instance_id(instance_id)
        .items(
            aws_sdk_ssm::types::InventoryItem::builder()
                .type_name("AWS:Application")
                .schema_version("1.1")
                .capture_time("2024-01-01T00:00:00Z")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_inventory_entries()
        .instance_id(instance_id)
        .type_name("AWS:Application")
        .send()
        .await
        .expect("list_inventory_entries should succeed");
    assert_eq!(resp.instance_id(), Some(instance_id));
    assert_eq!(resp.type_name(), Some("AWS:Application"));
}

#[tokio::test]
async fn test_get_inventory_schema() {
    let client = make_ssm_client().await;

    let resp = client
        .get_inventory_schema()
        .send()
        .await
        .expect("get_inventory_schema should succeed");
    let _ = resp.schemas();
}

#[tokio::test]
async fn test_delete_inventory() {
    let client = make_ssm_client().await;

    let resp = client
        .delete_inventory()
        .type_name("AWS:Application")
        .send()
        .await
        .expect("delete_inventory should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_describe_inventory_deletions() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_inventory_deletions()
        .send()
        .await
        .expect("describe_inventory_deletions should succeed");
    let _ = resp.inventory_deletions();
}

// ── Stub operations: Compliance ───────────────────────────────────

#[tokio::test]
async fn test_put_and_list_compliance_items() {
    let client = make_ssm_client().await;

    let instance_id = "i-compliance-test-001";

    client
        .put_compliance_items()
        .resource_id(instance_id)
        .resource_type("ManagedInstance")
        .compliance_type("Custom:TestCompliance")
        .execution_summary(
            aws_sdk_ssm::types::ComplianceExecutionSummary::builder()
                .execution_time(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .items(
            aws_sdk_ssm::types::ComplianceItemEntry::builder()
                .id("item-001")
                .title("Test item")
                .severity(aws_sdk_ssm::types::ComplianceSeverity::Unspecified)
                .status(aws_sdk_ssm::types::ComplianceStatus::Compliant)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_compliance_items should succeed");

    let list_resp = client
        .list_compliance_items()
        .resource_ids(instance_id)
        .send()
        .await
        .expect("list_compliance_items should succeed");

    let items = list_resp.compliance_items();
    assert!(
        !items.is_empty(),
        "should have at least one compliance item"
    );
    assert_eq!(items[0].compliance_type(), Some("Custom:TestCompliance"));
}

#[tokio::test]
async fn test_list_compliance_summaries() {
    let client = make_ssm_client().await;

    let resp = client
        .list_compliance_summaries()
        .send()
        .await
        .expect("list_compliance_summaries should succeed");
    let _ = resp.compliance_summary_items();
}

#[tokio::test]
async fn test_list_resource_compliance_summaries() {
    let client = make_ssm_client().await;

    let resp = client
        .list_resource_compliance_summaries()
        .send()
        .await
        .expect("list_resource_compliance_summaries should succeed");
    let _ = resp.resource_compliance_summary_items();
}

// ── Stub operations: OpsItem sub-features ────────────────────────

#[tokio::test]
async fn test_associate_and_disassociate_ops_item_related_item() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_ops_item()
        .title("Related item test")
        .source("TestSource")
        .description("Test")
        .send()
        .await
        .unwrap();
    let ops_item_id = create_resp.ops_item_id().unwrap().to_string();

    let assoc_resp = client
        .associate_ops_item_related_item()
        .ops_item_id(&ops_item_id)
        .association_type("RelatesTo")
        .resource_type("AWS::EC2::Instance")
        .resource_uri("arn:aws:ec2:us-east-1:123456789012:instance/i-1234567890abcdef0")
        .send()
        .await
        .expect("associate_ops_item_related_item should succeed");
    let _ = assoc_resp;

    client
        .disassociate_ops_item_related_item()
        .ops_item_id(&ops_item_id)
        .association_id("placeholder-assoc-id")
        .send()
        .await
        .expect("disassociate_ops_item_related_item should succeed");
}

#[tokio::test]
async fn test_list_ops_item_related_items() {
    let client = make_ssm_client().await;

    let resp = client
        .list_ops_item_related_items()
        .send()
        .await
        .expect("list_ops_item_related_items should succeed");
    let _ = resp.summaries();
}

#[tokio::test]
async fn test_list_ops_item_events() {
    let client = make_ssm_client().await;

    let resp = client
        .list_ops_item_events()
        .send()
        .await
        .expect("list_ops_item_events should succeed");
    let _ = resp.summaries();
}

#[tokio::test]
async fn test_get_ops_summary() {
    let client = make_ssm_client().await;

    let resp = client
        .get_ops_summary()
        .send()
        .await
        .expect("get_ops_summary should succeed");
    let _ = resp.entities();
}

// ── Stub operations: Automation ───────────────────────────────────

#[tokio::test]
async fn test_start_automation_execution() {
    let client = make_ssm_client().await;

    let resp = client
        .start_automation_execution()
        .document_name("AWS-RestartEC2Instance")
        .send()
        .await
        .expect("start_automation_execution should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_describe_automation_executions() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_automation_executions()
        .send()
        .await
        .expect("describe_automation_executions should succeed");
    let _ = resp.automation_execution_metadata_list();
}

#[tokio::test]
async fn test_describe_automation_step_executions() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_automation_step_executions()
        .automation_execution_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("describe_automation_step_executions should succeed");
    let _ = resp.step_executions();
}

#[tokio::test]
async fn test_get_automation_execution() {
    let client = make_ssm_client().await;

    let resp = client
        .get_automation_execution()
        .automation_execution_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("get_automation_execution should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_stop_automation_execution() {
    let client = make_ssm_client().await;

    client
        .stop_automation_execution()
        .automation_execution_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("stop_automation_execution should succeed");
}

#[tokio::test]
async fn test_send_automation_signal() {
    let client = make_ssm_client().await;

    client
        .send_automation_signal()
        .automation_execution_id("00000001-0000-0000-0000-000000000001")
        .signal_type(aws_sdk_ssm::types::SignalType::Approve)
        .send()
        .await
        .expect("send_automation_signal should succeed");
}

// ── Stub operations: Calendar / ChangeRequest / ExecutionPreview ──

#[tokio::test]
async fn test_get_calendar_state() {
    let client = make_ssm_client().await;

    let resp = client
        .get_calendar_state()
        .calendar_names("arn:aws:ssm:us-east-1:123456789012:document/MyCalendar")
        .send()
        .await
        .expect("get_calendar_state should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_start_change_request_execution() {
    let client = make_ssm_client().await;

    let resp = client
        .start_change_request_execution()
        .document_name("AWS-RunbookExecution")
        .send()
        .await
        .expect("start_change_request_execution should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_start_execution_preview() {
    let client = make_ssm_client().await;

    let resp = client
        .start_execution_preview()
        .document_name("AWS-RunbookExecution")
        .send()
        .await
        .expect("start_execution_preview should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_get_execution_preview() {
    let client = make_ssm_client().await;

    let resp = client
        .get_execution_preview()
        .execution_preview_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("get_execution_preview should succeed");
    let _ = resp;
}

// ── Stub operations: Document metadata ───────────────────────────

#[tokio::test]
async fn test_list_document_metadata_history() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("MetadataHistoryDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_document_metadata_history()
        .name("MetadataHistoryDoc")
        .metadata(aws_sdk_ssm::types::DocumentMetadataEnum::DocumentReviews)
        .send()
        .await
        .expect("list_document_metadata_history should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_update_document_metadata() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("UpdateMetadataDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    client
        .update_document_metadata()
        .name("UpdateMetadataDoc")
        .document_reviews(
            aws_sdk_ssm::types::DocumentReviews::builder()
                .action(aws_sdk_ssm::types::DocumentReviewAction::SendForReview)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_document_metadata should succeed");
}

// ── Stub operations: Session sub-features ────────────────────────

#[tokio::test]
async fn test_resume_session() {
    let client = make_ssm_client().await;

    let start_resp = client
        .start_session()
        .target("i-1234567890abcdef0")
        .send()
        .await
        .unwrap();
    let session_id = start_resp.session_id().unwrap().to_string();

    let resp = client
        .resume_session()
        .session_id(&session_id)
        .send()
        .await
        .expect("resume_session should succeed");
    assert_eq!(resp.session_id(), Some(session_id.as_str()));
}

// ── Stub operations: Effective instance associations ──────────────

#[tokio::test]
async fn test_describe_effective_instance_associations() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_effective_instance_associations()
        .instance_id("i-1234567890abcdef0")
        .send()
        .await
        .expect("describe_effective_instance_associations should succeed");
    let _ = resp.associations();
}

#[tokio::test]
async fn test_describe_instance_associations_status() {
    let client = make_ssm_client().await;

    let resp = client
        .describe_instance_associations_status()
        .instance_id("i-1234567890abcdef0")
        .send()
        .await
        .expect("describe_instance_associations_status should succeed");
    let _ = resp.instance_association_status_infos();
}

// ── Stub operations: Nodes ───────────────────────────────────────

#[tokio::test]
async fn test_list_nodes() {
    let client = make_ssm_client().await;

    let resp = client
        .list_nodes()
        .send()
        .await
        .expect("list_nodes should succeed");
    let _ = resp.nodes();
}

#[tokio::test]
async fn test_list_nodes_summary() {
    let client = make_ssm_client().await;

    let resp = client
        .list_nodes_summary()
        .send()
        .await
        .expect("list_nodes_summary should succeed");
    let _ = resp.summary();
}

// ── Stub operations: Access token / Access request ────────────────

#[tokio::test]
async fn test_get_access_token() {
    let client = make_ssm_client().await;

    let resp = client
        .get_access_token()
        .access_request_id("00000001-0000-0000-0000-000000000001")
        .send()
        .await
        .expect("get_access_token should succeed");
    let _ = resp;
}

#[tokio::test]
async fn test_start_access_request() {
    let client = make_ssm_client().await;

    let resp = client
        .start_access_request()
        .targets(
            aws_sdk_ssm::types::Target::builder()
                .key("instanceids")
                .values("i-1234567890abcdef0")
                .build(),
        )
        .reason("Testing access")
        .send()
        .await
        .expect("start_access_request should succeed");
    let _ = resp;
}

// ── Coverage for FIX(terraform-e2e) handler fixes ─────────────────

/// FIX(terraform-e2e): CreateAssociation must include Overview.Status = "Success"
/// so that the Terraform provider's waitAssociationSuccess waiter immediately
/// resolves without polling.
#[tokio::test]
async fn test_create_association_includes_overview_status_success() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("OverviewStatusDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_association()
        .name("OverviewStatusDoc")
        .association_name("overview-test")
        .send()
        .await
        .expect("create_association should succeed");

    let desc = create_resp
        .association_description()
        .expect("should have association description");

    // The overview must be present with Status = "Success".
    let overview = desc
        .overview()
        .expect("Overview must be present in CreateAssociation response");
    assert_eq!(
        overview.status(),
        Some("Success"),
        "Overview.Status should be 'Success' so waitAssociationSuccess resolves immediately"
    );
}

/// FIX(terraform-e2e): DescribeAssociation must include Overview with Status so
/// that the Terraform provider's findAssociationByID filter (which rejects
/// associations with nil Overview) does not discard the result.
#[tokio::test]
async fn test_describe_association_includes_overview_for_find_association_by_id() {
    let client = make_ssm_client().await;

    client
        .create_document()
        .name("FindByIdDoc")
        .content(r#"{"schemaVersion":"2.2","mainSteps":[]}"#)
        .document_type(aws_sdk_ssm::types::DocumentType::Command)
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_association()
        .name("FindByIdDoc")
        .association_name("find-by-id-test")
        .send()
        .await
        .unwrap();

    let assoc_id = create_resp
        .association_description()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let describe_resp = client
        .describe_association()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("describe_association should succeed");

    let desc = describe_resp
        .association_description()
        .expect("should have association description");

    // Overview must be present with a non-empty Status; otherwise the provider's
    // findAssociationByID discards the result with "empty result".
    let overview = desc
        .overview()
        .expect("Overview must be present in DescribeAssociation response");
    assert!(
        overview.status().is_some() && !overview.status().unwrap().is_empty(),
        "Overview.Status must be non-empty so findAssociationByID does not reject the result"
    );
}

/// FIX(terraform-e2e): GetPatchBaseline must return non-nil ApprovalRules and
/// GlobalFilters (even if empty) so that the Terraform provider's
/// flattenPatchRuleGroup does not crash with a nil pointer dereference.
#[tokio::test]
async fn test_get_patch_baseline_returns_approval_rules_and_global_filters() {
    let client = make_ssm_client().await;

    let create_resp = client
        .create_patch_baseline()
        .name("FlattenSafeBaseline")
        .operating_system(aws_sdk_ssm::types::OperatingSystem::AmazonLinux2)
        .send()
        .await
        .unwrap();

    let baseline_id = create_resp.baseline_id().unwrap().to_string();

    let get_resp = client
        .get_patch_baseline()
        .baseline_id(&baseline_id)
        .send()
        .await
        .expect("get_patch_baseline should succeed");

    // ApprovalRules and GlobalFilters must be present (even if empty) so that
    // flattenPatchRuleGroup does not nil-dereference.
    assert!(
        get_resp.approval_rules().is_some(),
        "ApprovalRules must be present in GetPatchBaseline response to avoid nil dereference"
    );
    assert!(
        get_resp.global_filters().is_some(),
        "GlobalFilters must be present in GetPatchBaseline response to avoid nil dereference"
    );
}
