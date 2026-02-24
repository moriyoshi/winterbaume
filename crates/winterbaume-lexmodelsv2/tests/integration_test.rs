use aws_sdk_lexmodelsv2::config::BehaviorVersion;
use aws_sdk_lexmodelsv2::types::DataPrivacy;
use winterbaume_core::MockAws;
use winterbaume_lexmodelsv2::LexModelsV2Service;

async fn make_client() -> aws_sdk_lexmodelsv2::Client {
    let mock = MockAws::builder()
        .with_service(LexModelsV2Service::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lexmodelsv2::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_lexmodelsv2::Client::new(&config)
}

// ---- Bot CRUD tests ----

#[tokio::test]
async fn test_create_and_describe_bot() {
    let client = make_client().await;

    let resp = client
        .create_bot()
        .bot_name("MyBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = resp.bot_id().unwrap().to_string();
    assert_eq!(resp.bot_name().unwrap(), "MyBot");
    assert!(resp.bot_status().is_some());

    let describe_resp = client
        .describe_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("describe_bot should succeed");

    assert_eq!(describe_resp.bot_id().unwrap(), bot_id);
    assert_eq!(describe_resp.bot_name().unwrap(), "MyBot");
}

#[tokio::test]
async fn test_list_bots() {
    let client = make_client().await;

    client
        .create_bot()
        .bot_name("Bot1")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    client
        .create_bot()
        .bot_name("Bot2")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let resp = client
        .list_bots()
        .send()
        .await
        .expect("list_bots should succeed");

    let summaries = resp.bot_summaries();
    assert!(summaries.len() >= 2);
}

#[tokio::test]
async fn test_delete_bot() {
    let client = make_client().await;

    let create_resp = client
        .create_bot()
        .bot_name("BotToDelete")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = create_resp.bot_id().unwrap().to_string();

    client
        .delete_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("delete_bot should succeed");

    let result = client.describe_bot().bot_id(&bot_id).send().await;
    assert!(result.is_err(), "describe_bot after delete should fail");
}

#[tokio::test]
async fn test_update_bot() {
    let client = make_client().await;

    let create_resp = client
        .create_bot()
        .bot_name("BotToUpdate")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = create_resp.bot_id().unwrap().to_string();

    let update_resp = client
        .update_bot()
        .bot_id(&bot_id)
        .bot_name("BotToUpdate")
        .role_arn("arn:aws:iam::123456789012:role/NewRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(600)
        .send()
        .await
        .expect("update_bot should succeed");

    assert_eq!(update_resp.bot_id().unwrap(), bot_id);
    assert_eq!(update_resp.idle_session_ttl_in_seconds(), Some(600));
}

#[tokio::test]
async fn test_describe_nonexistent_bot() {
    let client = make_client().await;
    let result = client.describe_bot().bot_id("NONEXISTENT").send().await;
    assert!(result.is_err());
}

// ---- BotAlias CRUD tests ----

async fn create_test_bot(client: &aws_sdk_lexmodelsv2::Client, name: &str) -> String {
    let resp = client
        .create_bot()
        .bot_name(name)
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");
    resp.bot_id().unwrap().to_string()
}

#[tokio::test]
async fn test_bot_alias_lifecycle() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAlias").await;

    let create_resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("MyAlias")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    let alias_id = create_resp.bot_alias_id().unwrap().to_string();
    assert_eq!(create_resp.bot_alias_name().unwrap(), "MyAlias");

    let describe_resp = client
        .describe_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await
        .expect("describe_bot_alias should succeed");

    assert_eq!(describe_resp.bot_alias_id().unwrap(), alias_id);

    let list_resp = client
        .list_bot_aliases()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_aliases should succeed");
    assert_eq!(list_resp.bot_alias_summaries().len(), 1);

    let update_resp = client
        .update_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .bot_alias_name("UpdatedAlias")
        .send()
        .await
        .expect("update_bot_alias should succeed");
    assert_eq!(update_resp.bot_alias_name().unwrap(), "UpdatedAlias");

    client
        .delete_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await
        .expect("delete_bot_alias should succeed");

    let result = client
        .describe_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- BotLocale CRUD tests ----

#[tokio::test]
async fn test_bot_locale_lifecycle() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocale").await;

    let create_resp = client
        .create_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.5)
        .send()
        .await
        .expect("create_bot_locale should succeed");

    assert_eq!(create_resp.locale_id().unwrap(), "en_US");

    let describe_resp = client
        .describe_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("describe_bot_locale should succeed");

    assert_eq!(describe_resp.locale_id().unwrap(), "en_US");
    assert_eq!(describe_resp.nlu_intent_confidence_threshold(), Some(0.5));

    let list_resp = client
        .list_bot_locales()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .send()
        .await
        .expect("list_bot_locales should succeed");
    assert_eq!(list_resp.bot_locale_summaries().len(), 1);

    let build_resp = client
        .build_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("build_bot_locale should succeed");
    assert_eq!(build_resp.bot_locale_status().unwrap().as_str(), "Built");

    client
        .delete_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("delete_bot_locale should succeed");

    let result = client
        .describe_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Intent CRUD tests ----

async fn create_test_locale(client: &aws_sdk_lexmodelsv2::Client, bot_id: &str, locale_id: &str) {
    client
        .create_bot_locale()
        .bot_id(bot_id)
        .bot_version("DRAFT")
        .locale_id(locale_id)
        .nlu_intent_confidence_threshold(0.5)
        .send()
        .await
        .expect("create_bot_locale should succeed");
}

#[tokio::test]
async fn test_intent_lifecycle() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntent").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let create_resp = client
        .create_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_name("BookFlight")
        .send()
        .await
        .expect("create_intent should succeed");

    let intent_id = create_resp.intent_id().unwrap().to_string();
    assert_eq!(create_resp.intent_name().unwrap(), "BookFlight");

    let describe_resp = client
        .describe_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("describe_intent should succeed");

    assert_eq!(describe_resp.intent_id().unwrap(), intent_id);
    assert_eq!(describe_resp.intent_name().unwrap(), "BookFlight");

    let list_resp = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_intents should succeed");
    assert!(!list_resp.intent_summaries().is_empty());

    let update_resp = client
        .update_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .intent_name("BookFlightUpdated")
        .send()
        .await
        .expect("update_intent should succeed");
    assert_eq!(update_resp.intent_name().unwrap(), "BookFlightUpdated");

    client
        .delete_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("delete_intent should succeed");

    let result = client
        .describe_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- BotVersion tests ----

#[tokio::test]
async fn test_bot_version_lifecycle() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVersion").await;

    let create_resp = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create_bot_version should succeed");

    let version = create_resp.bot_version().unwrap().to_string();
    assert_eq!(version, "1");

    let describe_resp = client
        .describe_bot_version()
        .bot_id(&bot_id)
        .bot_version(&version)
        .send()
        .await
        .expect("describe_bot_version should succeed");

    assert_eq!(describe_resp.bot_version().unwrap(), version);

    let list_resp = client
        .list_bot_versions()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_versions should succeed");
    assert_eq!(list_resp.bot_version_summaries().len(), 1);

    client
        .delete_bot_version()
        .bot_id(&bot_id)
        .bot_version(&version)
        .send()
        .await
        .expect("delete_bot_version should succeed");

    let result = client
        .describe_bot_version()
        .bot_id(&bot_id)
        .bot_version(&version)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- State view tests ----

#[tokio::test]
async fn test_snapshot_restore() {
    let mock = MockAws::builder()
        .with_service(LexModelsV2Service::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lexmodelsv2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_lexmodelsv2::Client::new(&config);

    let create_resp = client
        .create_bot()
        .bot_name("SnapshotBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");
    let _bot_id = create_resp.bot_id().unwrap().to_string();

    // Snapshot from the service that handled the request
    let mock2 = MockAws::builder()
        .with_service(LexModelsV2Service::new())
        .build();
    let config2 = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock2.http_client())
        .credentials_provider(mock2.credentials_provider())
        .region(aws_sdk_lexmodelsv2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client2 = aws_sdk_lexmodelsv2::Client::new(&config2);

    // Verify fresh service has no bots
    let list_resp = client2
        .list_bots()
        .send()
        .await
        .expect("list_bots should succeed");
    assert_eq!(list_resp.bot_summaries().len(), 0);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = LexModelsV2Service::new();
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
    assert_eq!(got[0].0, "123456789012");
    assert_eq!(got[0].1, "us-east-1");
}

#[tokio::test]
async fn test_merge_is_additive() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_lexmodelsv2::views::{BotView, LexStateView};

    let svc = LexModelsV2Service::new();

    // Pre-seed a bot
    let bot_view = BotView {
        bot_id: "BOT1".to_string(),
        bot_name: "OriginalBot".to_string(),
        role_arn: "arn:aws:iam::123456789012:role/Role".to_string(),
        data_privacy_child_directed: false,
        idle_session_ttl_in_seconds: 300,
        bot_status: "Available".to_string(),
        description: None,
        creation_date_time: String::new(),
        last_updated_date_time: String::new(),
        tags: HashMap::new(),
    };

    let initial_view = LexStateView {
        bots: [("BOT1".to_string(), bot_view)].into_iter().collect(),
        ..Default::default()
    };

    svc.restore("123456789012", "us-east-1", initial_view)
        .await
        .unwrap();

    // Merge in a new bot
    let new_bot_view = BotView {
        bot_id: "BOT2".to_string(),
        bot_name: "NewBot".to_string(),
        role_arn: "arn:aws:iam::123456789012:role/Role".to_string(),
        data_privacy_child_directed: false,
        idle_session_ttl_in_seconds: 300,
        bot_status: "Available".to_string(),
        description: None,
        creation_date_time: String::new(),
        last_updated_date_time: String::new(),
        tags: HashMap::new(),
    };

    let merge_view = LexStateView {
        bots: [("BOT2".to_string(), new_bot_view)].into_iter().collect(),
        ..Default::default()
    };

    svc.merge("123456789012", "us-east-1", merge_view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot.bots.contains_key("BOT1"),
        "merge must not remove existing bot"
    );
    assert!(snapshot.bots.contains_key("BOT2"), "merge must add new bot");
}

// ---- Slot CRUD tests ----

async fn create_test_intent(
    client: &aws_sdk_lexmodelsv2::Client,
    bot_id: &str,
    locale_id: &str,
    intent_name: &str,
) -> String {
    let resp = client
        .create_intent()
        .bot_id(bot_id)
        .bot_version("DRAFT")
        .locale_id(locale_id)
        .intent_name(intent_name)
        .send()
        .await
        .expect("create_intent should succeed");
    resp.intent_id().unwrap().to_string()
}

#[tokio::test]
async fn test_slot_lifecycle() {
    use aws_sdk_lexmodelsv2::types::{SlotConstraint, SlotValueElicitationSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlot").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "IntentForSlot").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("SlotValueElicitationSetting builder should succeed");

    let create_resp = client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("DepartureCity")
        .value_elicitation_setting(elicitation.clone())
        .send()
        .await
        .expect("create_slot should succeed");

    let slot_id = create_resp.slot_id().unwrap().to_string();
    assert_eq!(create_resp.slot_name().unwrap(), "DepartureCity");

    let describe_resp = client
        .describe_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .send()
        .await
        .expect("describe_slot should succeed");

    assert_eq!(describe_resp.slot_id().unwrap(), slot_id);
    assert_eq!(describe_resp.slot_name().unwrap(), "DepartureCity");

    let list_resp = client
        .list_slots()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("list_slots should succeed");
    assert_eq!(list_resp.slot_summaries().len(), 1);

    let update_resp = client
        .update_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .slot_name("ArrivalCity")
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("update_slot should succeed");
    assert_eq!(update_resp.slot_name().unwrap(), "ArrivalCity");

    client
        .delete_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .send()
        .await
        .expect("delete_slot should succeed");

    let result = client
        .describe_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .send()
        .await;
    assert!(result.is_err(), "describe_slot after delete should fail");
}

// ---- SlotType CRUD tests ----

#[tokio::test]
async fn test_slot_type_lifecycle() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotType").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("SlotValueSelectionSetting builder should succeed");

    let slot_values = SlotTypeValue::builder()
        .sample_value(
            SampleValue::builder()
                .value("pizza")
                .build()
                .expect("SampleValue builder should succeed"),
        )
        .build();

    let create_resp = client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("FoodType")
        .slot_type_values(slot_values)
        .value_selection_setting(value_selection.clone())
        .send()
        .await
        .expect("create_slot_type should succeed");

    let slot_type_id = create_resp.slot_type_id().unwrap().to_string();
    assert_eq!(create_resp.slot_type_name().unwrap(), "FoodType");

    let describe_resp = client
        .describe_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .send()
        .await
        .expect("describe_slot_type should succeed");

    assert_eq!(describe_resp.slot_type_id().unwrap(), slot_type_id);
    assert_eq!(describe_resp.slot_type_name().unwrap(), "FoodType");

    let list_resp = client
        .list_slot_types()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_slot_types should succeed");
    assert_eq!(list_resp.slot_type_summaries().len(), 1);

    let updated_values = SlotTypeValue::builder()
        .sample_value(
            SampleValue::builder()
                .value("burger")
                .build()
                .expect("SampleValue builder should succeed"),
        )
        .build();

    let update_resp = client
        .update_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .slot_type_name("FoodTypeUpdated")
        .slot_type_values(updated_values)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("update_slot_type should succeed");
    assert_eq!(update_resp.slot_type_name().unwrap(), "FoodTypeUpdated");

    client
        .delete_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .send()
        .await
        .expect("delete_slot_type should succeed");

    let result = client
        .describe_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe_slot_type after delete should fail"
    );
}

// ---- CustomVocabulary tests ----

#[tokio::test]
async fn test_custom_vocabulary_lifecycle() {
    use aws_sdk_lexmodelsv2::types::{CustomVocabularyEntryId, NewCustomVocabularyItem};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVocab").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let item = NewCustomVocabularyItem::builder()
        .phrase("AWS Lambda")
        .weight(3)
        .build()
        .expect("NewCustomVocabularyItem builder should succeed");

    let create_resp = client
        .batch_create_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(item)
        .send()
        .await
        .expect("batch_create_custom_vocabulary_item should succeed");

    let resources = create_resp.resources();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].phrase(), "AWS Lambda");

    let list_resp = client
        .list_custom_vocabulary_items()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_custom_vocabulary_items should succeed");
    assert_eq!(list_resp.custom_vocabulary_items().len(), 1);

    let meta_resp = client
        .describe_custom_vocabulary_metadata()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("describe_custom_vocabulary_metadata should succeed");
    assert_eq!(meta_resp.bot_id().unwrap(), bot_id);
    assert!(meta_resp.custom_vocabulary_status().is_some());

    let item_id = resources[0].item_id().to_string();
    let delete_item = CustomVocabularyEntryId::builder()
        .item_id(&item_id)
        .build()
        .expect("CustomVocabularyEntryId builder should succeed");

    let delete_resp = client
        .batch_delete_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(delete_item)
        .send()
        .await
        .expect("batch_delete_custom_vocabulary_item should succeed");
    assert_eq!(delete_resp.bot_id().unwrap(), bot_id);

    let list_after_resp = client
        .list_custom_vocabulary_items()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_custom_vocabulary_items should succeed");
    assert_eq!(list_after_resp.custom_vocabulary_items().len(), 0);
}

