use aws_sdk_batch::config::BehaviorVersion;
use aws_sdk_batch::error::ProvideErrorMetadata;
use winterbaume_batch::BatchService;
use winterbaume_core::MockAws;

async fn make_batch_client() -> aws_sdk_batch::Client {
    let mock = MockAws::builder().with_service(BatchService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_batch::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_batch::Client::new(&config)
}

// === Job Queue tests ===

#[tokio::test]
async fn test_create_and_describe_job_queue() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    let resp = client
        .create_job_queue()
        .job_queue_name("test-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .expect("create_job_queue should succeed");

    assert_eq!(resp.job_queue_name().unwrap(), "test-queue");
    assert!(resp.job_queue_arn().unwrap().contains("test-queue"));

    let desc_resp = client
        .describe_job_queues()
        .job_queues("test-queue")
        .send()
        .await
        .expect("describe_job_queues should succeed");

    let queues = desc_resp.job_queues();
    assert_eq!(queues.len(), 1);
    assert_eq!(queues[0].job_queue_name().unwrap(), "test-queue");
    assert_eq!(queues[0].priority().unwrap(), 1);
}

#[tokio::test]
async fn test_delete_job_queue() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("del-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    client
        .delete_job_queue()
        .job_queue("del-queue")
        .send()
        .await
        .expect("delete_job_queue should succeed");

    let desc_resp = client
        .describe_job_queues()
        .job_queues("del-queue")
        .send()
        .await
        .expect("describe after delete should succeed");

    assert!(desc_resp.job_queues().is_empty());
}

#[tokio::test]
async fn test_update_job_queue() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("upd-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_job_queue()
        .job_queue("upd-queue")
        .priority(10)
        .state(aws_sdk_batch::types::JqState::Disabled)
        .send()
        .await
        .expect("update_job_queue should succeed");

    assert_eq!(resp.job_queue_name().unwrap(), "upd-queue");

    let desc_resp = client
        .describe_job_queues()
        .job_queues("upd-queue")
        .send()
        .await
        .unwrap();

    let queues = desc_resp.job_queues();
    assert_eq!(queues.len(), 1);
    assert_eq!(queues[0].priority().unwrap(), 10);
}

#[tokio::test]
async fn test_describe_all_job_queues() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("queue-a")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order.clone())
        .send()
        .await
        .unwrap();

    client
        .create_job_queue()
        .job_queue_name("queue-b")
        .priority(5)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_job_queues()
        .send()
        .await
        .expect("describe all job queues should succeed");

    assert_eq!(desc_resp.job_queues().len(), 2);
}

// === Job Definition tests ===

#[tokio::test]
async fn test_register_and_describe_job_definition() {
    let client = make_batch_client().await;

    let resource_req_memory = aws_sdk_batch::types::ResourceRequirement::builder()
        .r#type(aws_sdk_batch::types::ResourceType::Memory)
        .value("128")
        .build();

    let resource_req_vcpu = aws_sdk_batch::types::ResourceRequirement::builder()
        .r#type(aws_sdk_batch::types::ResourceType::Vcpu)
        .value("1")
        .build();

    let container_props = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox")
        .command("sleep")
        .command("10")
        .resource_requirements(resource_req_memory)
        .resource_requirements(resource_req_vcpu)
        .build();

    let resp = client
        .register_job_definition()
        .job_definition_name("sleep10")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props)
        .send()
        .await
        .expect("register_job_definition should succeed");

    assert_eq!(resp.job_definition_name(), Some("sleep10"));
    assert_eq!(resp.revision(), Some(1));

    let desc_resp = client
        .describe_job_definitions()
        .job_definitions("sleep10")
        .send()
        .await
        .expect("describe_job_definitions should succeed");

    let defs = desc_resp.job_definitions();
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].job_definition_name().unwrap(), "sleep10");
    assert_eq!(defs[0].revision().unwrap(), 1);
    assert_eq!(defs[0].status().unwrap(), "ACTIVE");
}

#[tokio::test]
async fn test_deregister_job_definition() {
    let client = make_batch_client().await;

    let container_props = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox")
        .command("sleep")
        .command("30")
        .build();

    let resp = client
        .register_job_definition()
        .job_definition_name("sleep30")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props)
        .send()
        .await
        .unwrap();

    let arn = resp.job_definition_arn().unwrap().to_string();

    client
        .deregister_job_definition()
        .job_definition(&arn)
        .send()
        .await
        .expect("deregister should succeed");

    let desc_resp = client
        .describe_job_definitions()
        .job_definitions(&arn)
        .send()
        .await
        .unwrap();

    let defs = desc_resp.job_definitions();
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].status().unwrap(), "INACTIVE");
}

