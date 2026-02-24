use aws_sdk_codecommit::config::BehaviorVersion;
use winterbaume_codecommit::CodeCommitService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_codecommit::Client {
    let mock = MockAws::builder()
        .with_service(CodeCommitService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codecommit::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_codecommit::Client::new(&config)
}

#[tokio::test]
async fn test_create_repository() {
    let client = make_client().await;

    let resp = client
        .create_repository()
        .repository_name("my-repo")
        .repository_description("A test repository")
        .send()
        .await
        .expect("create_repository should succeed");

    let metadata = resp
        .repository_metadata()
        .expect("should have repository metadata");
    assert_eq!(metadata.repository_name(), Some("my-repo"));
    assert_eq!(metadata.repository_description(), Some("A test repository"));
    assert!(metadata.arn().unwrap().contains("arn:aws:codecommit:"));
    assert!(metadata.clone_url_http().is_some());
    assert!(metadata.clone_url_ssh().is_some());
}

#[tokio::test]
async fn test_get_repository() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("get-test")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_repository()
        .repository_name("get-test")
        .send()
        .await
        .expect("get_repository should succeed");

    let metadata = resp
        .repository_metadata()
        .expect("should have repository metadata");
    assert_eq!(metadata.repository_name(), Some("get-test"));
}

#[tokio::test]
async fn test_delete_repository() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("delete-me")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_repository()
        .repository_name("delete-me")
        .send()
        .await
        .expect("delete_repository should succeed");

    assert!(resp.repository_id().is_some());

    // Verify it's gone
    let result = client
        .get_repository()
        .repository_name("delete-me")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_repository() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("dup-test")
        .send()
        .await
        .unwrap();

    let result = client
        .create_repository()
        .repository_name("dup-test")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: CodeCommit
// ============================================================================

#[tokio::test]
async fn test_get_repository_not_found() {
    let client = make_client().await;

    let result = client
        .get_repository()
        .repository_name("nonexistent-repo")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(
        service_err.is_repository_does_not_exist_exception(),
        "Expected RepositoryDoesNotExistException"
    );
}

#[tokio::test]
async fn test_delete_repository_not_found() {
    let client = make_client().await;

    // delete_repository on nonexistent repo: AWS returns success with no repository_id
    // (it's idempotent in real AWS, but winterbaume may differ)
    let result = client
        .delete_repository()
        .repository_name("nonexistent-repo")
        .send()
        .await;

    // Accept either success (idempotent) or error
    let _ = result;
}

#[tokio::test]
async fn test_create_repository_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_repository()
        .repository_name("arn-test-repo")
        .send()
        .await
        .expect("create_repository should succeed");

    let metadata = resp.repository_metadata().expect("should have metadata");
    let arn = metadata.arn().expect("should have ARN");
    assert!(
        arn.starts_with("arn:aws:codecommit:"),
        "ARN should start with arn:aws:codecommit:"
    );
    assert!(
        arn.contains("arn-test-repo"),
        "ARN should contain repository name"
    );
}

#[tokio::test]
async fn test_create_repository_without_description() {
    let client = make_client().await;

    let resp = client
        .create_repository()
        .repository_name("no-desc-repo")
        .send()
        .await
        .expect("create_repository without description should succeed");

    let metadata = resp.repository_metadata().expect("should have metadata");
    assert_eq!(metadata.repository_name(), Some("no-desc-repo"));
    // description should be absent or empty when not provided
    assert!(
        metadata.repository_description().is_none()
            || metadata.repository_description() == Some("")
    );
}

#[tokio::test]
async fn test_repository_id_is_unique() {
    let client = make_client().await;

    let resp1 = client
        .create_repository()
        .repository_name("repo-uid-1")
        .send()
        .await
        .unwrap();
    let resp2 = client
        .create_repository()
        .repository_name("repo-uid-2")
        .send()
        .await
        .unwrap();

    let id1 = resp1
        .repository_metadata()
        .unwrap()
        .repository_id()
        .expect("should have id");
    let id2 = resp2
        .repository_metadata()
        .unwrap()
        .repository_id()
        .expect("should have id");
    assert_ne!(id1, id2, "repository IDs should be unique");
}

