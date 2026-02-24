use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A global network.
#[derive(Debug, Clone)]
pub struct GlobalNetwork {
    pub global_network_id: String,
    pub global_network_arn: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// A core network.
#[derive(Debug, Clone)]
pub struct CoreNetwork {
    pub core_network_id: String,
    pub core_network_arn: String,
    pub global_network_id: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// A site within a global network.
#[derive(Debug, Clone)]
pub struct Site {
    pub site_id: String,
    pub site_arn: String,
    pub global_network_id: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// A device within a global network.
#[derive(Debug, Clone)]
pub struct Device {
    pub device_id: String,
    pub device_arn: String,
    pub global_network_id: String,
    pub description: String,
    pub site_id: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub device_type: Option<String>,
    pub vendor: Option<String>,
    pub created_at: DateTime<Utc>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// A link within a global network.
#[derive(Debug, Clone)]
pub struct Link {
    pub link_id: String,
    pub link_arn: String,
    pub global_network_id: String,
    pub site_id: String,
    pub description: String,
    pub provider: String,
    pub link_type: String,
    pub bandwidth_download_speed: i32,
    pub bandwidth_upload_speed: i32,
    pub created_at: DateTime<Utc>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// A connection between two devices in a global network.
#[derive(Debug, Clone)]
pub struct Connection {
    pub connection_id: String,
    pub connection_arn: String,
    pub global_network_id: String,
    pub device_id: String,
    pub connected_device_id: String,
    pub link_id: Option<String>,
    pub connected_link_id: Option<String>,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// A transit gateway registration in a global network.
#[derive(Debug, Clone)]
pub struct TransitGatewayRegistration {
    pub global_network_id: String,
    pub transit_gateway_arn: String,
    pub state: String,
    pub state_message: String,
}

/// An attachment to a core network (VPC, Site-to-Site VPN, Connect, etc.).
#[derive(Debug, Clone)]
pub struct Attachment {
    pub attachment_id: String,
    pub attachment_type: String,
    pub core_network_id: String,
    pub core_network_arn: String,
    pub owner_account_id: String,
    pub resource_arn: String,
    pub edge_location: Option<String>,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    // VPC attachment extras
    pub subnet_arns: Vec<String>,
    // segment info
    pub segment_name: Option<String>,
}

/// A connect peer.
#[derive(Debug, Clone)]
pub struct ConnectPeer {
    pub connect_peer_id: String,
    pub connect_attachment_id: String,
    pub core_network_id: Option<String>,
    pub edge_location: Option<String>,
    pub peer_address: String,
    pub core_network_address: Option<String>,
    pub inside_cidr_blocks: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// Association of a connect peer to a device/link.
#[derive(Debug, Clone)]
pub struct ConnectPeerAssociation {
    pub connect_peer_id: String,
    pub global_network_id: String,
    pub device_id: String,
    pub link_id: Option<String>,
    pub state: String,
}

/// Association of a link to a device.
#[derive(Debug, Clone)]
pub struct LinkAssociation {
    pub global_network_id: String,
    pub device_id: String,
    pub link_id: String,
    pub state: String,
}

/// Association of a customer gateway to a device (and optionally a link).
#[derive(Debug, Clone)]
pub struct CustomerGatewayAssociation {
    pub customer_gateway_arn: String,
    pub global_network_id: String,
    pub device_id: String,
    pub link_id: Option<String>,
    pub state: String,
}

/// A transit gateway connect peer association.
#[derive(Debug, Clone)]
pub struct TransitGatewayConnectPeerAssociation {
    pub transit_gateway_connect_peer_arn: String,
    pub global_network_id: String,
    pub device_id: String,
    pub link_id: Option<String>,
    pub state: String,
}

/// Route analysis result.
#[derive(Debug, Clone)]
pub struct RouteAnalysis {
    pub route_analysis_id: String,
    pub global_network_id: String,
    pub owner_account_id: String,
    pub source_transit_gateway_arn: Option<String>,
    pub source_transit_gateway_attachment_arn: Option<String>,
    pub source_ip_address: Option<String>,
    pub destination_transit_gateway_arn: Option<String>,
    pub destination_transit_gateway_attachment_arn: Option<String>,
    pub destination_ip_address: Option<String>,
    pub include_return_path: bool,
    pub use_middleboxes: bool,
    pub started_at: DateTime<Utc>,
    pub status: String,
}
