use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::*;
use crate::wire;

/// Key for standards control associations.
pub fn sca_key(security_control_id: &str, standards_arn: &str) -> String {
    format!("{security_control_id}::{standards_arn}")
}

/// Configuration policy association keyed by target_id.
pub type AssocKey = String;

#[derive(Debug, Default)]
pub struct SecurityHubState {
    pub findings: Vec<Finding>,
    pub hub: HubInfo,
    pub members: HashMap<String, Member>,
    pub action_targets: HashMap<String, ActionTargetInfo>,
    pub finding_aggregators: HashMap<String, FindingAggregatorInfo>,
    pub enabled_standards: Vec<StandardsSubscriptionInfo>,
    pub org_config: OrganizationConfig,
    pub org_admin_account: Option<OrgAdminAccount>,
    /// Automation rules keyed by ARN (both v1 and v2).
    pub automation_rules: HashMap<String, AutomationRuleInfo>,
    /// Configuration policies keyed by ID.
    pub config_policies: HashMap<String, ConfigPolicyInfo>,
    /// Connectors (v2) keyed by connector ID.
    pub connectors: HashMap<String, ConnectorV2Info>,
    /// Insights keyed by ARN.
    pub insights: HashMap<String, InsightInfo>,
    /// Hub v2 state.
    pub hub_v2: HubV2Info,
    /// AggregatorV2 keyed by ARN.
    pub aggregators_v2: HashMap<String, AggregatorV2Info>,
    /// Product subscriptions keyed by product_subscription_arn.
    pub product_subscriptions: HashMap<String, ProductSubscription>,
    /// Configuration policy associations keyed by target_id.
    pub policy_associations: HashMap<AssocKey, ConfigPolicyAssociation>,
    /// auto_enable_controls (UpdateSecurityHubConfiguration).
    pub auto_enable_controls: bool,
    /// control_finding_generator (UpdateSecurityHubConfiguration).
    pub control_finding_generator: String,
    /// Security controls keyed by security_control_id.
    pub security_controls: HashMap<String, SecurityControlInfo>,
    /// Standards control associations keyed by "security_control_id::standards_arn".
    pub standards_control_associations: HashMap<String, StandardsControlAssociationInfo>,
    /// Standards controls (legacy v1) keyed by standards_control_arn.
    pub standards_controls: HashMap<String, StandardsControlInfo>,
    /// Standards catalog keyed by standards_arn.
    pub standards: HashMap<String, StandardInfo>,
    /// Product catalog keyed by product_arn.
    pub products: HashMap<String, ProductInfo>,
    /// Tickets v2 keyed by ticket_id.
    pub tickets_v2: HashMap<String, TicketV2Info>,
    /// Findings v2 (OCSF) keyed by metadata_uid.
    pub findings_v2: HashMap<String, FindingV2>,
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityHubError {
    #[error("Account is not subscribed to AWS Security Hub")]
    NotSubscribed,
    #[error("The request was rejected because no hub was found with ARN {arn}.")]
    HubNotFound { arn: String },
    #[error("MaxResults must be a number between 1 and 100")]
    MaxResultsOutOfRange,
    #[error("ActionTarget with ARN {arn} already exists")]
    ActionTargetAlreadyExists { arn: String },
    #[error("ActionTarget with ARN {arn} not found")]
    ActionTargetNotFound { arn: String },
    #[error("Resource with ARN {arn} not found")]
    ResourceNotFound { arn: String },
}

pub struct DescribeHubResult {
    pub hub_arn: String,
    pub subscribed_at: Option<String>,
    pub auto_enable_controls: bool,
    pub control_finding_generator: String,
}

pub struct BatchImportFindingsResult {
    pub failed_count: i32,
    pub success_count: i32,
    pub failed_findings: Vec<wire::ImportFindingsError>,
}

pub struct GetFindingsResult {
    pub findings: Vec<serde_json::Value>,
    pub next_token: Option<String>,
}

pub struct CreateMembersResult {
    pub unprocessed_accounts: Vec<wire::Result_>,
}

pub struct GetMembersResult {
    pub members: Vec<wire::Member>,
    pub unprocessed_accounts: Vec<wire::Result_>,
}

pub struct ListMembersResult {
    pub members: Vec<wire::Member>,
    pub next_token: Option<String>,
}

pub struct CreateActionTargetResult {
    pub action_target_arn: String,
}

pub struct DescribeActionTargetsResult {
    pub action_targets: Vec<wire::ActionTarget>,
    pub next_token: Option<String>,
}

pub struct DeleteActionTargetResult {
    pub action_target_arn: String,
}

pub struct CreateFindingAggregatorResult {
    pub finding_aggregator_arn: String,
    pub finding_aggregation_region: String,
    pub region_linking_mode: String,
    pub regions: Vec<String>,
}

pub struct GetEnabledStandardsResult {
    pub standards_subscriptions: Vec<wire::StandardsSubscription>,
    pub next_token: Option<String>,
}

impl SecurityHubState {
    pub fn enable_security_hub(
        &mut self,
        _enable_default_standards: bool,
        tags: HashMap<String, String>,
    ) {
        if self.hub.enabled {
            return;
        }
        self.hub.enabled = true;
        self.hub.subscribed_at =
            Some(Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true));
        self.hub.tags = tags;
    }

    pub fn disable_security_hub(&mut self) -> Result<(), SecurityHubError> {
        if !self.hub.enabled {
            return Err(SecurityHubError::NotSubscribed);
        }
        self.hub.enabled = false;
        self.hub.subscribed_at = None;
        self.findings.clear();
        Ok(())
    }

