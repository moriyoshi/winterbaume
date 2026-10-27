//! CloudFormation's SSM parameter handler, backed by the live SSM service state.
//! Contract: https://schema.cloudformation.us-east-1.amazonaws.com/aws-ssm-parameter.json

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;
use winterbaume_ssm::{state::SsmError, types::Parameter};

use crate::cfn_schema::ShapeContext;
use crate::handlers::CloudControlService;
use crate::state::CloudControlError;
use crate::types::{ManagedResource, OperationType, ResourceRequest};

pub(crate) const TYPE_NAME: &str = "AWS::SSM::Parameter";

fn invalid(message: impl Into<String>) -> CloudControlError {
    CloudControlError::InvalidRequest {
        message: message.into(),
    }
}

fn ssm_error(error: SsmError, name: &str) -> CloudControlError {
    match error {
        SsmError::ParameterNotFound(_) => CloudControlError::ResourceNotFound {
            type_name: TYPE_NAME.to_string(),
            identifier: name.to_string(),
        },
        SsmError::ParameterAlreadyExists => CloudControlError::AlreadyExists {
            type_name: TYPE_NAME.to_string(),
            identifier: name.to_string(),
        },
        other => invalid(other.to_string()),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ParameterModel {
    name: String,
    #[serde(rename = "Type")]
    parameter_type: String,
    value: String,
    #[serde(default = "default_data_type")]
    data_type: String,
    #[serde(default)]
    tags: HashMap<String, String>,
}

fn default_data_type() -> String {
    "text".to_string()
}

impl ParameterModel {
    fn parse(value: Value) -> Result<Self, CloudControlError> {
        let model: Self = serde_json::from_value(value).map_err(|e| invalid(e.to_string()))?;
        if model.name.is_empty() || model.value.is_empty() {
            return Err(invalid("Name and Value must not be empty"));
        }
        if !matches!(model.parameter_type.as_str(), "String" | "StringList") {
            return Err(invalid("Type must be String or StringList"));
        }
        if !matches!(model.data_type.as_str(), "text" | "aws:ec2:image") {
            return Err(invalid("DataType must be text or aws:ec2:image"));
        }
        Ok(model)
    }
}

fn read_model(parameter: &Parameter) -> ManagedResource {
    // Build the read model from SSM, never from DesiredState. In particular,
    // Description, Tier, AllowedPattern and Policies are write-only in CFN.
    ManagedResource {
        type_name: TYPE_NAME.to_string(),
        identifier: parameter.name.clone(),
        resource_model: json!({
            "Name": parameter.name, "Type": parameter.r#type, "Value": parameter.value,
            "Arn": parameter.arn, "DataType": parameter.data_type, "Tags": parameter.tags,
        })
        .to_string(),
    }
}

impl CloudControlService {
    pub(crate) async fn create_ssm_parameter(
        &self,
        desired: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let mut desired: Value =
            serde_json::from_str(desired).map_err(|e| invalid(e.to_string()))?;
        let object = desired
            .as_object_mut()
            .ok_or_else(|| invalid("DesiredState must be an object"))?;
        object
            .entry("Name")
            .or_insert_with(|| Value::String(format!("ssm-{}", Uuid::new_v4())));
        let model = ParameterModel::parse(desired)?;
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self.ssm.shared_state().get(ctx.account_id, ctx.region);
        let mut state = state.write().await;
        state
            .put_parameter(
                &model.name,
                &model.value,
                &model.parameter_type,
                ctx.account_id,
                ctx.region,
                false,
                model.tags,
            )
            .map_err(|e| ssm_error(e, &model.name))?;
        let parameter = state.parameters.get_mut(&model.name).unwrap();
        parameter.data_type = model.data_type;
        let resource = read_model(parameter);
        Ok(cc_state.record_operation(
            TYPE_NAME,
            &model.name,
            OperationType::Create,
            Some(resource.resource_model),
        ))
    }

    pub(crate) async fn get_ssm_parameter(
        &self,
        name: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ManagedResource, CloudControlError> {
        let state = self.ssm.shared_state().get(ctx.account_id, ctx.region);
        let state = state.read().await;
        state
            .get_parameter(name)
            .map(read_model)
            .map_err(|e| ssm_error(e, name))
    }

    pub(crate) async fn list_ssm_parameters(&self, ctx: &ShapeContext<'_>) -> Vec<ManagedResource> {
        let state = self.ssm.shared_state().get(ctx.account_id, ctx.region);
        let state = state.read().await;
        let mut resources: Vec<_> = state.parameters.values().map(read_model).collect();
        resources.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        resources
    }

    pub(crate) async fn update_ssm_parameter(
        &self,
        name: &str,
        patch: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self.ssm.shared_state().get(ctx.account_id, ctx.region);
        let mut state = state.write().await;
        let previous = state
            .get_parameter(name)
            .map_err(|e| ssm_error(e, name))?
            .clone();
        let mut desired: Value =
            serde_json::from_str(&read_model(&previous).resource_model).unwrap();
        patch_model(&mut desired, patch)?;
        let model = ParameterModel::parse(desired)?;
        // Validate the entire patch before touching SSM or recording a success.
        if model.name != name {
            return Err(invalid("Name is a create-only property"));
        }
        if model.value != previous.value
            || model.parameter_type != previous.r#type
            || model.data_type != previous.data_type
        {
            state
                .put_parameter(
                    name,
                    &model.value,
                    &model.parameter_type,
                    ctx.account_id,
                    ctx.region,
                    true,
                    HashMap::new(),
                )
                .map_err(|e| ssm_error(e, name))?;
        }
        state
            .resource_tags
            .insert(name.to_string(), model.tags.clone());
        let parameter = state.parameters.get_mut(name).unwrap();
        parameter.tags = model.tags;
        parameter.data_type = model.data_type;
        let resource = read_model(parameter);
        Ok(cc_state.record_operation(
            TYPE_NAME,
            name,
            OperationType::Update,
            Some(resource.resource_model),
        ))
    }

    pub(crate) async fn delete_ssm_parameter(
        &self,
        name: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self.ssm.shared_state().get(ctx.account_id, ctx.region);
        state
            .write()
            .await
            .delete_parameter(name)
            .map_err(|e| ssm_error(e, name))?;
        Ok(cc_state.record_operation(TYPE_NAME, name, OperationType::Delete, None))
    }
}

fn patch_model(model: &mut Value, document: &str) -> Result<(), CloudControlError> {
    let patches: Vec<Value> = serde_json::from_str(document).map_err(|e| invalid(e.to_string()))?;
    for patch in patches {
        let path = patch
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("Missing patch path"))?;
        let tokens: Vec<_> = path
            .strip_prefix('/')
            .ok_or_else(|| invalid("Invalid patch path"))?
            .split('/')
            .map(|s| s.replace("~1", "/").replace("~0", "~"))
            .collect();
        if matches!(tokens[0].as_str(), "Name" | "Arn") {
            return Err(invalid("Name and Arn cannot be updated"));
        }
        let allowed = [
            "Type",
            "Value",
            "DataType",
            "Tags",
            "Description",
            "Tier",
            "AllowedPattern",
            "Policies",
        ];
        if !allowed.contains(&tokens[0].as_str())
            || (tokens.len() > 1 && (tokens[0] != "Tags" || tokens.len() != 2))
        {
            return Err(invalid("Unsupported parameter property path"));
        }
        let parent = if tokens.len() == 2 {
            model.get_mut("Tags").and_then(Value::as_object_mut)
        } else {
            model.as_object_mut()
        }
        .ok_or_else(|| invalid("Patch parent is not an object"))?;
        let key = tokens.last().unwrap();
        match patch.get("op").and_then(Value::as_str) {
            Some("add" | "replace") => {
                let value = patch
                    .get("value")
                    .ok_or_else(|| invalid("Missing patch value"))?;
                parent.insert(key.clone(), value.clone());
            }
            Some("remove") => {
                parent.remove(key);
            }
            Some("test") => {
                let value = patch
                    .get("value")
                    .ok_or_else(|| invalid("Missing patch value"))?;
                if parent.get(key) != Some(value) {
                    return Err(invalid("Patch test failed"));
                }
            }
            _ => return Err(invalid("Unsupported or failed patch operation")),
        }
    }
    Ok(())
}
