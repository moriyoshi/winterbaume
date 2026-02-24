use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::LexModelsV2Service;
use crate::model::{ExportResourceSpecification, ImportResourceSpecification};
use crate::state::LexState;
use crate::types::{
    Bot, BotAlias, BotLocale, BotVersion, CustomVocabularyEntry, CustomVocabularyMeta, ExportJob,
    ImportJob, Intent, Slot, SlotType,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LexStateView {
    #[serde(default)]
    pub bots: HashMap<String, BotView>,
    #[serde(default)]
    pub bot_aliases: HashMap<String, BotAliasView>,
    #[serde(default)]
    pub bot_locales: HashMap<String, BotLocaleView>,
    #[serde(default)]
    pub intents: HashMap<String, IntentView>,
    #[serde(default)]
    pub bot_versions: HashMap<String, BotVersionView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    /// resource policies keyed by resourceArn: (policy JSON, revisionId)
    #[serde(default)]
    pub resource_policies: HashMap<String, (String, String)>,
    /// slots keyed by "{botId}/{localeId}/{intentId}/{slotId}"
    #[serde(default)]
    pub slots: HashMap<String, SlotView>,
    /// slot types keyed by "{botId}/{localeId}/{slotTypeId}"
    #[serde(default)]
    pub slot_types: HashMap<String, SlotTypeView>,
    /// custom vocabulary items keyed by "{botId}/{localeId}/{itemId}"
    #[serde(default)]
    pub custom_vocabulary_items: HashMap<String, CustomVocabularyEntryView>,
    /// custom vocabulary metadata keyed by "{botId}/{localeId}"
    #[serde(default)]
    pub custom_vocabulary_meta: HashMap<String, CustomVocabularyMetaView>,
    /// export jobs keyed by exportId
    #[serde(default)]
    pub exports: HashMap<String, ExportJobView>,
    /// import jobs keyed by importId
    #[serde(default)]
    pub imports: HashMap<String, ImportJobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotAliasView {
    pub bot_alias_id: String,
    pub bot_alias_name: String,
    pub bot_id: String,
    pub bot_alias_status: String,
    pub bot_version: Option<String>,
    pub description: Option<String>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotLocaleView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentView {
    pub intent_id: String,
    pub intent_name: String,
    pub bot_id: String,
    pub bot_version: String,
    pub locale_id: String,
    pub description: Option<String>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotVersionView {
    pub bot_id: String,
    pub bot_version: String,
    pub bot_status: String,
    pub description: Option<String>,
    pub creation_date_time: String,
}

/// Slot view — complex model fields stored as opaque JSON blobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotView {
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
    /// Opaque wire-shaped blob for SlotValueElicitationSetting.
    pub value_elicitation_setting: Value,
    /// Opaque wire-shaped blob for MultipleValuesSetting.
    pub multiple_values_setting: Option<Value>,
    /// Opaque wire-shaped blob for ObfuscationSetting.
    pub obfuscation_setting: Option<Value>,
    /// Opaque wire-shaped blob for SubSlotSetting.
    pub sub_slot_setting: Option<Value>,
}

/// SlotType view — complex model fields stored as opaque JSON blobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotTypeView {
    pub slot_type_id: String,
    pub slot_type_name: String,
    pub bot_id: String,
    pub bot_version: String,
    pub locale_id: String,
    pub description: Option<String>,
    pub parent_slot_type_signature: Option<String>,
    /// Opaque wire-shaped blob for slot_type_values.
    pub slot_type_values: Option<Value>,
    /// Opaque wire-shaped blob for value_selection_setting.
    pub value_selection_setting: Option<Value>,
    /// Opaque wire-shaped blob for external_source_setting.
    pub external_source_setting: Option<Value>,
    /// Opaque wire-shaped blob for composite_slot_type_setting.
    pub composite_slot_type_setting: Option<Value>,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomVocabularyEntryView {
    pub item_id: String,
    pub phrase: String,
    pub display_as: Option<String>,
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomVocabularyMetaView {
    pub bot_id: String,
    pub bot_version: String,
    pub locale_id: String,
    pub status: String,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJobView {
    pub export_id: String,
    pub file_format: String,
    pub export_status: String,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
    pub resource_specification: Option<ExportResourceSpecification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportJobView {
    pub import_id: String,
    pub import_status: String,
    pub merge_strategy: String,
    pub creation_date_time: String,
    pub last_updated_date_time: String,
    pub resource_specification: Option<ImportResourceSpecification>,
    pub imported_resource_id: Option<String>,
    pub imported_resource_name: Option<String>,
}

impl From<&Bot> for BotView {
    fn from(b: &Bot) -> Self {
        Self {
            bot_id: b.bot_id.clone(),
            bot_name: b.bot_name.clone(),
            role_arn: b.role_arn.clone(),
            data_privacy_child_directed: b.data_privacy_child_directed,
            idle_session_ttl_in_seconds: b.idle_session_ttl_in_seconds,
            bot_status: b.bot_status.clone(),
            description: b.description.clone(),
            creation_date_time: b.creation_date_time.clone(),
            last_updated_date_time: b.last_updated_date_time.clone(),
            tags: b.tags.clone(),
        }
    }
}

impl From<BotView> for Bot {
    fn from(v: BotView) -> Self {
        Self {
            bot_id: v.bot_id,
            bot_name: v.bot_name,
            role_arn: v.role_arn,
            data_privacy_child_directed: v.data_privacy_child_directed,
            idle_session_ttl_in_seconds: v.idle_session_ttl_in_seconds,
            bot_status: v.bot_status,
            description: v.description,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
            tags: v.tags,
        }
    }
}

impl From<&BotAlias> for BotAliasView {
    fn from(a: &BotAlias) -> Self {
        Self {
            bot_alias_id: a.bot_alias_id.clone(),
            bot_alias_name: a.bot_alias_name.clone(),
            bot_id: a.bot_id.clone(),
            bot_alias_status: a.bot_alias_status.clone(),
            bot_version: a.bot_version.clone(),
            description: a.description.clone(),
            creation_date_time: a.creation_date_time.clone(),
            last_updated_date_time: a.last_updated_date_time.clone(),
        }
    }
}

impl From<BotAliasView> for BotAlias {
    fn from(v: BotAliasView) -> Self {
        Self {
            bot_alias_id: v.bot_alias_id,
            bot_alias_name: v.bot_alias_name,
            bot_id: v.bot_id,
            bot_alias_status: v.bot_alias_status,
            bot_version: v.bot_version,
            description: v.description,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<&BotLocale> for BotLocaleView {
    fn from(l: &BotLocale) -> Self {
        Self {
            bot_id: l.bot_id.clone(),
            locale_id: l.locale_id.clone(),
            locale_name: l.locale_name.clone(),
            bot_version: l.bot_version.clone(),
            nlu_intent_confidence_threshold: l.nlu_intent_confidence_threshold,
            bot_locale_status: l.bot_locale_status.clone(),
            description: l.description.clone(),
            creation_date_time: l.creation_date_time.clone(),
            last_updated_date_time: l.last_updated_date_time.clone(),
        }
    }
}

impl From<BotLocaleView> for BotLocale {
    fn from(v: BotLocaleView) -> Self {
        Self {
            bot_id: v.bot_id,
            locale_id: v.locale_id,
            locale_name: v.locale_name,
            bot_version: v.bot_version,
            nlu_intent_confidence_threshold: v.nlu_intent_confidence_threshold,
            bot_locale_status: v.bot_locale_status,
            description: v.description,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<&Intent> for IntentView {
    fn from(i: &Intent) -> Self {
        Self {
            intent_id: i.intent_id.clone(),
            intent_name: i.intent_name.clone(),
            bot_id: i.bot_id.clone(),
            bot_version: i.bot_version.clone(),
            locale_id: i.locale_id.clone(),
            description: i.description.clone(),
            creation_date_time: i.creation_date_time.clone(),
            last_updated_date_time: i.last_updated_date_time.clone(),
        }
    }
}

impl From<IntentView> for Intent {
    fn from(v: IntentView) -> Self {
        Self {
            intent_id: v.intent_id,
            intent_name: v.intent_name,
            bot_id: v.bot_id,
            bot_version: v.bot_version,
            locale_id: v.locale_id,
            description: v.description,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<&BotVersion> for BotVersionView {
    fn from(v: &BotVersion) -> Self {
        Self {
            bot_id: v.bot_id.clone(),
            bot_version: v.bot_version.clone(),
            bot_status: v.bot_status.clone(),
            description: v.description.clone(),
            creation_date_time: v.creation_date_time.clone(),
        }
    }
}

impl From<BotVersionView> for BotVersion {
    fn from(v: BotVersionView) -> Self {
        Self {
            bot_id: v.bot_id,
            bot_version: v.bot_version,
            bot_status: v.bot_status,
            description: v.description,
            creation_date_time: v.creation_date_time,
        }
    }
}

impl From<&Slot> for SlotView {
    fn from(s: &Slot) -> Self {
        Self {
            slot_id: s.slot_id.clone(),
            slot_name: s.slot_name.clone(),
            bot_id: s.bot_id.clone(),
            bot_version: s.bot_version.clone(),
            locale_id: s.locale_id.clone(),
            intent_id: s.intent_id.clone(),
            slot_type_id: s.slot_type_id.clone(),
            description: s.description.clone(),
            creation_date_time: s.creation_date_time.clone(),
            last_updated_date_time: s.last_updated_date_time.clone(),
            value_elicitation_setting: serde_json::to_value(&s.value_elicitation_setting)
                .unwrap_or(Value::Null),
            multiple_values_setting: s
                .multiple_values_setting
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
            obfuscation_setting: s
                .obfuscation_setting
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
            sub_slot_setting: s
                .sub_slot_setting
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
        }
    }
}

impl From<SlotView> for Slot {
    fn from(v: SlotView) -> Self {
        Self {
            slot_id: v.slot_id,
            slot_name: v.slot_name,
            bot_id: v.bot_id,
            bot_version: v.bot_version,
            locale_id: v.locale_id,
            intent_id: v.intent_id,
            slot_type_id: v.slot_type_id,
            description: v.description,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
            value_elicitation_setting: serde_json::from_value(v.value_elicitation_setting)
                .unwrap_or_default(),
            multiple_values_setting: v
                .multiple_values_setting
                .and_then(|j| serde_json::from_value(j).ok()),
            obfuscation_setting: v
                .obfuscation_setting
                .and_then(|j| serde_json::from_value(j).ok()),
            sub_slot_setting: v
                .sub_slot_setting
                .and_then(|j| serde_json::from_value(j).ok()),
        }
    }
}

impl From<&SlotType> for SlotTypeView {
    fn from(s: &SlotType) -> Self {
        Self {
            slot_type_id: s.slot_type_id.clone(),
            slot_type_name: s.slot_type_name.clone(),
            bot_id: s.bot_id.clone(),
            bot_version: s.bot_version.clone(),
            locale_id: s.locale_id.clone(),
            description: s.description.clone(),
            parent_slot_type_signature: s.parent_slot_type_signature.clone(),
            slot_type_values: s
                .slot_type_values
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
            value_selection_setting: s
                .value_selection_setting
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
            external_source_setting: s
                .external_source_setting
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
            composite_slot_type_setting: s
                .composite_slot_type_setting
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
            creation_date_time: s.creation_date_time.clone(),
            last_updated_date_time: s.last_updated_date_time.clone(),
        }
    }
}

impl From<SlotTypeView> for SlotType {
    fn from(v: SlotTypeView) -> Self {
        Self {
            slot_type_id: v.slot_type_id,
            slot_type_name: v.slot_type_name,
            bot_id: v.bot_id,
            bot_version: v.bot_version,
            locale_id: v.locale_id,
            description: v.description,
            parent_slot_type_signature: v.parent_slot_type_signature,
            slot_type_values: v
                .slot_type_values
                .and_then(|j| serde_json::from_value(j).ok()),
            value_selection_setting: v
                .value_selection_setting
                .and_then(|j| serde_json::from_value(j).ok()),
            external_source_setting: v
                .external_source_setting
                .and_then(|j| serde_json::from_value(j).ok()),
            composite_slot_type_setting: v
                .composite_slot_type_setting
                .and_then(|j| serde_json::from_value(j).ok()),
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<&CustomVocabularyEntry> for CustomVocabularyEntryView {
    fn from(e: &CustomVocabularyEntry) -> Self {
        Self {
            item_id: e.item_id.clone(),
            phrase: e.phrase.clone(),
            display_as: e.display_as.clone(),
            weight: e.weight,
        }
    }
}

impl From<CustomVocabularyEntryView> for CustomVocabularyEntry {
    fn from(v: CustomVocabularyEntryView) -> Self {
        Self {
            item_id: v.item_id,
            phrase: v.phrase,
            display_as: v.display_as,
            weight: v.weight,
        }
    }
}

impl From<&CustomVocabularyMeta> for CustomVocabularyMetaView {
    fn from(m: &CustomVocabularyMeta) -> Self {
        Self {
            bot_id: m.bot_id.clone(),
            bot_version: m.bot_version.clone(),
            locale_id: m.locale_id.clone(),
            status: m.status.clone(),
            creation_date_time: m.creation_date_time.clone(),
            last_updated_date_time: m.last_updated_date_time.clone(),
        }
    }
}

impl From<CustomVocabularyMetaView> for CustomVocabularyMeta {
    fn from(v: CustomVocabularyMetaView) -> Self {
        Self {
            bot_id: v.bot_id,
            bot_version: v.bot_version,
            locale_id: v.locale_id,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<&ExportJob> for ExportJobView {
    fn from(e: &ExportJob) -> Self {
        Self {
            export_id: e.export_id.clone(),
            file_format: e.file_format.clone(),
            export_status: e.export_status.clone(),
            creation_date_time: e.creation_date_time.clone(),
            last_updated_date_time: e.last_updated_date_time.clone(),
            resource_specification: e.resource_specification.clone(),
        }
    }
}

impl From<ExportJobView> for ExportJob {
    fn from(v: ExportJobView) -> Self {
        Self {
            export_id: v.export_id,
            file_format: v.file_format,
            export_status: v.export_status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
            resource_specification: v.resource_specification,
        }
    }
}

impl From<&ImportJob> for ImportJobView {
    fn from(i: &ImportJob) -> Self {
        Self {
            import_id: i.import_id.clone(),
            import_status: i.import_status.clone(),
            merge_strategy: i.merge_strategy.clone(),
            creation_date_time: i.creation_date_time.clone(),
            last_updated_date_time: i.last_updated_date_time.clone(),
            resource_specification: i.resource_specification.clone(),
            imported_resource_id: i.imported_resource_id.clone(),
            imported_resource_name: i.imported_resource_name.clone(),
        }
    }
}

impl From<ImportJobView> for ImportJob {
    fn from(v: ImportJobView) -> Self {
        Self {
            import_id: v.import_id,
            import_status: v.import_status,
            merge_strategy: v.merge_strategy,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
            resource_specification: v.resource_specification,
            imported_resource_id: v.imported_resource_id,
            imported_resource_name: v.imported_resource_name,
        }
    }
}

impl From<&LexState> for LexStateView {
    fn from(s: &LexState) -> Self {
        Self {
            bots: s
                .bots
                .iter()
                .map(|(k, v)| (k.clone(), BotView::from(v)))
                .collect(),
            bot_aliases: s
                .bot_aliases
                .iter()
                .map(|((bid, aid), v)| (format!("{bid}/{aid}"), BotAliasView::from(v)))
                .collect(),
            bot_locales: s
                .bot_locales
                .iter()
                .map(|((bid, lid), v)| (format!("{bid}/{lid}"), BotLocaleView::from(v)))
                .collect(),
            intents: s
                .intents
                .iter()
                .map(|((bid, lid, iid), v)| (format!("{bid}/{lid}/{iid}"), IntentView::from(v)))
                .collect(),
            bot_versions: s
                .bot_versions
                .iter()
                .map(|((bid, ver), v)| (format!("{bid}/{ver}"), BotVersionView::from(v)))
                .collect(),
            tags: s.tags.clone(),
            resource_policies: s
                .resource_policies
                .iter()
                .map(|(arn, rp)| (arn.clone(), (rp.policy.clone(), rp.revision_id.clone())))
                .collect(),
            slots: s
                .slots
                .iter()
                .map(|((bid, lid, iid, sid), v)| {
                    (format!("{bid}/{lid}/{iid}/{sid}"), SlotView::from(v))
                })
                .collect(),
            slot_types: s
                .slot_types
                .iter()
                .map(|((bid, lid, stid), v)| (format!("{bid}/{lid}/{stid}"), SlotTypeView::from(v)))
                .collect(),
            custom_vocabulary_items: s
                .custom_vocabulary_items
                .iter()
                .map(|((bid, lid, iid), v)| {
                    (
                        format!("{bid}/{lid}/{iid}"),
                        CustomVocabularyEntryView::from(v),
                    )
                })
                .collect(),
            custom_vocabulary_meta: s
                .custom_vocabulary_meta
                .iter()
                .map(|((bid, lid), v)| (format!("{bid}/{lid}"), CustomVocabularyMetaView::from(v)))
                .collect(),
            exports: s
                .exports
                .iter()
                .map(|(k, v)| (k.clone(), ExportJobView::from(v)))
                .collect(),
            imports: s
                .imports
                .iter()
                .map(|(k, v)| (k.clone(), ImportJobView::from(v)))
                .collect(),
        }
    }
}

impl From<LexStateView> for LexState {
    fn from(view: LexStateView) -> Self {
        let mut state = LexState::default();

        for (bot_id, bv) in view.bots {
            state.bots.insert(bot_id, Bot::from(bv));
        }

        for (key, av) in view.bot_aliases {
            let parts: Vec<&str> = key.splitn(2, '/').collect();
            if parts.len() == 2 {
                state.bot_aliases.insert(
                    (parts[0].to_string(), parts[1].to_string()),
                    BotAlias::from(av),
                );
            }
        }

        for (key, lv) in view.bot_locales {
            let parts: Vec<&str> = key.splitn(2, '/').collect();
            if parts.len() == 2 {
                state.bot_locales.insert(
                    (parts[0].to_string(), parts[1].to_string()),
                    BotLocale::from(lv),
                );
            }
        }

        for (key, iv) in view.intents {
            let parts: Vec<&str> = key.splitn(3, '/').collect();
            if parts.len() == 3 {
                state.intents.insert(
                    (
                        parts[0].to_string(),
                        parts[1].to_string(),
                        parts[2].to_string(),
                    ),
                    Intent::from(iv),
                );
            }
        }

        for (key, vv) in view.bot_versions {
            let parts: Vec<&str> = key.splitn(2, '/').collect();
            if parts.len() == 2 {
                state.bot_versions.insert(
                    (parts[0].to_string(), parts[1].to_string()),
                    BotVersion::from(vv),
                );
            }
        }

        state.tags = view.tags;
        state.resource_policies = view
            .resource_policies
            .into_iter()
            .map(|(arn, (policy, revision_id))| {
                (
                    arn.clone(),
                    crate::types::ResourcePolicy {
                        resource_arn: arn,
                        policy,
                        revision_id,
                    },
                )
            })
            .collect();

        for (key, sv) in view.slots {
            let parts: Vec<&str> = key.splitn(4, '/').collect();
            if parts.len() == 4 {
                state.slots.insert(
                    (
                        parts[0].to_string(),
                        parts[1].to_string(),
                        parts[2].to_string(),
                        parts[3].to_string(),
                    ),
                    Slot::from(sv),
                );
            }
        }

        for (key, stv) in view.slot_types {
            let parts: Vec<&str> = key.splitn(3, '/').collect();
            if parts.len() == 3 {
                state.slot_types.insert(
                    (
                        parts[0].to_string(),
                        parts[1].to_string(),
                        parts[2].to_string(),
                    ),
                    SlotType::from(stv),
                );
            }
        }

        for (key, cv) in view.custom_vocabulary_items {
            let parts: Vec<&str> = key.splitn(3, '/').collect();
            if parts.len() == 3 {
                state.custom_vocabulary_items.insert(
                    (
                        parts[0].to_string(),
                        parts[1].to_string(),
                        parts[2].to_string(),
                    ),
                    CustomVocabularyEntry::from(cv),
                );
            }
        }

        for (key, cmv) in view.custom_vocabulary_meta {
            let parts: Vec<&str> = key.splitn(2, '/').collect();
            if parts.len() == 2 {
                state.custom_vocabulary_meta.insert(
                    (parts[0].to_string(), parts[1].to_string()),
                    CustomVocabularyMeta::from(cmv),
                );
            }
        }

        for (export_id, ev) in view.exports {
            state.exports.insert(export_id, ExportJob::from(ev));
        }

        for (import_id, iv) in view.imports {
            state.imports.insert(import_id, ImportJob::from(iv));
        }

        state
    }
}

impl StatefulService for LexModelsV2Service {
    type StateView = LexStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        LexStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = LexState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let incoming = LexState::from(view);
            for (k, v) in incoming.bots {
                guard.bots.insert(k, v);
            }
            for (k, v) in incoming.bot_aliases {
                guard.bot_aliases.insert(k, v);
            }
            for (k, v) in incoming.bot_locales {
                guard.bot_locales.insert(k, v);
            }
            for (k, v) in incoming.intents {
                guard.intents.insert(k, v);
            }
            for (k, v) in incoming.bot_versions {
                guard.bot_versions.insert(k, v);
            }
            for (arn, tags) in incoming.tags {
                guard.tags.entry(arn).or_default().extend(tags);
            }
            for (arn, rp) in incoming.resource_policies {
                guard.resource_policies.insert(arn, rp);
            }
            for (k, v) in incoming.slots {
                guard.slots.insert(k, v);
            }
            for (k, v) in incoming.slot_types {
                guard.slot_types.insert(k, v);
            }
            for (k, v) in incoming.custom_vocabulary_items {
                guard.custom_vocabulary_items.insert(k, v);
            }
            for (k, v) in incoming.custom_vocabulary_meta {
                guard.custom_vocabulary_meta.insert(k, v);
            }
            for (k, v) in incoming.exports {
                guard.exports.insert(k, v);
            }
            for (k, v) in incoming.imports {
                guard.imports.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
