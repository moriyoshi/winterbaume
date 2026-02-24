use aws_sdk_emr::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_emr::EmrService;

async fn make_client() -> aws_sdk_emr::Client {
    let mock = MockAws::builder().with_service(EmrService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emr::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_emr::Client::new(&config)
}

// ---- Cluster lifecycle tests ----

#[tokio::test]
async fn test_run_job_flow_and_describe_cluster() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("test-cluster")
        .release_label("emr-6.10.0")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .slave_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");
    assert!(
        cluster_id.starts_with("j-"),
        "cluster id should start with j-"
    );

    let desc = client
        .describe_cluster()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("describe_cluster should succeed");

    let cluster = desc.cluster().expect("should have cluster");
    assert_eq!(cluster.name(), Some("test-cluster"));
    assert_eq!(
        cluster.status().unwrap().state().unwrap(),
        &aws_sdk_emr::types::ClusterState::Waiting
    );
}

#[tokio::test]
async fn test_list_clusters() {
    let client = make_client().await;

    client
        .run_job_flow()
        .name("cluster-a")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    client
        .run_job_flow()
        .name("cluster-b")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");

    let clusters = resp.clusters();
    assert!(clusters.len() >= 2, "should have at least 2 clusters");
    let names: Vec<&str> = clusters.iter().filter_map(|c| c.name()).collect();
    assert!(names.contains(&"cluster-a"));
    assert!(names.contains(&"cluster-b"));
}

#[tokio::test]
async fn test_describe_nonexistent_cluster() {
    let client = make_client().await;

    let result = client
        .describe_cluster()
        .cluster_id("j-NONEXISTENT")
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent cluster");
}

#[tokio::test]
async fn test_terminate_job_flows() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("to-terminate")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    client
        .terminate_job_flows()
        .job_flow_ids(cluster_id)
        .send()
        .await
        .expect("terminate_job_flows should succeed");

    let desc = client
        .describe_cluster()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("describe_cluster should succeed after termination");

    let state = desc.cluster().unwrap().status().unwrap().state().unwrap();
    assert_eq!(state, &aws_sdk_emr::types::ClusterState::Terminated);
}

// ---- Step tests ----

