use std::collections::HashMap;

use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct WafV2State {
    pub web_acls: HashMap<String, WebAcl>,
    pub ip_sets: HashMap<String, IpSet>,
    pub regex_pattern_sets: HashMap<String, RegexPatternSet>,
    pub rule_groups: HashMap<String, RuleGroupData>,
    pub logging_configs: HashMap<String, LoggingConfigData>,
    /// resource_arn -> web_acl_arn
    pub web_acl_associations: HashMap<String, String>,
    /// resource_arn -> policy JSON
    pub permission_policies: HashMap<String, String>,
    /// (scope, token) -> ApiKeyData
    pub api_keys: HashMap<String, ApiKeyData>,
}

#[derive(Debug, thiserror::Error)]
pub enum WafV2Error {
    #[error("AWS WAF couldn't perform the operation because your resource doesn't exist.")]
    NonexistentItem,
    #[error(
        "AWS WAF could not perform the operation because some resource in your request is a duplicate of an existing one."
    )]
    DuplicateItem,
    #[error(
        "AWS WAF couldn't save your changes because someone changed the resource after you started to edit it."
    )]
    OptimisticLock,
}

fn scope_path(scope: &str) -> &str {
    match scope {
        "CLOUDFRONT" => "global",
        _ => "regional",
    }
}

fn make_arn(
    region: &str,
    account_id: &str,
    scope: &str,
    resource_type: &str,
    name: &str,
    id: &str,
) -> String {
    format!(
        "arn:aws:wafv2:{region}:{account_id}:{}/{resource_type}/{name}/{id}",
        scope_path(scope)
    )
}

fn nonexistent_error() -> WafV2Error {
    WafV2Error::NonexistentItem
}

fn duplicate_error() -> WafV2Error {
    WafV2Error::DuplicateItem
}

fn optimistic_lock_error() -> WafV2Error {
    WafV2Error::OptimisticLock
}

fn extract_tags(tags: &[(String, String)]) -> Vec<(String, String)> {
    tags.to_vec()
}

impl WafV2State {
    // ── WebACL ──

    pub fn create_web_acl(
        &mut self,
        name: &str,
        scope: &str,
        description: &str,
        default_action_json: serde_json::Value,
        visibility_config_json: serde_json::Value,
        region: &str,
        account_id: &str,
        rules_json: serde_json::Value,
        association_config_json: Option<serde_json::Value>,
        custom_response_bodies_json: Option<serde_json::Value>,
        captcha_config_json: Option<serde_json::Value>,
        challenge_config_json: Option<serde_json::Value>,
        token_domains: Option<Vec<String>>,
        tags: Vec<(String, String)>,
    ) -> Result<&WebAcl, WafV2Error> {
        let key = format!("{scope}:{name}");
        if self.web_acls.contains_key(&key) {
            return Err(duplicate_error());
        }

        let id = Uuid::new_v4().to_string();
        let lock_token = Uuid::new_v4().to_string();
        let arn = make_arn(region, account_id, scope, "webacl", name, &id);
        let label_namespace = format!("awswaf:{account_id}:webacl:{name}:");

        let acl = WebAcl {
            name: name.to_string(),
            id,
            arn,
            scope: scope.to_string(),
            description: description.to_string(),
            lock_token,
            default_action_json,
            visibility_config_json,
            rules_json,
            association_config_json,
            custom_response_bodies_json,
            captcha_config_json,
            challenge_config_json,
            token_domains,
            label_namespace,
            tags,
        };

        self.web_acls.insert(key.clone(), acl);
        Ok(self.web_acls.get(&key).unwrap())
    }

