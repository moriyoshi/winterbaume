use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::model::{
    CompositeSlotTypeSetting, ExportResourceSpecification, ExternalSourceSetting,
    ImportResourceSpecification, MultipleValuesSetting, ObfuscationSetting, SlotTypeValue,
    SlotValueElicitationSetting, SlotValueSelectionSetting, SubSlotSetting,
};
use crate::types::{
    Bot, BotAlias, BotLocale, BotVersion, CustomVocabularyEntry, CustomVocabularyMeta, ExportJob,
    ImportJob, Intent, ResourcePolicy, Slot, SlotType,
};

#[derive(Debug, Default)]
pub struct LexState {
    /// bots keyed by botId
    pub bots: HashMap<String, Bot>,
    /// aliases keyed by (botId, botAliasId)
    pub bot_aliases: HashMap<(String, String), BotAlias>,
    /// locales keyed by (botId, localeId)
    pub bot_locales: HashMap<(String, String), BotLocale>,
    /// intents keyed by (botId, localeId, intentId)
    pub intents: HashMap<(String, String, String), Intent>,
    /// bot versions keyed by (botId, version)
    pub bot_versions: HashMap<(String, String), BotVersion>,
    /// tags keyed by resourceArn
    pub tags: HashMap<String, HashMap<String, String>>,
    /// resource policies keyed by resourceArn
    pub resource_policies: HashMap<String, ResourcePolicy>,
    /// slots keyed by (botId, localeId, intentId, slotId)
    pub slots: HashMap<(String, String, String, String), Slot>,
    /// slot types keyed by (botId, localeId, slotTypeId)
    pub slot_types: HashMap<(String, String, String), SlotType>,
    /// custom vocabulary items keyed by (botId, localeId, itemId)
    pub custom_vocabulary_items: HashMap<(String, String, String), CustomVocabularyEntry>,
    /// custom vocabulary metadata keyed by (botId, localeId)
    pub custom_vocabulary_meta: HashMap<(String, String), CustomVocabularyMeta>,
    /// export jobs keyed by exportId
    pub exports: HashMap<String, ExportJob>,
    /// import jobs keyed by importId
    pub imports: HashMap<String, ImportJob>,
}

#[derive(Debug, thiserror::Error)]
pub enum LexError {
    #[error("Could not find {resource_type}: {id}")]
    ResourceNotFound { resource_type: String, id: String },

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    Validation(String),
}

impl LexError {
    pub fn resource_not_found(resource_type: &str, id: &str) -> Self {
        Self::ResourceNotFound {
            resource_type: resource_type.to_string(),
            id: id.to_string(),
        }
    }

    pub fn conflict(msg: &str) -> Self {
        Self::Conflict(msg.to_string())
    }

    pub fn validation(msg: &str) -> Self {
        Self::Validation(msg.to_string())
    }
}