#[tokio::test]
async fn test_add_and_describe_step() {
    let client = make_client().await;

    let cluster_resp = client
        .run_job_flow()
        .name("step-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = cluster_resp.job_flow_id().expect("should have job_flow_id");

    let step_config = aws_sdk_emr::types::StepConfig::builder()
        .name("my-step")
        .hadoop_jar_step(
            aws_sdk_emr::types::HadoopJarStepConfig::builder()
                .jar("command-runner.jar")
                .args("spark-submit")
                .args("my_app.py")
                .build(),
        )
        .build();

    let add_resp = client
        .add_job_flow_steps()
        .job_flow_id(cluster_id)
        .steps(step_config)
        .send()
        .await
        .expect("add_job_flow_steps should succeed");

    let step_ids = add_resp.step_ids();
    assert_eq!(step_ids.len(), 1, "should have 1 step id");
    let step_id = &step_ids[0];
    assert!(step_id.starts_with("s-"), "step id should start with s-");

    let step_desc = client
        .describe_step()
        .cluster_id(cluster_id)
        .step_id(step_id)
        .send()
        .await
        .expect("describe_step should succeed");

    let step = step_desc.step().expect("should have step");
    assert_eq!(step.name(), Some("my-step"));
}

#[tokio::test]
async fn test_list_steps() {
    let client = make_client().await;

    let cluster_resp = client
        .run_job_flow()
        .name("list-steps-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = cluster_resp.job_flow_id().expect("should have job_flow_id");

    let step_config = aws_sdk_emr::types::StepConfig::builder()
        .name("step-one")
        .hadoop_jar_step(
            aws_sdk_emr::types::HadoopJarStepConfig::builder()
                .jar("command-runner.jar")
                .build(),
        )
        .build();

    client
        .add_job_flow_steps()
        .job_flow_id(cluster_id)
        .steps(step_config)
        .send()
        .await
        .expect("add_job_flow_steps should succeed");

    let steps_resp = client
        .list_steps()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("list_steps should succeed");

    assert_eq!(steps_resp.steps().len(), 1);
    assert_eq!(steps_resp.steps()[0].name(), Some("step-one"));
}

#[tokio::test]
async fn test_cancel_steps() {
    let client = make_client().await;

    let cluster_resp = client
        .run_job_flow()
        .name("cancel-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = cluster_resp.job_flow_id().expect("should have job_flow_id");

    let step_config = aws_sdk_emr::types::StepConfig::builder()
        .name("cancel-step")
        .hadoop_jar_step(
            aws_sdk_emr::types::HadoopJarStepConfig::builder()
                .jar("command-runner.jar")
                .build(),
        )
        .build();

    let add_resp = client
        .add_job_flow_steps()
        .job_flow_id(cluster_id)
        .steps(step_config)
        .send()
        .await
        .expect("add_job_flow_steps should succeed");

    let step_id = add_resp.step_ids()[0].clone();

    let cancel_resp = client
        .cancel_steps()
        .cluster_id(cluster_id)
        .step_ids(&step_id)
        .send()
        .await
        .expect("cancel_steps should succeed");

    assert_eq!(cancel_resp.cancel_steps_info_list().len(), 1);
}

// ---- Security Configuration tests ----

#[tokio::test]
async fn test_security_configuration_lifecycle() {
    let client = make_client().await;

    client
        .create_security_configuration()
        .name("my-sec-config")
        .security_configuration(
            r#"{"EncryptionConfiguration":{"EnableInTransitEncryption":false}}"#,
        )
        .send()
        .await
        .expect("create_security_configuration should succeed");

    let desc = client
        .describe_security_configuration()
        .name("my-sec-config")
        .send()
        .await
        .expect("describe_security_configuration should succeed");

    assert_eq!(desc.name(), Some("my-sec-config"));

    let list_resp = client
        .list_security_configurations()
        .send()
        .await
        .expect("list_security_configurations should succeed");

    assert!(!list_resp.security_configurations().is_empty());

    client
        .delete_security_configuration()
        .name("my-sec-config")
        .send()
        .await
        .expect("delete_security_configuration should succeed");

    let result = client
        .describe_security_configuration()
        .name("my-sec-config")
        .send()
        .await;

    assert!(result.is_err(), "should fail after deletion");
}

// ---- Tags tests ----

#[tokio::test]
async fn test_add_and_remove_tags() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("tagged-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    let tag = aws_sdk_emr::types::Tag::builder()
        .key("env")
        .value("test")
        .build();

    client
        .add_tags()
        .resource_id(cluster_id)
        .tags(tag)
        .send()
        .await
        .expect("add_tags should succeed");

    let desc = client
        .describe_cluster()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("describe_cluster should succeed");

    let tags = desc.cluster().unwrap().tags();
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("env") && t.value() == Some("test"))
    );

    client
        .remove_tags()
        .resource_id(cluster_id)
        .tag_keys("env")
        .send()
        .await
        .expect("remove_tags should succeed");

    let desc2 = client
        .describe_cluster()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("describe_cluster should succeed after tag removal");

    let tags2 = desc2.cluster().unwrap().tags();
    assert!(!tags2.iter().any(|t| t.key() == Some("env")));
}

// ---- Managed Scaling Policy tests ----

#[tokio::test]
async fn test_managed_scaling_policy() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("scaling-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    let compute_limits = aws_sdk_emr::types::ComputeLimits::builder()
        .unit_type(aws_sdk_emr::types::ComputeLimitsUnitType::Instances)
        .minimum_capacity_units(1)
        .maximum_capacity_units(10)
        .build();

    let policy = aws_sdk_emr::types::ManagedScalingPolicy::builder()
        .compute_limits(compute_limits)
        .build();

    client
        .put_managed_scaling_policy()
        .cluster_id(cluster_id)
        .managed_scaling_policy(policy)
        .send()
        .await
        .expect("put_managed_scaling_policy should succeed");

    let get_resp = client
        .get_managed_scaling_policy()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("get_managed_scaling_policy should succeed");

    let msp = get_resp.managed_scaling_policy().unwrap();
    let limits = msp.compute_limits().unwrap();
    assert_eq!(limits.minimum_capacity_units(), Some(1));
    assert_eq!(limits.maximum_capacity_units(), Some(10));

    client
        .remove_managed_scaling_policy()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("remove_managed_scaling_policy should succeed");

    let get_resp2 = client
        .get_managed_scaling_policy()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("get_managed_scaling_policy should succeed after removal");

    assert!(get_resp2.managed_scaling_policy().is_none());
}

// ---- Auto Termination Policy tests ----

#[tokio::test]
async fn test_auto_termination_policy() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("auto-term-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    let policy = aws_sdk_emr::types::AutoTerminationPolicy::builder()
        .idle_timeout(3600)
        .build();

    client
        .put_auto_termination_policy()
        .cluster_id(cluster_id)
        .auto_termination_policy(policy)
        .send()
        .await
        .expect("put_auto_termination_policy should succeed");

    let get_resp = client
        .get_auto_termination_policy()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("get_auto_termination_policy should succeed");

    let atp = get_resp.auto_termination_policy().unwrap();
    assert_eq!(atp.idle_timeout(), Some(3600));

    client
        .remove_auto_termination_policy()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("remove_auto_termination_policy should succeed");
}

// ---- Block Public Access Configuration tests ----

#[tokio::test]
async fn test_block_public_access_configuration() {
    let client = make_client().await;

    let config = aws_sdk_emr::types::BlockPublicAccessConfiguration::builder()
        .block_public_security_group_rules(true)
        .build();

    client
        .put_block_public_access_configuration()
        .block_public_access_configuration(config)
        .send()
        .await
        .expect("put_block_public_access_configuration should succeed");

    let get_resp = client
        .get_block_public_access_configuration()
        .send()
        .await
        .expect("get_block_public_access_configuration should succeed");

    let bpac = get_resp
        .block_public_access_configuration()
        .expect("should have block_public_access_configuration");
    assert_eq!(bpac.block_public_security_group_rules(), Some(true));
}

// ---- Termination Protection tests ----

#[tokio::test]
async fn test_set_termination_protection() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("protected-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    client
        .set_termination_protection()
        .job_flow_ids(cluster_id)
        .termination_protected(true)
        .send()
        .await
        .expect("set_termination_protection should succeed");

    let desc = client
        .describe_cluster()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("describe_cluster should succeed");

    assert_eq!(desc.cluster().unwrap().termination_protected(), Some(true));
}

// ---- Visible to all users test ----

#[tokio::test]
async fn test_set_visible_to_all_users() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("visible-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    client
        .set_visible_to_all_users()
        .job_flow_ids(cluster_id)
        .visible_to_all_users(false)
        .send()
        .await
        .expect("set_visible_to_all_users should succeed");

    let desc = client
        .describe_cluster()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("describe_cluster should succeed");

    assert_eq!(desc.cluster().unwrap().visible_to_all_users(), Some(false));
}

// ---- Bootstrap Actions tests ----

#[tokio::test]
async fn test_list_bootstrap_actions() {
    let client = make_client().await;

    let bootstrap = aws_sdk_emr::types::BootstrapActionConfig::builder()
        .name("my-bootstrap")
        .script_bootstrap_action(
            aws_sdk_emr::types::ScriptBootstrapActionConfig::builder()
                .path("s3://mybucket/bootstrap.sh")
                .build(),
        )
        .build();

    let resp = client
        .run_job_flow()
        .name("bootstrap-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .bootstrap_actions(bootstrap)
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    let list_resp = client
        .list_bootstrap_actions()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("list_bootstrap_actions should succeed");

    assert_eq!(list_resp.bootstrap_actions().len(), 1);
    assert_eq!(
        list_resp.bootstrap_actions()[0].name(),
        Some("my-bootstrap")
    );
}

// ---- Instance Group tests ----

#[tokio::test]
async fn test_add_list_modify_instance_groups() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("ig-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    // AddInstanceGroups
    let ig_config = aws_sdk_emr::types::InstanceGroupConfig::builder()
        .instance_role(aws_sdk_emr::types::InstanceRoleType::Core)
        .instance_type("m5.xlarge")
        .instance_count(2)
        .build();

    let add_resp = client
        .add_instance_groups()
        .job_flow_id(cluster_id)
        .instance_groups(ig_config)
        .send()
        .await
        .expect("add_instance_groups should succeed");

    let ig_ids = add_resp.instance_group_ids();
    assert_eq!(ig_ids.len(), 1, "should have 1 instance group id");
    let ig_id = &ig_ids[0];
    assert!(
        ig_id.starts_with("ig-"),
        "instance group id should start with ig-"
    );

    // ListInstanceGroups
    let list_resp = client
        .list_instance_groups()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("list_instance_groups should succeed");

    let groups = list_resp.instance_groups();
    assert!(
        groups.iter().any(|g| g.id() == Some(ig_id.as_str())),
        "listed groups should contain the added group"
    );

    // ModifyInstanceGroups
    let modification = aws_sdk_emr::types::InstanceGroupModifyConfig::builder()
        .instance_group_id(ig_id)
        .instance_count(4)
        .build();

    client
        .modify_instance_groups()
        .cluster_id(cluster_id)
        .instance_groups(modification)
        .send()
        .await
        .expect("modify_instance_groups should succeed");

    // Verify modification
    let list_resp2 = client
        .list_instance_groups()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("list_instance_groups should succeed after modify");

    let modified_group = list_resp2
        .instance_groups()
        .iter()
        .find(|g| g.id() == Some(ig_id.as_str()))
        .expect("should find modified group");
    assert_eq!(
        modified_group.requested_instance_count(),
        Some(4),
        "instance count should be updated to 4"
    );
}

// ---- Instance Fleet tests ----

#[tokio::test]
async fn test_add_list_modify_instance_fleets() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("fleet-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    // AddInstanceFleet
    let fleet_config = aws_sdk_emr::types::InstanceFleetConfig::builder()
        .instance_fleet_type(aws_sdk_emr::types::InstanceFleetType::Core)
        .target_on_demand_capacity(2)
        .target_spot_capacity(0)
        .build();

    let add_resp = client
        .add_instance_fleet()
        .cluster_id(cluster_id)
        .instance_fleet(fleet_config)
        .send()
        .await
        .expect("add_instance_fleet should succeed");

    let fleet_id = add_resp
        .instance_fleet_id()
        .expect("should have instance_fleet_id");
    assert!(
        fleet_id.starts_with("if-"),
        "instance fleet id should start with if-"
    );

    // ListInstanceFleets
    let list_resp = client
        .list_instance_fleets()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("list_instance_fleets should succeed");

    let fleets = list_resp.instance_fleets();
    assert!(
        fleets.iter().any(|f| f.id() == Some(fleet_id)),
        "listed fleets should contain the added fleet"
    );

    // ModifyInstanceFleet
    let modify_config = aws_sdk_emr::types::InstanceFleetModifyConfig::builder()
        .instance_fleet_id(fleet_id)
        .target_on_demand_capacity(5)
        .target_spot_capacity(3)
        .build();

    client
        .modify_instance_fleet()
        .cluster_id(cluster_id)
        .instance_fleet(modify_config)
        .send()
        .await
        .expect("modify_instance_fleet should succeed");
}

// ---- ModifyCluster tests ----

#[tokio::test]
async fn test_modify_cluster() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("modify-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    let modify_resp = client
        .modify_cluster()
        .cluster_id(cluster_id)
        .step_concurrency_level(5)
        .send()
        .await
        .expect("modify_cluster should succeed");

    assert_eq!(
        modify_resp.step_concurrency_level(),
        Some(5),
        "step_concurrency_level should be 5"
    );
}

// ---- DescribeJobFlows tests ----

#[tokio::test]
async fn test_describe_job_flows() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("djf-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    #[allow(deprecated)]
    let djf_resp = client
        .describe_job_flows()
        .job_flow_ids(cluster_id)
        .send()
        .await
        .expect("describe_job_flows should succeed");

    let flows = djf_resp.job_flows();
    assert_eq!(flows.len(), 1, "should have exactly 1 job flow");
    assert_eq!(flows[0].name(), Some("djf-cluster"));
}

// ---- ListInstances tests ----

#[tokio::test]
async fn test_list_instances() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("instances-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    let list_resp = client
        .list_instances()
        .cluster_id(cluster_id)
        .send()
        .await
        .expect("list_instances should succeed");

    // Simplified implementation returns empty list
    let instances = list_resp.instances();
    // Simplified implementation returns empty list; just verify the call succeeded
    let _ = instances;
}

// ---- ListReleaseLabels tests ----

#[tokio::test]
async fn test_list_release_labels() {
    let client = make_client().await;

    let resp = client
        .list_release_labels()
        .send()
        .await
        .expect("list_release_labels should succeed");

    let labels = resp.release_labels();
    assert!(!labels.is_empty(), "should have at least one release label");
    assert!(
        labels.iter().any(|l| l.starts_with("emr-")),
        "labels should start with emr-"
    );
}

// ---- ListSupportedInstanceTypes tests ----

#[tokio::test]
async fn test_list_supported_instance_types() {
    let client = make_client().await;

    let resp = client
        .list_supported_instance_types()
        .release_label("emr-6.10.0")
        .send()
        .await
        .expect("list_supported_instance_types should succeed");

    // Simplified implementation returns empty list
    let _types = resp.supported_instance_types();
}

// ---- AutoScalingPolicy tests ----

#[tokio::test]
async fn test_put_and_remove_auto_scaling_policy() {
    let client = make_client().await;

    let resp = client
        .run_job_flow()
        .name("autoscale-cluster")
        .instances(
            aws_sdk_emr::types::JobFlowInstancesConfig::builder()
                .master_instance_type("m5.xlarge")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("run_job_flow should succeed");

    let cluster_id = resp.job_flow_id().expect("should have job_flow_id");

    // Add an instance group first so we have an instance_group_id
    let ig_config = aws_sdk_emr::types::InstanceGroupConfig::builder()
        .instance_role(aws_sdk_emr::types::InstanceRoleType::Core)
        .instance_type("m5.xlarge")
        .instance_count(2)
        .build();

    let add_ig_resp = client
        .add_instance_groups()
        .job_flow_id(cluster_id)
        .instance_groups(ig_config)
        .send()
        .await
        .expect("add_instance_groups should succeed");

    let ig_id = &add_ig_resp.instance_group_ids()[0];

    // PutAutoScalingPolicy
    let scaling_rule = aws_sdk_emr::types::ScalingRule::builder()
        .name("scale-out")
        .action(
            aws_sdk_emr::types::ScalingAction::builder()
                .simple_scaling_policy_configuration(
                    aws_sdk_emr::types::SimpleScalingPolicyConfiguration::builder()
                        .scaling_adjustment(1)
                        .build(),
                )
                .build(),
        )
        .trigger(
            aws_sdk_emr::types::ScalingTrigger::builder()
                .cloud_watch_alarm_definition(
                    aws_sdk_emr::types::CloudWatchAlarmDefinition::builder()
                        .comparison_operator(
                            aws_sdk_emr::types::ComparisonOperator::GreaterThanOrEqual,
                        )
                        .metric_name("YARNMemoryAvailablePercentage")
                        .period(300)
                        .threshold(15.0)
                        .build(),
                )
                .build(),
        )
        .build();

    let policy = aws_sdk_emr::types::AutoScalingPolicy::builder()
        .constraints(
            aws_sdk_emr::types::ScalingConstraints::builder()
                .min_capacity(1)
                .max_capacity(10)
                .build(),
        )
        .rules(scaling_rule)
        .build();

    let put_resp = client
        .put_auto_scaling_policy()
        .cluster_id(cluster_id)
        .instance_group_id(ig_id)
        .auto_scaling_policy(policy)
        .send()
        .await
        .expect("put_auto_scaling_policy should succeed");

    assert_eq!(put_resp.cluster_id(), Some(cluster_id));

    // RemoveAutoScalingPolicy
    client
        .remove_auto_scaling_policy()
        .cluster_id(cluster_id)
        .instance_group_id(ig_id)
        .send()
        .await
        .expect("remove_auto_scaling_policy should succeed");
}

// ---- State view tests ----

#[tokio::test]
async fn test_snapshot_restore() {
    use chrono::Utc;
    use winterbaume_core::StatefulService;
    use winterbaume_emr::views::ClusterView;
    use winterbaume_emr::{EmrService, EmrStateView};

    // Build a view with a known cluster
    let mut view = EmrStateView::default();
    view.clusters.insert(
        "j-SNAP001".to_string(),
        ClusterView {
            id: "j-SNAP001".to_string(),
            name: "snapshot-cluster".to_string(),
            status: "WAITING".to_string(),
            creation_date_time: Utc::now().to_rfc3339(),
            ready_date_time: None,
            end_date_time: None,
            termination_protected: false,
            visible_to_all_users: true,
            log_uri: None,
            release_label: None,
            applications: vec![],
            tags: std::collections::HashMap::new(),
            service_role: None,
            job_flow_role: None,
            auto_scaling_role: None,
            scale_down_behavior: None,
            security_configuration: None,
            step_concurrency_level: None,
            auto_termination_policy: None,
            managed_scaling_policy: None,
            cluster_arn: "arn:aws:elasticmapreduce:us-east-1:000000000000:cluster/j-SNAP001"
                .to_string(),
            normalized_instance_hours: None,
            master_public_dns_name: None,
            instance_groups: vec![],
            instance_fleets: vec![],
            bootstrap_actions: vec![],
        },
    );

    let svc = EmrService::new();
    svc.restore("000000000000", "us-east-1", view)
        .await
        .expect("restore should succeed");

    let snapshot: EmrStateView = svc.snapshot("000000000000", "us-east-1").await;
    assert_eq!(snapshot.clusters.len(), 1);
    assert!(snapshot.clusters.contains_key("j-SNAP001"));
    assert_eq!(snapshot.clusters["j-SNAP001"].name, "snapshot-cluster");

    // Restore the snapshot into a new service
    let svc2 = EmrService::new();
    svc2.restore("000000000000", "us-east-1", snapshot.clone())
        .await
        .expect("second restore should succeed");

    let snapshot2: EmrStateView = svc2.snapshot("000000000000", "us-east-1").await;
    assert_eq!(snapshot2.clusters.len(), 1);
    assert!(snapshot2.clusters.contains_key("j-SNAP001"));
}

#[tokio::test]
async fn test_restore_and_merge() {
    use winterbaume_core::StatefulService;
    use winterbaume_emr::{EmrService, EmrStateView};

    // Build a view manually
    let mut view = EmrStateView::default();
    use chrono::Utc;
    use winterbaume_emr::views::ClusterView;
    view.clusters.insert(
        "j-TESTCLUSTER1".to_string(),
        ClusterView {
            id: "j-TESTCLUSTER1".to_string(),
            name: "test-restore-cluster".to_string(),
            status: "WAITING".to_string(),
            creation_date_time: Utc::now().to_rfc3339(),
            ready_date_time: None,
            end_date_time: None,
            termination_protected: false,
            visible_to_all_users: true,
            log_uri: None,
            release_label: None,
            applications: vec![],
            tags: std::collections::HashMap::new(),
            service_role: None,
            job_flow_role: None,
            auto_scaling_role: None,
            scale_down_behavior: None,
            security_configuration: None,
            step_concurrency_level: None,
            auto_termination_policy: None,
            managed_scaling_policy: None,
            cluster_arn: "arn:aws:elasticmapreduce:us-east-1:000000000000:cluster/j-TESTCLUSTER1"
                .to_string(),
            normalized_instance_hours: None,
            master_public_dns_name: None,
            instance_groups: vec![],
            instance_fleets: vec![],
            bootstrap_actions: vec![],
        },
    );

    let svc = EmrService::new();
    svc.restore("000000000000", "us-east-1", view.clone())
        .await
        .expect("restore should succeed");

    let snapshot = svc.snapshot("000000000000", "us-east-1").await;
    assert_eq!(snapshot.clusters.len(), 1);
    assert!(snapshot.clusters.contains_key("j-TESTCLUSTER1"));

    // Merge additional cluster
    let mut view2 = EmrStateView::default();
    use winterbaume_emr::views::ClusterView as CV;
    view2.clusters.insert(
        "j-TESTCLUSTER2".to_string(),
        CV {
            id: "j-TESTCLUSTER2".to_string(),
            name: "test-merge-cluster".to_string(),
            status: "WAITING".to_string(),
            creation_date_time: Utc::now().to_rfc3339(),
            ready_date_time: None,
            end_date_time: None,
            termination_protected: false,
            visible_to_all_users: true,
            log_uri: None,
            release_label: None,
            applications: vec![],
            tags: std::collections::HashMap::new(),
            service_role: None,
            job_flow_role: None,
            auto_scaling_role: None,
            scale_down_behavior: None,
            security_configuration: None,
            step_concurrency_level: None,
            auto_termination_policy: None,
            managed_scaling_policy: None,
            cluster_arn: "arn:aws:elasticmapreduce:us-east-1:000000000000:cluster/j-TESTCLUSTER2"
                .to_string(),
            normalized_instance_hours: None,
            master_public_dns_name: None,
            instance_groups: vec![],
            instance_fleets: vec![],
            bootstrap_actions: vec![],
        },
    );

    svc.merge("000000000000", "us-east-1", view2)
        .await
        .expect("merge should succeed");

    let snapshot2 = svc.snapshot("000000000000", "us-east-1").await;
    assert_eq!(
        snapshot2.clusters.len(),
        2,
        "merge should have added the new cluster without removing the existing one"
    );
    assert!(
        snapshot2.clusters.contains_key("j-TESTCLUSTER1"),
        "original cluster should still exist after merge"
    );
    assert!(
        snapshot2.clusters.contains_key("j-TESTCLUSTER2"),
        "merged cluster should exist"
    );
}

// ---- State change notification tests ----

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_emr::{EmrService, EmrStateView};

    let svc = EmrService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);

    svc.notifier()
        .subscribe(move |account_id, region, _view: &EmrStateView| {
            events2
                .lock()
                .unwrap()
                .push((account_id.to_string(), region.to_string()));
        });

    svc.restore("000000000000", "us-east-1", EmrStateView::default())
        .await
        .expect("restore should succeed");

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("000000000000".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_snapshot_reflects_mutation() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_emr::{EmrService, EmrStateView};

    let svc = EmrService::new();

    // Populate initial state via restore so we ignore that event
    svc.restore("000000000000", "us-east-1", EmrStateView::default())
        .await
        .expect("restore should succeed");

    let snapshots: Arc<Mutex<Vec<EmrStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);

    svc.notifier()
        .subscribe(move |_account_id, _region, view: &EmrStateView| {
            snapshots2.lock().unwrap().push(view.clone());
        });

    // Trigger a mutation via restore (since we can't share the service with MockAws without Clone)
    use chrono::Utc;
    use winterbaume_emr::views::ClusterView;
    let mut new_view = EmrStateView::default();
    new_view.clusters.insert(
        "j-NOTIFCLUSTER".to_string(),
        ClusterView {
            id: "j-NOTIFCLUSTER".to_string(),
            name: "notification-cluster".to_string(),
            status: "WAITING".to_string(),
            creation_date_time: Utc::now().to_rfc3339(),
            ready_date_time: None,
            end_date_time: None,
            termination_protected: false,
            visible_to_all_users: true,
            log_uri: None,
            release_label: None,
            applications: vec![],
            tags: std::collections::HashMap::new(),
            service_role: None,
            job_flow_role: None,
            auto_scaling_role: None,
            scale_down_behavior: None,
            security_configuration: None,
            step_concurrency_level: None,
            auto_termination_policy: None,
            managed_scaling_policy: None,
            cluster_arn: "arn:aws:elasticmapreduce:us-east-1:000000000000:cluster/j-NOTIFCLUSTER"
                .to_string(),
            normalized_instance_hours: None,
            master_public_dns_name: None,
            instance_groups: vec![],
            instance_fleets: vec![],
            bootstrap_actions: vec![],
        },
    );

    svc.restore("000000000000", "us-east-1", new_view)
        .await
        .expect("restore should succeed");

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1, "listener should have been called once");
    assert!(
        !got[0].clusters.is_empty(),
        "snapshot should reflect the new cluster"
    );
    assert!(
        got[0].clusters.contains_key("j-NOTIFCLUSTER"),
        "snapshot should contain the new cluster"
    );
}
