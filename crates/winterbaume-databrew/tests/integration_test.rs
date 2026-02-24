use aws_sdk_databrew::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_databrew::DataBrewService;

async fn make_databrew_client() -> aws_sdk_databrew::Client {
    let mock = MockAws::builder()
        .with_service(DataBrewService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_databrew::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_databrew::Client::new(&config)
}

// ── Dataset tests ───────────────────────────────────────────────────

#[tokio::test]
async fn test_create_dataset() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .key("data/input.csv")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .create_dataset()
        .name("test-dataset")
        .input(input)
        .format(aws_sdk_databrew::types::InputFormat::Csv)
        .send()
        .await
        .expect("create_dataset should succeed");

    assert_eq!(resp.name(), "test-dataset");
}

#[tokio::test]
async fn test_describe_dataset() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .key("data/input.csv")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("desc-dataset")
        .input(input)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_dataset()
        .name("desc-dataset")
        .send()
        .await
        .expect("describe_dataset should succeed");

    assert_eq!(resp.name(), "desc-dataset");
    assert!(resp.resource_arn().is_some());
    assert!(resp.input().is_some());
}

#[tokio::test]
async fn test_delete_dataset() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .key("data/input.csv")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("del-dataset")
        .input(input)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_dataset()
        .name("del-dataset")
        .send()
        .await
        .expect("delete_dataset should succeed");

    assert_eq!(resp.name(), "del-dataset");

    // Verify it's gone
    let result = client.describe_dataset().name("del-dataset").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_datasets() {
    let client = make_databrew_client().await;

    for name in ["dataset-a", "dataset-b", "dataset-c"] {
        let input = aws_sdk_databrew::types::Input::builder()
            .s3_input_definition(
                aws_sdk_databrew::types::S3Location::builder()
                    .bucket("my-bucket")
                    .key("data/input.csv")
                    .build()
                    .unwrap(),
            )
            .build();

        client
            .create_dataset()
            .name(name)
            .input(input)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_datasets()
        .send()
        .await
        .expect("list_datasets should succeed");

    assert_eq!(resp.datasets().len(), 3);
}

#[tokio::test]
async fn test_create_duplicate_dataset_fails() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .key("data/input.csv")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("dup-dataset")
        .input(input.clone())
        .send()
        .await
        .unwrap();

    let result = client
        .create_dataset()
        .name("dup-dataset")
        .input(input)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_dataset() {
    let client = make_databrew_client().await;

    let result = client.describe_dataset().name("nonexistent").send().await;

    assert!(result.is_err());
}

// ── Recipe tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_recipe() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .create_recipe()
        .name("test-recipe")
        .steps(step)
        .send()
        .await
        .expect("create_recipe should succeed");

    assert_eq!(resp.name(), "test-recipe");
}

#[tokio::test]
async fn test_describe_recipe() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("desc-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_recipe()
        .name("desc-recipe")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .expect("describe_recipe should succeed");

    assert_eq!(resp.name(), "desc-recipe");
    assert!(resp.resource_arn().is_some());
}

#[tokio::test]
async fn test_update_recipe() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("upd-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    let new_step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("LOWER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .update_recipe()
        .name("upd-recipe")
        .steps(new_step)
        .send()
        .await
        .expect("update_recipe should succeed");

    assert_eq!(resp.name(), "upd-recipe");
}

#[tokio::test]
async fn test_publish_recipe() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("pub-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    let resp = client
        .publish_recipe()
        .name("pub-recipe")
        .send()
        .await
        .expect("publish_recipe should succeed");

    assert_eq!(resp.name(), "pub-recipe");
}

#[tokio::test]
async fn test_list_recipes() {
    let client = make_databrew_client().await;

    for name in ["recipe-a", "recipe-b"] {
        let step = aws_sdk_databrew::types::RecipeStep::builder()
            .action(
                aws_sdk_databrew::types::RecipeAction::builder()
                    .operation("UPPER_CASE")
                    .build()
                    .unwrap(),
            )
            .build();

        client
            .create_recipe()
            .name(name)
            .steps(step)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_recipes()
        .send()
        .await
        .expect("list_recipes should succeed");

    assert_eq!(resp.recipes().len(), 2);
}

#[tokio::test]
async fn test_list_recipe_versions() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("ver-recipe")
        .steps(step.clone())
        .send()
        .await
        .unwrap();

    // Before publishing, list_recipe_versions should return 0
    let resp = client
        .list_recipe_versions()
        .name("ver-recipe")
        .send()
        .await
        .expect("list_recipe_versions should succeed");
    assert_eq!(resp.recipes().len(), 0);

    // Publish to create version 1.0
    client
        .publish_recipe()
        .name("ver-recipe")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_recipe_versions()
        .name("ver-recipe")
        .send()
        .await
        .expect("list_recipe_versions should succeed");
    assert_eq!(resp.recipes().len(), 1);

    // Publish again to create version 2.0
    client
        .publish_recipe()
        .name("ver-recipe")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_recipe_versions()
        .name("ver-recipe")
        .send()
        .await
        .expect("list_recipe_versions should succeed");
    assert_eq!(resp.recipes().len(), 2);
}