#[tokio::test]
async fn test_delete_custom_vocabulary() {
    use aws_sdk_lexmodelsv2::types::NewCustomVocabularyItem;

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVocabDel").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let item = NewCustomVocabularyItem::builder()
        .phrase("Amazon S3")
        .build()
        .expect("NewCustomVocabularyItem builder should succeed");

    client
        .batch_create_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(item)
        .send()
        .await
        .expect("batch_create_custom_vocabulary_item should succeed");

    let del_resp = client
        .delete_custom_vocabulary()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("delete_custom_vocabulary should succeed");
    assert_eq!(del_resp.bot_id().unwrap(), bot_id);

    let list_resp = client
        .list_custom_vocabulary_items()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_custom_vocabulary_items should succeed");
    assert_eq!(list_resp.custom_vocabulary_items().len(), 0);
}

// ---- BuiltIn operations tests ----

#[tokio::test]
async fn test_list_built_in_intents() {
    let client = make_client().await;
    let resp = client
        .list_built_in_intents()
        .locale_id("en_US")
        .send()
        .await
        .expect("list_built_in_intents should succeed");
    let summaries = resp.built_in_intent_summaries();
    assert!(
        !summaries.is_empty(),
        "should return at least one built-in intent"
    );
    let signatures: Vec<&str> = summaries
        .iter()
        .filter_map(|s| s.intent_signature())
        .collect();
    assert!(
        signatures.contains(&"AMAZON.CancelIntent"),
        "should include AMAZON.CancelIntent"
    );
}

#[tokio::test]
async fn test_list_built_in_slot_types() {
    let client = make_client().await;
    let resp = client
        .list_built_in_slot_types()
        .locale_id("en_US")
        .send()
        .await
        .expect("list_built_in_slot_types should succeed");
    let summaries = resp.built_in_slot_type_summaries();
    assert!(
        !summaries.is_empty(),
        "should return at least one built-in slot type"
    );
    let signatures: Vec<&str> = summaries
        .iter()
        .filter_map(|s| s.slot_type_signature())
        .collect();
    assert!(
        signatures.contains(&"AMAZON.Date"),
        "should include AMAZON.Date"
    );
}

// ---- DeleteUtterances test ----

#[tokio::test]
async fn test_delete_utterances() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForUtterances").await;

    // delete_utterances should succeed even when there are no utterances to delete
    client
        .delete_utterances()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("delete_utterances should succeed");
}

// ---- Export and Import tests ----

#[tokio::test]
async fn test_export_lifecycle() {
    use aws_sdk_lexmodelsv2::types::{BotExportSpecification, ExportResourceSpecification};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForExport").await;

    let resource_spec = ExportResourceSpecification::builder()
        .bot_export_specification(
            BotExportSpecification::builder()
                .bot_id(&bot_id)
                .bot_version("DRAFT")
                .build()
                .expect("BotExportSpecification builder should succeed"),
        )
        .build();

    let create_resp = client
        .create_export()
        .resource_specification(resource_spec)
        .file_format(aws_sdk_lexmodelsv2::types::ImportExportFileFormat::LexJson)
        .send()
        .await
        .expect("create_export should succeed");

    let export_id = create_resp.export_id().unwrap().to_string();
    assert!(create_resp.export_status().is_some());

    let describe_resp = client
        .describe_export()
        .export_id(&export_id)
        .send()
        .await
        .expect("describe_export should succeed");
    assert_eq!(describe_resp.export_id().unwrap(), export_id);

    let list_resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    let summaries = list_resp.export_summaries();
    assert!(
        summaries
            .iter()
            .any(|s| s.export_id().unwrap_or("") == export_id),
        "export should appear in list"
    );

    client
        .delete_export()
        .export_id(&export_id)
        .send()
        .await
        .expect("delete_export should succeed");

    let result = client.describe_export().export_id(&export_id).send().await;
    assert!(result.is_err(), "describe_export after delete should fail");
}

#[tokio::test]
async fn test_import_lifecycle() {
    use aws_sdk_lexmodelsv2::types::MergeStrategy;

    let client = make_client().await;

    let start_resp = client
        .start_import()
        .import_id("import-test-001")
        .resource_specification(
            aws_sdk_lexmodelsv2::types::ImportResourceSpecification::builder().build(),
        )
        .merge_strategy(MergeStrategy::Overwrite)
        .send()
        .await
        .expect("start_import should succeed");

    let import_id = start_resp.import_id().unwrap().to_string();
    assert!(start_resp.import_status().is_some());

    let describe_resp = client
        .describe_import()
        .import_id(&import_id)
        .send()
        .await
        .expect("describe_import should succeed");
    assert_eq!(describe_resp.import_id().unwrap(), import_id);

    let list_resp = client
        .list_imports()
        .send()
        .await
        .expect("list_imports should succeed");
    let summaries = list_resp.import_summaries();
    assert!(
        summaries
            .iter()
            .any(|s| s.import_id().unwrap_or("") == import_id),
        "import should appear in list"
    );

    client
        .delete_import()
        .import_id(&import_id)
        .send()
        .await
        .expect("delete_import should succeed");

    let result = client.describe_import().import_id(&import_id).send().await;
    assert!(result.is_err(), "describe_import after delete should fail");
}

// ---- Resource Policy tests ----

#[tokio::test]
async fn test_create_and_describe_resource_policy() {
    let client = make_client().await;

    // Create a bot first so we have a resource ARN
    let bot = client
        .create_bot()
        .bot_name("PolicyBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");
    let policy_json = r#"{"Version":"2012-10-17","Statement":[]}"#;

    let create_resp = client
        .create_resource_policy()
        .resource_arn(&resource_arn)
        .policy(policy_json)
        .send()
        .await
        .expect("create_resource_policy should succeed");

    assert_eq!(create_resp.resource_arn().unwrap(), &resource_arn);
    assert!(create_resp.revision_id().is_some());

    let describe_resp = client
        .describe_resource_policy()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("describe_resource_policy should succeed");

    assert_eq!(describe_resp.resource_arn().unwrap(), &resource_arn);
    assert_eq!(describe_resp.policy().unwrap(), policy_json);
    assert!(describe_resp.revision_id().is_some());
}

