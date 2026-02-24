use aws_sdk_emrcontainers::config::BehaviorVersion;
use aws_sdk_emrcontainers::error::{ProvideErrorMetadata, SdkError};
use winterbaume_core::MockAws;
use winterbaume_emrcontainers::EmrContainersService;

async fn make_emrcontainers_client() -> aws_sdk_emrcontainers::Client {
    let mock = MockAws::builder()
        .with_service(EmrContainersService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emrcontainers::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_emrcontainers::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_virtual_cluster() {
    let client = make_emrcontainers_client().await;

    let container_provider = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("my-eks-cluster")
        .info(aws_sdk_emrcontainers::types::ContainerInfo::EksInfo(
            aws_sdk_emrcontainers::types::EksInfo::builder()
                .namespace("my-namespace")
                .build(),
        ))
        .build()
        .unwrap();

    let create_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(container_provider)
        .client_token("token-1")
        .send()
        .await
        .expect("create_virtual_cluster should succeed");

    let vc_id = create_resp.id().expect("id should be present");
    assert!(!vc_id.is_empty());
    assert_eq!(create_resp.name().unwrap(), "test-vc");
    assert!(create_resp.arn().is_some());

    let describe_resp = client
        .describe_virtual_cluster()
        .id(vc_id)
        .send()
        .await
        .expect("describe_virtual_cluster should succeed");

    let vc = describe_resp
        .virtual_cluster()
        .expect("virtual_cluster should be present");
    assert_eq!(vc.name().unwrap(), "test-vc");
    assert_eq!(
        vc.state().unwrap(),
        &aws_sdk_emrcontainers::types::VirtualClusterState::Running,
    );
    assert_eq!(vc.container_provider().unwrap().id(), "my-eks-cluster");
}

#[tokio::test]
async fn test_delete_virtual_cluster() {
    let client = make_emrcontainers_client().await;

    let container_provider = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("my-eks-cluster")
        .build()
        .unwrap();

    let create_resp = client
        .create_virtual_cluster()
        .name("to-delete")
        .container_provider(container_provider)
        .client_token("token-2")
        .send()
        .await
        .unwrap();

    let vc_id = create_resp.id().unwrap();

    let delete_resp = client
        .delete_virtual_cluster()
        .id(vc_id)
        .send()
        .await
        .expect("delete_virtual_cluster should succeed");

    assert_eq!(delete_resp.id().unwrap(), vc_id);
}

#[tokio::test]
async fn test_list_virtual_clusters() {
    let client = make_emrcontainers_client().await;

    let cp1 = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("cluster-1")
        .build()
        .unwrap();

    client
        .create_virtual_cluster()
        .name("vc-1")
        .container_provider(cp1)
        .client_token("token-3")
        .send()
        .await
        .unwrap();

    let cp2 = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("cluster-2")
        .build()
        .unwrap();

    client
        .create_virtual_cluster()
        .name("vc-2")
        .container_provider(cp2)
        .client_token("token-4")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_virtual_clusters()
        .send()
        .await
        .expect("list_virtual_clusters should succeed");

    assert_eq!(list_resp.virtual_clusters().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_virtual_cluster() {
    let client = make_emrcontainers_client().await;

    let result = client
        .describe_virtual_cluster()
        .id("nonexistent-id")
        .send()
        .await;

    assert!(result.is_err(), "describe nonexistent cluster should fail");
}

// --- Ported from moto test_emrcontainers.py ---

fn make_container_provider(
    cluster_id: &str,
    namespace: &str,
) -> aws_sdk_emrcontainers::types::ContainerProvider {
    aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id(cluster_id)
        .info(aws_sdk_emrcontainers::types::ContainerInfo::EksInfo(
            aws_sdk_emrcontainers::types::EksInfo::builder()
                .namespace(namespace)
                .build(),
        ))
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_virtual_cluster_checks_id_and_arn_format() {
    let client = make_emrcontainers_client().await;

    let cp = make_container_provider("test-eks-cluster", "emr-container");
    let resp = client
        .create_virtual_cluster()
        .name("test-emr-virtual-cluster")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");

    let vc_id = resp.id().expect("should have id");
    assert_eq!(vc_id.len(), 25, "virtual cluster id should be 25 chars");
    assert!(
        vc_id.chars().all(|c| c.is_ascii_alphanumeric()),
        "id should be alphanumeric"
    );

    let expected_arn =
        format!("arn:aws:emr-containers:us-east-1:123456789012:/virtualclusters/{vc_id}");
    assert_eq!(resp.arn().expect("should have arn"), expected_arn);
}

#[tokio::test]
async fn test_create_virtual_cluster_duplicate_namespace_fails() {
    let client = make_emrcontainers_client().await;

    let cp1 = make_container_provider("test-eks-cluster", "emr-container");
    client
        .create_virtual_cluster()
        .name("test-emr-virtual-cluster")
        .container_provider(cp1)
        .send()
        .await
        .expect("first create should succeed");

    let cp2 = make_container_provider("test-eks-cluster", "emr-container");
    let result = client
        .create_virtual_cluster()
        .name("test-emr-virtual-cluster")
        .container_provider(cp2)
        .send()
        .await;

    assert!(result.is_err(), "duplicate namespace should fail");
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(
            msg,
            "A virtual cluster already exists in the given namespace"
        );
    }
}

#[tokio::test]
async fn test_delete_virtual_cluster_non_existing_fails() {
    let client = make_emrcontainers_client().await;

    let result = client.delete_virtual_cluster().id("foobaa").send().await;

    assert!(result.is_err(), "deleting non-existing cluster should fail");
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, "VirtualCluster does not exist");
    }
}

#[tokio::test]
async fn test_describe_virtual_cluster_non_existing_error_message() {
    let client = make_emrcontainers_client().await;

    let result = client.describe_virtual_cluster().id("foobaa").send().await;

    assert!(result.is_err());
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, "Virtual cluster foobaa doesn't exist.");
    }
}

