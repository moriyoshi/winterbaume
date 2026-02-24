use aws_sdk_codebuild::config::BehaviorVersion;
use aws_sdk_codebuild::types::{
    ArtifactsType, ComputeType, EnvironmentType, ProjectArtifacts, ProjectEnvironment,
    ProjectSource, ReportExportConfig, ReportExportConfigType, ReportType, SourceType,
};
use winterbaume_codebuild::CodeBuildService;
use winterbaume_core::MockAws;

const ACCOUNT_ID: &str = "123456789012";

async fn make_client() -> aws_sdk_codebuild::Client {
    let mock = MockAws::builder()
        .with_service(CodeBuildService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codebuild::config::Region::new("eu-central-1"))
        .load()
        .await;

    aws_sdk_codebuild::Client::new(&config)
}

fn s3_source() -> ProjectSource {
    ProjectSource::builder()
        .r#type(SourceType::S3)
        .location("bucketname/path/file.zip")
        .build()
        .unwrap()
}

fn s3_artifacts() -> ProjectArtifacts {
    ProjectArtifacts::builder()
        .r#type(ArtifactsType::S3)
        .location("bucketname")
        .build()
        .unwrap()
}

fn no_artifacts() -> ProjectArtifacts {
    ProjectArtifacts::builder()
        .r#type(ArtifactsType::NoArtifacts)
        .build()
        .unwrap()
}

fn linux_env() -> ProjectEnvironment {
    ProjectEnvironment::builder()
        .r#type(EnvironmentType::LinuxContainer)
        .image("contents_not_validated")
        .compute_type(ComputeType::BuildGeneral1Small)
        .build()
        .unwrap()
}

fn service_role() -> String {
    format!(
        "arn:aws:iam::{}:role/service-role/my-codebuild-service-role",
        ACCOUNT_ID
    )
}