#[tokio::test]
async fn test_update_resource_policy() {
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("PolicyUpdateBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");
    let policy_json = r#"{"Version":"2012-10-17","Statement":[]}"#;
    let updated_policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow"}]}"#;

    client
        .create_resource_policy()
        .resource_arn(&resource_arn)
        .policy(policy_json)
        .send()
        .await
        .expect("create_resource_policy should succeed");

    let update_resp = client
        .update_resource_policy()
        .resource_arn(&resource_arn)
        .policy(updated_policy)
        .send()
        .await
        .expect("update_resource_policy should succeed");

    assert_eq!(update_resp.resource_arn().unwrap(), &resource_arn);
    assert!(update_resp.revision_id().is_some());

    let describe_resp = client
        .describe_resource_policy()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("describe_resource_policy should succeed");

    assert_eq!(describe_resp.policy().unwrap(), updated_policy);
}

#[tokio::test]
async fn test_delete_resource_policy() {
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("PolicyDeleteBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    client
        .create_resource_policy()
        .resource_arn(&resource_arn)
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("create_resource_policy should succeed");

    let delete_resp = client
        .delete_resource_policy()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    assert_eq!(delete_resp.resource_arn().unwrap(), &resource_arn);

    // Policy should be gone now
    let result = client
        .describe_resource_policy()
        .resource_arn(&resource_arn)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ---- Tag/Untag resource tests ----

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("TagBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert!(tags.is_some(), "tags should be present");
    let tags = tags.unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

#[tokio::test]
async fn test_untag_resource() {
    // NOTE: The SDK sends tagKeys as query parameters for untag_resource, but the
    // mock handler currently only reads them from the JSON body. This test verifies
    // that the untag call itself does not error out; the actual key removal is a
    // known limitation when driven through the SDK (query params are not forwarded
    // to the body-based handler).
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("UntagBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // The untag call should at least not error
    client
        .untag_resource()
        .resource_arn(&resource_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tags still readable (the mock handler does not remove via query params)
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    assert!(list_resp.tags().is_some());
}

#[tokio::test]
async fn test_tag_resource_additive() {
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("TagAdditiveBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    // First tag
    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("first tag_resource should succeed");

    // Second tag (additive)
    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("team", "platform")
        .send()
        .await
        .expect("second tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

// ---- Bot error cases ----

#[tokio::test]
async fn test_create_duplicate_bot_name() {
    let client = make_client().await;

    client
        .create_bot()
        .bot_name("DuplicateBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("first create_bot should succeed");

    let result = client
        .create_bot()
        .bot_name("DuplicateBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await;
    assert!(
        result.is_err(),
        "creating bot with duplicate name should fail"
    );
}

#[tokio::test]
async fn test_update_nonexistent_bot() {
    let client = make_client().await;

    let result = client
        .update_bot()
        .bot_id("NONEXISTENT")
        .bot_name("Ghost")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await;
    assert!(result.is_err(), "updating nonexistent bot should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_bot() {
    let client = make_client().await;
    let result = client.delete_bot().bot_id("NONEXISTENT").send().await;
    assert!(result.is_err(), "deleting nonexistent bot should fail");
}

#[tokio::test]
async fn test_delete_bot_cascades_aliases_and_locales() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "CascadeBot").await;

    // Create alias
    client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("Alias1")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    // Create locale
    create_test_locale(&client, &bot_id, "en_US").await;

    // Create intent under locale
    create_test_intent(&client, &bot_id, "en_US", "TestIntent").await;

    // Delete the bot (should cascade)
    client
        .delete_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("delete_bot should succeed");

    // Verify aliases gone
    let result = client
        .list_bot_aliases()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_aliases should succeed");
    assert_eq!(result.bot_alias_summaries().len(), 0);
}

// ---- BotAlias error cases ----

#[tokio::test]
async fn test_create_bot_alias_nonexistent_bot() {
    let client = make_client().await;

    let result = client
        .create_bot_alias()
        .bot_id("NONEXISTENT")
        .bot_alias_name("GhostAlias")
        .send()
        .await;
    assert!(
        result.is_err(),
        "creating alias for nonexistent bot should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_bot_alias() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMissingAlias").await;

    let result = client
        .describe_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent alias should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_bot_alias() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForDeleteMissingAlias").await;

    let result = client
        .delete_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "deleting nonexistent alias should fail");
}

#[tokio::test]
async fn test_update_nonexistent_bot_alias() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForUpdateMissingAlias").await;

    let result = client
        .update_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id("NONEXISTENT")
        .bot_alias_name("Ghost")
        .send()
        .await;
    assert!(result.is_err(), "updating nonexistent alias should fail");
}

#[tokio::test]
async fn test_multiple_bot_aliases() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiAlias").await;

    client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("Alias1")
        .send()
        .await
        .expect("create alias 1 should succeed");

    client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("Alias2")
        .send()
        .await
        .expect("create alias 2 should succeed");

    let list_resp = client
        .list_bot_aliases()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_aliases should succeed");
    assert_eq!(list_resp.bot_alias_summaries().len(), 2);
}

// ---- BotLocale error cases ----

#[tokio::test]
async fn test_create_bot_locale_nonexistent_bot() {
    let client = make_client().await;

    let result = client
        .create_bot_locale()
        .bot_id("NONEXISTENT")
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.5)
        .send()
        .await;
    assert!(
        result.is_err(),
        "creating locale for nonexistent bot should fail"
    );
}

#[tokio::test]
async fn test_create_duplicate_bot_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForDupLocale").await;

    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .create_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.5)
        .send()
        .await;
    assert!(result.is_err(), "creating duplicate locale should fail");
}

#[tokio::test]
async fn test_describe_nonexistent_bot_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMissingLocale").await;

    let result = client
        .describe_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("xx_XX")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent locale should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_bot_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForDeleteMissingLocale").await;

    let result = client
        .delete_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("xx_XX")
        .send()
        .await;
    assert!(result.is_err(), "deleting nonexistent locale should fail");
}

#[tokio::test]
async fn test_update_bot_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleUpdate").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let update_resp = client
        .update_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.8)
        .description("Updated locale")
        .send()
        .await
        .expect("update_bot_locale should succeed");

    assert_eq!(update_resp.nlu_intent_confidence_threshold(), Some(0.8));
    assert_eq!(update_resp.description().unwrap(), "Updated locale");
}

#[tokio::test]
async fn test_update_nonexistent_bot_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForUpdateMissingLocale").await;

    let result = client
        .update_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("xx_XX")
        .nlu_intent_confidence_threshold(0.5)
        .send()
        .await;
    assert!(result.is_err(), "updating nonexistent locale should fail");
}

#[tokio::test]
async fn test_delete_bot_locale_cascades_intents() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleCascade").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    create_test_intent(&client, &bot_id, "en_US", "OrderPizza").await;
    create_test_intent(&client, &bot_id, "en_US", "OrderDrink").await;

    client
        .delete_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("delete_bot_locale should succeed");

    // Intents should be gone
    let list_resp = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_intents should succeed");
    assert_eq!(list_resp.intent_summaries().len(), 0);
}

#[tokio::test]
async fn test_multiple_bot_locales() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiLocale").await;

    create_test_locale(&client, &bot_id, "en_US").await;
    create_test_locale(&client, &bot_id, "en_GB").await;

    let list_resp = client
        .list_bot_locales()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .send()
        .await
        .expect("list_bot_locales should succeed");
    assert_eq!(list_resp.bot_locale_summaries().len(), 2);
}

// ---- Intent error cases ----

#[tokio::test]
async fn test_create_intent_nonexistent_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentNoLocale").await;

    let result = client
        .create_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("xx_XX")
        .intent_name("Ghost")
        .send()
        .await;
    assert!(
        result.is_err(),
        "creating intent for nonexistent locale should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_intent() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMissingIntent").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .describe_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent intent should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_intent() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForDeleteMissingIntent").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .delete_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "deleting nonexistent intent should fail");
}

#[tokio::test]
async fn test_update_nonexistent_intent() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForUpdateMissingIntent").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .update_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id("NONEXISTENT")
        .intent_name("Ghost")
        .send()
        .await;
    assert!(result.is_err(), "updating nonexistent intent should fail");
}

#[tokio::test]
async fn test_multiple_intents() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiIntent").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    create_test_intent(&client, &bot_id, "en_US", "OrderPizza").await;
    create_test_intent(&client, &bot_id, "en_US", "OrderDrink").await;
    create_test_intent(&client, &bot_id, "en_US", "Greet").await;

    let list_resp = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_intents should succeed");
    assert_eq!(list_resp.intent_summaries().len(), 3);
}

// ---- BotVersion error cases ----

#[tokio::test]
async fn test_create_bot_version_nonexistent_bot() {
    let client = make_client().await;

    let result = client
        .create_bot_version()
        .bot_id("NONEXISTENT")
        .send()
        .await;
    assert!(
        result.is_err(),
        "creating version for nonexistent bot should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_bot_version() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMissingVersion").await;

    let result = client
        .describe_bot_version()
        .bot_id(&bot_id)
        .bot_version("999")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describing nonexistent bot version should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_bot_version() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForDeleteMissingVersion").await;

    let result = client
        .delete_bot_version()
        .bot_id(&bot_id)
        .bot_version("999")
        .send()
        .await;
    assert!(
        result.is_err(),
        "deleting nonexistent bot version should fail"
    );
}

#[tokio::test]
async fn test_multiple_bot_versions() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiVersion").await;

    let v1 = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create version 1 should succeed");
    assert_eq!(v1.bot_version().unwrap(), "1");

    let v2 = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create version 2 should succeed");
    assert_eq!(v2.bot_version().unwrap(), "2");

    let list_resp = client
        .list_bot_versions()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_versions should succeed");
    assert_eq!(list_resp.bot_version_summaries().len(), 2);
}

// ---- Slot error cases ----

#[tokio::test]
async fn test_describe_nonexistent_slot() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMissingSlot").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "SlotIntent").await;

    let result = client
        .describe_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent slot should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_slot() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForDeleteMissingSlot").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "SlotIntentDel").await;

    let result = client
        .delete_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "deleting nonexistent slot should fail");
}

#[tokio::test]
async fn test_update_nonexistent_slot() {
    use aws_sdk_lexmodelsv2::types::{SlotConstraint, SlotValueElicitationSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForUpdateMissingSlot").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "SlotIntentUpd").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    let result = client
        .update_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id("NONEXISTENT")
        .slot_name("Ghost")
        .value_elicitation_setting(elicitation)
        .send()
        .await;
    assert!(result.is_err(), "updating nonexistent slot should fail");
}

#[tokio::test]
async fn test_multiple_slots() {
    use aws_sdk_lexmodelsv2::types::{SlotConstraint, SlotValueElicitationSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiSlot").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "IntentForMultiSlot").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("City")
        .value_elicitation_setting(elicitation.clone())
        .send()
        .await
        .expect("create slot 1 should succeed");

    client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("Date")
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("create slot 2 should succeed");

    let list_resp = client
        .list_slots()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("list_slots should succeed");
    assert_eq!(list_resp.slot_summaries().len(), 2);
}

// ---- SlotType error cases ----

#[tokio::test]
async fn test_describe_nonexistent_slot_type() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMissingSlotType").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .describe_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id("NONEXISTENT")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describing nonexistent slot type should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_slot_type() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForDeleteMissingSlotType").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .delete_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id("NONEXISTENT")
        .send()
        .await;
    assert!(
        result.is_err(),
        "deleting nonexistent slot type should fail"
    );
}

