use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A Managed Blockchain node.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub network_id: String,
    pub member_id: String,
    pub instance_type: String,
    pub availability_zone: Option<String>,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub arn: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub framework: String,
    pub framework_version: String,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub arn: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub id: String,
    pub network_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub arn: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Accessor {
    pub id: String,
    pub accessor_type: String,
    pub network_type: String,
    pub billing_token: String,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub arn: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: String,
    pub network_id: String,
    pub member_id: String,
    pub member_name: String,
    pub description: Option<String>,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub yes_vote_count: i32,
    pub no_vote_count: i32,
    pub outstanding_vote_count: i32,
    pub arn: String,
    pub tags: HashMap<String, String>,
    pub actions: ProposalActions,
}

#[derive(Debug, Clone, Default)]
pub struct ProposalActions {
    pub invitations: Vec<InviteAction>,
    pub removals: Vec<RemoveAction>,
}

#[derive(Debug, Clone)]
pub struct InviteAction {
    pub principal: String,
}

#[derive(Debug, Clone)]
pub struct RemoveAction {
    pub member_id: String,
}

#[derive(Debug, Clone)]
pub struct ProposalVote {
    pub member_id: String,
    pub member_name: String,
    pub vote: String,
}

#[derive(Debug, Clone)]
pub struct Invitation {
    pub invitation_id: String,
    pub network_id: String,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub arn: String,
}