#[tokio::test]
async fn test_delete_recipe_version() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("del-ver-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    // Publish to create version 1.0
    client
        .publish_recipe()
        .name("del-ver-recipe")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_recipe_version()
        .name("del-ver-recipe")
        .recipe_version("1.0")
        .send()
        .await
        .expect("delete_recipe_version should succeed");

    assert_eq!(resp.name(), "del-ver-recipe");
    assert_eq!(resp.recipe_version(), "1.0");
}

#[tokio::test]
async fn test_describe_nonexistent_recipe() {
    let client = make_databrew_client().await;

    let result = client.describe_recipe().name("nonexistent").send().await;
    assert!(result.is_err());
}

// ── Ruleset tests ───────────────────────────────────────────────────

#[tokio::test]
async fn test_create_ruleset() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("rule-1")
        .check_expression("column_values > 0")
        .build()
        .unwrap();

    let resp = client
        .create_ruleset()
        .name("test-ruleset")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/my-dataset")
        .rules(rule)
        .send()
        .await
        .expect("create_ruleset should succeed");

    assert_eq!(resp.name(), "test-ruleset");
}

#[tokio::test]
async fn test_describe_ruleset() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("rule-1")
        .check_expression("column_values > 0")
        .build()
        .unwrap();

    client
        .create_ruleset()
        .name("desc-ruleset")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/my-dataset")
        .rules(rule)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_ruleset()
        .name("desc-ruleset")
        .send()
        .await
        .expect("describe_ruleset should succeed");

    assert_eq!(resp.name(), "desc-ruleset");
    assert!(resp.resource_arn().is_some());
    assert!(resp.target_arn().is_some());
}