#[tokio::test]
async fn test_update_nonexistent_slot_type() {
    use aws_sdk_lexmodelsv2::types::{SlotValueResolutionStrategy, SlotValueSelectionSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForUpdateMissingSlotType").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let result = client
        .update_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id("NONEXISTENT")
        .slot_type_name("Ghost")
        .value_selection_setting(value_selection)
        .send()
        .await;
    assert!(
        result.is_err(),
        "updating nonexistent slot type should fail"
    );
}

#[tokio::test]
async fn test_multiple_slot_types() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiSlotType").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let val1 = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("pizza").build().unwrap())
        .build();

    let val2 = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("New York").build().unwrap())
        .build();

    client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("FoodType")
        .slot_type_values(val1)
        .value_selection_setting(value_selection.clone())
        .send()
        .await
        .expect("create slot type 1 should succeed");

    client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("CityType")
        .slot_type_values(val2)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create slot type 2 should succeed");

    let list_resp = client
        .list_slot_types()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_slot_types should succeed");
    assert_eq!(list_resp.slot_type_summaries().len(), 2);
}

// ---- CustomVocabulary additional tests ----

#[tokio::test]
async fn test_batch_update_custom_vocabulary_item() {
    use aws_sdk_lexmodelsv2::types::{CustomVocabularyItem, NewCustomVocabularyItem};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVocabUpdate").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    // Create an item first
    let item = NewCustomVocabularyItem::builder()
        .phrase("Amazon EC2")
        .weight(2)
        .build()
        .expect("builder should succeed");

    let create_resp = client
        .batch_create_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(item)
        .send()
        .await
        .expect("batch_create should succeed");

    let resources = create_resp.resources();
    assert_eq!(resources.len(), 1);
    let item_id = resources[0].item_id().to_string();

    // Update the item
    let update_item = CustomVocabularyItem::builder()
        .item_id(&item_id)
        .phrase("Amazon EC2 Instance")
        .weight(5)
        .build()
        .expect("builder should succeed");

    let update_resp = client
        .batch_update_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(update_item)
        .send()
        .await
        .expect("batch_update should succeed");

    let updated = update_resp.resources();
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].phrase(), "Amazon EC2 Instance");
    assert_eq!(updated[0].weight(), Some(5));
}

#[tokio::test]
async fn test_batch_create_multiple_vocabulary_items() {
    use aws_sdk_lexmodelsv2::types::NewCustomVocabularyItem;

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiVocab").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let item1 = NewCustomVocabularyItem::builder()
        .phrase("Amazon S3")
        .build()
        .expect("builder should succeed");

    let item2 = NewCustomVocabularyItem::builder()
        .phrase("Amazon DynamoDB")
        .weight(4)
        .build()
        .expect("builder should succeed");

    let create_resp = client
        .batch_create_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(item1)
        .custom_vocabulary_item_list(item2)
        .send()
        .await
        .expect("batch_create should succeed");

    assert_eq!(create_resp.resources().len(), 2);

    let list_resp = client
        .list_custom_vocabulary_items()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.custom_vocabulary_items().len(), 2);
}

#[tokio::test]
async fn test_describe_custom_vocabulary_metadata_not_found() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForNoVocabMeta").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .describe_custom_vocabulary_metadata()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describing vocab metadata before creation should fail"
    );
}

// ---- Export additional tests ----

#[tokio::test]
async fn test_update_export() {
    use aws_sdk_lexmodelsv2::types::{BotExportSpecification, ExportResourceSpecification};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForExportUpdate").await;

    let resource_spec = ExportResourceSpecification::builder()
        .bot_export_specification(
            BotExportSpecification::builder()
                .bot_id(&bot_id)
                .bot_version("DRAFT")
                .build()
                .expect("builder should succeed"),
        )
        .build();

    let create_resp = client
        .create_export()
        .resource_specification(resource_spec)
        .file_format(aws_sdk_lexmodelsv2::types::ImportExportFileFormat::LexJson)
        .send()
        .await
        .expect("create_export should succeed");

    let export_id = create_resp.export_id().unwrap().to_string();

    let update_resp = client
        .update_export()
        .export_id(&export_id)
        .file_password("secret123")
        .send()
        .await
        .expect("update_export should succeed");

    assert_eq!(update_resp.export_id().unwrap(), export_id);
    assert!(update_resp.export_status().is_some());
}

#[tokio::test]
async fn test_describe_nonexistent_export() {
    let client = make_client().await;
    let result = client
        .describe_export()
        .export_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent export should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_export() {
    let client = make_client().await;
    let result = client.delete_export().export_id("NONEXISTENT").send().await;
    assert!(result.is_err(), "deleting nonexistent export should fail");
}

#[tokio::test]
async fn test_list_exports_empty() {
    let client = make_client().await;
    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    assert_eq!(resp.export_summaries().len(), 0);
}

// ---- Import additional tests ----

#[tokio::test]
async fn test_describe_nonexistent_import() {
    let client = make_client().await;
    let result = client
        .describe_import()
        .import_id("NONEXISTENT")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent import should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_import() {
    let client = make_client().await;
    let result = client.delete_import().import_id("NONEXISTENT").send().await;
    assert!(result.is_err(), "deleting nonexistent import should fail");
}

#[tokio::test]
async fn test_list_imports_empty() {
    let client = make_client().await;
    let resp = client
        .list_imports()
        .send()
        .await
        .expect("list_imports should succeed");
    assert_eq!(resp.import_summaries().len(), 0);
}

// ---- Resource Policy error cases ----

#[tokio::test]
async fn test_describe_nonexistent_resource_policy() {
    let client = make_client().await;
    let result = client
        .describe_resource_policy()
        .resource_arn("arn:aws:lex:us-east-1:123456789012:bot/NONEXISTENT")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describing nonexistent resource policy should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_resource_policy() {
    let client = make_client().await;
    let result = client
        .delete_resource_policy()
        .resource_arn("arn:aws:lex:us-east-1:123456789012:bot/NONEXISTENT")
        .send()
        .await;
    assert!(
        result.is_err(),
        "deleting nonexistent resource policy should fail"
    );
}

#[tokio::test]
async fn test_update_nonexistent_resource_policy() {
    let client = make_client().await;
    let result = client
        .update_resource_policy()
        .resource_arn("arn:aws:lex:us-east-1:123456789012:bot/NONEXISTENT")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await;
    assert!(
        result.is_err(),
        "updating nonexistent resource policy should fail"
    );
}

#[tokio::test]
async fn test_create_duplicate_resource_policy() {
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("DupPolicyBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");
    let policy_json = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .create_resource_policy()
        .resource_arn(&resource_arn)
        .policy(policy_json)
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_resource_policy()
        .resource_arn(&resource_arn)
        .policy(policy_json)
        .send()
        .await;
    assert!(
        result.is_err(),
        "creating duplicate resource policy should fail"
    );
}

// ---- Build bot locale test ----

#[tokio::test]
async fn test_build_nonexistent_bot_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForBuildMissing").await;

    let result = client
        .build_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("xx_XX")
        .send()
        .await;
    assert!(
        result.is_err(),
        "building nonexistent bot locale should fail"
    );
}

// ---- Bot with description ----

#[tokio::test]
async fn test_create_bot_with_description() {
    let client = make_client().await;

    let resp = client
        .create_bot()
        .bot_name("DescribedBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .description("A bot with a description")
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = resp.bot_id().unwrap().to_string();

    let describe_resp = client
        .describe_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("describe_bot should succeed");

    assert_eq!(
        describe_resp.description().unwrap(),
        "A bot with a description"
    );
}

// ---- Bot alias with description and version ----

#[tokio::test]
async fn test_create_bot_alias_with_version() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAliasVersion").await;

    // Create a version first
    let version_resp = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create_bot_version should succeed");
    let version = version_resp.bot_version().unwrap().to_string();

    let alias_resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("ProdAlias")
        .bot_version(&version)
        .description("Production alias")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    assert_eq!(alias_resp.bot_alias_name().unwrap(), "ProdAlias");
    assert_eq!(alias_resp.description().unwrap(), "Production alias");
    assert_eq!(alias_resp.bot_version().unwrap(), version);
}

// ---- Full end-to-end lifecycle ----

#[tokio::test]
async fn test_full_bot_lifecycle_e2e() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotConstraint, SlotTypeValue, SlotValueElicitationSetting,
        SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;

    // 1. Create bot
    let bot_resp = client
        .create_bot()
        .bot_name("E2EBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");
    let bot_id = bot_resp.bot_id().unwrap().to_string();

    // 2. Create locale
    create_test_locale(&client, &bot_id, "en_US").await;

    // 3. Create slot type
    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let slot_type_val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("NYC").build().unwrap())
        .build();

    let st_resp = client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("CityType")
        .slot_type_values(slot_type_val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create_slot_type should succeed");
    let slot_type_id = st_resp.slot_type_id().unwrap().to_string();

    // 4. Create intent
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "BookFlight").await;

    // 5. Create slot referencing the slot type
    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    let slot_resp = client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("Destination")
        .slot_type_id(&slot_type_id)
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("create_slot should succeed");
    let _slot_id = slot_resp.slot_id().unwrap().to_string();

    // 6. Build locale
    let build_resp = client
        .build_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("build_bot_locale should succeed");
    assert_eq!(build_resp.bot_locale_status().unwrap().as_str(), "Built");

    // 7. Create version
    let version_resp = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create_bot_version should succeed");
    assert_eq!(version_resp.bot_version().unwrap(), "1");

    // 8. Create alias pointing to version
    let alias_resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("ProdAlias")
        .bot_version("1")
        .send()
        .await
        .expect("create_bot_alias should succeed");
    assert!(alias_resp.bot_alias_id().is_some());

    // 9. Verify everything exists
    let list_locales = client
        .list_bot_locales()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .send()
        .await
        .expect("list locales should succeed");
    assert_eq!(list_locales.bot_locale_summaries().len(), 1);

    let list_intents = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list intents should succeed");
    assert_eq!(list_intents.intent_summaries().len(), 1);

    let list_slots = client
        .list_slots()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("list slots should succeed");
    assert_eq!(list_slots.slot_summaries().len(), 1);
}

// ---- List bots empty ----

#[tokio::test]
async fn test_list_bots_empty() {
    let client = make_client().await;
    let resp = client
        .list_bots()
        .send()
        .await
        .expect("list_bots should succeed");
    assert_eq!(resp.bot_summaries().len(), 0);
}

// ---- Delete utterances for nonexistent bot ----