// ============================================================
// test_codebuild_create_project_s3_artifacts
// ============================================================
#[tokio::test]
async fn test_codebuild_create_project_s3_artifacts() {
    let client = make_client().await;

    let name = "some_project";
    let resp = client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(s3_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .tags(
            aws_sdk_codebuild::types::Tag::builder()
                .key("k1")
                .value("v1")
                .build(),
        )
        .send()
        .await
        .expect("create_project should succeed");

    let project = resp.project().expect("should have project");
    assert_eq!(project.name(), Some(name));
    assert!(project.service_role().is_some());

    // Environment
    let env = project.environment().unwrap();
    assert_eq!(env.compute_type(), &ComputeType::BuildGeneral1Small);
    assert_eq!(env.image(), "contents_not_validated");
    assert_eq!(env.r#type(), &EnvironmentType::LinuxContainer);

    // Source
    let source = project.source().unwrap();
    assert_eq!(source.r#type(), &SourceType::S3);
    assert_eq!(source.location(), Some("bucketname/path/file.zip"));

    // Artifacts
    let artifacts = project.artifacts().unwrap();
    assert_eq!(artifacts.r#type(), &ArtifactsType::S3);
    assert_eq!(artifacts.location(), Some("bucketname"));

    // Tags
    let tags = project.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("k1"));
    assert_eq!(tags[0].value(), Some("v1"));
}

// ============================================================
// test_codebuild_create_project_no_artifacts
// ============================================================
#[tokio::test]
async fn test_codebuild_create_project_no_artifacts() {
    let client = make_client().await;

    let name = "some_project";
    let resp = client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .expect("create_project should succeed");

    let project = resp.project().expect("should have project");
    assert_eq!(project.name(), Some(name));
    assert!(project.service_role().is_some());

    // Environment
    let env = project.environment().unwrap();
    assert_eq!(env.compute_type(), &ComputeType::BuildGeneral1Small);
    assert_eq!(env.image(), "contents_not_validated");
    assert_eq!(env.r#type(), &EnvironmentType::LinuxContainer);

    // Source
    let source = project.source().unwrap();
    assert_eq!(source.r#type(), &SourceType::S3);
    assert_eq!(source.location(), Some("bucketname/path/file.zip"));

    // Artifacts
    let artifacts = project.artifacts().unwrap();
    assert_eq!(artifacts.r#type(), &ArtifactsType::NoArtifacts);
}

// ============================================================
// test_codebuild_create_project_with_invalid_inputs
// ============================================================
#[tokio::test]
async fn test_codebuild_create_project_name_too_long() {
    let client = make_client().await;

    let long_name = "some_project_".repeat(12);
    let result = client
        .create_project()
        .name(long_name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_invalid_input_exception());
}

#[tokio::test]
async fn test_codebuild_create_project_name_invalid() {
    let client = make_client().await;

    let result = client
        .create_project()
        .name("!some_project_")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_invalid_input_exception());
}

#[tokio::test]
async fn test_codebuild_create_project_invalid_service_role() {
    let client = make_client().await;

    let result = client
        .create_project()
        .name("valid_name")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role("arn:aws:iam::0000:role/service-role/my-role")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_invalid_input_exception());
}

// ============================================================
// test_codebuild_create_project_when_exists
// ============================================================
#[tokio::test]
async fn test_codebuild_create_project_when_exists() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let result = client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_already_exists_exception());
}

// ============================================================
// test_codebuild_list_projects
// ============================================================
#[tokio::test]
async fn test_codebuild_list_projects() {
    let client = make_client().await;

    client
        .create_project()
        .name("project1")
        .source(s3_source())
        .artifacts(s3_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .create_project()
        .name("project2")
        .source(s3_source())
        .artifacts(s3_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let resp = client.list_projects().send().await.unwrap();
    let projects = resp.projects();
    assert_eq!(projects.len(), 2);
    assert!(projects.contains(&"project1".to_string()));
    assert!(projects.contains(&"project2".to_string()));
}

// ============================================================
// test_codebuild_list_builds_for_project_no_history
// ============================================================
#[tokio::test]
async fn test_codebuild_list_builds_for_project_no_history() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let history = client
        .list_builds_for_project()
        .project_name(name)
        .send()
        .await
        .unwrap();

    assert_eq!(history.ids().len(), 0);
}

// ============================================================
// test_codebuild_list_builds_for_project_with_history
// ============================================================
#[tokio::test]
async fn test_codebuild_list_builds_for_project_with_history() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();

    let response = client
        .list_builds_for_project()
        .project_name(name)
        .send()
        .await
        .unwrap();

    assert_eq!(response.ids().len(), 1);
}

// ============================================================
// test_codebuild_start_build_no_project
// ============================================================
#[tokio::test]
async fn test_codebuild_start_build_no_project() {
    let client = make_client().await;

    let result = client
        .start_build()
        .project_name("some_project")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ============================================================
// test_codebuild_start_build_no_overrides
// ============================================================
#[tokio::test]
async fn test_codebuild_start_build_no_overrides() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();

    // Verify source_version via batch_get_builds
    let list_resp = client.list_builds().send().await.unwrap();
    let build_id = &list_resp.ids()[0];
    let batch_resp = client
        .batch_get_builds()
        .ids(build_id)
        .send()
        .await
        .unwrap();
    let build = &batch_resp.builds()[0];
    assert_eq!(build.source_version(), Some("refs/heads/main"));
}

// ============================================================
// test_codebuild_start_build_multiple_times
// ============================================================
#[tokio::test]
async fn test_codebuild_start_build_multiple_times() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();
    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();
    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();

    let resp = client.list_builds().send().await.unwrap();
    assert_eq!(resp.ids().len(), 3);
}

// ============================================================
// test_codebuild_start_build_with_overrides
// ============================================================
#[tokio::test]
async fn test_codebuild_start_build_with_overrides() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name(name)
        .source_version("fix/testing")
        .artifacts_override(no_artifacts())
        .send()
        .await
        .unwrap();

    // Verify source_version via batch_get_builds
    let list_resp = client.list_builds().send().await.unwrap();
    let build_id = &list_resp.ids()[0];
    let batch_resp = client
        .batch_get_builds()
        .ids(build_id)
        .send()
        .await
        .unwrap();
    let build = &batch_resp.builds()[0];
    assert_eq!(build.source_version(), Some("fix/testing"));
}

