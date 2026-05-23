use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CloudFrontState {
    pub distributions: HashMap<String, Distribution>,
    pub invalidations: HashMap<String, Vec<Invalidation>>,
    pub key_groups: HashMap<String, KeyGroupData>,
    pub origin_access_controls: HashMap<String, OriginAccessControlData>,
    pub public_keys: HashMap<String, PublicKeyData>,
    pub cache_policies: HashMap<String, CachePolicyData>,
    pub origin_request_policies: HashMap<String, OriginRequestPolicyData>,
    pub response_headers_policies: HashMap<String, ResponseHeadersPolicyData>,
    pub functions: HashMap<String, CloudFrontFunctionData>,
    pub oais: HashMap<String, OaiData>,
    pub streaming_distributions: HashMap<String, StreamingDistributionData>,
    pub key_value_stores: HashMap<String, KeyValueStoreData>,
    pub trust_stores: HashMap<String, TrustStoreData>,
    pub vpc_origins: HashMap<String, VpcOriginData>,
    pub anycast_ip_lists: HashMap<String, AnycastIpListData>,
    pub realtime_log_configs: HashMap<String, RealtimeLogConfigData>,
    pub field_level_encryptions: HashMap<String, FieldLevelEncryptionData>,
    pub field_level_encryption_profiles: HashMap<String, FieldLevelEncryptionProfileData>,
    pub connection_functions: HashMap<String, ConnectionFunctionData>,
    pub connection_groups: HashMap<String, ConnectionGroupData>,
    pub continuous_deployment_policies: HashMap<String, ContinuousDeploymentPolicyData>,
    pub distribution_tenants: HashMap<String, DistributionTenantData>,
    pub monitoring_subscriptions: HashMap<String, MonitoringSubscriptionData>,
    pub resource_policies: HashMap<String, ResourcePolicyData>,
    pub web_acl_associations: HashMap<String, WebAclAssociation>,
}

#[derive(Debug, thiserror::Error)]
pub enum CloudFrontError {
    #[error(
        "The caller reference that you are using to create a distribution is associated with another distribution. Already exists: {0}"
    )]
    DistributionAlreadyExists(String),
    #[error("The specified distribution does not exist: {0}")]
    NoSuchDistribution(String),
    #[error(
        "The precondition given in one or more of the request-header fields evaluated to false."
    )]
    PreconditionFailed,
    #[error("The specified invalidation does not exist: {0}")]
    NoSuchInvalidation(String),
    #[error("A key group with the name {0} already exists.")]
    KeyGroupAlreadyExists(String),
    #[error("{0}")]
    NoSuchResource(String),
    #[error("The specified origin access control does not exist: {0}")]
    NoSuchOriginAccessControl(String),
    #[error("The specified public key does not exist: {0}")]
    NoSuchPublicKey(String),
    #[error("The specified cache policy does not exist: {0}")]
    NoSuchCachePolicy(String),
    #[error("The specified origin request policy does not exist: {0}")]
    NoSuchOriginRequestPolicy(String),
    #[error("The specified response headers policy does not exist: {0}")]
    NoSuchResponseHeadersPolicy(String),
    #[error("The specified function does not exist: {0}")]
    NoSuchFunctionExists(String),
    #[error("The caller reference {0} is already used by another OAI.")]
    CloudFrontOriginAccessIdentityAlreadyExists(String),
    #[error("The specified origin access identity does not exist: {0}")]
    NoSuchCloudFrontOriginAccessIdentity(String),
    #[error("The caller reference is already in use.")]
    StreamingDistributionAlreadyExists,
    #[error("{0}")]
    EntityAlreadyExists(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("The specified streaming distribution does not exist: {0}")]
    NoSuchStreamingDistribution(String),
    #[error("A realtime log config named {0} already exists.")]
    RealtimeLogConfigAlreadyExists(String),
    #[error("The specified realtime log config does not exist: {0}")]
    NoSuchRealtimeLogConfig(String),
    #[error("Name or ARN must be provided.")]
    InvalidArgument,
    #[error("The specified field-level encryption config does not exist: {0}")]
    NoSuchFieldLevelEncryptionConfig(String),
    #[error("The specified field-level encryption profile does not exist: {0}")]
    NoSuchFieldLevelEncryptionProfile(String),
    #[error("The specified anycast IP list does not exist: {0}")]
    NoSuchAnycastIpList(String),
    #[error("The specified connection function does not exist: {0}")]
    NoSuchConnectionFunction(String),
    #[error("The specified connection group does not exist: {0}")]
    NoSuchConnectionGroup(String),
    #[error("The specified continuous deployment policy does not exist: {0}")]
    NoSuchContinuousDeploymentPolicy(String),
    #[error("The specified distribution tenant does not exist: {0}")]
    NoSuchDistributionTenant(String),
    #[error("The specified monitoring subscription does not exist: {0}")]
    NoSuchMonitoringSubscription(String),
}

impl CloudFrontState {
    // ---- Distribution operations ----

    pub fn create_distribution(
        &mut self,
        caller_reference: &str,
        origins: Vec<Origin>,
        default_cache_behavior: DefaultCacheBehavior,
        account_id: &str,
        enabled: bool,
        tags: HashMap<String, String>,
        raw_config: crate::model::DistributionConfig,
    ) -> Result<&Distribution, CloudFrontError> {
        for dist in self.distributions.values() {
            if dist.caller_reference == caller_reference {
                return Err(CloudFrontError::DistributionAlreadyExists(dist.id.clone()));
            }
        }

        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:distribution/{id}");
        let domain_name = format!("{id}.cloudfront.net").to_lowercase();
        let etag = uuid::Uuid::new_v4().to_string();

        let dist = Distribution {
            id: id.clone(),
            arn,
            domain_name,
            status: "Deployed".to_string(),
            origins,
            default_cache_behavior,
            caller_reference: caller_reference.to_string(),
            created_at: Utc::now(),
            etag,
            enabled,
            tags,
            raw_config,
        };

        self.distributions.insert(id.clone(), dist);
        Ok(self.distributions.get(&id).unwrap())
    }