#[tokio::test]
async fn test_register_multiple_revisions() {
    let client = make_batch_client().await;

    let container_props1 = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox:v1")
        .command("echo")
        .command("hello")
        .build();

    let resp1 = client
        .register_job_definition()
        .job_definition_name("multi-rev")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp1.revision(), Some(1));

    let container_props2 = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox:v2")
        .command("echo")
        .command("world")
        .build();

    let resp2 = client
        .register_job_definition()
        .job_definition_name("multi-rev")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props2)
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.revision(), Some(2));

    let desc_resp = client
        .describe_job_definitions()
        .job_definition_name("multi-rev")
        .send()
        .await
        .unwrap();

    assert_eq!(desc_resp.job_definitions().len(), 2);
}

// === Compute Environment tests ===

#[tokio::test]
async fn test_create_and_describe_compute_environment() {
    let client = make_batch_client().await;

    let resp = client
        .create_compute_environment()
        .compute_environment_name("test-ce")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .state(aws_sdk_batch::types::CeState::Enabled)
        .send()
        .await
        .expect("create_compute_environment should succeed");

    assert_eq!(resp.compute_environment_name().unwrap(), "test-ce");
    assert!(resp.compute_environment_arn().unwrap().contains("test-ce"));

    let desc_resp = client
        .describe_compute_environments()
        .compute_environments("test-ce")
        .send()
        .await
        .expect("describe_compute_environments should succeed");

    let envs = desc_resp.compute_environments();
    assert_eq!(envs.len(), 1);
    assert_eq!(envs[0].compute_environment_name().unwrap(), "test-ce");
}

#[tokio::test]
async fn test_delete_compute_environment() {
    let client = make_batch_client().await;

    client
        .create_compute_environment()
        .compute_environment_name("del-ce")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .send()
        .await
        .unwrap();

    client
        .delete_compute_environment()
        .compute_environment("del-ce")
        .send()
        .await
        .expect("delete_compute_environment should succeed");

    let desc_resp = client
        .describe_compute_environments()
        .compute_environments("del-ce")
        .send()
        .await
        .unwrap();

    assert!(desc_resp.compute_environments().is_empty());
}

#[tokio::test]
async fn test_update_compute_environment() {
    let client = make_batch_client().await;

    client
        .create_compute_environment()
        .compute_environment_name("upd-ce")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .state(aws_sdk_batch::types::CeState::Enabled)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_compute_environment()
        .compute_environment("upd-ce")
        .state(aws_sdk_batch::types::CeState::Disabled)
        .send()
        .await
        .expect("update_compute_environment should succeed");

    assert_eq!(resp.compute_environment_name().unwrap(), "upd-ce");

    let desc_resp = client
        .describe_compute_environments()
        .compute_environments("upd-ce")
        .send()
        .await
        .unwrap();

    let envs = desc_resp.compute_environments();
    assert_eq!(envs.len(), 1);
    assert_eq!(envs[0].state().unwrap().as_str(), "DISABLED");
}

#[tokio::test]
async fn test_describe_all_compute_environments() {
    let client = make_batch_client().await;

    client
        .create_compute_environment()
        .compute_environment_name("ce-a")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .send()
        .await
        .unwrap();

    client
        .create_compute_environment()
        .compute_environment_name("ce-b")
        .r#type(aws_sdk_batch::types::CeType::Unmanaged)
        .send()
        .await
        .unwrap();

    let desc_resp = client.describe_compute_environments().send().await.unwrap();

    assert_eq!(desc_resp.compute_environments().len(), 2);
}

// === Scheduling Policy tests ===

#[tokio::test]
async fn test_create_and_describe_scheduling_policy() {
    let client = make_batch_client().await;

    let share = aws_sdk_batch::types::ShareAttributes::builder()
        .share_identifier("groupA")
        .weight_factor(1.0)
        .build();

    let fairshare = aws_sdk_batch::types::FairsharePolicy::builder()
        .compute_reservation(50)
        .share_decay_seconds(3600)
        .share_distribution(share)
        .build();

    let resp = client
        .create_scheduling_policy()
        .name("test-sp")
        .fairshare_policy(fairshare)
        .send()
        .await
        .expect("create_scheduling_policy should succeed");

    assert_eq!(resp.name().unwrap(), "test-sp");
    assert!(resp.arn().unwrap().contains("test-sp"));

    let arn = resp.arn().unwrap().to_string();

    let desc_resp = client
        .describe_scheduling_policies()
        .arns(&arn)
        .send()
        .await
        .expect("describe_scheduling_policies should succeed");

    let policies = desc_resp.scheduling_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].name().unwrap(), "test-sp");
    let fp = policies[0].fairshare_policy().unwrap();
    assert_eq!(fp.compute_reservation(), Some(50));
}