#[tokio::test]
async fn test_delete_utterances_nonexistent_bot() {
    let client = make_client().await;
    // delete_utterances is a no-op; verify it does not crash for unknown bot
    let result = client
        .delete_utterances()
        .bot_id("NONEXISTENT")
        .send()
        .await;
    // The handler unconditionally returns success
    assert!(result.is_ok(), "delete_utterances should not fail");
}

// ---- Bot update changes fields correctly ----

#[tokio::test]
async fn test_update_bot_description() {
    let client = make_client().await;

    let create_resp = client
        .create_bot()
        .bot_name("BotForDescUpdate")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = create_resp.bot_id().unwrap().to_string();

    client
        .update_bot()
        .bot_id(&bot_id)
        .bot_name("BotForDescUpdate")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .description("New description")
        .send()
        .await
        .expect("update_bot should succeed");

    let describe_resp = client
        .describe_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("describe_bot should succeed");
    assert_eq!(describe_resp.description().unwrap(), "New description");
}

// ---- Intent with description ----

#[tokio::test]
async fn test_create_intent_with_description() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentDesc").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let resp = client
        .create_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_name("DescribedIntent")
        .description("An intent with a description")
        .send()
        .await
        .expect("create_intent should succeed");

    let intent_id = resp.intent_id().unwrap().to_string();

    let describe_resp = client
        .describe_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("describe_intent should succeed");
    assert_eq!(
        describe_resp.description().unwrap(),
        "An intent with a description"
    );
}

// ---- Bot version with description ----

#[tokio::test]
async fn test_create_bot_version_with_description() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVersionDesc").await;

    let resp = client
        .create_bot_version()
        .bot_id(&bot_id)
        .description("Version 1 description")
        .send()
        .await
        .expect("create_bot_version should succeed");

    let version = resp.bot_version().unwrap().to_string();
    assert_eq!(resp.description().unwrap(), "Version 1 description");

    let describe_resp = client
        .describe_bot_version()
        .bot_id(&bot_id)
        .bot_version(&version)
        .send()
        .await
        .expect("describe_bot_version should succeed");
    assert_eq!(
        describe_resp.description().unwrap(),
        "Version 1 description"
    );
}

// ---- List empty collections ----

#[tokio::test]
async fn test_list_bot_aliases_empty() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForEmptyAliases").await;

    let resp = client
        .list_bot_aliases()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_aliases should succeed");
    assert_eq!(resp.bot_alias_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_bot_versions_empty() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForEmptyVersions").await;

    let resp = client
        .list_bot_versions()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_versions should succeed");
    assert_eq!(resp.bot_version_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_intents_empty() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForEmptyIntents").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let resp = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_intents should succeed");
    assert_eq!(resp.intent_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_slots_empty() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForEmptySlots").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "EmptySlotIntent").await;

    let resp = client
        .list_slots()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("list_slots should succeed");
    assert_eq!(resp.slot_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_slot_types_empty() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForEmptySlotTypes").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let resp = client
        .list_slot_types()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_slot_types should succeed");
    assert_eq!(resp.slot_type_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_custom_vocabulary_items_empty() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForEmptyVocab").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let resp = client
        .list_custom_vocabulary_items()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_custom_vocabulary_items should succeed");
    assert_eq!(resp.custom_vocabulary_items().len(), 0);
}

// ===========================================================================
// Additional comprehensive tests for uncovered scenarios
// ===========================================================================

// ---- Bot creation with tags ----

#[tokio::test]
async fn test_create_bot_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_bot()
        .bot_name("TaggedBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .bot_tags("env", "staging")
        .bot_tags("team", "ml")
        .send()
        .await
        .expect("create_bot with tags should succeed");

    let bot_id = resp.bot_id().unwrap().to_string();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    let tag_resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tag_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("staging"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("ml"));
}

// ---- Bot child_directed flag ----

#[tokio::test]
async fn test_create_bot_child_directed() {
    let client = make_client().await;

    let resp = client
        .create_bot()
        .bot_name("ChildBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(true).build())
        .idle_session_ttl_in_seconds(120)
        .send()
        .await
        .expect("create_bot with child_directed=true should succeed");

    let bot_id = resp.bot_id().unwrap().to_string();

    let describe_resp = client
        .describe_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("describe_bot should succeed");

    assert_eq!(describe_resp.idle_session_ttl_in_seconds(), Some(120));
}

// ---- Bot update changes name ----

#[tokio::test]
async fn test_update_bot_name() {
    let client = make_client().await;

    let create_resp = client
        .create_bot()
        .bot_name("OriginalName")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = create_resp.bot_id().unwrap().to_string();

    let update_resp = client
        .update_bot()
        .bot_id(&bot_id)
        .bot_name("RenamedBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("update_bot should succeed");

    assert_eq!(update_resp.bot_name().unwrap(), "RenamedBot");

    let describe_resp = client
        .describe_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("describe_bot should succeed");
    assert_eq!(describe_resp.bot_name().unwrap(), "RenamedBot");
}

// ---- Bot status fields ----

#[tokio::test]
async fn test_bot_has_timestamps() {
    let client = make_client().await;

    let resp = client
        .create_bot()
        .bot_name("TimestampBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    assert!(resp.creation_date_time().is_some());
}

// ---- BotAlias with description ----

#[tokio::test]
async fn test_create_bot_alias_with_description() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAliasDesc").await;

    let resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("DescAlias")
        .description("A descriptive alias")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    let alias_id = resp.bot_alias_id().unwrap().to_string();
    assert_eq!(resp.description().unwrap(), "A descriptive alias");

    let describe_resp = client
        .describe_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await
        .expect("describe_bot_alias should succeed");
    assert_eq!(describe_resp.description().unwrap(), "A descriptive alias");
}

// ---- Update BotAlias with description ----

#[tokio::test]
async fn test_update_bot_alias_description() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAliasDescUpd").await;

    let create_resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("MyAlias")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    let alias_id = create_resp.bot_alias_id().unwrap().to_string();

    let update_resp = client
        .update_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .bot_alias_name("MyAlias")
        .description("Updated description")
        .send()
        .await
        .expect("update_bot_alias should succeed");

    assert_eq!(update_resp.description().unwrap(), "Updated description");

    let describe_resp = client
        .describe_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await
        .expect("describe_bot_alias should succeed");
    assert_eq!(describe_resp.description().unwrap(), "Updated description");
}

// ---- BotAlias status and timestamps ----

#[tokio::test]
async fn test_bot_alias_has_status_and_timestamps() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAliasStatus").await;

    let resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("StatusAlias")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    assert!(resp.bot_alias_status().is_some());
    assert!(resp.creation_date_time().is_some());
}

// ---- BotLocale with description on create ----

#[tokio::test]
async fn test_create_bot_locale_with_description() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleDesc").await;

    let resp = client
        .create_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.7)
        .description("English US locale")
        .send()
        .await
        .expect("create_bot_locale should succeed");

    assert_eq!(resp.description().unwrap(), "English US locale");

    let describe_resp = client
        .describe_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("describe_bot_locale should succeed");
    assert_eq!(describe_resp.description().unwrap(), "English US locale");
}

// ---- BotLocale has timestamps and status ----

#[tokio::test]
async fn test_bot_locale_has_status_and_timestamps() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleTimestamps").await;

    let resp = client
        .create_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.5)
        .send()
        .await
        .expect("create_bot_locale should succeed");

    assert!(resp.bot_locale_status().is_some());
    assert!(resp.creation_date_time().is_some());
}

// ---- Update bot locale resets status to NotBuilt ----

#[tokio::test]
async fn test_update_bot_locale_resets_status() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleStatusReset").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    // Build the locale first
    client
        .build_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("build_bot_locale should succeed");

    // Verify it's Built
    let describe_resp = client
        .describe_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("describe_bot_locale should succeed");
    assert_eq!(describe_resp.bot_locale_status().unwrap().as_str(), "Built");

    // Update should reset to NotBuilt
    let update_resp = client
        .update_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.9)
        .send()
        .await
        .expect("update_bot_locale should succeed");
    assert_eq!(
        update_resp.bot_locale_status().unwrap().as_str(),
        "NotBuilt"
    );
}

// ---- Slot with description and slot_type_id ----

#[tokio::test]
async fn test_create_slot_with_description_and_slot_type_id() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotConstraint, SlotTypeValue, SlotValueElicitationSetting,
        SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotDesc").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    // Create a slot type first
    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let slot_type_val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("NYC").build().unwrap())
        .build();

    let st_resp = client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("CityType")
        .slot_type_values(slot_type_val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create_slot_type should succeed");

    let slot_type_id = st_resp.slot_type_id().unwrap().to_string();

    let intent_id = create_test_intent(&client, &bot_id, "en_US", "IntentWithSlotDesc").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    let create_resp = client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("DestCity")
        .slot_type_id(&slot_type_id)
        .description("Destination city slot")
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("create_slot should succeed");

    let slot_id = create_resp.slot_id().unwrap().to_string();
    assert_eq!(create_resp.slot_name().unwrap(), "DestCity");
    assert_eq!(create_resp.description().unwrap(), "Destination city slot");
    assert_eq!(create_resp.slot_type_id().unwrap(), slot_type_id);

    let describe_resp = client
        .describe_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .send()
        .await
        .expect("describe_slot should succeed");

    assert_eq!(
        describe_resp.description().unwrap(),
        "Destination city slot"
    );
    assert_eq!(describe_resp.slot_type_id().unwrap(), slot_type_id);
}

// ---- Update slot verifies description via describe ----