    pub fn get_distribution(&self, id: &str) -> Result<&Distribution, CloudFrontError> {
        self.distributions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchDistribution(id.to_string()))
    }

    pub fn get_distribution_config(&self, id: &str) -> Result<&Distribution, CloudFrontError> {
        self.get_distribution(id)
    }

    pub fn update_distribution(
        &mut self,
        id: &str,
        caller_reference: &str,
        origins: Vec<Origin>,
        default_cache_behavior: DefaultCacheBehavior,
        enabled: bool,
        if_match: Option<&str>,
        raw_config: crate::model::DistributionConfig,
    ) -> Result<&Distribution, CloudFrontError> {
        let dist = self
            .distributions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchDistribution(id.to_string()))?;

        if let Some(expected) = if_match
            && dist.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let dist = self.distributions.get_mut(id).unwrap();
        dist.caller_reference = caller_reference.to_string();
        dist.origins = origins;
        dist.default_cache_behavior = default_cache_behavior;
        dist.enabled = enabled;
        dist.raw_config = raw_config;
        dist.etag = uuid::Uuid::new_v4().to_string();

        Ok(self.distributions.get(id).unwrap())
    }

    pub fn delete_distribution(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        // The Smithy model marks `If-Match` on DeleteDistribution as a real
        // input. Terraform's `aws_cloudfront_distribution` destroy path sends
        // it on every call, and real CloudFront returns 412 on mismatch.
        if let Some(expected) = if_match
            && let Some(dist) = self.distributions.get(id)
            && dist.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.distributions.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchDistribution(id.to_string()));
        }
        Ok(())
    }

    pub fn list_distributions(&self) -> Vec<&Distribution> {
        self.distributions.values().collect()
    }

    // ---- Invalidation operations ----

    pub fn create_invalidation(
        &mut self,
        distribution_id: &str,
        caller_reference: &str,
        paths: Vec<String>,
    ) -> Result<&Invalidation, CloudFrontError> {
        if !self.distributions.contains_key(distribution_id) {
            return Err(CloudFrontError::NoSuchDistribution(
                distribution_id.to_string(),
            ));
        }

        let id = format!(
            "I{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let inv = Invalidation {
            id: id.clone(),
            distribution_id: distribution_id.to_string(),
            status: "COMPLETED".to_string(),
            create_time: Utc::now(),
            caller_reference: caller_reference.to_string(),
            paths,
        };

        let invs = self
            .invalidations
            .entry(distribution_id.to_string())
            .or_default();
        invs.push(inv);
        Ok(invs.last().unwrap())
    }

    pub fn get_invalidation(
        &self,
        distribution_id: &str,
        invalidation_id: &str,
    ) -> Result<&Invalidation, CloudFrontError> {
        if !self.distributions.contains_key(distribution_id) {
            return Err(CloudFrontError::NoSuchDistribution(
                distribution_id.to_string(),
            ));
        }

        self.invalidations
            .get(distribution_id)
            .and_then(|invs| invs.iter().find(|i| i.id == invalidation_id))
            .ok_or_else(|| CloudFrontError::NoSuchInvalidation(invalidation_id.to_string()))
    }

    pub fn list_invalidations(&self, distribution_id: &str) -> Vec<&Invalidation> {
        self.invalidations
            .get(distribution_id)
            .map(|invs| invs.iter().collect())
            .unwrap_or_default()
    }

    // ---- Key Group operations ----

    pub fn create_key_group(
        &mut self,
        name: &str,
        items: Vec<String>,
        comment: Option<String>,
    ) -> Result<&KeyGroupData, CloudFrontError> {
        for kg in self.key_groups.values() {
            if kg.name == name {
                return Err(CloudFrontError::KeyGroupAlreadyExists(name.to_string()));
            }
        }

        let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let etag = uuid::Uuid::new_v4().to_string();
        let kg = KeyGroupData {
            id: id.clone(),
            name: name.to_string(),
            comment,
            items,
            last_modified_time: Utc::now(),
            etag,
        };
        self.key_groups.insert(id.clone(), kg);
        Ok(self.key_groups.get(&id).unwrap())
    }

    pub fn get_key_group(&self, id: &str) -> Result<&KeyGroupData, CloudFrontError> {
        self.key_groups.get(id).ok_or_else(|| {
            CloudFrontError::NoSuchResource(format!("The specified key group does not exist: {id}"))
        })
    }

    pub fn update_key_group(
        &mut self,
        id: &str,
        name: &str,
        items: Vec<String>,
        comment: Option<String>,
        if_match: Option<&str>,
    ) -> Result<&KeyGroupData, CloudFrontError> {
        let kg = self.key_groups.get(id).ok_or_else(|| {
            CloudFrontError::NoSuchResource(format!("The specified key group does not exist: {id}"))
        })?;

        if let Some(expected) = if_match
            && kg.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let kg = self.key_groups.get_mut(id).unwrap();
        kg.name = name.to_string();
        kg.items = items;
        kg.comment = comment;
        kg.last_modified_time = Utc::now();
        kg.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.key_groups.get(id).unwrap())
    }

    pub fn delete_key_group(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(kg) = self.key_groups.get(id)
            && kg.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.key_groups.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchResource(format!(
                "The specified key group does not exist: {id}"
            )));
        }
        Ok(())
    }

    pub fn list_key_groups(&self) -> Vec<&KeyGroupData> {
        self.key_groups.values().collect()
    }

    // ---- Origin Access Control operations ----

    pub fn create_origin_access_control(
        &mut self,
        name: &str,
        description: Option<String>,
        origin_type: &str,
        signing_behavior: &str,
        signing_protocol: &str,
    ) -> Result<&OriginAccessControlData, CloudFrontError> {
        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let etag = uuid::Uuid::new_v4().to_string();
        let oac = OriginAccessControlData {
            id: id.clone(),
            name: name.to_string(),
            description,
            origin_access_control_origin_type: origin_type.to_string(),
            signing_behavior: signing_behavior.to_string(),
            signing_protocol: signing_protocol.to_string(),
            etag,
        };
        self.origin_access_controls.insert(id.clone(), oac);
        Ok(self.origin_access_controls.get(&id).unwrap())
    }

    pub fn get_origin_access_control(
        &self,
        id: &str,
    ) -> Result<&OriginAccessControlData, CloudFrontError> {
        self.origin_access_controls
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchOriginAccessControl(id.to_string()))
    }

    pub fn update_origin_access_control(
        &mut self,
        id: &str,
        name: &str,
        description: Option<String>,
        origin_type: &str,
        signing_behavior: &str,
        signing_protocol: &str,
        if_match: Option<&str>,
    ) -> Result<&OriginAccessControlData, CloudFrontError> {
        let oac = self
            .origin_access_controls
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchOriginAccessControl(id.to_string()))?;

        if let Some(expected) = if_match
            && oac.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let oac = self.origin_access_controls.get_mut(id).unwrap();
        oac.name = name.to_string();
        oac.description = description;
        oac.origin_access_control_origin_type = origin_type.to_string();
        oac.signing_behavior = signing_behavior.to_string();
        oac.signing_protocol = signing_protocol.to_string();
        oac.etag = uuid::Uuid::new_v4().to_string();

        Ok(self.origin_access_controls.get(id).unwrap())
    }

    pub fn delete_origin_access_control(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        // The Smithy model marks `If-Match` on DeleteOriginAccessControl as a
        // real input. Terraform sends it on destroy of
        // `aws_cloudfront_origin_access_control`.
        if let Some(expected) = if_match
            && let Some(oac) = self.origin_access_controls.get(id)
            && oac.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.origin_access_controls.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchOriginAccessControl(id.to_string()));
        }
        Ok(())
    }

    pub fn list_origin_access_controls(&self) -> Vec<&OriginAccessControlData> {
        self.origin_access_controls.values().collect()
    }

    // ---- Public Key operations ----

    pub fn create_public_key(
        &mut self,
        caller_reference: &str,
        name: &str,
        encoded_key: &str,
        comment: Option<String>,
    ) -> Result<&PublicKeyData, CloudFrontError> {
        let id = format!(
            "K{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let etag = uuid::Uuid::new_v4().to_string();
        let pk = PublicKeyData {
            id: id.clone(),
            name: name.to_string(),
            caller_reference: caller_reference.to_string(),
            encoded_key: encoded_key.to_string(),
            comment,
            created_time: Utc::now(),
            etag,
        };
        self.public_keys.insert(id.clone(), pk);
        Ok(self.public_keys.get(&id).unwrap())
    }

    pub fn get_public_key(&self, id: &str) -> Result<&PublicKeyData, CloudFrontError> {
        self.public_keys
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchPublicKey(id.to_string()))
    }

    pub fn update_public_key(
        &mut self,
        id: &str,
        caller_reference: &str,
        name: &str,
        encoded_key: &str,
        comment: Option<String>,
        if_match: Option<&str>,
    ) -> Result<&PublicKeyData, CloudFrontError> {
        let pk = self
            .public_keys
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchPublicKey(id.to_string()))?;

        if let Some(expected) = if_match
            && pk.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let pk = self.public_keys.get_mut(id).unwrap();
        pk.caller_reference = caller_reference.to_string();
        pk.name = name.to_string();
        pk.encoded_key = encoded_key.to_string();
        pk.comment = comment;
        pk.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.public_keys.get(id).unwrap())
    }

    pub fn delete_public_key(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(pk) = self.public_keys.get(id)
            && pk.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.public_keys.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchPublicKey(id.to_string()));
        }
        Ok(())
    }

    pub fn list_public_keys(&self) -> Vec<&PublicKeyData> {
        self.public_keys.values().collect()
    }

    // ---- Tag operations ----

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), CloudFrontError> {
        let dist = self.distributions.values_mut().find(|d| d.arn == arn);
        match dist {
            Some(d) => {
                d.tags.extend(tags);
                Ok(())
            }
            None => Err(CloudFrontError::NoSuchResource(format!(
                "The specified resource does not exist: {arn}"
            ))),
        }
    }

    pub fn untag_resource(
        &mut self,
        arn: &str,
        tag_keys: &[String],
    ) -> Result<(), CloudFrontError> {
        let dist = self.distributions.values_mut().find(|d| d.arn == arn);
        match dist {
            Some(d) => {
                for key in tag_keys {
                    d.tags.remove(key);
                }
                Ok(())
            }
            None => Err(CloudFrontError::NoSuchResource(format!(
                "The specified resource does not exist: {arn}"
            ))),
        }
    }

    pub fn list_tags_for_resource(
        &self,
        arn: &str,
    ) -> Result<&HashMap<String, String>, CloudFrontError> {
        let dist = self.distributions.values().find(|d| d.arn == arn);
        match dist {
            Some(d) => Ok(&d.tags),
            None => Err(CloudFrontError::NoSuchResource(format!(
                "The specified resource does not exist: {arn}"
            ))),
        }
    }

    // ---- Cache Policy operations ----

    pub fn create_cache_policy(&mut self, config: CachePolicyConfig) -> &CachePolicyData {
        let id = format!(
            "CP{}",
            &uuid::Uuid::new_v4().to_string()[..11]
                .to_uppercase()
                .replace('-', "")
        );
        let etag = uuid::Uuid::new_v4().to_string();
        let cp = CachePolicyData {
            id: id.clone(),
            last_modified_time: Utc::now(),
            config,
            etag,
        };
        self.cache_policies.insert(id.clone(), cp);
        self.cache_policies.get(&id).unwrap()
    }

    pub fn get_cache_policy(&self, id: &str) -> Result<&CachePolicyData, CloudFrontError> {
        self.cache_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchCachePolicy(id.to_string()))
    }

    pub fn update_cache_policy(
        &mut self,
        id: &str,
        config: CachePolicyConfig,
        if_match: Option<&str>,
    ) -> Result<&CachePolicyData, CloudFrontError> {
        let cp = self
            .cache_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchCachePolicy(id.to_string()))?;

        if let Some(expected) = if_match
            && cp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let cp = self.cache_policies.get_mut(id).unwrap();
        cp.config = config;
        cp.last_modified_time = Utc::now();
        cp.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.cache_policies.get(id).unwrap())
    }

    pub fn delete_cache_policy(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let cp = self
            .cache_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchCachePolicy(id.to_string()))?;

        if let Some(expected) = if_match
            && cp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        self.cache_policies.remove(id);
        Ok(())
    }

    pub fn list_cache_policies(&self) -> Vec<&CachePolicyData> {
        self.cache_policies.values().collect()
    }

    // ---- Origin Request Policy operations ----

    pub fn create_origin_request_policy(
        &mut self,
        config: OriginRequestPolicyConfig,
    ) -> &OriginRequestPolicyData {
        let id = uuid::Uuid::new_v4().to_string();
        let etag = uuid::Uuid::new_v4().to_string();
        let orp = OriginRequestPolicyData {
            id: id.clone(),
            last_modified_time: Utc::now(),
            config,
            etag,
        };
        self.origin_request_policies.insert(id.clone(), orp);
        self.origin_request_policies.get(&id).unwrap()
    }

    pub fn get_origin_request_policy(
        &self,
        id: &str,
    ) -> Result<&OriginRequestPolicyData, CloudFrontError> {
        self.origin_request_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchOriginRequestPolicy(id.to_string()))
    }

    pub fn update_origin_request_policy(
        &mut self,
        id: &str,
        config: OriginRequestPolicyConfig,
        if_match: Option<&str>,
    ) -> Result<&OriginRequestPolicyData, CloudFrontError> {
        let orp = self
            .origin_request_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchOriginRequestPolicy(id.to_string()))?;

        if let Some(expected) = if_match
            && orp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let orp = self.origin_request_policies.get_mut(id).unwrap();
        orp.config = config;
        orp.last_modified_time = Utc::now();
        orp.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.origin_request_policies.get(id).unwrap())
    }

    pub fn delete_origin_request_policy(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(orp) = self.origin_request_policies.get(id)
            && orp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.origin_request_policies.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchOriginRequestPolicy(id.to_string()));
        }
        Ok(())
    }

    pub fn list_origin_request_policies(&self) -> Vec<&OriginRequestPolicyData> {
        self.origin_request_policies.values().collect()
    }

    // ---- Response Headers Policy operations ----

    pub fn create_response_headers_policy(
        &mut self,
        config: ResponseHeadersPolicyConfig,
    ) -> &ResponseHeadersPolicyData {
        let id = uuid::Uuid::new_v4().to_string();
        let etag = uuid::Uuid::new_v4().to_string();
        let rhp = ResponseHeadersPolicyData {
            id: id.clone(),
            last_modified_time: Utc::now(),
            config,
            etag,
        };
        self.response_headers_policies.insert(id.clone(), rhp);
        self.response_headers_policies.get(&id).unwrap()
    }

    pub fn get_response_headers_policy(
        &self,
        id: &str,
    ) -> Result<&ResponseHeadersPolicyData, CloudFrontError> {
        self.response_headers_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchResponseHeadersPolicy(id.to_string()))
    }

    pub fn update_response_headers_policy(
        &mut self,
        id: &str,
        config: ResponseHeadersPolicyConfig,
        if_match: Option<&str>,
    ) -> Result<&ResponseHeadersPolicyData, CloudFrontError> {
        let rhp = self
            .response_headers_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchResponseHeadersPolicy(id.to_string()))?;

        if let Some(expected) = if_match
            && rhp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let rhp = self.response_headers_policies.get_mut(id).unwrap();
        rhp.config = config;
        rhp.last_modified_time = Utc::now();
        rhp.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.response_headers_policies.get(id).unwrap())
    }

    pub fn delete_response_headers_policy(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(rhp) = self.response_headers_policies.get(id)
            && rhp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.response_headers_policies.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchResponseHeadersPolicy(id.to_string()));
        }
        Ok(())
    }

    pub fn list_response_headers_policies(&self) -> Vec<&ResponseHeadersPolicyData> {
        self.response_headers_policies.values().collect()
    }

    // ---- CloudFront Function operations ----

    pub fn create_function(
        &mut self,
        name: &str,
        config: FunctionConfig,
        code: Vec<u8>,
        account_id: &str,
    ) -> &CloudFrontFunctionData {
        let arn = format!("arn:aws:cloudfront::{account_id}:function/{name}");
        let etag = uuid::Uuid::new_v4().to_string();
        let func = CloudFrontFunctionData {
            name: name.to_string(),
            arn,
            status: "UNPUBLISHED".to_string(),
            created_time: Utc::now(),
            last_modified_time: Utc::now(),
            config,
            code,
            etag,
        };
        self.functions.insert(name.to_string(), func);
        self.functions.get(name).unwrap()
    }

    pub fn get_function(&self, name: &str) -> Result<&CloudFrontFunctionData, CloudFrontError> {
        self.functions
            .get(name)
            .ok_or_else(|| CloudFrontError::NoSuchFunctionExists(name.to_string()))
    }

    pub fn update_function(
        &mut self,
        name: &str,
        config: FunctionConfig,
        code: Vec<u8>,
        if_match: Option<&str>,
    ) -> Result<&CloudFrontFunctionData, CloudFrontError> {
        let func = self
            .functions
            .get(name)
            .ok_or_else(|| CloudFrontError::NoSuchFunctionExists(name.to_string()))?;

        if let Some(expected) = if_match
            && func.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let func = self.functions.get_mut(name).unwrap();
        func.config = config;
        func.code = code;
        func.last_modified_time = Utc::now();
        func.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.functions.get(name).unwrap())
    }

    pub fn delete_function(
        &mut self,
        name: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let func = self
            .functions
            .get(name)
            .ok_or_else(|| CloudFrontError::NoSuchFunctionExists(name.to_string()))?;

        if let Some(expected) = if_match
            && func.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        self.functions.remove(name);
        Ok(())
    }

    pub fn publish_function(
        &mut self,
        name: &str,
        if_match: Option<&str>,
    ) -> Result<&CloudFrontFunctionData, CloudFrontError> {
        let func = self
            .functions
            .get(name)
            .ok_or_else(|| CloudFrontError::NoSuchFunctionExists(name.to_string()))?;

        if let Some(expected) = if_match
            && func.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }

        let func = self.functions.get_mut(name).unwrap();
        func.status = "DEPLOYED".to_string();
        func.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.functions.get(name).unwrap())
    }

    pub fn list_functions(&self) -> Vec<&CloudFrontFunctionData> {
        self.functions.values().collect()
    }

    // ---- CloudFront Origin Access Identity (OAI) operations ----

    pub fn create_oai(
        &mut self,
        caller_reference: &str,
        comment: &str,
    ) -> Result<&OaiData, CloudFrontError> {
        for oai in self.oais.values() {
            if oai.caller_reference == caller_reference {
                return Err(
                    CloudFrontError::CloudFrontOriginAccessIdentityAlreadyExists(
                        caller_reference.to_string(),
                    ),
                );
            }
        }
        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let etag = uuid::Uuid::new_v4().to_string();
        let oai = OaiData {
            id: id.clone(),
            caller_reference: caller_reference.to_string(),
            comment: comment.to_string(),
            s3_canonical_user_id: format!("{id}-canonical"),
            etag,
        };
        self.oais.insert(id.clone(), oai);
        Ok(self.oais.get(&id).unwrap())
    }

    pub fn get_oai(&self, id: &str) -> Result<&OaiData, CloudFrontError> {
        self.oais
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchCloudFrontOriginAccessIdentity(id.to_string()))
    }

    pub fn update_oai(
        &mut self,
        id: &str,
        caller_reference: &str,
        comment: &str,
        if_match: Option<&str>,
    ) -> Result<&OaiData, CloudFrontError> {
        let oai = self
            .oais
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchCloudFrontOriginAccessIdentity(id.to_string()))?;
        if let Some(expected) = if_match
            && oai.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let oai = self.oais.get_mut(id).unwrap();
        oai.caller_reference = caller_reference.to_string();
        oai.comment = comment.to_string();
        oai.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.oais.get(id).unwrap())
    }

    pub fn delete_oai(&mut self, id: &str, if_match: Option<&str>) -> Result<(), CloudFrontError> {
        let oai = self
            .oais
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchCloudFrontOriginAccessIdentity(id.to_string()))?;
        if let Some(expected) = if_match
            && oai.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        self.oais.remove(id);
        Ok(())
    }

    pub fn list_oais(&self) -> Vec<&OaiData> {
        self.oais.values().collect()
    }

    // ---- Streaming Distribution operations ----

    pub fn create_streaming_distribution(
        &mut self,
        config: StreamingDistributionConfig,
        account_id: &str,
    ) -> Result<&StreamingDistributionData, CloudFrontError> {
        for sd in self.streaming_distributions.values() {
            if sd.config.caller_reference == config.caller_reference {
                return Err(CloudFrontError::StreamingDistributionAlreadyExists);
            }
        }
        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:streaming-distribution/{id}");
        let domain_name = format!("{}.cloudfront.net", id.to_lowercase());
        let etag = uuid::Uuid::new_v4().to_string();
        let sd = StreamingDistributionData {
            id: id.clone(),
            arn,
            domain_name,
            status: "Deployed".to_string(),
            last_modified_time: Utc::now(),
            config,
            etag,
        };
        self.streaming_distributions.insert(id.clone(), sd);
        Ok(self.streaming_distributions.get(&id).unwrap())
    }

    pub fn get_streaming_distribution(
        &self,
        id: &str,
    ) -> Result<&StreamingDistributionData, CloudFrontError> {
        self.streaming_distributions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchStreamingDistribution(id.to_string()))
    }

    pub fn update_streaming_distribution(
        &mut self,
        id: &str,
        config: StreamingDistributionConfig,
        if_match: Option<&str>,
    ) -> Result<&StreamingDistributionData, CloudFrontError> {
        let sd = self
            .streaming_distributions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchStreamingDistribution(id.to_string()))?;
        if let Some(expected) = if_match
            && sd.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let sd = self.streaming_distributions.get_mut(id).unwrap();
        sd.config = config;
        sd.last_modified_time = Utc::now();
        sd.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.streaming_distributions.get(id).unwrap())
    }

    pub fn delete_streaming_distribution(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let sd = self
            .streaming_distributions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchStreamingDistribution(id.to_string()))?;
        if let Some(expected) = if_match
            && sd.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        self.streaming_distributions.remove(id);
        Ok(())
    }

    pub fn list_streaming_distributions(&self) -> Vec<&StreamingDistributionData> {
        self.streaming_distributions.values().collect()
    }

    // ---- Key Value Store operations ----

    pub fn create_key_value_store(
        &mut self,
        name: &str,
        comment: Option<String>,
        account_id: &str,
    ) -> Result<&KeyValueStoreData, CloudFrontError> {
        if self.key_value_stores.contains_key(name) {
            return Err(CloudFrontError::EntityAlreadyExists(format!(
                "A key value store with name {name} already exists."
            )));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:cloudfront::{account_id}:key-value-store/{name}");
        let etag = uuid::Uuid::new_v4().to_string();
        let kvs = KeyValueStoreData {
            name: name.to_string(),
            arn,
            id,
            comment,
            last_modified_time: Utc::now(),
            status: "READY".to_string(),
            etag,
        };
        self.key_value_stores.insert(name.to_string(), kvs);
        Ok(self.key_value_stores.get(name).unwrap())
    }

    pub fn get_key_value_store(&self, name: &str) -> Result<&KeyValueStoreData, CloudFrontError> {
        self.key_value_stores.get(name).ok_or_else(|| {
            CloudFrontError::EntityNotFound(format!(
                "The specified key value store does not exist: {name}"
            ))
        })
    }

    pub fn update_key_value_store(
        &mut self,
        name: &str,
        comment: Option<String>,
        if_match: Option<&str>,
    ) -> Result<&KeyValueStoreData, CloudFrontError> {
        let kvs = self.key_value_stores.get(name).ok_or_else(|| {
            CloudFrontError::EntityNotFound(format!(
                "The specified key value store does not exist: {name}"
            ))
        })?;
        if let Some(expected) = if_match
            && kvs.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let kvs = self.key_value_stores.get_mut(name).unwrap();
        kvs.comment = comment;
        kvs.last_modified_time = Utc::now();
        kvs.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.key_value_stores.get(name).unwrap())
    }

    pub fn delete_key_value_store(
        &mut self,
        name: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let kvs = self.key_value_stores.get(name).ok_or_else(|| {
            CloudFrontError::EntityNotFound(format!(
                "The specified key value store does not exist: {name}"
            ))
        })?;
        if let Some(expected) = if_match
            && kvs.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        self.key_value_stores.remove(name);
        Ok(())
    }

    pub fn list_key_value_stores(&self) -> Vec<&KeyValueStoreData> {
        self.key_value_stores.values().collect()
    }

    // ---- Trust Store operations ----

    pub fn create_trust_store(
        &mut self,
        name: &str,
        account_id: &str,
    ) -> Result<&TrustStoreData, CloudFrontError> {
        for ts in self.trust_stores.values() {
            if ts.name == name {
                return Err(CloudFrontError::EntityAlreadyExists(format!(
                    "A trust store named {name} already exists."
                )));
            }
        }
        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:trust-store/{id}");
        let etag = uuid::Uuid::new_v4().to_string();
        let ts = TrustStoreData {
            id: id.clone(),
            arn,
            name: name.to_string(),
            last_modified_time: Utc::now(),
            status: "Active".to_string(),
            number_of_ca_certificates: 0,
            etag,
        };
        self.trust_stores.insert(id.clone(), ts);
        Ok(self.trust_stores.get(&id).unwrap())
    }

    pub fn get_trust_store(&self, id: &str) -> Result<&TrustStoreData, CloudFrontError> {
        self.trust_stores.get(id).ok_or_else(|| {
            CloudFrontError::EntityNotFound(format!(
                "The specified trust store does not exist: {id}"
            ))
        })
    }

    pub fn update_trust_store(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<&TrustStoreData, CloudFrontError> {
        let ts = self.trust_stores.get(id).ok_or_else(|| {
            CloudFrontError::EntityNotFound(format!(
                "The specified trust store does not exist: {id}"
            ))
        })?;
        if let Some(expected) = if_match
            && ts.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let ts = self.trust_stores.get_mut(id).unwrap();
        ts.last_modified_time = Utc::now();
        ts.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.trust_stores.get(id).unwrap())
    }

    pub fn delete_trust_store(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let ts = self.trust_stores.get(id).ok_or_else(|| {
            CloudFrontError::EntityNotFound(format!(
                "The specified trust store does not exist: {id}"
            ))
        })?;
        if let Some(expected) = if_match
            && ts.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        self.trust_stores.remove(id);
        Ok(())
    }

    pub fn list_trust_stores(&self) -> Vec<&TrustStoreData> {
        self.trust_stores.values().collect()
    }

    // ---- VPC Origin operations ----

    pub fn create_vpc_origin(
        &mut self,
        config: VpcOriginEndpointConfig,
        account_id: &str,
    ) -> &VpcOriginData {
        let id = format!(
            "vo{}",
            &uuid::Uuid::new_v4().to_string()[..12]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:vpc-origin/{id}");
        let etag = uuid::Uuid::new_v4().to_string();
        let vpc = VpcOriginData {
            id: id.clone(),
            arn,
            account_id: account_id.to_string(),
            created_time: Utc::now(),
            last_modified_time: Utc::now(),
            status: "Deployed".to_string(),
            config,
            etag,
        };
        self.vpc_origins.insert(id.clone(), vpc);
        self.vpc_origins.get(&id).unwrap()
    }

    pub fn get_vpc_origin(&self, id: &str) -> Result<&VpcOriginData, CloudFrontError> {
        self.vpc_origins.get(id).ok_or_else(|| {
            CloudFrontError::NoSuchResource(format!(
                "The specified VPC origin does not exist: {id}"
            ))
        })
    }

    pub fn update_vpc_origin(
        &mut self,
        id: &str,
        config: VpcOriginEndpointConfig,
        if_match: Option<&str>,
    ) -> Result<&VpcOriginData, CloudFrontError> {
        let vpc = self.vpc_origins.get(id).ok_or_else(|| {
            CloudFrontError::NoSuchResource(format!(
                "The specified VPC origin does not exist: {id}"
            ))
        })?;
        if let Some(expected) = if_match
            && vpc.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let vpc = self.vpc_origins.get_mut(id).unwrap();
        vpc.config = config;
        vpc.last_modified_time = Utc::now();
        vpc.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.vpc_origins.get(id).unwrap())
    }

    pub fn delete_vpc_origin(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<VpcOriginData, CloudFrontError> {
        let vpc = self.vpc_origins.get(id).ok_or_else(|| {
            CloudFrontError::NoSuchResource(format!(
                "The specified VPC origin does not exist: {id}"
            ))
        })?;
        if let Some(expected) = if_match
            && vpc.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        Ok(self.vpc_origins.remove(id).unwrap())
    }

    pub fn list_vpc_origins(&self) -> Vec<&VpcOriginData> {
        self.vpc_origins.values().collect()
    }

    // ---- Anycast IP List operations ----

    pub fn create_anycast_ip_list(
        &mut self,
        name: &str,
        ip_count: i32,
        ip_address_type: Option<String>,
        account_id: &str,
    ) -> &AnycastIpListData {
        let id = format!(
            "al{}",
            &uuid::Uuid::new_v4().to_string()[..12]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:anycast-ip-list/{id}");
        let etag = uuid::Uuid::new_v4().to_string();
        let list = AnycastIpListData {
            id: id.clone(),
            arn,
            name: name.to_string(),
            status: "Deployed".to_string(),
            ip_count,
            ip_address_type,
            last_modified_time: Utc::now(),
            etag,
        };
        self.anycast_ip_lists.insert(id.clone(), list);
        self.anycast_ip_lists.get(&id).unwrap()
    }

    pub fn get_anycast_ip_list(&self, id: &str) -> Result<&AnycastIpListData, CloudFrontError> {
        self.anycast_ip_lists
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchAnycastIpList(id.to_string()))
    }

    pub fn update_anycast_ip_list(
        &mut self,
        id: &str,
        ip_address_type: Option<String>,
        if_match: Option<&str>,
    ) -> Result<&AnycastIpListData, CloudFrontError> {
        let list = self
            .anycast_ip_lists
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchAnycastIpList(id.to_string()))?;
        if let Some(expected) = if_match
            && list.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let list = self.anycast_ip_lists.get_mut(id).unwrap();
        if ip_address_type.is_some() {
            list.ip_address_type = ip_address_type;
        }
        list.last_modified_time = Utc::now();
        list.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.anycast_ip_lists.get(id).unwrap())
    }

    pub fn delete_anycast_ip_list(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let list = self
            .anycast_ip_lists
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchAnycastIpList(id.to_string()))?;
        if let Some(expected) = if_match
            && list.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        self.anycast_ip_lists.remove(id);
        Ok(())
    }

    pub fn list_anycast_ip_lists(&self) -> Vec<&AnycastIpListData> {
        self.anycast_ip_lists.values().collect()
    }

    // ---- Realtime Log Config operations ----

    pub fn create_realtime_log_config(
        &mut self,
        name: &str,
        sampling_rate: i64,
        end_points: Vec<crate::model::EndPoint>,
        fields: Vec<String>,
        account_id: &str,
    ) -> Result<&RealtimeLogConfigData, CloudFrontError> {
        if self.realtime_log_configs.contains_key(name) {
            return Err(CloudFrontError::RealtimeLogConfigAlreadyExists(
                name.to_string(),
            ));
        }
        let arn = format!("arn:aws:cloudfront::{account_id}:realtime-log-config/{name}");
        let cfg = RealtimeLogConfigData {
            name: name.to_string(),
            arn,
            sampling_rate,
            end_points,
            fields,
        };
        self.realtime_log_configs.insert(name.to_string(), cfg);
        Ok(self.realtime_log_configs.get(name).unwrap())
    }

    pub fn get_realtime_log_config(
        &self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<&RealtimeLogConfigData, CloudFrontError> {
        if let Some(n) = name {
            return self
                .realtime_log_configs
                .get(n)
                .ok_or_else(|| CloudFrontError::NoSuchRealtimeLogConfig(n.to_string()));
        }
        if let Some(a) = arn {
            return self
                .realtime_log_configs
                .values()
                .find(|c| c.arn == a)
                .ok_or_else(|| CloudFrontError::NoSuchRealtimeLogConfig(a.to_string()));
        }
        Err(CloudFrontError::InvalidArgument)
    }

    pub fn update_realtime_log_config(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
        sampling_rate: i64,
        end_points: Vec<crate::model::EndPoint>,
        fields: Vec<String>,
    ) -> Result<&RealtimeLogConfigData, CloudFrontError> {
        let key = if let Some(n) = name {
            if !self.realtime_log_configs.contains_key(n) {
                return Err(CloudFrontError::NoSuchRealtimeLogConfig(n.to_string()));
            }
            n.to_string()
        } else if let Some(a) = arn {
            match self.realtime_log_configs.values().find(|c| c.arn == a) {
                Some(c) => c.name.clone(),
                None => {
                    return Err(CloudFrontError::NoSuchRealtimeLogConfig(a.to_string()));
                }
            }
        } else {
            return Err(CloudFrontError::InvalidArgument);
        };
        let cfg = self.realtime_log_configs.get_mut(&key).unwrap();
        cfg.sampling_rate = sampling_rate;
        cfg.end_points = end_points;
        cfg.fields = fields;
        Ok(self.realtime_log_configs.get(&key).unwrap())
    }

    pub fn delete_realtime_log_config(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let key = if let Some(n) = name {
            if !self.realtime_log_configs.contains_key(n) {
                return Err(CloudFrontError::NoSuchRealtimeLogConfig(n.to_string()));
            }
            n.to_string()
        } else if let Some(a) = arn {
            match self.realtime_log_configs.values().find(|c| c.arn == a) {
                Some(c) => c.name.clone(),
                None => {
                    return Err(CloudFrontError::NoSuchRealtimeLogConfig(a.to_string()));
                }
            }
        } else {
            return Err(CloudFrontError::InvalidArgument);
        };
        self.realtime_log_configs.remove(&key);
        Ok(())
    }

    pub fn list_realtime_log_configs(&self) -> Vec<&RealtimeLogConfigData> {
        self.realtime_log_configs.values().collect()
    }

    // ---- Field-Level Encryption Config operations ----

    pub fn create_field_level_encryption_config(
        &mut self,
        config: crate::model::FieldLevelEncryptionConfig,
    ) -> &FieldLevelEncryptionData {
        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let etag = uuid::Uuid::new_v4().to_string();
        let fle = FieldLevelEncryptionData {
            id: id.clone(),
            last_modified_time: Utc::now(),
            config,
            etag,
        };
        self.field_level_encryptions.insert(id.clone(), fle);
        self.field_level_encryptions.get(&id).unwrap()
    }

    pub fn get_field_level_encryption_config(
        &self,
        id: &str,
    ) -> Result<&FieldLevelEncryptionData, CloudFrontError> {
        self.field_level_encryptions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchFieldLevelEncryptionConfig(id.to_string()))
    }

    pub fn update_field_level_encryption_config(
        &mut self,
        id: &str,
        config: crate::model::FieldLevelEncryptionConfig,
        if_match: Option<&str>,
    ) -> Result<&FieldLevelEncryptionData, CloudFrontError> {
        let fle = self
            .field_level_encryptions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchFieldLevelEncryptionConfig(id.to_string()))?;
        if let Some(expected) = if_match
            && fle.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let fle = self.field_level_encryptions.get_mut(id).unwrap();
        fle.config = config;
        fle.last_modified_time = Utc::now();
        fle.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.field_level_encryptions.get(id).unwrap())
    }

    pub fn delete_field_level_encryption_config(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let fle = self
            .field_level_encryptions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchFieldLevelEncryptionConfig(id.to_string()))?;
        if let Some(expected) = if_match
            && fle.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        self.field_level_encryptions.remove(id);
        Ok(())
    }

    pub fn list_field_level_encryption_configs(&self) -> Vec<&FieldLevelEncryptionData> {
        self.field_level_encryptions.values().collect()
    }

    // ---- Field-Level Encryption Profile operations ----

    pub fn create_field_level_encryption_profile(
        &mut self,
        config: crate::model::FieldLevelEncryptionProfileConfig,
    ) -> &FieldLevelEncryptionProfileData {
        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let etag = uuid::Uuid::new_v4().to_string();
        let profile = FieldLevelEncryptionProfileData {
            id: id.clone(),
            last_modified_time: Utc::now(),
            config,
            etag,
        };
        self.field_level_encryption_profiles
            .insert(id.clone(), profile);
        self.field_level_encryption_profiles.get(&id).unwrap()
    }

    pub fn get_field_level_encryption_profile(
        &self,
        id: &str,
    ) -> Result<&FieldLevelEncryptionProfileData, CloudFrontError> {
        self.field_level_encryption_profiles
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchFieldLevelEncryptionProfile(id.to_string()))
    }

    pub fn update_field_level_encryption_profile(
        &mut self,
        id: &str,
        config: crate::model::FieldLevelEncryptionProfileConfig,
        if_match: Option<&str>,
    ) -> Result<&FieldLevelEncryptionProfileData, CloudFrontError> {
        let profile = self
            .field_level_encryption_profiles
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchFieldLevelEncryptionProfile(id.to_string()))?;
        if let Some(expected) = if_match
            && profile.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let profile = self.field_level_encryption_profiles.get_mut(id).unwrap();
        profile.config = config;
        profile.last_modified_time = Utc::now();
        profile.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.field_level_encryption_profiles.get(id).unwrap())
    }

    pub fn delete_field_level_encryption_profile(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        let profile = self
            .field_level_encryption_profiles
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchFieldLevelEncryptionProfile(id.to_string()))?;
        if let Some(expected) = if_match
            && profile.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        self.field_level_encryption_profiles.remove(id);
        Ok(())
    }

    pub fn list_field_level_encryption_profiles(&self) -> Vec<&FieldLevelEncryptionProfileData> {
        self.field_level_encryption_profiles.values().collect()
    }

    // ---- Connection Function operations ----

    pub fn create_connection_function(
        &mut self,
        name: &str,
        config: crate::model::FunctionConfig,
        code: &str,
        account_id: &str,
    ) -> &ConnectionFunctionData {
        let id = format!(
            "cf{}",
            &uuid::Uuid::new_v4().to_string()[..12]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:connection-function/{name}");
        let etag = uuid::Uuid::new_v4().to_string();
        let func = ConnectionFunctionData {
            id: id.clone(),
            name: name.to_string(),
            arn,
            status: "UNPUBLISHED".to_string(),
            stage: "DEVELOPMENT".to_string(),
            created_time: Utc::now(),
            last_modified_time: Utc::now(),
            config,
            code: code.to_string(),
            etag,
        };
        self.connection_functions.insert(id.clone(), func);
        self.connection_functions.get(&id).unwrap()
    }

    pub fn get_connection_function(
        &self,
        id: &str,
    ) -> Result<&ConnectionFunctionData, CloudFrontError> {
        self.connection_functions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchConnectionFunction(id.to_string()))
    }

    pub fn update_connection_function(
        &mut self,
        id: &str,
        config: crate::model::FunctionConfig,
        code: &str,
        if_match: Option<&str>,
    ) -> Result<&ConnectionFunctionData, CloudFrontError> {
        let func = self
            .connection_functions
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchConnectionFunction(id.to_string()))?;
        if let Some(expected) = if_match
            && func.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let func = self.connection_functions.get_mut(id).unwrap();
        func.config = config;
        func.code = code.to_string();
        func.last_modified_time = Utc::now();
        func.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.connection_functions.get(id).unwrap())
    }

    pub fn delete_connection_function(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(cf) = self.connection_functions.get(id)
            && cf.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.connection_functions.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchConnectionFunction(id.to_string()));
        }
        Ok(())
    }

    pub fn publish_connection_function(
        &mut self,
        id: &str,
    ) -> Result<&ConnectionFunctionData, CloudFrontError> {
        let func = self
            .connection_functions
            .get_mut(id)
            .ok_or_else(|| CloudFrontError::NoSuchConnectionFunction(id.to_string()))?;
        func.status = "DEPLOYED".to_string();
        func.stage = "LIVE".to_string();
        func.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.connection_functions.get(id).unwrap())
    }

    pub fn list_connection_functions(&self) -> Vec<&ConnectionFunctionData> {
        self.connection_functions.values().collect()
    }

    // ---- Connection Group operations ----

    pub fn create_connection_group(
        &mut self,
        name: &str,
        enabled: bool,
        ipv6_enabled: bool,
        anycast_ip_list_id: Option<String>,
        account_id: &str,
    ) -> &ConnectionGroupData {
        let id = format!(
            "cg{}",
            &uuid::Uuid::new_v4().to_string()[..12]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:connection-group/{id}");
        let routing_endpoint = format!("{id}.cloudfront.net").to_lowercase();
        let etag = uuid::Uuid::new_v4().to_string();
        let cg = ConnectionGroupData {
            id: id.clone(),
            name: name.to_string(),
            arn,
            status: "Deployed".to_string(),
            routing_endpoint,
            enabled,
            ipv6_enabled,
            is_default: false,
            anycast_ip_list_id,
            created_time: Utc::now(),
            last_modified_time: Utc::now(),
            etag,
        };
        self.connection_groups.insert(id.clone(), cg);
        self.connection_groups.get(&id).unwrap()
    }

    pub fn get_connection_group(&self, id: &str) -> Result<&ConnectionGroupData, CloudFrontError> {
        self.connection_groups
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchConnectionGroup(id.to_string()))
    }

    pub fn get_connection_group_by_routing_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<&ConnectionGroupData, CloudFrontError> {
        self.connection_groups
            .values()
            .find(|cg| cg.routing_endpoint == endpoint)
            .ok_or_else(|| CloudFrontError::NoSuchConnectionGroup(endpoint.to_string()))
    }

    pub fn update_connection_group(
        &mut self,
        id: &str,
        enabled: Option<bool>,
        ipv6_enabled: Option<bool>,
        anycast_ip_list_id: Option<String>,
        if_match: Option<&str>,
    ) -> Result<&ConnectionGroupData, CloudFrontError> {
        let cg = self
            .connection_groups
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchConnectionGroup(id.to_string()))?;
        if let Some(expected) = if_match
            && cg.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let cg = self.connection_groups.get_mut(id).unwrap();
        if let Some(e) = enabled {
            cg.enabled = e;
        }
        if let Some(e) = ipv6_enabled {
            cg.ipv6_enabled = e;
        }
        if anycast_ip_list_id.is_some() {
            cg.anycast_ip_list_id = anycast_ip_list_id;
        }
        cg.last_modified_time = Utc::now();
        cg.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.connection_groups.get(id).unwrap())
    }

    pub fn delete_connection_group(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(cg) = self.connection_groups.get(id)
            && cg.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.connection_groups.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchConnectionGroup(id.to_string()));
        }
        Ok(())
    }

    pub fn list_connection_groups(&self) -> Vec<&ConnectionGroupData> {
        self.connection_groups.values().collect()
    }

    // ---- Continuous Deployment Policy operations ----

    pub fn create_continuous_deployment_policy(
        &mut self,
        config: crate::model::ContinuousDeploymentPolicyConfig,
    ) -> &ContinuousDeploymentPolicyData {
        let id = format!(
            "E{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let etag = uuid::Uuid::new_v4().to_string();
        let cdp = ContinuousDeploymentPolicyData {
            id: id.clone(),
            last_modified_time: Utc::now(),
            config,
            etag,
        };
        self.continuous_deployment_policies.insert(id.clone(), cdp);
        self.continuous_deployment_policies.get(&id).unwrap()
    }

    pub fn get_continuous_deployment_policy(
        &self,
        id: &str,
    ) -> Result<&ContinuousDeploymentPolicyData, CloudFrontError> {
        self.continuous_deployment_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchContinuousDeploymentPolicy(id.to_string()))
    }

    pub fn update_continuous_deployment_policy(
        &mut self,
        id: &str,
        config: crate::model::ContinuousDeploymentPolicyConfig,
        if_match: Option<&str>,
    ) -> Result<&ContinuousDeploymentPolicyData, CloudFrontError> {
        let cdp = self
            .continuous_deployment_policies
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchContinuousDeploymentPolicy(id.to_string()))?;
        if let Some(expected) = if_match
            && cdp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let cdp = self.continuous_deployment_policies.get_mut(id).unwrap();
        cdp.config = config;
        cdp.last_modified_time = Utc::now();
        cdp.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.continuous_deployment_policies.get(id).unwrap())
    }

    pub fn delete_continuous_deployment_policy(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(cdp) = self.continuous_deployment_policies.get(id)
            && cdp.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.continuous_deployment_policies.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchContinuousDeploymentPolicy(
                id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_continuous_deployment_policies(&self) -> Vec<&ContinuousDeploymentPolicyData> {
        self.continuous_deployment_policies.values().collect()
    }

    // ---- Distribution Tenant operations ----

    pub fn create_distribution_tenant(
        &mut self,
        name: &str,
        distribution_id: &str,
        connection_group_id: Option<String>,
        enabled: bool,
        domains: crate::model::DomainResultList,
        customizations: Option<crate::model::Customizations>,
        parameters: Option<crate::model::Parameters>,
        account_id: &str,
    ) -> &DistributionTenantData {
        let id = format!(
            "dt{}",
            &uuid::Uuid::new_v4().to_string()[..12]
                .to_uppercase()
                .replace('-', "")
        );
        let arn = format!("arn:aws:cloudfront::{account_id}:distribution-tenant/{id}");
        let etag = uuid::Uuid::new_v4().to_string();
        let dt = DistributionTenantData {
            id: id.clone(),
            name: name.to_string(),
            arn,
            distribution_id: distribution_id.to_string(),
            connection_group_id,
            enabled,
            status: "Deployed".to_string(),
            domains,
            customizations,
            parameters,
            created_time: Utc::now(),
            last_modified_time: Utc::now(),
            etag,
        };
        self.distribution_tenants.insert(id.clone(), dt);
        self.distribution_tenants.get(&id).unwrap()
    }

    pub fn get_distribution_tenant(
        &self,
        id: &str,
    ) -> Result<&DistributionTenantData, CloudFrontError> {
        self.distribution_tenants
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchDistributionTenant(id.to_string()))
    }

    pub fn get_distribution_tenant_by_domain(
        &self,
        domain: &str,
    ) -> Result<&DistributionTenantData, CloudFrontError> {
        self.distribution_tenants
            .values()
            .find(|dt| {
                dt.domains
                    .items
                    .iter()
                    .any(|d| d.domain.as_deref() == Some(domain))
            })
            .ok_or_else(|| CloudFrontError::NoSuchDistributionTenant(domain.to_string()))
    }

    pub fn update_distribution_tenant(
        &mut self,
        id: &str,
        connection_group_id: Option<String>,
        enabled: Option<bool>,
        domains: Option<crate::model::DomainList>,
        customizations: Option<crate::model::Customizations>,
        if_match: Option<&str>,
    ) -> Result<&DistributionTenantData, CloudFrontError> {
        let dt = self
            .distribution_tenants
            .get(id)
            .ok_or_else(|| CloudFrontError::NoSuchDistributionTenant(id.to_string()))?;
        if let Some(expected) = if_match
            && dt.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        let dt = self.distribution_tenants.get_mut(id).unwrap();
        if let Some(cg) = connection_group_id {
            dt.connection_group_id = Some(cg);
        }
        if let Some(e) = enabled {
            dt.enabled = e;
        }
        if let Some(d) = domains {
            // Convert DomainList to DomainResultList
            dt.domains = crate::model::DomainResultList {
                items: d
                    .items
                    .iter()
                    .map(|item| crate::model::DomainResult {
                        domain: Some(item.domain.clone()),
                        status: Some("active".to_string()),
                    })
                    .collect(),
            };
        }
        if customizations.is_some() {
            dt.customizations = customizations;
        }
        dt.last_modified_time = Utc::now();
        dt.etag = uuid::Uuid::new_v4().to_string();
        Ok(self.distribution_tenants.get(id).unwrap())
    }

    pub fn delete_distribution_tenant(
        &mut self,
        id: &str,
        if_match: Option<&str>,
    ) -> Result<(), CloudFrontError> {
        if let Some(expected) = if_match
            && let Some(dt) = self.distribution_tenants.get(id)
            && dt.etag != expected
        {
            return Err(CloudFrontError::PreconditionFailed);
        }
        if self.distribution_tenants.remove(id).is_none() {
            return Err(CloudFrontError::NoSuchDistributionTenant(id.to_string()));
        }
        Ok(())
    }

    pub fn list_distribution_tenants(&self) -> Vec<&DistributionTenantData> {
        self.distribution_tenants.values().collect()
    }

    // ---- Monitoring Subscription operations ----

    pub fn create_monitoring_subscription(
        &mut self,
        distribution_id: &str,
        status: &str,
    ) -> Result<&MonitoringSubscriptionData, CloudFrontError> {
        if !self.distributions.contains_key(distribution_id) {
            return Err(CloudFrontError::NoSuchDistribution(
                distribution_id.to_string(),
            ));
        }
        let ms = MonitoringSubscriptionData {
            distribution_id: distribution_id.to_string(),
            realtime_metrics_subscription_status: status.to_string(),
        };
        self.monitoring_subscriptions
            .insert(distribution_id.to_string(), ms);
        Ok(self.monitoring_subscriptions.get(distribution_id).unwrap())
    }

    pub fn get_monitoring_subscription(
        &self,
        distribution_id: &str,
    ) -> Result<&MonitoringSubscriptionData, CloudFrontError> {
        self.monitoring_subscriptions
            .get(distribution_id)
            .ok_or_else(|| {
                CloudFrontError::NoSuchMonitoringSubscription(distribution_id.to_string())
            })
    }

    pub fn delete_monitoring_subscription(
        &mut self,
        distribution_id: &str,
    ) -> Result<(), CloudFrontError> {
        if self
            .monitoring_subscriptions
            .remove(distribution_id)
            .is_none()
        {
            return Err(CloudFrontError::NoSuchMonitoringSubscription(
                distribution_id.to_string(),
            ));
        }
        Ok(())
    }

    // ---- Resource Policy operations ----

    pub fn put_resource_policy(
        &mut self,
        resource_arn: &str,
        policy_document: &str,
    ) -> &ResourcePolicyData {
        let rp = ResourcePolicyData {
            resource_arn: resource_arn.to_string(),
            policy_document: policy_document.to_string(),
        };
        self.resource_policies.insert(resource_arn.to_string(), rp);
        self.resource_policies.get(resource_arn).unwrap()
    }

    pub fn get_resource_policy(
        &self,
        resource_arn: &str,
    ) -> Result<&ResourcePolicyData, CloudFrontError> {
        self.resource_policies.get(resource_arn).ok_or_else(|| {
            CloudFrontError::NoSuchResource(format!(
                "Resource policy not found for: {resource_arn}"
            ))
        })
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) -> Result<(), CloudFrontError> {
        if self.resource_policies.remove(resource_arn).is_none() {
            return Err(CloudFrontError::NoSuchResource(format!(
                "Resource policy not found for: {resource_arn}"
            )));
        }
        Ok(())
    }

    // ---- WebACL Association operations ----

    pub fn associate_web_acl(
        &mut self,
        resource_id: &str,
        web_acl_arn: &str,
    ) -> &WebAclAssociation {
        let assoc = WebAclAssociation {
            resource_id: resource_id.to_string(),
            web_acl_arn: web_acl_arn.to_string(),
        };
        self.web_acl_associations
            .insert(resource_id.to_string(), assoc);
        self.web_acl_associations.get(resource_id).unwrap()
    }

    pub fn disassociate_web_acl(&mut self, resource_id: &str) {
        self.web_acl_associations.remove(resource_id);
    }

    pub fn get_web_acl_association(&self, resource_id: &str) -> Option<&WebAclAssociation> {
        self.web_acl_associations.get(resource_id)
    }

    pub fn list_distributions_by_web_acl_id(&self, web_acl_id: &str) -> Vec<&str> {
        self.web_acl_associations
            .values()
            .filter(|a| a.web_acl_arn == web_acl_id)
            .map(|a| a.resource_id.as_str())
            .collect()
    }

    // ---- Distribution Tenant Invalidation operations ----

    pub fn create_invalidation_for_distribution_tenant(
        &mut self,
        tenant_id: &str,
        caller_reference: &str,
        paths: Vec<String>,
    ) -> Result<&Invalidation, CloudFrontError> {
        if !self.distribution_tenants.contains_key(tenant_id) {
            return Err(CloudFrontError::NoSuchDistributionTenant(
                tenant_id.to_string(),
            ));
        }
        let id = format!(
            "I{}",
            &uuid::Uuid::new_v4().to_string()[..13]
                .to_uppercase()
                .replace('-', "")
        );
        let inv = Invalidation {
            id: id.clone(),
            distribution_id: tenant_id.to_string(),
            status: "COMPLETED".to_string(),
            create_time: Utc::now(),
            caller_reference: caller_reference.to_string(),
            paths,
        };
        let invs = self
            .invalidations
            .entry(format!("tenant:{tenant_id}"))
            .or_default();
        invs.push(inv);
        Ok(invs.last().unwrap())
    }

    pub fn get_invalidation_for_distribution_tenant(
        &self,
        tenant_id: &str,
        invalidation_id: &str,
    ) -> Result<&Invalidation, CloudFrontError> {
        if !self.distribution_tenants.contains_key(tenant_id) {
            return Err(CloudFrontError::NoSuchDistributionTenant(
                tenant_id.to_string(),
            ));
        }
        self.invalidations
            .get(&format!("tenant:{tenant_id}"))
            .and_then(|invs| invs.iter().find(|i| i.id == invalidation_id))
            .ok_or_else(|| CloudFrontError::NoSuchInvalidation(invalidation_id.to_string()))
    }

    pub fn list_invalidations_for_distribution_tenant(
        &self,
        tenant_id: &str,
    ) -> Vec<&Invalidation> {
        self.invalidations
            .get(&format!("tenant:{tenant_id}"))
            .map(|invs| invs.iter().collect())
            .unwrap_or_default()
    }
}
