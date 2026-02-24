use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct NetworkManagerState {
    pub global_networks: HashMap<String, GlobalNetwork>,
    pub core_networks: HashMap<String, CoreNetwork>,
    pub sites: HashMap<String, Site>,
    pub links: HashMap<String, Link>,
    pub devices: HashMap<String, Device>,
    pub connections: HashMap<String, Connection>,
    pub transit_gateway_registrations: HashMap<String, TransitGatewayRegistration>,
    pub attachments: HashMap<String, Attachment>,
    pub connect_peers: HashMap<String, ConnectPeer>,
    pub connect_peer_associations: HashMap<String, ConnectPeerAssociation>,
    pub link_associations: Vec<LinkAssociation>,
    pub customer_gateway_associations: HashMap<String, CustomerGatewayAssociation>,
    pub transit_gateway_connect_peer_associations:
        HashMap<String, TransitGatewayConnectPeerAssociation>,
    pub route_analyses: HashMap<String, RouteAnalysis>,
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkManagerError {
    #[error("Global network {id} not found.")]
    GlobalNetworkNotFound { id: String },

    #[error("Core network {id} not found.")]
    CoreNetworkNotFound { id: String },

    #[error("Site {id} not found.")]
    SiteNotFound { id: String },

    #[error("Link {id} not found.")]
    LinkNotFound { id: String },

    #[error("Device {id} not found.")]
    DeviceNotFound { id: String },

    #[error("Connection {id} not found.")]
    ConnectionNotFound { id: String },

    #[error("Resource {arn} not found.")]
    ResourceNotFound { arn: String },

    #[error("Transit gateway {arn} not registered in {gn_id}.")]
    TransitGatewayNotRegistered { arn: String, gn_id: String },

    #[error("Attachment {id} not found.")]
    AttachmentNotFound { id: String },

    #[error("Connect peer {id} not found.")]
    ConnectPeerNotFound { id: String },

    #[error("Connect peer association for {cp_id} in {gn_id} not found.")]
    ConnectPeerAssociationNotFound { cp_id: String, gn_id: String },

    #[error("Link association for device {device_id} and link {link_id} not found.")]
    LinkAssociationNotFound { device_id: String, link_id: String },

    #[error("Customer gateway {arn} not found in {gn_id}.")]
    CustomerGatewayNotFound { arn: String, gn_id: String },

    #[error("Transit gateway connect peer association for {arn} not found.")]
    TransitGatewayConnectPeerAssociationNotFound { arn: String },

    #[error("Transit gateway connect peer association for {arn} not found in {gn_id}.")]
    TransitGatewayConnectPeerAssociationNotFoundInNetwork { arn: String, gn_id: String },

    #[error("Route analysis {id} not found.")]
    RouteAnalysisNotFound { id: String },

    #[error("Route analysis {id} not found in {gn_id}.")]
    RouteAnalysisNotFoundInNetwork { id: String, gn_id: String },
}

impl NetworkManagerState {
    // ── Global Networks ──

    pub fn create_global_network(
        &mut self,
        description: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&GlobalNetwork, NetworkManagerError> {
        let id = format!("global-network-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let arn = format!("arn:aws:networkmanager:{account_id}:global-network/{id}");

        let network = GlobalNetwork {
            global_network_id: id.clone(),
            global_network_arn: arn,
            description: description.to_string(),
            created_at: Utc::now(),
            state: "AVAILABLE".to_string(),
            tags,
        };

        self.global_networks.insert(id.clone(), network);
        Ok(self.global_networks.get(&id).unwrap())
    }

    pub fn describe_global_networks(&self) -> Vec<&GlobalNetwork> {
        self.global_networks.values().collect()
    }

    pub fn get_global_network(
        &self,
        global_network_id: &str,
    ) -> Result<&GlobalNetwork, NetworkManagerError> {
        self.global_networks.get(global_network_id).ok_or_else(|| {
            NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            }
        })
    }

    pub fn delete_global_network(
        &mut self,
        global_network_id: &str,
    ) -> Result<GlobalNetwork, NetworkManagerError> {
        self.global_networks
            .remove(global_network_id)
            .ok_or_else(|| NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            })
    }

    pub fn update_global_network(
        &mut self,
        global_network_id: &str,
        description: Option<&str>,
    ) -> Result<&GlobalNetwork, NetworkManagerError> {
        let network = self
            .global_networks
            .get_mut(global_network_id)
            .ok_or_else(|| NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            })?;

        if let Some(desc) = description {
            network.description = desc.to_string();
        }

        Ok(network)
    }

    // ── Core Networks ──

    pub fn create_core_network(
        &mut self,
        global_network_id: &str,
        description: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&CoreNetwork, NetworkManagerError> {
        // Validate that the global network exists
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        let id = format!("core-network-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let arn = format!("arn:aws:networkmanager::{account_id}:core-network/{id}");

        let core_network = CoreNetwork {
            core_network_id: id.clone(),
            core_network_arn: arn,
            global_network_id: global_network_id.to_string(),
            description: description.to_string(),
            created_at: Utc::now(),
            state: "AVAILABLE".to_string(),
            tags,
        };

        self.core_networks.insert(id.clone(), core_network);
        Ok(self.core_networks.get(&id).unwrap())
    }

    pub fn get_core_network(
        &self,
        core_network_id: &str,
    ) -> Result<&CoreNetwork, NetworkManagerError> {
        self.core_networks.get(core_network_id).ok_or_else(|| {
            NetworkManagerError::CoreNetworkNotFound {
                id: core_network_id.to_string(),
            }
        })
    }

    pub fn delete_core_network(
        &mut self,
        core_network_id: &str,
    ) -> Result<CoreNetwork, NetworkManagerError> {
        self.core_networks.remove(core_network_id).ok_or_else(|| {
            NetworkManagerError::CoreNetworkNotFound {
                id: core_network_id.to_string(),
            }
        })
    }

    pub fn update_core_network(
        &mut self,
        core_network_id: &str,
        description: Option<&str>,
    ) -> Result<&CoreNetwork, NetworkManagerError> {
        let cn = self.core_networks.get_mut(core_network_id).ok_or_else(|| {
            NetworkManagerError::CoreNetworkNotFound {
                id: core_network_id.to_string(),
            }
        })?;

        if let Some(desc) = description {
            cn.description = desc.to_string();
        }

        Ok(cn)
    }

    pub fn list_core_networks(&self) -> Vec<&CoreNetwork> {
        self.core_networks.values().collect()
    }

    // ── Sites ──

    pub fn create_site(
        &mut self,
        global_network_id: &str,
        description: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Site, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        let id = format!("site-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let arn = format!("arn:aws:networkmanager::{account_id}:site/{global_network_id}/{id}");

        let site = Site {
            site_id: id.clone(),
            site_arn: arn,
            global_network_id: global_network_id.to_string(),
            description: description.to_string(),
            created_at: Utc::now(),
            state: "AVAILABLE".to_string(),
            tags,
        };

        self.sites.insert(id.clone(), site);
        Ok(self.sites.get(&id).unwrap())
    }

    pub fn get_site(&self, site_id: &str) -> Result<&Site, NetworkManagerError> {
        self.sites
            .get(site_id)
            .ok_or_else(|| NetworkManagerError::SiteNotFound {
                id: site_id.to_string(),
            })
    }

    pub fn get_sites(&self, global_network_id: &str) -> Vec<&Site> {
        self.sites
            .values()
            .filter(|s| s.global_network_id == global_network_id)
            .collect()
    }

    pub fn delete_site(&mut self, site_id: &str) -> Result<Site, NetworkManagerError> {
        self.sites
            .remove(site_id)
            .ok_or_else(|| NetworkManagerError::SiteNotFound {
                id: site_id.to_string(),
            })
    }

    pub fn update_site(
        &mut self,
        site_id: &str,
        description: Option<&str>,
    ) -> Result<&Site, NetworkManagerError> {
        let site =
            self.sites
                .get_mut(site_id)
                .ok_or_else(|| NetworkManagerError::SiteNotFound {
                    id: site_id.to_string(),
                })?;

        if let Some(desc) = description {
            site.description = desc.to_string();
        }

        Ok(site)
    }

    // ── Links ──

    pub fn create_link(
        &mut self,
        global_network_id: &str,
        site_id: &str,
        description: &str,
        provider: &str,
        link_type: &str,
        download_speed: i32,
        upload_speed: i32,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Link, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        if !self.sites.contains_key(site_id) {
            return Err(NetworkManagerError::SiteNotFound {
                id: site_id.to_string(),
            });
        }

        let id = format!("link-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let arn = format!("arn:aws:networkmanager::{account_id}:link/{global_network_id}/{id}");

        let link = Link {
            link_id: id.clone(),
            link_arn: arn,
            global_network_id: global_network_id.to_string(),
            site_id: site_id.to_string(),
            description: description.to_string(),
            provider: provider.to_string(),
            link_type: link_type.to_string(),
            bandwidth_download_speed: download_speed,
            bandwidth_upload_speed: upload_speed,
            created_at: Utc::now(),
            state: "AVAILABLE".to_string(),
            tags,
        };

        self.links.insert(id.clone(), link);
        Ok(self.links.get(&id).unwrap())
    }

    pub fn delete_link(&mut self, link_id: &str) -> Result<Link, NetworkManagerError> {
        self.links
            .remove(link_id)
            .ok_or_else(|| NetworkManagerError::LinkNotFound {
                id: link_id.to_string(),
            })
    }

    pub fn update_link(
        &mut self,
        link_id: &str,
        description: Option<&str>,
        provider: Option<&str>,
        link_type: Option<&str>,
        download_speed: Option<i32>,
        upload_speed: Option<i32>,
    ) -> Result<&Link, NetworkManagerError> {
        let link =
            self.links
                .get_mut(link_id)
                .ok_or_else(|| NetworkManagerError::LinkNotFound {
                    id: link_id.to_string(),
                })?;
        if let Some(d) = description {
            link.description = d.to_string();
        }
        if let Some(p) = provider {
            link.provider = p.to_string();
        }
        if let Some(t) = link_type {
            link.link_type = t.to_string();
        }
        if let Some(ds) = download_speed {
            link.bandwidth_download_speed = ds;
        }
        if let Some(us) = upload_speed {
            link.bandwidth_upload_speed = us;
        }
        Ok(link)
    }

    // ── Tags ──

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), NetworkManagerError> {
        // Try to find the resource by ARN and add tags
        for network in self.global_networks.values_mut() {
            if network.global_network_arn == resource_arn {
                network.tags.extend(tags);
                return Ok(());
            }
        }
        for cn in self.core_networks.values_mut() {
            if cn.core_network_arn == resource_arn {
                cn.tags.extend(tags);
                return Ok(());
            }
        }
        for site in self.sites.values_mut() {
            if site.site_arn == resource_arn {
                site.tags.extend(tags);
                return Ok(());
            }
        }
        for link in self.links.values_mut() {
            if link.link_arn == resource_arn {
                link.tags.extend(tags);
                return Ok(());
            }
        }
        for device in self.devices.values_mut() {
            if device.device_arn == resource_arn {
                device.tags.extend(tags);
                return Ok(());
            }
        }
        for conn in self.connections.values_mut() {
            if conn.connection_arn == resource_arn {
                conn.tags.extend(tags);
                return Ok(());
            }
        }
        for att in self.attachments.values_mut() {
            if att.attachment_id == resource_arn {
                att.tags.extend(tags);
                return Ok(());
            }
        }
        for cp in self.connect_peers.values_mut() {
            if cp.connect_peer_id == resource_arn {
                cp.tags.extend(tags);
                return Ok(());
            }
        }

        Err(NetworkManagerError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), NetworkManagerError> {
        for network in self.global_networks.values_mut() {
            if network.global_network_arn == resource_arn {
                for key in tag_keys {
                    network.tags.remove(key);
                }
                return Ok(());
            }
        }
        for cn in self.core_networks.values_mut() {
            if cn.core_network_arn == resource_arn {
                for key in tag_keys {
                    cn.tags.remove(key);
                }
                return Ok(());
            }
        }
        for site in self.sites.values_mut() {
            if site.site_arn == resource_arn {
                for key in tag_keys {
                    site.tags.remove(key);
                }
                return Ok(());
            }
        }
        for link in self.links.values_mut() {
            if link.link_arn == resource_arn {
                for key in tag_keys {
                    link.tags.remove(key);
                }
                return Ok(());
            }
        }
        for device in self.devices.values_mut() {
            if device.device_arn == resource_arn {
                for key in tag_keys {
                    device.tags.remove(key);
                }
                return Ok(());
            }
        }
        for conn in self.connections.values_mut() {
            if conn.connection_arn == resource_arn {
                for key in tag_keys {
                    conn.tags.remove(key);
                }
                return Ok(());
            }
        }
        for att in self.attachments.values_mut() {
            if att.attachment_id == resource_arn {
                for key in tag_keys {
                    att.tags.remove(key);
                }
                return Ok(());
            }
        }
        for cp in self.connect_peers.values_mut() {
            if cp.connect_peer_id == resource_arn {
                for key in tag_keys {
                    cp.tags.remove(key);
                }
                return Ok(());
            }
        }

        Err(NetworkManagerError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, NetworkManagerError> {
        for network in self.global_networks.values() {
            if network.global_network_arn == resource_arn {
                return Ok(&network.tags);
            }
        }
        for cn in self.core_networks.values() {
            if cn.core_network_arn == resource_arn {
                return Ok(&cn.tags);
            }
        }
        for site in self.sites.values() {
            if site.site_arn == resource_arn {
                return Ok(&site.tags);
            }
        }
        for link in self.links.values() {
            if link.link_arn == resource_arn {
                return Ok(&link.tags);
            }
        }
        for device in self.devices.values() {
            if device.device_arn == resource_arn {
                return Ok(&device.tags);
            }
        }
        for conn in self.connections.values() {
            if conn.connection_arn == resource_arn {
                return Ok(&conn.tags);
            }
        }
        for att in self.attachments.values() {
            if att.attachment_id == resource_arn {
                return Ok(&att.tags);
            }
        }
        for cp in self.connect_peers.values() {
            if cp.connect_peer_id == resource_arn {
                return Ok(&cp.tags);
            }
        }

        Err(NetworkManagerError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    // ── Get Links ──

    pub fn get_links(&self, global_network_id: &str) -> Vec<&Link> {
        self.links
            .values()
            .filter(|l| l.global_network_id == global_network_id)
            .collect()
    }

    // ── Devices ──

    #[allow(clippy::too_many_arguments)]
    pub fn create_device(
        &mut self,
        global_network_id: &str,
        description: &str,
        site_id: Option<&str>,
        model: Option<&str>,
        serial_number: Option<&str>,
        device_type: Option<&str>,
        vendor: Option<&str>,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Device, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        let id = format!("device-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let arn = format!("arn:aws:networkmanager::{account_id}:device/{global_network_id}/{id}");

        let device = Device {
            device_id: id.clone(),
            device_arn: arn,
            global_network_id: global_network_id.to_string(),
            description: description.to_string(),
            site_id: site_id.map(|s| s.to_string()),
            model: model.map(|s| s.to_string()),
            serial_number: serial_number.map(|s| s.to_string()),
            device_type: device_type.map(|s| s.to_string()),
            vendor: vendor.map(|s| s.to_string()),
            created_at: Utc::now(),
            state: "AVAILABLE".to_string(),
            tags,
        };

        self.devices.insert(id.clone(), device);
        Ok(self.devices.get(&id).unwrap())
    }

    pub fn delete_device(&mut self, device_id: &str) -> Result<Device, NetworkManagerError> {
        self.devices
            .remove(device_id)
            .ok_or_else(|| NetworkManagerError::DeviceNotFound {
                id: device_id.to_string(),
            })
    }

    pub fn get_devices(&self, global_network_id: &str) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|d| d.global_network_id == global_network_id)
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_device(
        &mut self,
        device_id: &str,
        description: Option<&str>,
        site_id: Option<&str>,
        model: Option<&str>,
        serial_number: Option<&str>,
        device_type: Option<&str>,
        vendor: Option<&str>,
    ) -> Result<&Device, NetworkManagerError> {
        let device =
            self.devices
                .get_mut(device_id)
                .ok_or_else(|| NetworkManagerError::DeviceNotFound {
                    id: device_id.to_string(),
                })?;
        if let Some(d) = description {
            device.description = d.to_string();
        }
        if let Some(s) = site_id {
            device.site_id = Some(s.to_string());
        }
        if let Some(m) = model {
            device.model = Some(m.to_string());
        }
        if let Some(sn) = serial_number {
            device.serial_number = Some(sn.to_string());
        }
        if let Some(t) = device_type {
            device.device_type = Some(t.to_string());
        }
        if let Some(v) = vendor {
            device.vendor = Some(v.to_string());
        }
        Ok(device)
    }

    // ── Connections ──

    #[allow(clippy::too_many_arguments)]
    pub fn create_connection(
        &mut self,
        global_network_id: &str,
        device_id: &str,
        connected_device_id: &str,
        link_id: Option<&str>,
        connected_link_id: Option<&str>,
        description: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Connection, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }
        if !self.devices.contains_key(device_id) {
            return Err(NetworkManagerError::DeviceNotFound {
                id: device_id.to_string(),
            });
        }
        if !self.devices.contains_key(connected_device_id) {
            return Err(NetworkManagerError::DeviceNotFound {
                id: connected_device_id.to_string(),
            });
        }

        let id = format!("conn-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let arn =
            format!("arn:aws:networkmanager::{account_id}:connection/{global_network_id}/{id}");

        let connection = Connection {
            connection_id: id.clone(),
            connection_arn: arn,
            global_network_id: global_network_id.to_string(),
            device_id: device_id.to_string(),
            connected_device_id: connected_device_id.to_string(),
            link_id: link_id.map(|s| s.to_string()),
            connected_link_id: connected_link_id.map(|s| s.to_string()),
            description: description.to_string(),
            created_at: Utc::now(),
            state: "AVAILABLE".to_string(),
            tags,
        };

        self.connections.insert(id.clone(), connection);
        Ok(self.connections.get(&id).unwrap())
    }

    pub fn get_connections(&self, global_network_id: &str) -> Vec<&Connection> {
        self.connections
            .values()
            .filter(|c| c.global_network_id == global_network_id)
            .collect()
    }

    pub fn delete_connection(
        &mut self,
        connection_id: &str,
    ) -> Result<Connection, NetworkManagerError> {
        self.connections.remove(connection_id).ok_or_else(|| {
            NetworkManagerError::ConnectionNotFound {
                id: connection_id.to_string(),
            }
        })
    }

    pub fn update_connection(
        &mut self,
        connection_id: &str,
        description: Option<&str>,
        link_id: Option<&str>,
        connected_link_id: Option<&str>,
    ) -> Result<&Connection, NetworkManagerError> {
        let conn = self.connections.get_mut(connection_id).ok_or_else(|| {
            NetworkManagerError::ConnectionNotFound {
                id: connection_id.to_string(),
            }
        })?;
        if let Some(d) = description {
            conn.description = d.to_string();
        }
        if let Some(l) = link_id {
            conn.link_id = Some(l.to_string());
        }
        if let Some(cl) = connected_link_id {
            conn.connected_link_id = Some(cl.to_string());
        }
        Ok(conn)
    }

    // ── Transit Gateway Registrations ──

    pub fn register_transit_gateway(
        &mut self,
        global_network_id: &str,
        transit_gateway_arn: &str,
    ) -> Result<&TransitGatewayRegistration, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        let reg = TransitGatewayRegistration {
            global_network_id: global_network_id.to_string(),
            transit_gateway_arn: transit_gateway_arn.to_string(),
            state: "AVAILABLE".to_string(),
            state_message: String::new(),
        };

        self.transit_gateway_registrations
            .insert(transit_gateway_arn.to_string(), reg);
        Ok(self
            .transit_gateway_registrations
            .get(transit_gateway_arn)
            .unwrap())
    }

    pub fn deregister_transit_gateway(
        &mut self,
        global_network_id: &str,
        transit_gateway_arn: &str,
    ) -> Result<TransitGatewayRegistration, NetworkManagerError> {
        let reg = self
            .transit_gateway_registrations
            .get(transit_gateway_arn)
            .ok_or_else(|| NetworkManagerError::TransitGatewayNotRegistered {
                arn: transit_gateway_arn.to_string(),
                gn_id: global_network_id.to_string(),
            })?;
        if reg.global_network_id != global_network_id {
            return Err(NetworkManagerError::TransitGatewayNotRegistered {
                arn: transit_gateway_arn.to_string(),
                gn_id: global_network_id.to_string(),
            });
        }
        Ok(self
            .transit_gateway_registrations
            .remove(transit_gateway_arn)
            .unwrap())
    }

    pub fn get_transit_gateway_registrations(
        &self,
        global_network_id: &str,
    ) -> Vec<&TransitGatewayRegistration> {
        self.transit_gateway_registrations
            .values()
            .filter(|r| r.global_network_id == global_network_id)
            .collect()
    }

    // ── VPC / Attachments ──

    pub fn create_vpc_attachment(
        &mut self,
        core_network_id: &str,
        vpc_arn: &str,
        subnet_arns: Vec<String>,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Attachment, NetworkManagerError> {
        let cn = self.core_networks.get(core_network_id).ok_or_else(|| {
            NetworkManagerError::CoreNetworkNotFound {
                id: core_network_id.to_string(),
            }
        })?;
        let cn_arn = cn.core_network_arn.clone();

        let id = format!("attachment-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let now = Utc::now();
        let attachment = Attachment {
            attachment_id: id.clone(),
            attachment_type: "VPC".to_string(),
            core_network_id: core_network_id.to_string(),
            core_network_arn: cn_arn,
            owner_account_id: account_id.to_string(),
            resource_arn: vpc_arn.to_string(),
            edge_location: None,
            state: "AVAILABLE".to_string(),
            created_at: now,
            updated_at: now,
            tags,
            subnet_arns,
            segment_name: None,
        };

        self.attachments.insert(id.clone(), attachment);
        Ok(self.attachments.get(&id).unwrap())
    }

    pub fn get_attachment(&self, attachment_id: &str) -> Result<&Attachment, NetworkManagerError> {
        self.attachments
            .get(attachment_id)
            .ok_or_else(|| NetworkManagerError::AttachmentNotFound {
                id: attachment_id.to_string(),
            })
    }

    pub fn delete_attachment(
        &mut self,
        attachment_id: &str,
    ) -> Result<Attachment, NetworkManagerError> {
        self.attachments.remove(attachment_id).ok_or_else(|| {
            NetworkManagerError::AttachmentNotFound {
                id: attachment_id.to_string(),
            }
        })
    }

    pub fn accept_attachment(
        &mut self,
        attachment_id: &str,
    ) -> Result<&Attachment, NetworkManagerError> {
        let att = self.attachments.get_mut(attachment_id).ok_or_else(|| {
            NetworkManagerError::AttachmentNotFound {
                id: attachment_id.to_string(),
            }
        })?;
        att.state = "AVAILABLE".to_string();
        Ok(att)
    }

    pub fn reject_attachment(
        &mut self,
        attachment_id: &str,
    ) -> Result<&Attachment, NetworkManagerError> {
        let att = self.attachments.get_mut(attachment_id).ok_or_else(|| {
            NetworkManagerError::AttachmentNotFound {
                id: attachment_id.to_string(),
            }
        })?;
        att.state = "REJECTED".to_string();
        Ok(att)
    }

    pub fn list_attachments(&self) -> Vec<&Attachment> {
        self.attachments.values().collect()
    }

    // ── Connect Peers ──

    pub fn create_connect_peer(
        &mut self,
        connect_attachment_id: &str,
        peer_address: &str,
        core_network_address: Option<&str>,
        inside_cidr_blocks: Vec<String>,
        tags: HashMap<String, String>,
    ) -> Result<&ConnectPeer, NetworkManagerError> {
        // Look up the core_network_id from the attachment if it exists
        let core_network_id = self
            .attachments
            .get(connect_attachment_id)
            .map(|a| a.core_network_id.clone());

        let id = format!("cp-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let peer = ConnectPeer {
            connect_peer_id: id.clone(),
            connect_attachment_id: connect_attachment_id.to_string(),
            core_network_id,
            edge_location: None,
            peer_address: peer_address.to_string(),
            core_network_address: core_network_address.map(|s| s.to_string()),
            inside_cidr_blocks,
            created_at: Utc::now(),
            state: "AVAILABLE".to_string(),
            tags,
        };

        self.connect_peers.insert(id.clone(), peer);
        Ok(self.connect_peers.get(&id).unwrap())
    }

    pub fn get_connect_peer(
        &self,
        connect_peer_id: &str,
    ) -> Result<&ConnectPeer, NetworkManagerError> {
        self.connect_peers.get(connect_peer_id).ok_or_else(|| {
            NetworkManagerError::ConnectPeerNotFound {
                id: connect_peer_id.to_string(),
            }
        })
    }

    pub fn delete_connect_peer(
        &mut self,
        connect_peer_id: &str,
    ) -> Result<ConnectPeer, NetworkManagerError> {
        self.connect_peers.remove(connect_peer_id).ok_or_else(|| {
            NetworkManagerError::ConnectPeerNotFound {
                id: connect_peer_id.to_string(),
            }
        })
    }

    pub fn list_connect_peers(&self) -> Vec<&ConnectPeer> {
        self.connect_peers.values().collect()
    }

    // ── Connect Peer Associations ──

    pub fn associate_connect_peer(
        &mut self,
        global_network_id: &str,
        connect_peer_id: &str,
        device_id: &str,
        link_id: Option<&str>,
    ) -> Result<&ConnectPeerAssociation, NetworkManagerError> {
        if !self.connect_peers.contains_key(connect_peer_id) {
            return Err(NetworkManagerError::ConnectPeerNotFound {
                id: connect_peer_id.to_string(),
            });
        }

        let assoc = ConnectPeerAssociation {
            connect_peer_id: connect_peer_id.to_string(),
            global_network_id: global_network_id.to_string(),
            device_id: device_id.to_string(),
            link_id: link_id.map(|s| s.to_string()),
            state: "AVAILABLE".to_string(),
        };

        self.connect_peer_associations
            .insert(connect_peer_id.to_string(), assoc);
        Ok(self.connect_peer_associations.get(connect_peer_id).unwrap())
    }

    pub fn disassociate_connect_peer(
        &mut self,
        global_network_id: &str,
        connect_peer_id: &str,
    ) -> Result<ConnectPeerAssociation, NetworkManagerError> {
        let assoc = self
            .connect_peer_associations
            .get(connect_peer_id)
            .ok_or_else(|| NetworkManagerError::ConnectPeerAssociationNotFound {
                cp_id: connect_peer_id.to_string(),
                gn_id: global_network_id.to_string(),
            })?;
        if assoc.global_network_id != global_network_id {
            return Err(NetworkManagerError::ConnectPeerAssociationNotFound {
                cp_id: connect_peer_id.to_string(),
                gn_id: global_network_id.to_string(),
            });
        }
        Ok(self
            .connect_peer_associations
            .remove(connect_peer_id)
            .unwrap())
    }

    pub fn get_connect_peer_associations(
        &self,
        global_network_id: &str,
    ) -> Vec<&ConnectPeerAssociation> {
        self.connect_peer_associations
            .values()
            .filter(|a| a.global_network_id == global_network_id)
            .collect()
    }

    // ── Link Associations ──

    pub fn associate_link(
        &mut self,
        global_network_id: &str,
        device_id: &str,
        link_id: &str,
    ) -> Result<LinkAssociation, NetworkManagerError> {
        if !self.devices.contains_key(device_id) {
            return Err(NetworkManagerError::DeviceNotFound {
                id: device_id.to_string(),
            });
        }
        if !self.links.contains_key(link_id) {
            return Err(NetworkManagerError::LinkNotFound {
                id: link_id.to_string(),
            });
        }

        let assoc = LinkAssociation {
            global_network_id: global_network_id.to_string(),
            device_id: device_id.to_string(),
            link_id: link_id.to_string(),
            state: "AVAILABLE".to_string(),
        };
        self.link_associations.push(assoc.clone());
        Ok(assoc)
    }

    pub fn disassociate_link(
        &mut self,
        global_network_id: &str,
        device_id: &str,
        link_id: &str,
    ) -> Result<LinkAssociation, NetworkManagerError> {
        if let Some(pos) = self.link_associations.iter().position(|a| {
            a.global_network_id == global_network_id
                && a.device_id == device_id
                && a.link_id == link_id
        }) {
            Ok(self.link_associations.remove(pos))
        } else {
            Err(NetworkManagerError::LinkAssociationNotFound {
                device_id: device_id.to_string(),
                link_id: link_id.to_string(),
            })
        }
    }

    pub fn get_link_associations(&self, global_network_id: &str) -> Vec<&LinkAssociation> {
        self.link_associations
            .iter()
            .filter(|a| a.global_network_id == global_network_id)
            .collect()
    }

    // ── Customer Gateway Associations ──

    pub fn associate_customer_gateway(
        &mut self,
        global_network_id: &str,
        customer_gateway_arn: &str,
        device_id: &str,
        link_id: Option<&str>,
    ) -> Result<&CustomerGatewayAssociation, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        let assoc = CustomerGatewayAssociation {
            customer_gateway_arn: customer_gateway_arn.to_string(),
            global_network_id: global_network_id.to_string(),
            device_id: device_id.to_string(),
            link_id: link_id.map(|s| s.to_string()),
            state: "AVAILABLE".to_string(),
        };

        self.customer_gateway_associations
            .insert(customer_gateway_arn.to_string(), assoc);
        Ok(self
            .customer_gateway_associations
            .get(customer_gateway_arn)
            .unwrap())
    }

    pub fn disassociate_customer_gateway(
        &mut self,
        global_network_id: &str,
        customer_gateway_arn: &str,
    ) -> Result<CustomerGatewayAssociation, NetworkManagerError> {
        let assoc = self
            .customer_gateway_associations
            .get(customer_gateway_arn)
            .ok_or_else(|| NetworkManagerError::CustomerGatewayNotFound {
                arn: customer_gateway_arn.to_string(),
                gn_id: global_network_id.to_string(),
            })?;
        if assoc.global_network_id != global_network_id {
            return Err(NetworkManagerError::CustomerGatewayNotFound {
                arn: customer_gateway_arn.to_string(),
                gn_id: global_network_id.to_string(),
            });
        }
        Ok(self
            .customer_gateway_associations
            .remove(customer_gateway_arn)
            .unwrap())
    }

    pub fn get_customer_gateway_associations(
        &self,
        global_network_id: &str,
    ) -> Vec<&CustomerGatewayAssociation> {
        self.customer_gateway_associations
            .values()
            .filter(|a| a.global_network_id == global_network_id)
            .collect()
    }

    // ── Transit Gateway Connect Peer Associations ──

    pub fn associate_transit_gateway_connect_peer(
        &mut self,
        global_network_id: &str,
        transit_gateway_connect_peer_arn: &str,
        device_id: &str,
        link_id: Option<&str>,
    ) -> Result<&TransitGatewayConnectPeerAssociation, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        let assoc = TransitGatewayConnectPeerAssociation {
            transit_gateway_connect_peer_arn: transit_gateway_connect_peer_arn.to_string(),
            global_network_id: global_network_id.to_string(),
            device_id: device_id.to_string(),
            link_id: link_id.map(|s| s.to_string()),
            state: "AVAILABLE".to_string(),
        };

        self.transit_gateway_connect_peer_associations
            .insert(transit_gateway_connect_peer_arn.to_string(), assoc);
        Ok(self
            .transit_gateway_connect_peer_associations
            .get(transit_gateway_connect_peer_arn)
            .unwrap())
    }

    pub fn disassociate_transit_gateway_connect_peer(
        &mut self,
        global_network_id: &str,
        transit_gateway_connect_peer_arn: &str,
    ) -> Result<TransitGatewayConnectPeerAssociation, NetworkManagerError> {
        let assoc = self
            .transit_gateway_connect_peer_associations
            .get(transit_gateway_connect_peer_arn)
            .ok_or_else(
                || NetworkManagerError::TransitGatewayConnectPeerAssociationNotFound {
                    arn: transit_gateway_connect_peer_arn.to_string(),
                },
            )?;
        if assoc.global_network_id != global_network_id {
            return Err(
                NetworkManagerError::TransitGatewayConnectPeerAssociationNotFoundInNetwork {
                    arn: transit_gateway_connect_peer_arn.to_string(),
                    gn_id: global_network_id.to_string(),
                },
            );
        }
        Ok(self
            .transit_gateway_connect_peer_associations
            .remove(transit_gateway_connect_peer_arn)
            .unwrap())
    }

    pub fn get_transit_gateway_connect_peer_associations(
        &self,
        global_network_id: &str,
    ) -> Vec<&TransitGatewayConnectPeerAssociation> {
        self.transit_gateway_connect_peer_associations
            .values()
            .filter(|a| a.global_network_id == global_network_id)
            .collect()
    }

    // ── Route Analysis ──

    #[allow(clippy::too_many_arguments)]
    pub fn start_route_analysis(
        &mut self,
        global_network_id: &str,
        owner_account_id: &str,
        source_transit_gateway_arn: Option<&str>,
        source_transit_gateway_attachment_arn: Option<&str>,
        source_ip_address: Option<&str>,
        destination_transit_gateway_arn: Option<&str>,
        destination_transit_gateway_attachment_arn: Option<&str>,
        destination_ip_address: Option<&str>,
        include_return_path: bool,
        use_middleboxes: bool,
    ) -> Result<&RouteAnalysis, NetworkManagerError> {
        if !self.global_networks.contains_key(global_network_id) {
            return Err(NetworkManagerError::GlobalNetworkNotFound {
                id: global_network_id.to_string(),
            });
        }

        let id = format!("route-analysis-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let ra = RouteAnalysis {
            route_analysis_id: id.clone(),
            global_network_id: global_network_id.to_string(),
            owner_account_id: owner_account_id.to_string(),
            source_transit_gateway_arn: source_transit_gateway_arn.map(|s| s.to_string()),
            source_transit_gateway_attachment_arn: source_transit_gateway_attachment_arn
                .map(|s| s.to_string()),
            source_ip_address: source_ip_address.map(|s| s.to_string()),
            destination_transit_gateway_arn: destination_transit_gateway_arn.map(|s| s.to_string()),
            destination_transit_gateway_attachment_arn: destination_transit_gateway_attachment_arn
                .map(|s| s.to_string()),
            destination_ip_address: destination_ip_address.map(|s| s.to_string()),
            include_return_path,
            use_middleboxes,
            started_at: Utc::now(),
            status: "RUNNING".to_string(),
        };

        self.route_analyses.insert(id.clone(), ra);
        Ok(self.route_analyses.get(&id).unwrap())
    }

    pub fn get_route_analysis(
        &self,
        global_network_id: &str,
        route_analysis_id: &str,
    ) -> Result<&RouteAnalysis, NetworkManagerError> {
        let ra = self.route_analyses.get(route_analysis_id).ok_or_else(|| {
            NetworkManagerError::RouteAnalysisNotFound {
                id: route_analysis_id.to_string(),
            }
        })?;
        if ra.global_network_id != global_network_id {
            return Err(NetworkManagerError::RouteAnalysisNotFoundInNetwork {
                id: route_analysis_id.to_string(),
                gn_id: global_network_id.to_string(),
            });
        }
        Ok(ra)
    }

    // ── Tag helpers for new resource types ──

    fn find_resource_tags_mut(
        &mut self,
        resource_arn: &str,
    ) -> Option<&mut HashMap<String, String>> {
        for c in self.connections.values_mut() {
            if c.connection_arn == resource_arn {
                return Some(&mut c.tags);
            }
        }
        for att in self.attachments.values_mut() {
            if att.attachment_id == resource_arn {
                return Some(&mut att.tags);
            }
        }
        for cp in self.connect_peers.values_mut() {
            if cp.connect_peer_id == resource_arn {
                return Some(&mut cp.tags);
            }
        }
        None
    }
}
