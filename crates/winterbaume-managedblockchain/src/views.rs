//! Serde-compatible view types for ManagedBlockchain state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ManagedBlockchainService;
use crate::state::ManagedBlockchainState;
use crate::types::{
    Accessor, Invitation, InviteAction, Member, Network, Node, Proposal, ProposalActions,
    ProposalVote, RemoveAction,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedBlockchainStateView {
    #[serde(default)]
    pub networks: HashMap<String, NetworkView>,
    #[serde(default)]
    pub members: HashMap<String, MemberView>,
    #[serde(default)]
    pub nodes: HashMap<String, NodeView>,
    #[serde(default)]
    pub accessors: HashMap<String, AccessorView>,
    #[serde(default)]
    pub proposals: HashMap<String, ProposalView>,
    #[serde(default)]
    pub proposal_votes: HashMap<String, Vec<ProposalVoteView>>,
    #[serde(default)]
    pub invitations: Vec<InvitationView>,
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub framework: String,
    pub framework_version: String,
    pub status: String,
    pub creation_date: String,
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberView {
    pub id: String,
    pub network_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub creation_date: String,
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeView {
    pub id: String,
    pub network_id: String,
    pub member_id: String,
    pub instance_type: String,
    pub availability_zone: Option<String>,
    pub status: String,
    pub creation_date: String,
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessorView {
    pub id: String,
    pub accessor_type: String,
    pub network_type: String,
    pub billing_token: String,
    pub status: String,
    pub creation_date: String,
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalView {
    pub id: String,
    pub network_id: String,
    pub member_id: String,
    pub member_name: String,
    pub description: Option<String>,
    pub status: String,
    pub creation_date: String,
    pub expiration_date: String,
    pub yes_vote_count: i32,
    pub no_vote_count: i32,
    pub outstanding_vote_count: i32,
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub invite_actions: Vec<String>,
    #[serde(default)]
    pub remove_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalVoteView {
    pub member_id: String,
    pub member_name: String,
    pub vote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationView {
    pub invitation_id: String,
    pub network_id: String,
    pub status: String,
    pub creation_date: String,
    pub expiration_date: String,
    pub arn: String,
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&ManagedBlockchainState> for ManagedBlockchainStateView {
    fn from(state: &ManagedBlockchainState) -> Self {
        ManagedBlockchainStateView {
            networks: state
                .networks
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        NetworkView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            framework: v.framework.clone(),
                            framework_version: v.framework_version.clone(),
                            status: v.status.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            arn: v.arn.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            members: state
                .members
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MemberView {
                            id: v.id.clone(),
                            network_id: v.network_id.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            status: v.status.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            arn: v.arn.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            nodes: state
                .nodes
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        NodeView {
                            id: v.id.clone(),
                            network_id: v.network_id.clone(),
                            member_id: v.member_id.clone(),
                            instance_type: v.instance_type.clone(),
                            availability_zone: v.availability_zone.clone(),
                            status: v.status.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            arn: v.arn.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            accessors: state
                .accessors
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        AccessorView {
                            id: v.id.clone(),
                            accessor_type: v.accessor_type.clone(),
                            network_type: v.network_type.clone(),
                            billing_token: v.billing_token.clone(),
                            status: v.status.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            arn: v.arn.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            proposals: state
                .proposals
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ProposalView {
                            id: v.id.clone(),
                            network_id: v.network_id.clone(),
                            member_id: v.member_id.clone(),
                            member_name: v.member_name.clone(),
                            description: v.description.clone(),
                            status: v.status.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            expiration_date: v.expiration_date.to_rfc3339(),
                            yes_vote_count: v.yes_vote_count,
                            no_vote_count: v.no_vote_count,
                            outstanding_vote_count: v.outstanding_vote_count,
                            arn: v.arn.clone(),
                            tags: v.tags.clone(),
                            invite_actions: v
                                .actions
                                .invitations
                                .iter()
                                .map(|a| a.principal.clone())
                                .collect(),
                            remove_actions: v
                                .actions
                                .removals
                                .iter()
                                .map(|a| a.member_id.clone())
                                .collect(),
                        },
                    )
                })
                .collect(),
            proposal_votes: state
                .proposal_votes
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|pv| ProposalVoteView {
                                member_id: pv.member_id.clone(),
                                member_name: pv.member_name.clone(),
                                vote: pv.vote.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            invitations: state
                .invitations
                .iter()
                .map(|inv| InvitationView {
                    invitation_id: inv.invitation_id.clone(),
                    network_id: inv.network_id.clone(),
                    status: inv.status.clone(),
                    creation_date: inv.creation_date.to_rfc3339(),
                    expiration_date: inv.expiration_date.to_rfc3339(),
                    arn: inv.arn.clone(),
                })
                .collect(),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<ManagedBlockchainStateView> for ManagedBlockchainState {
    fn from(view: ManagedBlockchainStateView) -> Self {
        ManagedBlockchainState {
            networks: view
                .networks
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Network {
                            id: v.id,
                            name: v.name,
                            description: v.description,
                            framework: v.framework,
                            framework_version: v.framework_version,
                            status: v.status,
                            creation_date: parse_dt(&v.creation_date),
                            arn: v.arn,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            members: view
                .members
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Member {
                            id: v.id,
                            network_id: v.network_id,
                            name: v.name,
                            description: v.description,
                            status: v.status,
                            creation_date: parse_dt(&v.creation_date),
                            arn: v.arn,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            nodes: view
                .nodes
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Node {
                            id: v.id,
                            network_id: v.network_id,
                            member_id: v.member_id,
                            instance_type: v.instance_type,
                            availability_zone: v.availability_zone,
                            status: v.status,
                            creation_date: parse_dt(&v.creation_date),
                            arn: v.arn,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            accessors: view
                .accessors
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Accessor {
                            id: v.id,
                            accessor_type: v.accessor_type,
                            network_type: v.network_type,
                            billing_token: v.billing_token,
                            status: v.status,
                            creation_date: parse_dt(&v.creation_date),
                            arn: v.arn,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            proposals: view
                .proposals
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Proposal {
                            id: v.id,
                            network_id: v.network_id,
                            member_id: v.member_id,
                            member_name: v.member_name,
                            description: v.description,
                            status: v.status,
                            creation_date: parse_dt(&v.creation_date),
                            expiration_date: parse_dt(&v.expiration_date),
                            yes_vote_count: v.yes_vote_count,
                            no_vote_count: v.no_vote_count,
                            outstanding_vote_count: v.outstanding_vote_count,
                            arn: v.arn,
                            tags: v.tags,
                            actions: ProposalActions {
                                invitations: v
                                    .invite_actions
                                    .into_iter()
                                    .map(|p| InviteAction { principal: p })
                                    .collect(),
                                removals: v
                                    .remove_actions
                                    .into_iter()
                                    .map(|m| RemoveAction { member_id: m })
                                    .collect(),
                            },
                        },
                    )
                })
                .collect(),
            proposal_votes: view
                .proposal_votes
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|pv| ProposalVote {
                                member_id: pv.member_id,
                                member_name: pv.member_name,
                                vote: pv.vote,
                            })
                            .collect(),
                    )
                })
                .collect(),
            invitations: view
                .invitations
                .into_iter()
                .map(|inv| Invitation {
                    invitation_id: inv.invitation_id,
                    network_id: inv.network_id,
                    status: inv.status,
                    creation_date: parse_dt(&inv.creation_date),
                    expiration_date: parse_dt(&inv.expiration_date),
                    arn: inv.arn,
                })
                .collect(),
            resource_tags: view.resource_tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for ManagedBlockchainService {
    type StateView = ManagedBlockchainStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ManagedBlockchainStateView::from(&*guard)
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
            *guard = ManagedBlockchainState::from(view);
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
            let incoming = ManagedBlockchainState::from(view);
            for (k, v) in incoming.networks {
                guard.networks.insert(k, v);
            }
            for (k, v) in incoming.members {
                guard.members.insert(k, v);
            }
            for (k, v) in incoming.nodes {
                guard.nodes.insert(k, v);
            }
            for (k, v) in incoming.accessors {
                guard.accessors.insert(k, v);
            }
            for (k, v) in incoming.proposals {
                guard.proposals.insert(k, v);
            }
            for (k, v) in incoming.proposal_votes {
                guard.proposal_votes.insert(k, v);
            }
            for inv in incoming.invitations {
                guard.invitations.push(inv);
            }
            for (k, v) in incoming.resource_tags {
                guard.resource_tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
