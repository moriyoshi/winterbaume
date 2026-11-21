//! CloudFormation's CloudFront function handler, backed by the live
//! CloudFront service state.
//! Contract: https://schema.cloudformation.us-east-1.amazonaws.com/aws-cloudfront-function.json

use std::collections::HashMap;

use serde_json::{Value, json};
use winterbaume_cloudfront::model::{
    FunctionConfig, KeyValueStoreAssociation, KeyValueStoreAssociations,
};
use winterbaume_cloudfront::state::{CloudFrontError, CloudFrontState};
use winterbaume_cloudfront::types::CloudFrontFunctionData;

use crate::cfn_schema::ShapeContext;
use crate::cfn_validate::{Schema, validate};
use crate::cloudfront_response_headers_policy::CLOUDFRONT_REGION;
use crate::handlers::CloudControlService;
use crate::json_patch;
use crate::state::CloudControlError;
use crate::types::{ManagedResource, OperationType, ResourceRequest};

pub(crate) const TYPE_NAME: &str = "AWS::CloudFront::Function";

/// Transcribed from the registry schema (checked 2026-11-06).
const SCHEMA: Schema = Schema::Object {
    properties: &[
        ("AutoPublish", Schema::Boolean),
        ("FunctionARN", Schema::String),
        ("FunctionCode", Schema::String),
        (
            "FunctionConfig",
            Schema::Object {
                properties: &[
                    ("Comment", Schema::String),
                    ("Runtime", Schema::String),
                    (
                        "KeyValueStoreAssociations",
                        Schema::Array(&Schema::Object {
                            properties: &[("KeyValueStoreARN", Schema::String)],
                            required: &["KeyValueStoreARN"],
                        }),
                    ),
                ],
                required: &["Comment", "Runtime"],
            },
        ),
        (
            "FunctionMetadata",
            Schema::Object {
                properties: &[("FunctionARN", Schema::String)],
                required: &[],
            },
        ),
        ("Name", Schema::String),
        ("Stage", Schema::String),
        (
            "Tags",
            Schema::Array(&Schema::Object {
                properties: &[("Key", Schema::String), ("Value", Schema::String)],
                required: &["Key", "Value"],
            }),
        ),
    ],
    required: &["Name", "FunctionConfig", "FunctionCode"],
};

fn invalid(message: impl Into<String>) -> CloudControlError {
    CloudControlError::InvalidRequest {
        message: message.into(),
    }
}

fn not_found(identifier: &str) -> CloudControlError {
    CloudControlError::ResourceNotFound {
        type_name: TYPE_NAME.to_string(),
        identifier: identifier.to_string(),
    }
}

fn cloudfront_error(error: CloudFrontError, identifier: &str) -> CloudControlError {
    match error {
        CloudFrontError::NoSuchFunctionExists(_) => not_found(identifier),
        CloudFrontError::FunctionAlreadyExists(_) => CloudControlError::AlreadyExists {
            type_name: TYPE_NAME.to_string(),
            identifier: identifier.to_string(),
        },
        other => invalid(other.to_string()),
    }
}

fn function_arn(account_id: &str, name: &str) -> String {
    format!("arn:aws:cloudfront::{account_id}:function/{name}")
}

/// The function whose ARN, the primary identifier, is `arn`.
fn lookup<'a>(
    state: &'a CloudFrontState,
    arn: &str,
) -> Result<&'a CloudFrontFunctionData, CloudControlError> {
    let name = arn
        .rsplit_once(":function/")
        .map(|(_, name)| name)
        .ok_or_else(|| not_found(arn))?;
    state
        .get_function(name)
        .ok()
        .filter(|f| f.arn == arn)
        .ok_or_else(|| not_found(arn))
}

/// The function's desired properties, parsed from a validated model.
struct FunctionModel {
    name: String,
    code: String,
    config: FunctionConfig,
    auto_publish: bool,
    tags: HashMap<String, String>,
}

impl FunctionModel {
    fn parse(model: &Value) -> Result<Self, CloudControlError> {
        validate(&SCHEMA, model)
            .map_err(|message| CloudControlError::ValidationFailed { message })?;
        let text = |v: &Value| v.as_str().unwrap_or_default().to_string();
        let cfn_config = &model["FunctionConfig"];
        let associations: Vec<KeyValueStoreAssociation> = cfn_config
            .get("KeyValueStoreAssociations")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|a| KeyValueStoreAssociation {
                key_value_store_a_r_n: text(&a["KeyValueStoreARN"]),
            })
            .collect();
        let config = FunctionConfig {
            comment: text(&cfn_config["Comment"]),
            runtime: text(&cfn_config["Runtime"]),
            key_value_store_associations: (!associations.is_empty()).then(|| {
                KeyValueStoreAssociations {
                    quantity: associations.len() as i32,
                    items: Some(associations.into()),
                }
            }),
        };
        let tags = model
            .get("Tags")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|t| (text(&t["Key"]), text(&t["Value"])))
            .collect();
        Ok(Self {
            name: text(&model["Name"]),
            code: text(&model["FunctionCode"]),
            config,
            auto_publish: model.get("AutoPublish") == Some(&Value::Bool(true)),
            tags,
        })
    }
}