#[tokio::test]
async fn test_list_scheduling_policies() {
    let client = make_batch_client().await;

    client
        .create_scheduling_policy()
        .name("sp-a")
        .send()
        .await
        .unwrap();

    client
        .create_scheduling_policy()
        .name("sp-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_scheduling_policies()
        .send()
        .await
        .expect("list_scheduling_policies should succeed");

    assert_eq!(resp.scheduling_policies().len(), 2);
}

#[tokio::test]
async fn test_delete_scheduling_policy() {
    let client = make_batch_client().await;

    let resp = client
        .create_scheduling_policy()
        .name("del-sp")
        .send()
        .await
        .unwrap();

    let arn = resp.arn().unwrap().to_string();

    client
        .delete_scheduling_policy()
        .arn(&arn)
        .send()
        .await
        .expect("delete_scheduling_policy should succeed");

    let desc_resp = client
        .describe_scheduling_policies()
        .arns(&arn)
        .send()
        .await
        .unwrap();

    assert!(desc_resp.scheduling_policies().is_empty());
}

#[tokio::test]
async fn test_update_scheduling_policy() {
    let client = make_batch_client().await;

    let resp = client
        .create_scheduling_policy()
        .name("upd-sp")
        .send()
        .await
        .unwrap();

    let arn = resp.arn().unwrap().to_string();

    let new_fairshare = aws_sdk_batch::types::FairsharePolicy::builder()
        .compute_reservation(75)
        .build();

    client
        .update_scheduling_policy()
        .arn(&arn)
        .fairshare_policy(new_fairshare)
        .send()
        .await
        .expect("update_scheduling_policy should succeed");

    let desc_resp = client
        .describe_scheduling_policies()
        .arns(&arn)
        .send()
        .await
        .unwrap();

    let policies = desc_resp.scheduling_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(
        policies[0]
            .fairshare_policy()
            .unwrap()
            .compute_reservation(),
        Some(75)
    );
}

// === Job tests ===

#[tokio::test]
async fn test_submit_and_describe_job() {
    let client = make_batch_client().await;

    // Set up prerequisites
    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("job-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let container_props = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox")
        .command("echo")
        .command("hello")
        .build();

    client
        .register_job_definition()
        .job_definition_name("echo-job")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props)
        .send()
        .await
        .unwrap();

    let resp = client
        .submit_job()
        .job_name("my-job")
        .job_queue("job-queue")
        .job_definition("echo-job")
        .send()
        .await
        .expect("submit_job should succeed");

    assert_eq!(resp.job_name().unwrap(), "my-job");
    let job_id = resp.job_id().unwrap().to_string();
    assert!(!job_id.is_empty());

    let desc_resp = client
        .describe_jobs()
        .jobs(&job_id)
        .send()
        .await
        .expect("describe_jobs should succeed");

    let jobs = desc_resp.jobs();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].job_name().unwrap(), "my-job");
    assert_eq!(jobs[0].status().unwrap().as_str(), "SUBMITTED");
}

#[tokio::test]
async fn test_list_jobs() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("list-jq")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let container_props = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox")
        .command("echo")
        .build();

    client
        .register_job_definition()
        .job_definition_name("list-jd")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props)
        .send()
        .await
        .unwrap();

    client
        .submit_job()
        .job_name("job-1")
        .job_queue("list-jq")
        .job_definition("list-jd")
        .send()
        .await
        .unwrap();

    client
        .submit_job()
        .job_name("job-2")
        .job_queue("list-jq")
        .job_definition("list-jd")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_jobs()
        .job_queue("list-jq")
        .send()
        .await
        .expect("list_jobs should succeed");

    assert_eq!(resp.job_summary_list().len(), 2);
}

