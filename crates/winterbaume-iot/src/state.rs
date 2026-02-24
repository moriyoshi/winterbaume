use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

fn timestamp() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn sha256_hex(data: &str) -> String {
    // Simple deterministic hash for certificate IDs
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let h = hasher.finish();
    format!(
        "{:016x}{:016x}{:016x}{:016x}",
        h,
        h ^ 0xdead,
        h ^ 0xbeef,
        h ^ 0xcafe
    )
}

#[derive(Debug, Default)]
pub struct IotState {
    pub things: HashMap<String, Thing>,
    pub thing_types: HashMap<String, ThingType>,
    pub thing_groups: HashMap<String, ThingGroup>,
    pub billing_groups: HashMap<String, BillingGroup>,
    pub certificates: HashMap<String, Certificate>,
    pub ca_certificates: HashMap<String, CACertificate>,
    pub policies: HashMap<String, IotPolicy>,
    pub role_aliases: HashMap<String, RoleAlias>,
    pub domain_configurations: HashMap<String, DomainConfiguration>,
    pub jobs: HashMap<String, IotJob>,
    pub job_templates: HashMap<String, JobTemplate>,
    pub topic_rules: HashMap<String, TopicRule>,
    pub registration_code: String,
    pub indexing_config_thing: Option<serde_json::Value>,
    pub indexing_config_thing_group: Option<serde_json::Value>,
    /// Tags indexed by resource ARN — IoT exposes a single tag surface across
    /// things, thing types, policies, and other resources via `TagResource`,
    /// `UntagResource`, and `ListTagsForResource`.
    pub tags: HashMap<String, HashMap<String, String>>,
    next_id: u64,
    next_policy_version: u64,
    next_execution_number: i64,
}

#[derive(Debug, Error)]
pub enum IotError {
    // ResourceAlreadyExistsException (409)
    #[error("Thing {thing_name} already exists")]
    ThingAlreadyExists { thing_name: String },
    #[error("Thing type {thing_type_name} already exists")]
    ThingTypeAlreadyExists { thing_type_name: String },
    #[error("Thing group {thing_group_name} already exists")]
    ThingGroupAlreadyExists { thing_group_name: String },
    #[error("Billing group {name} already exists")]
    BillingGroupAlreadyExists { name: String },
    #[error("Policy {policy_name} already exists")]
    PolicyAlreadyExists { policy_name: String },
    #[error("Role alias {role_alias} already exists")]
    RoleAliasAlreadyExists { role_alias: String },
    #[error("Domain configuration {name} already exists")]
    DomainConfigurationAlreadyExists { name: String },
    #[error("Job {job_id} already exists")]
    JobAlreadyExists { job_id: String },
    #[error("Topic rule {rule_name} already exists")]
    TopicRuleAlreadyExists { rule_name: String },

    // ConflictException (409)
    #[error("Job template {job_template_id} already exists")]
    JobTemplateAlreadyExists { job_template_id: String },

    // ResourceNotFoundException (404)
    #[error("Thing {thing_name} does not exist")]
    ThingNotFound { thing_name: String },
    #[error("Thing type {thing_type_name} does not exist")]
    ThingTypeNotFound { thing_type_name: String },
    #[error("Thing group {name} does not exist")]
    ThingGroupNotFound { name: String },
    #[error("Billing group {name} does not exist")]
    BillingGroupNotFound { name: String },
    #[error("Certificate {cert_id} does not exist")]
    CertificateNotFound { cert_id: String },
    #[error("CA Certificate {cert_id} does not exist")]
    CaCertificateNotFound { cert_id: String },
    #[error("Policy {policy_name} does not exist")]
    PolicyNotFound { policy_name: String },
    #[error("Policy version {version_id} does not exist")]
    PolicyVersionNotFound { version_id: String },
    #[error("Role alias {role_alias} does not exist")]
    RoleAliasNotFound { role_alias: String },
    #[error("Domain configuration {name} does not exist")]
    DomainConfigurationNotFound { name: String },
    #[error("Job {job_id} does not exist")]
    JobNotFound { job_id: String },
    #[error("Job execution for {thing_name} does not exist")]
    JobExecutionNotFound { thing_name: String },
    #[error("Job template {job_template_id} does not exist")]
    JobTemplateNotFound { job_template_id: String },
    #[error("Topic rule {rule_name} does not exist")]
    TopicRuleNotFound { rule_name: String },

    // VersionConflictException (409)
    #[error("Version conflict")]
    VersionConflict,
    #[error("The version {actual} does not match the expected version {expected}")]
    VersionMismatch { actual: i64, expected: i64 },

    // VersionsLimitExceededException (409)
    #[error("The maximum number of policy versions (5) has been reached")]
    PolicyVersionsLimitExceeded,

    // InvalidRequestException (400)
    #[error("Cannot delete the default policy version")]
    CannotDeleteDefaultPolicyVersion,
}