    pub fn get_web_acl(&self, name: &str, scope: &str, id: &str) -> Result<&WebAcl, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.web_acls.get(&key) {
            Some(acl) if acl.id == id => Ok(acl),
            _ => Err(nonexistent_error()),
        }
    }

    pub fn delete_web_acl(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
    ) -> Result<(), WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.web_acls.get(&key) {
            Some(acl) if acl.id == id => {
                if acl.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                self.web_acls.remove(&key);
                Ok(())
            }
            _ => Err(nonexistent_error()),
        }
    }

    pub fn list_web_acls(&self, scope: &str) -> Vec<&WebAcl> {
        self.web_acls
            .values()
            .filter(|acl| acl.scope == scope)
            .collect()
    }

    pub fn update_web_acl(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
        description: Option<&str>,
        default_action_json: Option<serde_json::Value>,
        visibility_config_json: Option<serde_json::Value>,
        rules_json: Option<serde_json::Value>,
    ) -> Result<String, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.web_acls.get_mut(&key) {
            Some(acl) if acl.id == id => {
                if acl.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                if let Some(d) = description {
                    acl.description = d.to_string();
                }
                if let Some(a) = default_action_json {
                    acl.default_action_json = a;
                }
                if let Some(v) = visibility_config_json {
                    acl.visibility_config_json = v;
                }
                if let Some(r) = rules_json {
                    acl.rules_json = r;
                }
                acl.lock_token = Uuid::new_v4().to_string();
                Ok(acl.lock_token.clone())
            }
            _ => Err(nonexistent_error()),
        }
    }

    // ── IPSet ──

    pub fn create_ip_set(
        &mut self,
        name: &str,
        scope: &str,
        description: &str,
        ip_address_version: &str,
        addresses: Vec<String>,
        region: &str,
        account_id: &str,
        tags: Vec<(String, String)>,
    ) -> Result<&IpSet, WafV2Error> {
        let key = format!("{scope}:{name}");
        if self.ip_sets.contains_key(&key) {
            return Err(duplicate_error());
        }

        let id = Uuid::new_v4().to_string();
        let lock_token = Uuid::new_v4().to_string();
        let arn = make_arn(region, account_id, scope, "ipset", name, &id);

        let ip_set = IpSet {
            name: name.to_string(),
            id,
            arn,
            scope: scope.to_string(),
            description: description.to_string(),
            lock_token,
            ip_address_version: ip_address_version.to_string(),
            addresses,
            tags,
        };

        self.ip_sets.insert(key.clone(), ip_set);
        Ok(self.ip_sets.get(&key).unwrap())
    }

    pub fn get_ip_set(&self, name: &str, scope: &str, id: &str) -> Result<&IpSet, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.ip_sets.get(&key) {
            Some(s) if s.id == id => Ok(s),
            _ => Err(nonexistent_error()),
        }
    }

    pub fn delete_ip_set(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
    ) -> Result<(), WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.ip_sets.get(&key) {
            Some(s) if s.id == id => {
                if s.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                self.ip_sets.remove(&key);
                Ok(())
            }
            _ => Err(nonexistent_error()),
        }
    }

    pub fn update_ip_set(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
        description: Option<&str>,
        addresses: Vec<String>,
    ) -> Result<String, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.ip_sets.get_mut(&key) {
            Some(s) if s.id == id => {
                if s.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                if let Some(d) = description {
                    s.description = d.to_string();
                }
                s.addresses = addresses;
                s.lock_token = Uuid::new_v4().to_string();
                Ok(s.lock_token.clone())
            }
            _ => Err(nonexistent_error()),
        }
    }

    pub fn list_ip_sets(&self, scope: &str) -> Vec<&IpSet> {
        self.ip_sets.values().filter(|s| s.scope == scope).collect()
    }

    // ── RegexPatternSet ──

    pub fn create_regex_pattern_set(
        &mut self,
        name: &str,
        scope: &str,
        description: &str,
        regular_expressions: Vec<String>,
        region: &str,
        account_id: &str,
        tags: Vec<(String, String)>,
    ) -> Result<&RegexPatternSet, WafV2Error> {
        let key = format!("{scope}:{name}");
        if self.regex_pattern_sets.contains_key(&key) {
            return Err(duplicate_error());
        }

        let id = Uuid::new_v4().to_string();
        let lock_token = Uuid::new_v4().to_string();
        let arn = make_arn(region, account_id, scope, "regexpatternset", name, &id);

        let rps = RegexPatternSet {
            name: name.to_string(),
            id,
            arn,
            scope: scope.to_string(),
            description: description.to_string(),
            lock_token,
            regular_expressions,
            tags,
        };

        self.regex_pattern_sets.insert(key.clone(), rps);
        Ok(self.regex_pattern_sets.get(&key).unwrap())
    }

    pub fn get_regex_pattern_set(
        &self,
        name: &str,
        scope: &str,
        id: &str,
    ) -> Result<&RegexPatternSet, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.regex_pattern_sets.get(&key) {
            Some(s) if s.id == id => Ok(s),
            _ => Err(nonexistent_error()),
        }
    }

    pub fn delete_regex_pattern_set(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
    ) -> Result<(), WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.regex_pattern_sets.get(&key) {
            Some(s) if s.id == id => {
                if s.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                self.regex_pattern_sets.remove(&key);
                Ok(())
            }
            _ => Err(nonexistent_error()),
        }
    }

    pub fn update_regex_pattern_set(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
        description: Option<&str>,
        regular_expressions: Vec<String>,
    ) -> Result<String, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.regex_pattern_sets.get_mut(&key) {
            Some(s) if s.id == id => {
                if s.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                if let Some(d) = description {
                    s.description = d.to_string();
                }
                s.regular_expressions = regular_expressions;
                s.lock_token = Uuid::new_v4().to_string();
                Ok(s.lock_token.clone())
            }
            _ => Err(nonexistent_error()),
        }
    }

    pub fn list_regex_pattern_sets(&self, scope: &str) -> Vec<&RegexPatternSet> {
        self.regex_pattern_sets
            .values()
            .filter(|s| s.scope == scope)
            .collect()
    }

    // ── RuleGroup ──

    pub fn create_rule_group(
        &mut self,
        name: &str,
        scope: &str,
        description: &str,
        capacity: i64,
        rules_json: serde_json::Value,
        visibility_config_json: serde_json::Value,
        custom_response_bodies_json: Option<serde_json::Value>,
        region: &str,
        account_id: &str,
        tags: Vec<(String, String)>,
    ) -> Result<&RuleGroupData, WafV2Error> {
        let key = format!("{scope}:{name}");
        if self.rule_groups.contains_key(&key) {
            return Err(duplicate_error());
        }

        let id = Uuid::new_v4().to_string();
        let lock_token = Uuid::new_v4().to_string();
        let arn = make_arn(region, account_id, scope, "rulegroup", name, &id);
        let label_namespace = format!("awswaf:{account_id}:rulegroup:{name}:");

        let rg = RuleGroupData {
            name: name.to_string(),
            id,
            arn,
            scope: scope.to_string(),
            description: description.to_string(),
            lock_token,
            capacity,
            rules_json,
            visibility_config_json,
            custom_response_bodies_json,
            label_namespace,
            tags,
        };

        self.rule_groups.insert(key.clone(), rg);
        Ok(self.rule_groups.get(&key).unwrap())
    }

    pub fn get_rule_group(
        &self,
        name: &str,
        scope: &str,
        id: &str,
    ) -> Result<&RuleGroupData, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.rule_groups.get(&key) {
            Some(s) if s.id == id => Ok(s),
            _ => Err(nonexistent_error()),
        }
    }

    pub fn delete_rule_group(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
    ) -> Result<(), WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.rule_groups.get(&key) {
            Some(s) if s.id == id => {
                if s.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                self.rule_groups.remove(&key);
                Ok(())
            }
            _ => Err(nonexistent_error()),
        }
    }

    pub fn update_rule_group(
        &mut self,
        name: &str,
        scope: &str,
        id: &str,
        lock_token: &str,
        description: Option<&str>,
        rules_json: Option<serde_json::Value>,
        visibility_config_json: Option<serde_json::Value>,
        custom_response_bodies_json: Option<serde_json::Value>,
    ) -> Result<String, WafV2Error> {
        let key = format!("{scope}:{name}");
        match self.rule_groups.get_mut(&key) {
            Some(s) if s.id == id => {
                if s.lock_token != lock_token {
                    return Err(optimistic_lock_error());
                }
                if let Some(d) = description {
                    s.description = d.to_string();
                }
                if let Some(r) = rules_json {
                    s.rules_json = r;
                }
                if let Some(v) = visibility_config_json {
                    s.visibility_config_json = v;
                }
                if let Some(c) = custom_response_bodies_json {
                    s.custom_response_bodies_json = Some(c);
                }
                s.lock_token = Uuid::new_v4().to_string();
                Ok(s.lock_token.clone())
            }
            _ => Err(nonexistent_error()),
        }
    }

    pub fn list_rule_groups(&self, scope: &str) -> Vec<&RuleGroupData> {
        self.rule_groups
            .values()
            .filter(|s| s.scope == scope)
            .collect()
    }

    // ── LoggingConfiguration ──

    pub fn put_logging_configuration(
        &mut self,
        resource_arn: &str,
        log_destination_configs: Vec<String>,
        logging_filter_json: Option<serde_json::Value>,
        redacted_fields_json: Option<serde_json::Value>,
        log_scope: Option<String>,
        log_type: Option<String>,
    ) -> Result<&LoggingConfigData, WafV2Error> {
        // Verify the resource ARN corresponds to an existing web ACL
        let _found = self.web_acls.values().any(|acl| acl.arn == resource_arn);
        if !_found {
            return Err(nonexistent_error());
        }

        let lc = LoggingConfigData {
            resource_arn: resource_arn.to_string(),
            log_destination_configs,
            logging_filter_json,
            redacted_fields_json,
            log_scope,
            log_type,
        };

        self.logging_configs.insert(resource_arn.to_string(), lc);
        Ok(self.logging_configs.get(resource_arn).unwrap())
    }

    pub fn get_logging_configuration(
        &self,
        resource_arn: &str,
    ) -> Result<&LoggingConfigData, WafV2Error> {
        self.logging_configs
            .get(resource_arn)
            .ok_or_else(nonexistent_error)
    }

    pub fn delete_logging_configuration(&mut self, resource_arn: &str) -> Result<(), WafV2Error> {
        match self.logging_configs.remove(resource_arn) {
            Some(_) => Ok(()),
            None => Err(nonexistent_error()),
        }
    }

    pub fn list_logging_configurations(&self, scope: &str) -> Vec<&LoggingConfigData> {
        // Filter by scope: check if the resource_arn contains "global" or "regional"
        let scope_segment = match scope {
            "CLOUDFRONT" => "global",
            _ => "regional",
        };
        self.logging_configs
            .values()
            .filter(|lc| lc.resource_arn.contains(scope_segment))
            .collect()
    }

    // ── WebACL Association ──

    pub fn associate_web_acl(
        &mut self,
        web_acl_arn: &str,
        resource_arn: &str,
    ) -> Result<(), WafV2Error> {
        // Verify web ACL exists
        let _found = self.web_acls.values().any(|acl| acl.arn == web_acl_arn);
        if !_found {
            return Err(nonexistent_error());
        }
        self.web_acl_associations
            .insert(resource_arn.to_string(), web_acl_arn.to_string());
        Ok(())
    }

    pub fn disassociate_web_acl(&mut self, resource_arn: &str) -> Result<(), WafV2Error> {
        self.web_acl_associations.remove(resource_arn);
        Ok(())
    }

    pub fn get_web_acl_for_resource(&self, resource_arn: &str) -> Option<&WebAcl> {
        let web_acl_arn = self.web_acl_associations.get(resource_arn)?;
        self.web_acls.values().find(|acl| &acl.arn == web_acl_arn)
    }

    // ── Tags ──

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), WafV2Error> {
        // Find the resource by ARN
        if let Some(acl) = self.web_acls.values_mut().find(|a| a.arn == arn) {
            for (k, v) in tags {
                if let Some(existing) = acl.tags.iter_mut().find(|(ek, _)| ek == &k) {
                    existing.1 = v;
                } else {
                    acl.tags.push((k, v));
                }
            }
            return Ok(());
        }
        if let Some(s) = self.ip_sets.values_mut().find(|a| a.arn == arn) {
            for (k, v) in tags {
                if let Some(existing) = s.tags.iter_mut().find(|(ek, _)| ek == &k) {
                    existing.1 = v;
                } else {
                    s.tags.push((k, v));
                }
            }
            return Ok(());
        }
        if let Some(s) = self.regex_pattern_sets.values_mut().find(|a| a.arn == arn) {
            for (k, v) in tags {
                if let Some(existing) = s.tags.iter_mut().find(|(ek, _)| ek == &k) {
                    existing.1 = v;
                } else {
                    s.tags.push((k, v));
                }
            }
            return Ok(());
        }
        if let Some(s) = self.rule_groups.values_mut().find(|a| a.arn == arn) {
            for (k, v) in tags {
                if let Some(existing) = s.tags.iter_mut().find(|(ek, _)| ek == &k) {
                    existing.1 = v;
                } else {
                    s.tags.push((k, v));
                }
            }
            return Ok(());
        }
        Err(nonexistent_error())
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), WafV2Error> {
        if let Some(acl) = self.web_acls.values_mut().find(|a| a.arn == arn) {
            acl.tags.retain(|(k, _)| !tag_keys.contains(k));
            return Ok(());
        }
        if let Some(s) = self.ip_sets.values_mut().find(|a| a.arn == arn) {
            s.tags.retain(|(k, _)| !tag_keys.contains(k));
            return Ok(());
        }
        if let Some(s) = self.regex_pattern_sets.values_mut().find(|a| a.arn == arn) {
            s.tags.retain(|(k, _)| !tag_keys.contains(k));
            return Ok(());
        }
        if let Some(s) = self.rule_groups.values_mut().find(|a| a.arn == arn) {
            s.tags.retain(|(k, _)| !tag_keys.contains(k));
            return Ok(());
        }
        Err(nonexistent_error())
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> Result<Vec<(String, String)>, WafV2Error> {
        if let Some(acl) = self.web_acls.values().find(|a| a.arn == arn) {
            return Ok(extract_tags(&acl.tags));
        }
        if let Some(s) = self.ip_sets.values().find(|a| a.arn == arn) {
            return Ok(extract_tags(&s.tags));
        }
        if let Some(s) = self.regex_pattern_sets.values().find(|a| a.arn == arn) {
            return Ok(extract_tags(&s.tags));
        }
        if let Some(s) = self.rule_groups.values().find(|a| a.arn == arn) {
            return Ok(extract_tags(&s.tags));
        }
        Err(nonexistent_error())
    }

    // ── Permission Policies ──

    pub fn put_permission_policy(
        &mut self,
        resource_arn: &str,
        policy: &str,
    ) -> Result<(), WafV2Error> {
        // Verify resource exists (web ACL or rule group)
        let found = self.web_acls.values().any(|a| a.arn == resource_arn)
            || self.rule_groups.values().any(|a| a.arn == resource_arn);
        if !found {
            return Err(nonexistent_error());
        }
        self.permission_policies
            .insert(resource_arn.to_string(), policy.to_string());
        Ok(())
    }

    pub fn get_permission_policy(&self, resource_arn: &str) -> Result<&str, WafV2Error> {
        self.permission_policies
            .get(resource_arn)
            .map(|s| s.as_str())
            .ok_or_else(nonexistent_error)
    }

    pub fn delete_permission_policy(&mut self, resource_arn: &str) -> Result<(), WafV2Error> {
        match self.permission_policies.remove(resource_arn) {
            Some(_) => Ok(()),
            None => Err(nonexistent_error()),
        }
    }

    // ── API Keys ──

    pub fn create_api_key(
        &mut self,
        scope: &str,
        token_domains: Vec<String>,
    ) -> &crate::types::ApiKeyData {
        let api_key = Uuid::new_v4().to_string();
        let key = format!("{scope}:{api_key}");
        let data = crate::types::ApiKeyData {
            api_key: api_key.clone(),
            scope: scope.to_string(),
            token_domains,
            creation_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
        };
        self.api_keys.insert(key.clone(), data);
        self.api_keys.get(&key).unwrap()
    }

    pub fn list_api_keys(&self, scope: &str) -> Vec<&crate::types::ApiKeyData> {
        self.api_keys
            .values()
            .filter(|k| k.scope == scope)
            .collect()
    }

    pub fn delete_api_key(&mut self, scope: &str, api_key: &str) -> Result<(), WafV2Error> {
        let key = format!("{scope}:{api_key}");
        match self.api_keys.remove(&key) {
            Some(_) => Ok(()),
            None => Err(nonexistent_error()),
        }
    }

    // ── WebACL Associations (list resources) ──

    pub fn list_resources_for_web_acl(&self, web_acl_arn: &str) -> Vec<String> {
        self.web_acl_associations
            .iter()
            .filter_map(|(resource_arn, acl_arn)| {
                if acl_arn == web_acl_arn {
                    Some(resource_arn.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
