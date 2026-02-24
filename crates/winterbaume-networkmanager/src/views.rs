//! Serde-compatible view types for NetworkManager state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::NetworkManagerService;
use crate::state::NetworkManagerState;

/// Serializable view of a global network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalNetworkView {
    pub global_network_id: String,
    pub global_network_arn: String,
    pub description: String,
    pub created_at: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a core network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoreNetworkView {
    pub core_network_id: String,
    pub core_network_arn: String,
    pub global_network_id: String,
    pub description: String,
    pub created_at: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a site.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SiteView {
    pub site_id: String,
    pub site_arn: String,
    pub global_network_id: String,
    pub description: String,
    pub created_at: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub location: Option<serde_json::Value>,
}

/// Serializable view of a link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkView {
    pub link_id: String,
    pub link_arn: String,
    pub global_network_id: String,
    pub site_id: String,
    pub description: String,
    pub provider: String,
    pub link_type: String,
    pub bandwidth_download_speed: i32,
    pub bandwidth_upload_speed: i32,
    pub created_at: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a device.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceView {
    pub device_id: String,
    pub device_arn: String,
    pub global_network_id: String,
    pub description: String,
    #[serde(default)]
    pub site_id: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub serial_number: Option<String>,
    #[serde(default)]
    pub device_type: Option<String>,
    #[serde(default)]
    pub vendor: Option<String>,
    pub created_at: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub location: Option<serde_json::Value>,
    #[serde(default)]
    pub aws_location: Option<serde_json::Value>,
}

/// Serializable view of a connection between two devices.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionView {
    pub connection_id: String,
    pub connection_arn: String,
    pub global_network_id: String,
    pub device_id: String,
    pub connected_device_id: String,
    #[serde(default)]
    pub link_id: Option<String>,
    #[serde(default)]
    pub connected_link_id: Option<String>,
    pub description: String,
    pub created_at: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a transit gateway registration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransitGatewayRegistrationView {
    pub global_network_id: String,
    pub transit_gateway_arn: String,
    pub state: String,
    #[serde(default)]
    pub state_message: String,
}

/// Serializable view of an attachment (VPC, Connect, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentView {
    pub attachment_id: String,
    pub attachment_type: String,
    pub core_network_id: String,
    pub core_network_arn: String,
    pub owner_account_id: String,
    pub resource_arn: String,
    #[serde(default)]
    pub edge_location: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub subnet_arns: Vec<String>,
    #[serde(default)]
    pub segment_name: Option<String>,
}

/// Serializable view of a connect peer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectPeerView {
    pub connect_peer_id: String,
    pub connect_attachment_id: String,
    #[serde(default)]
    pub core_network_id: Option<String>,
    #[serde(default)]
    pub edge_location: Option<String>,
    pub peer_address: String,
    #[serde(default)]
    pub core_network_address: Option<String>,
    #[serde(default)]
    pub inside_cidr_blocks: Vec<String>,
    pub created_at: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a connect peer association.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectPeerAssociationView {
    pub connect_peer_id: String,
    pub global_network_id: String,
    pub device_id: String,
    #[serde(default)]
    pub link_id: Option<String>,
    pub state: String,
}

/// Serializable view of a link association.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkAssociationView {
    pub global_network_id: String,
    pub device_id: String,
    pub link_id: String,
    pub state: String,
}

/// Serializable view of a customer gateway association.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerGatewayAssociationView {
    pub customer_gateway_arn: String,
    pub global_network_id: String,
    pub device_id: String,
    #[serde(default)]
    pub link_id: Option<String>,
    pub state: String,
}

/// Serializable view of a transit gateway connect peer association.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransitGatewayConnectPeerAssociationView {
    pub transit_gateway_connect_peer_arn: String,
    pub global_network_id: String,
    pub device_id: String,
    #[serde(default)]
    pub link_id: Option<String>,
    pub state: String,
}