impl IotState {
    fn generate_thing_id(&mut self) -> String {
        self.next_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_id, 0, 0, 0, self.next_id
        )
    }

    fn next_policy_version_id(&mut self) -> String {
        self.next_policy_version += 1;
        self.next_policy_version.to_string()
    }

    fn next_exec_number(&mut self) -> i64 {
        self.next_execution_number += 1;
        self.next_execution_number
    }

    // ==================== Thing ====================

    pub fn create_thing(
        &mut self,
        thing_name: &str,
        thing_type_name: Option<&str>,
        attributes: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&Thing, IotError> {
        if self.things.contains_key(thing_name) {
            return Err(IotError::ThingAlreadyExists {
                thing_name: thing_name.to_string(),
            });
        }

        let thing_id = self.generate_thing_id();
        let thing_arn = format!("arn:aws:iot:{region}:{account_id}:thing/{thing_name}");

        let thing = Thing {
            thing_name: thing_name.to_string(),
            thing_id,
            thing_type_name: thing_type_name.map(String::from),
            attributes,
            version: 1,
            thing_arn,
            billing_group_name: None,
            principals: Vec::new(),
            thing_groups: Vec::new(),
        };

        self.things.insert(thing_name.to_string(), thing);
        Ok(self.things.get(thing_name).unwrap())
    }

    pub fn describe_thing(&self, thing_name: &str) -> Result<&Thing, IotError> {
        self.things
            .get(thing_name)
            .ok_or_else(|| IotError::ThingNotFound {
                thing_name: thing_name.to_string(),
            })
    }

    pub fn update_thing(
        &mut self,
        thing_name: &str,
        thing_type_name: Option<&str>,
        attributes: Option<HashMap<String, String>>,
        expected_version: Option<i64>,
        remove_thing_type: bool,
    ) -> Result<(), IotError> {
        let thing = self
            .things
            .get_mut(thing_name)
            .ok_or_else(|| IotError::ThingNotFound {
                thing_name: thing_name.to_string(),
            })?;
        if let Some(expected) = expected_version {
            if thing.version != expected {
                return Err(IotError::VersionConflict);
            }
        }
        if remove_thing_type {
            thing.thing_type_name = None;
        } else if let Some(ttn) = thing_type_name {
            thing.thing_type_name = Some(ttn.to_string());
        }
        if let Some(attrs) = attributes {
            thing.attributes = attrs;
        }
        thing.version += 1;
        Ok(())
    }

    pub fn delete_thing(
        &mut self,
        thing_name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), IotError> {
        if let Some(expected) = expected_version {
            if let Some(thing) = self.things.get(thing_name) {
                if thing.version != expected {
                    return Err(IotError::VersionMismatch {
                        actual: thing.version,
                        expected,
                    });
                }
            }
        }

        if self.things.remove(thing_name).is_none() {
            return Err(IotError::ThingNotFound {
                thing_name: thing_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_things(&self) -> Vec<&Thing> {
        let mut things: Vec<&Thing> = self.things.values().collect();
        things.sort_by(|a, b| a.thing_name.cmp(&b.thing_name));
        things
    }

    // ==================== ThingType ====================

    pub fn create_thing_type(
        &mut self,
        thing_type_name: &str,
        description: Option<&str>,
        searchable_attributes: Option<Vec<String>>,
        region: &str,
        account_id: &str,
    ) -> Result<&ThingType, IotError> {
        if self.thing_types.contains_key(thing_type_name) {
            return Err(IotError::ThingTypeAlreadyExists {
                thing_type_name: thing_type_name.to_string(),
            });
        }
        let id = self.generate_thing_id();
        let arn = format!("arn:aws:iot:{region}:{account_id}:thingtype/{thing_type_name}");
        let tt = ThingType {
            thing_type_name: thing_type_name.to_string(),
            thing_type_id: id,
            thing_type_arn: arn,
            thing_type_description: description.map(String::from),
            searchable_attributes,
            creation_date: timestamp(),
            deprecated: false,
            deprecation_date: None,
        };
        self.thing_types.insert(thing_type_name.to_string(), tt);
        Ok(self.thing_types.get(thing_type_name).unwrap())
    }

    pub fn describe_thing_type(&self, thing_type_name: &str) -> Result<&ThingType, IotError> {
        self.thing_types
            .get(thing_type_name)
            .ok_or_else(|| IotError::ThingTypeNotFound {
                thing_type_name: thing_type_name.to_string(),
            })
    }

    pub fn delete_thing_type(&mut self, thing_type_name: &str) -> Result<(), IotError> {
        if self.thing_types.remove(thing_type_name).is_none() {
            return Err(IotError::ThingTypeNotFound {
                thing_type_name: thing_type_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn deprecate_thing_type(
        &mut self,
        thing_type_name: &str,
        undo: bool,
    ) -> Result<(), IotError> {
        let tt = self.thing_types.get_mut(thing_type_name).ok_or_else(|| {
            IotError::ThingTypeNotFound {
                thing_type_name: thing_type_name.to_string(),
            }
        })?;
        if undo {
            tt.deprecated = false;
            tt.deprecation_date = None;
        } else {
            tt.deprecated = true;
            tt.deprecation_date = Some(timestamp());
        }
        Ok(())
    }

    pub fn list_thing_types(&self) -> Vec<&ThingType> {
        let mut tts: Vec<&ThingType> = self.thing_types.values().collect();
        tts.sort_by(|a, b| a.thing_type_name.cmp(&b.thing_type_name));
        tts
    }

    // ==================== ThingGroup ====================

    pub fn create_thing_group(
        &mut self,
        thing_group_name: &str,
        parent_group_name: Option<&str>,
        description: Option<&str>,
        attributes: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&ThingGroup, IotError> {
        if self.thing_groups.contains_key(thing_group_name) {
            return Err(IotError::ThingGroupAlreadyExists {
                thing_group_name: thing_group_name.to_string(),
            });
        }
        let id = self.generate_thing_id();
        let arn = format!("arn:aws:iot:{region}:{account_id}:thinggroup/{thing_group_name}");
        let tg = ThingGroup {
            thing_group_name: thing_group_name.to_string(),
            thing_group_id: id,
            thing_group_arn: arn,
            parent_group_name: parent_group_name.map(String::from),
            thing_group_description: description.map(String::from),
            attributes,
            version: 1,
            creation_date: timestamp(),
            things: Vec::new(),
        };
        self.thing_groups.insert(thing_group_name.to_string(), tg);
        Ok(self.thing_groups.get(thing_group_name).unwrap())
    }

    pub fn describe_thing_group(&self, name: &str) -> Result<&ThingGroup, IotError> {
        self.thing_groups
            .get(name)
            .ok_or_else(|| IotError::ThingGroupNotFound {
                name: name.to_string(),
            })
    }

    pub fn update_thing_group(
        &mut self,
        name: &str,
        description: Option<&str>,
        attributes: Option<HashMap<String, String>>,
        expected_version: Option<i64>,
    ) -> Result<i64, IotError> {
        let tg = self
            .thing_groups
            .get_mut(name)
            .ok_or_else(|| IotError::ThingGroupNotFound {
                name: name.to_string(),
            })?;
        if let Some(ev) = expected_version {
            if tg.version != ev {
                return Err(IotError::VersionConflict);
            }
        }
        if let Some(d) = description {
            tg.thing_group_description = Some(d.to_string());
        }
        if let Some(a) = attributes {
            tg.attributes = a;
        }
        tg.version += 1;
        Ok(tg.version)
    }

    pub fn delete_thing_group(
        &mut self,
        name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), IotError> {
        if let Some(ev) = expected_version {
            if let Some(tg) = self.thing_groups.get(name) {
                if tg.version != ev {
                    return Err(IotError::VersionConflict);
                }
            }
        }
        // IoT DeleteThingGroup succeeds even if group doesn't exist
        self.thing_groups.remove(name);
        Ok(())
    }

    pub fn list_thing_groups(&self) -> Vec<&ThingGroup> {
        let mut gs: Vec<&ThingGroup> = self.thing_groups.values().collect();
        gs.sort_by(|a, b| a.thing_group_name.cmp(&b.thing_group_name));
        gs
    }

    pub fn add_thing_to_thing_group(
        &mut self,
        thing_group_name: Option<&str>,
        thing_name: Option<&str>,
    ) -> Result<(), IotError> {
        let gn = thing_group_name.unwrap_or("");
        let tn = thing_name.unwrap_or("");
        if !gn.is_empty() {
            if let Some(tg) = self.thing_groups.get_mut(gn) {
                if !tn.is_empty() && !tg.things.contains(&tn.to_string()) {
                    tg.things.push(tn.to_string());
                }
            }
        }
        if !tn.is_empty() {
            if let Some(t) = self.things.get_mut(tn) {
                if !gn.is_empty() && !t.thing_groups.contains(&gn.to_string()) {
                    t.thing_groups.push(gn.to_string());
                }
            }
        }
        Ok(())
    }

    pub fn remove_thing_from_thing_group(
        &mut self,
        thing_group_name: Option<&str>,
        thing_name: Option<&str>,
    ) -> Result<(), IotError> {
        let gn = thing_group_name.unwrap_or("");
        let tn = thing_name.unwrap_or("");
        if !gn.is_empty() {
            if let Some(tg) = self.thing_groups.get_mut(gn) {
                tg.things.retain(|t| t != tn);
            }
        }
        if !tn.is_empty() {
            if let Some(t) = self.things.get_mut(tn) {
                t.thing_groups.retain(|g| g != gn);
            }
        }
        Ok(())
    }

    pub fn list_things_in_thing_group(&self, name: &str) -> Result<Vec<String>, IotError> {
        let tg = self
            .thing_groups
            .get(name)
            .ok_or_else(|| IotError::ThingGroupNotFound {
                name: name.to_string(),
            })?;
        Ok(tg.things.clone())
    }

    pub fn list_thing_groups_for_thing(
        &self,
        thing_name: &str,
    ) -> Result<Vec<&ThingGroup>, IotError> {
        let thing = self
            .things
            .get(thing_name)
            .ok_or_else(|| IotError::ThingNotFound {
                thing_name: thing_name.to_string(),
            })?;
        let groups: Vec<&ThingGroup> = thing
            .thing_groups
            .iter()
            .filter_map(|gn| self.thing_groups.get(gn))
            .collect();
        Ok(groups)
    }

    pub fn update_thing_groups_for_thing(
        &mut self,
        thing_name: Option<&str>,
        groups_to_add: Option<Vec<String>>,
        groups_to_remove: Option<Vec<String>>,
    ) -> Result<(), IotError> {
        let tn = thing_name.unwrap_or("");
        if let Some(adds) = groups_to_add {
            for gn in adds {
                self.add_thing_to_thing_group(Some(&gn), Some(tn))?;
            }
        }
        if let Some(removes) = groups_to_remove {
            for gn in removes {
                self.remove_thing_from_thing_group(Some(&gn), Some(tn))?;
            }
        }
        Ok(())
    }

    // ==================== BillingGroup ====================

    pub fn create_billing_group(
        &mut self,
        name: &str,
        description: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&BillingGroup, IotError> {
        if self.billing_groups.contains_key(name) {
            return Err(IotError::BillingGroupAlreadyExists {
                name: name.to_string(),
            });
        }
        let id = self.generate_thing_id();
        let arn = format!("arn:aws:iot:{region}:{account_id}:billinggroup/{name}");
        let bg = BillingGroup {
            billing_group_name: name.to_string(),
            billing_group_id: id,
            billing_group_arn: arn,
            billing_group_description: description.map(String::from),
            version: 1,
            creation_date: timestamp(),
            things: Vec::new(),
        };
        self.billing_groups.insert(name.to_string(), bg);
        Ok(self.billing_groups.get(name).unwrap())
    }

    pub fn describe_billing_group(&self, name: &str) -> Result<&BillingGroup, IotError> {
        self.billing_groups
            .get(name)
            .ok_or_else(|| IotError::BillingGroupNotFound {
                name: name.to_string(),
            })
    }

    pub fn update_billing_group(
        &mut self,
        name: &str,
        description: Option<&str>,
        expected_version: Option<i64>,
    ) -> Result<i64, IotError> {
        let bg =
            self.billing_groups
                .get_mut(name)
                .ok_or_else(|| IotError::BillingGroupNotFound {
                    name: name.to_string(),
                })?;
        if let Some(ev) = expected_version {
            if bg.version != ev {
                return Err(IotError::VersionConflict);
            }
        }
        if let Some(d) = description {
            bg.billing_group_description = Some(d.to_string());
        }
        bg.version += 1;
        Ok(bg.version)
    }

    pub fn delete_billing_group(
        &mut self,
        name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), IotError> {
        if let Some(ev) = expected_version {
            if let Some(bg) = self.billing_groups.get(name) {
                if bg.version != ev {
                    return Err(IotError::VersionConflict);
                }
            }
        }
        self.billing_groups.remove(name);
        Ok(())
    }

    pub fn list_billing_groups(&self) -> Vec<&BillingGroup> {
        let mut bgs: Vec<&BillingGroup> = self.billing_groups.values().collect();
        bgs.sort_by(|a, b| a.billing_group_name.cmp(&b.billing_group_name));
        bgs
    }

    pub fn add_thing_to_billing_group(
        &mut self,
        billing_group_name: Option<&str>,
        thing_name: Option<&str>,
    ) -> Result<(), IotError> {
        let bgn = billing_group_name.unwrap_or("");
        let tn = thing_name.unwrap_or("");
        if !bgn.is_empty() {
            if let Some(bg) = self.billing_groups.get_mut(bgn) {
                if !tn.is_empty() && !bg.things.contains(&tn.to_string()) {
                    bg.things.push(tn.to_string());
                }
            }
        }
        if !tn.is_empty() {
            if let Some(t) = self.things.get_mut(tn) {
                if !bgn.is_empty() {
                    t.billing_group_name = Some(bgn.to_string());
                }
            }
        }
        Ok(())
    }

    pub fn remove_thing_from_billing_group(
        &mut self,
        billing_group_name: Option<&str>,
        thing_name: Option<&str>,
    ) -> Result<(), IotError> {
        let bgn = billing_group_name.unwrap_or("");
        let tn = thing_name.unwrap_or("");
        if !bgn.is_empty() {
            if let Some(bg) = self.billing_groups.get_mut(bgn) {
                bg.things.retain(|t| t != tn);
            }
        }
        if !tn.is_empty() {
            if let Some(t) = self.things.get_mut(tn) {
                if t.billing_group_name.as_deref() == Some(bgn) {
                    t.billing_group_name = None;
                }
            }
        }
        Ok(())
    }

    pub fn list_things_in_billing_group(&self, name: &str) -> Result<Vec<String>, IotError> {
        let bg = self
            .billing_groups
            .get(name)
            .ok_or_else(|| IotError::BillingGroupNotFound {
                name: name.to_string(),
            })?;
        Ok(bg.things.clone())
    }

    // ==================== Certificate ====================

    pub fn create_keys_and_certificate(
        &mut self,
        set_as_active: bool,
        region: &str,
        account_id: &str,
    ) -> Result<(String, String, String, String, String), IotError> {
        // Returns (cert_id, cert_arn, cert_pem, public_key, private_key)
        let cert_id = generate_id().replace('-', "");
        let cert_arn = format!("arn:aws:iot:{region}:{account_id}:cert/{cert_id}");
        let cert_pem =
            format!("-----BEGIN CERTIFICATE-----\nMOCK_CERT_{cert_id}\n-----END CERTIFICATE-----");
        let public_key =
            format!("-----BEGIN PUBLIC KEY-----\nMOCK_PUB_{cert_id}\n-----END PUBLIC KEY-----");
        let private_key = format!(
            "-----BEGIN RSA PRIVATE KEY-----\nMOCK_PRIV_{cert_id}\n-----END RSA PRIVATE KEY-----"
        );
        let status = if set_as_active { "ACTIVE" } else { "INACTIVE" };
        let cert = Certificate {
            certificate_id: cert_id.clone(),
            certificate_arn: cert_arn.clone(),
            certificate_pem: cert_pem.clone(),
            status: status.to_string(),
            creation_date: timestamp(),
            ca_certificate_id: None,
            owned_by: account_id.to_string(),
            mode: "DEFAULT".to_string(),
        };
        self.certificates.insert(cert_id.clone(), cert);
        Ok((cert_id, cert_arn, cert_pem, public_key, private_key))
    }

    pub fn create_certificate_from_csr(
        &mut self,
        csr: &str,
        set_as_active: bool,
        region: &str,
        account_id: &str,
    ) -> Result<(String, String, String), IotError> {
        let cert_id = sha256_hex(csr);
        let cert_arn = format!("arn:aws:iot:{region}:{account_id}:cert/{cert_id}");
        let cert_pem = format!(
            "-----BEGIN CERTIFICATE-----\nMOCK_CSR_CERT_{}\n-----END CERTIFICATE-----",
            &cert_id[..16]
        );
        let status = if set_as_active { "ACTIVE" } else { "INACTIVE" };
        let cert = Certificate {
            certificate_id: cert_id.clone(),
            certificate_arn: cert_arn.clone(),
            certificate_pem: cert_pem.clone(),
            status: status.to_string(),
            creation_date: timestamp(),
            ca_certificate_id: None,
            owned_by: account_id.to_string(),
            mode: "DEFAULT".to_string(),
        };
        self.certificates.insert(cert_id.clone(), cert);
        Ok((cert_id, cert_arn, cert_pem))
    }

    pub fn register_certificate(
        &mut self,
        certificate_pem: &str,
        ca_certificate_pem: Option<&str>,
        status: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<(String, String), IotError> {
        let cert_id = sha256_hex(certificate_pem);
        let cert_arn = format!("arn:aws:iot:{region}:{account_id}:cert/{cert_id}");
        let st = status.unwrap_or("INACTIVE");
        let ca_id = ca_certificate_pem.map(sha256_hex);
        let cert = Certificate {
            certificate_id: cert_id.clone(),
            certificate_arn: cert_arn.clone(),
            certificate_pem: certificate_pem.to_string(),
            status: st.to_string(),
            creation_date: timestamp(),
            ca_certificate_id: ca_id,
            owned_by: account_id.to_string(),
            mode: "DEFAULT".to_string(),
        };
        self.certificates.insert(cert_id.clone(), cert);
        Ok((cert_id, cert_arn))
    }

    pub fn register_certificate_without_ca(
        &mut self,
        certificate_pem: &str,
        status: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<(String, String), IotError> {
        let cert_id = sha256_hex(certificate_pem);
        let cert_arn = format!("arn:aws:iot:{region}:{account_id}:cert/{cert_id}");
        let st = status.unwrap_or("INACTIVE");
        let cert = Certificate {
            certificate_id: cert_id.clone(),
            certificate_arn: cert_arn.clone(),
            certificate_pem: certificate_pem.to_string(),
            status: st.to_string(),
            creation_date: timestamp(),
            ca_certificate_id: None,
            owned_by: account_id.to_string(),
            mode: "SNI_ONLY".to_string(),
        };
        self.certificates.insert(cert_id.clone(), cert);
        Ok((cert_id, cert_arn))
    }

    pub fn describe_certificate(&self, cert_id: &str) -> Result<&Certificate, IotError> {
        self.certificates
            .get(cert_id)
            .ok_or_else(|| IotError::CertificateNotFound {
                cert_id: cert_id.to_string(),
            })
    }

    pub fn update_certificate(&mut self, cert_id: &str, new_status: &str) -> Result<(), IotError> {
        let cert =
            self.certificates
                .get_mut(cert_id)
                .ok_or_else(|| IotError::CertificateNotFound {
                    cert_id: cert_id.to_string(),
                })?;
        cert.status = new_status.to_string();
        Ok(())
    }

    pub fn delete_certificate(&mut self, cert_id: &str) -> Result<(), IotError> {
        if self.certificates.remove(cert_id).is_none() {
            return Err(IotError::CertificateNotFound {
                cert_id: cert_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_certificates(&self) -> Vec<&Certificate> {
        let mut certs: Vec<&Certificate> = self.certificates.values().collect();
        certs.sort_by(|a, b| a.certificate_id.cmp(&b.certificate_id));
        certs
    }

    pub fn list_certificates_by_ca(&self, ca_cert_id: &str) -> Vec<&Certificate> {
        let mut certs: Vec<&Certificate> = self
            .certificates
            .values()
            .filter(|c| c.ca_certificate_id.as_deref() == Some(ca_cert_id))
            .collect();
        certs.sort_by(|a, b| a.certificate_id.cmp(&b.certificate_id));
        certs
    }

    // ==================== CA Certificate ====================

    pub fn register_ca_certificate(
        &mut self,
        ca_cert_pem: &str,
        set_as_active: bool,
        allow_auto_reg: bool,
        region: &str,
        account_id: &str,
    ) -> Result<(String, String), IotError> {
        let cert_id = sha256_hex(ca_cert_pem);
        let cert_arn = format!("arn:aws:iot:{region}:{account_id}:cacert/{cert_id}");
        let status = if set_as_active { "ACTIVE" } else { "INACTIVE" };
        let auto_reg = if allow_auto_reg { "ENABLE" } else { "DISABLE" };
        let ca = CACertificate {
            certificate_id: cert_id.clone(),
            certificate_arn: cert_arn.clone(),
            certificate_pem: ca_cert_pem.to_string(),
            status: status.to_string(),
            auto_registration_status: auto_reg.to_string(),
            creation_date: timestamp(),
            owned_by: account_id.to_string(),
            mode: "DEFAULT".to_string(),
        };
        self.ca_certificates.insert(cert_id.clone(), ca);
        Ok((cert_id, cert_arn))
    }

    pub fn describe_ca_certificate(&self, cert_id: &str) -> Result<&CACertificate, IotError> {
        self.ca_certificates
            .get(cert_id)
            .ok_or_else(|| IotError::CaCertificateNotFound {
                cert_id: cert_id.to_string(),
            })
    }

    pub fn update_ca_certificate(
        &mut self,
        cert_id: &str,
        new_status: Option<&str>,
        new_auto_reg: Option<&str>,
    ) -> Result<(), IotError> {
        let ca = self.ca_certificates.get_mut(cert_id).ok_or_else(|| {
            IotError::CaCertificateNotFound {
                cert_id: cert_id.to_string(),
            }
        })?;
        if let Some(s) = new_status {
            ca.status = s.to_string();
        }
        if let Some(a) = new_auto_reg {
            ca.auto_registration_status = a.to_string();
        }
        Ok(())
    }

    pub fn delete_ca_certificate(&mut self, cert_id: &str) -> Result<(), IotError> {
        if self.ca_certificates.remove(cert_id).is_none() {
            return Err(IotError::CaCertificateNotFound {
                cert_id: cert_id.to_string(),
            });
        }
        Ok(())
    }

    // ==================== Policy ====================

    pub fn create_policy(
        &mut self,
        policy_name: &str,
        policy_document: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&IotPolicy, IotError> {
        if self.policies.contains_key(policy_name) {
            return Err(IotError::PolicyAlreadyExists {
                policy_name: policy_name.to_string(),
            });
        }
        let arn = format!("arn:aws:iot:{region}:{account_id}:policy/{policy_name}");
        let now = timestamp();
        let vid = self.next_policy_version_id();
        let pv = PolicyVersionData {
            version_id: vid.clone(),
            policy_document: policy_document.to_string(),
            is_default: true,
            create_date: now,
        };
        let policy = IotPolicy {
            policy_name: policy_name.to_string(),
            policy_arn: arn,
            policy_document: policy_document.to_string(),
            creation_date: now,
            last_modified_date: now,
            generation_id: generate_id(),
            versions: vec![pv],
            default_version_id: vid,
            targets: Vec::new(),
            principals: Vec::new(),
        };
        self.policies.insert(policy_name.to_string(), policy);
        Ok(self.policies.get(policy_name).unwrap())
    }

    pub fn get_policy(&self, policy_name: &str) -> Result<&IotPolicy, IotError> {
        self.policies
            .get(policy_name)
            .ok_or_else(|| IotError::PolicyNotFound {
                policy_name: policy_name.to_string(),
            })
    }

    pub fn create_policy_version(
        &mut self,
        policy_name: &str,
        policy_document: &str,
        set_as_default: bool,
    ) -> Result<PolicyVersionData, IotError> {
        let policy =
            self.policies
                .get_mut(policy_name)
                .ok_or_else(|| IotError::PolicyNotFound {
                    policy_name: policy_name.to_string(),
                })?;
        if policy.versions.len() >= 5 {
            return Err(IotError::PolicyVersionsLimitExceeded);
        }
        let vid = self.next_policy_version_id();
        let pv = PolicyVersionData {
            version_id: vid.clone(),
            policy_document: policy_document.to_string(),
            is_default: set_as_default,
            create_date: timestamp(),
        };
        // Re-borrow after self method call
        let policy = self.policies.get_mut(policy_name).unwrap();
        if set_as_default {
            for v in &mut policy.versions {
                v.is_default = false;
            }
            policy.default_version_id = vid;
            policy.policy_document = policy_document.to_string();
            policy.last_modified_date = timestamp();
            policy.generation_id = generate_id();
        }
        let pv_clone = pv.clone();
        policy.versions.push(pv);
        Ok(pv_clone)
    }

    pub fn get_policy_version(
        &self,
        policy_name: &str,
        version_id: &str,
    ) -> Result<(&IotPolicy, &PolicyVersionData), IotError> {
        let policy = self.get_policy(policy_name)?;
        let pv = policy
            .versions
            .iter()
            .find(|v| v.version_id == version_id)
            .ok_or_else(|| IotError::PolicyVersionNotFound {
                version_id: version_id.to_string(),
            })?;
        Ok((policy, pv))
    }

    pub fn set_default_policy_version(
        &mut self,
        policy_name: &str,
        version_id: &str,
    ) -> Result<(), IotError> {
        let policy =
            self.policies
                .get_mut(policy_name)
                .ok_or_else(|| IotError::PolicyNotFound {
                    policy_name: policy_name.to_string(),
                })?;
        let mut found = false;
        let mut doc = String::new();
        for v in &mut policy.versions {
            if v.version_id == version_id {
                v.is_default = true;
                found = true;
                doc = v.policy_document.clone();
            } else {
                v.is_default = false;
            }
        }
        if !found {
            return Err(IotError::PolicyVersionNotFound {
                version_id: version_id.to_string(),
            });
        }
        policy.default_version_id = version_id.to_string();
        policy.policy_document = doc;
        policy.last_modified_date = timestamp();
        policy.generation_id = generate_id();
        Ok(())
    }

    pub fn delete_policy_version(
        &mut self,
        policy_name: &str,
        version_id: &str,
    ) -> Result<(), IotError> {
        let policy =
            self.policies
                .get_mut(policy_name)
                .ok_or_else(|| IotError::PolicyNotFound {
                    policy_name: policy_name.to_string(),
                })?;
        let idx = policy
            .versions
            .iter()
            .position(|v| v.version_id == version_id)
            .ok_or_else(|| IotError::PolicyVersionNotFound {
                version_id: version_id.to_string(),
            })?;
        if policy.versions[idx].is_default {
            return Err(IotError::CannotDeleteDefaultPolicyVersion);
        }
        policy.versions.remove(idx);
        Ok(())
    }

    pub fn list_policy_versions(
        &self,
        policy_name: &str,
    ) -> Result<&Vec<PolicyVersionData>, IotError> {
        let policy = self.get_policy(policy_name)?;
        Ok(&policy.versions)
    }

    pub fn delete_policy(&mut self, policy_name: &str) -> Result<(), IotError> {
        if self.policies.remove(policy_name).is_none() {
            return Err(IotError::PolicyNotFound {
                policy_name: policy_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_policies(&self) -> Vec<&IotPolicy> {
        let mut ps: Vec<&IotPolicy> = self.policies.values().collect();
        ps.sort_by(|a, b| a.policy_name.cmp(&b.policy_name));
        ps
    }

    pub fn attach_policy(&mut self, policy_name: &str, target: &str) -> Result<(), IotError> {
        let policy =
            self.policies
                .get_mut(policy_name)
                .ok_or_else(|| IotError::PolicyNotFound {
                    policy_name: policy_name.to_string(),
                })?;
        if !policy.targets.contains(&target.to_string()) {
            policy.targets.push(target.to_string());
        }
        Ok(())
    }

    pub fn detach_policy(&mut self, policy_name: &str, target: &str) -> Result<(), IotError> {
        let policy =
            self.policies
                .get_mut(policy_name)
                .ok_or_else(|| IotError::PolicyNotFound {
                    policy_name: policy_name.to_string(),
                })?;
        policy.targets.retain(|t| t != target);
        Ok(())
    }

    pub fn attach_principal_policy(
        &mut self,
        policy_name: &str,
        principal: &str,
    ) -> Result<(), IotError> {
        let policy =
            self.policies
                .get_mut(policy_name)
                .ok_or_else(|| IotError::PolicyNotFound {
                    policy_name: policy_name.to_string(),
                })?;
        if !policy.principals.contains(&principal.to_string()) {
            policy.principals.push(principal.to_string());
        }
        Ok(())
    }

    pub fn detach_principal_policy(
        &mut self,
        policy_name: &str,
        principal: &str,
    ) -> Result<(), IotError> {
        let policy =
            self.policies
                .get_mut(policy_name)
                .ok_or_else(|| IotError::PolicyNotFound {
                    policy_name: policy_name.to_string(),
                })?;
        policy.principals.retain(|p| p != principal);
        Ok(())
    }

    pub fn list_attached_policies(&self, target: &str) -> Vec<&IotPolicy> {
        self.policies
            .values()
            .filter(|p| p.targets.contains(&target.to_string()))
            .collect()
    }

    pub fn list_targets_for_policy(&self, policy_name: &str) -> Result<Vec<String>, IotError> {
        let policy = self.get_policy(policy_name)?;
        Ok(policy.targets.clone())
    }

    pub fn list_policy_principals(&self, policy_name: &str) -> Result<Vec<String>, IotError> {
        let policy = self.get_policy(policy_name)?;
        Ok(policy.principals.clone())
    }

    pub fn list_principal_policies(&self, principal: &str) -> Vec<&IotPolicy> {
        self.policies
            .values()
            .filter(|p| p.principals.contains(&principal.to_string()))
            .collect()
    }

    // ==================== Thing Principal ====================

    pub fn attach_thing_principal(
        &mut self,
        thing_name: &str,
        principal: &str,
    ) -> Result<(), IotError> {
        let thing = self
            .things
            .get_mut(thing_name)
            .ok_or_else(|| IotError::ThingNotFound {
                thing_name: thing_name.to_string(),
            })?;
        if !thing.principals.contains(&principal.to_string()) {
            thing.principals.push(principal.to_string());
        }
        Ok(())
    }

    pub fn detach_thing_principal(
        &mut self,
        thing_name: &str,
        principal: &str,
    ) -> Result<(), IotError> {
        let thing = self
            .things
            .get_mut(thing_name)
            .ok_or_else(|| IotError::ThingNotFound {
                thing_name: thing_name.to_string(),
            })?;
        thing.principals.retain(|p| p != principal);
        Ok(())
    }

    pub fn list_thing_principals(&self, thing_name: &str) -> Result<Vec<String>, IotError> {
        let thing = self
            .things
            .get(thing_name)
            .ok_or_else(|| IotError::ThingNotFound {
                thing_name: thing_name.to_string(),
            })?;
        Ok(thing.principals.clone())
    }

    pub fn list_thing_principals_v2(&self, thing_name: &str) -> Result<Vec<String>, IotError> {
        self.list_thing_principals(thing_name)
    }

    pub fn list_principal_things(&self, principal: &str) -> Vec<String> {
        self.things
            .values()
            .filter(|t| t.principals.contains(&principal.to_string()))
            .map(|t| t.thing_name.clone())
            .collect()
    }

    // ==================== RoleAlias ====================

    pub fn create_role_alias(
        &mut self,
        role_alias: &str,
        role_arn: &str,
        credential_duration: Option<i32>,
        region: &str,
        account_id: &str,
    ) -> Result<&RoleAlias, IotError> {
        if self.role_aliases.contains_key(role_alias) {
            return Err(IotError::RoleAliasAlreadyExists {
                role_alias: role_alias.to_string(),
            });
        }
        let arn = format!("arn:aws:iot:{region}:{account_id}:rolealias/{role_alias}");
        let now = timestamp();
        let ra = RoleAlias {
            role_alias: role_alias.to_string(),
            role_alias_arn: arn,
            role_arn: role_arn.to_string(),
            credential_duration_seconds: credential_duration.unwrap_or(3600),
            creation_date: now,
            last_modified_date: now,
            owner: account_id.to_string(),
        };
        self.role_aliases.insert(role_alias.to_string(), ra);
        Ok(self.role_aliases.get(role_alias).unwrap())
    }

    pub fn describe_role_alias(&self, role_alias: &str) -> Result<&RoleAlias, IotError> {
        self.role_aliases
            .get(role_alias)
            .ok_or_else(|| IotError::RoleAliasNotFound {
                role_alias: role_alias.to_string(),
            })
    }

    pub fn update_role_alias(
        &mut self,
        role_alias_name: &str,
        role_arn: Option<&str>,
        credential_duration: Option<i32>,
    ) -> Result<&RoleAlias, IotError> {
        let ra = self.role_aliases.get_mut(role_alias_name).ok_or_else(|| {
            IotError::RoleAliasNotFound {
                role_alias: role_alias_name.to_string(),
            }
        })?;
        if let Some(arn) = role_arn {
            ra.role_arn = arn.to_string();
        }
        if let Some(d) = credential_duration {
            ra.credential_duration_seconds = d;
        }
        ra.last_modified_date = timestamp();
        Ok(ra)
    }

    pub fn delete_role_alias(&mut self, role_alias: &str) -> Result<(), IotError> {
        if self.role_aliases.remove(role_alias).is_none() {
            return Err(IotError::RoleAliasNotFound {
                role_alias: role_alias.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_role_aliases(&self) -> Vec<String> {
        let mut names: Vec<String> = self.role_aliases.keys().cloned().collect();
        names.sort();
        names
    }

    // ==================== DomainConfiguration ====================

    pub fn create_domain_configuration(
        &mut self,
        name: &str,
        domain_name: Option<&str>,
        service_type: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&DomainConfiguration, IotError> {
        if self.domain_configurations.contains_key(name) {
            return Err(IotError::DomainConfigurationAlreadyExists {
                name: name.to_string(),
            });
        }
        let arn = format!("arn:aws:iot:{region}:{account_id}:domainconfiguration/{name}");
        let dc = DomainConfiguration {
            domain_configuration_name: name.to_string(),
            domain_configuration_arn: arn,
            domain_name: domain_name.map(String::from),
            domain_configuration_status: "ENABLED".to_string(),
            service_type: service_type.map(String::from),
            creation_date: timestamp(),
        };
        self.domain_configurations.insert(name.to_string(), dc);
        Ok(self.domain_configurations.get(name).unwrap())
    }

    pub fn describe_domain_configuration(
        &self,
        name: &str,
    ) -> Result<&DomainConfiguration, IotError> {
        self.domain_configurations
            .get(name)
            .ok_or_else(|| IotError::DomainConfigurationNotFound {
                name: name.to_string(),
            })
    }

    pub fn update_domain_configuration(
        &mut self,
        name: &str,
        status: Option<&str>,
    ) -> Result<&DomainConfiguration, IotError> {
        let dc = self.domain_configurations.get_mut(name).ok_or_else(|| {
            IotError::DomainConfigurationNotFound {
                name: name.to_string(),
            }
        })?;
        if let Some(s) = status {
            dc.domain_configuration_status = s.to_string();
        }
        Ok(dc)
    }

    pub fn delete_domain_configuration(&mut self, name: &str) -> Result<(), IotError> {
        if self.domain_configurations.remove(name).is_none() {
            return Err(IotError::DomainConfigurationNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_domain_configurations(&self) -> Vec<&DomainConfiguration> {
        let mut dcs: Vec<&DomainConfiguration> = self.domain_configurations.values().collect();
        dcs.sort_by(|a, b| {
            a.domain_configuration_name
                .cmp(&b.domain_configuration_name)
        });
        dcs
    }

    // ==================== Job ====================

    pub fn create_job(
        &mut self,
        job_id: &str,
        targets: Vec<String>,
        document: Option<&str>,
        document_source: Option<&str>,
        description: Option<&str>,
        target_selection: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&IotJob, IotError> {
        if self.jobs.contains_key(job_id) {
            return Err(IotError::JobAlreadyExists {
                job_id: job_id.to_string(),
            });
        }
        let arn = format!("arn:aws:iot:{region}:{account_id}:job/{job_id}");
        let now = timestamp();
        let mut job = IotJob {
            job_id: job_id.to_string(),
            job_arn: arn,
            description: description.map(String::from),
            targets,
            document: document.map(String::from),
            document_source: document_source.map(String::from),
            status: "IN_PROGRESS".to_string(),
            target_selection: target_selection.map(String::from),
            creation_date: now,
            last_updated_date: now,
            completed_date: None,
            comment: None,
            reason_code: None,
            force_canceled: None,
            job_template_arn: None,
            executions: HashMap::new(),
        };
        // Auto-create executions for thing targets
        for target in &job.targets {
            if let Some(thing_name) = target.rsplit('/').next() {
                if target.contains(":thing/") && self.things.contains_key(thing_name) {
                    let exec_num = self.next_exec_number();
                    job.executions.insert(
                        thing_name.to_string(),
                        JobExecData {
                            status: "QUEUED".to_string(),
                            queued_at: now,
                            started_at: None,
                            last_updated_at: now,
                            execution_number: exec_num,
                            version_number: 1,
                        },
                    );
                }
            }
        }
        self.jobs.insert(job_id.to_string(), job);
        Ok(self.jobs.get(job_id).unwrap())
    }

    pub fn describe_job(&self, job_id: &str) -> Result<&IotJob, IotError> {
        self.jobs.get(job_id).ok_or_else(|| IotError::JobNotFound {
            job_id: job_id.to_string(),
        })
    }

    pub fn cancel_job(
        &mut self,
        job_id: &str,
        reason_code: Option<&str>,
        comment: Option<&str>,
        force: bool,
    ) -> Result<&IotJob, IotError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| IotError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        job.status = "CANCELED".to_string();
        job.reason_code = reason_code.map(String::from);
        job.comment = comment.map(String::from);
        job.force_canceled = Some(force);
        job.last_updated_date = timestamp();
        Ok(job)
    }

    pub fn delete_job(&mut self, job_id: &str) -> Result<(), IotError> {
        if self.jobs.remove(job_id).is_none() {
            return Err(IotError::JobNotFound {
                job_id: job_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_jobs(&self) -> Vec<&IotJob> {
        let mut js: Vec<&IotJob> = self.jobs.values().collect();
        js.sort_by(|a, b| a.job_id.cmp(&b.job_id));
        js
    }

    pub fn get_job_document(&self, job_id: &str) -> Result<Option<String>, IotError> {
        let job = self.describe_job(job_id)?;
        // Moto returns empty string when document_source is set but document is not
        match &job.document {
            Some(doc) => Ok(Some(doc.clone())),
            None if job.document_source.is_some() => Ok(Some(String::new())),
            None => Ok(None),
        }
    }

    pub fn describe_job_execution(
        &self,
        job_id: &str,
        thing_name: &str,
    ) -> Result<(&IotJob, &JobExecData), IotError> {
        let job = self.describe_job(job_id)?;
        let exec =
            job.executions
                .get(thing_name)
                .ok_or_else(|| IotError::JobExecutionNotFound {
                    thing_name: thing_name.to_string(),
                })?;
        Ok((job, exec))
    }

    pub fn cancel_job_execution(&mut self, job_id: &str, thing_name: &str) -> Result<(), IotError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| IotError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        if let Some(exec) = job.executions.get_mut(thing_name) {
            exec.status = "CANCELED".to_string();
            exec.last_updated_at = timestamp();
        }
        Ok(())
    }

    pub fn delete_job_execution(&mut self, job_id: &str, thing_name: &str) -> Result<(), IotError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| IotError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        job.executions.remove(thing_name);
        Ok(())
    }

    pub fn list_job_executions_for_job(
        &self,
        job_id: &str,
    ) -> Result<Vec<(&str, &JobExecData)>, IotError> {
        let job = self.describe_job(job_id)?;
        Ok(job
            .executions
            .iter()
            .map(|(k, v)| (k.as_str(), v))
            .collect())
    }

    pub fn list_job_executions_for_thing(&self, thing_name: &str) -> Vec<(&str, &JobExecData)> {
        self.jobs
            .values()
            .filter_map(|j| j.executions.get(thing_name).map(|e| (j.job_id.as_str(), e)))
            .collect()
    }

    // ==================== JobTemplate ====================

    pub fn create_job_template(
        &mut self,
        job_template_id: &str,
        description: &str,
        document: Option<&str>,
        document_source: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&JobTemplate, IotError> {
        if self.job_templates.contains_key(job_template_id) {
            return Err(IotError::JobTemplateAlreadyExists {
                job_template_id: job_template_id.to_string(),
            });
        }
        let arn = format!("arn:aws:iot:{region}:{account_id}:jobtemplate/{job_template_id}");
        let jt = JobTemplate {
            job_template_id: job_template_id.to_string(),
            job_template_arn: arn,
            description: description.to_string(),
            document: document.map(String::from),
            document_source: document_source.map(String::from),
            creation_date: timestamp(),
        };
        self.job_templates.insert(job_template_id.to_string(), jt);
        Ok(self.job_templates.get(job_template_id).unwrap())
    }

    pub fn describe_job_template(&self, job_template_id: &str) -> Result<&JobTemplate, IotError> {
        self.job_templates
            .get(job_template_id)
            .ok_or_else(|| IotError::JobTemplateNotFound {
                job_template_id: job_template_id.to_string(),
            })
    }

    pub fn delete_job_template(&mut self, job_template_id: &str) -> Result<(), IotError> {
        if self.job_templates.remove(job_template_id).is_none() {
            return Err(IotError::JobTemplateNotFound {
                job_template_id: job_template_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_job_templates(&self) -> Vec<&JobTemplate> {
        let mut jts: Vec<&JobTemplate> = self.job_templates.values().collect();
        jts.sort_by(|a, b| a.job_template_id.cmp(&b.job_template_id));
        jts
    }

    // ==================== TopicRule ====================

    pub fn create_topic_rule(
        &mut self,
        rule_name: &str,
        sql: &str,
        description: Option<&str>,
        actions_json: serde_json::Value,
        error_action_json: Option<serde_json::Value>,
        aws_iot_sql_version: Option<&str>,
        rule_disabled: bool,
        region: &str,
        account_id: &str,
    ) -> Result<(), IotError> {
        if self.topic_rules.contains_key(rule_name) {
            return Err(IotError::TopicRuleAlreadyExists {
                rule_name: rule_name.to_string(),
            });
        }
        let arn = format!("arn:aws:iot:{region}:{account_id}:rule/{rule_name}");
        let tr = TopicRule {
            rule_name: rule_name.to_string(),
            rule_arn: arn,
            sql: sql.to_string(),
            description: description.map(String::from),
            rule_disabled,
            creation_date: timestamp(),
            actions_json,
            error_action_json,
            aws_iot_sql_version: aws_iot_sql_version.map(String::from),
        };
        self.topic_rules.insert(rule_name.to_string(), tr);
        Ok(())
    }

    pub fn get_topic_rule(&self, rule_name: &str) -> Result<&TopicRule, IotError> {
        self.topic_rules
            .get(rule_name)
            .ok_or_else(|| IotError::TopicRuleNotFound {
                rule_name: rule_name.to_string(),
            })
    }

    pub fn delete_topic_rule(&mut self, rule_name: &str) -> Result<(), IotError> {
        if self.topic_rules.remove(rule_name).is_none() {
            return Err(IotError::TopicRuleNotFound {
                rule_name: rule_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn replace_topic_rule(
        &mut self,
        rule_name: &str,
        sql: &str,
        description: Option<&str>,
        actions_json: serde_json::Value,
        error_action_json: Option<serde_json::Value>,
        aws_iot_sql_version: Option<&str>,
        rule_disabled: bool,
    ) -> Result<(), IotError> {
        let tr =
            self.topic_rules
                .get_mut(rule_name)
                .ok_or_else(|| IotError::TopicRuleNotFound {
                    rule_name: rule_name.to_string(),
                })?;
        tr.sql = sql.to_string();
        tr.description = description.map(String::from);
        tr.actions_json = actions_json;
        tr.error_action_json = error_action_json;
        tr.aws_iot_sql_version = aws_iot_sql_version.map(String::from);
        tr.rule_disabled = rule_disabled;
        Ok(())
    }

    pub fn enable_topic_rule(&mut self, rule_name: &str) -> Result<(), IotError> {
        let tr =
            self.topic_rules
                .get_mut(rule_name)
                .ok_or_else(|| IotError::TopicRuleNotFound {
                    rule_name: rule_name.to_string(),
                })?;
        tr.rule_disabled = false;
        Ok(())
    }

    pub fn disable_topic_rule(&mut self, rule_name: &str) -> Result<(), IotError> {
        let tr =
            self.topic_rules
                .get_mut(rule_name)
                .ok_or_else(|| IotError::TopicRuleNotFound {
                    rule_name: rule_name.to_string(),
                })?;
        tr.rule_disabled = true;
        Ok(())
    }

    pub fn list_topic_rules(&self) -> Vec<&TopicRule> {
        let mut trs: Vec<&TopicRule> = self.topic_rules.values().collect();
        trs.sort_by(|a, b| a.rule_name.cmp(&b.rule_name));
        trs
    }

    // ==================== Misc ====================

    pub fn describe_endpoint(&self, region: &str) -> String {
        format!("{region}-ats.iot.{region}.amazonaws.com")
    }

    pub fn get_registration_code(&mut self) -> String {
        if self.registration_code.is_empty() {
            self.registration_code = generate_id().replace('-', "");
        }
        self.registration_code.clone()
    }

    pub fn search_index(&self, query_string: &str) -> Vec<&Thing> {
        // Simple mock: return all things if query is "*", else filter by name contains
        if query_string == "*" {
            return self.list_things();
        }
        self.things
            .values()
            .filter(|t| t.thing_name.contains(query_string))
            .collect()
    }

    // ==================== Tags ====================

    pub fn tag_resource(&mut self, resource_arn: &str, tags: Vec<(String, String)>) {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for k in tag_keys {
                entry.remove(k);
            }
            if entry.is_empty() {
                self.tags.remove(resource_arn);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<(String, String)> {
        self.tags
            .get(resource_arn)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }
}