#[tokio::test]
async fn test_start_job_run_basic() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let create_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = create_resp.id().unwrap().to_string();

    let resp = client
        .start_job_run()
        .name("test_job")
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/iamrole-emrcontainers")
        .release_label("emr-6.3.0-latest")
        .send()
        .await
        .expect("start_job_run should succeed");

    let job_id = resp.id().expect("should have id");
    assert_eq!(job_id.len(), 19, "job run id should be 19 chars");
    assert_eq!(resp.name().unwrap(), "test_job");
    assert_eq!(resp.virtual_cluster_id().unwrap(), vc_id.as_str());

    let expected_arn = format!(
        "arn:aws:emr-containers:us-east-1:123456789012:/virtualclusters/{vc_id}/jobruns/{job_id}"
    );
    assert_eq!(resp.arn().unwrap(), expected_arn);
}

#[tokio::test]
async fn test_start_job_run_invalid_virtual_cluster() {
    let client = make_emrcontainers_client().await;

    let result = client
        .start_job_run()
        .name("test_job")
        .virtual_cluster_id("foobaa")
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-latest")
        .send()
        .await;

    assert!(result.is_err());
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, "Virtual cluster foobaa doesn't exist.");
    }
}

#[tokio::test]
async fn test_start_job_run_invalid_release_label() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let create_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = create_resp.id().unwrap().to_string();

    let result = client
        .start_job_run()
        .name("test_job")
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("foobaa")
        .send()
        .await;

    assert!(result.is_err());
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, "Release foobaa doesn't exist.");
    }
}

#[tokio::test]
async fn test_cancel_job_run_success() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    let jr_resp = client
        .start_job_run()
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-latest")
        .send()
        .await
        .expect("start_job_run should succeed");
    let jr_id = jr_resp.id().unwrap().to_string();

    let cancel_resp = client
        .cancel_job_run()
        .id(&jr_id)
        .virtual_cluster_id(&vc_id)
        .send()
        .await
        .expect("cancel_job_run should succeed");

    assert_eq!(cancel_resp.id().unwrap(), jr_id.as_str());
    assert_eq!(cancel_resp.virtual_cluster_id().unwrap(), vc_id.as_str());
}