/// Serializable view of a route analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouteAnalysisView {
    pub route_analysis_id: String,
    pub global_network_id: String,
    pub owner_account_id: String,
    #[serde(default)]
    pub source_transit_gateway_arn: Option<String>,
    #[serde(default)]
    pub source_transit_gateway_attachment_arn: Option<String>,
    #[serde(default)]
    pub source_ip_address: Option<String>,
    #[serde(default)]
    pub destination_transit_gateway_arn: Option<String>,
    #[serde(default)]
    pub destination_transit_gateway_attachment_arn: Option<String>,
    #[serde(default)]
    pub destination_ip_address: Option<String>,
    pub include_return_path: bool,
    pub use_middleboxes: bool,
    pub started_at: String,
    pub status: String,
}

/// Serializable view of the entire NetworkManager state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkManagerStateView {
    #[serde(default)]
    pub global_networks: HashMap<String, GlobalNetworkView>,
    #[serde(default)]
    pub core_networks: HashMap<String, CoreNetworkView>,
    #[serde(default)]
    pub sites: HashMap<String, SiteView>,
    #[serde(default)]
    pub links: HashMap<String, LinkView>,
    #[serde(default)]
    pub devices: HashMap<String, DeviceView>,
    #[serde(default)]
    pub connections: HashMap<String, ConnectionView>,
    #[serde(default)]
    pub transit_gateway_registrations: HashMap<String, TransitGatewayRegistrationView>,
    #[serde(default)]
    pub attachments: HashMap<String, AttachmentView>,
    #[serde(default)]
    pub connect_peers: HashMap<String, ConnectPeerView>,
    #[serde(default)]
    pub connect_peer_associations: HashMap<String, ConnectPeerAssociationView>,
    #[serde(default)]
    pub link_associations: Vec<LinkAssociationView>,
    #[serde(default)]
    pub customer_gateway_associations: HashMap<String, CustomerGatewayAssociationView>,
    #[serde(default)]
    pub transit_gateway_connect_peer_associations:
        HashMap<String, TransitGatewayConnectPeerAssociationView>,
    #[serde(default)]
    pub route_analyses: HashMap<String, RouteAnalysisView>,
}

// --- From internal types to view types ---