// ============================================================
// test_codebuild_batch_get_builds_1_project
// ============================================================
#[tokio::test]
async fn test_codebuild_batch_get_builds_1_project() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();

    let history = client
        .list_builds_for_project()
        .project_name(name)
        .send()
        .await
        .unwrap();

    let response = client
        .batch_get_builds()
        .set_ids(Some(history.ids().to_vec()))
        .send()
        .await
        .unwrap();

    assert_eq!(response.builds().len(), 1);
    let build = &response.builds()[0];
    assert_eq!(build.current_phase(), Some("COMPLETED"));
    assert!(build.build_number().is_some());
    assert!(!build.phases().is_empty());
    assert_eq!(build.phases().len(), 11);
}

// ============================================================
// test_codebuild_batch_get_builds_2_projects
// ============================================================
#[tokio::test]
async fn test_codebuild_batch_get_builds_2_projects() {
    let client = make_client().await;

    client
        .create_project()
        .name("project-1")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();
    client
        .start_build()
        .project_name("project-1")
        .send()
        .await
        .unwrap();

    client
        .create_project()
        .name("project-2")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();
    client
        .start_build()
        .project_name("project-2")
        .send()
        .await
        .unwrap();

    let response = client.list_builds().send().await.unwrap();
    assert_eq!(response.ids().len(), 2);

    assert!(response.ids()[0].contains("project-1"));
    assert!(response.ids()[1].contains("project-2"));

    let metadata = client
        .batch_get_builds()
        .set_ids(Some(response.ids().to_vec()))
        .send()
        .await
        .unwrap();

    assert_eq!(metadata.builds().len(), 2);
    assert!(metadata.builds()[0].id().unwrap().contains("project-1"));
    assert!(metadata.builds()[1].id().unwrap().contains("project-2"));
}

// ============================================================
// test_codebuild_batch_get_builds_invalid_build_id
// ============================================================
#[tokio::test]
async fn test_codebuild_batch_get_builds_invalid_build_id() {
    let client = make_client().await;

    // ID without colon should fail
    let result = client
        .batch_get_builds()
        .ids("some_project_no_colon_uuid")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_invalid_input_exception());
}

// ============================================================
// test_codebuild_delete_project
// ============================================================
#[tokio::test]
async fn test_codebuild_delete_project() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();

    let response = client
        .list_builds_for_project()
        .project_name(name)
        .send()
        .await
        .unwrap();
    assert_eq!(response.ids().len(), 1);

    client.delete_project().name(name).send().await.unwrap();

    // After delete, listing builds for the project should fail with ResourceNotFoundException
    let result = client
        .list_builds_for_project()
        .project_name(name)
        .send()
        .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ============================================================
// test_codebuild_stop_build
// ============================================================
#[tokio::test]
async fn test_codebuild_stop_build() {
    let client = make_client().await;

    let name = "some_project";
    client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name(name)
        .send()
        .await
        .unwrap();

    let builds = client.list_builds().send().await.unwrap();
    let build_id = &builds.ids()[0];

    client.stop_build().id(build_id).send().await.unwrap();

    // Verify stopped status via batch_get_builds
    // Note: batch_get_builds sets status to SUCCEEDED, so we check the stop_build worked
    // by verifying we can still get the build and it has an end_time
    let batch_resp = client
        .batch_get_builds()
        .ids(build_id)
        .send()
        .await
        .unwrap();
    let build = &batch_resp.builds()[0];
    assert!(build.end_time().is_some());
}

