# winterbaume-databrew

AWS Glue DataBrew service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | DataBrew |
| AWS model | `databrew` |
| Protocol | restJson1 |
| winterbaume coverage | 32/44 operations (72.7%) |
| stubs (routed, returns empty/default) | 1/44 operations (2.3%) |
| moto coverage | 24/44 operations (54.5%) |
| floci coverage | 0/44 operations (0.0%) |
| kumo coverage | 0/44 operations (0.0%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws databrew list-projects
```

## Example

```rust
use aws_sdk_databrew::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_databrew::DataBrewService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DataBrewService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_databrew::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_databrew::Client::new(&config);

    let resp = client
        .list_projects()
        .send()
        .await
        .expect("list_projects should succeed");
    println!("DataBrew projects: {}", resp.projects().len());
}
```

## Implemented APIs (32)

- `CreateDataset`
- `CreateProfileJob`
- `CreateRecipe`
- `CreateRecipeJob`
- `CreateRuleset`
- `CreateSchedule`
- `DeleteDataset`
- `DeleteJob`
- `DeleteRecipeVersion`
- `DeleteRuleset`
- `DeleteSchedule`
- `DescribeDataset`
- `DescribeJob`
- `DescribeRecipe`
- `DescribeRuleset`
- `DescribeSchedule`
- `ListDatasets`
- `ListJobs`
- `ListRecipeVersions`
- `ListRecipes`
- `ListRulesets`
- `ListSchedules`
- `ListTagsForResource`
- `PublishRecipe`
- `TagResource`
- `UntagResource`
- `UpdateDataset`
- `UpdateProfileJob`
- `UpdateRecipe`
- `UpdateRecipeJob`
- `UpdateRuleset`
- `UpdateSchedule`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `SendProjectSessionAction`

</details>

<details><summary>Not yet implemented APIs (11)</summary>

- `BatchDeleteRecipeVersion`
- `CreateProject`
- `DeleteProject`
- `DescribeJobRun`
- `DescribeProject`
- `ListJobRuns`
- `ListProjects`
- `StartJobRun`
- `StartProjectSession`
- `StopJobRun`
- `UpdateProject`

</details>
