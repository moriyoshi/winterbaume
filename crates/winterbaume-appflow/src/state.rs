use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AppFlowState {
    /// Flows keyed by name.
    pub flows: HashMap<String, Flow>,
}

#[derive(Debug, Error)]
pub enum AppFlowError {
    #[error("Flow {name} not found")]
    FlowNotFound { name: String },

    #[error("Flow {name} already exists")]
    FlowAlreadyExists { name: String },

    #[error("Resource {arn} not found")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl AppFlowState {
    pub fn create_flow(&mut self, flow: Flow) -> Result<&Flow, AppFlowError> {
        if self.flows.contains_key(&flow.flow_name) {
            return Err(AppFlowError::FlowAlreadyExists {
                name: flow.flow_name.clone(),
            });
        }
        let name = flow.flow_name.clone();
        self.flows.insert(name.clone(), flow);
        Ok(self.flows.get(&name).unwrap())
    }

    pub fn get_flow(&self, name: &str) -> Result<&Flow, AppFlowError> {
        self.flows
            .get(name)
            .ok_or_else(|| AppFlowError::FlowNotFound {
                name: name.to_string(),
            })
    }

    pub fn update_flow(
        &mut self,
        name: &str,
        update: impl FnOnce(&mut Flow),
    ) -> Result<&Flow, AppFlowError> {
        let flow = self
            .flows
            .get_mut(name)
            .ok_or_else(|| AppFlowError::FlowNotFound {
                name: name.to_string(),
            })?;
        update(flow);
        Ok(flow)
    }

    pub fn delete_flow(&mut self, name: &str) -> Result<(), AppFlowError> {
        self.flows.remove(name).ok_or(AppFlowError::FlowNotFound {
            name: name.to_string(),
        })?;
        Ok(())
    }

    pub fn list_flows(&self) -> Vec<&Flow> {
        let mut flows: Vec<&Flow> = self.flows.values().collect();
        flows.sort_by(|a, b| a.flow_name.cmp(&b.flow_name));
        flows
    }

    pub fn flow_by_arn_mut(&mut self, arn: &str) -> Result<&mut Flow, AppFlowError> {
        self.flows
            .values_mut()
            .find(|f| f.flow_arn == arn)
            .ok_or_else(|| AppFlowError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn flow_by_arn(&self, arn: &str) -> Result<&Flow, AppFlowError> {
        self.flows
            .values()
            .find(|f| f.flow_arn == arn)
            .ok_or_else(|| AppFlowError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AppFlowError> {
        let flow = self.flow_by_arn_mut(arn)?;
        for (k, v) in tags {
            flow.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) -> Result<(), AppFlowError> {
        let flow = self.flow_by_arn_mut(arn)?;
        for k in keys {
            flow.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, AppFlowError> {
        Ok(self.flow_by_arn(arn)?.tags.clone())
    }
}
