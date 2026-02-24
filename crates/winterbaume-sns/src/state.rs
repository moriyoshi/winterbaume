use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Error)]
pub enum SnsError {
    #[error("Topic does not exist")]
    TopicNotFound,
    #[error("Subscription does not exist")]
    SubscriptionNotFound,
    #[error("PlatformApplication does not exist")]
    PlatformApplicationNotFound,
    #[error("Endpoint does not exist")]
    EndpointNotFound,
    #[error("Phone number already exists in sandbox")]
    PhoneNumberAlreadyInSandbox,
    #[error("Phone number not found in sandbox")]
    SandboxPhoneNumberNotFound,
    #[error("{0}")]
    ResourceNotFound(String),
}

#[derive(Debug, Default)]
pub struct SnsState {
    pub topics: HashMap<String, Topic>,
    pub subscriptions: HashMap<String, Subscription>,
    /// Global SMS attributes (e.g., DefaultSenderID, DefaultSMSType, etc.)
    pub sms_attributes: HashMap<String, String>,
    /// Platform applications keyed by ARN.
    pub platform_applications: HashMap<String, PlatformApplicationState>,
    /// Platform endpoints keyed by ARN.
    pub platform_endpoints: HashMap<String, PlatformEndpointState>,
    /// SMS sandbox phone numbers.
    pub sms_sandbox_phone_numbers: Vec<SmsSandboxPhoneNumber>,
    /// Whether the account is in the SMS sandbox (default: true).
    pub sms_sandbox_enabled: bool,
    /// Phone numbers that have opted out of SMS.
    pub opted_out_phone_numbers: Vec<String>,
}

