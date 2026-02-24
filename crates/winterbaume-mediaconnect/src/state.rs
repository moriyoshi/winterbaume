use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct MediaConnectState {
    pub flows: HashMap<String, Flow>,
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum MediaConnectError {
    #[error("A flow with the name '{name}' already exists.")]
    DuplicateFlowName { name: String },

    #[error("Flow with ARN '{flow_arn}' was not found.")]
    FlowNotFound { flow_arn: String },

    #[error("Flow not found: {flow_arn}")]
    FlowNotFoundShort { flow_arn: String },

    #[error("Output with ARN '{output_arn}' was not found.")]
    OutputNotFound { output_arn: String },

    #[error("Source with ARN '{source_arn}' was not found.")]
    SourceNotFound { source_arn: String },

    #[error("VPC interface '{vpc_interface_name}' was not found.")]
    VpcInterfaceNotFound { vpc_interface_name: String },

    #[error("Entitlement '{entitlement_arn}' not found.")]
    EntitlementNotFound { entitlement_arn: String },
}

impl MediaConnectState {
    pub fn create_flow(
        &mut self,
        name: &str,
        availability_zone: &str,
        description: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Flow, MediaConnectError> {
        // Check for duplicate name
        for flow in self.flows.values() {
            if flow.name == name {
                return Err(MediaConnectError::DuplicateFlowName {
                    name: name.to_string(),
                });
            }
        }

        let flow_arn = format!(
            "arn:aws:mediaconnect:{region}:{account_id}:flow:{id}:{name}",
            id = uuid::Uuid::new_v4(),
        );

        let az = if availability_zone.is_empty() {
            format!("{region}a")
        } else {
            availability_zone.to_string()
        };

        let flow = Flow {
            flow_arn: flow_arn.clone(),
            name: name.to_string(),
            description: description.to_string(),
            availability_zone: az,
            status: "STANDBY".to_string(),
            created_at: Utc::now(),
            outputs: vec![],
            sources: vec![],
            vpc_interfaces: vec![],
            tags: HashMap::new(),
            entitlements: vec![],
        };

        self.flows.insert(flow_arn.clone(), flow);
        Ok(self.flows.get(&flow_arn).unwrap())
    }

    pub fn describe_flow(&self, flow_arn: &str) -> Result<&Flow, MediaConnectError> {
        self.flows
            .get(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })
    }

    pub fn delete_flow(&mut self, flow_arn: &str) -> Result<Flow, MediaConnectError> {
        self.flows
            .remove(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })
    }

    pub fn list_flows(&self) -> Vec<&Flow> {
        self.flows.values().collect()
    }

    pub fn start_flow(&mut self, flow_arn: &str) -> Result<&Flow, MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        flow.status = "ACTIVE".to_string();
        Ok(flow)
    }

    pub fn stop_flow(&mut self, flow_arn: &str) -> Result<&Flow, MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        flow.status = "STANDBY".to_string();
        Ok(flow)
    }

    pub fn update_flow(
        &mut self,
        flow_arn: &str,
        description: Option<&str>,
    ) -> Result<&Flow, MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        if let Some(desc) = description {
            flow.description = desc.to_string();
        }
        Ok(flow)
    }

    pub fn add_flow_outputs(
        &mut self,
        flow_arn: &str,
        outputs: Vec<FlowOutput>,
    ) -> Result<(&Flow, Vec<FlowOutput>), MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let added = outputs.clone();
        flow.outputs.extend(outputs);
        Ok((flow, added))
    }

    pub fn remove_flow_output(
        &mut self,
        flow_arn: &str,
        output_arn: &str,
    ) -> Result<&Flow, MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let before = flow.outputs.len();
        flow.outputs.retain(|o| o.output_arn != output_arn);
        if flow.outputs.len() == before {
            return Err(MediaConnectError::OutputNotFound {
                output_arn: output_arn.to_string(),
            });
        }
        Ok(flow)
    }

    pub fn update_flow_output(
        &mut self,
        flow_arn: &str,
        output_arn: &str,
        description: Option<&str>,
        port: Option<i32>,
        protocol: Option<&str>,
        destination: Option<&str>,
    ) -> Result<(&Flow, &FlowOutput), MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let output = flow
            .outputs
            .iter_mut()
            .find(|o| o.output_arn == output_arn)
            .ok_or_else(|| MediaConnectError::OutputNotFound {
                output_arn: output_arn.to_string(),
            })?;
        if let Some(d) = description {
            output.description = d.to_string();
        }
        if let Some(p) = port {
            output.port = p;
        }
        if let Some(pr) = protocol {
            output.protocol = pr.to_string();
        }
        if let Some(dest) = destination {
            output.destination = dest.to_string();
        }
        // Re-borrow to satisfy borrow checker
        let flow = self.flows.get(flow_arn).unwrap();
        let output = flow
            .outputs
            .iter()
            .find(|o| o.output_arn == output_arn)
            .unwrap();
        Ok((flow, output))
    }

    pub fn add_flow_sources(
        &mut self,
        flow_arn: &str,
        sources: Vec<FlowSource>,
    ) -> Result<(&Flow, Vec<FlowSource>), MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let added = sources.clone();
        flow.sources.extend(sources);
        Ok((flow, added))
    }

    pub fn remove_flow_source(
        &mut self,
        flow_arn: &str,
        source_arn: &str,
    ) -> Result<&Flow, MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let before = flow.sources.len();
        flow.sources.retain(|s| s.source_arn != source_arn);
        if flow.sources.len() == before {
            return Err(MediaConnectError::SourceNotFound {
                source_arn: source_arn.to_string(),
            });
        }
        Ok(flow)
    }

    pub fn update_flow_source(
        &mut self,
        flow_arn: &str,
        source_arn: &str,
        description: Option<&str>,
        protocol: Option<&str>,
        whitelist_cidr: Option<&str>,
    ) -> Result<(&Flow, &FlowSource), MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let source = flow
            .sources
            .iter_mut()
            .find(|s| s.source_arn == source_arn)
            .ok_or_else(|| MediaConnectError::SourceNotFound {
                source_arn: source_arn.to_string(),
            })?;
        if let Some(d) = description {
            source.description = d.to_string();
        }
        if let Some(pr) = protocol {
            source.protocol = pr.to_string();
        }
        if let Some(wc) = whitelist_cidr {
            source.whitelist_cidr = wc.to_string();
        }
        let flow = self.flows.get(flow_arn).unwrap();
        let source = flow
            .sources
            .iter()
            .find(|s| s.source_arn == source_arn)
            .unwrap();
        Ok((flow, source))
    }

    pub fn add_flow_vpc_interfaces(
        &mut self,
        flow_arn: &str,
        vpc_interfaces: Vec<FlowVpcInterface>,
    ) -> Result<(&Flow, Vec<FlowVpcInterface>), MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let added = vpc_interfaces.clone();
        flow.vpc_interfaces.extend(vpc_interfaces);
        Ok((flow, added))
    }

    pub fn remove_flow_vpc_interface(
        &mut self,
        flow_arn: &str,
        vpc_interface_name: &str,
    ) -> Result<&Flow, MediaConnectError> {
        let flow = self
            .flows
            .get_mut(flow_arn)
            .ok_or_else(|| MediaConnectError::FlowNotFound {
                flow_arn: flow_arn.to_string(),
            })?;
        let before = flow.vpc_interfaces.len();
        flow.vpc_interfaces.retain(|v| v.name != vpc_interface_name);
        if flow.vpc_interfaces.len() == before {
            return Err(MediaConnectError::VpcInterfaceNotFound {
                vpc_interface_name: vpc_interface_name.to_string(),
            });
        }
        Ok(flow)
    }

    // --- Entitlement operations ---

    pub fn grant_flow_entitlements(
        &mut self,
        flow_arn: &str,
        entitlements: Vec<(String, String, Vec<String>)>,
        account_id: &str,
        region: &str,
    ) -> Result<Vec<FlowEntitlement>, MediaConnectError> {
        let flow =
            self.flows
                .get_mut(flow_arn)
                .ok_or_else(|| MediaConnectError::FlowNotFoundShort {
                    flow_arn: flow_arn.to_string(),
                })?;

        let mut new_entitlements = Vec::new();
        for (name, description, subscribers) in entitlements {
            let arn = format!(
                "arn:aws:mediaconnect:{region}:{account_id}:entitlement:{id}:{name}",
                id = uuid::Uuid::new_v4(),
            );
            let ent = FlowEntitlement {
                entitlement_arn: arn,
                name,
                description,
                subscribers,
            };
            flow.entitlements.push(ent.clone());
            new_entitlements.push(ent);
        }

        Ok(new_entitlements)
    }

    pub fn revoke_flow_entitlement(
        &mut self,
        flow_arn: &str,
        entitlement_arn: &str,
    ) -> Result<(String, String), MediaConnectError> {
        let flow =
            self.flows
                .get_mut(flow_arn)
                .ok_or_else(|| MediaConnectError::FlowNotFoundShort {
                    flow_arn: flow_arn.to_string(),
                })?;

        let before = flow.entitlements.len();
        flow.entitlements
            .retain(|e| e.entitlement_arn != entitlement_arn);
        if flow.entitlements.len() == before {
            return Err(MediaConnectError::EntitlementNotFound {
                entitlement_arn: entitlement_arn.to_string(),
            });
        }

        Ok((flow_arn.to_string(), entitlement_arn.to_string()))
    }

    pub fn update_flow_entitlement(
        &mut self,
        flow_arn: &str,
        entitlement_arn: &str,
        description: Option<&str>,
        subscribers: Option<Vec<String>>,
    ) -> Result<FlowEntitlement, MediaConnectError> {
        let flow =
            self.flows
                .get_mut(flow_arn)
                .ok_or_else(|| MediaConnectError::FlowNotFoundShort {
                    flow_arn: flow_arn.to_string(),
                })?;

        let ent = flow
            .entitlements
            .iter_mut()
            .find(|e| e.entitlement_arn == entitlement_arn)
            .ok_or_else(|| MediaConnectError::EntitlementNotFound {
                entitlement_arn: entitlement_arn.to_string(),
            })?;

        if let Some(d) = description {
            ent.description = d.to_string();
        }
        if let Some(s) = subscribers {
            ent.subscribers = s;
        }

        Ok(ent.clone())
    }

    // --- Tag operations ---

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.resource_tags.entry(arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> HashMap<String, String> {
        self.resource_tags.get(arn).cloned().unwrap_or_default()
    }
}