    pub fn describe_hub(
        &self,
        hub_arn: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<DescribeHubResult, SecurityHubError> {
        if !self.hub.enabled {
            return Err(SecurityHubError::NotSubscribed);
        }

        let expected_arn = format!("arn:aws:securityhub:{region}:{account_id}:hub/default");
        if let Some(arn) = hub_arn
            && arn != expected_arn
        {
            return Err(SecurityHubError::HubNotFound {
                arn: arn.to_string(),
            });
        }

        Ok(DescribeHubResult {
            hub_arn: expected_arn,
            subscribed_at: self.hub.subscribed_at.clone(),
            auto_enable_controls: true,
            control_finding_generator: "SECURITY_CONTROL".to_string(),
        })
    }

    pub fn batch_import_findings(
        &mut self,
        findings: &[serde_json::Value],
    ) -> BatchImportFindingsResult {
        let mut failed_count = 0i32;
        let mut success_count = 0i32;
        let mut failed_findings = Vec::new();

        for finding_data in findings {
            let finding_id = match finding_data.get("Id").and_then(|v| v.as_str()) {
                Some(id) => id.to_string(),
                None => {
                    failed_count += 1;
                    failed_findings.push(wire::ImportFindingsError {
                        id: Some(String::new()),
                        error_code: Some("InvalidInput".to_string()),
                        error_message: Some("Missing Id field".to_string()),
                    });
                    continue;
                }
            };

            // Check Resources array
            match finding_data.get("Resources").and_then(|v| v.as_array()) {
                Some(arr) if !arr.is_empty() => {}
                _ => {
                    failed_count += 1;
                    failed_findings.push(wire::ImportFindingsError {
                        id: Some(finding_id),
                        error_code: Some("InvalidInput".to_string()),
                        error_message: Some(
                            "Finding must contain at least one resource in the Resources array"
                                .to_string(),
                        ),
                    });
                    continue;
                }
            }

            // Check if finding already exists and update
            if let Some(existing) = self.findings.iter_mut().find(|f| f.id == finding_id) {
                existing.data = finding_data.clone();
            } else {
                self.findings.push(Finding {
                    id: finding_id,
                    data: finding_data.clone(),
                });
            }
            success_count += 1;
        }

        BatchImportFindingsResult {
            failed_count,
            success_count,
            failed_findings,
        }
    }

    pub fn get_findings(
        &self,
        max_results: Option<i64>,
        next_token: Option<&str>,
    ) -> Result<GetFindingsResult, SecurityHubError> {
        if let Some(max) = max_results
            && (!(1..=100).contains(&max))
        {
            return Err(SecurityHubError::MaxResultsOutOfRange);
        }

        let limit = max_results.unwrap_or(100) as usize;
        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let all_findings: Vec<serde_json::Value> =
            self.findings.iter().map(|f| f.data.clone()).collect();
        let total = all_findings.len();
        let end = (start + limit).min(total);
        let page = all_findings[start..end].to_vec();

        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };

        Ok(GetFindingsResult {
            findings: page,
            next_token: new_token,
        })
    }

    pub fn create_members(&mut self, account_details: &[serde_json::Value]) -> CreateMembersResult {
        let mut unprocessed = Vec::new();

        for account in account_details {
            let account_id = match account.get("AccountId").and_then(|v| v.as_str()) {
                Some(id) => id.to_string(),
                None => {
                    unprocessed.push(wire::Result_ {
                        account_id: Some(String::new()),
                        processing_result: Some("Invalid input: AccountId is required".to_string()),
                    });
                    continue;
                }
            };
            let email = account
                .get("Email")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if self.members.contains_key(&account_id) {
                unprocessed.push(wire::Result_ {
                    account_id: Some(account_id.clone()),
                    processing_result: Some(format!("Account {} is already a member", account_id)),
                });
                continue;
            }

            let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
            self.members.insert(
                account_id.clone(),
                Member {
                    account_id,
                    email,
                    member_status: "ENABLED".to_string(),
                    invited_at: now.clone(),
                    updated_at: now,
                },
            );
        }

        CreateMembersResult {
            unprocessed_accounts: unprocessed,
        }
    }

    pub fn get_members(&self, account_ids: &[String]) -> GetMembersResult {
        let mut members = Vec::new();
        let mut unprocessed = Vec::new();

        for account_id in account_ids {
            if let Some(member) = self.members.get(account_id) {
                members.push(wire::Member {
                    account_id: Some(member.account_id.clone()),
                    administrator_id: None,
                    email: Some(member.email.clone()),
                    invited_at: Some(member.invited_at.clone()),
                    master_id: None,
                    member_status: Some(member.member_status.clone()),
                    updated_at: Some(member.updated_at.clone()),
                });
            } else {
                unprocessed.push(wire::Result_ {
                    account_id: Some(account_id.clone()),
                    processing_result: Some(format!("Account {} is not a member", account_id)),
                });
            }
        }

        GetMembersResult {
            members,
            unprocessed_accounts: unprocessed,
        }
    }

    pub fn list_members(
        &self,
        _only_associated: Option<bool>,
        max_results: Option<i64>,
        next_token: Option<&str>,
    ) -> ListMembersResult {
        let limit = max_results.unwrap_or(50) as usize;
        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let all_members: Vec<wire::Member> = self
            .members
            .values()
            .map(|m| wire::Member {
                account_id: Some(m.account_id.clone()),
                administrator_id: None,
                email: Some(m.email.clone()),
                invited_at: Some(m.invited_at.clone()),
                master_id: None,
                member_status: Some(m.member_status.clone()),
                updated_at: Some(m.updated_at.clone()),
            })
            .collect();

        let total = all_members.len();
        let end = (start + limit).min(total);
        let page = all_members[start..end].to_vec();

        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };

