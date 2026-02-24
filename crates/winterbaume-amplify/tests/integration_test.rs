use std::sync::Arc;

use aws_sdk_amplify::config::BehaviorVersion;
use aws_sdk_amplify::config::Region;
use winterbaume_amplify::{AmplifyService, AmplifyStateView};
use winterbaume_core::{MockAws, StatefulService};

async fn make_client_with_service(svc: AmplifyService) -> aws_sdk_amplify::Client {
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_amplify::Client::new(&config)
}

async fn make_client() -> aws_sdk_amplify::Client {
    make_client_with_service(AmplifyService::new()).await
}

// ---- App tests ----

#[tokio::test]
async fn test_create_and_get_app() {
    let client = make_client().await;

    let create_resp = client
        .create_app()
        .name("my-app")
        .send()
        .await
        .expect("create_app should succeed");

    let app = create_resp.app().expect("app should be present");
    assert_eq!(app.name(), "my-app");
    let app_id = app.app_id().to_string();

    let get_resp = client
        .get_app()
        .app_id(&app_id)
        .send()
        .await
        .expect("get_app should succeed");

    let fetched = get_resp.app().expect("app should be present");
    assert_eq!(fetched.app_id(), app_id);
    assert_eq!(fetched.name(), "my-app");
}

#[tokio::test]
async fn test_list_apps() {
    let client = make_client().await;

    client
        .create_app()
        .name("app-alpha")
        .send()
        .await
        .expect("create_app alpha should succeed");

    client
        .create_app()
        .name("app-beta")
        .send()
        .await
        .expect("create_app beta should succeed");

    let list_resp = client
        .list_apps()
        .send()
        .await
        .expect("list_apps should succeed");

    let apps = list_resp.apps();
    assert_eq!(apps.len(), 2);
}

#[tokio::test]
async fn test_update_app() {
    let client = make_client().await;

    let create_resp = client
        .create_app()
        .name("update-app")
        .send()
        .await
        .expect("create_app should succeed");
    let app_id = create_resp.app().unwrap().app_id().to_string();

    let update_resp = client
        .update_app()
        .app_id(&app_id)
        .description("updated description")
        .send()
        .await
        .expect("update_app should succeed");

    let updated = update_resp.app().expect("app should be present");
    assert_eq!(updated.description(), "updated description");
}

#[tokio::test]
async fn test_delete_app() {
    let client = make_client().await;

    let create_resp = client
        .create_app()
        .name("delete-app")
        .send()
        .await
        .expect("create_app should succeed");
    let app_id = create_resp.app().unwrap().app_id().to_string();

    client
        .delete_app()
        .app_id(&app_id)
        .send()
        .await
        .expect("delete_app should succeed");

    // Should be gone
    let err = client.get_app().app_id(&app_id).send().await;
    assert!(err.is_err(), "get_app after delete should fail");
}

#[tokio::test]
async fn test_get_nonexistent_app_fails() {
    let client = make_client().await;
    let err = client.get_app().app_id("nonexistent").send().await;
    assert!(err.is_err(), "get_app for nonexistent app should fail");
}

// ---- Branch tests ----

#[tokio::test]
async fn test_branch_lifecycle() {
    let client = make_client().await;

    let app_id = client
        .create_app()
        .name("branch-test-app")
        .send()
        .await
        .unwrap()
        .app()
        .unwrap()
        .app_id()
        .to_string();

    // Create
    let create_resp = client
        .create_branch()
        .app_id(&app_id)
        .branch_name("main")
        .send()
        .await
        .expect("create_branch should succeed");

    let branch = create_resp.branch().expect("branch should be present");
    assert_eq!(branch.branch_name(), "main");

    // Get
    let get_resp = client
        .get_branch()
        .app_id(&app_id)
        .branch_name("main")
        .send()
        .await
        .expect("get_branch should succeed");
    assert_eq!(get_resp.branch().unwrap().branch_name(), "main");

    // List
    let list_resp = client
        .list_branches()
        .app_id(&app_id)
        .send()
        .await
        .expect("list_branches should succeed");
    assert_eq!(list_resp.branches().len(), 1);

    // Update
    let update_resp = client
        .update_branch()
        .app_id(&app_id)
        .branch_name("main")
        .description("main branch")
        .send()
        .await
        .expect("update_branch should succeed");
    assert_eq!(update_resp.branch().unwrap().description(), "main branch");

    // Delete
    client
        .delete_branch()
        .app_id(&app_id)
        .branch_name("main")
        .send()
        .await
        .expect("delete_branch should succeed");

    // Verify gone
    let err = client
        .get_branch()
        .app_id(&app_id)
        .branch_name("main")
        .send()
        .await;
    assert!(err.is_err(), "get_branch after delete should fail");
}