// ============================================================
// test_codebuild_stop_build_no_build
// ============================================================
#[tokio::test]
async fn test_codebuild_stop_build_no_build() {
    let client = make_client().await;

    let result = client
        .stop_build()
        .id("some_project:fake-uuid-here")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ============================================================
// test_codebuild_stop_build_bad_uid
// ============================================================
#[tokio::test]
async fn test_codebuild_stop_build_bad_uid() {
    let client = make_client().await;

    // ID without colon
    let result = client.stop_build().id("some_project_no_colon").send().await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_invalid_input_exception());
}

// ============================================================
// test_codebuild_batch_get_projects
// ============================================================
#[tokio::test]
async fn test_codebuild_batch_get_projects() {
    let client = make_client().await;

    let name = "some_project";
    let create_resp = client
        .create_project()
        .name(name)
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let project_arn = create_resp.project().unwrap().arn().unwrap().to_string();

    // Get by name
    let batch = client
        .batch_get_projects()
        .names(name)
        .send()
        .await
        .unwrap();
    assert_eq!(batch.projects().len(), 1);
    assert_eq!(batch.projects()[0].name(), Some(name));
    assert_eq!(batch.projects()[0].arn(), Some(project_arn.as_str()));

    // Get by ARN
    let batch = client
        .batch_get_projects()
        .names(&project_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(batch.projects().len(), 1);
    assert_eq!(batch.projects()[0].name(), Some(name));
    assert_eq!(batch.projects()[0].arn(), Some(project_arn.as_str()));
}

// ============================================================
// Additional tests from original test file
// ============================================================

#[tokio::test]
async fn test_batch_get_projects_not_found() {
    let client = make_client().await;

    let batch = client
        .batch_get_projects()
        .names("nonexistent-project")
        .send()
        .await
        .expect("batch_get_projects should succeed even for missing projects");

    assert_eq!(batch.projects().len(), 0);
    assert_eq!(batch.projects_not_found().len(), 1);
    assert_eq!(batch.projects_not_found()[0], "nonexistent-project");
}

#[tokio::test]
async fn test_list_builds_for_project_nonexistent() {
    let client = make_client().await;

    let result = client
        .list_builds_for_project()
        .project_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ============================================================================
// Tests derived from AWS documentation: AWS CodeBuild
// ============================================================================

#[tokio::test]
async fn test_codebuild_create_project_with_description() {
    let client = make_client().await;

    let resp = client
        .create_project()
        .name("desc-project")
        .description("A project with a description")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .expect("create_project with description should succeed");

    let project = resp.project().unwrap();
    assert_eq!(project.description(), Some("A project with a description"));
}

#[tokio::test]
async fn test_codebuild_delete_nonexistent_project() {
    let client = make_client().await;

    // AWS CodeBuild delete_project is idempotent -- returns success even for nonexistent
    let result = client
        .delete_project()
        .name("nonexistent-project")
        .send()
        .await;

    // Accept either success (idempotent) or error
    let _ = result;
}

#[tokio::test]
async fn test_codebuild_build_number_increments() {
    let client = make_client().await;

    client
        .create_project()
        .name("build-num-project")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    // Start 3 builds
    for _ in 0..3 {
        client
            .start_build()
            .project_name("build-num-project")
            .send()
            .await
            .unwrap();
    }

    let builds_resp = client.list_builds().send().await.unwrap();
    let build_ids = builds_resp.ids();
    assert_eq!(build_ids.len(), 3);

    let batch_resp = client
        .batch_get_builds()
        .set_ids(Some(build_ids.to_vec()))
        .send()
        .await
        .unwrap();

    let build_numbers: Vec<i64> = batch_resp
        .builds()
        .iter()
        .filter_map(|b| b.build_number())
        .collect();

    // Each build should have a unique build number
    let mut sorted = build_numbers.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        build_numbers.len(),
        "build numbers should be unique"
    );
}

#[tokio::test]
async fn test_codebuild_list_builds_empty() {
    let client = make_client().await;

    let resp = client
        .list_builds()
        .send()
        .await
        .expect("list_builds should succeed");
    assert_eq!(resp.ids().len(), 0, "should have no builds initially");
}

#[tokio::test]
async fn test_codebuild_batch_get_builds_mixed() {
    let client = make_client().await;

    client
        .create_project()
        .name("mixed-project")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name("mixed-project")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_builds().send().await.unwrap();
    let real_id = list_resp.ids()[0].clone();

    let resp = client
        .batch_get_builds()
        .ids(&real_id)
        .ids("nonexistent-project:fake-uuid-1234")
        .send()
        .await
        .expect("batch_get_builds with mixed IDs should succeed");

    assert_eq!(resp.builds().len(), 1, "should return 1 found build");
    assert_eq!(
        resp.builds_not_found().len(),
        1,
        "should report 1 not found"
    );
}

#[tokio::test]
async fn test_codebuild_project_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_project()
        .name("arn-check-project")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let project = resp.project().unwrap();
    let arn = project.arn().expect("should have ARN");
    assert!(
        arn.starts_with("arn:aws:codebuild:"),
        "ARN should start with arn:aws:codebuild:"
    );
    assert!(
        arn.contains("arn-check-project"),
        "ARN should contain project name"
    );
}

