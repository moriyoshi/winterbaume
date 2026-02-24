use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct MqState {
    pub brokers: HashMap<String, Broker>,
    pub configurations: HashMap<String, MqConfiguration>,
}

#[derive(Debug, thiserror::Error)]
pub enum MqError {
    #[error("The broker name '{broker_name}' is already in use by broker '{existing_broker_id}'.")]
    BrokerNameConflict {
        broker_name: String,
        existing_broker_id: String,
    },
    #[error("Can't find requested broker [{broker_id}]. Make sure your broker exists.")]
    BrokerNotFound { broker_id: String },
    #[error("Can't find requested configuration [{config_id}].")]
    ConfigurationNotFound { config_id: String },
    #[error("Can't find revision [{revision}] for configuration [{config_id}].")]
    ConfigurationRevisionNotFound { revision: i32, config_id: String },
    #[error("Can't find requested resource [{resource_arn}].")]
    ResourceNotFound { resource_arn: String },
    #[error("The user '{username}' already exists for broker [{broker_id}].")]
    UserConflict { username: String, broker_id: String },
    #[error("Can't find requested user [{username}] for broker [{broker_id}].")]
    UserNotFound { username: String, broker_id: String },
    #[error("Can't find requested user [{username}].")]
    UserNotFoundShort { username: String },
}

impl MqState {
    pub fn create_broker(
        &mut self,
        broker_name: &str,
        engine_type: &str,
        engine_version: &str,
        host_instance_type: &str,
        deployment_mode: &str,
        publicly_accessible: bool,
        auto_minor_version_upgrade: bool,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&Broker, MqError> {
        // Check for duplicate broker name
        for broker in self.brokers.values() {
            if broker.broker_name == broker_name {
                return Err(MqError::BrokerNameConflict {
                    broker_name: broker_name.to_string(),
                    existing_broker_id: broker.broker_id.clone(),
                });
            }
        }

        let broker_id = uuid::Uuid::new_v4().to_string();
        let broker_arn =
            format!("arn:aws:mq:{region}:{account_id}:broker:{broker_name}:{broker_id}");

        let broker = Broker {
            broker_id: broker_id.clone(),
            broker_name: broker_name.to_string(),
            broker_arn,
            broker_state: "RUNNING".to_string(),
            engine_type: engine_type.to_string(),
            engine_version: engine_version.to_string(),
            host_instance_type: host_instance_type.to_string(),
            deployment_mode: deployment_mode.to_string(),
            created: Utc::now(),
            publicly_accessible,
            auto_minor_version_upgrade,
            tags: tags.unwrap_or_default(),
            users: HashMap::new(),
        };

        self.brokers.insert(broker_id.clone(), broker);
        Ok(self.brokers.get(&broker_id).unwrap())
    }

    pub fn describe_broker(&self, broker_id: &str) -> Result<&Broker, MqError> {
        self.brokers
            .get(broker_id)
            .ok_or_else(|| MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            })
    }

