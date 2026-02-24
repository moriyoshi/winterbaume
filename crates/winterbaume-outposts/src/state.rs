use std::collections::HashMap;

use thiserror::Error;

use crate::types::{Outpost, Site};

#[derive(Debug, Default)]
pub struct OutpostsState {
    pub outposts: HashMap<String, Outpost>,
    pub sites: HashMap<String, Site>,
}

#[derive(Debug, Error)]
pub enum OutpostsError {
    #[error("Outpost '{id}' not found.")]
    OutpostNotFound { id: String },

    #[error("Site '{id}' not found.")]
    SiteNotFound { id: String },

    #[error("Resource '{arn}' not found.")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl OutpostsState {
    // -------------------------------------------------------------------------
    // Outposts
    // -------------------------------------------------------------------------

    pub fn create_outpost(
        &mut self,
        name: &str,
        site_id: &str,
        description: Option<&str>,
        availability_zone: Option<&str>,
        availability_zone_id: Option<&str>,
        supported_hardware_type: Option<&str>,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&Outpost, OutpostsError> {
        // Site must exist
        let site = self
            .sites
            .get(site_id)
            .ok_or_else(|| OutpostsError::SiteNotFound {
                id: site_id.to_string(),
            })?;
        let site_arn = site.site_arn.clone();

        let outpost_id = format!("op-{:032x}", uuid::Uuid::new_v4().as_u128())
            .chars()
            .take(20)
            .collect::<String>();
        let outpost_arn = format!("arn:aws:outposts:{region}:{account_id}:outpost/{outpost_id}");

        let outpost = Outpost {
            outpost_id: outpost_id.clone(),
            outpost_arn,
            owner_id: account_id.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            site_id: site_id.to_string(),
            site_arn,
            availability_zone: availability_zone.map(|s| s.to_string()),
            availability_zone_id: availability_zone_id.map(|s| s.to_string()),
            life_cycle_status: "ACTIVE".to_string(),
            supported_hardware_type: supported_hardware_type.map(|s| s.to_string()),
            tags,
        };

        self.outposts.insert(outpost_id.clone(), outpost);
        Ok(self.outposts.get(&outpost_id).unwrap())
    }

    pub fn get_outpost(&self, id: &str) -> Result<&Outpost, OutpostsError> {
        // Accept either outpost ID or ARN
        let outpost_id = extract_outpost_id(id);
        self.outposts
            .get(outpost_id)
            .ok_or_else(|| OutpostsError::OutpostNotFound { id: id.to_string() })
    }

    pub fn delete_outpost(&mut self, id: &str) -> Result<(), OutpostsError> {
        let outpost_id = extract_outpost_id(id);
        if self.outposts.remove(outpost_id).is_none() {
            return Err(OutpostsError::OutpostNotFound { id: id.to_string() });
        }
        Ok(())
    }

    pub fn update_outpost(
        &mut self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        supported_hardware_type: Option<&str>,
    ) -> Result<&Outpost, OutpostsError> {
        let outpost_id = extract_outpost_id(id).to_string();
        let outpost = self
            .outposts
            .get_mut(&outpost_id)
            .ok_or_else(|| OutpostsError::OutpostNotFound { id: id.to_string() })?;

        if let Some(n) = name {
            outpost.name = n.to_string();
        }
        if let Some(d) = description {
            outpost.description = Some(d.to_string());
        }
        if let Some(h) = supported_hardware_type {
            outpost.supported_hardware_type = Some(h.to_string());
        }

        Ok(self.outposts.get(&outpost_id).unwrap())
    }

    pub fn list_outposts(&self) -> Vec<&Outpost> {
        self.outposts.values().collect()
    }

    // -------------------------------------------------------------------------
    // Sites
    // -------------------------------------------------------------------------

    pub fn create_site(
        &mut self,
        name: &str,
        description: Option<&str>,
        notes: Option<&str>,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> &Site {
        let site_id = format!("os-{:032x}", uuid::Uuid::new_v4().as_u128())
            .chars()
            .take(20)
            .collect::<String>();
        let site_arn = format!("arn:aws:outposts:{region}:{account_id}:site/{site_id}");

        let site = Site {
            site_id: site_id.clone(),
            site_arn,
            account_id: account_id.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            notes: notes.map(|s| s.to_string()),
            operating_address_country_code: None,
            operating_address_state_or_region: None,
            operating_address_city: None,
            tags,
        };

        self.sites.insert(site_id.clone(), site);
        self.sites.get(&site_id).unwrap()
    }

    pub fn get_site(&self, id: &str) -> Result<&Site, OutpostsError> {
        let site_id = extract_site_id(id);
        self.sites
            .get(site_id)
            .ok_or_else(|| OutpostsError::SiteNotFound { id: id.to_string() })
    }

    pub fn delete_site(&mut self, id: &str) -> Result<(), OutpostsError> {
        let site_id = extract_site_id(id);
        if self.sites.remove(site_id).is_none() {
            return Err(OutpostsError::SiteNotFound { id: id.to_string() });
        }
        Ok(())
    }

    pub fn update_site(
        &mut self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        notes: Option<&str>,
    ) -> Result<&Site, OutpostsError> {
        let site_id = extract_site_id(id).to_string();
        let site = self
            .sites
            .get_mut(&site_id)
            .ok_or_else(|| OutpostsError::SiteNotFound { id: id.to_string() })?;

        if let Some(n) = name {
            site.name = n.to_string();
        }
        if let Some(d) = description {
            site.description = Some(d.to_string());
        }
        if let Some(n) = notes {
            site.notes = Some(n.to_string());
        }

        Ok(self.sites.get(&site_id).unwrap())
    }

    pub fn list_sites(&self) -> Vec<&Site> {
        self.sites.values().collect()
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    pub fn get_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, OutpostsError> {
        // Check outposts by ARN
        for outpost in self.outposts.values() {
            if outpost.outpost_arn == resource_arn {
                return Ok(outpost.tags.clone());
            }
        }
        // Check sites by ARN
        for site in self.sites.values() {
            if site.site_arn == resource_arn {
                return Ok(site.tags.clone());
            }
        }
        Err(OutpostsError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), OutpostsError> {
        for outpost in self.outposts.values_mut() {
            if outpost.outpost_arn == resource_arn {
                outpost.tags.extend(tags);
                return Ok(());
            }
        }
        for site in self.sites.values_mut() {
            if site.site_arn == resource_arn {
                site.tags.extend(tags);
                return Ok(());
            }
        }
        Err(OutpostsError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), OutpostsError> {
        for outpost in self.outposts.values_mut() {
            if outpost.outpost_arn == resource_arn {
                for k in tag_keys {
                    outpost.tags.remove(k);
                }
                return Ok(());
            }
        }
        for site in self.sites.values_mut() {
            if site.site_arn == resource_arn {
                for k in tag_keys {
                    site.tags.remove(k);
                }
                return Ok(());
            }
        }
        Err(OutpostsError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }
}

/// Extract the outpost ID from an ARN or return the input as-is.
fn extract_outpost_id(id_or_arn: &str) -> &str {
    if let Some(pos) = id_or_arn.rfind('/') {
        &id_or_arn[pos + 1..]
    } else {
        id_or_arn
    }
}

/// Extract the site ID from an ARN or return the input as-is.
fn extract_site_id(id_or_arn: &str) -> &str {
    if let Some(pos) = id_or_arn.rfind('/') {
        &id_or_arn[pos + 1..]
    } else {
        id_or_arn
    }
}