fn now_rfc3339() -> String {
    use chrono::SecondsFormat;
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

impl LexState {
    // ---- Bot CRUD ----

    pub fn create_bot(
        &mut self,
        bot_name: String,
        role_arn: String,
        data_privacy_child_directed: bool,
        idle_session_ttl_in_seconds: i32,
        description: Option<String>,
        bot_tags: Option<HashMap<String, String>>,
    ) -> Result<Bot, LexError> {
        // Check name uniqueness
        if self.bots.values().any(|b| b.bot_name == bot_name) {
            return Err(LexError::conflict(&format!(
                "Bot with name '{bot_name}' already exists"
            )));
        }

        let bot_id = Uuid::new_v4().to_string().replace('-', "")[..10].to_uppercase();
        let now = now_rfc3339();
        let tags = bot_tags.unwrap_or_default();

        let arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{bot_id}");

        let bot = Bot {
            bot_id: bot_id.clone(),
            bot_name,
            role_arn,
            data_privacy_child_directed,
            idle_session_ttl_in_seconds,
            bot_status: "Available".to_string(),
            description,
            creation_date_time: now.clone(),
            last_updated_date_time: now,
            tags: tags.clone(),
        };

        self.bots.insert(bot_id.clone(), bot.clone());
        if !tags.is_empty() {
            self.tags.insert(arn, tags);
        }

        Ok(bot)
    }

    pub fn describe_bot(&self, bot_id: &str) -> Result<&Bot, LexError> {
        self.bots
            .get(bot_id)
            .ok_or_else(|| LexError::resource_not_found("Bot", bot_id))
    }

    pub fn list_bots(&self) -> Vec<&Bot> {
        let mut bots: Vec<&Bot> = self.bots.values().collect();
        bots.sort_by_key(|b| &b.bot_name);
        bots
    }

    pub fn delete_bot(&mut self, bot_id: &str) -> Result<String, LexError> {
        let bot = self
            .bots
            .remove(bot_id)
            .ok_or_else(|| LexError::resource_not_found("Bot", bot_id))?;

        // Remove associated resources
        self.bot_aliases.retain(|(bid, _), _| bid != bot_id);
        self.bot_locales.retain(|(bid, _), _| bid != bot_id);
        self.intents.retain(|(bid, _, _), _| bid != bot_id);
        self.bot_versions.retain(|(bid, _), _| bid != bot_id);

        let arn = format!("arn:aws:lex:us-east-1:123456789012:bot/{}", bot.bot_id);
        self.tags.remove(&arn);

        Ok(bot_id.to_string())
    }

    pub fn update_bot(
        &mut self,
        bot_id: &str,
        bot_name: String,
        role_arn: String,
        data_privacy_child_directed: bool,
        idle_session_ttl_in_seconds: i32,
        description: Option<String>,
    ) -> Result<Bot, LexError> {
        let bot = self
            .bots
            .get_mut(bot_id)
            .ok_or_else(|| LexError::resource_not_found("Bot", bot_id))?;

        bot.bot_name = bot_name;
        bot.role_arn = role_arn;
        bot.data_privacy_child_directed = data_privacy_child_directed;
        bot.idle_session_ttl_in_seconds = idle_session_ttl_in_seconds;
        bot.description = description;
        bot.last_updated_date_time = now_rfc3339();
        bot.bot_status = "Available".to_string();

        Ok(bot.clone())
    }

    // ---- BotAlias CRUD ----

    pub fn create_bot_alias(
        &mut self,
        bot_id: &str,
        bot_alias_name: String,
        bot_version: Option<String>,
        description: Option<String>,
    ) -> Result<BotAlias, LexError> {
        // Verify bot exists
        if !self.bots.contains_key(bot_id) {
            return Err(LexError::resource_not_found("Bot", bot_id));
        }

        let alias_id = Uuid::new_v4().to_string().replace('-', "")[..10].to_uppercase();
        let now = now_rfc3339();

        let alias = BotAlias {
            bot_alias_id: alias_id.clone(),
            bot_alias_name,
            bot_id: bot_id.to_string(),
            bot_alias_status: "Available".to_string(),
            bot_version,
            description,
            creation_date_time: now.clone(),
            last_updated_date_time: now,
        };

        self.bot_aliases
            .insert((bot_id.to_string(), alias_id), alias.clone());
        Ok(alias)
    }

    pub fn describe_bot_alias(&self, bot_id: &str, alias_id: &str) -> Result<&BotAlias, LexError> {
        self.bot_aliases
            .get(&(bot_id.to_string(), alias_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotAlias", alias_id))
    }

    pub fn list_bot_aliases(&self, bot_id: &str) -> Vec<&BotAlias> {
        let mut aliases: Vec<&BotAlias> = self
            .bot_aliases
            .iter()
            .filter(|((bid, _), _)| bid == bot_id)
            .map(|(_, a)| a)
            .collect();
        aliases.sort_by_key(|a| &a.bot_alias_name);
        aliases
    }

    pub fn delete_bot_alias(
        &mut self,
        bot_id: &str,
        alias_id: &str,
    ) -> Result<(String, String), LexError> {
        self.bot_aliases
            .remove(&(bot_id.to_string(), alias_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotAlias", alias_id))?;
        Ok((bot_id.to_string(), alias_id.to_string()))
    }

    pub fn update_bot_alias(
        &mut self,
        bot_id: &str,
        alias_id: &str,
        bot_alias_name: String,
        bot_version: Option<String>,
        description: Option<String>,
    ) -> Result<BotAlias, LexError> {
        let alias = self
            .bot_aliases
            .get_mut(&(bot_id.to_string(), alias_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotAlias", alias_id))?;

        alias.bot_alias_name = bot_alias_name;
        alias.bot_version = bot_version;
        alias.description = description;
        alias.last_updated_date_time = now_rfc3339();
        alias.bot_alias_status = "Available".to_string();

        Ok(alias.clone())
    }

    // ---- BotLocale CRUD ----

    pub fn create_bot_locale(
        &mut self,
        bot_id: &str,
        locale_id: String,
        nlu_intent_confidence_threshold: f64,
        description: Option<String>,
    ) -> Result<BotLocale, LexError> {
        if !self.bots.contains_key(bot_id) {
            return Err(LexError::resource_not_found("Bot", bot_id));
        }

        if self
            .bot_locales
            .contains_key(&(bot_id.to_string(), locale_id.clone()))
        {
            return Err(LexError::conflict(&format!(
                "BotLocale {locale_id} already exists for bot {bot_id}"
            )));
        }

        let locale_name = locale_id_to_name(&locale_id);
        let now = now_rfc3339();

        let locale = BotLocale {
            bot_id: bot_id.to_string(),
            locale_id: locale_id.clone(),
            locale_name,
            bot_version: "DRAFT".to_string(),
            nlu_intent_confidence_threshold,
            bot_locale_status: "NotBuilt".to_string(),
            description,
            creation_date_time: now.clone(),
            last_updated_date_time: now,
        };

        self.bot_locales
            .insert((bot_id.to_string(), locale_id), locale.clone());
        Ok(locale)
    }

    pub fn describe_bot_locale(
        &self,
        bot_id: &str,
        locale_id: &str,
    ) -> Result<&BotLocale, LexError> {
        self.bot_locales
            .get(&(bot_id.to_string(), locale_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotLocale", locale_id))
    }

    pub fn list_bot_locales(&self, bot_id: &str) -> Vec<&BotLocale> {
        let mut locales: Vec<&BotLocale> = self
            .bot_locales
            .iter()
            .filter(|((bid, _), _)| bid == bot_id)
            .map(|(_, l)| l)
            .collect();
        locales.sort_by_key(|l| &l.locale_id);
        locales
    }

    pub fn delete_bot_locale(
        &mut self,
        bot_id: &str,
        locale_id: &str,
    ) -> Result<(String, String), LexError> {
        self.bot_locales
            .remove(&(bot_id.to_string(), locale_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotLocale", locale_id))?;

        // Remove associated intents
        self.intents
            .retain(|(bid, lid, _), _| !(bid == bot_id && lid == locale_id));

        Ok((bot_id.to_string(), locale_id.to_string()))
    }

    pub fn update_bot_locale(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        nlu_intent_confidence_threshold: f64,
        description: Option<String>,
    ) -> Result<BotLocale, LexError> {
        let locale = self
            .bot_locales
            .get_mut(&(bot_id.to_string(), locale_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotLocale", locale_id))?;

        locale.nlu_intent_confidence_threshold = nlu_intent_confidence_threshold;
        locale.description = description;
        locale.last_updated_date_time = now_rfc3339();
        locale.bot_locale_status = "NotBuilt".to_string();

        Ok(locale.clone())
    }

    pub fn build_bot_locale(
        &mut self,
        bot_id: &str,
        locale_id: &str,
    ) -> Result<(String, String, String), LexError> {
        let locale = self
            .bot_locales
            .get_mut(&(bot_id.to_string(), locale_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotLocale", locale_id))?;

        locale.bot_locale_status = "Built".to_string();
        locale.last_updated_date_time = now_rfc3339();

        Ok((
            bot_id.to_string(),
            locale.bot_version.clone(),
            locale_id.to_string(),
        ))
    }

    // ---- Intent CRUD ----

    pub fn create_intent(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        intent_name: String,
        description: Option<String>,
    ) -> Result<Intent, LexError> {
        if !self
            .bot_locales
            .contains_key(&(bot_id.to_string(), locale_id.to_string()))
        {
            return Err(LexError::resource_not_found("BotLocale", locale_id));
        }

        let intent_id = Uuid::new_v4().to_string().replace('-', "")[..10].to_uppercase();
        let now = now_rfc3339();

        let intent = Intent {
            intent_id: intent_id.clone(),
            intent_name,
            bot_id: bot_id.to_string(),
            bot_version: "DRAFT".to_string(),
            locale_id: locale_id.to_string(),
            description,
            creation_date_time: now.clone(),
            last_updated_date_time: now,
        };

        self.intents.insert(
            (bot_id.to_string(), locale_id.to_string(), intent_id),
            intent.clone(),
        );
        Ok(intent)
    }

    pub fn describe_intent(
        &self,
        bot_id: &str,
        locale_id: &str,
        intent_id: &str,
    ) -> Result<&Intent, LexError> {
        self.intents
            .get(&(
                bot_id.to_string(),
                locale_id.to_string(),
                intent_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("Intent", intent_id))
    }

    pub fn list_intents(&self, bot_id: &str, locale_id: &str) -> Vec<&Intent> {
        let mut intents: Vec<&Intent> = self
            .intents
            .iter()
            .filter(|((bid, lid, _), _)| bid == bot_id && lid == locale_id)
            .map(|(_, i)| i)
            .collect();
        intents.sort_by_key(|i| &i.intent_name);
        intents
    }

    pub fn delete_intent(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        intent_id: &str,
    ) -> Result<(), LexError> {
        self.intents
            .remove(&(
                bot_id.to_string(),
                locale_id.to_string(),
                intent_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("Intent", intent_id))?;
        Ok(())
    }

    pub fn update_intent(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        intent_id: &str,
        intent_name: String,
        description: Option<String>,
    ) -> Result<Intent, LexError> {
        let intent = self
            .intents
            .get_mut(&(
                bot_id.to_string(),
                locale_id.to_string(),
                intent_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("Intent", intent_id))?;

        intent.intent_name = intent_name;
        intent.description = description;
        intent.last_updated_date_time = now_rfc3339();

        Ok(intent.clone())
    }

    // ---- BotVersion CRUD ----

    pub fn create_bot_version(
        &mut self,
        bot_id: &str,
        description: Option<String>,
    ) -> Result<BotVersion, LexError> {
        if !self.bots.contains_key(bot_id) {
            return Err(LexError::resource_not_found("Bot", bot_id));
        }

        // Find next version number
        let max_ver: u32 = self
            .bot_versions
            .keys()
            .filter(|(bid, _)| bid == bot_id)
            .filter_map(|(_, v)| v.parse::<u32>().ok())
            .max()
            .unwrap_or(0);
        let version = (max_ver + 1).to_string();

        let now = now_rfc3339();
        let bv = BotVersion {
            bot_id: bot_id.to_string(),
            bot_version: version.clone(),
            bot_status: "Available".to_string(),
            description,
            creation_date_time: now,
        };

        self.bot_versions
            .insert((bot_id.to_string(), version), bv.clone());
        Ok(bv)
    }

    pub fn describe_bot_version(
        &self,
        bot_id: &str,
        version: &str,
    ) -> Result<&BotVersion, LexError> {
        self.bot_versions
            .get(&(bot_id.to_string(), version.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotVersion", version))
    }

    pub fn list_bot_versions(&self, bot_id: &str) -> Vec<&BotVersion> {
        let mut versions: Vec<&BotVersion> = self
            .bot_versions
            .iter()
            .filter(|((bid, _), _)| bid == bot_id)
            .map(|(_, v)| v)
            .collect();
        versions.sort_by(|a, b| {
            let an: u32 = a.bot_version.parse().unwrap_or(0);
            let bn: u32 = b.bot_version.parse().unwrap_or(0);
            bn.cmp(&an) // descending
        });
        versions
    }

    pub fn delete_bot_version(
        &mut self,
        bot_id: &str,
        version: &str,
    ) -> Result<(String, String), LexError> {
        self.bot_versions
            .remove(&(bot_id.to_string(), version.to_string()))
            .ok_or_else(|| LexError::resource_not_found("BotVersion", version))?;
        Ok((bot_id.to_string(), version.to_string()))
    }

    // ---- Tags ----

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), LexError> {
        self.tags
            .entry(resource_arn.to_string())
            .or_default()
            .extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: Vec<String>,
    ) -> Result<(), LexError> {
        if let Some(existing) = self.tags.get_mut(resource_arn) {
            for key in &tag_keys {
                existing.remove(key);
            }
        }
        Ok(())
    }

    // ---- Resource Policies ----

    pub fn create_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: String,
    ) -> Result<ResourcePolicy, LexError> {
        if self.resource_policies.contains_key(resource_arn) {
            return Err(LexError::conflict(&format!(
                "Resource policy already exists for: {resource_arn}"
            )));
        }
        let rp = ResourcePolicy {
            resource_arn: resource_arn.to_string(),
            policy,
            revision_id: Uuid::new_v4().to_string(),
        };
        self.resource_policies
            .insert(resource_arn.to_string(), rp.clone());
        Ok(rp)
    }

    pub fn describe_resource_policy(
        &self,
        resource_arn: &str,
    ) -> Result<&ResourcePolicy, LexError> {
        self.resource_policies
            .get(resource_arn)
            .ok_or_else(|| LexError::resource_not_found("ResourcePolicy", resource_arn))
    }

    pub fn update_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: String,
    ) -> Result<ResourcePolicy, LexError> {
        let rp = self
            .resource_policies
            .get_mut(resource_arn)
            .ok_or_else(|| LexError::resource_not_found("ResourcePolicy", resource_arn))?;
        rp.policy = policy;
        rp.revision_id = Uuid::new_v4().to_string();
        Ok(rp.clone())
    }

    pub fn delete_resource_policy(
        &mut self,
        resource_arn: &str,
    ) -> Result<ResourcePolicy, LexError> {
        self.resource_policies
            .remove(resource_arn)
            .ok_or_else(|| LexError::resource_not_found("ResourcePolicy", resource_arn))
    }

    // ---- Slot CRUD ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_slot(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        intent_id: &str,
        slot_name: String,
        slot_type_id: Option<String>,
        description: Option<String>,
        value_elicitation_setting: SlotValueElicitationSetting,
        multiple_values_setting: Option<MultipleValuesSetting>,
        obfuscation_setting: Option<ObfuscationSetting>,
        sub_slot_setting: Option<SubSlotSetting>,
    ) -> Result<Slot, LexError> {
        let slot_id = Uuid::new_v4().to_string().replace('-', "")[..10].to_uppercase();
        let now = now_rfc3339();

        let slot = Slot {
            slot_id: slot_id.clone(),
            slot_name,
            bot_id: bot_id.to_string(),
            bot_version: "DRAFT".to_string(),
            locale_id: locale_id.to_string(),
            intent_id: intent_id.to_string(),
            slot_type_id,
            description,
            creation_date_time: now.clone(),
            last_updated_date_time: now,
            value_elicitation_setting,
            multiple_values_setting,
            obfuscation_setting,
            sub_slot_setting,
        };

        self.slots.insert(
            (
                bot_id.to_string(),
                locale_id.to_string(),
                intent_id.to_string(),
                slot_id,
            ),
            slot.clone(),
        );
        Ok(slot)
    }

    pub fn describe_slot(
        &self,
        bot_id: &str,
        locale_id: &str,
        intent_id: &str,
        slot_id: &str,
    ) -> Result<&Slot, LexError> {
        self.slots
            .get(&(
                bot_id.to_string(),
                locale_id.to_string(),
                intent_id.to_string(),
                slot_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("Slot", slot_id))
    }

    pub fn list_slots(&self, bot_id: &str, locale_id: &str, intent_id: &str) -> Vec<&Slot> {
        let mut slots: Vec<&Slot> = self
            .slots
            .iter()
            .filter(|((bid, lid, iid, _), _)| bid == bot_id && lid == locale_id && iid == intent_id)
            .map(|(_, s)| s)
            .collect();
        slots.sort_by_key(|s| &s.slot_name);
        slots
    }

    pub fn delete_slot(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        intent_id: &str,
        slot_id: &str,
    ) -> Result<(), LexError> {
        self.slots
            .remove(&(
                bot_id.to_string(),
                locale_id.to_string(),
                intent_id.to_string(),
                slot_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("Slot", slot_id))?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_slot(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        intent_id: &str,
        slot_id: &str,
        slot_name: String,
        slot_type_id: Option<String>,
        description: Option<String>,
        value_elicitation_setting: SlotValueElicitationSetting,
        multiple_values_setting: Option<MultipleValuesSetting>,
        obfuscation_setting: Option<ObfuscationSetting>,
        sub_slot_setting: Option<SubSlotSetting>,
    ) -> Result<Slot, LexError> {
        let slot = self
            .slots
            .get_mut(&(
                bot_id.to_string(),
                locale_id.to_string(),
                intent_id.to_string(),
                slot_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("Slot", slot_id))?;

        slot.slot_name = slot_name;
        slot.slot_type_id = slot_type_id;
        slot.description = description;
        slot.value_elicitation_setting = value_elicitation_setting;
        slot.multiple_values_setting = multiple_values_setting;
        slot.obfuscation_setting = obfuscation_setting;
        slot.sub_slot_setting = sub_slot_setting;
        slot.last_updated_date_time = now_rfc3339();

        Ok(slot.clone())
    }

    // ---- SlotType CRUD ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_slot_type(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        slot_type_name: String,
        description: Option<String>,
        parent_slot_type_signature: Option<String>,
        slot_type_values: Option<Vec<SlotTypeValue>>,
        value_selection_setting: Option<SlotValueSelectionSetting>,
        external_source_setting: Option<ExternalSourceSetting>,
        composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    ) -> Result<SlotType, LexError> {
        let slot_type_id = Uuid::new_v4().to_string().replace('-', "")[..10].to_uppercase();
        let now = now_rfc3339();

        let st = SlotType {
            slot_type_id: slot_type_id.clone(),
            slot_type_name,
            bot_id: bot_id.to_string(),
            bot_version: "DRAFT".to_string(),
            locale_id: locale_id.to_string(),
            description,
            parent_slot_type_signature,
            slot_type_values,
            value_selection_setting,
            external_source_setting,
            composite_slot_type_setting,
            creation_date_time: now.clone(),
            last_updated_date_time: now,
        };

        self.slot_types.insert(
            (bot_id.to_string(), locale_id.to_string(), slot_type_id),
            st.clone(),
        );
        Ok(st)
    }

    pub fn describe_slot_type(
        &self,
        bot_id: &str,
        locale_id: &str,
        slot_type_id: &str,
    ) -> Result<&SlotType, LexError> {
        self.slot_types
            .get(&(
                bot_id.to_string(),
                locale_id.to_string(),
                slot_type_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("SlotType", slot_type_id))
    }

    pub fn list_slot_types(&self, bot_id: &str, locale_id: &str) -> Vec<&SlotType> {
        let mut types: Vec<&SlotType> = self
            .slot_types
            .iter()
            .filter(|((bid, lid, _), _)| bid == bot_id && lid == locale_id)
            .map(|(_, s)| s)
            .collect();
        types.sort_by_key(|s| &s.slot_type_name);
        types
    }

    pub fn delete_slot_type(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        slot_type_id: &str,
    ) -> Result<(), LexError> {
        self.slot_types
            .remove(&(
                bot_id.to_string(),
                locale_id.to_string(),
                slot_type_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("SlotType", slot_type_id))?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_slot_type(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        slot_type_id: &str,
        slot_type_name: String,
        description: Option<String>,
        parent_slot_type_signature: Option<String>,
        slot_type_values: Option<Vec<SlotTypeValue>>,
        value_selection_setting: Option<SlotValueSelectionSetting>,
        external_source_setting: Option<ExternalSourceSetting>,
        composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    ) -> Result<SlotType, LexError> {
        let st = self
            .slot_types
            .get_mut(&(
                bot_id.to_string(),
                locale_id.to_string(),
                slot_type_id.to_string(),
            ))
            .ok_or_else(|| LexError::resource_not_found("SlotType", slot_type_id))?;

        st.slot_type_name = slot_type_name;
        st.description = description;
        st.parent_slot_type_signature = parent_slot_type_signature;
        st.slot_type_values = slot_type_values;
        st.value_selection_setting = value_selection_setting;
        st.external_source_setting = external_source_setting;
        st.composite_slot_type_setting = composite_slot_type_setting;
        st.last_updated_date_time = now_rfc3339();

        Ok(st.clone())
    }

    // ---- CustomVocabulary operations ----

    pub fn batch_create_custom_vocabulary_items(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        items: Vec<crate::model::CustomVocabularyItem>,
    ) -> Result<Vec<crate::model::CustomVocabularyItem>, LexError> {
        let now = now_rfc3339();
        let key = (bot_id.to_string(), locale_id.to_string());
        // Ensure metadata entry exists
        self.custom_vocabulary_meta
            .entry(key.clone())
            .or_insert_with(|| CustomVocabularyMeta {
                bot_id: bot_id.to_string(),
                bot_version: "DRAFT".to_string(),
                locale_id: locale_id.to_string(),
                status: "Ready".to_string(),
                creation_date_time: now.clone(),
                last_updated_date_time: now,
            });

        let mut created = Vec::new();
        for item in items {
            let item_id = if item.item_id.is_empty() {
                Uuid::new_v4().to_string()
            } else {
                item.item_id.clone()
            };
            let entry = CustomVocabularyEntry {
                item_id: item_id.clone(),
                phrase: item.phrase.clone(),
                display_as: item.display_as.clone(),
                weight: item.weight,
            };
            self.custom_vocabulary_items.insert(
                (bot_id.to_string(), locale_id.to_string(), item_id.clone()),
                entry,
            );
            created.push(crate::model::CustomVocabularyItem {
                item_id,
                phrase: item.phrase,
                display_as: item.display_as,
                weight: item.weight,
            });
        }
        Ok(created)
    }

    pub fn batch_delete_custom_vocabulary_items(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        item_ids: Vec<String>,
    ) -> Vec<String> {
        let mut deleted = Vec::new();
        for item_id in item_ids {
            if self
                .custom_vocabulary_items
                .remove(&(bot_id.to_string(), locale_id.to_string(), item_id.clone()))
                .is_some()
            {
                deleted.push(item_id);
            }
        }
        deleted
    }

    pub fn batch_update_custom_vocabulary_items(
        &mut self,
        bot_id: &str,
        locale_id: &str,
        items: Vec<crate::model::CustomVocabularyItem>,
    ) -> Result<Vec<crate::model::CustomVocabularyItem>, LexError> {
        let mut updated = Vec::new();
        for item in items {
            let key = (
                bot_id.to_string(),
                locale_id.to_string(),
                item.item_id.clone(),
            );
            if let Some(entry) = self.custom_vocabulary_items.get_mut(&key) {
                entry.phrase = item.phrase.clone();
                entry.display_as = item.display_as.clone();
                entry.weight = item.weight;
                updated.push(crate::model::CustomVocabularyItem {
                    item_id: item.item_id,
                    phrase: item.phrase,
                    display_as: item.display_as,
                    weight: item.weight,
                });
            }
        }
        Ok(updated)
    }

    pub fn list_custom_vocabulary_items(
        &self,
        bot_id: &str,
        locale_id: &str,
    ) -> Vec<&CustomVocabularyEntry> {
        let mut items: Vec<&CustomVocabularyEntry> = self
            .custom_vocabulary_items
            .iter()
            .filter(|((bid, lid, _), _)| bid == bot_id && lid == locale_id)
            .map(|(_, e)| e)
            .collect();
        items.sort_by_key(|e| &e.phrase);
        items
    }

    pub fn describe_custom_vocabulary_metadata(
        &self,
        bot_id: &str,
        locale_id: &str,
    ) -> Result<&CustomVocabularyMeta, LexError> {
        self.custom_vocabulary_meta
            .get(&(bot_id.to_string(), locale_id.to_string()))
            .ok_or_else(|| LexError::resource_not_found("CustomVocabulary", locale_id))
    }

    pub fn delete_custom_vocabulary(
        &mut self,
        bot_id: &str,
        locale_id: &str,
    ) -> Result<(), LexError> {
        if self
            .custom_vocabulary_meta
            .remove(&(bot_id.to_string(), locale_id.to_string()))
            .is_none()
        {
            return Err(LexError::resource_not_found("CustomVocabulary", locale_id));
        }
        self.custom_vocabulary_items
            .retain(|(bid, lid, _), _| !(bid == bot_id && lid == locale_id));
        Ok(())
    }

    // ---- Export operations ----

    pub fn create_export(
        &mut self,
        file_format: String,
        resource_specification: Option<ExportResourceSpecification>,
    ) -> Result<ExportJob, LexError> {
        let export_id = Uuid::new_v4().to_string();
        let now = now_rfc3339();

        let job = ExportJob {
            export_id: export_id.clone(),
            file_format,
            export_status: "Completed".to_string(),
            creation_date_time: now.clone(),
            last_updated_date_time: now,
            resource_specification,
        };

        self.exports.insert(export_id, job.clone());
        Ok(job)
    }

    pub fn describe_export(&self, export_id: &str) -> Result<&ExportJob, LexError> {
        self.exports
            .get(export_id)
            .ok_or_else(|| LexError::resource_not_found("Export", export_id))
    }

    pub fn delete_export(&mut self, export_id: &str) -> Result<ExportJob, LexError> {
        self.exports
            .remove(export_id)
            .ok_or_else(|| LexError::resource_not_found("Export", export_id))
    }

    pub fn list_exports(&self) -> Vec<&ExportJob> {
        let mut exports: Vec<&ExportJob> = self.exports.values().collect();
        exports.sort_by(|a, b| {
            b.creation_date_time
                .partial_cmp(&a.creation_date_time)
                .unwrap()
        });
        exports
    }

    pub fn update_export(
        &mut self,
        export_id: &str,
        file_format: String,
    ) -> Result<ExportJob, LexError> {
        let job = self
            .exports
            .get_mut(export_id)
            .ok_or_else(|| LexError::resource_not_found("Export", export_id))?;
        job.file_format = file_format;
        job.last_updated_date_time = now_rfc3339();
        Ok(job.clone())
    }

    // ---- Import operations ----

    pub fn start_import(
        &mut self,
        import_id: String,
        merge_strategy: String,
        resource_specification: Option<ImportResourceSpecification>,
    ) -> Result<ImportJob, LexError> {
        let now = now_rfc3339();

        let job = ImportJob {
            import_id: import_id.clone(),
            import_status: "Completed".to_string(),
            merge_strategy,
            creation_date_time: now.clone(),
            last_updated_date_time: now,
            resource_specification,
            imported_resource_id: None,
            imported_resource_name: None,
        };

        self.imports.insert(import_id, job.clone());
        Ok(job)
    }

    pub fn describe_import(&self, import_id: &str) -> Result<&ImportJob, LexError> {
        self.imports
            .get(import_id)
            .ok_or_else(|| LexError::resource_not_found("Import", import_id))
    }

    pub fn delete_import(&mut self, import_id: &str) -> Result<ImportJob, LexError> {
        self.imports
            .remove(import_id)
            .ok_or_else(|| LexError::resource_not_found("Import", import_id))
    }

    pub fn list_imports(&self) -> Vec<&ImportJob> {
        let mut imports: Vec<&ImportJob> = self.imports.values().collect();
        imports.sort_by(|a, b| {
            b.creation_date_time
                .partial_cmp(&a.creation_date_time)
                .unwrap()
        });
        imports
    }

    // ---- Utterances operations ----

    pub fn delete_utterances(&mut self, _bot_id: &str) -> Result<(), LexError> {
        // In real Lex, utterances are analytics data; we just return success
        Ok(())
    }
}

/// Map a Lex locale ID to a human-readable locale name.
fn locale_id_to_name(locale_id: &str) -> String {
    match locale_id {
        "en_US" => "English (US)".to_string(),
        "en_GB" => "English (GB)".to_string(),
        "de_DE" => "German (DE)".to_string(),
        "es_ES" => "Spanish (ES)".to_string(),
        "es_US" => "Spanish (US)".to_string(),
        "fr_FR" => "French (FR)".to_string(),
        "fr_CA" => "French (CA)".to_string(),
        "it_IT" => "Italian (IT)".to_string(),
        "ja_JP" => "Japanese (JP)".to_string(),
        "ko_KR" => "Korean (KR)".to_string(),
        "pt_BR" => "Portuguese (BR)".to_string(),
        "pt_PT" => "Portuguese (PT)".to_string(),
        "zh_CN" => "Chinese Simplified (PRC)".to_string(),
        "nl_NL" => "Dutch".to_string(),
        other => other.to_string(),
    }
}
