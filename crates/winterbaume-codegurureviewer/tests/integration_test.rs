use std::sync::{Arc, Mutex};

use aws_sdk_codegurureviewer::config::BehaviorVersion;
use aws_sdk_codegurureviewer::types::{
    CodeCommitRepository, CodeReviewType, Reaction, Repository, RepositoryAnalysis,
    RepositoryHeadSourceCodeType,
};
use winterbaume_codegurureviewer::CodeGuruReviewerService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_codegurureviewer::Client {
    let mock = MockAws::builder()
        .with_service(CodeGuruReviewerService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codegurureviewer::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_codegurureviewer::Client::new(&config)
}

async fn associate(client: &aws_sdk_codegurureviewer::Client) -> String {
    let repo = Repository::builder()
        .code_commit(
            CodeCommitRepository::builder()
                .name("my-repo")
                .build()
                .expect("cc"),
        )
        .build();
    let resp = client
        .associate_repository()
        .repository(repo)
        .send()
        .await
        .expect("associate");
    resp.repository_association()
        .and_then(|a| a.association_arn())
        .expect("arn")
        .to_string()
}

#[tokio::test]
async fn test_associate_describe_disassociate() {
    let client = make_client().await;
    let arn = associate(&client).await;
    let described = client
        .describe_repository_association()
        .association_arn(&arn)
        .send()
        .await
        .expect("describe");
    let assoc = described.repository_association().expect("assoc");
    assert_eq!(assoc.association_arn(), Some(arn.as_str()));
    assert_eq!(assoc.name(), Some("my-repo"));
    assert_eq!(
        assoc.provider_type().map(|p| p.as_str()),
        Some("CodeCommit")
    );

    let listed = client
        .list_repository_associations()
        .send()
        .await
        .expect("list");
    assert_eq!(listed.repository_association_summaries().len(), 1);

    client
        .disassociate_repository()
        .association_arn(&arn)
        .send()
        .await
        .expect("disassociate");
    let err = client
        .describe_repository_association()
        .association_arn(&arn)
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("NotFoundException"));
}

#[tokio::test]
async fn test_create_describe_code_review() {
    let client = make_client().await;
    let association_arn = associate(&client).await;
    let head = RepositoryHeadSourceCodeType::builder()
        .branch_name("main")
        .build()
        .expect("head");
    let analysis = RepositoryAnalysis::builder().repository_head(head).build();
    let cr_type = CodeReviewType::builder()
        .repository_analysis(analysis)
        .build();
    let resp = client
        .create_code_review()
        .name("review-1")
        .repository_association_arn(&association_arn)
        .r#type(cr_type)
        .send()
        .await
        .expect("create");
    let cr_arn = resp
        .code_review()
        .and_then(|cr| cr.code_review_arn())
        .expect("arn")
        .to_string();
    let got = client
        .describe_code_review()
        .code_review_arn(&cr_arn)
        .send()
        .await
        .expect("describe");
    let cr = got.code_review().expect("code_review");
    assert_eq!(cr.code_review_arn(), Some(cr_arn.as_str()));
    assert_eq!(cr.state().map(|s| s.as_str()), Some("Completed"));
    assert_eq!(cr.r#type().map(|t| t.as_str()), Some("RepositoryAnalysis"));
}

#[tokio::test]
async fn test_list_code_reviews_and_recommendations() {
    let client = make_client().await;
    let association_arn = associate(&client).await;
    let analysis = RepositoryAnalysis::builder()
        .repository_head(
            RepositoryHeadSourceCodeType::builder()
                .branch_name("main")
                .build()
                .expect("h"),
        )
        .build();
    let cr_type = CodeReviewType::builder()
        .repository_analysis(analysis)
        .build();
    let resp = client
        .create_code_review()
        .name("review-2")
        .repository_association_arn(&association_arn)
        .r#type(cr_type)
        .send()
        .await
        .expect("create");
    let cr_arn = resp
        .code_review()
        .and_then(|cr| cr.code_review_arn())
        .expect("arn")
        .to_string();
    let listed = client
        .list_code_reviews()
        .r#type(aws_sdk_codegurureviewer::types::Type::RepositoryAnalysis)
        .send()
        .await
        .expect("list");
    assert_eq!(listed.code_review_summaries().len(), 1);
    let recs = client
        .list_recommendations()
        .code_review_arn(&cr_arn)
        .send()
        .await
        .expect("list_recs");
    assert_eq!(recs.recommendation_summaries().len(), 0);
}

#[tokio::test]
async fn test_feedback_lifecycle() {
    let client = make_client().await;
    let association_arn = associate(&client).await;
    let analysis = RepositoryAnalysis::builder()
        .repository_head(
            RepositoryHeadSourceCodeType::builder()
                .branch_name("main")
                .build()
                .expect("h"),
        )
        .build();
    let cr_type = CodeReviewType::builder()
        .repository_analysis(analysis)
        .build();
    let resp = client
        .create_code_review()
        .name("feedback-review")
        .repository_association_arn(&association_arn)
        .r#type(cr_type)
        .send()
        .await
        .expect("create");
    let cr_arn = resp
        .code_review()
        .and_then(|cr| cr.code_review_arn())
        .expect("arn")
        .to_string();

    client
        .put_recommendation_feedback()
        .code_review_arn(&cr_arn)
        .recommendation_id("rec-1")
        .reactions(Reaction::ThumbsUp)
        .send()
        .await
        .expect("put");

    let described = client
        .describe_recommendation_feedback()
        .code_review_arn(&cr_arn)
        .recommendation_id("rec-1")
        .send()
        .await
        .expect("describe");
    let fb = described.recommendation_feedback().expect("feedback");
    assert_eq!(fb.recommendation_id(), Some("rec-1"));
    assert_eq!(fb.reactions().first().map(|r| r.as_str()), Some("ThumbsUp"));

    let listed = client
        .list_recommendation_feedback()
        .code_review_arn(&cr_arn)
        .send()
        .await
        .expect("list");
    assert_eq!(listed.recommendation_feedback_summaries().len(), 1);
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = associate(&client).await;
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag");
    let listed = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list");
    assert_eq!(
        listed.tags().and_then(|t| t.get("env")),
        Some(&"prod".to_string())
    );
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag");
    let listed = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list");
    assert!(listed.tags().map(|t| t.is_empty()).unwrap_or(true));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CodeGuruReviewerService::new();
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
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}
