use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

/// In-memory state for the Directory Service.
#[derive(Debug, Default)]
pub struct DsState {
    /// Directories keyed by directory ID.
    pub directories: HashMap<String, Directory>,
}

/// Error type for Directory Service operations.
#[derive(Debug, Error)]
pub enum DsError {
    #[error(
        "1 validation error detected: Value at 'name' failed to satisfy constraint: Member must not be empty"
    )]
    NameEmpty,

    #[error(
        "1 validation error detected: Value at 'password' failed to satisfy constraint: Member must not be empty"
    )]
    PasswordEmpty,

    #[error("Invalid directory size: {size}")]
    InvalidDirectorySize { size: String },

    #[error("At least one DNS IP address must be specified")]
    DnsIpsEmpty,

    #[error("Directory {directory_id} does not exist")]
    DirectoryNotFound { directory_id: String },
}

impl DsState {
    /// Create a Simple AD or Microsoft AD directory.
    pub fn create_directory(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        size: &str,
        password: Option<&str>,
        description: Option<&str>,
        short_name: Option<&str>,
        vpc_settings: Option<(&str, &[String])>,
    ) -> Result<String, DsError> {
        // Validate required fields
        if name.is_empty() {
            return Err(DsError::NameEmpty);
        }

        if password.is_none() || password.unwrap().is_empty() {
            return Err(DsError::PasswordEmpty);
        }

        // Validate size
        let valid_sizes = ["Small", "Large"];
        if !valid_sizes.contains(&size) {
            return Err(DsError::InvalidDirectorySize {
                size: size.to_string(),
            });
        }

        let directory_id = format!(
            "d-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..10]
        );
        let now = Utc::now();

        let resolved_vpc_settings = vpc_settings.map(|(vpc_id, subnet_ids)| DirectoryVpcSettings {
            vpc_id: vpc_id.to_string(),
            subnet_ids: subnet_ids.to_vec(),
            security_group_id: format!(
                "sg-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..8]
            ),
            availability_zones: subnet_ids
                .iter()
                .enumerate()
                .map(|(i, _)| format!("{}{}", region, (b'a' + i as u8) as char))
                .collect(),
        });

        let _ = account_id; // reserved for future ARN use

        let directory = Directory {
            directory_id: directory_id.clone(),
            name: name.to_string(),
            short_name: short_name.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            size: size.to_string(),
            directory_type: DirectoryType::SimpleAD,
            alias: directory_id.clone(),
            access_url: format!("{}.awsapps.com", directory_id),
            stage: DirectoryStage::Active,
            launch_time: now,
            stage_last_updated_date_time: now,
            dns_ip_addrs: vec!["10.0.0.1".to_string(), "10.0.0.2".to_string()],
            vpc_settings: resolved_vpc_settings,
            connect_settings: None,
            ssoid_enabled: false,
        };

        self.directories.insert(directory_id.clone(), directory);
        Ok(directory_id)
    }

    /// Connect to an existing on-premises directory (AD Connector).
    pub fn connect_directory(
        &mut self,
        _account_id: &str,
        region: &str,
        name: &str,
        size: &str,
        password: Option<&str>,
        short_name: Option<&str>,
        description: Option<&str>,
        connect_settings_vpc_id: &str,
        connect_settings_subnet_ids: &[String],
        connect_settings_dns_ips: &[String],
        connect_settings_user_name: &str,
    ) -> Result<String, DsError> {
        if name.is_empty() {
            return Err(DsError::NameEmpty);
        }

        if password.is_none() || password.unwrap().is_empty() {
            return Err(DsError::PasswordEmpty);
        }

        let valid_sizes = ["Small", "Large"];
        if !valid_sizes.contains(&size) {
            return Err(DsError::InvalidDirectorySize {
                size: size.to_string(),
            });
        }

        if connect_settings_dns_ips.is_empty() {
            return Err(DsError::DnsIpsEmpty);
        }

        let directory_id = format!(
            "d-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..10]
        );
        let now = Utc::now();

        let connect_settings = DirectoryConnectSettings {
            vpc_id: connect_settings_vpc_id.to_string(),
            subnet_ids: connect_settings_subnet_ids.to_vec(),
            customer_dns_ips: connect_settings_dns_ips.to_vec(),
            customer_user_name: connect_settings_user_name.to_string(),
            security_group_id: format!(
                "sg-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..8]
            ),
            availability_zones: connect_settings_subnet_ids
                .iter()
                .enumerate()
                .map(|(i, _)| format!("{}{}", region, (b'a' + i as u8) as char))
                .collect(),
            connect_ips: vec!["203.0.113.1".to_string(), "203.0.113.2".to_string()],
        };

        let directory = Directory {
            directory_id: directory_id.clone(),
            name: name.to_string(),
            short_name: short_name.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            size: size.to_string(),
            directory_type: DirectoryType::ADConnector,
            alias: directory_id.clone(),
            access_url: format!("{}.awsapps.com", directory_id),
            stage: DirectoryStage::Active,
            launch_time: now,
            stage_last_updated_date_time: now,
            dns_ip_addrs: connect_settings_dns_ips.to_vec(),
            vpc_settings: None,
            connect_settings: Some(connect_settings),
            ssoid_enabled: false,
        };

        self.directories.insert(directory_id.clone(), directory);
        Ok(directory_id)
    }

    /// Describe one or more directories.
    pub fn describe_directories(
        &self,
        directory_ids: Option<&[String]>,
    ) -> Result<Vec<&Directory>, DsError> {
        match directory_ids {
            Some(ids) if !ids.is_empty() => {
                let mut result = Vec::new();
                for id in ids {
                    match self.directories.get(id) {
                        Some(d) => result.push(d),
                        None => {
                            return Err(DsError::DirectoryNotFound {
                                directory_id: id.clone(),
                            });
                        }
                    }
                }
                Ok(result)
            }
            _ => Ok(self.directories.values().collect()),
        }
    }

    /// Delete a directory.
    pub fn delete_directory(&mut self, directory_id: &str) -> Result<String, DsError> {
        if self.directories.remove(directory_id).is_none() {
            return Err(DsError::DirectoryNotFound {
                directory_id: directory_id.to_string(),
            });
        }
        Ok(directory_id.to_string())
    }
}
