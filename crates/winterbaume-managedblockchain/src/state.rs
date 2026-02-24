use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ManagedBlockchainState {
    pub networks: HashMap<String, Network>,
    pub members: HashMap<String, Member>,
    pub nodes: HashMap<String, Node>,
    pub accessors: HashMap<String, Accessor>,
    pub proposals: HashMap<String, Proposal>,
    pub proposal_votes: HashMap<String, Vec<ProposalVote>>,
    pub invitations: Vec<Invitation>,
    /// Map from resource ARN to tags
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ManagedBlockchainError {
    #[error("Network {network_id} not found.")]
    NetworkNotFound { network_id: String },
    #[error("Member {member_id} not found.")]
    MemberNotFound { member_id: String },
    #[error("Node {node_id} not found.")]
    NodeNotFound { node_id: String },
    #[error("Accessor {accessor_id} not found.")]
    AccessorNotFound { accessor_id: String },
    #[error("InvitationId {invitation_id} not found.")]
    InvitationNotFound { invitation_id: String },
    #[error("Proposal {proposal_id} not found.")]
    ProposalNotFound { proposal_id: String },
    #[error("Resource {resource_arn} not found.")]
    ResourceNotFound { resource_arn: String },
    #[error("Member {member_id} has already voted.")]
    MemberAlreadyVoted { member_id: String },
    #[error("Invitation {invitation_id} is not in PENDING status.")]
    InvitationNotPending { invitation_id: String },
}

impl ManagedBlockchainState {
    pub fn create_network(
        &mut self,
        name: &str,
        description: Option<&str>,
        framework: &str,
        framework_version: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(&Network, &Member), ManagedBlockchainError> {
        let network_id = uuid::Uuid::new_v4().to_string();
        let member_id = uuid::Uuid::new_v4().to_string();
        let network_arn =
            format!("arn:aws:managedblockchain:{region}:{account_id}:networks/{network_id}");

        let network = Network {
            id: network_id.clone(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            framework: framework.to_string(),
            framework_version: framework_version.to_string(),
            status: "AVAILABLE".to_string(),
            creation_date: Utc::now(),
            arn: network_arn,
            tags: HashMap::new(),
        };

        let member_arn =
            format!("arn:aws:managedblockchain:{region}:{account_id}:members/{member_id}");

        let member = Member {
            id: member_id.clone(),
            network_id: network_id.clone(),
            name: format!("{name}-member"),
            description: None,
            status: "AVAILABLE".to_string(),
            creation_date: Utc::now(),
            arn: member_arn,
            tags: HashMap::new(),
        };

        self.networks.insert(network_id.clone(), network);
        self.members.insert(member_id.clone(), member);
        Ok((
            self.networks.get(&network_id).unwrap(),
            self.members.get(&member_id).unwrap(),
        ))
    }

    pub fn get_network(&self, network_id: &str) -> Result<&Network, ManagedBlockchainError> {
        self.networks
            .get(network_id)
            .ok_or_else(|| ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            })
    }

    pub fn list_networks(&self) -> Vec<&Network> {
        self.networks.values().collect()
    }

    pub fn delete_network(&mut self, network_id: &str) -> Result<(), ManagedBlockchainError> {
        if self.networks.remove(network_id).is_none() {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        // Also remove all members and nodes belonging to this network
        self.members.retain(|_, m| m.network_id != network_id);
        self.nodes.retain(|_, n| n.network_id != network_id);
        Ok(())
    }

    pub fn get_member(
        &self,
        network_id: &str,
        member_id: &str,
    ) -> Result<&Member, ManagedBlockchainError> {
        // Verify network exists
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        self.members
            .get(member_id)
            .and_then(|m| {
                if m.network_id == network_id {
                    Some(m)
                } else {
                    None
                }
            })
            .ok_or_else(|| ManagedBlockchainError::MemberNotFound {
                member_id: member_id.to_string(),
            })
    }

    pub fn list_members(&self, network_id: &str) -> Result<Vec<&Member>, ManagedBlockchainError> {
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        Ok(self
            .members
            .values()
            .filter(|m| m.network_id == network_id)
            .collect())
    }

    // --- Node operations ---

    pub fn create_node(
        &mut self,
        network_id: &str,
        member_id: &str,
        instance_type: &str,
        availability_zone: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Node, ManagedBlockchainError> {
        // Verify network
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        // Verify member belongs to network
        let member_ok = self
            .members
            .get(member_id)
            .map(|m| m.network_id == network_id)
            .unwrap_or(false);
        if !member_ok {
            return Err(ManagedBlockchainError::MemberNotFound {
                member_id: member_id.to_string(),
            });
        }

        let node_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:managedblockchain:{region}:{account_id}:nodes/{node_id}");
        let node = Node {
            id: node_id.clone(),
            network_id: network_id.to_string(),
            member_id: member_id.to_string(),
            instance_type: instance_type.to_string(),
            availability_zone: availability_zone.map(|s| s.to_string()),
            status: "AVAILABLE".to_string(),
            creation_date: Utc::now(),
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.nodes.insert(node_id.clone(), node);
        Ok(self.nodes.get(&node_id).unwrap())
    }

    pub fn get_node(
        &self,
        network_id: &str,
        member_id: &str,
        node_id: &str,
    ) -> Result<&Node, ManagedBlockchainError> {
        self.nodes
            .get(node_id)
            .and_then(|n| {
                let network_matches = n.network_id == network_id;
                let member_matches = member_id.is_empty() || n.member_id == member_id;
                if network_matches && member_matches {
                    Some(n)
                } else {
                    None
                }
            })
            .ok_or_else(|| ManagedBlockchainError::NodeNotFound {
                node_id: node_id.to_string(),
            })
    }

    pub fn delete_node(
        &mut self,
        network_id: &str,
        member_id: &str,
        node_id: &str,
    ) -> Result<(), ManagedBlockchainError> {
        let node = self
            .nodes
            .get(node_id)
            .and_then(|n| {
                let network_matches = n.network_id == network_id;
                let member_matches = member_id.is_empty() || n.member_id == member_id;
                if network_matches && member_matches {
                    Some(n)
                } else {
                    None
                }
            })
            .ok_or_else(|| ManagedBlockchainError::NodeNotFound {
                node_id: node_id.to_string(),
            })?;
        let arn = node.arn.clone();
        self.resource_tags.remove(&arn);
        self.nodes.remove(node_id);
        Ok(())
    }

    pub fn list_nodes(
        &self,
        network_id: &str,
        member_id: &str,
    ) -> Result<Vec<&Node>, ManagedBlockchainError> {
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        Ok(self
            .nodes
            .values()
            .filter(|n| {
                n.network_id == network_id && (member_id.is_empty() || n.member_id == member_id)
            })
            .collect())
    }

    pub fn update_member(
        &mut self,
        network_id: &str,
        member_id: &str,
    ) -> Result<(), ManagedBlockchainError> {
        let member = self
            .members
            .get_mut(member_id)
            .and_then(|m| {
                if m.network_id == network_id {
                    Some(m)
                } else {
                    None
                }
            })
            .ok_or_else(|| ManagedBlockchainError::MemberNotFound {
                member_id: member_id.to_string(),
            })?;
        // In real AWS, update modifies log publishing config etc.
        // For mock purposes, we just verify the member exists and return success.
        let _ = member;
        Ok(())
    }

    pub fn update_node(
        &mut self,
        network_id: &str,
        member_id: &str,
        node_id: &str,
    ) -> Result<(), ManagedBlockchainError> {
        let node = self
            .nodes
            .get_mut(node_id)
            .and_then(|n| {
                let network_matches = n.network_id == network_id;
                // member_id is optional in this call; if empty, skip check
                let member_matches = member_id.is_empty() || n.member_id == member_id;
                if network_matches && member_matches {
                    Some(n)
                } else {
                    None
                }
            })
            .ok_or_else(|| ManagedBlockchainError::NodeNotFound {
                node_id: node_id.to_string(),
            })?;
        // In real AWS, update modifies log publishing config etc.
        // For mock purposes, we just verify the node exists and return success.
        let _ = node;
        Ok(())
    }

    // --- Accessor operations ---

    pub fn create_accessor(
        &mut self,
        accessor_type: &str,
        network_type: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Accessor, ManagedBlockchainError> {
        let accessor_id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:managedblockchain:{region}:{account_id}:accessors/{accessor_id}");
        let billing_token = uuid::Uuid::new_v4().to_string();
        let net_type = network_type
            .unwrap_or("ETHEREUM_MAINNET_AND_GOERLI")
            .to_string();

        let accessor = Accessor {
            id: accessor_id.clone(),
            accessor_type: accessor_type.to_string(),
            network_type: net_type.clone(),
            billing_token,
            status: "AVAILABLE".to_string(),
            creation_date: Utc::now(),
            arn: arn.clone(),
            tags: tags.clone(),
        };

        self.resource_tags.insert(arn, tags);
        self.accessors.insert(accessor_id.clone(), accessor);
        Ok(self.accessors.get(&accessor_id).unwrap())
    }

    pub fn get_accessor(&self, accessor_id: &str) -> Result<&Accessor, ManagedBlockchainError> {
        self.accessors
            .get(accessor_id)
            .ok_or_else(|| ManagedBlockchainError::AccessorNotFound {
                accessor_id: accessor_id.to_string(),
            })
    }

    pub fn delete_accessor(&mut self, accessor_id: &str) -> Result<(), ManagedBlockchainError> {
        match self.accessors.remove(accessor_id) {
            Some(accessor) => {
                self.resource_tags.remove(&accessor.arn);
                Ok(())
            }
            None => Err(ManagedBlockchainError::AccessorNotFound {
                accessor_id: accessor_id.to_string(),
            }),
        }
    }

    pub fn list_accessors(&self) -> Vec<&Accessor> {
        self.accessors.values().collect()
    }

    // --- Proposal operations ---

    pub fn create_proposal(
        &mut self,
        network_id: &str,
        member_id: &str,
        description: Option<&str>,
        actions: ProposalActions,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Proposal, ManagedBlockchainError> {
        // Verify network exists
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        // Verify member exists
        let member =
            self.members
                .get(member_id)
                .ok_or_else(|| ManagedBlockchainError::MemberNotFound {
                    member_id: member_id.to_string(),
                })?;
        let member_name = member.name.clone();

        let proposal_id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:managedblockchain:{region}:{account_id}:proposals/{proposal_id}");
        let now = Utc::now();
        let expiration = now + chrono::Duration::hours(24);

        // Count outstanding votes (number of members in network)
        let member_count = self
            .members
            .values()
            .filter(|m| m.network_id == network_id)
            .count() as i32;

        let proposal = Proposal {
            id: proposal_id.clone(),
            network_id: network_id.to_string(),
            member_id: member_id.to_string(),
            member_name,
            description: description.map(|s| s.to_string()),
            status: "IN_PROGRESS".to_string(),
            creation_date: now,
            expiration_date: expiration,
            yes_vote_count: 0,
            no_vote_count: 0,
            outstanding_vote_count: member_count,
            arn: arn.clone(),
            tags: tags.clone(),
            actions,
        };

        self.resource_tags.insert(arn, tags);
        self.proposal_votes.insert(proposal_id.clone(), Vec::new());
        self.proposals.insert(proposal_id.clone(), proposal);
        Ok(self.proposals.get(&proposal_id).unwrap())
    }

    pub fn get_proposal(
        &self,
        network_id: &str,
        proposal_id: &str,
    ) -> Result<&Proposal, ManagedBlockchainError> {
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        self.proposals
            .get(proposal_id)
            .and_then(|p| {
                if p.network_id == network_id {
                    Some(p)
                } else {
                    None
                }
            })
            .ok_or_else(|| ManagedBlockchainError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            })
    }

    pub fn list_proposals(
        &self,
        network_id: &str,
    ) -> Result<Vec<&Proposal>, ManagedBlockchainError> {
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        Ok(self
            .proposals
            .values()
            .filter(|p| p.network_id == network_id)
            .collect())
    }

    pub fn vote_on_proposal(
        &mut self,
        network_id: &str,
        proposal_id: &str,
        voter_member_id: &str,
        vote: &str,
    ) -> Result<(), ManagedBlockchainError> {
        // Verify network
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        // Verify proposal
        let proposal = self.proposals.get_mut(proposal_id).ok_or_else(|| {
            ManagedBlockchainError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            }
        })?;
        if proposal.network_id != network_id {
            return Err(ManagedBlockchainError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            });
        }
        // Verify voter member
        let member = self.members.get(voter_member_id).ok_or_else(|| {
            ManagedBlockchainError::MemberNotFound {
                member_id: voter_member_id.to_string(),
            }
        })?;
        let member_name = member.name.clone();

        // Check if already voted
        let votes = self
            .proposal_votes
            .entry(proposal_id.to_string())
            .or_default();
        if votes.iter().any(|v| v.member_id == voter_member_id) {
            return Err(ManagedBlockchainError::MemberAlreadyVoted {
                member_id: voter_member_id.to_string(),
            });
        }

        votes.push(ProposalVote {
            member_id: voter_member_id.to_string(),
            member_name,
            vote: vote.to_string(),
        });

        // Update counts
        {
            let proposal = self.proposals.get_mut(proposal_id).unwrap();
            match vote {
                "YES" => proposal.yes_vote_count += 1,
                "NO" => proposal.no_vote_count += 1,
                _ => {}
            }
            proposal.outstanding_vote_count -= 1;
        }

        // Determine if proposal should be finalized
        let (yes, no, outstanding, invite_actions, p_network_id) = {
            let proposal = self.proposals.get(proposal_id).unwrap();
            (
                proposal.yes_vote_count,
                proposal.no_vote_count,
                proposal.outstanding_vote_count,
                proposal.actions.invitations.clone(),
                proposal.network_id.clone(),
            )
        };
        let total = yes + no + outstanding;

        // Approved when yes >= 50% of total possible votes
        let is_approved = total > 0 && yes * 100 / total >= 50;
        // Rejected when all votes cast and not approved
        let is_rejected = outstanding == 0 && !is_approved;

        if is_approved {
            {
                let proposal = self.proposals.get_mut(proposal_id).unwrap();
                proposal.status = "APPROVED".to_string();
            }

            // Create invitations for each InviteAction
            let now = Utc::now();
            let expiration = now + chrono::Duration::hours(24);
            for _ in &invite_actions {
                let raw_uuid = uuid::Uuid::new_v4().to_string().replace('-', "");
                let invitation_id = format!("in-{}", &raw_uuid[..24]);
                let arn = format!("arn:aws:managedblockchain:::invitations/{invitation_id}");
                self.invitations.push(Invitation {
                    invitation_id,
                    network_id: p_network_id.clone(),
                    status: "PENDING".to_string(),
                    creation_date: now,
                    expiration_date: expiration,
                    arn,
                });
            }
        } else if is_rejected {
            let proposal = self.proposals.get_mut(proposal_id).unwrap();
            proposal.status = "REJECTED".to_string();
        }

        Ok(())
    }

    pub fn list_proposal_votes(
        &self,
        network_id: &str,
        proposal_id: &str,
    ) -> Result<Vec<&ProposalVote>, ManagedBlockchainError> {
        // Verify network
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        // Verify proposal
        let proposal = self.proposals.get(proposal_id).ok_or_else(|| {
            ManagedBlockchainError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            }
        })?;
        if proposal.network_id != network_id {
            return Err(ManagedBlockchainError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            });
        }
        Ok(self
            .proposal_votes
            .get(proposal_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }

    pub fn create_member(
        &mut self,
        network_id: &str,
        invitation_id: &str,
        name: &str,
        description: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Member, ManagedBlockchainError> {
        // Verify network exists
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        // Verify invitation exists and is PENDING
        let inv = self
            .invitations
            .iter_mut()
            .find(|i| i.invitation_id == invitation_id)
            .ok_or_else(|| ManagedBlockchainError::InvitationNotFound {
                invitation_id: invitation_id.to_string(),
            })?;
        if inv.status != "PENDING" {
            return Err(ManagedBlockchainError::InvitationNotPending {
                invitation_id: invitation_id.to_string(),
            });
        }
        // Mark invitation as ACCEPTED
        inv.status = "ACCEPTED".to_string();

        let member_id = uuid::Uuid::new_v4().to_string();
        let member_arn =
            format!("arn:aws:managedblockchain:{region}:{account_id}:members/{member_id}");

        let member = Member {
            id: member_id.clone(),
            network_id: network_id.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            status: "AVAILABLE".to_string(),
            creation_date: Utc::now(),
            arn: member_arn.clone(),
            tags: tags.clone(),
        };

        self.resource_tags.insert(member_arn, tags);
        self.members.insert(member_id.clone(), member);
        Ok(self.members.get(&member_id).unwrap())
    }

    pub fn delete_member(
        &mut self,
        network_id: &str,
        member_id: &str,
    ) -> Result<(), ManagedBlockchainError> {
        // Verify network
        if !self.networks.contains_key(network_id) {
            return Err(ManagedBlockchainError::NetworkNotFound {
                network_id: network_id.to_string(),
            });
        }
        // Verify member belongs to the network
        let member = self
            .members
            .get(member_id)
            .and_then(|m| {
                if m.network_id == network_id {
                    Some(m)
                } else {
                    None
                }
            })
            .ok_or_else(|| ManagedBlockchainError::MemberNotFound {
                member_id: member_id.to_string(),
            })?;
        let arn = member.arn.clone();
        self.resource_tags.remove(&arn);
        self.members.remove(member_id);
        Ok(())
    }

    // --- Invitation operations ---

    pub fn list_invitations(&self) -> Vec<&Invitation> {
        self.invitations.iter().collect()
    }

    pub fn reject_invitation(&mut self, invitation_id: &str) -> Result<(), ManagedBlockchainError> {
        let invitation = self
            .invitations
            .iter_mut()
            .find(|i| i.invitation_id == invitation_id)
            .ok_or_else(|| ManagedBlockchainError::InvitationNotFound {
                invitation_id: invitation_id.to_string(),
            })?;
        invitation.status = "REJECTED".to_string();
        Ok(())
    }

    // --- Tag operations ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), ManagedBlockchainError> {
        // Verify the resource exists
        if !self.resource_exists(resource_arn) {
            return Err(ManagedBlockchainError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        entry.extend(tags);
        let updated = entry.clone();

        // Also update the tags on the resource object itself
        self.sync_tags_to_resource(resource_arn, updated);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), ManagedBlockchainError> {
        if !self.resource_exists(resource_arn) {
            return Err(ManagedBlockchainError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                tags.remove(key);
            }
            let updated = tags.clone();
            self.sync_tags_to_resource(resource_arn, updated);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, ManagedBlockchainError> {
        if !self.resource_exists(resource_arn) {
            return Err(ManagedBlockchainError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        Ok(self
            .resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default())
    }

    fn resource_exists(&self, arn: &str) -> bool {
        self.networks.values().any(|n| n.arn == arn)
            || self.members.values().any(|m| m.arn == arn)
            || self.nodes.values().any(|n| n.arn == arn)
            || self.accessors.values().any(|a| a.arn == arn)
            || self.proposals.values().any(|p| p.arn == arn)
    }

    fn sync_tags_to_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        for network in self.networks.values_mut() {
            if network.arn == arn {
                network.tags = tags.clone();
                return;
            }
        }
        for member in self.members.values_mut() {
            if member.arn == arn {
                member.tags = tags.clone();
                return;
            }
        }
        for node in self.nodes.values_mut() {
            if node.arn == arn {
                node.tags = tags.clone();
                return;
            }
        }
        for accessor in self.accessors.values_mut() {
            if accessor.arn == arn {
                accessor.tags = tags.clone();
                return;
            }
        }
        for proposal in self.proposals.values_mut() {
            if proposal.arn == arn {
                proposal.tags = tags;
                return;
            }
        }
    }
}