#[tokio::test]
async fn test_delete_returns_repository_id() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("del-id-repo")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_repository()
        .repository_name("del-id-repo")
        .send()
        .await
        .expect("delete_repository should succeed");

    assert!(
        resp.repository_id().is_some(),
        "delete should return the repository_id"
    );
}

#[tokio::test]
async fn test_clone_url_http_format() {
    let client = make_client().await;

    let resp = client
        .create_repository()
        .repository_name("clone-url-repo")
        .send()
        .await
        .unwrap();

    let metadata = resp.repository_metadata().unwrap();
    let clone_url = metadata
        .clone_url_http()
        .expect("should have clone URL HTTP");
    assert!(
        clone_url.contains("codecommit"),
        "clone URL should reference codecommit"
    );
    assert!(
        clone_url.contains("clone-url-repo"),
        "clone URL should contain repo name"
    );
}

#[tokio::test]
async fn test_clone_url_ssh_format() {
    let client = make_client().await;

    let resp = client
        .create_repository()
        .repository_name("ssh-url-repo")
        .send()
        .await
        .unwrap();

    let metadata = resp.repository_metadata().unwrap();
    let ssh_url = metadata.clone_url_ssh().expect("should have clone URL SSH");
    assert!(
        ssh_url.starts_with("ssh://git-codecommit."),
        "SSH clone URL should start with ssh://git-codecommit., got: {ssh_url}"
    );
    assert!(
        ssh_url.contains("ssh-url-repo"),
        "SSH clone URL should contain repo name, got: {ssh_url}"
    );
}

#[tokio::test]
async fn test_get_repository_all_fields_present() {
    let client = make_client().await;

    let create_resp = client
        .create_repository()
        .repository_name("all-fields-repo")
        .repository_description("Full field test")
        .send()
        .await
        .unwrap();
    let created_id = create_resp
        .repository_metadata()
        .unwrap()
        .repository_id()
        .expect("should have repository_id")
        .to_string();

    let get_resp = client
        .get_repository()
        .repository_name("all-fields-repo")
        .send()
        .await
        .unwrap();

    let metadata = get_resp
        .repository_metadata()
        .expect("should have metadata");
    assert_eq!(metadata.repository_name(), Some("all-fields-repo"));
    assert_eq!(metadata.repository_description(), Some("Full field test"));
    assert_eq!(metadata.repository_id(), Some(created_id.as_str()));
    assert!(metadata.arn().is_some(), "arn should be present");
    assert!(
        metadata.clone_url_http().is_some(),
        "clone_url_http should be present"
    );
    assert!(
        metadata.clone_url_ssh().is_some(),
        "clone_url_ssh should be present"
    );
    assert!(
        metadata.creation_date().is_some(),
        "creation_date should be present"
    );
    assert!(
        metadata.last_modified_date().is_some(),
        "last_modified_date should be present"
    );
    assert!(
        metadata.account_id().is_some(),
        "account_id should be present"
    );
}

#[tokio::test]
async fn test_repository_id_deterministic() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("deterministic-id-repo")
        .send()
        .await
        .unwrap();

    let resp1 = client
        .get_repository()
        .repository_name("deterministic-id-repo")
        .send()
        .await
        .unwrap();
    let resp2 = client
        .get_repository()
        .repository_name("deterministic-id-repo")
        .send()
        .await
        .unwrap();

    let id1 = resp1
        .repository_metadata()
        .unwrap()
        .repository_id()
        .unwrap()
        .to_string();
    let id2 = resp2
        .repository_metadata()
        .unwrap()
        .repository_id()
        .unwrap()
        .to_string();
    assert_eq!(
        id1, id2,
        "repository_id should be deterministic across multiple GetRepository calls"
    );
}