        ListMembersResult {
            members: page,
            next_token: new_token,
        }
    }

    pub fn create_action_target(
        &mut self,
        name: &str,
        description: &str,
        id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<CreateActionTargetResult, SecurityHubError> {
        let arn = format!("arn:aws:securityhub:{region}:{account_id}:action/custom/{id}");
        if self.action_targets.contains_key(&arn) {
            return Err(SecurityHubError::ActionTargetAlreadyExists { arn });
        }
        self.action_targets.insert(
            arn.clone(),
            ActionTargetInfo {
                arn: arn.clone(),
                name: name.to_string(),
                description: description.to_string(),
            },
        );
        Ok(CreateActionTargetResult {
            action_target_arn: arn,
        })
    }

    pub fn describe_action_targets(
        &self,
        action_target_arns: Option<&[String]>,
        max_results: Option<i64>,
        next_token: Option<&str>,
    ) -> Result<DescribeActionTargetsResult, SecurityHubError> {
        let targets: Vec<wire::ActionTarget> = if let Some(arns) = action_target_arns {
            let mut result = Vec::new();
            for arn in arns {
                match self.action_targets.get(arn) {
                    Some(t) => result.push(wire::ActionTarget {
                        action_target_arn: Some(t.arn.clone()),
                        name: Some(t.name.clone()),
                        description: Some(t.description.clone()),
                    }),
                    None => {
                        return Err(SecurityHubError::ActionTargetNotFound { arn: arn.clone() });
                    }
                }
            }
            result
        } else {
            self.action_targets
                .values()
                .map(|t| wire::ActionTarget {
                    action_target_arn: Some(t.arn.clone()),
                    name: Some(t.name.clone()),
                    description: Some(t.description.clone()),
                })
                .collect()
        };

        let limit = max_results.unwrap_or(100) as usize;
        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);
        let total = targets.len();
        let end = (start + limit).min(total);
        let page = targets[start..end].to_vec();

        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };

        Ok(DescribeActionTargetsResult {
            action_targets: page,
            next_token: new_token,
        })
    }

    pub fn delete_action_target(
        &mut self,
        action_target_arn: &str,
    ) -> Result<DeleteActionTargetResult, SecurityHubError> {
        match self.action_targets.remove(action_target_arn) {
            Some(_) => Ok(DeleteActionTargetResult {
                action_target_arn: action_target_arn.to_string(),
            }),
            None => Err(SecurityHubError::ActionTargetNotFound {
                arn: action_target_arn.to_string(),
            }),
        }
    }

    pub fn create_finding_aggregator(
        &mut self,
        region_linking_mode: &str,
        regions: Vec<String>,
        account_id: &str,
        region: &str,
    ) -> Result<CreateFindingAggregatorResult, SecurityHubError> {
        let arn = format!(
            "arn:aws:securityhub:{region}:{account_id}:finding-aggregator/{}",
            uuid::Uuid::new_v4()
        );
        let info = FindingAggregatorInfo {
            arn: arn.clone(),
            region_linking_mode: region_linking_mode.to_string(),
            regions: regions.clone(),
            finding_aggregation_region: region.to_string(),
        };
        self.finding_aggregators.insert(arn.clone(), info);
        Ok(CreateFindingAggregatorResult {
            finding_aggregator_arn: arn,
            finding_aggregation_region: region.to_string(),
            region_linking_mode: region_linking_mode.to_string(),
            regions,
        })
    }

    pub fn get_enabled_standards(
        &self,
        max_results: Option<i64>,
        next_token: Option<&str>,
    ) -> GetEnabledStandardsResult {
        let subs: Vec<wire::StandardsSubscription> = self
            .enabled_standards
            .iter()
            .map(|s| wire::StandardsSubscription {
                standards_subscription_arn: Some(s.standards_subscription_arn.clone()),
                standards_arn: Some(s.standards_arn.clone()),
                standards_status: Some(s.standards_status.clone()),
                standards_input: Some(s.standards_input.clone()),
                ..Default::default()
            })
            .collect();

        let limit = max_results.unwrap_or(100) as usize;
        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);
        let total = subs.len();
        let end = (start + limit).min(total);
        let page = subs[start..end].to_vec();

        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };

        GetEnabledStandardsResult {
            standards_subscriptions: page,
            next_token: new_token,
        }
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<HashMap<String, String>, SecurityHubError> {
        let hub_arn = format!("arn:aws:securityhub:{region}:{account_id}:hub/default");
        if resource_arn == hub_arn {
            return Ok(self.hub.tags.clone());
        }
        // Check action targets
        if self.action_targets.contains_key(resource_arn) {
            // Action targets don't have tags in this mock
            return Ok(HashMap::new());
        }
        Err(SecurityHubError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<(), SecurityHubError> {
        let hub_arn = format!("arn:aws:securityhub:{region}:{account_id}:hub/default");
        if resource_arn == hub_arn {
            self.hub.tags.extend(tags);
            return Ok(());
        }
        Err(SecurityHubError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
        account_id: &str,
        region: &str,
    ) -> Result<(), SecurityHubError> {
        let hub_arn = format!("arn:aws:securityhub:{region}:{account_id}:hub/default");
        if resource_arn == hub_arn {
            for key in tag_keys {
                self.hub.tags.remove(key);
            }
            return Ok(());
        }
        Err(SecurityHubError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    // --- Organization configuration ---

    pub fn describe_organization_configuration(&self) -> &OrganizationConfig {
        &self.org_config
    }

    pub fn update_organization_configuration(
        &mut self,
        auto_enable: Option<bool>,
        auto_enable_standards: Option<&str>,
    ) {
        if let Some(v) = auto_enable {
            self.org_config.auto_enable = v;
        }
        if let Some(s) = auto_enable_standards {
            self.org_config.auto_enable_standards = s.to_string();
        }
    }

    pub fn enable_organization_admin_account(&mut self, admin_account_id: &str) {
        self.org_admin_account = Some(OrgAdminAccount {
            account_id: admin_account_id.to_string(),
            status: "ENABLED".to_string(),
        });
    }

    pub fn get_administrator_account(&self) -> Option<&OrgAdminAccount> {
        self.org_admin_account.as_ref()
    }

    // --- Automation rules ---

    pub fn create_automation_rule(
        &mut self,
        rule_arn: String,
        rule_id: Option<String>,
        body: &serde_json::Value,
    ) {
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let rule_name = body
            .get("RuleName")
            .and_then(|v| v.as_str())
            .unwrap_or("unnamed")
            .to_string();
        let rule_order = body.get("RuleOrder").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
        let rule_status = body
            .get("RuleStatus")
            .and_then(|v| v.as_str())
            .unwrap_or("ENABLED")
            .to_string();
        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let is_terminal = body
            .get("IsTerminal")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        self.automation_rules.insert(
            rule_arn.clone(),
            AutomationRuleInfo {
                rule_arn,
                rule_id,
                rule_name,
                rule_order,
                rule_status,
                description,
                is_terminal,
                created_at: now.clone(),
                updated_at: now,
                raw: body.clone(),
            },
        );
    }

    pub fn batch_delete_automation_rules(
        &mut self,
        arns: &[String],
    ) -> (Vec<String>, Vec<wire::UnprocessedAutomationRule>) {
        let mut processed = Vec::new();
        let mut unprocessed = Vec::new();
        for arn in arns {
            if self.automation_rules.remove(arn).is_some() {
                processed.push(arn.clone());
            } else {
                unprocessed.push(wire::UnprocessedAutomationRule {
                    rule_arn: Some(arn.clone()),
                    error_code: Some(404),
                    error_message: Some(format!("Rule {arn} not found")),
                });
            }
        }
        (processed, unprocessed)
    }

    pub fn batch_get_automation_rules(&self, arns: &[String]) -> Vec<wire::AutomationRulesConfig> {
        arns.iter()
            .filter_map(|arn| self.automation_rules.get(arn))
            .map(|rule| wire::AutomationRulesConfig {
                rule_arn: Some(rule.rule_arn.clone()),
                rule_name: Some(rule.rule_name.clone()),
                rule_order: Some(rule.rule_order),
                rule_status: Some(rule.rule_status.clone()),
                description: rule.description.clone(),
                is_terminal: Some(rule.is_terminal),
                created_at: Some(rule.created_at.clone()),
                updated_at: Some(rule.updated_at.clone()),
                created_by: None,
                actions: None,
                criteria: None,
            })
            .collect()
    }

    pub fn batch_update_automation_rules(
        &mut self,
        updates: &[serde_json::Value],
    ) -> (Vec<String>, Vec<wire::UnprocessedAutomationRule>) {
        let mut processed = Vec::new();
        let mut unprocessed = Vec::new();
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        for update in updates {
            let arn = match update.get("RuleArn").and_then(|v| v.as_str()) {
                Some(a) => a.to_string(),
                None => continue,
            };
            if let Some(rule) = self.automation_rules.get_mut(&arn) {
                if let Some(name) = update.get("RuleName").and_then(|v| v.as_str()) {
                    rule.rule_name = name.to_string();
                }
                if let Some(order) = update.get("RuleOrder").and_then(|v| v.as_i64()) {
                    rule.rule_order = order as i32;
                }
                if let Some(status) = update.get("RuleStatus").and_then(|v| v.as_str()) {
                    rule.rule_status = status.to_string();
                }
                if let Some(desc) = update.get("Description").and_then(|v| v.as_str()) {
                    rule.description = Some(desc.to_string());
                }
                if let Some(term) = update.get("IsTerminal").and_then(|v| v.as_bool()) {
                    rule.is_terminal = term;
                }
                rule.updated_at = now.clone();
                processed.push(arn);
            } else {
                unprocessed.push(wire::UnprocessedAutomationRule {
                    rule_arn: Some(arn.clone()),
                    error_code: Some(404),
                    error_message: Some(format!("Rule {arn} not found")),
                });
            }
        }
        (processed, unprocessed)
    }

    pub fn list_automation_rules(&self) -> Vec<wire::AutomationRulesMetadata> {
        self.automation_rules
            .values()
            .map(|rule| wire::AutomationRulesMetadata {
                rule_arn: Some(rule.rule_arn.clone()),
                rule_name: Some(rule.rule_name.clone()),
                rule_order: Some(rule.rule_order),
                rule_status: Some(rule.rule_status.clone()),
                description: rule.description.clone(),
                is_terminal: Some(rule.is_terminal),
                created_at: Some(rule.created_at.clone()),
                updated_at: Some(rule.updated_at.clone()),
                created_by: None,
            })
            .collect()
    }

    pub fn create_automation_rule_v2(
        &mut self,
        body: &serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> &AutomationRuleInfo {
        let rule_id = Uuid::new_v4().to_string();
        let rule_arn =
            format!("arn:aws:securityhub:{region}:{account_id}:automation-rule-v2/{rule_id}");
        self.create_automation_rule(rule_arn.clone(), Some(rule_id), body);
        self.automation_rules.get(&rule_arn).unwrap()
    }

    pub fn delete_automation_rule_v2(&mut self, identifier: &str) -> bool {
        let arn = self
            .automation_rules
            .values()
            .find(|r| r.rule_id.as_deref() == Some(identifier) || r.rule_arn == identifier)
            .map(|r| r.rule_arn.clone());
        if let Some(arn) = arn {
            self.automation_rules.remove(&arn);
            return true;
        }
        false
    }

    pub fn get_automation_rule_v2(&self, identifier: &str) -> Option<&AutomationRuleInfo> {
        self.automation_rules
            .values()
            .find(|r| r.rule_id.as_deref() == Some(identifier) || r.rule_arn == identifier)
    }

    pub fn update_automation_rule_v2(
        &mut self,
        identifier: &str,
        body: &serde_json::Value,
    ) -> bool {
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let arn = self
            .automation_rules
            .values()
            .find(|r| r.rule_id.as_deref() == Some(identifier) || r.rule_arn == identifier)
            .map(|r| r.rule_arn.clone());
        if let Some(arn) = arn {
            if let Some(rule) = self.automation_rules.get_mut(&arn) {
                if let Some(name) = body.get("RuleName").and_then(|v| v.as_str()) {
                    rule.rule_name = name.to_string();
                }
                if let Some(order) = body.get("RuleOrder").and_then(|v| v.as_i64()) {
                    rule.rule_order = order as i32;
                }
                if let Some(status) = body.get("RuleStatus").and_then(|v| v.as_str()) {
                    rule.rule_status = status.to_string();
                }
                if let Some(desc) = body.get("Description").and_then(|v| v.as_str()) {
                    rule.description = Some(desc.to_string());
                }
                if let Some(term) = body.get("IsTerminal").and_then(|v| v.as_bool()) {
                    rule.is_terminal = term;
                }
                rule.updated_at = now;
                return true;
            }
        }
        false
    }

    pub fn list_automation_rules_v2(&self) -> Vec<&AutomationRuleInfo> {
        self.automation_rules
            .values()
            .filter(|r| r.rule_id.is_some())
            .collect()
    }

    // --- Configuration policies ---

    pub fn create_config_policy(
        &mut self,
        id: String,
        arn: String,
        name: String,
        description: Option<String>,
        configuration_policy: Option<serde_json::Value>,
    ) -> &ConfigPolicyInfo {
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        self.config_policies.insert(
            id.clone(),
            ConfigPolicyInfo {
                id: id.clone(),
                arn,
                name,
                description,
                created_at: now.clone(),
                updated_at: now,
                configuration_policy,
            },
        );
        self.config_policies.get(&id).unwrap()
    }

    pub fn get_config_policy(&self, id_or_arn: &str) -> Option<&ConfigPolicyInfo> {
        // Try by ID first
        if let Some(p) = self.config_policies.get(id_or_arn) {
            return Some(p);
        }
        // Try by ARN
        self.config_policies.values().find(|p| p.arn == id_or_arn)
    }

    pub fn update_config_policy(
        &mut self,
        id_or_arn: &str,
        name: Option<String>,
        description: Option<String>,
        configuration_policy: Option<serde_json::Value>,
    ) -> Option<&ConfigPolicyInfo> {
        // Find the ID
        let id = if self.config_policies.contains_key(id_or_arn) {
            id_or_arn.to_string()
        } else {
            self.config_policies
                .values()
                .find(|p| p.arn == id_or_arn)
                .map(|p| p.id.clone())?
        };
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        if let Some(policy) = self.config_policies.get_mut(&id) {
            if let Some(n) = name {
                policy.name = n;
            }
            if let Some(d) = description {
                policy.description = Some(d);
            }
            if configuration_policy.is_some() {
                policy.configuration_policy = configuration_policy;
            }
            policy.updated_at = now;
        }
        self.config_policies.get(&id)
    }

    pub fn delete_config_policy(&mut self, id_or_arn: &str) -> bool {
        if self.config_policies.remove(id_or_arn).is_some() {
            return true;
        }
        // Try by ARN
        let id = self
            .config_policies
            .values()
            .find(|p| p.arn == id_or_arn)
            .map(|p| p.id.clone());
        if let Some(id) = id {
            self.config_policies.remove(&id);
            return true;
        }
        false
    }

    pub fn list_config_policies(&self) -> Vec<&ConfigPolicyInfo> {
        self.config_policies.values().collect()
    }

    // --- Connectors ---

    pub fn create_connector_v2(
        &mut self,
        body: &serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> &ConnectorV2Info {
        let connector_id = Uuid::new_v4().to_string();
        let connector_arn =
            format!("arn:aws:securityhub:{region}:{account_id}:connector/{connector_id}");
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let name = body
            .get("Name")
            .and_then(|v| v.as_str())
            .unwrap_or("connector")
            .to_string();
        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let provider_name = body
            .get("ProviderDetail")
            .and_then(|v| v.get("ProviderName"))
            .and_then(|v| v.as_str())
            .or_else(|| body.get("ProviderName").and_then(|v| v.as_str()))
            .map(|s| s.to_string());
        let info = ConnectorV2Info {
            connector_id: connector_id.clone(),
            connector_arn,
            name,
            description,
            created_at: now.clone(),
            last_updated_at: now,
            connector_status: "ACTIVE".to_string(),
            provider_name,
            tags: HashMap::new(),
            raw: body.clone(),
        };
        self.connectors.insert(connector_id.clone(), info);
        self.connectors.get(&connector_id).unwrap()
    }

    pub fn get_connector_v2(&self, connector_id: &str) -> Option<&ConnectorV2Info> {
        self.connectors.get(connector_id)
    }

    pub fn delete_connector_v2(&mut self, connector_id: &str) -> bool {
        self.connectors.remove(connector_id).is_some()
    }

    pub fn update_connector_v2(&mut self, connector_id: &str, body: &serde_json::Value) -> bool {
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        if let Some(info) = self.connectors.get_mut(connector_id) {
            if let Some(name) = body.get("Name").and_then(|v| v.as_str()) {
                info.name = name.to_string();
            }
            if let Some(desc) = body.get("Description").and_then(|v| v.as_str()) {
                info.description = Some(desc.to_string());
            }
            info.last_updated_at = now;
            true
        } else {
            false
        }
    }

    pub fn list_connectors_v2(&self) -> Vec<&ConnectorV2Info> {
        self.connectors.values().collect()
    }

    // --- Insights ---

    pub fn create_insight(
        &mut self,
        body: &serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> String {
        let arn = format!(
            "arn:aws:securityhub:{region}:{account_id}:insight/{}",
            Uuid::new_v4()
        );
        let name = body
            .get("Name")
            .and_then(|v| v.as_str())
            .unwrap_or("insight")
            .to_string();
        let group_by_attribute = body
            .get("GroupByAttribute")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let filters = body
            .get("Filters")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        self.insights.insert(
            arn.clone(),
            InsightInfo {
                arn: arn.clone(),
                name,
                group_by_attribute,
                filters,
            },
        );
        arn
    }

    pub fn get_insight(&self, arn: &str) -> Option<&InsightInfo> {
        self.insights.get(arn)
    }

    pub fn delete_insight(&mut self, arn: &str) -> bool {
        self.insights.remove(arn).is_some()
    }

    pub fn update_insight(&mut self, arn: &str, body: &serde_json::Value) -> bool {
        if let Some(insight) = self.insights.get_mut(arn) {
            if let Some(name) = body.get("Name").and_then(|v| v.as_str()) {
                insight.name = name.to_string();
            }
            if let Some(gba) = body.get("GroupByAttribute").and_then(|v| v.as_str()) {
                insight.group_by_attribute = gba.to_string();
            }
            if let Some(filters) = body.get("Filters") {
                insight.filters = filters.clone();
            }
            true
        } else {
            false
        }
    }

    pub fn list_insights(&self) -> Vec<&InsightInfo> {
        self.insights.values().collect()
    }

    // --- Hub v2 ---

    pub fn enable_security_hub_v2(&mut self, account_id: &str, region: &str) -> String {
        let arn = format!("arn:aws:securityhub:{region}:{account_id}:hub-v2/default");
        self.hub_v2.enabled = true;
        self.hub_v2.hub_v2_arn = Some(arn.clone());
        self.hub_v2.subscribed_at =
            Some(Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true));
        arn
    }

    pub fn disable_security_hub_v2(&mut self) {
        self.hub_v2.enabled = false;
        self.hub_v2.hub_v2_arn = None;
        self.hub_v2.subscribed_at = None;
    }

    // --- AggregatorV2 ---

    pub fn create_aggregator_v2(
        &mut self,
        region_linking_mode: &str,
        linked_regions: Vec<String>,
        account_id: &str,
        region: &str,
    ) -> &AggregatorV2Info {
        let arn = format!(
            "arn:aws:securityhub:{region}:{account_id}:aggregator-v2/{}",
            Uuid::new_v4()
        );
        self.aggregators_v2.insert(
            arn.clone(),
            AggregatorV2Info {
                arn: arn.clone(),
                aggregation_region: region.to_string(),
                region_linking_mode: region_linking_mode.to_string(),
                linked_regions,
            },
        );
        self.aggregators_v2.get(&arn).unwrap()
    }

    pub fn get_aggregator_v2(&self, arn: &str) -> Option<&AggregatorV2Info> {
        self.aggregators_v2.get(arn)
    }

    pub fn delete_aggregator_v2(&mut self, arn: &str) -> bool {
        self.aggregators_v2.remove(arn).is_some()
    }

    pub fn update_aggregator_v2(
        &mut self,
        arn: &str,
        region_linking_mode: Option<&str>,
        linked_regions: Option<Vec<String>>,
    ) -> Option<&AggregatorV2Info> {
        if let Some(info) = self.aggregators_v2.get_mut(arn) {
            if let Some(mode) = region_linking_mode {
                info.region_linking_mode = mode.to_string();
            }
            if let Some(regions) = linked_regions {
                info.linked_regions = regions;
            }
        }
        self.aggregators_v2.get(arn)
    }

    pub fn list_aggregators_v2(&self) -> Vec<&AggregatorV2Info> {
        self.aggregators_v2.values().collect()
    }

    // --- Product subscriptions ---

    pub fn enable_import_findings_for_product(
        &mut self,
        product_arn: &str,
        account_id: &str,
        region: &str,
    ) -> String {
        // product_subscription_arn is constructed from the product ARN
        let subscription_arn = format!(
            "arn:aws:securityhub:{region}:{account_id}:product-subscription/{}",
            Uuid::new_v4()
        );
        self.product_subscriptions.insert(
            subscription_arn.clone(),
            ProductSubscription {
                product_subscription_arn: subscription_arn.clone(),
                product_arn: product_arn.to_string(),
            },
        );
        subscription_arn
    }

    pub fn disable_import_findings_for_product(&mut self, subscription_arn: &str) -> bool {
        self.product_subscriptions
            .remove(subscription_arn)
            .is_some()
    }

    pub fn list_enabled_products_for_import(&self) -> Vec<String> {
        self.product_subscriptions
            .values()
            .map(|s| s.product_subscription_arn.clone())
            .collect()
    }

    // --- Configuration policy associations ---

    pub fn start_configuration_policy_association(
        &mut self,
        configuration_policy_id: &str,
        target_id: &str,
        target_type: &str,
    ) -> ConfigPolicyAssociation {
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let assoc = ConfigPolicyAssociation {
            configuration_policy_id: configuration_policy_id.to_string(),
            target_id: target_id.to_string(),
            target_type: target_type.to_string(),
            association_type: "APPLIED".to_string(),
            association_status: "SUCCESS".to_string(),
            association_status_message: None,
            updated_at: now,
        };
        self.policy_associations
            .insert(target_id.to_string(), assoc.clone());
        assoc
    }

    pub fn start_configuration_policy_disassociation(&mut self, target_id: &str) -> bool {
        self.policy_associations.remove(target_id).is_some()
    }

    pub fn get_configuration_policy_association(
        &self,
        target_id: &str,
    ) -> Option<&ConfigPolicyAssociation> {
        self.policy_associations.get(target_id)
    }

    pub fn list_configuration_policy_associations(&self) -> Vec<&ConfigPolicyAssociation> {
        self.policy_associations.values().collect()
    }

    pub fn batch_get_configuration_policy_associations(
        &self,
        targets: &[(String, String)],
    ) -> (
        Vec<wire::ConfigurationPolicyAssociationSummary>,
        Vec<wire::UnprocessedConfigurationPolicyAssociation>,
    ) {
        let mut found = Vec::new();
        let mut unprocessed = Vec::new();
        for (target_id, target_type) in targets {
            match self.policy_associations.get(target_id) {
                Some(assoc) => {
                    found.push(wire::ConfigurationPolicyAssociationSummary {
                        configuration_policy_id: Some(assoc.configuration_policy_id.clone()),
                        target_id: Some(assoc.target_id.clone()),
                        target_type: Some(assoc.target_type.clone()),
                        association_type: Some(assoc.association_type.clone()),
                        association_status: Some(assoc.association_status.clone()),
                        association_status_message: assoc.association_status_message.clone(),
                        updated_at: Some(assoc.updated_at.clone()),
                    });
                }
                None => {
                    unprocessed.push(wire::UnprocessedConfigurationPolicyAssociation {
                        configuration_policy_association_identifiers: Some(
                            wire::ConfigurationPolicyAssociation {
                                target: Some(wire::Target {
                                    account_id: if target_type == "ACCOUNT" {
                                        Some(target_id.clone())
                                    } else {
                                        None
                                    },
                                    organizational_unit_id: if target_type == "ORGANIZATIONAL_UNIT"
                                    {
                                        Some(target_id.clone())
                                    } else {
                                        None
                                    },
                                    root_id: if target_type == "ROOT" {
                                        Some(target_id.clone())
                                    } else {
                                        None
                                    },
                                }),
                            },
                        ),
                        error_code: Some("ResourceNotFoundException".to_string()),
                        error_reason: Some(format!("Association for target {target_id} not found")),
                    });
                }
            }
        }
        (found, unprocessed)
    }

    // --- Security Hub configuration ---

    pub fn update_security_hub_configuration(
        &mut self,
        auto_enable_controls: Option<bool>,
        control_finding_generator: Option<&str>,
    ) {
        if let Some(v) = auto_enable_controls {
            self.auto_enable_controls = v;
        }
        if let Some(g) = control_finding_generator {
            self.control_finding_generator = g.to_string();
        }
    }

    // --- Security controls ---

    pub fn get_security_control(&self, id: &str) -> Option<&SecurityControlInfo> {
        self.security_controls.get(id)
    }

    pub fn batch_get_security_controls(
        &self,
        ids: &[String],
    ) -> (Vec<&SecurityControlInfo>, Vec<(String, String, String)>) {
        let mut found = Vec::new();
        let mut unprocessed = Vec::new();
        for id in ids {
            if let Some(ctrl) = self.security_controls.get(id) {
                found.push(ctrl);
            } else {
                unprocessed.push((
                    id.clone(),
                    "RESOURCE_NOT_FOUND".to_string(),
                    format!("Security control {id} not found"),
                ));
            }
        }
        (found, unprocessed)
    }

    pub fn list_security_control_definitions(&self) -> Vec<&SecurityControlInfo> {
        self.security_controls.values().collect()
    }

    pub fn update_security_control(
        &mut self,
        security_control_id: &str,
        parameters: Option<serde_json::Value>,
        last_update_reason: Option<&str>,
    ) -> bool {
        if let Some(ctrl) = self.security_controls.get_mut(security_control_id) {
            if let Some(params) = parameters {
                if let Some(obj) = params.as_object() {
                    ctrl.parameters = obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                }
            }
            if let Some(reason) = last_update_reason {
                ctrl.last_update_reason = Some(reason.to_string());
            }
            true
        } else {
            false
        }
    }

    // --- Standards control associations ---

    pub fn batch_get_standards_control_associations(
        &self,
        ids: &[(String, String)],
    ) -> (
        Vec<&StandardsControlAssociationInfo>,
        Vec<(String, String, String, String)>,
    ) {
        let mut found = Vec::new();
        let mut unprocessed = Vec::new();
        for (security_control_id, standards_arn) in ids {
            let key = sca_key(security_control_id, standards_arn);
            if let Some(assoc) = self.standards_control_associations.get(&key) {
                found.push(assoc);
            } else {
                unprocessed.push((
                    security_control_id.clone(),
                    standards_arn.clone(),
                    "RESOURCE_NOT_FOUND".to_string(),
                    format!("Association for {security_control_id} / {standards_arn} not found"),
                ));
            }
        }
        (found, unprocessed)
    }

    pub fn batch_update_standards_control_associations(
        &mut self,
        updates: &[(String, String, String, Option<String>)],
    ) -> Vec<(String, String, String, String, String)> {
        let mut unprocessed = Vec::new();
        for (security_control_id, standards_arn, association_status, updated_reason) in updates {
            let key = sca_key(security_control_id, standards_arn);
            if let Some(assoc) = self.standards_control_associations.get_mut(&key) {
                assoc.association_status = association_status.clone();
                assoc.updated_reason = updated_reason.clone();
            } else {
                // If not found, create a new one
                self.standards_control_associations.insert(
                    key,
                    StandardsControlAssociationInfo {
                        security_control_id: security_control_id.clone(),
                        standards_arn: standards_arn.clone(),
                        association_status: association_status.clone(),
                        updated_reason: updated_reason.clone(),
                        security_control_arn: None,
                        standards_control_title: None,
                        standards_control_description: None,
                        standards_control_arns: Vec::new(),
                        related_requirements: Vec::new(),
                    },
                );
            }
        }
        unprocessed
    }

    pub fn list_standards_control_associations(&self) -> Vec<&StandardsControlAssociationInfo> {
        self.standards_control_associations.values().collect()
    }

    // --- Standards controls (legacy v1) ---

    pub fn describe_standards_controls(
        &self,
        _standards_subscription_arn: &str,
    ) -> Vec<&StandardsControlInfo> {
        // In a mock, return all controls for any subscription ARN.
        self.standards_controls.values().collect()
    }

    pub fn update_standards_control(
        &mut self,
        arn: &str,
        control_status: Option<&str>,
        disabled_reason: Option<&str>,
    ) -> bool {
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        if let Some(ctrl) = self.standards_controls.get_mut(arn) {
            if let Some(status) = control_status {
                ctrl.control_status = status.to_string();
                ctrl.control_status_updated_at = Some(now);
            }
            if let Some(reason) = disabled_reason {
                ctrl.disabled_reason = Some(reason.to_string());
            }
            true
        } else {
            // Create a new control entry for this arn
            self.standards_controls.insert(
                arn.to_string(),
                StandardsControlInfo {
                    standards_control_arn: arn.to_string(),
                    control_status: control_status.unwrap_or("ENABLED").to_string(),
                    disabled_reason: disabled_reason.map(|s| s.to_string()),
                    control_status_updated_at: Some(now),
                    control_id: None,
                    title: None,
                    description: None,
                    remediation_url: None,
                    severity_rating: None,
                    related_requirements: Vec::new(),
                },
            );
            true
        }
    }

    // --- Standards catalog ---

    pub fn describe_standards(&self) -> Vec<&StandardInfo> {
        self.standards.values().collect()
    }

    // --- Products catalog ---

    pub fn describe_products(&self) -> Vec<&ProductInfo> {
        self.products.values().collect()
    }

    // --- Tickets v2 ---

    pub fn create_ticket_v2(&mut self, body: &serde_json::Value) -> &TicketV2Info {
        let ticket_id = Uuid::new_v4().to_string();
        let ticket_src_url = body
            .get("TicketSrcUrl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        self.tickets_v2.insert(
            ticket_id.clone(),
            TicketV2Info {
                ticket_id: ticket_id.clone(),
                ticket_src_url,
            },
        );
        self.tickets_v2.get(&ticket_id).unwrap()
    }

    // --- Findings v2 (OCSF) ---

    pub fn get_findings_v2(
        &self,
        max_results: Option<i64>,
        next_token: Option<&str>,
    ) -> (Vec<&FindingV2>, Option<String>) {
        let limit = max_results.unwrap_or(100) as usize;
        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);
        let all: Vec<&FindingV2> = self.findings_v2.values().collect();
        let total = all.len();
        let end = (start + limit).min(total);
        let page = all[start..end].to_vec();
        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };
        (page, new_token)
    }

    pub fn batch_update_findings_v2(
        &mut self,
        finding_identifiers: &[serde_json::Value],
        updates: &serde_json::Value,
    ) -> (Vec<serde_json::Value>, Vec<serde_json::Value>) {
        let mut processed = Vec::new();
        let mut unprocessed = Vec::new();
        for identifier in finding_identifiers {
            let metadata_uid = identifier
                .get("MetadataUid")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let finding_info_uid = identifier
                .get("FindingInfoUid")
                .or_else(|| {
                    identifier
                        .get("FindingIdentifier")
                        .and_then(|fi| fi.get("FindingInfoUid"))
                })
                .and_then(|v| v.as_str())
                .unwrap_or("");

            // Try to find by metadata_uid first, then by finding_info_uid
            let found = if !metadata_uid.is_empty() {
                self.findings_v2.contains_key(metadata_uid)
            } else {
                self.findings_v2.values().any(|f| {
                    f.data
                        .get("finding_info")
                        .and_then(|fi| fi.get("uid"))
                        .and_then(|v| v.as_str())
                        == Some(finding_info_uid)
                })
            };

            if found {
                // Apply updates to the finding
                if !metadata_uid.is_empty() {
                    if let Some(finding) = self.findings_v2.get_mut(metadata_uid) {
                        if let Some(obj) = updates.as_object() {
                            for (k, v) in obj {
                                finding.data[k] = v.clone();
                            }
                        }
                    }
                }
                processed.push(identifier.clone());
            } else {
                unprocessed.push(identifier.clone());
            }
        }
        (processed, unprocessed)
    }

    // --- UpdateFindings (v1 - updates RecordState/Note) ---

    pub fn update_findings(
        &mut self,
        filters: &serde_json::Value,
        note: Option<&serde_json::Value>,
        record_state: Option<&str>,
    ) {
        // Apply updates to findings matching the filter criteria
        // In a mock, we apply to all findings if filters are present
        for finding in &mut self.findings {
            if let Some(n) = note {
                finding.data["Note"] = n.clone();
            }
            if let Some(rs) = record_state {
                finding.data["RecordState"] = serde_json::json!(rs);
            }
        }
    }

    // --- BatchUpdateFindings ---

    pub fn batch_update_findings(
        &mut self,
        finding_identifiers: &[serde_json::Value],
        note: Option<&serde_json::Value>,
        workflow: Option<&serde_json::Value>,
        severity: Option<&serde_json::Value>,
        confidence: Option<i64>,
        criticality: Option<i64>,
        types: Option<&[serde_json::Value]>,
        user_defined_fields: Option<&serde_json::Value>,
        related_findings: Option<&[serde_json::Value]>,
    ) -> (
        Vec<wire::AwsSecurityFindingIdentifier>,
        Vec<wire::BatchUpdateFindingsUnprocessedFinding>,
    ) {
        let mut processed = Vec::new();
        let mut unprocessed = Vec::new();

        for identifier in finding_identifiers {
            let id = identifier
                .get("Id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let product_arn = identifier
                .get("ProductArn")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if let Some(finding) = self.findings.iter_mut().find(|f| f.id == id) {
                if let Some(n) = note {
                    finding.data["Note"] = n.clone();
                }
                if let Some(w) = workflow {
                    finding.data["Workflow"] = w.clone();
                }
                if let Some(s) = severity {
                    finding.data["Severity"] = s.clone();
                }
                if let Some(c) = confidence {
                    finding.data["Confidence"] = serde_json::json!(c);
                }
                if let Some(cr) = criticality {
                    finding.data["Criticality"] = serde_json::json!(cr);
                }
                if let Some(t) = types {
                    finding.data["Types"] = serde_json::json!(t);
                }
                if let Some(udf) = user_defined_fields {
                    finding.data["UserDefinedFields"] = udf.clone();
                }
                if let Some(rf) = related_findings {
                    finding.data["RelatedFindings"] = serde_json::json!(rf);
                }
                processed.push(wire::AwsSecurityFindingIdentifier {
                    id: id.clone(),
                    product_arn: product_arn.clone(),
                });
            } else {
                unprocessed.push(wire::BatchUpdateFindingsUnprocessedFinding {
                    finding_identifier: Some(wire::AwsSecurityFindingIdentifier {
                        id: id.clone(),
                        product_arn: product_arn.clone(),
                    }),
                    error_code: Some("FindingNotFound".to_string()),
                    error_message: Some(format!("Finding with ID {id} not found")),
                });
            }
        }

        (processed, unprocessed)
    }
}