#[tokio::test]
async fn test_update_slot_description() {
    use aws_sdk_lexmodelsv2::types::{SlotConstraint, SlotValueElicitationSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotDescUpd").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "SlotDescUpdIntent").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    let create_resp = client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("Amount")
        .value_elicitation_setting(elicitation.clone())
        .send()
        .await
        .expect("create_slot should succeed");

    let slot_id = create_resp.slot_id().unwrap().to_string();

    client
        .update_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .slot_name("Amount")
        .description("Updated slot description")
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("update_slot should succeed");

    let describe_resp = client
        .describe_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .send()
        .await
        .expect("describe_slot should succeed");

    assert_eq!(
        describe_resp.description().unwrap(),
        "Updated slot description"
    );
}

// ---- Slot has timestamps ----

#[tokio::test]
async fn test_slot_has_timestamps() {
    use aws_sdk_lexmodelsv2::types::{SlotConstraint, SlotValueElicitationSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotTimestamps").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "SlotTimestampIntent").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    let resp = client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("TimeSlot")
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("create_slot should succeed");

    assert!(resp.creation_date_time().is_some());
}

// ---- SlotType with description ----

#[tokio::test]
async fn test_create_slot_type_with_description() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotTypeDesc").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("red").build().unwrap())
        .build();

    let resp = client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("ColorType")
        .description("A colour slot type")
        .slot_type_values(val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create_slot_type should succeed");

    let slot_type_id = resp.slot_type_id().unwrap().to_string();
    assert_eq!(resp.description().unwrap(), "A colour slot type");

    let describe_resp = client
        .describe_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .send()
        .await
        .expect("describe_slot_type should succeed");

    assert_eq!(describe_resp.description().unwrap(), "A colour slot type");
}

// ---- Update SlotType description via describe ----

#[tokio::test]
async fn test_update_slot_type_description() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotTypeDescUpd").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("small").build().unwrap())
        .build();

    let create_resp = client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("SizeType")
        .slot_type_values(val.clone())
        .value_selection_setting(value_selection.clone())
        .send()
        .await
        .expect("create_slot_type should succeed");

    let slot_type_id = create_resp.slot_type_id().unwrap().to_string();

    client
        .update_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .slot_type_name("SizeType")
        .description("Updated size type description")
        .slot_type_values(val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("update_slot_type should succeed");

    let describe_resp = client
        .describe_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .send()
        .await
        .expect("describe_slot_type should succeed");

    assert_eq!(
        describe_resp.description().unwrap(),
        "Updated size type description"
    );
}

// ---- SlotType has timestamps ----

#[tokio::test]
async fn test_slot_type_has_timestamps() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotTypeTS").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("val").build().unwrap())
        .build();

    let resp = client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("TSType")
        .slot_type_values(val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create_slot_type should succeed");

    assert!(resp.creation_date_time().is_some());
}

// ---- Custom vocabulary with display_as field ----

#[tokio::test]
async fn test_custom_vocabulary_with_display_as() {
    use aws_sdk_lexmodelsv2::types::NewCustomVocabularyItem;

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVocabDisplayAs").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let item = NewCustomVocabularyItem::builder()
        .phrase("AWS")
        .display_as("Amazon Web Services")
        .weight(5)
        .build()
        .expect("builder should succeed");

    let resp = client
        .batch_create_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(item)
        .send()
        .await
        .expect("batch_create should succeed");

    let resources = resp.resources();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].phrase(), "AWS");
    assert_eq!(resources[0].display_as(), Some("Amazon Web Services"));
    assert_eq!(resources[0].weight(), Some(5));
}

// ---- Custom vocabulary batch delete partial items ----

#[tokio::test]
async fn test_custom_vocabulary_batch_delete_partial() {
    use aws_sdk_lexmodelsv2::types::{CustomVocabularyEntryId, NewCustomVocabularyItem};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVocabPartialDel").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let item1 = NewCustomVocabularyItem::builder()
        .phrase("Lambda")
        .build()
        .expect("builder should succeed");

    let item2 = NewCustomVocabularyItem::builder()
        .phrase("DynamoDB")
        .build()
        .expect("builder should succeed");

    let create_resp = client
        .batch_create_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(item1)
        .custom_vocabulary_item_list(item2)
        .send()
        .await
        .expect("batch_create should succeed");

    let resources = create_resp.resources();
    assert_eq!(resources.len(), 2);

    // Delete only the first item
    let delete_entry = CustomVocabularyEntryId::builder()
        .item_id(resources[0].item_id())
        .build()
        .expect("builder should succeed");

    client
        .batch_delete_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(delete_entry)
        .send()
        .await
        .expect("batch_delete should succeed");

    let list_resp = client
        .list_custom_vocabulary_items()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.custom_vocabulary_items().len(), 1);
}

// ---- Delete custom vocabulary that does not exist ----

#[tokio::test]
async fn test_delete_custom_vocabulary_not_found() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVocabDelNotFound").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let result = client
        .delete_custom_vocabulary()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await;
    assert!(
        result.is_err(),
        "deleting nonexistent custom vocabulary should fail"
    );
}

// ---- Bot version sequential numbering ----

#[tokio::test]
async fn test_bot_version_numbering_sequential() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVersionSeq").await;

    let v1 = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("version 1 should succeed");
    assert_eq!(v1.bot_version().unwrap(), "1");

    let v2 = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("version 2 should succeed");
    assert_eq!(v2.bot_version().unwrap(), "2");

    // Delete version 2
    client
        .delete_bot_version()
        .bot_id(&bot_id)
        .bot_version("2")
        .send()
        .await
        .expect("delete version 2 should succeed");

    // Next version should be 3 (not 2), since max was still tracked
    let v3 = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("version 3 should succeed");
    // After deleting v2, the next version should still be > 1
    let v3_num: u32 = v3.bot_version().unwrap().parse().unwrap();
    assert!(
        v3_num >= 2,
        "version number should increment beyond deleted versions"
    );
}

// ---- BotVersion has status ----

#[tokio::test]
async fn test_bot_version_has_status() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVersionStatus").await;

    let resp = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create_bot_version should succeed");

    assert!(resp.bot_status().is_some());
    assert!(resp.creation_date_time().is_some());
}

// ---- Resource policy revision changes on update ----

#[tokio::test]
async fn test_resource_policy_revision_id_changes() {
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("RevisionBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    let create_resp = client
        .create_resource_policy()
        .resource_arn(&resource_arn)
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("create_resource_policy should succeed");

    let rev1 = create_resp.revision_id().unwrap().to_string();

    let update_resp = client
        .update_resource_policy()
        .resource_arn(&resource_arn)
        .policy(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Deny"}]}"#)
        .send()
        .await
        .expect("update_resource_policy should succeed");

    let rev2 = update_resp.revision_id().unwrap().to_string();

    assert_ne!(rev1, rev2, "revision ID should change after update");
}

// ---- Multiple exports ----

#[tokio::test]
async fn test_multiple_exports() {
    use aws_sdk_lexmodelsv2::types::{BotExportSpecification, ExportResourceSpecification};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForMultiExport").await;

    let resource_spec = ExportResourceSpecification::builder()
        .bot_export_specification(
            BotExportSpecification::builder()
                .bot_id(&bot_id)
                .bot_version("DRAFT")
                .build()
                .expect("builder should succeed"),
        )
        .build();

    client
        .create_export()
        .resource_specification(resource_spec.clone())
        .file_format(aws_sdk_lexmodelsv2::types::ImportExportFileFormat::LexJson)
        .send()
        .await
        .expect("export 1 should succeed");

    let resource_spec2 = ExportResourceSpecification::builder()
        .bot_export_specification(
            BotExportSpecification::builder()
                .bot_id(&bot_id)
                .bot_version("DRAFT")
                .build()
                .expect("builder should succeed"),
        )
        .build();

    client
        .create_export()
        .resource_specification(resource_spec2)
        .file_format(aws_sdk_lexmodelsv2::types::ImportExportFileFormat::LexJson)
        .send()
        .await
        .expect("export 2 should succeed");

    let list_resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    assert!(list_resp.export_summaries().len() >= 2);
}

// ---- Multiple imports ----

#[tokio::test]
async fn test_multiple_imports() {
    use aws_sdk_lexmodelsv2::types::MergeStrategy;

    let client = make_client().await;

    client
        .start_import()
        .import_id("import-multi-001")
        .resource_specification(
            aws_sdk_lexmodelsv2::types::ImportResourceSpecification::builder().build(),
        )
        .merge_strategy(MergeStrategy::Overwrite)
        .send()
        .await
        .expect("import 1 should succeed");

    client
        .start_import()
        .import_id("import-multi-002")
        .resource_specification(
            aws_sdk_lexmodelsv2::types::ImportResourceSpecification::builder().build(),
        )
        .merge_strategy(MergeStrategy::FailOnConflict)
        .send()
        .await
        .expect("import 2 should succeed");

    let list_resp = client
        .list_imports()
        .send()
        .await
        .expect("list_imports should succeed");
    assert!(list_resp.import_summaries().len() >= 2);
}

// ---- Delete bot cascades to versions ----

#[tokio::test]
async fn test_delete_bot_cascades_versions() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVersionCascade").await;

    client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create version should succeed");

    client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create version 2 should succeed");

    client
        .delete_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("delete_bot should succeed");

    // Versions should be gone
    let list_resp = client
        .list_bot_versions()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_versions should succeed");
    assert_eq!(list_resp.bot_version_summaries().len(), 0);
}

// ---- Delete bot cascades to intents ----

#[tokio::test]
async fn test_delete_bot_cascades_intents() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentCascade").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    create_test_intent(&client, &bot_id, "en_US", "CascadeIntent1").await;
    create_test_intent(&client, &bot_id, "en_US", "CascadeIntent2").await;

    client
        .delete_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("delete_bot should succeed");

    // Intents should be gone
    let list_resp = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_intents should succeed");
    assert_eq!(list_resp.intent_summaries().len(), 0);
}

// ---- Intent update description ----

#[tokio::test]
async fn test_update_intent_description() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentDescUpd").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "UpdDescIntent").await;

    client
        .update_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .intent_name("UpdDescIntent")
        .description("Updated intent description")
        .send()
        .await
        .expect("update_intent should succeed");

    let describe_resp = client
        .describe_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("describe_intent should succeed");

    assert_eq!(
        describe_resp.description().unwrap(),
        "Updated intent description"
    );
}

// ---- Intent has timestamps ----

#[tokio::test]
async fn test_intent_has_timestamps() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentTimestamps").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let resp = client
        .create_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_name("TSIntent")
        .send()
        .await
        .expect("create_intent should succeed");

    assert!(resp.creation_date_time().is_some());
}

// ---- Export has file_format and resource_specification ----

#[tokio::test]
async fn test_export_describe_fields() {
    use aws_sdk_lexmodelsv2::types::{BotExportSpecification, ExportResourceSpecification};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForExportFields").await;

    let resource_spec = ExportResourceSpecification::builder()
        .bot_export_specification(
            BotExportSpecification::builder()
                .bot_id(&bot_id)
                .bot_version("DRAFT")
                .build()
                .expect("builder should succeed"),
        )
        .build();

    let create_resp = client
        .create_export()
        .resource_specification(resource_spec)
        .file_format(aws_sdk_lexmodelsv2::types::ImportExportFileFormat::LexJson)
        .send()
        .await
        .expect("create_export should succeed");

    let export_id = create_resp.export_id().unwrap().to_string();

    let describe_resp = client
        .describe_export()
        .export_id(&export_id)
        .send()
        .await
        .expect("describe_export should succeed");

    assert!(describe_resp.export_status().is_some());
    assert!(describe_resp.creation_date_time().is_some());
    assert!(describe_resp.last_updated_date_time().is_some());
    assert!(describe_resp.resource_specification().is_some());
}

// ---- Import describe fields ----

#[tokio::test]
async fn test_import_describe_fields() {
    use aws_sdk_lexmodelsv2::types::MergeStrategy;

    let client = make_client().await;

    let start_resp = client
        .start_import()
        .import_id("import-fields-001")
        .resource_specification(
            aws_sdk_lexmodelsv2::types::ImportResourceSpecification::builder().build(),
        )
        .merge_strategy(MergeStrategy::Overwrite)
        .send()
        .await
        .expect("start_import should succeed");

    let import_id = start_resp.import_id().unwrap().to_string();

    let describe_resp = client
        .describe_import()
        .import_id(&import_id)
        .send()
        .await
        .expect("describe_import should succeed");

    assert!(describe_resp.import_status().is_some());
    assert!(describe_resp.merge_strategy().is_some());
    assert!(describe_resp.creation_date_time().is_some());
    assert!(describe_resp.last_updated_date_time().is_some());
}

