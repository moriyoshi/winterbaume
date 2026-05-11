//! Terraform converter for Connect resources.
//!
//! `ConnectInstanceTfModel` is generated from `specs/connect.toml`. The
//! ARN template, status / identity-management-type defaults, and the
//! pass-through `contact_flow_logs_enabled` / `early_media_enabled`
//! constants are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_connect::ConnectService;
use winterbaume_connect::views::{ConnectInstanceView, ConnectStateView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::connect as connect_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_connect_instance
// ---------------------------------------------------------------------------

/// Converts `aws_connect_instance` Terraform resources to/from Connect state.
pub struct AwsConnectInstanceConverter {
    service: Arc<ConnectService>,
}

impl AwsConnectInstanceConverter {
    pub fn new(service: Arc<ConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConnectInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_connect_instance"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsConnectInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: connect_gen::ConnectInstanceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_connect_instance", e))?;

        let id = model.id.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:connect:{}:{}:instance/{}",
                region, ctx.default_account_id, id
            )
        });
        let identity_management_type = model
            .identity_management_type
            .unwrap_or_else(|| "CONNECT_MANAGED".to_string());
        let instance_status = model
            .instance_status
            .unwrap_or_else(|| "ACTIVE".to_string());
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());

        let inst_view = ConnectInstanceView {
            id: id.clone(),
            arn,
            identity_management_type,
            instance_alias: model.instance_alias,
            instance_status,
            created_time,
            inbound_calls_enabled: model.inbound_calls_enabled,
            outbound_calls_enabled: model.outbound_calls_enabled,
            tags: model.tags,
        };

        let state_view = ConnectStateView {
            instances: {
                let mut m = HashMap::new();
                m.insert(id, inst_view);
                m
            },
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for inst in view.instances.values() {
            let attrs = serde_json::json!({
                "id": inst.id,
                "arn": inst.arn,
                "identity_management_type": inst.identity_management_type,
                "instance_alias": inst.instance_alias,
                "status": inst.instance_status,
                "created_time": inst.created_time,
                "inbound_calls_enabled": inst.inbound_calls_enabled,
                "outbound_calls_enabled": inst.outbound_calls_enabled,
                "contact_flow_logs_enabled": false,
                "early_media_enabled": true,
                "tags": inst.tags,
            });
            results.push(ExtractedResource {
                name: inst
                    .instance_alias
                    .clone()
                    .unwrap_or_else(|| inst.id.clone()),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Warning-only converters
//
// The following Terraform resource types do not have a corresponding state
// slot in `winterbaume_connect`. Inject validates the TF attributes against
// the generated model (so malformed input still fails fast), emits a
// warning, and otherwise no-ops. Extract returns an empty list.
// ---------------------------------------------------------------------------

macro_rules! connect_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<ConnectService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<ConnectService>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move { self.do_inject(instance, ctx).await })
            }

            fn extract<'a>(
                &'a self,
                _ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move { Ok(vec![]) })
            }
        }

        impl $struct_name {
            async fn do_inject(
                &self,
                instance: &ResourceInstance,
                ctx: &ConversionContext,
            ) -> Result<ConversionResult, ConversionError> {
                let attrs = &instance.attributes;
                let region = extract_region(attrs, &ctx.default_region);
                let _model: connect_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                eprintln!("warning: {}: {}", $resource_type, $warn_msg);
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

connect_warning_only_converter! {
    struct_name = AwsConnectBotAssociationConverter,
    resource_type = "aws_connect_bot_association",
    model_type = ConnectBotAssociationTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectContactFlowConverter,
    resource_type = "aws_connect_contact_flow",
    model_type = ConnectContactFlowTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectContactFlowModuleConverter,
    resource_type = "aws_connect_contact_flow_module",
    model_type = ConnectContactFlowModuleTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectHoursOfOperationConverter,
    resource_type = "aws_connect_hours_of_operation",
    model_type = ConnectHoursOfOperationTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectInstanceStorageConfigConverter,
    resource_type = "aws_connect_instance_storage_config",
    model_type = ConnectInstanceStorageConfigTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectLambdaFunctionAssociationConverter,
    resource_type = "aws_connect_lambda_function_association",
    model_type = ConnectLambdaFunctionAssociationTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectPhoneNumberConverter,
    resource_type = "aws_connect_phone_number",
    model_type = ConnectPhoneNumberTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectQueueConverter,
    resource_type = "aws_connect_queue",
    model_type = ConnectQueueTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectQuickConnectConverter,
    resource_type = "aws_connect_quick_connect",
    model_type = ConnectQuickConnectTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectRoutingProfileConverter,
    resource_type = "aws_connect_routing_profile",
    model_type = ConnectRoutingProfileTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectSecurityProfileConverter,
    resource_type = "aws_connect_security_profile",
    model_type = ConnectSecurityProfileTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectUserConverter,
    resource_type = "aws_connect_user",
    model_type = ConnectUserTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectUserHierarchyGroupConverter,
    resource_type = "aws_connect_user_hierarchy_group",
    model_type = ConnectUserHierarchyGroupTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectUserHierarchyStructureConverter,
    resource_type = "aws_connect_user_hierarchy_structure",
    model_type = ConnectUserHierarchyStructureTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}

connect_warning_only_converter! {
    struct_name = AwsConnectVocabularyConverter,
    resource_type = "aws_connect_vocabulary",
    model_type = ConnectVocabularyTfModel,
    warn_msg = "no state slot in winterbaume_connect; inject is a no-op",
}
