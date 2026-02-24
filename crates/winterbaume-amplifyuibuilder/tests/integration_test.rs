use std::sync::{Arc, Mutex};

use aws_sdk_amplifyuibuilder::config::BehaviorVersion;
use aws_sdk_amplifyuibuilder::types::{
    CreateComponentData, CreateFormData, CreateThemeData, FieldConfig, FormDataTypeConfig,
    FormStyle, ThemeValues,
};
use winterbaume_amplifyuibuilder::AmplifyUiBuilderService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_amplifyuibuilder::Client {
    let mock = MockAws::builder()
        .with_service(AmplifyUiBuilderService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplifyuibuilder::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_amplifyuibuilder::Client::new(&config)
}

fn comp_data(name: &str) -> CreateComponentData {
    CreateComponentData::builder()
        .name(name)
        .component_type("Button")
        .properties(
            "label",
            aws_sdk_amplifyuibuilder::types::ComponentProperty::builder().build(),
        )
        .variants(aws_sdk_amplifyuibuilder::types::ComponentVariant::builder().build())
        .overrides(
            "default",
            std::collections::HashMap::<String, String>::new(),
        )
        .binding_properties(
            "default",
            aws_sdk_amplifyuibuilder::types::ComponentBindingPropertiesValue::builder().build(),
        )
        .build()
        .unwrap()
}

fn theme_data(name: &str) -> CreateThemeData {
    CreateThemeData::builder()
        .name(name)
        .values(ThemeValues::builder().build())
        .build()
        .unwrap()
}

fn form_data(name: &str) -> CreateFormData {
    CreateFormData::builder()
        .name(name)
        .data_type(
            FormDataTypeConfig::builder()
                .data_source_type("DataStore".into())
                .data_type_name("MyType")
                .build()
                .unwrap(),
        )
        .form_action_type("create".into())
        .fields("name", FieldConfig::builder().build())
        .sectional_elements(
            "header",
            aws_sdk_amplifyuibuilder::types::SectionalElement::builder()
                .r#type("Heading")
                .build()
                .unwrap(),
        )
        .schema_version("1.0")
        .cta(aws_sdk_amplifyuibuilder::types::FormCta::builder().build())
        .style(FormStyle::builder().build())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_component_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_component()
        .app_id("app1")
        .environment_name("staging")
        .component_to_create(comp_data("MyButton"))
        .send()
        .await
        .expect("create");
    let id = create.entity().unwrap().id();
    assert!(!id.is_empty());

    let got = client
        .get_component()
        .app_id("app1")
        .environment_name("staging")
        .id(id)
        .send()
        .await
        .expect("get");
    assert_eq!(got.component().unwrap().name(), "MyButton");

    let list = client
        .list_components()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("list");
    assert_eq!(list.entities().len(), 1);

    let export = client
        .export_components()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("export");
    assert_eq!(export.entities().len(), 1);

    client
        .delete_component()
        .app_id("app1")
        .environment_name("staging")
        .id(id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_theme_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_theme()
        .app_id("app1")
        .environment_name("staging")
        .theme_to_create(theme_data("MyTheme"))
        .send()
        .await
        .expect("create");
    let id = create.entity().unwrap().id();

    let got = client
        .get_theme()
        .app_id("app1")
        .environment_name("staging")
        .id(id)
        .send()
        .await
        .expect("get");
    assert_eq!(got.theme().unwrap().name(), "MyTheme");

    let list = client
        .list_themes()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("list");
    assert_eq!(list.entities().len(), 1);

    client
        .delete_theme()
        .app_id("app1")
        .environment_name("staging")
        .id(id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_form_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_form()
        .app_id("app1")
        .environment_name("staging")
        .form_to_create(form_data("CreateUser"))
        .send()
        .await
        .expect("create");
    let id = create.entity().unwrap().id();

    let list = client
        .list_forms()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("list");
    assert_eq!(list.entities().len(), 1);

    client
        .delete_form()
        .app_id("app1")
        .environment_name("staging")
        .id(id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_get_component_not_found() {
    let client = make_client().await;
    let err = client
        .get_component()
        .app_id("app1")
        .environment_name("staging")
        .id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_metadata_round_trip() {
    let client = make_client().await;
    client
        .put_metadata_flag()
        .app_id("app1")
        .environment_name("staging")
        .feature_name("MyFeature")
        .body(
            aws_sdk_amplifyuibuilder::types::PutMetadataFlagBody::builder()
                .new_value("enabled")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put metadata");

    let resp = client
        .get_metadata()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("get metadata");
    assert_eq!(
        resp.features().get("MyFeature"),
        Some(&"enabled".to_string())
    );
}

#[tokio::test]
async fn test_codegen_job_lifecycle() {
    let client = make_client().await;
    let start = client
        .start_codegen_job()
        .app_id("app1")
        .environment_name("staging")
        .codegen_job_to_create(
            aws_sdk_amplifyuibuilder::types::StartCodegenJobData::builder()
                .render_config(
                    aws_sdk_amplifyuibuilder::types::CodegenJobRenderConfig::React(
                        aws_sdk_amplifyuibuilder::types::ReactStartCodegenJobData::builder()
                            .build(),
                    ),
                )
                .build(),
        )
        .send()
        .await
        .expect("start");
    let id = start.entity().unwrap().id().to_string();

    let get = client
        .get_codegen_job()
        .app_id("app1")
        .environment_name("staging")
        .id(&id)
        .send()
        .await
        .expect("get");
    assert!(get.job().is_some());

    let list = client
        .list_codegen_jobs()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("list");
    assert_eq!(list.entities().len(), 1);
}

#[tokio::test]
async fn test_exchange_code_for_token_validates_input() {
    let client = make_client().await;
    let err = client
        .exchange_code_for_token()
        .provider("figma".into())
        .request(
            aws_sdk_amplifyuibuilder::types::ExchangeCodeForTokenRequestBody::builder()
                .code("")
                .redirect_uri("https://example.com/cb")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("empty code");
    assert!(format!("{err:?}").contains("ValidationException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AmplifyUiBuilderService::new();
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