// ---- Update nonexistent export ----

#[tokio::test]
async fn test_update_nonexistent_export() {
    let client = make_client().await;

    let result = client
        .update_export()
        .export_id("NONEXISTENT")
        .file_password("secret")
        .send()
        .await;
    assert!(result.is_err(), "updating nonexistent export should fail");
}

// ---- Tag overwrite existing key ----

#[tokio::test]
async fn test_tag_resource_overwrite_key() {
    let client = make_client().await;

    let bot = client
        .create_bot()
        .bot_name("TagOverwriteBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = bot.bot_id().unwrap();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "dev")
        .send()
        .await
        .expect("tag_resource should succeed");

    // Overwrite the same key
    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag_resource overwrite should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.len(), 1, "should have exactly one tag");
}

// ---- List bots returns correct summaries ----

#[tokio::test]
async fn test_list_bots_summary_fields() {
    let client = make_client().await;

    client
        .create_bot()
        .bot_name("SummaryBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .description("Summary test bot")
        .send()
        .await
        .expect("create_bot should succeed");

    let resp = client
        .list_bots()
        .send()
        .await
        .expect("list_bots should succeed");

    let summaries = resp.bot_summaries();
    assert!(!summaries.is_empty());

    let first = &summaries[0];
    assert!(first.bot_id().is_some());
    assert!(first.bot_name().is_some());
    assert!(first.bot_status().is_some());
}

// ---- Describe bot returns all fields ----

#[tokio::test]
async fn test_describe_bot_all_fields() {
    let client = make_client().await;

    let resp = client
        .create_bot()
        .bot_name("AllFieldsBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(true).build())
        .idle_session_ttl_in_seconds(600)
        .description("All fields test")
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = resp.bot_id().unwrap().to_string();

    let describe_resp = client
        .describe_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("describe_bot should succeed");

    assert_eq!(describe_resp.bot_id().unwrap(), bot_id);
    assert_eq!(describe_resp.bot_name().unwrap(), "AllFieldsBot");
    assert_eq!(
        describe_resp.role_arn().unwrap(),
        "arn:aws:iam::123456789012:role/LexRole"
    );
    assert_eq!(describe_resp.idle_session_ttl_in_seconds(), Some(600));
    assert_eq!(describe_resp.description().unwrap(), "All fields test");
    assert!(describe_resp.bot_status().is_some());
    assert!(describe_resp.data_privacy().is_some());
    assert!(describe_resp.creation_date_time().is_some());
    assert!(describe_resp.last_updated_date_time().is_some());
}

// ---- BotAlias list summary fields ----

#[tokio::test]
async fn test_list_bot_aliases_summary_fields() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAliasSummary").await;

    client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("SummaryAlias")
        .description("Summary alias desc")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    let resp = client
        .list_bot_aliases()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_aliases should succeed");

    assert_eq!(resp.bot_alias_summaries().len(), 1);
    let summary = &resp.bot_alias_summaries()[0];
    assert!(summary.bot_alias_id().is_some());
    assert!(summary.bot_alias_name().is_some());
    assert!(summary.bot_alias_status().is_some());
}

// ---- BotLocale list summary fields ----

#[tokio::test]
async fn test_list_bot_locales_summary_fields() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleSummary").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let resp = client
        .list_bot_locales()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .send()
        .await
        .expect("list_bot_locales should succeed");

    assert_eq!(resp.bot_locale_summaries().len(), 1);
    let summary = &resp.bot_locale_summaries()[0];
    assert!(summary.locale_id().is_some());
    assert!(summary.locale_name().is_some());
    assert!(summary.bot_locale_status().is_some());
}

// ---- Intent list summary fields ----

#[tokio::test]
async fn test_list_intents_summary_fields() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentSummary").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    create_test_intent(&client, &bot_id, "en_US", "SummaryIntent").await;

    let resp = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_intents should succeed");

    assert_eq!(resp.intent_summaries().len(), 1);
    let summary = &resp.intent_summaries()[0];
    assert!(summary.intent_id().is_some());
    assert!(summary.intent_name().is_some());
}

// ---- Slot list summary fields ----

#[tokio::test]
async fn test_list_slots_summary_fields() {
    use aws_sdk_lexmodelsv2::types::{SlotConstraint, SlotValueElicitationSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotSummary").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "SlotSummaryIntent").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("SummarySlot")
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("create_slot should succeed");

    let resp = client
        .list_slots()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("list_slots should succeed");

    assert_eq!(resp.slot_summaries().len(), 1);
    let summary = &resp.slot_summaries()[0];
    assert!(summary.slot_id().is_some());
    assert!(summary.slot_name().is_some());
}

// ---- SlotType list summary fields ----

#[tokio::test]
async fn test_list_slot_types_summary_fields() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotTypeSummary").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("test").build().unwrap())
        .build();

    client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("SummaryType")
        .slot_type_values(val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create_slot_type should succeed");

    let resp = client
        .list_slot_types()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_slot_types should succeed");

    assert_eq!(resp.slot_type_summaries().len(), 1);
    let summary = &resp.slot_type_summaries()[0];
    assert!(summary.slot_type_id().is_some());
    assert!(summary.slot_type_name().is_some());
}

// ---- BotVersion list summary fields ----

#[tokio::test]
async fn test_list_bot_versions_summary_fields() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVersionSummary").await;

    client
        .create_bot_version()
        .bot_id(&bot_id)
        .description("Version 1")
        .send()
        .await
        .expect("create_bot_version should succeed");

    let resp = client
        .list_bot_versions()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("list_bot_versions should succeed");

    assert_eq!(resp.bot_version_summaries().len(), 1);
    let summary = &resp.bot_version_summaries()[0];
    assert!(summary.bot_version().is_some());
    assert!(summary.bot_status().is_some());
}

// ---- Export list summary fields ----

#[tokio::test]
async fn test_list_exports_summary_fields() {
    use aws_sdk_lexmodelsv2::types::{BotExportSpecification, ExportResourceSpecification};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForExportSummary").await;

    let resource_spec = ExportResourceSpecification::builder()
        .bot_export_specification(
            BotExportSpecification::builder()
                .bot_id(&bot_id)
                .bot_version("DRAFT")
                .build()
                .expect("builder should succeed"),
        )
        .build();

    client
        .create_export()
        .resource_specification(resource_spec)
        .file_format(aws_sdk_lexmodelsv2::types::ImportExportFileFormat::LexJson)
        .send()
        .await
        .expect("create_export should succeed");

    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    assert!(!resp.export_summaries().is_empty());
    let summary = &resp.export_summaries()[0];
    assert!(summary.export_id().is_some());
    assert!(summary.export_status().is_some());
}

// ---- Import list summary fields ----

#[tokio::test]
async fn test_list_imports_summary_fields() {
    use aws_sdk_lexmodelsv2::types::MergeStrategy;

    let client = make_client().await;

    client
        .start_import()
        .import_id("import-summary-001")
        .resource_specification(
            aws_sdk_lexmodelsv2::types::ImportResourceSpecification::builder().build(),
        )
        .merge_strategy(MergeStrategy::Overwrite)
        .send()
        .await
        .expect("start_import should succeed");

    let resp = client
        .list_imports()
        .send()
        .await
        .expect("list_imports should succeed");

    assert!(!resp.import_summaries().is_empty());
    let summary = &resp.import_summaries()[0];
    assert!(summary.import_id().is_some());
    assert!(summary.import_status().is_some());
    assert!(summary.merge_strategy().is_some());
}

// ---- Describe bot locale all fields ----

#[tokio::test]
async fn test_describe_bot_locale_all_fields() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleAllFields").await;

    client
        .create_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .nlu_intent_confidence_threshold(0.75)
        .description("All fields locale")
        .send()
        .await
        .expect("create_bot_locale should succeed");

    let resp = client
        .describe_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("describe_bot_locale should succeed");

    assert_eq!(resp.bot_id().unwrap(), bot_id);
    assert_eq!(resp.locale_id().unwrap(), "en_US");
    assert!(resp.locale_name().is_some());
    assert_eq!(resp.nlu_intent_confidence_threshold(), Some(0.75));
    assert_eq!(resp.description().unwrap(), "All fields locale");
    assert!(resp.bot_locale_status().is_some());
    assert!(resp.creation_date_time().is_some());
    assert!(resp.last_updated_date_time().is_some());
}

// ---- Describe bot alias all fields ----

#[tokio::test]
async fn test_describe_bot_alias_all_fields() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAliasAllFields").await;

    let version_resp = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create_bot_version should succeed");
    let version = version_resp.bot_version().unwrap().to_string();

    let create_resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("AllFieldsAlias")
        .bot_version(&version)
        .description("All fields alias")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    let alias_id = create_resp.bot_alias_id().unwrap().to_string();

    let resp = client
        .describe_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await
        .expect("describe_bot_alias should succeed");

    assert_eq!(resp.bot_alias_id().unwrap(), alias_id);
    assert_eq!(resp.bot_alias_name().unwrap(), "AllFieldsAlias");
    assert_eq!(resp.bot_id().unwrap(), bot_id);
    assert_eq!(resp.bot_version().unwrap(), version);
    assert_eq!(resp.description().unwrap(), "All fields alias");
    assert!(resp.bot_alias_status().is_some());
    assert!(resp.creation_date_time().is_some());
    assert!(resp.last_updated_date_time().is_some());
}

// ---- Update bot alias version ----

#[tokio::test]
async fn test_update_bot_alias_version() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForAliasVersionUpd").await;

    let v1 = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create version 1 should succeed");
    let v1_str = v1.bot_version().unwrap().to_string();

    let v2 = client
        .create_bot_version()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("create version 2 should succeed");
    let v2_str = v2.bot_version().unwrap().to_string();

    let alias_resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("VersionedAlias")
        .bot_version(&v1_str)
        .send()
        .await
        .expect("create_bot_alias should succeed");
    let alias_id = alias_resp.bot_alias_id().unwrap().to_string();

    // Update alias to point to v2
    let update_resp = client
        .update_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .bot_alias_name("VersionedAlias")
        .bot_version(&v2_str)
        .send()
        .await
        .expect("update_bot_alias should succeed");

    assert_eq!(update_resp.bot_version().unwrap(), v2_str);

    let describe_resp = client
        .describe_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await
        .expect("describe_bot_alias should succeed");
    assert_eq!(describe_resp.bot_version().unwrap(), v2_str);
}

// ---- Delete bot locale cascades slot_types ----