#[tokio::test]
async fn test_cancel_job() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("cancel-jq")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let container_props = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox")
        .command("sleep")
        .command("60")
        .build();

    client
        .register_job_definition()
        .job_definition_name("cancel-jd")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props)
        .send()
        .await
        .unwrap();

    let resp = client
        .submit_job()
        .job_name("cancel-me")
        .job_queue("cancel-jq")
        .job_definition("cancel-jd")
        .send()
        .await
        .unwrap();

    let job_id = resp.job_id().unwrap().to_string();

    client
        .cancel_job()
        .job_id(&job_id)
        .reason("Testing cancel")
        .send()
        .await
        .expect("cancel_job should succeed");

    let desc_resp = client.describe_jobs().jobs(&job_id).send().await.unwrap();

    let jobs = desc_resp.jobs();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].status().unwrap().as_str(), "FAILED");
    assert_eq!(jobs[0].status_reason().unwrap(), "Testing cancel");
}

#[tokio::test]
async fn test_terminate_job() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("term-jq")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let container_props = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox")
        .command("sleep")
        .command("60")
        .build();

    client
        .register_job_definition()
        .job_definition_name("term-jd")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props)
        .send()
        .await
        .unwrap();

    let resp = client
        .submit_job()
        .job_name("terminate-me")
        .job_queue("term-jq")
        .job_definition("term-jd")
        .send()
        .await
        .unwrap();

    let job_id = resp.job_id().unwrap().to_string();

    client
        .terminate_job()
        .job_id(&job_id)
        .reason("Termination test")
        .send()
        .await
        .expect("terminate_job should succeed");

    let desc_resp = client.describe_jobs().jobs(&job_id).send().await.unwrap();

    let jobs = desc_resp.jobs();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].status().unwrap().as_str(), "FAILED");
    assert_eq!(jobs[0].status_reason().unwrap(), "Termination test");
}

// === Tag tests ===

#[tokio::test]
async fn test_tag_and_list_tags_for_compute_environment() {
    let client = make_batch_client().await;

    let resp = client
        .create_compute_environment()
        .compute_environment_name("tag-ce")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .send()
        .await
        .unwrap();

    let arn = resp.compute_environment_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "test");
    assert_eq!(tags.get("team").unwrap(), "platform");

    // Untag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags2 = tags_resp2.tags().unwrap();
    assert_eq!(tags2.get("env").unwrap(), "test");
    assert!(tags2.get("team").is_none());
}

// === Lifecycle test ===

#[tokio::test]
async fn test_compute_environment_lifecycle() {
    let client = make_batch_client().await;

    // Create
    let resp = client
        .create_compute_environment()
        .compute_environment_name("lifecycle-ce")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .state(aws_sdk_batch::types::CeState::Enabled)
        .send()
        .await
        .unwrap();

    let arn = resp.compute_environment_arn().unwrap().to_string();

    // Describe
    let desc = client
        .describe_compute_environments()
        .compute_environments("lifecycle-ce")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.compute_environments().len(), 1);

    // Update
    client
        .update_compute_environment()
        .compute_environment("lifecycle-ce")
        .state(aws_sdk_batch::types::CeState::Disabled)
        .send()
        .await
        .unwrap();

    // Verify update
    let desc2 = client
        .describe_compute_environments()
        .compute_environments("lifecycle-ce")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.compute_environments()[0].state().unwrap().as_str(),
        "DISABLED"
    );

    // Delete
    client
        .delete_compute_environment()
        .compute_environment(&arn)
        .send()
        .await
        .unwrap();

    // Verify gone
    let desc3 = client
        .describe_compute_environments()
        .compute_environments("lifecycle-ce")
        .send()
        .await
        .unwrap();
    assert!(desc3.compute_environments().is_empty());
}

#[tokio::test]
async fn test_scheduling_policy_lifecycle() {
    let client = make_batch_client().await;

    // Create
    let resp = client
        .create_scheduling_policy()
        .name("lifecycle-sp")
        .send()
        .await
        .unwrap();

    let arn = resp.arn().unwrap().to_string();

    // Describe
    let desc = client
        .describe_scheduling_policies()
        .arns(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.scheduling_policies().len(), 1);

    // List
    let list = client.list_scheduling_policies().send().await.unwrap();
    assert!(!list.scheduling_policies().is_empty());

    // Update
    let new_fp = aws_sdk_batch::types::FairsharePolicy::builder()
        .compute_reservation(25)
        .build();
    client
        .update_scheduling_policy()
        .arn(&arn)
        .fairshare_policy(new_fp)
        .send()
        .await
        .unwrap();

    // Verify update
    let desc2 = client
        .describe_scheduling_policies()
        .arns(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.scheduling_policies()[0]
            .fairshare_policy()
            .unwrap()
            .compute_reservation(),
        Some(25)
    );

    // Delete
    client
        .delete_scheduling_policy()
        .arn(&arn)
        .send()
        .await
        .unwrap();

    // Verify gone
    let desc3 = client
        .describe_scheduling_policies()
        .arns(&arn)
        .send()
        .await
        .unwrap();
    assert!(desc3.scheduling_policies().is_empty());
}