#[tokio::test]
async fn test_delete_ruleset() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("rule-1")
        .check_expression("column_values > 0")
        .build()
        .unwrap();

    client
        .create_ruleset()
        .name("del-ruleset")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/my-dataset")
        .rules(rule)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_ruleset()
        .name("del-ruleset")
        .send()
        .await
        .expect("delete_ruleset should succeed");

    assert_eq!(resp.name(), "del-ruleset");

    // Verify it's gone
    let result = client.describe_ruleset().name("del-ruleset").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_rulesets() {
    let client = make_databrew_client().await;

    for name in ["ruleset-a", "ruleset-b"] {
        let rule = aws_sdk_databrew::types::Rule::builder()
            .name("rule-1")
            .check_expression("column_values > 0")
            .build()
            .unwrap();

        client
            .create_ruleset()
            .name(name)
            .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/my-dataset")
            .rules(rule)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_rulesets()
        .send()
        .await
        .expect("list_rulesets should succeed");

    assert_eq!(resp.rulesets().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_ruleset() {
    let client = make_databrew_client().await;

    let result = client.describe_ruleset().name("nonexistent").send().await;
    assert!(result.is_err());
}

// ── Schedule tests ──────────────────────────────────────────────────

#[tokio::test]
async fn test_create_schedule() {
    let client = make_databrew_client().await;

    let resp = client
        .create_schedule()
        .name("test-schedule")
        .cron_expression("cron(0 12 * * ? *)")
        .job_names("my-job")
        .send()
        .await
        .expect("create_schedule should succeed");

    assert_eq!(resp.name(), "test-schedule");
}

#[tokio::test]
async fn test_describe_schedule() {
    let client = make_databrew_client().await;

    client
        .create_schedule()
        .name("desc-schedule")
        .cron_expression("cron(0 12 * * ? *)")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_schedule()
        .name("desc-schedule")
        .send()
        .await
        .expect("describe_schedule should succeed");

    assert_eq!(resp.name(), "desc-schedule");
    assert!(resp.resource_arn().is_some());
    assert_eq!(resp.cron_expression(), Some("cron(0 12 * * ? *)"));
}

#[tokio::test]
async fn test_update_schedule() {
    let client = make_databrew_client().await;

    client
        .create_schedule()
        .name("upd-schedule")
        .cron_expression("cron(0 12 * * ? *)")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_schedule()
        .name("upd-schedule")
        .cron_expression("cron(0 6 * * ? *)")
        .send()
        .await
        .expect("update_schedule should succeed");

    assert_eq!(resp.name(), "upd-schedule");

    // Verify the update
    let desc = client
        .describe_schedule()
        .name("upd-schedule")
        .send()
        .await
        .unwrap();

    assert_eq!(desc.cron_expression(), Some("cron(0 6 * * ? *)"));
}

#[tokio::test]
async fn test_delete_schedule() {
    let client = make_databrew_client().await;

    client
        .create_schedule()
        .name("del-schedule")
        .cron_expression("cron(0 12 * * ? *)")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_schedule()
        .name("del-schedule")
        .send()
        .await
        .expect("delete_schedule should succeed");

    assert_eq!(resp.name(), "del-schedule");

    // Verify it's gone
    let result = client.describe_schedule().name("del-schedule").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_schedules() {
    let client = make_databrew_client().await;

    for name in ["sched-a", "sched-b", "sched-c"] {
        client
            .create_schedule()
            .name(name)
            .cron_expression("cron(0 12 * * ? *)")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_schedules()
        .send()
        .await
        .expect("list_schedules should succeed");

    assert_eq!(resp.schedules().len(), 3);
}

#[tokio::test]
async fn test_describe_nonexistent_schedule() {
    let client = make_databrew_client().await;

    let result = client.describe_schedule().name("nonexistent").send().await;
    assert!(result.is_err());
}

// ── Tag tests ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_tag_and_list_tags_for_dataset() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .key("data/input.csv")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("tag-dataset")
        .input(input)
        .send()
        .await
        .unwrap();

    let arn = "arn:aws:databrew:us-east-1:123456789012:dataset/tag-dataset";

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(arn)
        .tags("env", "test")
        .tags("team", "data")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("data"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .key("data/input.csv")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("untag-dataset")
        .input(input)
        .tags("env", "test")
        .tags("team", "data")
        .send()
        .await
        .unwrap();

    let arn = "arn:aws:databrew:us-east-1:123456789012:dataset/untag-dataset";

    // Untag
    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify
    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert!(tags.get("env").is_none());
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("data"));
}

// ============================================================================
// Ported from moto: test_databrew_datasets.py
// ============================================================================

// Ported from moto: test_databrew_datasets.py::test_dataset_list_when_empty
#[tokio::test]
async fn test_dataset_list_when_empty() {
    let client = make_databrew_client().await;
    let resp = client.list_datasets().send().await.unwrap();
    assert_eq!(resp.datasets().len(), 0);
}

// Ported from moto: test_databrew_datasets.py::test_describe_dataset_that_does_not_exist
#[tokio::test]
async fn test_describe_dataset_that_does_not_exist() {
    let client = make_databrew_client().await;
    let err = client
        .describe_dataset()
        .name("DoseNotExist")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_datasets.py::test_create_dataset_that_already_exists
#[tokio::test]
async fn test_create_dataset_that_already_exists() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .create_dataset()
        .name("dup-ds")
        .input(input.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_dataset()
        .name("dup-ds")
        .input(input)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("AlreadyExists") || err_str.contains("already exists"),
        "Expected AlreadyExists error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_datasets.py::test_delete_dataset
#[tokio::test]
async fn test_delete_dataset_and_verify_gone() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("to-delete")
        .input(input)
        .send()
        .await
        .unwrap();

    // Check it exists
    let ds = client
        .describe_dataset()
        .name("to-delete")
        .send()
        .await
        .unwrap();
    assert_eq!(ds.name(), "to-delete");

    // Delete
    client
        .delete_dataset()
        .name("to-delete")
        .send()
        .await
        .unwrap();

    // Verify gone
    let err = client
        .describe_dataset()
        .name("to-delete")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());

    // Delete again should error
    let err = client
        .delete_dataset()
        .name("to-delete")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_datasets.py::test_update_dataset
#[tokio::test]
async fn test_update_dataset() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("upd-ds")
        .input(input)
        .format(aws_sdk_databrew::types::InputFormat::Json)
        .send()
        .await
        .unwrap();

    // Update the dataset
    let new_input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("new-bucket")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .update_dataset()
        .name("upd-ds")
        .input(new_input)
        .format(aws_sdk_databrew::types::InputFormat::Csv)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.name(), "upd-ds");

    // Describe and verify
    let ds = client
        .describe_dataset()
        .name("upd-ds")
        .send()
        .await
        .unwrap();
    assert_eq!(ds.name(), "upd-ds");
    assert!(ds.resource_arn().is_some());
}

// Ported from moto: test_databrew_datasets.py::test_update_dataset_that_does_not_exist
#[tokio::test]
async fn test_update_dataset_that_does_not_exist() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .build();

    let err = client
        .update_dataset()
        .name("RANDOMNAME")
        .input(input)
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_datasets.py::test_describe_dataset (format check)
#[tokio::test]
async fn test_describe_dataset_format() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("csv-ds")
        .input(input)
        .format(aws_sdk_databrew::types::InputFormat::Csv)
        .send()
        .await
        .unwrap();

    let ds = client
        .describe_dataset()
        .name("csv-ds")
        .send()
        .await
        .unwrap();
    assert_eq!(ds.format().map(|f| f.as_str()), Some("CSV"));
}

// ============================================================================
// Ported from moto: test_databrew_recipes.py
// ============================================================================

// Ported from moto: test_databrew_recipes.py::test_recipe_list_when_empty
#[tokio::test]
async fn test_recipe_list_when_empty() {
    let client = make_databrew_client().await;
    let resp = client
        .list_recipes()
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.recipes().len(), 0);
}

// Ported from moto: test_databrew_recipes.py::test_describe_recipe_latest_working
#[tokio::test]
async fn test_describe_recipe_latest_working() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("REMOVE_COMBINED")
                .build()
                .unwrap(),
        )
        .build();

    let create_resp = client
        .create_recipe()
        .name("lw-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    let recipe = client
        .describe_recipe()
        .name("lw-recipe")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap();

    assert_eq!(recipe.name(), "lw-recipe");
    assert_eq!(recipe.steps().len(), 1);
    assert_eq!(recipe.recipe_version(), Some("0.1"));
}