    pub fn delete_broker(&mut self, broker_id: &str) -> Result<String, MqError> {
        match self.brokers.remove(broker_id) {
            Some(_) => Ok(broker_id.to_string()),
            None => Err(MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            }),
        }
    }

    pub fn list_brokers(&self) -> Vec<BrokerSummary> {
        self.brokers
            .values()
            .map(|b| BrokerSummary {
                broker_id: b.broker_id.clone(),
                broker_name: b.broker_name.clone(),
                broker_arn: b.broker_arn.clone(),
                broker_state: b.broker_state.clone(),
                engine_type: b.engine_type.clone(),
                deployment_mode: b.deployment_mode.clone(),
                created: b.created,
                host_instance_type: b.host_instance_type.clone(),
            })
            .collect()
    }

    // --- Configuration operations ---

    pub fn create_configuration(
        &mut self,
        name: &str,
        engine_type: &str,
        engine_version: &str,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&MqConfiguration, MqError> {
        let config_id = format!(
            "c-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..20]
        );
        let config_arn = format!("arn:aws:mq:{region}:{account_id}:configuration:{config_id}");

        let now = Utc::now();
        let initial_revision = MqConfigurationRevision {
            revision: 1,
            created: now,
            description: String::new(),
            data: String::new(),
        };

        let config = MqConfiguration {
            id: config_id.clone(),
            arn: config_arn,
            name: name.to_string(),
            description: String::new(),
            engine_type: engine_type.to_string(),
            engine_version: engine_version.to_string(),
            authentication_strategy: "SIMPLE".to_string(),
            created: now,
            tags: tags.unwrap_or_default(),
            revisions: vec![initial_revision],
        };

        self.configurations.insert(config_id.clone(), config);
        Ok(self.configurations.get(&config_id).unwrap())
    }

    pub fn describe_configuration(&self, config_id: &str) -> Result<&MqConfiguration, MqError> {
        self.configurations
            .get(config_id)
            .ok_or_else(|| MqError::ConfigurationNotFound {
                config_id: config_id.to_string(),
            })
    }

    pub fn describe_configuration_revision(
        &self,
        config_id: &str,
        revision: i32,
    ) -> Result<(&MqConfiguration, &MqConfigurationRevision), MqError> {
        let config = self.describe_configuration(config_id)?;
        let rev = config
            .revisions
            .iter()
            .find(|r| r.revision == revision)
            .ok_or_else(|| MqError::ConfigurationRevisionNotFound {
                revision,
                config_id: config_id.to_string(),
            })?;
        Ok((config, rev))
    }

    pub fn update_configuration(
        &mut self,
        config_id: &str,
        data: &str,
        description: Option<&str>,
    ) -> Result<&MqConfiguration, MqError> {
        let config = self.configurations.get_mut(config_id).ok_or_else(|| {
            MqError::ConfigurationNotFound {
                config_id: config_id.to_string(),
            }
        })?;

        let next_revision = config
            .revisions
            .iter()
            .map(|r| r.revision)
            .max()
            .unwrap_or(0)
            + 1;
        let rev = MqConfigurationRevision {
            revision: next_revision,
            created: Utc::now(),
            description: description.unwrap_or("").to_string(),
            data: data.to_string(),
        };
        config.revisions.push(rev);

        Ok(config)
    }

    pub fn delete_configuration(&mut self, config_id: &str) -> Result<String, MqError> {
        match self.configurations.remove(config_id) {
            Some(_) => Ok(config_id.to_string()),
            None => Err(MqError::ConfigurationNotFound {
                config_id: config_id.to_string(),
            }),
        }
    }

    pub fn list_configurations(&self) -> Vec<&MqConfiguration> {
        self.configurations.values().collect()
    }

    pub fn list_configuration_revisions(
        &self,
        config_id: &str,
    ) -> Result<&MqConfiguration, MqError> {
        self.describe_configuration(config_id)
    }

    // --- Tag operations ---

    pub fn create_tags(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), MqError> {
        // Check brokers
        for broker in self.brokers.values_mut() {
            if broker.broker_arn == resource_arn {
                broker.tags.extend(tags);
                return Ok(());
            }
        }
        // Check configurations
        for config in self.configurations.values_mut() {
            if config.arn == resource_arn {
                config.tags.extend(tags);
                return Ok(());
            }
        }
        Err(MqError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn delete_tags(&mut self, resource_arn: &str, tag_keys: &[String]) -> Result<(), MqError> {
        // Check brokers
        for broker in self.brokers.values_mut() {
            if broker.broker_arn == resource_arn {
                for key in tag_keys {
                    broker.tags.remove(key);
                }
                return Ok(());
            }
        }
        // Check configurations
        for config in self.configurations.values_mut() {
            if config.arn == resource_arn {
                for key in tag_keys {
                    config.tags.remove(key);
                }
                return Ok(());
            }
        }
        Err(MqError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn list_tags(&self, resource_arn: &str) -> Result<HashMap<String, String>, MqError> {
        // Check brokers
        for broker in self.brokers.values() {
            if broker.broker_arn == resource_arn {
                return Ok(broker.tags.clone());
            }
        }
        // Check configurations
        for config in self.configurations.values() {
            if config.arn == resource_arn {
                return Ok(config.tags.clone());
            }
        }
        Err(MqError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    // --- User operations ---

    pub fn create_user(
        &mut self,
        broker_id: &str,
        username: &str,
        console_access: bool,
        groups: Vec<String>,
    ) -> Result<(), MqError> {
        let broker = self
            .brokers
            .get_mut(broker_id)
            .ok_or_else(|| MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            })?;

        if broker.users.contains_key(username) {
            return Err(MqError::UserConflict {
                username: username.to_string(),
                broker_id: broker_id.to_string(),
            });
        }

        let user = MqUser {
            username: username.to_string(),
            console_access,
            groups,
            replication_user: false,
        };
        broker.users.insert(username.to_string(), user);
        Ok(())
    }

    pub fn delete_user(&mut self, broker_id: &str, username: &str) -> Result<(), MqError> {
        let broker = self
            .brokers
            .get_mut(broker_id)
            .ok_or_else(|| MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            })?;

        match broker.users.remove(username) {
            Some(_) => Ok(()),
            None => Err(MqError::UserNotFound {
                username: username.to_string(),
                broker_id: broker_id.to_string(),
            }),
        }
    }

    pub fn describe_user(&self, broker_id: &str, username: &str) -> Result<&MqUser, MqError> {
        let broker = self
            .brokers
            .get(broker_id)
            .ok_or_else(|| MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            })?;

        broker
            .users
            .get(username)
            .ok_or_else(|| MqError::UserNotFound {
                username: username.to_string(),
                broker_id: broker_id.to_string(),
            })
    }

    pub fn list_users(&self, broker_id: &str) -> Result<Vec<&MqUser>, MqError> {
        let broker = self
            .brokers
            .get(broker_id)
            .ok_or_else(|| MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            })?;

        Ok(broker.users.values().collect())
    }

    // --- Reboot ---

    pub fn reboot_broker(&self, broker_id: &str) -> Result<(), MqError> {
        if !self.brokers.contains_key(broker_id) {
            return Err(MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            });
        }
        Ok(())
    }

    // --- Update broker ---

    pub fn update_broker(
        &mut self,
        broker_id: &str,
        auto_minor_version_upgrade: Option<bool>,
        engine_version: Option<&str>,
    ) -> Result<&Broker, MqError> {
        let broker = self
            .brokers
            .get_mut(broker_id)
            .ok_or_else(|| MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            })?;

        if let Some(v) = auto_minor_version_upgrade {
            broker.auto_minor_version_upgrade = v;
        }
        if let Some(v) = engine_version {
            broker.engine_version = v.to_string();
        }

        Ok(broker)
    }

    // --- Update user ---

    pub fn update_user(
        &mut self,
        broker_id: &str,
        username: &str,
        console_access: Option<bool>,
        groups: Option<Vec<String>>,
    ) -> Result<(), MqError> {
        let broker = self
            .brokers
            .get_mut(broker_id)
            .ok_or_else(|| MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            })?;

        let user = broker
            .users
            .get_mut(username)
            .ok_or_else(|| MqError::UserNotFoundShort {
                username: username.to_string(),
            })?;

        if let Some(ca) = console_access {
            user.console_access = ca;
        }
        if let Some(g) = groups {
            user.groups = g;
        }

        Ok(())
    }

    // --- Promote broker ---

    pub fn promote_broker(&self, broker_id: &str, _mode: &str) -> Result<String, MqError> {
        if !self.brokers.contains_key(broker_id) {
            return Err(MqError::BrokerNotFound {
                broker_id: broker_id.to_string(),
            });
        }
        Ok(broker_id.to_string())
    }

    // --- Describe broker engine types (static) ---

    pub fn describe_broker_engine_types(&self) -> Vec<(String, Vec<String>)> {
        vec![
            (
                "ACTIVEMQ".to_string(),
                vec![
                    "5.17.6".to_string(),
                    "5.16.7".to_string(),
                    "5.15.16".to_string(),
                ],
            ),
            (
                "RABBITMQ".to_string(),
                vec!["3.11.20".to_string(), "3.10.20".to_string()],
            ),
        ]
    }
}
