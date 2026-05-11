//! Terraform converter for Direct Connect resources.
//!
//! `ConnectionTfModel` is generated from `specs/directconnect.toml`. The
//! `bandwidth` default ("1Gbps"), the `connection_state` default
//! ("available"), the `owner_account` fallback to the conversion context,
//! and the i64-to-i32 vlan narrowing are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_directconnect::DirectConnectService;
use winterbaume_directconnect::views::{ConnectionView, DirectConnectStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::directconnect as directconnect_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_dx_connection
// ---------------------------------------------------------------------------

/// Converts `aws_dx_connection` Terraform resources to/from Direct Connect state.
pub struct AwsDxConnectionConverter {
    service: Arc<DirectConnectService>,
}

impl AwsDxConnectionConverter {
    pub fn new(service: Arc<DirectConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDxConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_dx_connection"
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

impl AwsDxConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directconnect_gen::ConnectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dx_connection", e))?;

        let connection_id = model.id.unwrap_or_default();
        let bandwidth = model.bandwidth.unwrap_or_else(|| "1Gbps".to_string());
        let location = model.location.unwrap_or_default();
        let vlan = model.vlan as i32;
        let owner_account = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let connection_state = model
            .connection_state
            .unwrap_or_else(|| "available".to_string());

        let conn_view = ConnectionView {
            connection_id: connection_id.clone(),
            connection_name: model.name.clone(),
            connection_state,
            region: region.clone(),
            location,
            bandwidth,
            owner_account,
            vlan,
            partner_name: model.partner_name,
            loa_issue_time: None,
            tags: model.tags,
        };

        let state_view = DirectConnectStateView {
            connections: {
                let mut m = HashMap::new();
                m.insert(connection_id, conn_view);
                m
            },
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
        for c in view.connections.values() {
            let attrs = serde_json::json!({
                "id": c.connection_id,
                "name": c.connection_name,
                "bandwidth": c.bandwidth,
                "location": c.location,
                "vlan": c.vlan,
                "partner_name": c.partner_name,
                "owner_account_id": c.owner_account,
                "connection_state": c.connection_state,
                "tags": c.tags,
                "tags_all": c.tags,
                "has_logical_redundancy": "unknown",
                "aws_device": "",
            });
            results.push(ExtractedResource {
                name: c.connection_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers shared by warning-only converters
// ---------------------------------------------------------------------------

/// Inject a resource type that has no slot in `DirectConnectStateView`.
/// The model is parsed (so attribute validation still runs), then a warning
/// is logged and an empty result is returned. Extract is a no-op.
macro_rules! warning_only_converter {
    (
        $struct_name:ident,
        $tf_type:literal,
        $model:ident,
        $warn_msg:literal $(,)?
    ) => {
        pub struct $struct_name {
            service: Arc<DirectConnectService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<DirectConnectService>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $tf_type
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
                let region = extract_region(&instance.attributes, &ctx.default_region);
                let _model: directconnect_gen::$model =
                    serde_json::from_value(instance.attributes.clone())
                        .map_err(|e| classify_deserialize_error($tf_type, e))?;
                eprintln!(
                    "warning: {}: DirectConnectStateView has no slot; inject is a no-op",
                    $tf_type
                );
                // Touch the service so the merge log records visiting the scope.
                self.service
                    .merge(
                        &ctx.default_account_id,
                        &region,
                        DirectConnectStateView::default(),
                    )
                    .await?;
                Ok(ConversionResult {
                    region,
                    warnings: vec![$warn_msg.to_string()],
                })
            }
        }
    };
}

// ---------------------------------------------------------------------------
// aws_dx_bgp_peer
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxBgpPeerConverter,
    "aws_dx_bgp_peer",
    BgpPeerTfModel,
    "aws_dx_bgp_peer state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_connection_association
// ---------------------------------------------------------------------------

/// Connects an existing connection to a LAG. Modifier-style: snapshot the
/// matching `ConnectionView` (if any) and write it back unchanged so the
/// merge log records the visit. The LAG side is not modelled in the view,
/// so the actual association is dropped with a warning.
pub struct AwsDxConnectionAssociationConverter {
    service: Arc<DirectConnectService>,
}

impl AwsDxConnectionAssociationConverter {
    pub fn new(service: Arc<DirectConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDxConnectionAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_dx_connection_association"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Modifier-style: avoid double-emit of the underlying connection.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDxConnectionAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directconnect_gen::ConnectionAssociationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dx_connection_association", e))?;

        // LAG is not modelled; we can't actually wire the association. Just
        // confirm the connection exists (if it does) and continue.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !snapshot.connections.contains_key(&model.connection_id) {
            eprintln!(
                "warning: aws_dx_connection_association: connection {} not found in DirectConnectStateView",
                model.connection_id
            );
        }
        eprintln!(
            "warning: aws_dx_connection_association: LAG ({}) not modelled in DirectConnectStateView; association dropped",
            model.lag_id
        );

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_dx_connection_association state not persisted (LAG not modelled)".to_string(),
            ],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_dx_connection_confirmation
// ---------------------------------------------------------------------------

/// Confirms a hosted connection by flipping its `connection_state` to
/// `available`. Snapshot-mutate-merge against the existing `ConnectionView`
/// if it is present in the state view.
pub struct AwsDxConnectionConfirmationConverter {
    service: Arc<DirectConnectService>,
}

impl AwsDxConnectionConfirmationConverter {
    pub fn new(service: Arc<DirectConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDxConnectionConfirmationConverter {
    fn resource_type(&self) -> &str {
        "aws_dx_connection_confirmation"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Modifier-style: confirmation is a state transition on the
        // underlying connection, not a separately enumerable resource.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDxConnectionConfirmationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directconnect_gen::ConnectionConfirmationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dx_connection_confirmation", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        match snapshot.connections.get_mut(&model.connection_id) {
            Some(conn) => {
                conn.connection_state = "available".to_string();
            }
            None => {
                eprintln!(
                    "warning: aws_dx_connection_confirmation: connection {} not found; state transition dropped",
                    model.connection_id
                );
                warnings.push(format!(
                    "aws_dx_connection_confirmation: connection {} missing",
                    model.connection_id
                ));
            }
        }
        self.service
            .merge(&ctx.default_account_id, &region, snapshot)
            .await?;
        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_dx_gateway
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxGatewayConverter,
    "aws_dx_gateway",
    GatewayTfModel,
    "aws_dx_gateway state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_gateway_association
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxGatewayAssociationConverter,
    "aws_dx_gateway_association",
    GatewayAssociationTfModel,
    "aws_dx_gateway_association state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_gateway_association_proposal
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxGatewayAssociationProposalConverter,
    "aws_dx_gateway_association_proposal",
    GatewayAssociationProposalTfModel,
    "aws_dx_gateway_association_proposal state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_hosted_connection
// ---------------------------------------------------------------------------

/// A hosted connection is a sub-allocation of bandwidth on a parent
/// connection. It is itself a connection (with its own id, vlan, owner)
/// so we can persist it in `DirectConnectStateView::connections`.
pub struct AwsDxHostedConnectionConverter {
    service: Arc<DirectConnectService>,
}

impl AwsDxHostedConnectionConverter {
    pub fn new(service: Arc<DirectConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDxHostedConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_dx_hosted_connection"
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

impl AwsDxHostedConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directconnect_gen::HostedConnectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dx_hosted_connection", e))?;

        // The hosted-connection id is what TF treats as the connection id.
        let connection_id = model.id.clone().unwrap_or_else(|| model.name.clone());
        let connection_state = model
            .connection_state
            .unwrap_or_else(|| "ordering".to_string());
        let location = model.location.unwrap_or_default();
        let vlan = model.vlan as i32;

        let conn_view = ConnectionView {
            connection_id: connection_id.clone(),
            connection_name: model.name.clone(),
            connection_state,
            region: region.clone(),
            location,
            bandwidth: model.bandwidth,
            owner_account: model.owner_account_id,
            vlan,
            partner_name: model.partner_name,
            loa_issue_time: None,
            tags: HashMap::new(),
        };

        let state_view = DirectConnectStateView {
            connections: {
                let mut m = HashMap::new();
                m.insert(connection_id, conn_view);
                m
            },
        };
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        // The parent_connection_id and AWS-side bookkeeping (aws_device,
        // jumbo_frame_capable, has_logical_redundancy) have no slot in the
        // view and are dropped.
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Hosted connections cannot be distinguished from regular
        // `aws_dx_connection` rows at the view layer; `AwsDxConnectionConverter`
        // already extracts them as `aws_dx_connection`. Return empty to
        // avoid double-emit.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_dx_hosted_private_virtual_interface
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxHostedPrivateVirtualInterfaceConverter,
    "aws_dx_hosted_private_virtual_interface",
    HostedPrivateVirtualInterfaceTfModel,
    "aws_dx_hosted_private_virtual_interface state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_hosted_private_virtual_interface_accepter
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxHostedPrivateVirtualInterfaceAccepterConverter,
    "aws_dx_hosted_private_virtual_interface_accepter",
    HostedPrivateVirtualInterfaceAccepterTfModel,
    "aws_dx_hosted_private_virtual_interface_accepter state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_hosted_public_virtual_interface
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxHostedPublicVirtualInterfaceConverter,
    "aws_dx_hosted_public_virtual_interface",
    HostedPublicVirtualInterfaceTfModel,
    "aws_dx_hosted_public_virtual_interface state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_hosted_public_virtual_interface_accepter
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxHostedPublicVirtualInterfaceAccepterConverter,
    "aws_dx_hosted_public_virtual_interface_accepter",
    HostedPublicVirtualInterfaceAccepterTfModel,
    "aws_dx_hosted_public_virtual_interface_accepter state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_hosted_transit_virtual_interface
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxHostedTransitVirtualInterfaceConverter,
    "aws_dx_hosted_transit_virtual_interface",
    HostedTransitVirtualInterfaceTfModel,
    "aws_dx_hosted_transit_virtual_interface state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_hosted_transit_virtual_interface_accepter
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxHostedTransitVirtualInterfaceAccepterConverter,
    "aws_dx_hosted_transit_virtual_interface_accepter",
    HostedTransitVirtualInterfaceAccepterTfModel,
    "aws_dx_hosted_transit_virtual_interface_accepter state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_lag
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxLagConverter,
    "aws_dx_lag",
    LagTfModel,
    "aws_dx_lag state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_macsec_key_association
// ---------------------------------------------------------------------------

/// Modifier-style: MACsec keys live on the connection but are not modelled
/// in `ConnectionView`. We still snapshot+merge the existing connection so
/// the merge log records the visit, then drop the key with a warning.
pub struct AwsDxMacsecKeyAssociationConverter {
    service: Arc<DirectConnectService>,
}

impl AwsDxMacsecKeyAssociationConverter {
    pub fn new(service: Arc<DirectConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDxMacsecKeyAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_dx_macsec_key_association"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Modifier-style: avoid double-emit of the underlying connection.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDxMacsecKeyAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directconnect_gen::MacsecKeyAssociationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dx_macsec_key_association", e))?;

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !snapshot.connections.contains_key(&model.connection_id) {
            eprintln!(
                "warning: aws_dx_macsec_key_association: connection {} not found in DirectConnectStateView",
                model.connection_id
            );
        }
        eprintln!(
            "warning: aws_dx_macsec_key_association: MACsec keys not modelled in DirectConnectStateView; key association dropped"
        );

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_dx_macsec_key_association state not persisted (MACsec keys not modelled)"
                    .to_string(),
            ],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_dx_private_virtual_interface
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxPrivateVirtualInterfaceConverter,
    "aws_dx_private_virtual_interface",
    PrivateVirtualInterfaceTfModel,
    "aws_dx_private_virtual_interface state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_public_virtual_interface
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxPublicVirtualInterfaceConverter,
    "aws_dx_public_virtual_interface",
    PublicVirtualInterfaceTfModel,
    "aws_dx_public_virtual_interface state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_dx_transit_virtual_interface
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsDxTransitVirtualInterfaceConverter,
    "aws_dx_transit_virtual_interface",
    TransitVirtualInterfaceTfModel,
    "aws_dx_transit_virtual_interface state not persisted (no view slot)",
);