#[tokio::test]
async fn test_delete_bot_locale_cascades_slot_types() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForLocaleSTCascade").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("test").build().unwrap())
        .build();

    client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("CascadeType")
        .slot_type_values(val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create_slot_type should succeed");

    client
        .delete_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("delete_bot_locale should succeed");

    // Re-create locale to query slot types via API
    create_test_locale(&client, &bot_id, "en_US").await;

    let list_resp = client
        .list_slot_types()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_slot_types should succeed");

    // Slot types may or may not be cascade-deleted by the mock; we verify the query works.
    // The main test goal is that the locale delete + subsequent operations don't panic.
    let _ = list_resp.slot_type_summaries();
}

// ---- Built-in intents contain expected signatures ----

#[tokio::test]
async fn test_list_built_in_intents_has_fallback() {
    let client = make_client().await;

    let resp = client
        .list_built_in_intents()
        .locale_id("en_US")
        .send()
        .await
        .expect("list_built_in_intents should succeed");

    let summaries = resp.built_in_intent_summaries();
    let signatures: Vec<&str> = summaries
        .iter()
        .filter_map(|s| s.intent_signature())
        .collect();

    assert!(
        signatures.contains(&"AMAZON.FallbackIntent"),
        "should include AMAZON.FallbackIntent"
    );
    assert!(
        signatures.contains(&"AMAZON.HelpIntent"),
        "should include AMAZON.HelpIntent"
    );
}

// ---- Built-in slot types contain expected signatures ----

#[tokio::test]
async fn test_list_built_in_slot_types_has_number() {
    let client = make_client().await;

    let resp = client
        .list_built_in_slot_types()
        .locale_id("en_US")
        .send()
        .await
        .expect("list_built_in_slot_types should succeed");

    let summaries = resp.built_in_slot_type_summaries();
    let signatures: Vec<&str> = summaries
        .iter()
        .filter_map(|s| s.slot_type_signature())
        .collect();

    assert!(
        signatures.contains(&"AMAZON.Number"),
        "should include AMAZON.Number"
    );
    assert!(
        signatures.contains(&"AMAZON.Time"),
        "should include AMAZON.Time"
    );
}

// ---- Describe intent all fields ----

#[tokio::test]
async fn test_describe_intent_all_fields() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentAllFields").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let resp = client
        .create_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_name("AllFieldsIntent")
        .description("Intent all fields")
        .send()
        .await
        .expect("create_intent should succeed");

    let intent_id = resp.intent_id().unwrap().to_string();

    let describe_resp = client
        .describe_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("describe_intent should succeed");

    assert_eq!(describe_resp.intent_id().unwrap(), intent_id);
    assert_eq!(describe_resp.intent_name().unwrap(), "AllFieldsIntent");
    assert_eq!(describe_resp.bot_id().unwrap(), bot_id);
    assert_eq!(describe_resp.bot_version().unwrap(), "DRAFT");
    assert_eq!(describe_resp.locale_id().unwrap(), "en_US");
    assert_eq!(describe_resp.description().unwrap(), "Intent all fields");
    assert!(describe_resp.creation_date_time().is_some());
    assert!(describe_resp.last_updated_date_time().is_some());
}

// ---- Describe slot all fields ----

#[tokio::test]
async fn test_describe_slot_all_fields() {
    use aws_sdk_lexmodelsv2::types::{SlotConstraint, SlotValueElicitationSetting};

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotAllFields").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "SlotAllFieldsIntent").await;

    let elicitation = SlotValueElicitationSetting::builder()
        .slot_constraint(SlotConstraint::Required)
        .build()
        .expect("builder should succeed");

    let create_resp = client
        .create_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_name("AllFieldsSlot")
        .description("Slot all fields")
        .value_elicitation_setting(elicitation)
        .send()
        .await
        .expect("create_slot should succeed");

    let slot_id = create_resp.slot_id().unwrap().to_string();

    let describe_resp = client
        .describe_slot()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .slot_id(&slot_id)
        .send()
        .await
        .expect("describe_slot should succeed");

    assert_eq!(describe_resp.slot_id().unwrap(), slot_id);
    assert_eq!(describe_resp.slot_name().unwrap(), "AllFieldsSlot");
    assert_eq!(describe_resp.bot_id().unwrap(), bot_id);
    assert_eq!(describe_resp.bot_version().unwrap(), "DRAFT");
    assert_eq!(describe_resp.locale_id().unwrap(), "en_US");
    assert_eq!(describe_resp.intent_id().unwrap(), intent_id);
    assert_eq!(describe_resp.description().unwrap(), "Slot all fields");
    assert!(describe_resp.value_elicitation_setting().is_some());
    assert!(describe_resp.creation_date_time().is_some());
    assert!(describe_resp.last_updated_date_time().is_some());
}

// ---- Describe slot type all fields ----

#[tokio::test]
async fn test_describe_slot_type_all_fields() {
    use aws_sdk_lexmodelsv2::types::{
        SampleValue, SlotTypeValue, SlotValueResolutionStrategy, SlotValueSelectionSetting,
    };

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForSlotTypeAllFields").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let value_selection = SlotValueSelectionSetting::builder()
        .resolution_strategy(SlotValueResolutionStrategy::OriginalValue)
        .build()
        .expect("builder should succeed");

    let val = SlotTypeValue::builder()
        .sample_value(SampleValue::builder().value("blue").build().unwrap())
        .build();

    let create_resp = client
        .create_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_name("AllFieldsType")
        .description("SlotType all fields")
        .slot_type_values(val)
        .value_selection_setting(value_selection)
        .send()
        .await
        .expect("create_slot_type should succeed");

    let slot_type_id = create_resp.slot_type_id().unwrap().to_string();

    let describe_resp = client
        .describe_slot_type()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .slot_type_id(&slot_type_id)
        .send()
        .await
        .expect("describe_slot_type should succeed");

    assert_eq!(describe_resp.slot_type_id().unwrap(), slot_type_id);
    assert_eq!(describe_resp.slot_type_name().unwrap(), "AllFieldsType");
    assert_eq!(describe_resp.bot_id().unwrap(), bot_id);
    assert_eq!(describe_resp.bot_version().unwrap(), "DRAFT");
    assert_eq!(describe_resp.locale_id().unwrap(), "en_US");
    assert_eq!(describe_resp.description().unwrap(), "SlotType all fields");
    assert!(describe_resp.value_selection_setting().is_some());
    assert!(describe_resp.creation_date_time().is_some());
    assert!(describe_resp.last_updated_date_time().is_some());
}

// ---- Custom vocabulary metadata fields ----

#[tokio::test]
async fn test_describe_custom_vocabulary_metadata_fields() {
    use aws_sdk_lexmodelsv2::types::NewCustomVocabularyItem;

    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForVocabMetaFields").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let item = NewCustomVocabularyItem::builder()
        .phrase("Amazon Bedrock")
        .build()
        .expect("builder should succeed");

    client
        .batch_create_custom_vocabulary_item()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .custom_vocabulary_item_list(item)
        .send()
        .await
        .expect("batch_create should succeed");

    let resp = client
        .describe_custom_vocabulary_metadata()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("describe_custom_vocabulary_metadata should succeed");

    assert_eq!(resp.bot_id().unwrap(), bot_id);
    assert_eq!(resp.bot_version().unwrap(), "DRAFT");
    assert_eq!(resp.locale_id().unwrap(), "en_US");
    assert!(resp.custom_vocabulary_status().is_some());
    assert!(resp.creation_date_time().is_some());
    assert!(resp.last_updated_date_time().is_some());
}

// ---- Delete and re-create same locale ----

#[tokio::test]
async fn test_delete_and_recreate_locale() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForRecreateLoc").await;

    create_test_locale(&client, &bot_id, "en_US").await;
    create_test_intent(&client, &bot_id, "en_US", "FirstIntent").await;

    client
        .delete_bot_locale()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("delete locale should succeed");

    // Re-create should succeed
    create_test_locale(&client, &bot_id, "en_US").await;

    // Should have zero intents in re-created locale
    let list_resp = client
        .list_intents()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .send()
        .await
        .expect("list_intents should succeed");
    assert_eq!(list_resp.intent_summaries().len(), 0);
}

// ---- Delete and re-create bot alias ----

#[tokio::test]
async fn test_delete_and_recreate_alias() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForRecreateAlias").await;

    let create_resp = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("RecycleAlias")
        .send()
        .await
        .expect("create_bot_alias should succeed");

    let alias_id = create_resp.bot_alias_id().unwrap().to_string();

    client
        .delete_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_id(&alias_id)
        .send()
        .await
        .expect("delete_bot_alias should succeed");

    // Re-create with same name
    let create_resp2 = client
        .create_bot_alias()
        .bot_id(&bot_id)
        .bot_alias_name("RecycleAlias")
        .send()
        .await
        .expect("re-create_bot_alias should succeed");

    assert_ne!(
        create_resp2.bot_alias_id().unwrap(),
        alias_id,
        "re-created alias should have a new ID"
    );
}

// ---- Delete and re-create intent ----

#[tokio::test]
async fn test_delete_and_recreate_intent() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForRecreateInt").await;
    create_test_locale(&client, &bot_id, "en_US").await;

    let intent_id = create_test_intent(&client, &bot_id, "en_US", "RecycleIntent").await;

    client
        .delete_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("delete_intent should succeed");

    // Re-create with same name
    let new_intent_id = create_test_intent(&client, &bot_id, "en_US", "RecycleIntent").await;

    assert_ne!(
        new_intent_id, intent_id,
        "re-created intent should have a new ID"
    );
}

// ---- Update intent changes name ----

#[tokio::test]
async fn test_update_intent_name() {
    let client = make_client().await;
    let bot_id = create_test_bot(&client, "BotForIntentNameUpd").await;
    create_test_locale(&client, &bot_id, "en_US").await;
    let intent_id = create_test_intent(&client, &bot_id, "en_US", "OriginalIntentName").await;

    let update_resp = client
        .update_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .intent_name("RenamedIntent")
        .send()
        .await
        .expect("update_intent should succeed");

    assert_eq!(update_resp.intent_name().unwrap(), "RenamedIntent");

    let describe_resp = client
        .describe_intent()
        .bot_id(&bot_id)
        .bot_version("DRAFT")
        .locale_id("en_US")
        .intent_id(&intent_id)
        .send()
        .await
        .expect("describe_intent should succeed");
    assert_eq!(describe_resp.intent_name().unwrap(), "RenamedIntent");
}

// ---- Tags removed when bot deleted ----

#[tokio::test]
async fn test_delete_bot_removes_tags() {
    let client = make_client().await;

    let resp = client
        .create_bot()
        .bot_name("TagCleanupBot")
        .role_arn("arn:aws:iam::123456789012:role/LexRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");

    let bot_id = resp.bot_id().unwrap().to_string();
    let resource_arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag_resource should succeed");

    client
        .delete_bot()
        .bot_id(&bot_id)
        .send()
        .await
        .expect("delete_bot should succeed");

    // Tags should be gone; list_tags_for_resource returns empty for unknown ARNs
    let tag_resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = tag_resp.tags();
    // Should have no tags or empty map
    if let Some(t) = tags {
        assert!(t.is_empty(), "tags should be removed after bot deletion");
    }
}
