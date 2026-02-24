use std::collections::HashMap;

/// A MediaPackage v2 channel group.
#[derive(Debug, Clone)]
pub struct ChannelGroup {
    pub channel_group_name: String,
    pub arn: String,
    pub egress_domain: String,
    pub description: String,
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub modified_at: String,
    pub e_tag: String,
    pub channels: HashMap<String, Channel>,
}

/// A MediaPackage v2 channel within a channel group.
#[derive(Debug, Clone)]
pub struct Channel {
    pub channel_name: String,
    pub channel_group_name: String,
    pub arn: String,
    pub description: String,
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub modified_at: String,
    pub e_tag: String,
}