// Ported from moto: test_databrew_recipes.py::test_describe_recipe_with_version
#[tokio::test]
async fn test_describe_recipe_with_version() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("ver-desc-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    let recipe = client
        .describe_recipe()
        .name("ver-desc-recipe")
        .recipe_version("0.1")
        .send()
        .await
        .unwrap();

    assert_eq!(recipe.name(), "ver-desc-recipe");
    assert_eq!(recipe.steps().len(), 1);
    assert_eq!(recipe.recipe_version(), Some("0.1"));
}

// Ported from moto: test_databrew_recipes.py::test_describe_recipe_latest_published
#[tokio::test]
async fn test_describe_recipe_latest_published() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .create_recipe()
        .name("pub-desc-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .publish_recipe()
        .name("pub-desc-recipe")
        .send()
        .await
        .unwrap();

    let recipe = client
        .describe_recipe()
        .name("pub-desc-recipe")
        .recipe_version("LATEST_PUBLISHED")
        .send()
        .await
        .unwrap();

    assert_eq!(recipe.name(), "pub-desc-recipe");
    assert_eq!(recipe.steps().len(), 1);
    assert_eq!(recipe.recipe_version(), Some("1.0"));
}

// Ported from moto: test_databrew_recipes.py::test_describe_recipe_implicit_latest_published
#[tokio::test]
async fn test_describe_recipe_implicit_latest_published() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("impl-pub-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .publish_recipe()
        .name("impl-pub-recipe")
        .send()
        .await
        .unwrap();

    // Without specifying version, should return LATEST_PUBLISHED
    let recipe = client
        .describe_recipe()
        .name("impl-pub-recipe")
        .send()
        .await
        .unwrap();

    assert_eq!(recipe.name(), "impl-pub-recipe");
    assert_eq!(recipe.recipe_version(), Some("1.0"));
}

// Ported from moto: test_databrew_recipes.py::test_describe_recipe_that_does_not_exist
#[tokio::test]
async fn test_describe_recipe_that_does_not_exist() {
    let client = make_databrew_client().await;
    let err = client
        .describe_recipe()
        .name("DoseNotExist")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_recipes.py::test_create_recipe_that_already_exists
#[tokio::test]
async fn test_create_recipe_that_already_exists() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("dup-recipe")
        .steps(step.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_recipe()
        .name("dup-recipe")
        .steps(step)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Conflict") || err_str.contains("already exists"),
        "Expected conflict error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_recipes.py::test_update_recipe
#[tokio::test]
async fn test_update_recipe_steps() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("REMOVE_COMBINED")
                .parameters("sourceColumn", "FakeColumn")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("upd-steps-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    let new_step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("REMOVE_COMBINED")
                .parameters("removeCustomValue", "true")
                .parameters("sourceColumn", "FakeColumn")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .update_recipe()
        .name("upd-steps-recipe")
        .steps(new_step)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.name(), "upd-steps-recipe");

    // Describe and verify changes
    let recipe = client
        .describe_recipe()
        .name("upd-steps-recipe")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap();
    assert_eq!(recipe.name(), "upd-steps-recipe");
    assert_eq!(recipe.steps().len(), 1);
}

// Ported from moto: test_databrew_recipes.py::test_update_recipe_description
#[tokio::test]
async fn test_update_recipe_description() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("upd-desc-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .update_recipe()
        .name("upd-desc-recipe")
        .description("NewDescription")
        .send()
        .await
        .unwrap();

    let recipe = client
        .describe_recipe()
        .name("upd-desc-recipe")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap();
    assert_eq!(recipe.description(), Some("NewDescription"));
}

// Ported from moto: test_databrew_recipes.py::test_update_recipe_invalid
#[tokio::test]
async fn test_update_recipe_not_found() {
    let client = make_databrew_client().await;
    let err = client
        .update_recipe()
        .name("NotFound")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_recipes.py::test_publish_recipe
#[tokio::test]
async fn test_publish_recipe_creates_published_version() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("pub-ver-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    // Before publish, describe without version should fail
    let err = client
        .describe_recipe()
        .name("pub-ver-recipe")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());

    // Publish with description
    let pub_resp = client
        .publish_recipe()
        .name("pub-ver-recipe")
        .description("1st desc")
        .send()
        .await
        .unwrap();
    assert_eq!(pub_resp.name(), "pub-ver-recipe");

    // Now describe without version should return 1.0
    let recipe = client
        .describe_recipe()
        .name("pub-ver-recipe")
        .send()
        .await
        .unwrap();
    assert_eq!(recipe.recipe_version(), Some("1.0"));
    assert_eq!(recipe.description(), Some("1st desc"));

    // Publish again
    let pub_resp2 = client
        .publish_recipe()
        .name("pub-ver-recipe")
        .description("2nd desc")
        .send()
        .await
        .unwrap();
    assert_eq!(pub_resp2.name(), "pub-ver-recipe");

    let recipe2 = client
        .describe_recipe()
        .name("pub-ver-recipe")
        .send()
        .await
        .unwrap();
    assert_eq!(recipe2.recipe_version(), Some("2.0"));
    assert_eq!(recipe2.description(), Some("2nd desc"));
}

// Ported from moto: test_databrew_recipes.py::test_publish_recipe_that_does_not_exist
#[tokio::test]
async fn test_publish_recipe_that_does_not_exist() {
    let client = make_databrew_client().await;
    let err = client
        .publish_recipe()
        .name("DoesNotExist")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_recipes.py::test_list_recipe_versions_no_recipe
#[tokio::test]
async fn test_list_recipe_versions_no_recipe() {
    let client = make_databrew_client().await;
    let resp = client
        .list_recipe_versions()
        .name("NotExist")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.recipes().len(), 0);
}

// Ported from moto: test_databrew_recipes.py::test_list_recipe_versions_none_published
#[tokio::test]
async fn test_list_recipe_versions_none_published() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("unpub-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_recipe_versions()
        .name("unpub-recipe")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.recipes().len(), 0);
}