fn read_model(function: &CloudFrontFunctionData) -> Value {
    // Build the read model from CloudFront. AutoPublish is write-only. The
    // read handler reports the DEVELOPMENT code and config, and `Stage` is
    // LIVE once the function has ever been published, even when DEVELOPMENT
    // has since moved ahead of LIVE (captured from a live account on
    // 2026-10-27).
    let mut config = json!({
        "Comment": function.config.comment,
        "Runtime": function.config.runtime,
    });
    if let Some(associations) = function
        .config
        .key_value_store_associations
        .as_ref()
        .and_then(|a| a.items.as_ref())
    {
        config["KeyValueStoreAssociations"] = associations
            .items
            .iter()
            .map(|a| json!({"KeyValueStoreARN": a.key_value_store_a_r_n}))
            .collect();
    }
    let mut tags: Vec<_> = function.tags.iter().collect();
    tags.sort();
    json!({
        "Name": function.name,
        "FunctionARN": function.arn,
        "FunctionMetadata": {"FunctionARN": function.arn},
        "FunctionCode": String::from_utf8_lossy(&function.code),
        "FunctionConfig": config,
        "Stage": if function.live.is_some() { "LIVE" } else { "DEVELOPMENT" },
        "Tags": tags
            .into_iter()
            .map(|(k, v)| json!({"Key": k, "Value": v}))
            .collect::<Vec<_>>(),
    })
}

fn managed(function: &CloudFrontFunctionData) -> ManagedResource {
    ManagedResource {
        type_name: TYPE_NAME.to_string(),
        identifier: function.arn.clone(),
        resource_model: read_model(function).to_string(),
    }
}

impl CloudControlService {
    pub(crate) async fn create_cloudfront_function(
        &self,
        desired: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let desired: Value = serde_json::from_str(desired).map_err(|e| invalid(e.to_string()))?;
        let model = FunctionModel::parse(&desired)?;
        let arn = function_arn(ctx.account_id, &model.name);
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let mut state = state.write().await;
        state
            .create_function(
                &model.name,
                model.config,
                model.code.into_bytes(),
                ctx.account_id,
            )
            .map_err(|e| cloudfront_error(e, &arn))?;
        if model.auto_publish {
            state
                .publish_function(&model.name, None)
                .map_err(|e| cloudfront_error(e, &arn))?;
        }
        let function = state.functions.get_mut(&model.name).unwrap();
        function.tags = model.tags;
        let resource = managed(function);
        Ok(cc_state.record_operation(
            TYPE_NAME,
            &resource.identifier,
            OperationType::Create,
            Some(resource.resource_model),
        ))
    }

    pub(crate) async fn get_cloudfront_function(
        &self,
        arn: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ManagedResource, CloudControlError> {
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let state = state.read().await;
        lookup(&state, arn).map(managed)
    }

    pub(crate) async fn list_cloudfront_functions(
        &self,
        ctx: &ShapeContext<'_>,
    ) -> Vec<ManagedResource> {
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let state = state.read().await;
        let mut resources: Vec<_> = state.list_functions().into_iter().map(managed).collect();
        resources.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        resources
    }

    pub(crate) async fn update_cloudfront_function(
        &self,
        arn: &str,
        patch: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let mut state = state.write().await;
        let current = lookup(&state, arn)?;
        let name = current.name.clone();
        let previous = read_model(current);
        let mut model = previous.clone();
        json_patch::apply(&mut model, patch).map_err(invalid)?;
        if model.get("Name") != previous.get("Name") {
            return Err(invalid(
                "Invalid patch update: createOnlyProperties [/properties/Name] cannot be updated",
            ));
        }
        for property in ["FunctionARN", "FunctionMetadata", "Stage"] {
            if model.get(property) != previous.get(property) {
                return Err(invalid(format!(
                    "Invalid patch update: readOnlyProperties [/properties/{property}] cannot be updated"
                )));
            }
        }
        // Validate the entire patched model before touching CloudFront.
        let desired = FunctionModel::parse(&model)?;
        if model.get("FunctionCode") != previous.get("FunctionCode")
            || model.get("FunctionConfig") != previous.get("FunctionConfig")
        {
            // UpdateFunction changes DEVELOPMENT only; LIVE keeps the code it
            // was last published with unless AutoPublish is set.
            state
                .update_function(&name, desired.config, desired.code.into_bytes(), None)
                .map_err(|e| cloudfront_error(e, arn))?;
        }
        if desired.auto_publish {
            state
                .publish_function(&name, None)
                .map_err(|e| cloudfront_error(e, arn))?;
        }
        let function = state.functions.get_mut(&name).unwrap();
        function.tags = desired.tags;
        let resource = managed(function);
        Ok(cc_state.record_operation(
            TYPE_NAME,
            arn,
            OperationType::Update,
            Some(resource.resource_model),
        ))
    }

    pub(crate) async fn delete_cloudfront_function(
        &self,
        arn: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let mut state = state.write().await;
        let name = lookup(&state, arn)?.name.clone();
        state
            .delete_function(&name, None)
            .map_err(|e| cloudfront_error(e, arn))?;
        Ok(cc_state.record_operation(TYPE_NAME, arn, OperationType::Delete, None))
    }
}