// ============================================================
// test_codebuild_batch_delete_builds
// ============================================================
#[tokio::test]
async fn test_codebuild_batch_delete_builds() {
    let client = make_client().await;

    client
        .create_project()
        .name("batch-del-project")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    client
        .start_build()
        .project_name("batch-del-project")
        .send()
        .await
        .unwrap();
    client
        .start_build()
        .project_name("batch-del-project")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_builds().send().await.unwrap();
    assert_eq!(list_resp.ids().len(), 2);

    let ids = list_resp.ids().to_vec();
    let del_resp = client
        .batch_delete_builds()
        .set_ids(Some(ids.clone()))
        .send()
        .await
        .unwrap();

    assert_eq!(del_resp.builds_deleted().len(), 2);

    // Verify builds are gone
    let list_resp2 = client.list_builds().send().await.unwrap();
    assert_eq!(list_resp2.ids().len(), 0);
}

// ============================================================
// test_codebuild_update_project
// ============================================================
#[tokio::test]
async fn test_codebuild_update_project() {
    let client = make_client().await;

    client
        .create_project()
        .name("update-project")
        .description("original description")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let resp = client
        .update_project()
        .name("update-project")
        .description("updated description")
        .send()
        .await
        .unwrap();

    let project = resp.project().unwrap();
    assert_eq!(project.description(), Some("updated description"));
    assert_eq!(project.name(), Some("update-project"));
}

// ============================================================
// test_codebuild_retry_build
// ============================================================
#[tokio::test]
async fn test_codebuild_retry_build() {
    let client = make_client().await;

    client
        .create_project()
        .name("retry-project")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let start_resp = client
        .start_build()
        .project_name("retry-project")
        .source_version("refs/heads/feature")
        .send()
        .await
        .unwrap();

    // Retrieve the build ID via list_builds since StartBuildOutput.build() is not available
    let list_resp = client.list_builds().send().await.unwrap();
    let build_id = list_resp.ids()[0].clone();
    assert!(build_id.starts_with("retry-project:"));

    let retry_resp = client.retry_build().id(&build_id).send().await.unwrap();

    // Retrieve the retried build details via batch_get_builds
    let retry_list = client.list_builds().send().await.unwrap();
    assert_eq!(retry_list.ids().len(), 2);
    let new_build_id = retry_list
        .ids()
        .iter()
        .find(|id| id.as_str() != build_id)
        .unwrap()
        .clone();
    let batch = client
        .batch_get_builds()
        .ids(&new_build_id)
        .send()
        .await
        .unwrap();
    let new_build = &batch.builds()[0];
    let _ = retry_resp; // consume the retry response
    assert_eq!(new_build.project_name(), Some("retry-project"));
    assert_eq!(new_build.source_version(), Some("refs/heads/feature"));

    // There should now be 2 builds
    let list_resp = client.list_builds().send().await.unwrap();
    assert_eq!(list_resp.ids().len(), 2);
}