// Ported from moto: test_databrew_recipes.py::test_list_recipe_versions_one_published
#[tokio::test]
async fn test_list_recipe_versions_one_published() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("one-pub-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .publish_recipe()
        .name("one-pub-recipe")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_recipe_versions()
        .name("one-pub-recipe")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.recipes().len(), 1);
    assert_eq!(resp.recipes()[0].recipe_version(), Some("1.0"));
}

// Ported from moto: test_databrew_recipes.py::test_list_recipe_versions_two_published
#[tokio::test]
async fn test_list_recipe_versions_two_published() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("two-pub-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .publish_recipe()
        .name("two-pub-recipe")
        .send()
        .await
        .unwrap();
    client
        .publish_recipe()
        .name("two-pub-recipe")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_recipe_versions()
        .name("two-pub-recipe")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.recipes().len(), 2);
    assert_eq!(resp.recipes()[0].recipe_version(), Some("1.0"));
    assert_eq!(resp.recipes()[1].recipe_version(), Some("2.0"));
}

// Ported from moto: test_databrew_recipes.py::test_delete_recipe_version (LATEST_WORKING before publish)
#[tokio::test]
async fn test_delete_recipe_version_latest_working() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("del-lw-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .delete_recipe_version()
        .name("del-lw-recipe")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap();

    // Should be gone
    let err = client
        .describe_recipe()
        .name("del-lw-recipe")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_recipes.py::test_delete_recipe_version_published
#[tokio::test]
async fn test_delete_recipe_version_published() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("del-pub-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .publish_recipe()
        .name("del-pub-recipe")
        .send()
        .await
        .unwrap();

    // Delete published version 1.0
    client
        .delete_recipe_version()
        .name("del-pub-recipe")
        .recipe_version("1.0")
        .send()
        .await
        .unwrap();

    // LATEST_PUBLISHED should be gone
    let err = client
        .describe_recipe()
        .name("del-pub-recipe")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());

    // Working version (1.1) should still exist
    let recipe = client
        .describe_recipe()
        .name("del-pub-recipe")
        .recipe_version("1.1")
        .send()
        .await
        .unwrap();
    assert_eq!(recipe.recipe_version(), Some("1.1"));
}

// Ported from moto: test_databrew_recipes.py::test_delete_recipe_version_latest_working_after_publish
#[tokio::test]
async fn test_delete_recipe_version_latest_working_after_publish() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("del-lw-after-pub")
        .steps(step)
        .send()
        .await
        .unwrap();

    client
        .publish_recipe()
        .name("del-lw-after-pub")
        .send()
        .await
        .unwrap();

    // Should not be allowed to delete LATEST_WORKING after publish
    let err = client
        .delete_recipe_version()
        .name("del-lw-after-pub")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException") || err_str.contains("not allowed to be deleted"),
        "Expected validation error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_recipes.py::test_delete_recipe_version_unknown_recipe
#[tokio::test]
async fn test_delete_recipe_version_unknown_recipe() {
    let client = make_databrew_client().await;
    let err = client
        .delete_recipe_version()
        .name("Unknown")
        .recipe_version("1.1")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_recipes.py::test_delete_recipe_version_unknown_version
#[tokio::test]
async fn test_delete_recipe_version_unknown_version() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("del-unk-ver")
        .steps(step)
        .send()
        .await
        .unwrap();

    let err = client
        .delete_recipe_version()
        .name("del-unk-ver")
        .recipe_version("1.1")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_recipes.py::test_create_recipe_with_description_and_tags
#[tokio::test]
async fn test_create_recipe_with_description_and_tags() {
    let client = make_databrew_client().await;

    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("REMOVE_COMBINED")
                .parameters("sourceColumn", "FakeColumn")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("desc-tag-recipe")
        .description("Test recipe description")
        .steps(step)
        .tags("env", "test")
        .tags("project", "moto")
        .send()
        .await
        .unwrap();

    let recipe = client
        .describe_recipe()
        .name("desc-tag-recipe")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap();

    assert_eq!(recipe.name(), "desc-tag-recipe");
    assert_eq!(recipe.description(), Some("Test recipe description"));
    let tags = recipe.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("project").map(|s| s.as_str()), Some("moto"));
}

// ============================================================================
// Ported from moto: test_databrew_rulesets.py
// ============================================================================