#[tokio::test]
async fn test_create_branch_missing_app_fails() {
    let client = make_client().await;
    let err = client
        .create_branch()
        .app_id("nonexistent")
        .branch_name("main")
        .send()
        .await;
    assert!(
        err.is_err(),
        "create_branch for nonexistent app should fail"
    );
}

// ---- Domain Association tests ----

#[tokio::test]
async fn test_domain_association_lifecycle() {
    let client = make_client().await;

    let app_id = client
        .create_app()
        .name("domain-app")
        .send()
        .await
        .unwrap()
        .app()
        .unwrap()
        .app_id()
        .to_string();

    // Create
    let create_resp = client
        .create_domain_association()
        .app_id(&app_id)
        .domain_name("example.com")
        .send()
        .await
        .expect("create_domain_association should succeed");

    let domain = create_resp
        .domain_association()
        .expect("domain_association should be present");
    assert_eq!(domain.domain_name(), "example.com");

    // Get
    let get_resp = client
        .get_domain_association()
        .app_id(&app_id)
        .domain_name("example.com")
        .send()
        .await
        .expect("get_domain_association should succeed");
    assert_eq!(
        get_resp.domain_association().unwrap().domain_name(),
        "example.com"
    );

    // List
    let list_resp = client
        .list_domain_associations()
        .app_id(&app_id)
        .send()
        .await
        .expect("list_domain_associations should succeed");
    assert_eq!(list_resp.domain_associations().len(), 1);

    // Delete
    client
        .delete_domain_association()
        .app_id(&app_id)
        .domain_name("example.com")
        .send()
        .await
        .expect("delete_domain_association should succeed");

    // Verify gone
    let err = client
        .get_domain_association()
        .app_id(&app_id)
        .domain_name("example.com")
        .send()
        .await;
    assert!(
        err.is_err(),
        "get_domain_association after delete should fail"
    );
}

// ---- Job tests ----

#[tokio::test]
async fn test_job_lifecycle() {
    let client = make_client().await;

    let app_id = client
        .create_app()
        .name("job-app")
        .send()
        .await
        .unwrap()
        .app()
        .unwrap()
        .app_id()
        .to_string();

    client
        .create_branch()
        .app_id(&app_id)
        .branch_name("main")
        .send()
        .await
        .expect("create_branch should succeed");

    // Start job
    let start_resp = client
        .start_job()
        .app_id(&app_id)
        .branch_name("main")
        .job_type(aws_sdk_amplify::types::JobType::Release)
        .send()
        .await
        .expect("start_job should succeed");

    let summary = start_resp
        .job_summary()
        .expect("job_summary should be present");
    let job_id = summary.job_id().to_string();
    assert_eq!(
        summary.job_type(),
        &aws_sdk_amplify::types::JobType::Release
    );

    // Get job
    let get_resp = client
        .get_job()
        .app_id(&app_id)
        .branch_name("main")
        .job_id(&job_id)
        .send()
        .await
        .expect("get_job should succeed");
    let job = get_resp.job().expect("job should be present");
    assert_eq!(job.summary().unwrap().job_id(), job_id);

    // List jobs
    let list_resp = client
        .list_jobs()
        .app_id(&app_id)
        .branch_name("main")
        .send()
        .await
        .expect("list_jobs should succeed");
    assert_eq!(list_resp.job_summaries().len(), 1);

    // Stop job
    let stop_resp = client
        .stop_job()
        .app_id(&app_id)
        .branch_name("main")
        .job_id(&job_id)
        .send()
        .await
        .expect("stop_job should succeed");
    let stopped = stop_resp
        .job_summary()
        .expect("job_summary should be present");
    assert_eq!(
        stopped.status(),
        &aws_sdk_amplify::types::JobStatus::Cancelled
    );
}

