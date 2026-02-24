use aws_sdk_codeartifact::config::BehaviorVersion;
use winterbaume_codeartifact::CodeArtifactService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_codeartifact::Client {
    let mock = MockAws::builder()
        .with_service(CodeArtifactService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codeartifact::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_codeartifact::Client::new(&config)
}

// ---- Domain tests ----

#[tokio::test]
async fn test_create_and_describe_domain() {
    let client = make_client().await;

    client
        .create_domain()
        .domain("test-domain")
        .send()
        .await
        .expect("create_domain should succeed");

    let resp = client
        .describe_domain()
        .domain("test-domain")
        .send()
        .await
        .expect("describe_domain should succeed");

    let domain = resp.domain().expect("should have domain");
    assert_eq!(domain.name(), Some("test-domain"));
    assert_eq!(domain.status().map(|s| s.as_str()), Some("Active"));
}

#[tokio::test]
async fn test_list_domains() {
    let client = make_client().await;

    for name in ["dom-a", "dom-b", "dom-c"] {
        client
            .create_domain()
            .domain(name)
            .send()
            .await
            .expect("create_domain should succeed");
    }

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");

    assert_eq!(resp.domains().len(), 3);
}

#[tokio::test]
async fn test_delete_domain() {
    let client = make_client().await;

    client
        .create_domain()
        .domain("del-domain")
        .send()
        .await
        .expect("create_domain should succeed");

    client
        .delete_domain()
        .domain("del-domain")
        .send()
        .await
        .expect("delete_domain should succeed");

    let result = client.describe_domain().domain("del-domain").send().await;
    assert!(result.is_err(), "should error for deleted domain");
}

#[tokio::test]
async fn test_create_domain_duplicate_fails() {
    let client = make_client().await;

    client
        .create_domain()
        .domain("dup-domain")
        .send()
        .await
        .expect("create_domain should succeed");

    let result = client.create_domain().domain("dup-domain").send().await;
    assert!(result.is_err(), "duplicate domain creation should fail");
}

// ---- Repository tests ----

#[tokio::test]
async fn test_create_and_describe_repository() {
    let client = make_client().await;

    client
        .create_domain()
        .domain("repo-domain")
        .send()
        .await
        .expect("create_domain should succeed");

    client
        .create_repository()
        .domain("repo-domain")
        .repository("test-repo")
        .description("My test repo")
        .send()
        .await
        .expect("create_repository should succeed");

    let resp = client
        .describe_repository()
        .domain("repo-domain")
        .repository("test-repo")
        .send()
        .await
        .expect("describe_repository should succeed");

    let repo = resp.repository().expect("should have repository");
    assert_eq!(repo.name(), Some("test-repo"));
    assert_eq!(repo.domain_name(), Some("repo-domain"));
    assert_eq!(repo.description(), Some("My test repo"));
}

#[tokio::test]
async fn test_list_repositories() {
    let client = make_client().await;

    client
        .create_domain()
        .domain("list-repo-domain")
        .send()
        .await
        .expect("create_domain should succeed");

    for name in ["repo-1", "repo-2"] {
        client
            .create_repository()
            .domain("list-repo-domain")
            .repository(name)
            .send()
            .await
            .expect("create_repository should succeed");
    }

    let resp = client
        .list_repositories()
        .send()
        .await
        .expect("list_repositories should succeed");

    assert_eq!(resp.repositories().len(), 2);
}

#[tokio::test]
async fn test_list_repositories_in_domain() {
    let client = make_client().await;

    client
        .create_domain()
        .domain("domain-a")
        .send()
        .await
        .expect("create_domain should succeed");

    client
        .create_domain()
        .domain("domain-b")
        .send()
        .await
        .expect("create_domain should succeed");

    client
        .create_repository()
        .domain("domain-a")
        .repository("repo-in-a")
        .send()
        .await
        .expect("create_repository should succeed");

    client
        .create_repository()
        .domain("domain-b")
        .repository("repo-in-b")
        .send()
        .await
        .expect("create_repository should succeed");

    let resp = client
        .list_repositories_in_domain()
        .domain("domain-a")
        .send()
        .await
        .expect("list_repositories_in_domain should succeed");

    assert_eq!(resp.repositories().len(), 1);
    assert_eq!(resp.repositories()[0].name(), Some("repo-in-a"));
}

#[tokio::test]
async fn test_delete_repository() {
    let client = make_client().await;

    client
        .create_domain()
        .domain("del-repo-domain")
        .send()
        .await
        .expect("create_domain should succeed");

    client
        .create_repository()
        .domain("del-repo-domain")
        .repository("del-repo")
        .send()
        .await
        .expect("create_repository should succeed");

    client
        .delete_repository()
        .domain("del-repo-domain")
        .repository("del-repo")
        .send()
        .await
        .expect("delete_repository should succeed");

    let result = client
        .describe_repository()
        .domain("del-repo-domain")
        .repository("del-repo")
        .send()
        .await;
    assert!(result.is_err(), "should error for deleted repository");
}

#[tokio::test]
async fn test_create_repository_without_domain_fails() {
    let client = make_client().await;

    let result = client
        .create_repository()
        .domain("nonexistent-domain")
        .repository("some-repo")
        .send()
        .await;
    assert!(result.is_err(), "should fail when domain does not exist");
}

// ---- State view tests ----

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use std::collections::HashMap;

    use winterbaume_codeartifact::views::{CodeArtifactStateView, DomainView};
    use winterbaume_core::StatefulService;

    let svc = CodeArtifactService::new();

    let mut domains = HashMap::new();
    domains.insert(
        "my-domain".to_string(),
        DomainView {
            name: "my-domain".to_string(),
            owner: "123456789012".to_string(),
            arn: "arn:aws:codeartifact:us-east-1:123456789012:domain/my-domain".to_string(),
            encryption_key: None,
            status: "Active".to_string(),
            created_time: 0.0,
            tags: HashMap::new(),
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        CodeArtifactStateView {
            domains,
            ..Default::default()
        },
    )
    .await
    .expect("restore should succeed");

    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view.domains.len(), 1);
    assert!(view.domains.contains_key("my-domain"));

    let svc2 = CodeArtifactService::new();
    svc2.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore should succeed");
    let view2 = svc2.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view2.domains.len(), 1);
}

// ---- State change listener tests ----

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = CodeArtifactService::new();
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
