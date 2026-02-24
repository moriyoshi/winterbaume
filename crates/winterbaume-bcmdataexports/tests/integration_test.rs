use std::sync::{Arc, Mutex};

use aws_sdk_bcmdataexports::config::BehaviorVersion;
use aws_sdk_bcmdataexports::types::{
    DataQuery, DestinationConfigurations, Export, RefreshCadence, ResourceTag, S3Destination,
    S3OutputConfigurations,
};
use winterbaume_bcmdataexports::BcmDataExportsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_bcmdataexports::Client {
    let mock = MockAws::builder()
        .with_service(BcmDataExportsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmdataexports::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_bcmdataexports::Client::new(&config)
}

fn sample_export(name: &str) -> Export {
    Export::builder()
        .name(name)
        .description("test")
        .data_query(
            DataQuery::builder()
                .query_statement("SELECT * FROM COST_AND_USAGE_REPORT")
                .build()
                .unwrap(),
        )
        .destination_configurations(
            DestinationConfigurations::builder()
                .s3_destination(
                    S3Destination::builder()
                        .s3_bucket("my-bucket")
                        .s3_prefix("exports/")
                        .s3_region("us-east-1")
                        .s3_output_configurations(
                            S3OutputConfigurations::builder()
                                .compression("PARQUET".into())
                                .format("PARQUET".into())
                                .output_type("CUSTOM".into())
                                .overwrite("CREATE_NEW_REPORT".into())
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .refresh_cadence(
            RefreshCadence::builder()
                .frequency("SYNCHRONOUS".into())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_export_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_export()
        .export(sample_export("MyExport"))
        .send()
        .await
        .expect("create");
    let arn = create.export_arn().expect("arn").to_string();
    assert!(arn.contains("export/"));

    let got = client
        .get_export()
        .export_arn(&arn)
        .send()
        .await
        .expect("get");
    let inner = got.export().unwrap();
    assert_eq!(inner.name(), "MyExport");

    let list = client.list_exports().send().await.expect("list");
    assert_eq!(list.exports().len(), 1);

    client
        .update_export()
        .export_arn(&arn)
        .export(sample_export("MyExport"))
        .send()
        .await
        .expect("update");

    client
        .delete_export()
        .export_arn(&arn)
        .send()
        .await
        .expect("delete");
    let err = client
        .get_export()
        .export_arn(&arn)
        .send()
        .await
        .expect_err("gone");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_duplicate_export_name_conflicts() {
    let client = make_client().await;
    client
        .create_export()
        .export(sample_export("Dup"))
        .send()
        .await
        .expect("first");
    let err = client
        .create_export()
        .export(sample_export("Dup"))
        .send()
        .await
        .expect_err("dup");
    assert!(format!("{err:?}").contains("ConflictException"));
}

#[tokio::test]
async fn test_list_executions_empty_for_new_export() {
    let client = make_client().await;
    let create = client
        .create_export()
        .export(sample_export("ExecExport"))
        .send()
        .await
        .expect("create");
    let arn = create.export_arn().unwrap().to_string();
    let resp = client
        .list_executions()
        .export_arn(&arn)
        .send()
        .await
        .expect("list");
    assert!(resp.executions().is_empty());
}

#[tokio::test]
async fn test_list_executions_for_missing_export_404s() {
    let client = make_client().await;
    let err = client
        .list_executions()
        .export_arn("arn:aws:bcm-data-exports:us-east-1:123:export/missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_tables_empty_default() {
    let client = make_client().await;
    let resp = client.list_tables().send().await.expect("list");
    assert!(resp.tables().is_empty());
}

#[tokio::test]
async fn test_get_table_after_seeding() {
    use winterbaume_bcmdataexports::views::{
        BcmDataExportsStateView, TableCatalogueEntryView, TableColumnView,
    };
    let svc = BcmDataExportsService::new();
    let mut view = BcmDataExportsStateView::default();
    view.tables.insert(
        "COST_AND_USAGE_REPORT".to_string(),
        TableCatalogueEntryView {
            name: "COST_AND_USAGE_REPORT".to_string(),
            description: Some("Cost & usage report".to_string()),
            schema: vec![TableColumnView {
                name: "lineItem".to_string(),
                r#type: "STRING".to_string(),
                description: None,
            }],
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("seed");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.tables.len(), 1);
    assert!(snap.tables.contains_key("COST_AND_USAGE_REPORT"));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_export()
        .export(sample_export("Tagged"))
        .send()
        .await
        .expect("create");
    let arn = create.export_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .resource_tags(
            ResourceTag::builder()
                .key("Env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(tags.resource_tags().len(), 1);

    client
        .untag_resource()
        .resource_arn(&arn)
        .resource_tag_keys("Env")
        .send()
        .await
        .expect("untag");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = BcmDataExportsService::new();
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
