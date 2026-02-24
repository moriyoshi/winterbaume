use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BackupGatewayState {
    pub gateways: HashMap<String, Gateway>,
    pub hypervisors: HashMap<String, Hypervisor>,
    pub virtual_machines: HashMap<String, VirtualMachine>,
}

#[derive(Debug, Error)]
pub enum BackupGatewayError {
    #[error("Gateway {arn} does not exist.")]
    GatewayNotFound { arn: String },

    #[error("Hypervisor {arn} does not exist.")]
    HypervisorNotFound { arn: String },

    #[error("VirtualMachine {arn} does not exist.")]
    VirtualMachineNotFound { arn: String },

    #[error("Resource {arn} does not exist.")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl BackupGatewayState {
    pub fn create_gateway(&mut self, gateway: Gateway) -> &Gateway {
        let arn = gateway.arn.clone();
        self.gateways.insert(arn.clone(), gateway);
        self.gateways.get(&arn).unwrap()
    }

    pub fn get_gateway(&self, arn: &str) -> Result<&Gateway, BackupGatewayError> {
        self.gateways
            .get(arn)
            .ok_or_else(|| BackupGatewayError::GatewayNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn update_gateway(
        &mut self,
        arn: &str,
        update: impl FnOnce(&mut Gateway),
    ) -> Result<&Gateway, BackupGatewayError> {
        let g = self
            .gateways
            .get_mut(arn)
            .ok_or_else(|| BackupGatewayError::GatewayNotFound {
                arn: arn.to_string(),
            })?;
        update(g);
        Ok(g)
    }

    pub fn delete_gateway(&mut self, arn: &str) -> Result<(), BackupGatewayError> {
        self.gateways
            .remove(arn)
            .ok_or_else(|| BackupGatewayError::GatewayNotFound {
                arn: arn.to_string(),
            })?;
        Ok(())
    }

    pub fn list_gateways(&self) -> Vec<&Gateway> {
        let mut items: Vec<&Gateway> = self.gateways.values().collect();
        items.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        items
    }

    pub fn import_hypervisor(&mut self, hypervisor: Hypervisor) -> &Hypervisor {
        let arn = hypervisor.arn.clone();
        self.hypervisors.insert(arn.clone(), hypervisor);
        self.hypervisors.get(&arn).unwrap()
    }

    pub fn get_hypervisor(&self, arn: &str) -> Result<&Hypervisor, BackupGatewayError> {
        self.hypervisors
            .get(arn)
            .ok_or_else(|| BackupGatewayError::HypervisorNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn update_hypervisor(
        &mut self,
        arn: &str,
        update: impl FnOnce(&mut Hypervisor),
    ) -> Result<&Hypervisor, BackupGatewayError> {
        let h = self.hypervisors.get_mut(arn).ok_or_else(|| {
            BackupGatewayError::HypervisorNotFound {
                arn: arn.to_string(),
            }
        })?;
        update(h);
        Ok(h)
    }

    pub fn delete_hypervisor(&mut self, arn: &str) -> Result<(), BackupGatewayError> {
        self.hypervisors
            .remove(arn)
            .ok_or_else(|| BackupGatewayError::HypervisorNotFound {
                arn: arn.to_string(),
            })?;
        // Also remove dependent VMs
        self.virtual_machines
            .retain(|_, vm| vm.hypervisor_arn != arn);
        Ok(())
    }

    pub fn list_hypervisors(&self) -> Vec<&Hypervisor> {
        let mut items: Vec<&Hypervisor> = self.hypervisors.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn list_virtual_machines(&self, hypervisor_arn: Option<&str>) -> Vec<&VirtualMachine> {
        let mut items: Vec<&VirtualMachine> = self
            .virtual_machines
            .values()
            .filter(|vm| hypervisor_arn.is_none_or(|a| vm.hypervisor_arn == a))
            .collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn get_virtual_machine(&self, arn: &str) -> Result<&VirtualMachine, BackupGatewayError> {
        self.virtual_machines
            .get(arn)
            .ok_or_else(|| BackupGatewayError::VirtualMachineNotFound {
                arn: arn.to_string(),
            })
    }

    fn locate_tags_mut(&mut self, arn: &str) -> Option<&mut HashMap<String, String>> {
        if let Some(g) = self.gateways.values_mut().find(|g| g.arn == arn) {
            return Some(&mut g.tags);
        }
        if let Some(h) = self.hypervisors.values_mut().find(|h| h.arn == arn) {
            return Some(&mut h.tags);
        }
        None
    }

    fn locate_tags(&self, arn: &str) -> Option<&HashMap<String, String>> {
        if let Some(g) = self.gateways.values().find(|g| g.arn == arn) {
            return Some(&g.tags);
        }
        if let Some(h) = self.hypervisors.values().find(|h| h.arn == arn) {
            return Some(&h.tags);
        }
        None
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), BackupGatewayError> {
        let bag =
            self.locate_tags_mut(arn)
                .ok_or_else(|| BackupGatewayError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        for (k, v) in tags {
            bag.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) -> Result<(), BackupGatewayError> {
        let bag =
            self.locate_tags_mut(arn)
                .ok_or_else(|| BackupGatewayError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        for k in keys {
            bag.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, BackupGatewayError> {
        self.locate_tags(arn)
            .cloned()
            .ok_or_else(|| BackupGatewayError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }
}