// Ported from moto: test_databrew_rulesets.py::test_ruleset_list_when_empty
#[tokio::test]
async fn test_ruleset_list_when_empty() {
    let client = make_databrew_client().await;
    let resp = client.list_rulesets().send().await.unwrap();
    assert_eq!(resp.rulesets().len(), 0);
}

// Ported from moto: test_databrew_rulesets.py::test_describe_ruleset
#[tokio::test]
async fn test_describe_ruleset_details() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("Assert values > 0")
        .check_expression(":col1 > :val1")
        .build()
        .unwrap();

    let resp = client
        .create_ruleset()
        .name("desc-rs-details")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/fake-dataset")
        .rules(rule)
        .send()
        .await
        .unwrap();

    let rs = client
        .describe_ruleset()
        .name("desc-rs-details")
        .send()
        .await
        .unwrap();
    assert_eq!(rs.name(), "desc-rs-details");
    assert_eq!(rs.rules().len(), 1);
}

// Ported from moto: test_databrew_rulesets.py::test_describe_ruleset_that_does_not_exist
#[tokio::test]
async fn test_describe_ruleset_that_does_not_exist() {
    let client = make_databrew_client().await;
    let err = client
        .describe_ruleset()
        .name("DoseNotExist")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("EntityNotFoundException") || err_str.contains("not found"),
        "Expected entity not found error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_rulesets.py::test_create_ruleset_that_already_exists
#[tokio::test]
async fn test_create_ruleset_that_already_exists() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("rule-1")
        .check_expression("col > 0")
        .build()
        .unwrap();

    client
        .create_ruleset()
        .name("dup-rs")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/ds")
        .rules(rule.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_ruleset()
        .name("dup-rs")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/ds")
        .rules(rule)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("AlreadyExists") || err_str.contains("already exists"),
        "Expected already exists error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_rulesets.py::test_delete_ruleset
#[tokio::test]
async fn test_delete_ruleset_and_verify() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("rule-1")
        .check_expression("col > 0")
        .build()
        .unwrap();

    client
        .create_ruleset()
        .name("del-rs-verify")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/ds")
        .rules(rule)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_ruleset()
        .name("del-rs-verify")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.name(), "del-rs-verify");

    // Verify gone
    let err = client
        .describe_ruleset()
        .name("del-rs-verify")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("not found"));

    // Delete again should error
    let err = client
        .delete_ruleset()
        .name("del-rs-verify")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("not found"));
}

// Ported from moto: test_databrew_rulesets.py::test_update_ruleset
#[tokio::test]
async fn test_update_ruleset() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("Assert values > 0")
        .check_expression(":col1 > :val1")
        .build()
        .unwrap();

    client
        .create_ruleset()
        .name("upd-rs")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/ds")
        .rules(rule)
        .send()
        .await
        .unwrap();

    let new_rule = aws_sdk_databrew::types::Rule::builder()
        .name("Assert values > 10")
        .check_expression(":col1 > :val1")
        .build()
        .unwrap();

    let resp = client
        .update_ruleset()
        .name("upd-rs")
        .rules(new_rule)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.name(), "upd-rs");

    // Describe and verify
    let rs = client
        .describe_ruleset()
        .name("upd-rs")
        .send()
        .await
        .unwrap();
    assert_eq!(rs.rules().len(), 1);
}

// ============================================================================
// Ported from moto: test_databrew_jobs.py
// ============================================================================

// Ported from moto: test_databrew_jobs.py::test_job_list_when_empty
#[tokio::test]
async fn test_job_list_when_empty() {
    let client = make_databrew_client().await;
    let resp = client.list_jobs().send().await.unwrap();
    assert_eq!(resp.jobs().len(), 0);
}