#[tokio::test]
async fn test_cancel_job_run_invalid_id_fails() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    // "foobaa" is not 19 chars -> ValidationException "Invalid job run short id"
    let result = client
        .cancel_job_run()
        .id("foobaa")
        .virtual_cluster_id(&vc_id)
        .send()
        .await;

    assert!(result.is_err());
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, "Invalid job run short id");
    }
}

#[tokio::test]
async fn test_cancel_job_run_non_existing_id_fails() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    // 19-char ID that doesn't exist
    let fake_id = "123456789abcdefghij";
    let result = client
        .cancel_job_run()
        .id(fake_id)
        .virtual_cluster_id(&vc_id)
        .send()
        .await;

    assert!(result.is_err());
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, format!("Job run {fake_id} doesn't exist."));
    }
}

#[tokio::test]
async fn test_describe_job_run_basic() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    let jr_resp = client
        .start_job_run()
        .name("test_job")
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-latest")
        .send()
        .await
        .expect("start_job_run should succeed");
    let jr_id = jr_resp.id().unwrap().to_string();

    let describe_resp = client
        .describe_job_run()
        .id(&jr_id)
        .virtual_cluster_id(&vc_id)
        .send()
        .await
        .expect("describe_job_run should succeed");

    let jr = describe_resp.job_run().expect("should have job run");
    assert_eq!(jr.id().unwrap(), jr_id.as_str());
    assert_eq!(jr.virtual_cluster_id().unwrap(), vc_id.as_str());
    assert_eq!(jr.name().unwrap(), "test_job");
}

#[tokio::test]
async fn test_describe_job_run_invalid_id_fails() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    let result = client
        .describe_job_run()
        .id("foobaa")
        .virtual_cluster_id(&vc_id)
        .send()
        .await;

    assert!(result.is_err());
    if let Err(SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, "Invalid job run short id");
    }
}

#[tokio::test]
async fn test_list_job_runs_for_cluster() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    for i in 0..3 {
        client
            .start_job_run()
            .name(format!("job_{i}"))
            .virtual_cluster_id(&vc_id)
            .execution_role_arn("arn:aws:iam::123456789012:role/role")
            .release_label("emr-6.3.0-latest")
            .send()
            .await
            .expect("start_job_run should succeed");
    }

    let list_resp = client
        .list_job_runs()
        .virtual_cluster_id(&vc_id)
        .send()
        .await
        .expect("list_job_runs should succeed");

    assert_eq!(list_resp.job_runs().len(), 3);
}

#[tokio::test]
async fn test_list_job_runs_invalid_cluster_returns_empty() {
    let client = make_emrcontainers_client().await;

    let resp = client
        .list_job_runs()
        .virtual_cluster_id("foobaa")
        .send()
        .await
        .expect("list_job_runs for invalid cluster should return empty");

    assert_eq!(resp.job_runs().len(), 0);
}

#[tokio::test]
async fn test_list_virtual_clusters_empty() {
    let client = make_emrcontainers_client().await;

    let list_resp = client
        .list_virtual_clusters()
        .send()
        .await
        .expect("list_virtual_clusters with no clusters should succeed");

    assert_eq!(list_resp.virtual_clusters().len(), 0);
}

#[tokio::test]
async fn test_list_virtual_clusters_includes_terminated() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("eks-cluster")
        .build()
        .unwrap();

    let create_resp = client
        .create_virtual_cluster()
        .name("vc-to-terminate")
        .container_provider(cp)
        .client_token("token-term-1")
        .send()
        .await
        .unwrap();
    let vc_id = create_resp.id().unwrap().to_string();

    client
        .delete_virtual_cluster()
        .id(&vc_id)
        .send()
        .await
        .expect("delete_virtual_cluster should succeed");

    let list_resp = client
        .list_virtual_clusters()
        .send()
        .await
        .expect("list_virtual_clusters should succeed");

    // The terminated cluster should still appear in the list
    assert_eq!(list_resp.virtual_clusters().len(), 1);
    let vc = &list_resp.virtual_clusters()[0];
    assert_eq!(
        vc.state().unwrap(),
        &aws_sdk_emrcontainers::types::VirtualClusterState::Terminated,
    );
}