#[tokio::test]
async fn test_job_lifecycle() {
    let client = make_batch_client().await;

    // Setup
    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("lifecycle-jq")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let container_props = aws_sdk_batch::types::ContainerProperties::builder()
        .image("busybox")
        .command("echo")
        .command("test")
        .build();

    client
        .register_job_definition()
        .job_definition_name("lifecycle-jd")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(container_props)
        .send()
        .await
        .unwrap();

    // Submit
    let resp = client
        .submit_job()
        .job_name("lifecycle-job")
        .job_queue("lifecycle-jq")
        .job_definition("lifecycle-jd")
        .send()
        .await
        .unwrap();

    let job_id = resp.job_id().unwrap().to_string();

    // Describe
    let desc = client.describe_jobs().jobs(&job_id).send().await.unwrap();
    assert_eq!(desc.jobs().len(), 1);
    assert_eq!(desc.jobs()[0].status().unwrap().as_str(), "SUBMITTED");

    // List
    let list = client
        .list_jobs()
        .job_queue("lifecycle-jq")
        .send()
        .await
        .unwrap();
    assert!(!list.job_summary_list().is_empty());

    // Cancel
    client
        .cancel_job()
        .job_id(&job_id)
        .reason("lifecycle test cancel")
        .send()
        .await
        .unwrap();

    // Verify cancelled
    let desc2 = client.describe_jobs().jobs(&job_id).send().await.unwrap();
    assert_eq!(desc2.jobs()[0].status().unwrap().as_str(), "FAILED");
}

// === Additional Job Queue tests from moto ===

#[tokio::test]
async fn test_describe_job_queue_unknown_value() {
    let client = make_batch_client().await;

    let resp = client
        .describe_job_queues()
        .job_queues("test_invalid_queue")
        .send()
        .await
        .expect("describe_job_queues with unknown name should succeed (return empty)");

    assert_eq!(resp.job_queues().len(), 0);
}

#[tokio::test]
async fn test_create_job_queue_twice() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("dup-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_job_queue()
        .job_queue_name("dup-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert_eq!(
        svc_err.code(),
        Some("ClientException"),
        "expected ClientException, got: {svc_err:?}"
    );
    assert!(
        svc_err
            .message()
            .unwrap_or("")
            .contains("dup-queue already exists"),
        "expected 'already exists' in message, got: {:?}",
        svc_err.message()
    );
}

#[tokio::test]
async fn test_create_job_queue_with_tags() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    let resp = client
        .create_job_queue()
        .job_queue_name("tagged-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .tags("k1", "v1")
        .tags("k2", "v2")
        .send()
        .await
        .expect("create_job_queue with tags should succeed");

    let queue_arn = resp.job_queue_arn().unwrap().to_string();

    let desc_resp = client
        .describe_job_queues()
        .job_queues(&queue_arn)
        .send()
        .await
        .expect("describe_job_queues by ARN should succeed");

    let queues = desc_resp.job_queues();
    assert_eq!(queues.len(), 1);
    let tags = queues[0].tags().unwrap();
    assert_eq!(tags.get("k1").map(String::as_str), Some("v1"));
    assert_eq!(tags.get("k2").map(String::as_str), Some("v2"));
}

#[tokio::test]
async fn test_list_tags_for_job_queue() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    let resp = client
        .create_job_queue()
        .job_queue_name("tags-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .tags("k1", "v1")
        .tags("k2", "v2")
        .send()
        .await
        .unwrap();

    let queue_arn = resp.job_queue_arn().unwrap().to_string();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&queue_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("k1").map(String::as_str), Some("v1"));
    assert_eq!(tags.get("k2").map(String::as_str), Some("v2"));
}

#[tokio::test]
async fn test_tag_job_queue() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    let resp = client
        .create_job_queue()
        .job_queue_name("tag-jq")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let queue_arn = resp.job_queue_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&queue_arn)
        .tags("k1", "v1")
        .tags("k2", "v2")
        .send()
        .await
        .expect("tag_resource on job queue should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&queue_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("k1").map(String::as_str), Some("v1"));
    assert_eq!(tags.get("k2").map(String::as_str), Some("v2"));
}