// Ported from moto: test_databrew_jobs.py::test_create_profile_job_that_already_exists
#[tokio::test]
async fn test_create_profile_job_that_already_exists() {
    let client = make_databrew_client().await;

    let output_loc = aws_sdk_databrew::types::S3Location::builder()
        .bucket("output-bucket")
        .build()
        .unwrap();

    client
        .create_profile_job()
        .name("dup-prof-job")
        .dataset_name("ds1")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .output_location(output_loc.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_profile_job()
        .name("dup-prof-job")
        .dataset_name("ds2")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .output_location(output_loc)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Conflict") || err_str.contains("already exists"),
        "Expected conflict error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_jobs.py::test_create_recipe_job_that_already_exists
#[tokio::test]
async fn test_create_recipe_job_that_already_exists() {
    let client = make_databrew_client().await;

    client
        .create_recipe_job()
        .name("dup-rec-job")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .send()
        .await
        .unwrap();

    let err = client
        .create_recipe_job()
        .name("dup-rec-job")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Conflict") || err_str.contains("already exists"),
        "Expected conflict error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_jobs.py::test_create_recipe_job_with_same_name_as_profile_job
#[tokio::test]
async fn test_create_recipe_job_with_same_name_as_profile_job() {
    let client = make_databrew_client().await;

    let output_loc = aws_sdk_databrew::types::S3Location::builder()
        .bucket("output-bucket")
        .build()
        .unwrap();

    client
        .create_profile_job()
        .name("same-name-job")
        .dataset_name("ds1")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .output_location(output_loc)
        .send()
        .await
        .unwrap();

    let err = client
        .create_recipe_job()
        .name("same-name-job")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Conflict") || err_str.contains("already exists"),
        "Expected conflict error, got: {err_str}"
    );
}

// Ported from moto: test_databrew_jobs.py::test_describe_recipe_job
#[tokio::test]
async fn test_describe_recipe_job() {
    let client = make_databrew_client().await;

    client
        .create_recipe_job()
        .name("desc-rec-job")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .send()
        .await
        .unwrap();

    let job = client
        .describe_job()
        .name("desc-rec-job")
        .send()
        .await
        .unwrap();
    assert_eq!(job.name(), "desc-rec-job");
    assert_eq!(job.r#type().map(|t| t.as_str()), Some("RECIPE"));
    assert!(job.resource_arn().is_some());
}

// Ported from moto: test_databrew_jobs.py::test_describe_job_that_does_not_exist
#[tokio::test]
async fn test_describe_job_that_does_not_exist() {
    let client = make_databrew_client().await;
    let err = client
        .describe_job()
        .name("DoesNotExist")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_jobs.py::test_update_profile_job
#[tokio::test]
async fn test_update_profile_job() {
    let client = make_databrew_client().await;

    let output_loc = aws_sdk_databrew::types::S3Location::builder()
        .bucket("output-bucket")
        .build()
        .unwrap();

    client
        .create_profile_job()
        .name("upd-prof-job")
        .dataset_name("ds1")
        .role_arn("original-role")
        .output_location(output_loc)
        .send()
        .await
        .unwrap();

    let new_output_loc = aws_sdk_databrew::types::S3Location::builder()
        .bucket("new-bucket")
        .build()
        .unwrap();

    let resp = client
        .update_profile_job()
        .name("upd-prof-job")
        .role_arn("new-role-arn-12345678901")
        .output_location(new_output_loc)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.name(), "upd-prof-job");

    let job = client
        .describe_job()
        .name("upd-prof-job")
        .send()
        .await
        .unwrap();
    assert_eq!(job.role_arn(), Some("new-role-arn-12345678901"));
}

// Ported from moto: test_databrew_jobs.py::test_update_recipe_job
#[tokio::test]
async fn test_update_recipe_job() {
    let client = make_databrew_client().await;

    client
        .create_recipe_job()
        .name("upd-rec-job")
        .role_arn("original-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_recipe_job()
        .name("upd-rec-job")
        .role_arn("new-role-arn-12345678901")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.name(), "upd-rec-job");

    let job = client
        .describe_job()
        .name("upd-rec-job")
        .send()
        .await
        .unwrap();
    assert_eq!(job.role_arn(), Some("new-role-arn-12345678901"));
}

// Ported from moto: test_databrew_jobs.py::test_update_profile_job_does_not_exist
#[tokio::test]
async fn test_update_profile_job_does_not_exist() {
    let client = make_databrew_client().await;

    let output_loc = aws_sdk_databrew::types::S3Location::builder()
        .bucket("bucket")
        .build()
        .unwrap();

    let err = client
        .update_profile_job()
        .name("DoesNotExist")
        .role_arn("role")
        .output_location(output_loc)
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_jobs.py::test_update_recipe_job_does_not_exist
#[tokio::test]
async fn test_update_recipe_job_does_not_exist() {
    let client = make_databrew_client().await;
    let err = client
        .update_recipe_job()
        .name("DoesNotExist")
        .role_arn("role")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_jobs.py::test_delete_job
#[tokio::test]
async fn test_delete_job() {
    let client = make_databrew_client().await;

    client
        .create_recipe_job()
        .name("del-job")
        .role_arn("role")
        .send()
        .await
        .unwrap();

    let resp = client.delete_job().name("del-job").send().await.unwrap();
    assert_eq!(resp.name(), "del-job");

    // Verify gone
    let err = client
        .describe_job()
        .name("del-job")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_jobs.py::test_delete_job_does_not_exist
#[tokio::test]
async fn test_delete_job_does_not_exist() {
    let client = make_databrew_client().await;
    let err = client
        .delete_job()
        .name("DoesNotExist")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// Ported from moto: test_databrew_jobs.py::test_list_jobs_recipe_and_profile
#[tokio::test]
async fn test_list_jobs_recipe_and_profile() {
    let client = make_databrew_client().await;

    // Create 2 recipe jobs
    for i in 0..2 {
        client
            .create_recipe_job()
            .name(format!("rec-job-{i}"))
            .role_arn("role")
            .send()
            .await
            .unwrap();
    }

    // Create 1 profile job
    let output_loc = aws_sdk_databrew::types::S3Location::builder()
        .bucket("bucket")
        .build()
        .unwrap();

    client
        .create_profile_job()
        .name("prof-job-0")
        .dataset_name("ds")
        .role_arn("role")
        .output_location(output_loc)
        .send()
        .await
        .unwrap();

    let resp = client.list_jobs().send().await.unwrap();
    assert_eq!(resp.jobs().len(), 3);
}

// Ported from moto: test_databrew_jobs.py::test_list_jobs_dataset_name_filter
#[tokio::test]
async fn test_list_jobs_dataset_name_filter() {
    let client = make_databrew_client().await;

    // Create recipe jobs with specific dataset
    for i in 0..2 {
        client
            .create_recipe_job()
            .name(format!("ds-rec-job-{i}"))
            .role_arn("role")
            .dataset_name("TEST")
            .send()
            .await
            .unwrap();
    }

    // Create recipe job without dataset
    client
        .create_recipe_job()
        .name("no-ds-job")
        .role_arn("role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_jobs()
        .dataset_name("TEST")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.jobs().len(), 2);
}

// Ported from moto: test_databrew_jobs.py::test_list_jobs_project_name_filter
#[tokio::test]
async fn test_list_jobs_project_name_filter() {
    let client = make_databrew_client().await;

    for i in 0..3 {
        client
            .create_recipe_job()
            .name(format!("proj-job-{i}"))
            .role_arn("role")
            .project_name("TEST_PROJECT")
            .send()
            .await
            .unwrap();
    }

    client
        .create_recipe_job()
        .name("no-proj-job")
        .role_arn("role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_jobs()
        .project_name("TEST_PROJECT")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.jobs().len(), 3);
}

// ============================================================================
// Ported from moto: tag tests
// ============================================================================

// Ported from moto: test tags on create_dataset
#[tokio::test]
async fn test_create_dataset_with_tags() {
    let client = make_databrew_client().await;

    let input = aws_sdk_databrew::types::Input::builder()
        .s3_input_definition(
            aws_sdk_databrew::types::S3Location::builder()
                .bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_dataset()
        .name("tagged-ds")
        .input(input)
        .tags("env", "prod")
        .tags("team", "data")
        .send()
        .await
        .unwrap();

    let arn = "arn:aws:databrew:us-east-1:123456789012:dataset/tagged-ds";
    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("data"));
}

// ── SendProjectSessionAction test ───────────────────────────────────

#[tokio::test]
async fn test_send_project_session_action() {
    let client = make_databrew_client().await;

    let resp = client
        .send_project_session_action()
        .name("my-project")
        .preview(true)
        .send()
        .await
        .expect("send_project_session_action should succeed");

    assert_eq!(resp.name(), "my-project");
}

// ── Lifecycle tests ─────────────────────────────────────────────────

#[tokio::test]
async fn test_recipe_lifecycle() {
    let client = make_databrew_client().await;

    // Create
    let step = aws_sdk_databrew::types::RecipeStep::builder()
        .action(
            aws_sdk_databrew::types::RecipeAction::builder()
                .operation("UPPER_CASE")
                .build()
                .unwrap(),
        )
        .build();

    client
        .create_recipe()
        .name("lifecycle-recipe")
        .steps(step)
        .send()
        .await
        .unwrap();

    // Describe (must use LATEST_WORKING since not yet published)
    let desc = client
        .describe_recipe()
        .name("lifecycle-recipe")
        .recipe_version("LATEST_WORKING")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.name(), "lifecycle-recipe");

    // Update
    client
        .update_recipe()
        .name("lifecycle-recipe")
        .description("updated desc")
        .send()
        .await
        .unwrap();

    // Publish
    client
        .publish_recipe()
        .name("lifecycle-recipe")
        .send()
        .await
        .unwrap();

    // List versions (only published versions)
    let versions = client
        .list_recipe_versions()
        .name("lifecycle-recipe")
        .send()
        .await
        .unwrap();
    assert_eq!(versions.recipes().len(), 1);

    // Delete published version
    client
        .delete_recipe_version()
        .name("lifecycle-recipe")
        .recipe_version("1.0")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_schedule_lifecycle() {
    let client = make_databrew_client().await;

    // Create
    client
        .create_schedule()
        .name("lifecycle-schedule")
        .cron_expression("cron(0 12 * * ? *)")
        .send()
        .await
        .unwrap();

    // Describe
    let desc = client
        .describe_schedule()
        .name("lifecycle-schedule")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.name(), "lifecycle-schedule");

    // Update
    client
        .update_schedule()
        .name("lifecycle-schedule")
        .cron_expression("cron(0 6 * * ? *)")
        .send()
        .await
        .unwrap();

    // Verify update
    let desc2 = client
        .describe_schedule()
        .name("lifecycle-schedule")
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.cron_expression(), Some("cron(0 6 * * ? *)"));

    // Delete
    client
        .delete_schedule()
        .name("lifecycle-schedule")
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .describe_schedule()
        .name("lifecycle-schedule")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_ruleset_lifecycle() {
    let client = make_databrew_client().await;

    let rule = aws_sdk_databrew::types::Rule::builder()
        .name("rule-1")
        .check_expression("column_values > 0")
        .build()
        .unwrap();

    // Create
    client
        .create_ruleset()
        .name("lifecycle-ruleset")
        .target_arn("arn:aws:databrew:us-east-1:123456789012:dataset/my-dataset")
        .rules(rule)
        .send()
        .await
        .unwrap();

    // Describe
    let desc = client
        .describe_ruleset()
        .name("lifecycle-ruleset")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.name(), "lifecycle-ruleset");

    // Delete
    client
        .delete_ruleset()
        .name("lifecycle-ruleset")
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .describe_ruleset()
        .name("lifecycle-ruleset")
        .send()
        .await;
    assert!(result.is_err());
}