fn dt_to_string(dt: &chrono::DateTime<chrono::Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_dt(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

impl From<&crate::types::GlobalNetwork> for GlobalNetworkView {
    fn from(n: &crate::types::GlobalNetwork) -> Self {
        GlobalNetworkView {
            global_network_id: n.global_network_id.clone(),
            global_network_arn: n.global_network_arn.clone(),
            description: n.description.clone(),
            created_at: dt_to_string(&n.created_at),
            state: n.state.clone(),
            tags: n.tags.clone(),
        }
    }
}

impl From<&crate::types::CoreNetwork> for CoreNetworkView {
    fn from(n: &crate::types::CoreNetwork) -> Self {
        CoreNetworkView {
            core_network_id: n.core_network_id.clone(),
            core_network_arn: n.core_network_arn.clone(),
            global_network_id: n.global_network_id.clone(),
            description: n.description.clone(),
            created_at: dt_to_string(&n.created_at),
            state: n.state.clone(),
            tags: n.tags.clone(),
        }
    }
}

impl From<&crate::types::Site> for SiteView {
    fn from(s: &crate::types::Site) -> Self {
        SiteView {
            site_id: s.site_id.clone(),
            site_arn: s.site_arn.clone(),
            global_network_id: s.global_network_id.clone(),
            description: s.description.clone(),
            created_at: dt_to_string(&s.created_at),
            state: s.state.clone(),
            tags: s.tags.clone(),
            location: None,
        }
    }
}

impl From<&crate::types::Link> for LinkView {
    fn from(l: &crate::types::Link) -> Self {
        LinkView {
            link_id: l.link_id.clone(),
            link_arn: l.link_arn.clone(),
            global_network_id: l.global_network_id.clone(),
            site_id: l.site_id.clone(),
            description: l.description.clone(),
            provider: l.provider.clone(),
            link_type: l.link_type.clone(),
            bandwidth_download_speed: l.bandwidth_download_speed,
            bandwidth_upload_speed: l.bandwidth_upload_speed,
            created_at: dt_to_string(&l.created_at),
            state: l.state.clone(),
            tags: l.tags.clone(),
        }
    }
}

impl From<&crate::types::Device> for DeviceView {
    fn from(d: &crate::types::Device) -> Self {
        DeviceView {
            device_id: d.device_id.clone(),
            device_arn: d.device_arn.clone(),
            global_network_id: d.global_network_id.clone(),
            description: d.description.clone(),
            site_id: d.site_id.clone(),
            model: d.model.clone(),
            serial_number: d.serial_number.clone(),
            device_type: d.device_type.clone(),
            vendor: d.vendor.clone(),
            created_at: dt_to_string(&d.created_at),
            state: d.state.clone(),
            tags: d.tags.clone(),
            location: None,
            aws_location: None,
        }
    }
}

impl From<&crate::types::Connection> for ConnectionView {
    fn from(c: &crate::types::Connection) -> Self {
        ConnectionView {
            connection_id: c.connection_id.clone(),
            connection_arn: c.connection_arn.clone(),
            global_network_id: c.global_network_id.clone(),
            device_id: c.device_id.clone(),
            connected_device_id: c.connected_device_id.clone(),
            link_id: c.link_id.clone(),
            connected_link_id: c.connected_link_id.clone(),
            description: c.description.clone(),
            created_at: dt_to_string(&c.created_at),
            state: c.state.clone(),
            tags: c.tags.clone(),
        }
    }
}

impl From<&crate::types::TransitGatewayRegistration> for TransitGatewayRegistrationView {
    fn from(r: &crate::types::TransitGatewayRegistration) -> Self {
        TransitGatewayRegistrationView {
            global_network_id: r.global_network_id.clone(),
            transit_gateway_arn: r.transit_gateway_arn.clone(),
            state: r.state.clone(),
            state_message: r.state_message.clone(),
        }
    }
}

impl From<&crate::types::Attachment> for AttachmentView {
    fn from(a: &crate::types::Attachment) -> Self {
        AttachmentView {
            attachment_id: a.attachment_id.clone(),
            attachment_type: a.attachment_type.clone(),
            core_network_id: a.core_network_id.clone(),
            core_network_arn: a.core_network_arn.clone(),
            owner_account_id: a.owner_account_id.clone(),
            resource_arn: a.resource_arn.clone(),
            edge_location: a.edge_location.clone(),
            state: a.state.clone(),
            created_at: dt_to_string(&a.created_at),
            updated_at: dt_to_string(&a.updated_at),
            tags: a.tags.clone(),
            subnet_arns: a.subnet_arns.clone(),
            segment_name: a.segment_name.clone(),
        }
    }
}

impl From<&crate::types::ConnectPeer> for ConnectPeerView {
    fn from(cp: &crate::types::ConnectPeer) -> Self {
        ConnectPeerView {
            connect_peer_id: cp.connect_peer_id.clone(),
            connect_attachment_id: cp.connect_attachment_id.clone(),
            core_network_id: cp.core_network_id.clone(),
            edge_location: cp.edge_location.clone(),
            peer_address: cp.peer_address.clone(),
            core_network_address: cp.core_network_address.clone(),
            inside_cidr_blocks: cp.inside_cidr_blocks.clone(),
            created_at: dt_to_string(&cp.created_at),
            state: cp.state.clone(),
            tags: cp.tags.clone(),
        }
    }
}

impl From<&crate::types::ConnectPeerAssociation> for ConnectPeerAssociationView {
    fn from(a: &crate::types::ConnectPeerAssociation) -> Self {
        ConnectPeerAssociationView {
            connect_peer_id: a.connect_peer_id.clone(),
            global_network_id: a.global_network_id.clone(),
            device_id: a.device_id.clone(),
            link_id: a.link_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&crate::types::LinkAssociation> for LinkAssociationView {
    fn from(a: &crate::types::LinkAssociation) -> Self {
        LinkAssociationView {
            global_network_id: a.global_network_id.clone(),
            device_id: a.device_id.clone(),
            link_id: a.link_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&crate::types::CustomerGatewayAssociation> for CustomerGatewayAssociationView {
    fn from(a: &crate::types::CustomerGatewayAssociation) -> Self {
        CustomerGatewayAssociationView {
            customer_gateway_arn: a.customer_gateway_arn.clone(),
            global_network_id: a.global_network_id.clone(),
            device_id: a.device_id.clone(),
            link_id: a.link_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&crate::types::TransitGatewayConnectPeerAssociation>
    for TransitGatewayConnectPeerAssociationView
{
    fn from(a: &crate::types::TransitGatewayConnectPeerAssociation) -> Self {
        TransitGatewayConnectPeerAssociationView {
            transit_gateway_connect_peer_arn: a.transit_gateway_connect_peer_arn.clone(),
            global_network_id: a.global_network_id.clone(),
            device_id: a.device_id.clone(),
            link_id: a.link_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&crate::types::RouteAnalysis> for RouteAnalysisView {
    fn from(ra: &crate::types::RouteAnalysis) -> Self {
        RouteAnalysisView {
            route_analysis_id: ra.route_analysis_id.clone(),
            global_network_id: ra.global_network_id.clone(),
            owner_account_id: ra.owner_account_id.clone(),
            source_transit_gateway_arn: ra.source_transit_gateway_arn.clone(),
            source_transit_gateway_attachment_arn: ra.source_transit_gateway_attachment_arn.clone(),
            source_ip_address: ra.source_ip_address.clone(),
            destination_transit_gateway_arn: ra.destination_transit_gateway_arn.clone(),
            destination_transit_gateway_attachment_arn: ra
                .destination_transit_gateway_attachment_arn
                .clone(),
            destination_ip_address: ra.destination_ip_address.clone(),
            include_return_path: ra.include_return_path,
            use_middleboxes: ra.use_middleboxes,
            started_at: dt_to_string(&ra.started_at),
            status: ra.status.clone(),
        }
    }
}

impl From<&NetworkManagerState> for NetworkManagerStateView {
    fn from(state: &NetworkManagerState) -> Self {
        NetworkManagerStateView {
            global_networks: state
                .global_networks
                .iter()
                .map(|(k, v)| (k.clone(), GlobalNetworkView::from(v)))
                .collect(),
            core_networks: state
                .core_networks
                .iter()
                .map(|(k, v)| (k.clone(), CoreNetworkView::from(v)))
                .collect(),
            sites: state
                .sites
                .iter()
                .map(|(k, v)| (k.clone(), SiteView::from(v)))
                .collect(),
            links: state
                .links
                .iter()
                .map(|(k, v)| (k.clone(), LinkView::from(v)))
                .collect(),
            devices: state
                .devices
                .iter()
                .map(|(k, v)| (k.clone(), DeviceView::from(v)))
                .collect(),
            connections: state
                .connections
                .iter()
                .map(|(k, v)| (k.clone(), ConnectionView::from(v)))
                .collect(),
            transit_gateway_registrations: state
                .transit_gateway_registrations
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayRegistrationView::from(v)))
                .collect(),
            attachments: state
                .attachments
                .iter()
                .map(|(k, v)| (k.clone(), AttachmentView::from(v)))
                .collect(),
            connect_peers: state
                .connect_peers
                .iter()
                .map(|(k, v)| (k.clone(), ConnectPeerView::from(v)))
                .collect(),
            connect_peer_associations: state
                .connect_peer_associations
                .iter()
                .map(|(k, v)| (k.clone(), ConnectPeerAssociationView::from(v)))
                .collect(),
            link_associations: state
                .link_associations
                .iter()
                .map(LinkAssociationView::from)
                .collect(),
            customer_gateway_associations: state
                .customer_gateway_associations
                .iter()
                .map(|(k, v)| (k.clone(), CustomerGatewayAssociationView::from(v)))
                .collect(),
            transit_gateway_connect_peer_associations: state
                .transit_gateway_connect_peer_associations
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayConnectPeerAssociationView::from(v)))
                .collect(),
            route_analyses: state
                .route_analyses
                .iter()
                .map(|(k, v)| (k.clone(), RouteAnalysisView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<GlobalNetworkView> for crate::types::GlobalNetwork {
    fn from(v: GlobalNetworkView) -> Self {
        crate::types::GlobalNetwork {
            global_network_id: v.global_network_id,
            global_network_arn: v.global_network_arn,
            description: v.description,
            created_at: parse_dt(&v.created_at),
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<CoreNetworkView> for crate::types::CoreNetwork {
    fn from(v: CoreNetworkView) -> Self {
        crate::types::CoreNetwork {
            core_network_id: v.core_network_id,
            core_network_arn: v.core_network_arn,
            global_network_id: v.global_network_id,
            description: v.description,
            created_at: parse_dt(&v.created_at),
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<SiteView> for crate::types::Site {
    fn from(v: SiteView) -> Self {
        crate::types::Site {
            site_id: v.site_id,
            site_arn: v.site_arn,
            global_network_id: v.global_network_id,
            description: v.description,
            created_at: parse_dt(&v.created_at),
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<LinkView> for crate::types::Link {
    fn from(v: LinkView) -> Self {
        crate::types::Link {
            link_id: v.link_id,
            link_arn: v.link_arn,
            global_network_id: v.global_network_id,
            site_id: v.site_id,
            description: v.description,
            provider: v.provider,
            link_type: v.link_type,
            bandwidth_download_speed: v.bandwidth_download_speed,
            bandwidth_upload_speed: v.bandwidth_upload_speed,
            created_at: parse_dt(&v.created_at),
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<DeviceView> for crate::types::Device {
    fn from(v: DeviceView) -> Self {
        crate::types::Device {
            device_id: v.device_id,
            device_arn: v.device_arn,
            global_network_id: v.global_network_id,
            description: v.description,
            site_id: v.site_id,
            model: v.model,
            serial_number: v.serial_number,
            device_type: v.device_type,
            vendor: v.vendor,
            created_at: parse_dt(&v.created_at),
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<ConnectionView> for crate::types::Connection {
    fn from(v: ConnectionView) -> Self {
        crate::types::Connection {
            connection_id: v.connection_id,
            connection_arn: v.connection_arn,
            global_network_id: v.global_network_id,
            device_id: v.device_id,
            connected_device_id: v.connected_device_id,
            link_id: v.link_id,
            connected_link_id: v.connected_link_id,
            description: v.description,
            created_at: parse_dt(&v.created_at),
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<TransitGatewayRegistrationView> for crate::types::TransitGatewayRegistration {
    fn from(v: TransitGatewayRegistrationView) -> Self {
        crate::types::TransitGatewayRegistration {
            global_network_id: v.global_network_id,
            transit_gateway_arn: v.transit_gateway_arn,
            state: v.state,
            state_message: v.state_message,
        }
    }
}

impl From<AttachmentView> for crate::types::Attachment {
    fn from(v: AttachmentView) -> Self {
        crate::types::Attachment {
            attachment_id: v.attachment_id,
            attachment_type: v.attachment_type,
            core_network_id: v.core_network_id,
            core_network_arn: v.core_network_arn,
            owner_account_id: v.owner_account_id,
            resource_arn: v.resource_arn,
            edge_location: v.edge_location,
            state: v.state,
            created_at: parse_dt(&v.created_at),
            updated_at: parse_dt(&v.updated_at),
            tags: v.tags,
            subnet_arns: v.subnet_arns,
            segment_name: v.segment_name,
        }
    }
}

impl From<ConnectPeerView> for crate::types::ConnectPeer {
    fn from(v: ConnectPeerView) -> Self {
        crate::types::ConnectPeer {
            connect_peer_id: v.connect_peer_id,
            connect_attachment_id: v.connect_attachment_id,
            core_network_id: v.core_network_id,
            edge_location: v.edge_location,
            peer_address: v.peer_address,
            core_network_address: v.core_network_address,
            inside_cidr_blocks: v.inside_cidr_blocks,
            created_at: parse_dt(&v.created_at),
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<ConnectPeerAssociationView> for crate::types::ConnectPeerAssociation {
    fn from(v: ConnectPeerAssociationView) -> Self {
        crate::types::ConnectPeerAssociation {
            connect_peer_id: v.connect_peer_id,
            global_network_id: v.global_network_id,
            device_id: v.device_id,
            link_id: v.link_id,
            state: v.state,
        }
    }
}

impl From<LinkAssociationView> for crate::types::LinkAssociation {
    fn from(v: LinkAssociationView) -> Self {
        crate::types::LinkAssociation {
            global_network_id: v.global_network_id,
            device_id: v.device_id,
            link_id: v.link_id,
            state: v.state,
        }
    }
}

impl From<CustomerGatewayAssociationView> for crate::types::CustomerGatewayAssociation {
    fn from(v: CustomerGatewayAssociationView) -> Self {
        crate::types::CustomerGatewayAssociation {
            customer_gateway_arn: v.customer_gateway_arn,
            global_network_id: v.global_network_id,
            device_id: v.device_id,
            link_id: v.link_id,
            state: v.state,
        }
    }
}

impl From<TransitGatewayConnectPeerAssociationView>
    for crate::types::TransitGatewayConnectPeerAssociation
{
    fn from(v: TransitGatewayConnectPeerAssociationView) -> Self {
        crate::types::TransitGatewayConnectPeerAssociation {
            transit_gateway_connect_peer_arn: v.transit_gateway_connect_peer_arn,
            global_network_id: v.global_network_id,
            device_id: v.device_id,
            link_id: v.link_id,
            state: v.state,
        }
    }
}

impl From<RouteAnalysisView> for crate::types::RouteAnalysis {
    fn from(v: RouteAnalysisView) -> Self {
        crate::types::RouteAnalysis {
            route_analysis_id: v.route_analysis_id,
            global_network_id: v.global_network_id,
            owner_account_id: v.owner_account_id,
            source_transit_gateway_arn: v.source_transit_gateway_arn,
            source_transit_gateway_attachment_arn: v.source_transit_gateway_attachment_arn,
            source_ip_address: v.source_ip_address,
            destination_transit_gateway_arn: v.destination_transit_gateway_arn,
            destination_transit_gateway_attachment_arn: v
                .destination_transit_gateway_attachment_arn,
            destination_ip_address: v.destination_ip_address,
            include_return_path: v.include_return_path,
            use_middleboxes: v.use_middleboxes,
            started_at: parse_dt(&v.started_at),
            status: v.status,
        }
    }
}

impl From<NetworkManagerStateView> for NetworkManagerState {
    fn from(view: NetworkManagerStateView) -> Self {
        NetworkManagerState {
            global_networks: view
                .global_networks
                .into_iter()
                .map(|(k, v)| (k, crate::types::GlobalNetwork::from(v)))
                .collect(),
            core_networks: view
                .core_networks
                .into_iter()
                .map(|(k, v)| (k, crate::types::CoreNetwork::from(v)))
                .collect(),
            sites: view
                .sites
                .into_iter()
                .map(|(k, v)| (k, crate::types::Site::from(v)))
                .collect(),
            links: view
                .links
                .into_iter()
                .map(|(k, v)| (k, crate::types::Link::from(v)))
                .collect(),
            devices: view
                .devices
                .into_iter()
                .map(|(k, v)| (k, crate::types::Device::from(v)))
                .collect(),
            connections: view
                .connections
                .into_iter()
                .map(|(k, v)| (k, crate::types::Connection::from(v)))
                .collect(),
            transit_gateway_registrations: view
                .transit_gateway_registrations
                .into_iter()
                .map(|(k, v)| (k, crate::types::TransitGatewayRegistration::from(v)))
                .collect(),
            attachments: view
                .attachments
                .into_iter()
                .map(|(k, v)| (k, crate::types::Attachment::from(v)))
                .collect(),
            connect_peers: view
                .connect_peers
                .into_iter()
                .map(|(k, v)| (k, crate::types::ConnectPeer::from(v)))
                .collect(),
            connect_peer_associations: view
                .connect_peer_associations
                .into_iter()
                .map(|(k, v)| (k, crate::types::ConnectPeerAssociation::from(v)))
                .collect(),
            link_associations: view
                .link_associations
                .into_iter()
                .map(crate::types::LinkAssociation::from)
                .collect(),
            customer_gateway_associations: view
                .customer_gateway_associations
                .into_iter()
                .map(|(k, v)| (k, crate::types::CustomerGatewayAssociation::from(v)))
                .collect(),
            transit_gateway_connect_peer_associations: view
                .transit_gateway_connect_peer_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::TransitGatewayConnectPeerAssociation::from(v),
                    )
                })
                .collect(),
            route_analyses: view
                .route_analyses
                .into_iter()
                .map(|(k, v)| (k, crate::types::RouteAnalysis::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for NetworkManagerService {
    type StateView = NetworkManagerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        NetworkManagerStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = NetworkManagerState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.global_networks {
                guard
                    .global_networks
                    .insert(k, crate::types::GlobalNetwork::from(v));
            }
            for (k, v) in view.core_networks {
                guard
                    .core_networks
                    .insert(k, crate::types::CoreNetwork::from(v));
            }
            for (k, v) in view.sites {
                guard.sites.insert(k, crate::types::Site::from(v));
            }
            for (k, v) in view.links {
                guard.links.insert(k, crate::types::Link::from(v));
            }
            for (k, v) in view.devices {
                guard.devices.insert(k, crate::types::Device::from(v));
            }
            for (k, v) in view.connections {
                guard
                    .connections
                    .insert(k, crate::types::Connection::from(v));
            }
            for (k, v) in view.transit_gateway_registrations {
                guard
                    .transit_gateway_registrations
                    .insert(k, crate::types::TransitGatewayRegistration::from(v));
            }
            for (k, v) in view.attachments {
                guard
                    .attachments
                    .insert(k, crate::types::Attachment::from(v));
            }
            for (k, v) in view.connect_peers {
                guard
                    .connect_peers
                    .insert(k, crate::types::ConnectPeer::from(v));
            }
            for (k, v) in view.connect_peer_associations {
                guard
                    .connect_peer_associations
                    .insert(k, crate::types::ConnectPeerAssociation::from(v));
            }
            for assoc in view.link_associations {
                let la = crate::types::LinkAssociation::from(assoc);
                if !guard.link_associations.iter().any(|existing| {
                    existing.global_network_id == la.global_network_id
                        && existing.device_id == la.device_id
                        && existing.link_id == la.link_id
                }) {
                    guard.link_associations.push(la);
                }
            }
            for (k, v) in view.customer_gateway_associations {
                guard
                    .customer_gateway_associations
                    .insert(k, crate::types::CustomerGatewayAssociation::from(v));
            }
            for (k, v) in view.transit_gateway_connect_peer_associations {
                guard.transit_gateway_connect_peer_associations.insert(
                    k,
                    crate::types::TransitGatewayConnectPeerAssociation::from(v),
                );
            }
            for (k, v) in view.route_analyses {
                guard
                    .route_analyses
                    .insert(k, crate::types::RouteAnalysis::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