#[tokio::test]
async fn test_describe_job_run_non_existing_19char_id_fails() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    // 19-char alphanumeric id that doesn't correspond to any job run
    let fake_id = "1234567890123456789";
    let result = client
        .describe_job_run()
        .id(fake_id)
        .virtual_cluster_id(&vc_id)
        .send()
        .await;

    assert!(result.is_err());
    if let Err(aws_sdk_emrcontainers::error::SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, format!("Job run {fake_id} doesn't exist."));
    }
}

#[tokio::test]
async fn test_cancel_job_run_wrong_virtual_cluster_fails() {
    let client = make_emrcontainers_client().await;

    let cp1 = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("eks-cluster-1")
        .build()
        .unwrap();

    let vc1_resp = client
        .create_virtual_cluster()
        .name("vc-one")
        .container_provider(cp1)
        .send()
        .await
        .expect("create vc1 should succeed");
    let vc1_id = vc1_resp.id().unwrap().to_string();

    let cp2 = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("eks-cluster-2")
        .build()
        .unwrap();

    let vc2_resp = client
        .create_virtual_cluster()
        .name("vc-two")
        .container_provider(cp2)
        .send()
        .await
        .expect("create vc2 should succeed");
    let vc2_id = vc2_resp.id().unwrap().to_string();

    // Start a job run in vc1
    let jr_resp = client
        .start_job_run()
        .virtual_cluster_id(&vc1_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-latest")
        .send()
        .await
        .expect("start_job_run should succeed");
    let jr_id = jr_resp.id().unwrap().to_string();

    // Attempt to cancel it via vc2 — should fail
    let result = client
        .cancel_job_run()
        .id(&jr_id)
        .virtual_cluster_id(&vc2_id)
        .send()
        .await;

    assert!(
        result.is_err(),
        "cancel from wrong virtual cluster should fail"
    );
    if let Err(aws_sdk_emrcontainers::error::SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, format!("Job run {jr_id} doesn't exist."));
    }
}

#[tokio::test]
async fn test_list_job_runs_after_cancel_shows_cancelled_state() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    let jr_resp = client
        .start_job_run()
        .name("job-to-cancel")
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-latest")
        .send()
        .await
        .expect("start_job_run should succeed");
    let jr_id = jr_resp.id().unwrap().to_string();

    client
        .cancel_job_run()
        .id(&jr_id)
        .virtual_cluster_id(&vc_id)
        .send()
        .await
        .expect("cancel_job_run should succeed");

    let list_resp = client
        .list_job_runs()
        .virtual_cluster_id(&vc_id)
        .send()
        .await
        .expect("list_job_runs should succeed");

    assert_eq!(list_resp.job_runs().len(), 1);
    let jr = &list_resp.job_runs()[0];
    assert_eq!(
        jr.state().unwrap(),
        &aws_sdk_emrcontainers::types::JobRunState::Cancelled,
    );
    assert_eq!(jr.id().unwrap(), jr_id.as_str());
}

#[tokio::test]
async fn test_start_job_run_with_date_release_label() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    // Date-based release label (emr-X.Y.Z-YYYYMMDD) should be accepted
    let resp = client
        .start_job_run()
        .name("date-label-job")
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-20210726")
        .send()
        .await
        .expect("start_job_run with date release label should succeed");

    assert!(resp.id().is_some());
    assert_eq!(resp.virtual_cluster_id().unwrap(), vc_id.as_str());
}