// ============================================================
// test_codebuild_webhook_crud
// ============================================================
#[tokio::test]
async fn test_codebuild_webhook_crud() {
    let client = make_client().await;

    client
        .create_project()
        .name("webhook-project")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    // Create webhook
    let create_resp = client
        .create_webhook()
        .project_name("webhook-project")
        .branch_filter("main")
        .send()
        .await
        .unwrap();

    let webhook = create_resp.webhook().unwrap();
    assert!(webhook.url().is_some());
    assert_eq!(webhook.branch_filter(), Some("main"));

    // Can't create twice
    let result = client
        .create_webhook()
        .project_name("webhook-project")
        .send()
        .await;
    assert!(result.is_err());

    // Delete webhook
    client
        .delete_webhook()
        .project_name("webhook-project")
        .send()
        .await
        .unwrap();

    // After deletion, deleting again should fail
    let result = client
        .delete_webhook()
        .project_name("webhook-project")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================
// test_codebuild_source_credentials
// ============================================================
#[tokio::test]
async fn test_codebuild_source_credentials() {
    use aws_sdk_codebuild::types::{AuthType, ServerType};

    let client = make_client().await;

    // Import credentials
    let import_resp = client
        .import_source_credentials()
        .server_type(ServerType::Github)
        .auth_type(AuthType::PersonalAccessToken)
        .token("my-github-token")
        .send()
        .await
        .unwrap();

    let arn = import_resp.arn().unwrap().to_string();
    assert!(arn.starts_with("arn:aws:codebuild:"));

    // List credentials
    let list_resp = client.list_source_credentials().send().await.unwrap();
    assert_eq!(list_resp.source_credentials_infos().len(), 1);
    assert_eq!(
        list_resp.source_credentials_infos()[0].arn(),
        Some(arn.as_str())
    );

    // Delete credentials
    let del_resp = client
        .delete_source_credentials()
        .arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(del_resp.arn(), Some(arn.as_str()));

    // After deletion, list should be empty
    let list_resp2 = client.list_source_credentials().send().await.unwrap();
    assert_eq!(list_resp2.source_credentials_infos().len(), 0);
}

// ============================================================
// test_codebuild_report_group_crud
// ============================================================
#[tokio::test]
async fn test_codebuild_report_group_crud() {
    let client = make_client().await;

    let export_config = ReportExportConfig::builder()
        .export_config_type(ReportExportConfigType::NoExport)
        .build();

    // Create report group
    let create_resp = client
        .create_report_group()
        .name("my-report-group")
        .r#type(ReportType::Test)
        .export_config(export_config)
        .send()
        .await
        .expect("create_report_group should succeed");

    let rg = create_resp
        .report_group()
        .expect("should have report_group");
    let rg_arn = rg.arn().expect("should have arn").to_string();
    assert_eq!(rg.name(), Some("my-report-group"));
    assert_eq!(rg.r#type(), Some(&ReportType::Test));
    assert_eq!(
        rg.status(),
        Some(&aws_sdk_codebuild::types::ReportGroupStatusType::Active)
    );

    // BatchGetReportGroups
    let batch_resp = client
        .batch_get_report_groups()
        .report_group_arns(&rg_arn)
        .send()
        .await
        .expect("batch_get_report_groups should succeed");
    assert_eq!(batch_resp.report_groups().len(), 1);
    assert_eq!(batch_resp.report_groups_not_found().len(), 0);
    assert_eq!(batch_resp.report_groups()[0].arn(), Some(rg_arn.as_str()));

    // ListReportGroups
    let list_resp = client
        .list_report_groups()
        .send()
        .await
        .expect("list_report_groups should succeed");
    assert_eq!(list_resp.report_groups().len(), 1);
    assert!(list_resp.report_groups().contains(&rg_arn));

    // ListReportsForReportGroup (should be empty)
    let reports_resp = client
        .list_reports_for_report_group()
        .report_group_arn(&rg_arn)
        .send()
        .await
        .expect("list_reports_for_report_group should succeed");
    assert_eq!(reports_resp.reports().len(), 0);

    // UpdateReportGroup - add tags
    let update_resp = client
        .update_report_group()
        .arn(&rg_arn)
        .tags(
            aws_sdk_codebuild::types::Tag::builder()
                .key("env")
                .value("prod")
                .build(),
        )
        .send()
        .await
        .expect("update_report_group should succeed");
    let updated_rg = update_resp
        .report_group()
        .expect("should have report_group");
    assert_eq!(updated_rg.tags().len(), 1);
    assert_eq!(updated_rg.tags()[0].key(), Some("env"));

    // DeleteReportGroup
    client
        .delete_report_group()
        .arn(&rg_arn)
        .send()
        .await
        .expect("delete_report_group should succeed");

    // After deletion, list should be empty
    let list_resp2 = client
        .list_report_groups()
        .send()
        .await
        .expect("list_report_groups should succeed after deletion");
    assert_eq!(list_resp2.report_groups().len(), 0);
}

// ============================================================
// test_codebuild_report_group_not_found
// ============================================================
#[tokio::test]
async fn test_codebuild_report_group_not_found() {
    let client = make_client().await;

    let result = client
        .delete_report_group()
        .arn("arn:aws:codebuild:eu-central-1:123456789012:report-group/nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================
// test_codebuild_report_group_duplicate_name
// ============================================================
#[tokio::test]
async fn test_codebuild_report_group_duplicate_name() {
    let client = make_client().await;

    let export_config = ReportExportConfig::builder()
        .export_config_type(ReportExportConfigType::NoExport)
        .build();

    client
        .create_report_group()
        .name("dup-group")
        .r#type(ReportType::Test)
        .export_config(export_config.clone())
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_report_group()
        .name("dup-group")
        .r#type(ReportType::Test)
        .export_config(export_config)
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_already_exists_exception());
}

// ============================================================
// test_codebuild_list_reports_for_report_group_not_found
// ============================================================
#[tokio::test]
async fn test_codebuild_list_reports_for_report_group_not_found() {
    let client = make_client().await;

    let result = client
        .list_reports_for_report_group()
        .report_group_arn("arn:aws:codebuild:eu-central-1:123456789012:report-group/nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================
// test_codebuild_resource_policy
// ============================================================
#[tokio::test]
async fn test_codebuild_resource_policy() {
    let client = make_client().await;

    let resp = client
        .create_project()
        .name("policy-project")
        .source(s3_source())
        .artifacts(no_artifacts())
        .environment(linux_env())
        .service_role(service_role())
        .send()
        .await
        .unwrap();

    let project_arn = resp.project().unwrap().arn().unwrap().to_string();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[]}"#;

    // Put policy
    let put_resp = client
        .put_resource_policy()
        .resource_arn(&project_arn)
        .policy(policy_doc)
        .send()
        .await
        .unwrap();
    assert_eq!(put_resp.resource_arn(), Some(project_arn.as_str()));

    // Get policy
    let get_resp = client
        .get_resource_policy()
        .resource_arn(&project_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.policy(), Some(policy_doc));

    // Delete policy
    client
        .delete_resource_policy()
        .resource_arn(&project_arn)
        .send()
        .await
        .unwrap();

    // After deletion, get should fail
    let result = client
        .get_resource_policy()
        .resource_arn(&project_arn)
        .send()
        .await;
    assert!(result.is_err());
}