#[tokio::test]
async fn test_stop_nonexistent_job_fails() {
    let client = make_client().await;

    let app_id = client
        .create_app()
        .name("stop-job-app")
        .send()
        .await
        .unwrap()
        .app()
        .unwrap()
        .app_id()
        .to_string();

    client
        .create_branch()
        .app_id(&app_id)
        .branch_name("main")
        .send()
        .await
        .expect("create_branch should succeed");

    let err = client
        .stop_job()
        .app_id(&app_id)
        .branch_name("main")
        .job_id("nonexistent")
        .send()
        .await;
    assert!(err.is_err(), "stop_job for nonexistent job should fail");
}

// ---- Tag tests ----

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;

    let create_resp = client.create_app().name("tag-app").send().await.unwrap();
    let app = create_resp.app().unwrap();
    let resource_arn = app.app_arn().to_string();

    // Tag resource
    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().expect("tags should be present");
    assert_eq!(tags.get("env").map(|s: &String| s.as_str()), Some("test"));
    assert_eq!(
        tags.get("team").map(|s: &String| s.as_str()),
        Some("platform")
    );

    // Untag
    client
        .untag_resource()
        .resource_arn(&resource_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed after untag");

    let tags2 = list_resp2
        .tags()
        .expect("tags should be present after untag");
    assert!(!tags2.contains_key("env"), "env tag should be removed");
    assert!(tags2.contains_key("team"), "team tag should remain");
}

// ---- State view tests ----

#[tokio::test]
async fn test_snapshot_restore() {
    // Use restore to seed state, then snapshot and verify
    let svc_a = AmplifyService::new();
    let client_a = make_client_with_service(AmplifyService::new()).await;
    let _ = client_a;

    // Seed via restore
    let mut seed_view = AmplifyStateView::default();
    seed_view.apps.insert(
        "app-snap".to_string(),
        winterbaume_amplify::views::AmplifyAppView {
            app_id: "app-snap".to_string(),
            app_arn: "arn:aws:amplify:us-east-1:123456789012:apps/app-snap".to_string(),
            name: "snapshot-app".to_string(),
            description: None,
            repository: None,
            platform: None,
            create_time: 0.0,
            update_time: 0.0,
            iam_service_role_arn: None,
            environment_variables: Default::default(),
            default_domain: "app-snap.amplifyapp.com".to_string(),
            enable_branch_auto_build: false,
            enable_branch_auto_deletion: false,
            enable_basic_auth: false,
            build_spec: None,
            custom_headers: None,
            tags: Default::default(),
            auto_branch_creation_config: None,
            cache_config: None,
            custom_rules: vec![],
        },
    );

    svc_a
        .restore("123456789012", "us-east-1", seed_view)
        .await
        .expect("restore should succeed");

    let view = svc_a.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view.apps.len(), 1);

    // Restore into fresh service
    let svc_b = AmplifyService::new();
    svc_b
        .restore("123456789012", "us-east-1", view)
        .await
        .expect("restore should succeed");

    let view_b = svc_b.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view_b.apps.len(), 1);
    assert!(view_b.apps.values().any(|a| a.name == "snapshot-app"));
}