#[tokio::test]
async fn test_start_job_run_without_name_succeeds() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    // name is optional for StartJobRun
    let resp = client
        .start_job_run()
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-latest")
        .send()
        .await
        .expect("start_job_run without name should succeed");

    let job_id = resp.id().expect("should have id");
    assert_eq!(job_id.len(), 19);
    assert!(
        resp.name().is_none(),
        "name should be absent when not provided"
    );
}

#[tokio::test]
async fn test_create_virtual_cluster_with_tags_round_trips() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("tagged-eks-cluster")
        .info(aws_sdk_emrcontainers::types::ContainerInfo::EksInfo(
            aws_sdk_emrcontainers::types::EksInfo::builder()
                .namespace("tagged-ns")
                .build(),
        ))
        .build()
        .unwrap();

    let create_resp = client
        .create_virtual_cluster()
        .name("tagged-vc")
        .container_provider(cp)
        .tags("env", "test")
        .tags("team", "data")
        .send()
        .await
        .expect("create_virtual_cluster with tags should succeed");

    let vc_id = create_resp.id().unwrap().to_string();

    let describe_resp = client
        .describe_virtual_cluster()
        .id(&vc_id)
        .send()
        .await
        .expect("describe_virtual_cluster should succeed");

    let vc = describe_resp
        .virtual_cluster()
        .expect("should have virtual cluster");
    let tags = vc.tags();
    assert_eq!(
        tags.as_ref().and_then(|t| t.get("env")).map(|s| s.as_str()),
        Some("test")
    );
    assert_eq!(
        tags.as_ref()
            .and_then(|t| t.get("team"))
            .map(|s| s.as_str()),
        Some("data")
    );
}

#[tokio::test]
async fn test_describe_job_run_wrong_virtual_cluster_fails() {
    let client = make_emrcontainers_client().await;

    let cp1 = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("eks-cluster-a")
        .build()
        .unwrap();

    let vc1_resp = client
        .create_virtual_cluster()
        .name("vc-alpha")
        .container_provider(cp1)
        .send()
        .await
        .expect("create vc1 should succeed");
    let vc1_id = vc1_resp.id().unwrap().to_string();

    let cp2 = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("eks-cluster-b")
        .build()
        .unwrap();

    let vc2_resp = client
        .create_virtual_cluster()
        .name("vc-beta")
        .container_provider(cp2)
        .send()
        .await
        .expect("create vc2 should succeed");
    let vc2_id = vc2_resp.id().unwrap().to_string();

    // Start a job run in vc1
    let jr_resp = client
        .start_job_run()
        .virtual_cluster_id(&vc1_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6.3.0-latest")
        .send()
        .await
        .expect("start_job_run should succeed");
    let jr_id = jr_resp.id().unwrap().to_string();

    // Describe it via vc2 — should fail
    let result = client
        .describe_job_run()
        .id(&jr_id)
        .virtual_cluster_id(&vc2_id)
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe from wrong virtual cluster should fail"
    );
    if let Err(aws_sdk_emrcontainers::error::SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, format!("Job run {jr_id} doesn't exist."));
    }
}

