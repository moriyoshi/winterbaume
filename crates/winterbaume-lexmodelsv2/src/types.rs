use std::collections::HashMap;

use crate::model::{
    CompositeSlotTypeSetting, ExternalSourceSetting, MultipleValuesSetting, ObfuscationSetting,
    SlotTypeValue, SlotValueElicitationSetting, SlotValueSelectionSetting, SubSlotSetting,
};

/// Internal representation of a Lex V2 Bot.
#[derive(Debug, Clone)]
pub struct Bot {
    pub bot_id: String,
    pub bot_name: String,
    pub role_arn: String,
    pub data_privacy_child_directed: bool,
    pub idle_session_ttl_in_seconds: i32,
    pub bot_status: String,
    pub description: Option<String>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
    pub tags: HashMap<String, String>,
}

/// Internal representation of a Lex V2 Bot Alias.
#[derive(Debug, Clone)]
pub struct BotAlias {
    pub bot_alias_id: String,
    pub bot_alias_name: String,
    pub bot_id: String,
    pub bot_alias_status: String,
    pub bot_version: Option<String>,
    pub description: Option<String>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

/// Internal representation of a Lex V2 Bot Locale.
#[derive(Debug, Clone)]
pub struct BotLocale {
    pub bot_id: String,
    pub locale_id: String,
    pub locale_name: String,
    pub bot_version: String,
    pub nlu_intent_confidence_threshold: f64,
    pub bot_locale_status: String,
    pub description: Option<String>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

/// Internal representation of a Lex V2 Intent.
#[derive(Debug, Clone)]
pub struct Intent {
    pub intent_id: String,
    pub intent_name: String,
    pub bot_id: String,
    pub bot_version: String,
    pub locale_id: String,
    pub description: Option<String>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

/// Internal representation of a Lex V2 Bot Version.
#[derive(Debug, Clone)]
pub struct BotVersion {
    pub bot_id: String,
    pub bot_version: String,
    pub bot_status: String,
    pub description: Option<String>,
    pub creation_date_time: String,
}

/// Internal representation of a Lex V2 Resource Policy.
#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub resource_arn: String,
    pub policy: String,
    pub revision_id: String,
}

/// Internal representation of a Lex V2 Slot.
#[derive(Debug, Clone)]
pub struct Slot {
    pub slot_id: String,
    pub slot_name: String,
    pub bot_id: String,
    pub bot_version: String,
    pub locale_id: String,
    pub intent_id: String,
    pub slot_type_id: Option<String>,
    pub description: Option<String>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
    pub value_elicitation_setting: SlotValueElicitationSetting,
    pub multiple_values_setting: Option<MultipleValuesSetting>,
    pub obfuscation_setting: Option<ObfuscationSetting>,
    pub sub_slot_setting: Option<SubSlotSetting>,
}

/// Internal representation of a Lex V2 SlotType.
#[derive(Debug, Clone)]
pub struct SlotType {
    pub slot_type_id: String,
    pub slot_type_name: String,
    pub bot_id: String,
    pub bot_version: String,
    pub locale_id: String,
    pub description: Option<String>,
    pub parent_slot_type_signature: Option<String>,
    pub slot_type_values: Option<Vec<SlotTypeValue>>,
    pub value_selection_setting: Option<SlotValueSelectionSetting>,
    pub external_source_setting: Option<ExternalSourceSetting>,
    pub composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

/// Internal representation of a Lex V2 CustomVocabularyItem.
#[derive(Debug, Clone)]
pub struct CustomVocabularyEntry {
    pub item_id: String,
    pub phrase: String,
    pub display_as: Option<String>,
    pub weight: Option<i32>,
}

/// Internal representation of a Lex V2 CustomVocabulary per-locale metadata.
#[derive(Debug, Clone)]
pub struct CustomVocabularyMeta {
    pub bot_id: String,
    pub bot_version: String,
    pub locale_id: String,
    pub status: String,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

/// Internal representation of a Lex V2 Export job.
#[derive(Debug, Clone)]
pub struct ExportJob {
    pub export_id: String,
    pub file_format: String,
    pub export_status: String,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
    pub resource_specification: Option<crate::model::ExportResourceSpecification>,
}

/// Internal representation of a Lex V2 Import job.
#[derive(Debug, Clone)]
pub struct ImportJob {
    pub import_id: String,
    pub import_status: String,
    pub merge_strategy: String,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
    pub resource_specification: Option<crate::model::ImportResourceSpecification>,
    pub imported_resource_id: Option<String>,
    pub imported_resource_name: Option<String>,
}