#[tokio::test]
async fn test_merge_does_not_remove_existing() {
    let svc = Arc::new(AmplifyService::new());

    // Seed with one app
    let view1 = {
        let mut v = AmplifyStateView::default();
        v.apps.insert(
            "app1".to_string(),
            winterbaume_amplify::views::AmplifyAppView {
                app_id: "app1".to_string(),
                app_arn: "arn:aws:amplify:us-east-1:123456789012:apps/app1".to_string(),
                name: "app-one".to_string(),
                description: None,
                repository: None,
                platform: None,
                create_time: 0.0,
                update_time: 0.0,
                iam_service_role_arn: None,
                environment_variables: Default::default(),
                default_domain: "app1.amplifyapp.com".to_string(),
                enable_branch_auto_build: false,
                enable_branch_auto_deletion: false,
                enable_basic_auth: false,
                build_spec: None,
                custom_headers: None,
                tags: Default::default(),
                auto_branch_creation_config: None,
                cache_config: None,
                custom_rules: vec![],
            },
        );
        v
    };

    svc.restore("123456789012", "us-east-1", view1)
        .await
        .expect("restore should succeed");

    // Merge in a second app
    let view2 = {
        let mut v = AmplifyStateView::default();
        v.apps.insert(
            "app2".to_string(),
            winterbaume_amplify::views::AmplifyAppView {
                app_id: "app2".to_string(),
                app_arn: "arn:aws:amplify:us-east-1:123456789012:apps/app2".to_string(),
                name: "app-two".to_string(),
                description: None,
                repository: None,
                platform: None,
                create_time: 0.0,
                update_time: 0.0,
                iam_service_role_arn: None,
                environment_variables: Default::default(),
                default_domain: "app2.amplifyapp.com".to_string(),
                enable_branch_auto_build: false,
                enable_branch_auto_deletion: false,
                enable_basic_auth: false,
                build_spec: None,
                custom_headers: None,
                tags: Default::default(),
                auto_branch_creation_config: None,
                cache_config: None,
                custom_rules: vec![],
            },
        );
        v
    };

    svc.merge("123456789012", "us-east-1", view2)
        .await
        .expect("merge should succeed");

    let final_view = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(
        final_view.apps.len(),
        2,
        "both apps should be present after merge"
    );
    assert!(
        final_view.apps.contains_key("app1"),
        "app1 should still be present"
    );
    assert!(
        final_view.apps.contains_key("app2"),
        "app2 should be present"
    );
}

// ---- State change notification tests ----

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::Mutex;

    let svc = AmplifyService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);

    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::sync::Mutex;

    let svc = AmplifyService::new();

    // Pre-seed with one app
    let view1 = {
        let mut v = AmplifyStateView::default();
        v.apps.insert(
            "app1".to_string(),
            winterbaume_amplify::views::AmplifyAppView {
                app_id: "app1".to_string(),
                app_arn: "arn:aws:amplify:us-east-1:123456789012:apps/app1".to_string(),
                name: "seed-app".to_string(),
                description: None,
                repository: None,
                platform: None,
                create_time: 0.0,
                update_time: 0.0,
                iam_service_role_arn: None,
                environment_variables: Default::default(),
                default_domain: "app1.amplifyapp.com".to_string(),
                enable_branch_auto_build: false,
                enable_branch_auto_deletion: false,
                enable_basic_auth: false,
                build_spec: None,
                custom_headers: None,
                tags: Default::default(),
                auto_branch_creation_config: None,
                cache_config: None,
                custom_rules: vec![],
            },
        );
        v
    };
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap(); // ignore first event

    // Register listener
    let snapshots: Arc<Mutex<Vec<AmplifyStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let view2 = {
        let mut v = AmplifyStateView::default();
        v.apps.insert(
            "app2".to_string(),
            winterbaume_amplify::views::AmplifyAppView {
                app_id: "app2".to_string(),
                app_arn: "arn:aws:amplify:us-east-1:123456789012:apps/app2".to_string(),
                name: "new-app".to_string(),
                description: None,
                repository: None,
                platform: None,
                create_time: 0.0,
                update_time: 0.0,
                iam_service_role_arn: None,
                environment_variables: Default::default(),
                default_domain: "app2.amplifyapp.com".to_string(),
                enable_branch_auto_build: false,
                enable_branch_auto_deletion: false,
                enable_basic_auth: false,
                build_spec: None,
                custom_headers: None,
                tags: Default::default(),
                auto_branch_creation_config: None,
                cache_config: None,
                custom_rules: vec![],
            },
        );
        v
    };

    svc.restore("123456789012", "us-east-1", view2)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0].apps.contains_key("app2"),
        "snapshot should contain the restored app"
    );
    assert!(
        !got[0].apps.contains_key("app1"),
        "restore replaces state — app1 should be gone"
    );
}