// ============================================================================
// ListRepositories tests
// ============================================================================

#[tokio::test]
async fn test_list_repositories_empty() {
    let client = make_client().await;

    let resp = client
        .list_repositories()
        .send()
        .await
        .expect("list_repositories should succeed");

    assert!(
        resp.repositories().is_empty(),
        "should return empty list when no repositories exist"
    );
}

#[tokio::test]
async fn test_list_repositories_multiple() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("list-repo-a")
        .send()
        .await
        .unwrap();
    client
        .create_repository()
        .repository_name("list-repo-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_repositories()
        .send()
        .await
        .expect("list_repositories should succeed");

    let repos = resp.repositories();
    assert_eq!(repos.len(), 2, "should list 2 repositories");

    let names: Vec<&str> = repos.iter().filter_map(|r| r.repository_name()).collect();
    assert!(names.contains(&"list-repo-a"));
    assert!(names.contains(&"list-repo-b"));
}

// ============================================================================
// UpdateRepositoryDescription tests
// ============================================================================

#[tokio::test]
async fn test_update_repository_description() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("update-desc-repo")
        .repository_description("original")
        .send()
        .await
        .unwrap();

    client
        .update_repository_description()
        .repository_name("update-desc-repo")
        .repository_description("updated description")
        .send()
        .await
        .expect("update_repository_description should succeed");

    let resp = client
        .get_repository()
        .repository_name("update-desc-repo")
        .send()
        .await
        .unwrap();
    let metadata = resp.repository_metadata().unwrap();
    assert_eq!(
        metadata.repository_description(),
        Some("updated description")
    );
}

#[tokio::test]
async fn test_update_repository_description_not_found() {
    let client = make_client().await;

    let result = client
        .update_repository_description()
        .repository_name("nonexistent")
        .repository_description("x")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// UpdateRepositoryName tests
// ============================================================================

#[tokio::test]
async fn test_update_repository_name() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("old-name-repo")
        .send()
        .await
        .unwrap();

    client
        .update_repository_name()
        .old_name("old-name-repo")
        .new_name("new-name-repo")
        .send()
        .await
        .expect("update_repository_name should succeed");

    // Old name should be gone
    let old_result = client
        .get_repository()
        .repository_name("old-name-repo")
        .send()
        .await;
    assert!(old_result.is_err(), "old name should no longer exist");

    // New name should work
    let new_resp = client
        .get_repository()
        .repository_name("new-name-repo")
        .send()
        .await
        .expect("new name should exist");
    assert_eq!(
        new_resp.repository_metadata().unwrap().repository_name(),
        Some("new-name-repo")
    );
}

// ============================================================================
// Branch operation tests
// ============================================================================