impl SnsState {
    pub fn create_topic(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Topic, SnsError> {
        let arn = format!("arn:aws:sns:{region}:{account_id}:{name}");

        // Idempotent: return existing if same name
        if self.topics.contains_key(&arn) {
            return Ok(self.topics.get(&arn).unwrap());
        }

        let mut attributes = HashMap::new();
        attributes.insert("TopicArn".to_string(), arn.clone());
        attributes.insert("DisplayName".to_string(), name.to_string());
        // FIX(terraform-e2e): Added Owner, Policy, and EffectiveDeliveryPolicy attributes.
        // Terraform's aws_sns_topic resource reads GetTopicAttributes after create and
        // parses the Policy attribute as JSON. Without a valid default, it fails with
        // "parsing policy: unexpected end of JSON input".
        attributes.insert("Owner".to_string(), account_id.to_string());
        attributes.insert(
            "Policy".to_string(),
            serde_json::json!({
                "Version": "2008-10-17",
                "Id": "__default_policy_ID",
                "Statement": [{
                    "Sid": "__default_statement_ID",
                    "Effect": "Allow",
                    "Principal": {"AWS": "*"},
                    "Action": [
                        "SNS:GetTopicAttributes",
                        "SNS:SetTopicAttributes",
                        "SNS:AddPermission",
                        "SNS:RemovePermission",
                        "SNS:DeleteTopic",
                        "SNS:Subscribe",
                        "SNS:ListSubscriptionsByTopic",
                        "SNS:Publish"
                    ],
                    "Resource": &arn,
                    "Condition": {
                        "StringEquals": {"AWS:SourceOwner": account_id}
                    }
                }]
            })
            .to_string(),
        );
        attributes.insert(
            "EffectiveDeliveryPolicy".to_string(),
            serde_json::json!({"http":{"defaultHealthyRetryPolicy":{"minDelayTarget":20,"maxDelayTarget":20,"numRetries":3,"numMaxDelayRetries":0,"numNoDelayRetries":0,"numMinDelayRetries":0,"backoffFunction":"linear"},"disableSubscriptionOverrides":false}}).to_string(),
        );

        let topic = Topic {
            arn: arn.clone(),
            name: name.to_string(),
            attributes,
            tags,
            permissions: HashMap::new(),
            data_protection_policy: None,
        };

        self.topics.insert(arn.clone(), topic);
        Ok(self.topics.get(&arn).unwrap())
    }

    pub fn delete_topic(&mut self, arn: &str) -> Result<(), SnsError> {
        self.topics.remove(arn);
        // Also remove subscriptions for this topic
        self.subscriptions.retain(|_, s| s.topic_arn != arn);
        Ok(())
    }

    pub fn list_topics(&self) -> Vec<&Topic> {
        self.topics.values().collect()
    }

    pub fn get_topic_attributes(&self, arn: &str) -> Result<&Topic, SnsError> {
        self.topics.get(arn).ok_or(SnsError::TopicNotFound)
    }

    pub fn subscribe(
        &mut self,
        topic_arn: &str,
        protocol: &str,
        endpoint: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Subscription, SnsError> {
        if !self.topics.contains_key(topic_arn) {
            return Err(SnsError::TopicNotFound);
        }

        let sub_id = uuid::Uuid::new_v4().to_string();
        // Extract topic name from topic ARN (last segment after ':')
        let topic_name = topic_arn.rsplit(':').next().unwrap_or("unknown");
        let sub_arn = format!("arn:aws:sns:{region}:{account_id}:{topic_name}:{sub_id}");

        let mut attributes = HashMap::new();
        attributes.insert("SubscriptionArn".to_string(), sub_arn.clone());
        attributes.insert("TopicArn".to_string(), topic_arn.to_string());
        attributes.insert("Protocol".to_string(), protocol.to_string());
        attributes.insert("Endpoint".to_string(), endpoint.to_string());
        attributes.insert("Owner".to_string(), account_id.to_string());
        attributes.insert(
            "ConfirmationWasAuthenticated".to_string(),
            "true".to_string(),
        );
        attributes.insert("PendingConfirmation".to_string(), "false".to_string());
        attributes.insert("RawMessageDelivery".to_string(), "false".to_string());

        let subscription = Subscription {
            arn: sub_arn.clone(),
            topic_arn: topic_arn.to_string(),
            protocol: protocol.to_string(),
            endpoint: endpoint.to_string(),
            confirmed: true, // Auto-confirm for mock
            owner: account_id.to_string(),
            attributes,
        };

        self.subscriptions.insert(sub_arn.clone(), subscription);
        Ok(self.subscriptions.get(&sub_arn).unwrap())
    }

    pub fn unsubscribe(&mut self, subscription_arn: &str) -> Result<(), SnsError> {
        if self.subscriptions.remove(subscription_arn).is_none() {
            return Err(SnsError::SubscriptionNotFound);
        }
        Ok(())
    }

    pub fn list_subscriptions(&self) -> Vec<&Subscription> {
        self.subscriptions.values().collect()
    }

    pub fn publish(&self, topic_arn: &str, _message: &str) -> Result<String, SnsError> {
        if !self.topics.contains_key(topic_arn) {
            return Err(SnsError::TopicNotFound);
        }
        Ok(uuid::Uuid::new_v4().to_string())
    }

    pub fn publish_batch(
        &self,
        topic_arn: &str,
        messages: &[(&str, &str)], // (id, message)
    ) -> Result<Vec<(String, String)>, SnsError> {
        // Returns list of (message_id, batch_result_id)
        if !self.topics.contains_key(topic_arn) {
            return Err(SnsError::TopicNotFound);
        }
        let results = messages
            .iter()
            .map(|(id, _msg)| (id.to_string(), uuid::Uuid::new_v4().to_string()))
            .collect();
        Ok(results)
    }

    pub fn add_permission(
        &mut self,
        topic_arn: &str,
        label: &str,
        aws_account_ids: Vec<String>,
        action_names: Vec<String>,
    ) -> Result<(), SnsError> {
        let topic = self
            .topics
            .get_mut(topic_arn)
            .ok_or(SnsError::TopicNotFound)?;
        topic.permissions.insert(
            label.to_string(),
            Permission {
                label: label.to_string(),
                aws_account_ids,
                action_names,
            },
        );
        Ok(())
    }

    pub fn remove_permission(&mut self, topic_arn: &str, label: &str) -> Result<(), SnsError> {
        let topic = self
            .topics
            .get_mut(topic_arn)
            .ok_or(SnsError::TopicNotFound)?;
        topic.permissions.remove(label);
        Ok(())
    }

    pub fn get_sms_attributes(&self) -> &HashMap<String, String> {
        &self.sms_attributes
    }

    pub fn set_sms_attributes(&mut self, attributes: HashMap<String, String>) {
        self.sms_attributes.extend(attributes);
    }

    // --- Subscription attributes ---

    pub fn confirm_subscription(
        &mut self,
        topic_arn: &str,
        _token: &str,
    ) -> Result<String, SnsError> {
        if !self.topics.contains_key(topic_arn) {
            return Err(SnsError::TopicNotFound);
        }
        // For mock, find an unconfirmed subscription for this topic, or just return a fake ARN
        for sub in self.subscriptions.values_mut() {
            if sub.topic_arn == topic_arn && !sub.confirmed {
                sub.confirmed = true;
                sub.attributes
                    .insert("PendingConfirmation".to_string(), "false".to_string());
                return Ok(sub.arn.clone());
            }
        }
        // If no unconfirmed subscription found, return a fake confirmed ARN
        // (AWS behavior: ConfirmSubscription is idempotent)
        Ok(format!(
            "arn:aws:sns:us-east-1:000000000000:confirmed:{}",
            uuid::Uuid::new_v4()
        ))
    }

    pub fn get_subscription_attributes(
        &self,
        subscription_arn: &str,
    ) -> Result<&HashMap<String, String>, SnsError> {
        let sub = self
            .subscriptions
            .get(subscription_arn)
            .ok_or(SnsError::SubscriptionNotFound)?;
        Ok(&sub.attributes)
    }

    pub fn set_subscription_attributes(
        &mut self,
        subscription_arn: &str,
        attribute_name: &str,
        attribute_value: &str,
    ) -> Result<(), SnsError> {
        let sub = self
            .subscriptions
            .get_mut(subscription_arn)
            .ok_or(SnsError::SubscriptionNotFound)?;
        sub.attributes
            .insert(attribute_name.to_string(), attribute_value.to_string());
        Ok(())
    }

    pub fn list_subscriptions_by_topic(
        &self,
        topic_arn: &str,
    ) -> Result<Vec<&Subscription>, SnsError> {
        if !self.topics.contains_key(topic_arn) {
            return Err(SnsError::TopicNotFound);
        }
        Ok(self
            .subscriptions
            .values()
            .filter(|s| s.topic_arn == topic_arn)
            .collect())
    }

    // --- Data protection policy ---

    pub fn get_data_protection_policy(&self, topic_arn: &str) -> Result<Option<&str>, SnsError> {
        let topic = self.topics.get(topic_arn).ok_or(SnsError::TopicNotFound)?;
        Ok(topic.data_protection_policy.as_deref())
    }

    pub fn put_data_protection_policy(
        &mut self,
        topic_arn: &str,
        policy: &str,
    ) -> Result<(), SnsError> {
        let topic = self
            .topics
            .get_mut(topic_arn)
            .ok_or(SnsError::TopicNotFound)?;
        topic.data_protection_policy = Some(policy.to_string());
        Ok(())
    }

    // --- Platform applications ---

    pub fn create_platform_application(
        &mut self,
        name: &str,
        platform: &str,
        attributes: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> &PlatformApplicationState {
        let arn = format!("arn:aws:sns:{region}:{account_id}:app/{platform}/{name}");
        let app = PlatformApplicationState {
            arn: arn.clone(),
            name: name.to_string(),
            platform: platform.to_string(),
            attributes,
        };
        self.platform_applications.insert(arn.clone(), app);
        self.platform_applications.get(&arn).unwrap()
    }

    pub fn delete_platform_application(&mut self, arn: &str) -> Result<(), SnsError> {
        self.platform_applications.remove(arn);
        // Also remove endpoints for this application
        self.platform_endpoints
            .retain(|_, e| e.platform_application_arn != arn);
        Ok(())
    }

    pub fn get_platform_application_attributes(
        &self,
        arn: &str,
    ) -> Result<&PlatformApplicationState, SnsError> {
        self.platform_applications
            .get(arn)
            .ok_or(SnsError::PlatformApplicationNotFound)
    }

    pub fn set_platform_application_attributes(
        &mut self,
        arn: &str,
        attributes: HashMap<String, String>,
    ) -> Result<(), SnsError> {
        let app = self
            .platform_applications
            .get_mut(arn)
            .ok_or(SnsError::PlatformApplicationNotFound)?;
        app.attributes.extend(attributes);
        Ok(())
    }

    pub fn list_platform_applications(&self) -> Vec<&PlatformApplicationState> {
        self.platform_applications.values().collect()
    }

    // --- Platform endpoints ---

    pub fn create_platform_endpoint(
        &mut self,
        platform_application_arn: &str,
        token: &str,
        custom_user_data: Option<&str>,
        attributes: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&PlatformEndpointState, SnsError> {
        if !self
            .platform_applications
            .contains_key(platform_application_arn)
        {
            return Err(SnsError::PlatformApplicationNotFound);
        }
        let endpoint_id = uuid::Uuid::new_v4();
        let arn = format!(
            "arn:aws:sns:{region}:{account_id}:endpoint/{}/{}",
            platform_application_arn.rsplit('/').next().unwrap_or("app"),
            endpoint_id
        );

        let mut all_attrs = attributes;
        all_attrs.insert("Enabled".to_string(), "true".to_string());
        all_attrs.insert("Token".to_string(), token.to_string());
        if let Some(cud) = custom_user_data {
            all_attrs.insert("CustomUserData".to_string(), cud.to_string());
        }

        let endpoint = PlatformEndpointState {
            arn: arn.clone(),
            platform_application_arn: platform_application_arn.to_string(),
            token: token.to_string(),
            enabled: true,
            custom_user_data: custom_user_data.map(|s| s.to_string()),
            attributes: all_attrs,
        };
        self.platform_endpoints.insert(arn.clone(), endpoint);
        Ok(self.platform_endpoints.get(&arn).unwrap())
    }

    pub fn delete_endpoint(&mut self, arn: &str) -> Result<(), SnsError> {
        self.platform_endpoints.remove(arn);
        Ok(())
    }

    pub fn get_endpoint_attributes(&self, arn: &str) -> Result<&PlatformEndpointState, SnsError> {
        self.platform_endpoints
            .get(arn)
            .ok_or(SnsError::EndpointNotFound)
    }

    pub fn set_endpoint_attributes(
        &mut self,
        arn: &str,
        attributes: HashMap<String, String>,
    ) -> Result<(), SnsError> {
        let endpoint = self
            .platform_endpoints
            .get_mut(arn)
            .ok_or(SnsError::EndpointNotFound)?;
        if let Some(token) = attributes.get("Token") {
            endpoint.token = token.clone();
        }
        if let Some(enabled) = attributes.get("Enabled") {
            endpoint.enabled = enabled == "true";
        }
        if let Some(cud) = attributes.get("CustomUserData") {
            endpoint.custom_user_data = Some(cud.clone());
        }
        endpoint.attributes.extend(attributes);
        Ok(())
    }

    pub fn list_endpoints_by_platform_application(
        &self,
        platform_application_arn: &str,
    ) -> Result<Vec<&PlatformEndpointState>, SnsError> {
        if !self
            .platform_applications
            .contains_key(platform_application_arn)
        {
            return Err(SnsError::PlatformApplicationNotFound);
        }
        Ok(self
            .platform_endpoints
            .values()
            .filter(|e| e.platform_application_arn == platform_application_arn)
            .collect())
    }

    // --- SMS sandbox ---

    pub fn create_sms_sandbox_phone_number(&mut self, phone_number: &str) -> Result<(), SnsError> {
        // Check for duplicate
        if self
            .sms_sandbox_phone_numbers
            .iter()
            .any(|p| p.phone_number == phone_number)
        {
            return Err(SnsError::PhoneNumberAlreadyInSandbox);
        }
        self.sms_sandbox_phone_numbers.push(SmsSandboxPhoneNumber {
            phone_number: phone_number.to_string(),
            status: "Pending".to_string(),
        });
        Ok(())
    }

    pub fn delete_sms_sandbox_phone_number(&mut self, phone_number: &str) -> Result<(), SnsError> {
        let len_before = self.sms_sandbox_phone_numbers.len();
        self.sms_sandbox_phone_numbers
            .retain(|p| p.phone_number != phone_number);
        if self.sms_sandbox_phone_numbers.len() == len_before {
            return Err(SnsError::SandboxPhoneNumberNotFound);
        }
        Ok(())
    }

    pub fn verify_sms_sandbox_phone_number(
        &mut self,
        phone_number: &str,
        _one_time_password: &str,
    ) -> Result<(), SnsError> {
        let entry = self
            .sms_sandbox_phone_numbers
            .iter_mut()
            .find(|p| p.phone_number == phone_number)
            .ok_or(SnsError::SandboxPhoneNumberNotFound)?;
        entry.status = "Verified".to_string();
        Ok(())
    }

    pub fn list_sms_sandbox_phone_numbers(&self) -> &[SmsSandboxPhoneNumber] {
        &self.sms_sandbox_phone_numbers
    }

    pub fn get_sms_sandbox_account_status(&self) -> bool {
        self.sms_sandbox_enabled
    }

    // --- SMS opt-out ---

    pub fn check_if_phone_number_is_opted_out(&self, phone_number: &str) -> bool {
        self.opted_out_phone_numbers
            .contains(&phone_number.to_string())
    }

    pub fn list_phone_numbers_opted_out(&self) -> &[String] {
        &self.opted_out_phone_numbers
    }

    pub fn opt_in_phone_number(&mut self, phone_number: &str) -> Result<(), SnsError> {
        self.opted_out_phone_numbers.retain(|p| p != phone_number);
        Ok(())
    }
}