#[tokio::test]
async fn test_start_job_run_invalid_release_label_no_version_dots() {
    let client = make_emrcontainers_client().await;

    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();

    let vc_resp = client
        .create_virtual_cluster()
        .name("test-vc")
        .container_provider(cp)
        .send()
        .await
        .expect("create_virtual_cluster should succeed");
    let vc_id = vc_resp.id().unwrap().to_string();

    // emr- prefix but version has no dots — invalid
    let result = client
        .start_job_run()
        .virtual_cluster_id(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/role")
        .release_label("emr-6-latest")
        .send()
        .await;

    assert!(result.is_err());
    if let Err(aws_sdk_emrcontainers::error::SdkError::ServiceError(e)) = result {
        let msg = e.err().message().unwrap_or("");
        assert_eq!(msg, "Release emr-6-latest doesn't exist.");
    }
}

// ---- Managed Endpoint tests ----

async fn make_virtual_cluster(
    client: &aws_sdk_emrcontainers::Client,
    name: &str,
    token: &str,
) -> String {
    let cp = aws_sdk_emrcontainers::types::ContainerProvider::builder()
        .r#type(aws_sdk_emrcontainers::types::ContainerProviderType::Eks)
        .id("test-eks-cluster")
        .build()
        .unwrap();
    client
        .create_virtual_cluster()
        .name(name)
        .container_provider(cp)
        .client_token(token)
        .send()
        .await
        .unwrap()
        .id()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_create_and_describe_managed_endpoint() {
    let client = make_emrcontainers_client().await;
    let vc_id = make_virtual_cluster(&client, "vc-ep-test", "tok-ep-1").await;

    let create_resp = client
        .create_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .name("my-endpoint")
        .r#type("JUPYTER_ENTERPRISE_GATEWAY")
        .release_label("emr-6.5.0-latest")
        .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
        .client_token("ep-token-1")
        .send()
        .await
        .expect("create_managed_endpoint should succeed");

    let ep_id = create_resp.id().expect("id should be present");
    assert!(!ep_id.is_empty());
    assert_eq!(create_resp.name().unwrap(), "my-endpoint");
    assert!(create_resp.arn().is_some());
    assert_eq!(create_resp.virtual_cluster_id().unwrap(), &vc_id);

    let describe_resp = client
        .describe_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .id(ep_id)
        .send()
        .await
        .expect("describe_managed_endpoint should succeed");

    let ep = describe_resp
        .endpoint()
        .expect("endpoint should be present");
    assert_eq!(ep.name().unwrap(), "my-endpoint");
    assert_eq!(ep.virtual_cluster_id().unwrap(), &vc_id);
    assert_eq!(ep.r#type().unwrap(), "JUPYTER_ENTERPRISE_GATEWAY");
}

#[tokio::test]
async fn test_delete_managed_endpoint() {
    let client = make_emrcontainers_client().await;
    let vc_id = make_virtual_cluster(&client, "vc-ep-del", "tok-ep-2").await;

    let ep_id = client
        .create_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .name("ep-to-delete")
        .r#type("JUPYTER_ENTERPRISE_GATEWAY")
        .release_label("emr-6.5.0-latest")
        .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
        .client_token("ep-token-2")
        .send()
        .await
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let delete_resp = client
        .delete_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .id(&ep_id)
        .send()
        .await
        .expect("delete_managed_endpoint should succeed");

    assert_eq!(delete_resp.id().unwrap(), &ep_id);
    assert_eq!(delete_resp.virtual_cluster_id().unwrap(), &vc_id);

    // Verify it's gone
    let result = client
        .describe_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .id(&ep_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_managed_endpoints() {
    let client = make_emrcontainers_client().await;
    let vc_id = make_virtual_cluster(&client, "vc-ep-list", "tok-ep-3").await;

    for i in 0..3 {
        client
            .create_managed_endpoint()
            .virtual_cluster_id(&vc_id)
            .name(format!("ep-{i}"))
            .r#type("JUPYTER_ENTERPRISE_GATEWAY")
            .release_label("emr-6.5.0-latest")
            .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
            .client_token(format!("ep-tok-list-{i}"))
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_managed_endpoints()
        .virtual_cluster_id(&vc_id)
        .send()
        .await
        .expect("list_managed_endpoints should succeed");

    assert_eq!(list_resp.endpoints().len(), 3);
}

#[tokio::test]
async fn test_describe_nonexistent_managed_endpoint() {
    let client = make_emrcontainers_client().await;
    let vc_id = make_virtual_cluster(&client, "vc-ep-noexist", "tok-ep-4").await;

    let result = client
        .describe_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .id("nonexistent-endpoint")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_managed_endpoint_session_credentials() {
    let client = make_emrcontainers_client().await;
    let vc_id = make_virtual_cluster(&client, "vc-ep-creds", "tok-ep-5").await;

    let ep_id = client
        .create_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .name("ep-creds")
        .r#type("JUPYTER_ENTERPRISE_GATEWAY")
        .release_label("emr-6.5.0-latest")
        .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
        .client_token("ep-token-creds")
        .send()
        .await
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let creds_resp = client
        .get_managed_endpoint_session_credentials()
        .endpoint_identifier(&ep_id)
        .virtual_cluster_identifier(&vc_id)
        .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
        .credential_type("TOKEN")
        .client_token("creds-token-1")
        .send()
        .await
        .expect("get_managed_endpoint_session_credentials should succeed");

    assert!(creds_resp.id().is_some());
    let creds = creds_resp.credentials();
    assert!(creds.is_some());
    assert!(creds_resp.expires_at().is_some());
}

// ---- Job Template tests ----

#[tokio::test]
async fn test_create_and_describe_job_template() {
    let client = make_emrcontainers_client().await;

    let job_driver = aws_sdk_emrcontainers::types::JobDriver::builder()
        .spark_submit_job_driver(
            aws_sdk_emrcontainers::types::SparkSubmitJobDriver::builder()
                .entry_point("s3://mybucket/myscript.py")
                .build()
                .unwrap(),
        )
        .build();

    let job_template_data = aws_sdk_emrcontainers::types::JobTemplateData::builder()
        .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
        .release_label("emr-6.5.0-latest")
        .job_driver(job_driver)
        .build()
        .unwrap();

    let create_resp = client
        .create_job_template()
        .name("my-job-template")
        .job_template_data(job_template_data)
        .client_token("jt-token-1")
        .send()
        .await
        .expect("create_job_template should succeed");

    let jt_id = create_resp.id().expect("id should be present");
    assert!(!jt_id.is_empty());
    assert_eq!(create_resp.name().unwrap(), "my-job-template");
    assert!(create_resp.arn().is_some());
    assert!(create_resp.created_at().is_some());

    let describe_resp = client
        .describe_job_template()
        .id(jt_id)
        .send()
        .await
        .expect("describe_job_template should succeed");

    let jt = describe_resp
        .job_template()
        .expect("job_template should be present");
    assert_eq!(jt.name().unwrap(), "my-job-template");
}

#[tokio::test]
async fn test_delete_job_template() {
    let client = make_emrcontainers_client().await;

    let job_driver = aws_sdk_emrcontainers::types::JobDriver::builder()
        .spark_submit_job_driver(
            aws_sdk_emrcontainers::types::SparkSubmitJobDriver::builder()
                .entry_point("s3://mybucket/script.py")
                .build()
                .unwrap(),
        )
        .build();

    let job_template_data = aws_sdk_emrcontainers::types::JobTemplateData::builder()
        .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
        .release_label("emr-6.5.0-latest")
        .job_driver(job_driver)
        .build()
        .unwrap();

    let jt_id = client
        .create_job_template()
        .name("to-delete-template")
        .job_template_data(job_template_data)
        .client_token("jt-token-2")
        .send()
        .await
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let delete_resp = client
        .delete_job_template()
        .id(&jt_id)
        .send()
        .await
        .expect("delete_job_template should succeed");

    assert_eq!(delete_resp.id().unwrap(), &jt_id);

    // Verify it's gone
    let result = client.describe_job_template().id(&jt_id).send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_job_templates() {
    let client = make_emrcontainers_client().await;

    for i in 0..3 {
        let job_driver = aws_sdk_emrcontainers::types::JobDriver::builder()
            .spark_submit_job_driver(
                aws_sdk_emrcontainers::types::SparkSubmitJobDriver::builder()
                    .entry_point(format!("s3://bucket/script{i}.py"))
                    .build()
                    .unwrap(),
            )
            .build();
        let data = aws_sdk_emrcontainers::types::JobTemplateData::builder()
            .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
            .release_label("emr-6.5.0-latest")
            .job_driver(job_driver)
            .build()
            .unwrap();
        client
            .create_job_template()
            .name(format!("template-{i}"))
            .job_template_data(data)
            .client_token(format!("jt-tok-{i}"))
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_job_templates()
        .send()
        .await
        .expect("list_job_templates should succeed");

    assert_eq!(list_resp.templates().len(), 3);
}

// ---- Security Configuration tests ----

#[tokio::test]
async fn test_create_and_describe_security_configuration() {
    let client = make_emrcontainers_client().await;

    let security_configuration_data =
        aws_sdk_emrcontainers::types::SecurityConfigurationData::builder().build();

    let create_resp = client
        .create_security_configuration()
        .name("my-security-config")
        .security_configuration_data(security_configuration_data)
        .client_token("sc-token-1")
        .send()
        .await
        .expect("create_security_configuration should succeed");

    let sc_id = create_resp.id().expect("id should be present");
    assert!(!sc_id.is_empty());
    assert_eq!(create_resp.name().unwrap(), "my-security-config");
    assert!(create_resp.arn().is_some());

    let describe_resp = client
        .describe_security_configuration()
        .id(sc_id)
        .send()
        .await
        .expect("describe_security_configuration should succeed");

    let sc = describe_resp
        .security_configuration()
        .expect("security_configuration should be present");
    assert_eq!(sc.name().unwrap(), "my-security-config");
}

#[tokio::test]
async fn test_list_security_configurations() {
    let client = make_emrcontainers_client().await;

    for i in 0..2 {
        let sc_data = aws_sdk_emrcontainers::types::SecurityConfigurationData::builder().build();
        client
            .create_security_configuration()
            .name(format!("sc-{i}"))
            .security_configuration_data(sc_data)
            .client_token(format!("sc-tok-{i}"))
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_security_configurations()
        .send()
        .await
        .expect("list_security_configurations should succeed");

    assert_eq!(list_resp.security_configurations().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_security_configuration() {
    let client = make_emrcontainers_client().await;

    let result = client
        .describe_security_configuration()
        .id("nonexistent-sc")
        .send()
        .await;

    assert!(result.is_err());
}

// ---- Tag tests ----

#[tokio::test]
async fn test_tag_resource_list_and_untag() {
    let client = make_emrcontainers_client().await;
    let vc_id = make_virtual_cluster(&client, "vc-tags", "tok-tags-1").await;

    // Get the ARN via describe
    let describe_resp = client
        .describe_virtual_cluster()
        .id(&vc_id)
        .send()
        .await
        .unwrap();
    let vc_arn = describe_resp
        .virtual_cluster()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    // TagResource
    client
        .tag_resource()
        .resource_arn(&vc_arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // ListTagsForResource
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&vc_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));

    // UntagResource
    client
        .untag_resource()
        .resource_arn(&vc_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&vc_arn)
        .send()
        .await
        .unwrap();

    let tags2 = list_resp2.tags().unwrap();
    assert!(tags2.contains_key("env"));
    assert!(!tags2.contains_key("team"));
}

#[tokio::test]
async fn test_list_tags_for_nonexistent_resource() {
    let client = make_emrcontainers_client().await;

    let result = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:emr-containers:us-east-1:123456789012:/virtualclusters/nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_tags_on_managed_endpoint() {
    let client = make_emrcontainers_client().await;
    let vc_id = make_virtual_cluster(&client, "vc-ep-tags", "tok-ep-tags-1").await;

    let ep_create_resp = client
        .create_managed_endpoint()
        .virtual_cluster_id(&vc_id)
        .name("ep-for-tags")
        .r#type("JUPYTER_ENTERPRISE_GATEWAY")
        .release_label("emr-6.5.0-latest")
        .execution_role_arn("arn:aws:iam::123456789012:role/EMRContainersRole")
        .client_token("ep-tok-for-tags")
        .send()
        .await
        .unwrap();

    let ep_arn = ep_create_resp.arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&ep_arn)
        .tags("purpose", "ml-workload")
        .send()
        .await
        .expect("tag_resource on endpoint should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&ep_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        list_resp.tags().unwrap().get("purpose").map(|s| s.as_str()),
        Some("ml-workload")
    );
}