#[tokio::test]
async fn test_untag_job_queue() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    let resp = client
        .create_job_queue()
        .job_queue_name("untag-jq")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .tags("k1", "v1")
        .tags("k2", "v2")
        .send()
        .await
        .unwrap();

    let queue_arn = resp.job_queue_arn().unwrap().to_string();

    // Add k3, remove k2
    client
        .tag_resource()
        .resource_arn(&queue_arn)
        .tags("k3", "v3")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&queue_arn)
        .tag_keys("k2")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&queue_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("k1").map(String::as_str), Some("v1"));
    assert!(tags.get("k2").is_none(), "k2 should have been removed");
    assert_eq!(tags.get("k3").map(String::as_str), Some("v3"));
}

// ============================================================================
// Tests derived from AWS documentation: AWS Batch
// ============================================================================

#[tokio::test]
async fn test_duplicate_compute_environment_fails() {
    let client = make_batch_client().await;

    client
        .create_compute_environment()
        .compute_environment_name("dup-ce")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .send()
        .await
        .expect("first create_compute_environment should succeed");

    let result = client
        .create_compute_environment()
        .compute_environment_name("dup-ce")
        .r#type(aws_sdk_batch::types::CeType::Managed)
        .send()
        .await;

    assert!(result.is_err(), "duplicate compute environment should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_compute_environment() {
    let client = make_batch_client().await;

    let result = client
        .delete_compute_environment()
        .compute_environment("nonexistent-ce")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete nonexistent compute environment should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_job_queue() {
    let client = make_batch_client().await;

    let result = client
        .delete_job_queue()
        .job_queue("nonexistent-queue")
        .send()
        .await;

    assert!(result.is_err(), "delete nonexistent job queue should fail");
}

#[tokio::test]
async fn test_duplicate_scheduling_policy_fails() {
    let client = make_batch_client().await;

    client
        .create_scheduling_policy()
        .name("dup-sp")
        .send()
        .await
        .expect("first create_scheduling_policy should succeed");

    let result = client
        .create_scheduling_policy()
        .name("dup-sp")
        .send()
        .await;

    assert!(result.is_err(), "duplicate scheduling policy should fail");
}

#[tokio::test]
async fn test_list_jobs_empty() {
    let client = make_batch_client().await;

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("empty-jobs-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_jobs()
        .job_queue("empty-jobs-queue")
        .send()
        .await
        .expect("list_jobs should succeed on queue with no jobs");

    assert_eq!(resp.job_summary_list().len(), 0);
}

#[tokio::test]
async fn test_cancel_nonexistent_job() {
    let client = make_batch_client().await;

    let result = client
        .cancel_job()
        .job_id("nonexistent-job-id")
        .reason("test")
        .send()
        .await;

    assert!(result.is_err(), "cancel nonexistent job should fail");
}

#[tokio::test]
async fn test_terminate_nonexistent_job() {
    let client = make_batch_client().await;

    let result = client
        .terminate_job()
        .job_id("nonexistent-job-id")
        .reason("test")
        .send()
        .await;

    assert!(result.is_err(), "terminate nonexistent job should fail");
}

#[tokio::test]
async fn test_job_definition_name_revision_lookup() {
    let client = make_batch_client().await;

    let resp = client
        .register_job_definition()
        .job_definition_name("revision-test-jd")
        .r#type(aws_sdk_batch::types::JobDefinitionType::Container)
        .container_properties(
            aws_sdk_batch::types::ContainerProperties::builder()
                .image("busybox")
                .resource_requirements(
                    aws_sdk_batch::types::ResourceRequirement::builder()
                        .r#type(aws_sdk_batch::types::ResourceType::Memory)
                        .value("128")
                        .build(),
                )
                .resource_requirements(
                    aws_sdk_batch::types::ResourceRequirement::builder()
                        .r#type(aws_sdk_batch::types::ResourceType::Vcpu)
                        .value("1")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let revision = resp.revision().unwrap_or(1);
    let name_revision = format!("revision-test-jd:{}", revision);

    let desc_resp = client
        .describe_job_definitions()
        .job_definitions(&name_revision)
        .send()
        .await
        .expect("describe_job_definitions with name:revision should succeed");

    assert_eq!(desc_resp.job_definitions().len(), 1);
    assert_eq!(
        desc_resp.job_definitions()[0].job_definition_name(),
        Some("revision-test-jd")
    );
}

#[tokio::test]
async fn test_job_queue_with_scheduling_policy() {
    let client = make_batch_client().await;

    let sp_resp = client
        .create_scheduling_policy()
        .name("jq-sp")
        .send()
        .await
        .unwrap();

    let sp_arn = sp_resp.arn().unwrap().to_string();

    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    let jq_resp = client
        .create_job_queue()
        .job_queue_name("sp-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .scheduling_policy_arn(&sp_arn)
        .send()
        .await
        .expect("create_job_queue with scheduling_policy_arn should succeed");

    assert_eq!(jq_resp.job_queue_name().unwrap(), "sp-queue");

    let desc = client
        .describe_job_queues()
        .job_queues("sp-queue")
        .send()
        .await
        .unwrap();

    let queue = &desc.job_queues()[0];
    assert_eq!(queue.scheduling_policy_arn(), Some(sp_arn.as_str()));
}

// === GetJobQueueSnapshot tests ===

#[tokio::test]
async fn test_get_job_queue_snapshot() {
    let client = make_batch_client().await;

    // Create a job queue first
    let ce_order = aws_sdk_batch::types::ComputeEnvironmentOrder::builder()
        .order(1)
        .compute_environment("M4Spot")
        .build();

    client
        .create_job_queue()
        .job_queue_name("snapshot-queue")
        .priority(1)
        .state(aws_sdk_batch::types::JqState::Enabled)
        .compute_environment_order(ce_order)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_job_queue_snapshot()
        .job_queue("snapshot-queue")
        .send()
        .await
        .expect("get_job_queue_snapshot should succeed");

    let front = resp.front_of_queue().unwrap();
    assert!(front.jobs().is_empty());
    assert!(front.last_updated_at().is_some());
}

// === ConsumableResource tests ===

#[tokio::test]
async fn test_create_and_describe_consumable_resource() {
    let client = make_batch_client().await;

    let resp = client
        .create_consumable_resource()
        .consumable_resource_name("gpu-quota")
        .total_quantity(100)
        .resource_type("replenishable")
        .send()
        .await
        .expect("create_consumable_resource should succeed");

    assert_eq!(resp.consumable_resource_name().unwrap(), "gpu-quota");
    assert!(
        resp.consumable_resource_arn()
            .unwrap()
            .contains("gpu-quota")
    );

    let desc = client
        .describe_consumable_resource()
        .consumable_resource("gpu-quota")
        .send()
        .await
        .expect("describe_consumable_resource should succeed");

    assert_eq!(desc.consumable_resource_name().unwrap(), "gpu-quota");
    assert_eq!(desc.total_quantity(), Some(100));
    assert_eq!(desc.in_use_quantity(), Some(0));
    assert_eq!(desc.available_quantity(), Some(100));
    assert_eq!(desc.resource_type().unwrap(), "replenishable");
}

#[tokio::test]
async fn test_delete_consumable_resource() {
    let client = make_batch_client().await;

    client
        .create_consumable_resource()
        .consumable_resource_name("delete-me-cr")
        .total_quantity(50)
        .send()
        .await
        .unwrap();

    client
        .delete_consumable_resource()
        .consumable_resource("delete-me-cr")
        .send()
        .await
        .expect("delete_consumable_resource should succeed");

    let err = client
        .describe_consumable_resource()
        .consumable_resource("delete-me-cr")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_update_consumable_resource() {
    let client = make_batch_client().await;

    client
        .create_consumable_resource()
        .consumable_resource_name("update-cr")
        .total_quantity(100)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_consumable_resource()
        .consumable_resource("update-cr")
        .quantity(200)
        .send()
        .await
        .expect("update_consumable_resource should succeed");

    assert_eq!(resp.consumable_resource_name().unwrap(), "update-cr");
    assert_eq!(resp.total_quantity(), Some(200i64));
}

#[tokio::test]
async fn test_list_consumable_resources() {
    let client = make_batch_client().await;

    client
        .create_consumable_resource()
        .consumable_resource_name("cr-alpha")
        .total_quantity(10)
        .send()
        .await
        .unwrap();

    client
        .create_consumable_resource()
        .consumable_resource_name("cr-beta")
        .total_quantity(20)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_consumable_resources()
        .send()
        .await
        .expect("list_consumable_resources should succeed");

    assert_eq!(resp.consumable_resources().len(), 2);
}

#[tokio::test]
async fn test_list_jobs_by_consumable_resource() {
    let client = make_batch_client().await;

    client
        .create_consumable_resource()
        .consumable_resource_name("list-jobs-cr")
        .total_quantity(10)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_jobs_by_consumable_resource()
        .consumable_resource("list-jobs-cr")
        .send()
        .await
        .expect("list_jobs_by_consumable_resource should succeed");

    assert!(resp.jobs().is_empty());
}

// === ServiceEnvironment tests ===

#[tokio::test]
async fn test_create_and_describe_service_environment() {
    let client = make_batch_client().await;

    let resp = client
        .create_service_environment()
        .service_environment_name("test-se")
        .service_environment_type("FlexCompute".into())
        .state("ENABLED".into())
        .send()
        .await
        .expect("create_service_environment should succeed");

    assert_eq!(resp.service_environment_name().unwrap(), "test-se");
    assert!(resp.service_environment_arn().unwrap().contains("test-se"));

    let desc = client
        .describe_service_environments()
        .service_environments("test-se")
        .send()
        .await
        .expect("describe_service_environments should succeed");

    let envs = desc.service_environments();
    assert_eq!(envs.len(), 1);
    assert_eq!(envs[0].service_environment_name().unwrap(), "test-se");
}

#[tokio::test]
async fn test_delete_service_environment() {
    let client = make_batch_client().await;

    client
        .create_service_environment()
        .service_environment_name("delete-me-se")
        .service_environment_type("FlexCompute".into())
        .send()
        .await
        .unwrap();

    client
        .delete_service_environment()
        .service_environment("delete-me-se")
        .send()
        .await
        .expect("delete_service_environment should succeed");

    let desc = client
        .describe_service_environments()
        .service_environments("delete-me-se")
        .send()
        .await
        .unwrap();

    assert!(desc.service_environments().is_empty());
}

#[tokio::test]
async fn test_update_service_environment() {
    let client = make_batch_client().await;

    client
        .create_service_environment()
        .service_environment_name("update-se")
        .service_environment_type("FlexCompute".into())
        .state("ENABLED".into())
        .send()
        .await
        .unwrap();

    let resp = client
        .update_service_environment()
        .service_environment("update-se")
        .state("DISABLED".into())
        .send()
        .await
        .expect("update_service_environment should succeed");

    assert_eq!(resp.service_environment_name().unwrap(), "update-se");
}

// === ServiceJob tests ===

#[tokio::test]
async fn test_submit_and_describe_service_job() {
    let client = make_batch_client().await;

    let resp = client
        .submit_service_job()
        .job_name("test-svc-job")
        .job_queue("some-queue")
        .send()
        .await
        .expect("submit_service_job should succeed");

    assert_eq!(resp.job_name().unwrap(), "test-svc-job");
    let job_id = resp.job_id().unwrap().to_string();

    let desc = client
        .describe_service_job()
        .job_id(&job_id)
        .send()
        .await
        .expect("describe_service_job should succeed");

    assert_eq!(desc.job_name().unwrap(), "test-svc-job");
    assert_eq!(desc.status().unwrap().as_str(), "SUBMITTED");
    assert_eq!(desc.is_terminated(), Some(false));
}

#[tokio::test]
async fn test_terminate_service_job() {
    let client = make_batch_client().await;

    let resp = client
        .submit_service_job()
        .job_name("terminate-svc-job")
        .job_queue("some-queue")
        .send()
        .await
        .unwrap();

    let job_id = resp.job_id().unwrap().to_string();

    client
        .terminate_service_job()
        .job_id(&job_id)
        .send()
        .await
        .expect("terminate_service_job should succeed");

    let desc = client
        .describe_service_job()
        .job_id(&job_id)
        .send()
        .await
        .unwrap();

    assert_eq!(desc.status().unwrap().as_str(), "FAILED");
    assert_eq!(desc.is_terminated(), Some(true));
}

#[tokio::test]
async fn test_list_service_jobs() {
    let client = make_batch_client().await;

    client
        .submit_service_job()
        .job_name("svc-job-1")
        .job_queue("list-queue")
        .send()
        .await
        .unwrap();

    client
        .submit_service_job()
        .job_name("svc-job-2")
        .job_queue("list-queue")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_service_jobs()
        .job_queue("list-queue")
        .send()
        .await
        .expect("list_service_jobs should succeed");

    assert_eq!(resp.job_summary_list().len(), 2);
}