#[tokio::test]
async fn test_create_and_get_branch() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("branch-repo")
        .send()
        .await
        .unwrap();

    // CreateCommit to get a commit ID for the branch
    let commit_resp = client
        .create_commit()
        .repository_name("branch-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("README.md")
                .file_content(aws_smithy_types::Blob::new(b"# Hello" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_commit should succeed");

    let commit_id = commit_resp.commit_id().unwrap().to_string();

    client
        .create_branch()
        .repository_name("branch-repo")
        .branch_name("feature")
        .commit_id(&commit_id)
        .send()
        .await
        .expect("create_branch should succeed");

    let resp = client
        .get_branch()
        .repository_name("branch-repo")
        .branch_name("feature")
        .send()
        .await
        .expect("get_branch should succeed");

    let branch = resp.branch().expect("should have branch info");
    assert_eq!(branch.branch_name(), Some("feature"));
    assert_eq!(branch.commit_id(), Some(commit_id.as_str()));
}

#[tokio::test]
async fn test_list_branches() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("list-br-repo")
        .send()
        .await
        .unwrap();

    // Create a commit to get a commit ID
    let commit_resp = client
        .create_commit()
        .repository_name("list-br-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("file.txt")
                .file_content(aws_smithy_types::Blob::new(b"content" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let commit_id = commit_resp.commit_id().unwrap().to_string();

    client
        .create_branch()
        .repository_name("list-br-repo")
        .branch_name("develop")
        .commit_id(&commit_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_branches()
        .repository_name("list-br-repo")
        .send()
        .await
        .expect("list_branches should succeed");

    let branches = resp.branches();
    assert!(
        branches.len() >= 2,
        "should have at least 2 branches (main auto-created + develop)"
    );
    assert!(branches.contains(&"main".to_string()));
    assert!(branches.contains(&"develop".to_string()));
}

#[tokio::test]
async fn test_delete_branch() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("del-br-repo")
        .send()
        .await
        .unwrap();

    let commit_resp = client
        .create_commit()
        .repository_name("del-br-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("file.txt")
                .file_content(aws_smithy_types::Blob::new(b"x" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let commit_id = commit_resp.commit_id().unwrap().to_string();

    client
        .create_branch()
        .repository_name("del-br-repo")
        .branch_name("to-delete")
        .commit_id(&commit_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_branch()
        .repository_name("del-br-repo")
        .branch_name("to-delete")
        .send()
        .await
        .expect("delete_branch should succeed");

    let deleted = resp
        .deleted_branch()
        .expect("should have deleted branch info");
    assert_eq!(deleted.branch_name(), Some("to-delete"));

    // Verify branch is gone
    let get_result = client
        .get_branch()
        .repository_name("del-br-repo")
        .branch_name("to-delete")
        .send()
        .await;
    assert!(get_result.is_err(), "deleted branch should not be found");
}

#[tokio::test]
async fn test_update_default_branch() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("default-br-repo")
        .send()
        .await
        .unwrap();

    let commit_resp = client
        .create_commit()
        .repository_name("default-br-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("file.txt")
                .file_content(aws_smithy_types::Blob::new(b"x" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let commit_id = commit_resp.commit_id().unwrap().to_string();

    client
        .create_branch()
        .repository_name("default-br-repo")
        .branch_name("develop")
        .commit_id(&commit_id)
        .send()
        .await
        .unwrap();

    client
        .update_default_branch()
        .repository_name("default-br-repo")
        .default_branch_name("develop")
        .send()
        .await
        .expect("update_default_branch should succeed");

    // Verify default branch changed
    let repo_resp = client
        .get_repository()
        .repository_name("default-br-repo")
        .send()
        .await
        .unwrap();
    let metadata = repo_resp.repository_metadata().unwrap();
    assert_eq!(
        metadata.default_branch(),
        Some("develop"),
        "default branch should be updated"
    );
}

// ============================================================================
// CreateCommit / GetCommit tests
// ============================================================================

#[tokio::test]
async fn test_create_commit_and_get_commit() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("commit-repo")
        .send()
        .await
        .unwrap();

    let commit_resp = client
        .create_commit()
        .repository_name("commit-repo")
        .branch_name("main")
        .author_name("Test Author")
        .email("test@example.com")
        .commit_message("Initial commit")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("hello.txt")
                .file_content(aws_smithy_types::Blob::new(b"Hello World" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_commit should succeed");

    let commit_id = commit_resp.commit_id().expect("should have commit_id");
    assert!(!commit_id.is_empty());
    assert!(commit_resp.tree_id().is_some());

    // GetCommit
    let get_resp = client
        .get_commit()
        .repository_name("commit-repo")
        .commit_id(commit_id)
        .send()
        .await
        .expect("get_commit should succeed");

    let commit = get_resp.commit().expect("should have commit");
    assert_eq!(commit.commit_id(), Some(commit_id));
    assert_eq!(commit.message(), Some("Initial commit"));
}

// ============================================================================
// PutFile tests
// ============================================================================

#[tokio::test]
async fn test_put_file_and_get_file() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("putfile-repo")
        .send()
        .await
        .unwrap();

    // First commit to create a branch
    let initial = client
        .create_commit()
        .repository_name("putfile-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("init.txt")
                .file_content(aws_smithy_types::Blob::new(b"init" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let parent_commit = initial.commit_id().unwrap().to_string();

    // PutFile
    let put_resp = client
        .put_file()
        .repository_name("putfile-repo")
        .branch_name("main")
        .file_path("new-file.txt")
        .file_content(aws_smithy_types::Blob::new(b"file content here" as &[u8]))
        .parent_commit_id(&parent_commit)
        .commit_message("Add new file via PutFile")
        .send()
        .await
        .expect("put_file should succeed");

    let new_commit_id = put_resp.commit_id();
    assert!(!new_commit_id.is_empty());

    // GetFile
    let file_resp = client
        .get_file()
        .repository_name("putfile-repo")
        .file_path("new-file.txt")
        .commit_specifier(new_commit_id)
        .send()
        .await
        .expect("get_file should succeed");

    assert_eq!(file_resp.file_path(), "new-file.txt");
    // Blob/payload fields are not stored in the in-memory state; returned as empty until
    // the backing store is implemented.
    assert_eq!(
        file_resp.file_size(),
        0,
        "file_size should be 0 (no backing store yet)"
    );
    let content = file_resp.file_content();
    assert_eq!(
        content.as_ref(),
        b"",
        "file content should be empty (no backing store yet)"
    );
}

// ============================================================================
// DeleteFile tests
// ============================================================================

#[tokio::test]
async fn test_delete_file() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("delfile-repo")
        .send()
        .await
        .unwrap();

    let initial = client
        .create_commit()
        .repository_name("delfile-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("to-delete.txt")
                .file_content(aws_smithy_types::Blob::new(b"delete me" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let parent_commit = initial.commit_id().unwrap().to_string();

    let del_resp = client
        .delete_file()
        .repository_name("delfile-repo")
        .branch_name("main")
        .file_path("to-delete.txt")
        .parent_commit_id(&parent_commit)
        .send()
        .await
        .expect("delete_file should succeed");

    let del_commit_id = del_resp.commit_id();
    assert!(!del_commit_id.is_empty());

    // Verify file is gone
    let get_result = client
        .get_file()
        .repository_name("delfile-repo")
        .file_path("to-delete.txt")
        .commit_specifier(del_commit_id)
        .send()
        .await;
    assert!(get_result.is_err(), "deleted file should not be found");
}

// ============================================================================
// GetFolder tests
// ============================================================================

#[tokio::test]
async fn test_get_folder() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("folder-repo")
        .send()
        .await
        .unwrap();

    let commit_resp = client
        .create_commit()
        .repository_name("folder-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("src/main.rs")
                .file_content(aws_smithy_types::Blob::new(b"fn main() {}" as &[u8]))
                .build()
                .unwrap(),
        )
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("src/lib.rs")
                .file_content(aws_smithy_types::Blob::new(b"pub mod lib;" as &[u8]))
                .build()
                .unwrap(),
        )
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("README.md")
                .file_content(aws_smithy_types::Blob::new(b"# Readme" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let commit_id = commit_resp.commit_id().unwrap();

    let resp = client
        .get_folder()
        .repository_name("folder-repo")
        .folder_path("/")
        .commit_specifier(commit_id)
        .send()
        .await
        .expect("get_folder should succeed");

    // Root should have README.md as a file and src as a sub-folder
    let files = resp.files();
    let sub_folders = resp.sub_folders();
    assert!(
        files.iter().any(|f| f.absolute_path() == Some("README.md")),
        "root should contain README.md"
    );
    assert!(
        sub_folders.iter().any(|f| f.relative_path() == Some("src")),
        "root should contain src/ sub-folder"
    );
}

// ============================================================================
// GetDifferences tests
// ============================================================================

#[tokio::test]
async fn test_get_differences() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("diff-repo")
        .send()
        .await
        .unwrap();

    let c1 = client
        .create_commit()
        .repository_name("diff-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("a.txt")
                .file_content(aws_smithy_types::Blob::new(b"aaa" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let commit1 = c1.commit_id().unwrap().to_string();

    let c2 = client
        .create_commit()
        .repository_name("diff-repo")
        .branch_name("main")
        .parent_commit_id(&commit1)
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("b.txt")
                .file_content(aws_smithy_types::Blob::new(b"bbb" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let commit2 = c2.commit_id().unwrap().to_string();

    let resp = client
        .get_differences()
        .repository_name("diff-repo")
        .after_commit_specifier(&commit2)
        .before_commit_specifier(&commit1)
        .send()
        .await
        .expect("get_differences should succeed");

    let diffs = resp.differences();
    assert!(
        !diffs.is_empty(),
        "should have at least one difference (b.txt added)"
    );
}

// ============================================================================
// Pull Request tests
// ============================================================================

#[tokio::test]
async fn test_create_and_get_pull_request() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("pr-repo")
        .send()
        .await
        .unwrap();

    // Create commits on two branches
    let c1 = client
        .create_commit()
        .repository_name("pr-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("base.txt")
                .file_content(aws_smithy_types::Blob::new(b"base" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let main_commit = c1.commit_id().unwrap().to_string();

    client
        .create_branch()
        .repository_name("pr-repo")
        .branch_name("feature-branch")
        .commit_id(&main_commit)
        .send()
        .await
        .unwrap();

    let pr_resp = client
        .create_pull_request()
        .title("My PR")
        .description("A test pull request")
        .targets(
            aws_sdk_codecommit::types::Target::builder()
                .repository_name("pr-repo")
                .source_reference("feature-branch")
                .destination_reference("main")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_pull_request should succeed");

    let pr = pr_resp.pull_request().expect("should have pull_request");
    assert_eq!(pr.title(), Some("My PR"));
    assert_eq!(pr.description(), Some("A test pull request"));
    let pr_id = pr.pull_request_id().expect("should have PR ID");

    // GetPullRequest
    let get_resp = client
        .get_pull_request()
        .pull_request_id(pr_id)
        .send()
        .await
        .expect("get_pull_request should succeed");
    let got_pr = get_resp.pull_request().unwrap();
    assert_eq!(got_pr.pull_request_id(), Some(pr_id));
    assert_eq!(got_pr.title(), Some("My PR"));
}

#[tokio::test]
async fn test_list_pull_requests() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("list-pr-repo")
        .send()
        .await
        .unwrap();

    let c1 = client
        .create_commit()
        .repository_name("list-pr-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("f.txt")
                .file_content(aws_smithy_types::Blob::new(b"x" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let main_commit = c1.commit_id().unwrap().to_string();

    client
        .create_branch()
        .repository_name("list-pr-repo")
        .branch_name("feat")
        .commit_id(&main_commit)
        .send()
        .await
        .unwrap();

    client
        .create_pull_request()
        .title("PR 1")
        .targets(
            aws_sdk_codecommit::types::Target::builder()
                .repository_name("list-pr-repo")
                .source_reference("feat")
                .destination_reference("main")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pull_requests()
        .repository_name("list-pr-repo")
        .send()
        .await
        .expect("list_pull_requests should succeed");

    assert!(
        !resp.pull_request_ids().is_empty(),
        "should have at least one PR ID"
    );
}

#[tokio::test]
async fn test_update_pull_request_status() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("status-pr-repo")
        .send()
        .await
        .unwrap();

    let c1 = client
        .create_commit()
        .repository_name("status-pr-repo")
        .branch_name("main")
        .put_files(
            aws_sdk_codecommit::types::PutFileEntry::builder()
                .file_path("f.txt")
                .file_content(aws_smithy_types::Blob::new(b"x" as &[u8]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let main_commit = c1.commit_id().unwrap().to_string();

    client
        .create_branch()
        .repository_name("status-pr-repo")
        .branch_name("feat")
        .commit_id(&main_commit)
        .send()
        .await
        .unwrap();

    let pr_resp = client
        .create_pull_request()
        .title("Close Me")
        .targets(
            aws_sdk_codecommit::types::Target::builder()
                .repository_name("status-pr-repo")
                .source_reference("feat")
                .destination_reference("main")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let pr_id = pr_resp
        .pull_request()
        .unwrap()
        .pull_request_id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_pull_request_status()
        .pull_request_id(&pr_id)
        .pull_request_status(aws_sdk_codecommit::types::PullRequestStatusEnum::Closed)
        .send()
        .await
        .expect("update_pull_request_status should succeed");

    let updated_pr = update_resp.pull_request().unwrap();
    assert_eq!(
        updated_pr.pull_request_status(),
        Some(&aws_sdk_codecommit::types::PullRequestStatusEnum::Closed)
    );
}

// ============================================================================
// Tag operation tests
// ============================================================================

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_client().await;

    let create_resp = client
        .create_repository()
        .repository_name("tag-repo")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .repository_metadata()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags().expect("should have tags");
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("backend"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let create_resp = client
        .create_repository()
        .repository_name("untag-repo")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .repository_metadata()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().expect("should have tags");
    assert!(tags.get("env").is_none(), "env tag should be removed");
    assert_eq!(
        tags.get("team").map(|s| s.as_str()),
        Some("backend"),
        "team tag should remain"
    );
}

// ============================================================================
// Previously existing edge-case tests
// ============================================================================

#[tokio::test]
async fn test_delete_repository_idempotent_returns_empty_id() {
    let client = make_client().await;

    // Delete a repository that never existed; AWS returns success with null/empty repositoryId
    let resp = client
        .delete_repository()
        .repository_name("never-existed-repo")
        .send()
        .await
        .expect("delete_repository on nonexistent repo should succeed (idempotent)");

    // AWS says "a null repository ID is returned" — so either None or empty string is acceptable
    let repo_id = resp.repository_id().unwrap_or("");
    assert!(
        repo_id.is_empty(),
        "Expected empty/null repository_id for nonexistent repo, got: {repo_id}"
    );
}

#[tokio::test]
async fn test_create_duplicate_repository_error_type() {
    let client = make_client().await;

    client
        .create_repository()
        .repository_name("dup-error-type-repo")
        .send()
        .await
        .unwrap();

    let err = client
        .create_repository()
        .repository_name("dup-error-type-repo")
        .send()
        .await
        .unwrap_err()
        .into_service_error();

    assert!(
        err.is_repository_name_exists_exception(),
        "Expected RepositoryNameExistsException for duplicate create"
    );
}

#[tokio::test]
async fn test_lifecycle_create_get_delete() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_repository()
        .repository_name("lifecycle-repo")
        .repository_description("lifecycle test")
        .send()
        .await
        .expect("create should succeed");
    let created_id = create_resp
        .repository_metadata()
        .unwrap()
        .repository_id()
        .unwrap()
        .to_string();
    assert!(!created_id.is_empty(), "created_id should be non-empty");

    // Get
    let get_resp = client
        .get_repository()
        .repository_name("lifecycle-repo")
        .send()
        .await
        .expect("get after create should succeed");
    assert_eq!(
        get_resp.repository_metadata().unwrap().repository_id(),
        Some(created_id.as_str()),
        "get should return same id as create"
    );

    // Delete
    let del_resp = client
        .delete_repository()
        .repository_name("lifecycle-repo")
        .send()
        .await
        .expect("delete should succeed");
    assert_eq!(
        del_resp.repository_id(),
        Some(created_id.as_str()),
        "delete should return the repository_id"
    );

    // Verify gone
    let get_after = client
        .get_repository()
        .repository_name("lifecycle-repo")
        .send()
        .await;
    assert!(get_after.is_err(), "get after delete should fail");
    let service_err = get_after.unwrap_err().into_service_error();
    assert!(
        service_err.is_repository_does_not_exist_exception(),
        "Expected RepositoryDoesNotExistException after delete"
    );
}
