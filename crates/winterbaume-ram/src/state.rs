use std::collections::HashMap;

use chrono::Utc;
use uuid;

use crate::types::*;

/// Resource types that can be shared via RAM (matching moto's _RESOURCE_TYPE_DEFINITIONS).
/// This is the list used for list_resource_types.
pub const RAM_RESOURCE_TYPES: &[(&str, &str, &str)] = &[
    ("rds:Cluster", "rds", "REGIONAL"),
    ("imagebuilder:Component", "imagebuilder", "REGIONAL"),
    ("networkmanager:CoreNetwork", "networkmanager", "GLOBAL"),
    ("resource-groups:Group", "resource-groups", "REGIONAL"),
    ("imagebuilder:Image", "imagebuilder", "REGIONAL"),
    ("imagebuilder:ImageRecipe", "imagebuilder", "REGIONAL"),
    (
        "license-manager:LicenseConfiguration",
        "license-manager",
        "REGIONAL",
    ),
    ("appmesh:Mesh", "appmesh", "REGIONAL"),
    ("ec2:PrefixList", "ec2", "REGIONAL"),
    ("codebuild:Project", "codebuild", "REGIONAL"),
    ("codebuild:ReportGroup", "codebuild", "REGIONAL"),
    (
        "route53resolver:ResolverRule",
        "route53resolver",
        "REGIONAL",
    ),
    ("ec2:Subnet", "ec2", "REGIONAL"),
    ("ec2:TransitGatewayMulticastDomain", "ec2", "REGIONAL"),
    ("glue:Database", "glue", "REGIONAL"),
    ("glue:Table", "glue", "REGIONAL"),
    ("glue:Catalog", "glue", "REGIONAL"),
];

/// Additional shareable resource types for ARN validation (not in list_resource_types output).
const ADDITIONAL_SHAREABLE_TYPES: &[(&str, &str)] = &[
    ("ec2", "transitgateway"),
    ("ec2", "capacityreservation"),
    ("ec2", "dedicatedhost"),
    ("ec2", "image"),
    ("ec2", "localgatewayroutetable"),
    ("ec2", "trafficmirrortarget"),
    ("ec2", "ipam"),
    ("ec2", "ipampool"),
    ("ec2", "transitgatewayroutetable"),
    ("ec2", "verifiedaccessgroup"),
    ("ec2", "verifiedaccessendpoint"),
    ("ds", "shareddirectory"),
    ("route53resolver", "resolverquerylogconfig"),
    ("route53resolver", "firewallrulegroup"),
    ("route53resolver", "firewalldomainlist"),
    ("rds", "dbsnapshot"),
    ("rds", "clustersnapshot"),
    ("ssm", "patchbaseline"),
    ("ssm", "parameter"),
    ("s3", "accesspoint"),
    ("s3", "bucket"),
];

/// Shareable resource type prefixes extracted from ARN service:resource-type patterns.
fn is_shareable_resource_arn(arn: &str) -> bool {
    let parts: Vec<&str> = arn.splitn(6, ':').collect();
    if parts.len() < 6 {
        return false;
    }
    let service = parts[2].to_lowercase();
    // Extract resource type from the 6th part (e.g., "transit-gateway/tgw-123" -> "transit-gateway")
    let resource_part = parts[5];
    let resource_type_part = resource_part.split('/').next().unwrap_or(resource_part);
    let arn_resource = resource_type_part.to_lowercase().replace('-', "");

    // Check RAM_RESOURCE_TYPES
    for &(rt, _, _) in RAM_RESOURCE_TYPES {
        let rt_lower = rt.to_lowercase();
        let rt_parts: Vec<&str> = rt_lower.splitn(2, ':').collect();
        if rt_parts.len() == 2 && rt_parts[0] == service {
            let rt_normalized = rt_parts[1].replace('-', "");
            if arn_resource == rt_normalized {
                return true;
            }
        }
    }

    // Check ADDITIONAL_SHAREABLE_TYPES
    for &(svc, res) in ADDITIONAL_SHAREABLE_TYPES {
        if service == svc && arn_resource == res {
            return true;
        }
    }

    false
}

/// Validate an ARN format.
fn is_valid_arn(arn: &str) -> bool {
    let parts: Vec<&str> = arn.splitn(6, ':').collect();
    parts.len() >= 6
        && parts[0] == "arn"
        && (parts[1] == "aws" || parts[1] == "aws-cn" || parts[1] == "aws-us-gov")
}

/// Validate a principal string.
fn is_valid_principal(principal: &str) -> bool {
    // Account ID: 12 digits
    if principal.len() == 12 && principal.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    // Organization ARN
    if principal.starts_with("arn:aws:organizations::") {
        return true;
    }
    // IAM role/user ARN
    if principal.starts_with("arn:aws:iam::") || principal.starts_with("arn:aws:iam:") {
        return true;
    }
    false
}

#[derive(Debug, Default)]
pub struct RamState {
    pub resource_shares: HashMap<String, ResourceShare>,
    pub resources: Vec<Resource>,
    pub associations: Vec<ResourceShareAssociation>,
    pub sharing_with_org_enabled: bool,
    /// Customer-managed permissions (keyed by ARN).
    pub customer_managed_permissions: HashMap<String, CustomerManagedPermission>,
    /// Per-resource-share permission entries (each entry: share_arn -> permission_arn).
    pub share_permissions: Vec<ResourceSharePermissionEntry>,
    /// Replace-permission work items.
    pub replace_permission_works: Vec<ReplacePermissionWork>,
    /// Resource share invitations.
    pub invitations: Vec<ResourceShareInvitation>,
}

#[derive(Debug, thiserror::Error)]
pub enum RamError {
    #[error("The specified resource ARN {0} is not valid. Verify the ARN and try again.")]
    MalformedArnInvalid(String),
    #[error("You cannot share the selected resource type.")]
    MalformedArnUnshareable,
    #[error("Principal ID {0} is malformed. Verify the ID and try again.")]
    InvalidPrincipal(String),
    #[error(
        "{0} is not a valid resource owner. Specify either SELF or OTHER-ACCOUNTS and try again."
    )]
    InvalidResourceOwner(String),
    #[error("ResourceShare {0} could not be found.")]
    ResourceShareNotFound(String),
    #[error(
        "{0} is not a valid association type. Specify either PRINCIPAL or RESOURCE and try again."
    )]
    InvalidAssociationType(String),
    #[error("{0} is not a valid association status. Specify a valid status and try again.")]
    InvalidAssociationStatus(String),
    #[error("You cannot specify a resource ARN when the association type is PRINCIPAL.")]
    ResourceArnWithPrincipalType,
    #[error("You cannot specify a principal when the association type is RESOURCE.")]
    PrincipalWithResourceType,
    #[error("{0} is not a valid scope. Must be one of: ALL, AWS, LOCAL.")]
    InvalidPermissionScope(String),
    #[error("Invalid resource type: {0}")]
    InvalidResourceType(String),
    #[error("{0} is not a valid resource region scope value. Specify a valid value and try again.")]
    InvalidResourceRegionScope(String),
    #[error("Permission {0} could not be found.")]
    PermissionNotFound(String),
    #[error("Invitation {0} could not be found.")]
    InvitationNotFound(String),
    #[error("Invitation {0} has already been responded to.")]
    InvitationAlreadyResponded(String),
}

impl RamState {
    pub fn create_resource_share(
        &mut self,
        name: &str,
        resource_arns: Vec<String>,
        principals: Vec<String>,
        allow_external_principals: bool,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&ResourceShare, RamError> {
        // Validate resource ARNs
        for arn in &resource_arns {
            if !is_valid_arn(arn) {
                return Err(RamError::MalformedArnInvalid(arn.clone()));
            }
            if !is_shareable_resource_arn(arn) {
                return Err(RamError::MalformedArnUnshareable);
            }
        }

        // Validate principals
        for principal in &principals {
            if !is_valid_principal(principal) {
                return Err(RamError::InvalidPrincipal(principal.clone()));
            }
        }

        let share_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().timestamp() as f64;
        let resource_share_arn =
            format!("arn:aws:ram:{region}:{account_id}:resource-share/{share_id}");

        let resource_share = ResourceShare {
            resource_share_arn: resource_share_arn.clone(),
            name: name.to_string(),
            owning_account_id: account_id.to_string(),
            allow_external_principals,
            status: "ACTIVE".to_string(),
            creation_time: now,
            last_updated_time: now,
            tags,
        };

        // Associate resources with the share
        for arn in &resource_arns {
            let resource_type = extract_resource_type(arn);
            let resource = Resource {
                arn: arn.clone(),
                r#type: resource_type,
                resource_share_arn: resource_share_arn.clone(),
                status: "ASSOCIATED".to_string(),
                creation_time: now,
                last_updated_time: now,
            };
            self.resources.push(resource);

            // Create a RESOURCE association
            self.associations.push(ResourceShareAssociation {
                resource_share_arn: resource_share_arn.clone(),
                resource_share_name: name.to_string(),
                associated_entity: arn.clone(),
                association_type: "RESOURCE".to_string(),
                status: "ASSOCIATED".to_string(),
                creation_time: now,
                last_updated_time: now,
                external: false,
            });
        }

        // Create PRINCIPAL associations
        for principal in &principals {
            self.associations.push(ResourceShareAssociation {
                resource_share_arn: resource_share_arn.clone(),
                resource_share_name: name.to_string(),
                associated_entity: principal.clone(),
                association_type: "PRINCIPAL".to_string(),
                status: "ASSOCIATED".to_string(),
                creation_time: now,
                last_updated_time: now,
                external: false,
            });
        }

        self.resource_shares
            .insert(resource_share_arn.clone(), resource_share);
        Ok(self.resource_shares.get(&resource_share_arn).unwrap())
    }

    pub fn get_resource_shares(
        &self,
        resource_owner: &str,
        account_id: &str,
    ) -> Result<Vec<&ResourceShare>, RamError> {
        if resource_owner != "SELF" && resource_owner != "OTHER-ACCOUNTS" {
            return Err(RamError::InvalidResourceOwner(resource_owner.to_string()));
        }
        Ok(self
            .resource_shares
            .values()
            .filter(|rs| match resource_owner {
                "SELF" => rs.owning_account_id == account_id,
                "OTHER-ACCOUNTS" => rs.owning_account_id != account_id,
                _ => true,
            })
            .collect())
    }

    pub fn delete_resource_share(&mut self, resource_share_arn: &str) -> Result<(), RamError> {
        let rs = self
            .resource_shares
            .get_mut(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;
        rs.status = "DELETED".to_string();
        rs.last_updated_time = Utc::now().timestamp() as f64;
        Ok(())
    }

    pub fn list_resources(
        &self,
        resource_owner: &str,
        resource_share_arns: &[String],
        account_id: &str,
    ) -> Vec<&Resource> {
        self.resources
            .iter()
            .filter(|r| {
                // Filter by resource share ARNs if provided
                if !resource_share_arns.is_empty()
                    && !resource_share_arns.contains(&r.resource_share_arn)
                {
                    return false;
                }
                // Filter by ownership
                if let Some(share) = self.resource_shares.get(&r.resource_share_arn) {
                    match resource_owner {
                        "SELF" => share.owning_account_id == account_id,
                        "OTHER-ACCOUNTS" => share.owning_account_id != account_id,
                        _ => true,
                    }
                } else {
                    false
                }
            })
            .collect()
    }
    pub fn enable_sharing_with_aws_organization(&mut self) -> bool {
        self.sharing_with_org_enabled = true;
        true
    }

    pub fn get_resource_share_associations(
        &self,
        association_type: &str,
        resource_share_arns: &[String],
        principal: Option<&str>,
        resource_arn: Option<&str>,
        association_status: Option<&str>,
    ) -> Result<Vec<&ResourceShareAssociation>, RamError> {
        // Validate association type
        if association_type != "PRINCIPAL" && association_type != "RESOURCE" {
            return Err(RamError::InvalidAssociationType(
                association_type.to_string(),
            ));
        }

        // Validate association status
        if let Some(status) = association_status {
            let valid_statuses = [
                "ASSOCIATING",
                "ASSOCIATED",
                "FAILED",
                "DISASSOCIATING",
                "DISASSOCIATED",
            ];
            if !valid_statuses.contains(&status) {
                return Err(RamError::InvalidAssociationStatus(status.to_string()));
            }
        }

        // Validate filter type mismatches
        if association_type == "PRINCIPAL" && resource_arn.is_some() {
            return Err(RamError::ResourceArnWithPrincipalType);
        }
        if association_type == "RESOURCE" && principal.is_some() {
            return Err(RamError::PrincipalWithResourceType);
        }

        Ok(self
            .associations
            .iter()
            .filter(|a| {
                a.association_type == association_type
                    && (resource_share_arns.is_empty()
                        || resource_share_arns.contains(&a.resource_share_arn))
                    && principal.is_none_or(|p| a.associated_entity == p)
                    && resource_arn.is_none_or(|r| a.associated_entity == r)
                    && association_status.is_none_or(|s| a.status == s)
            })
            .collect())
    }

    pub fn update_resource_share(
        &mut self,
        resource_share_arn: &str,
        name: Option<&str>,
        allow_external_principals: Option<bool>,
    ) -> Result<&ResourceShare, RamError> {
        let now = Utc::now().timestamp() as f64;
        let rs = self
            .resource_shares
            .get_mut(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;

        if let Some(n) = name {
            rs.name = n.to_string();
        }
        if let Some(aep) = allow_external_principals {
            rs.allow_external_principals = aep;
        }
        rs.last_updated_time = now;

        Ok(self.resource_shares.get(resource_share_arn).unwrap())
    }

    pub fn list_permissions(
        &self,
        resource_type: Option<&str>,
        permission_type: Option<&str>,
    ) -> Result<Vec<RamPermission>, RamError> {
        // Validate permission_type
        if let Some(pt) = permission_type
            && pt != "ALL"
            && pt != "AWS"
            && pt != "LOCAL"
        {
            return Err(RamError::InvalidPermissionScope(pt.to_string()));
        }

        // Validate resource_type
        if let Some(rt) = resource_type {
            let valid = RAM_RESOURCE_TYPES
                .iter()
                .any(|&(t, _, _)| t.to_lowercase() == rt.to_lowercase());
            if !valid {
                return Err(RamError::InvalidResourceType(rt.to_string()));
            }
            // Also check exact case match
            let exact_match = RAM_RESOURCE_TYPES.iter().any(|&(t, _, _)| t == rt);
            if !exact_match {
                return Err(RamError::InvalidResourceType(rt.to_string()));
            }
        }

        let all_permissions = get_aws_managed_permissions();
        let mut filtered = all_permissions;

        if let Some(rt) = resource_type {
            filtered.retain(|p| p.resource_type.to_lowercase() == rt.to_lowercase());
        }

        if let Some(pt) = permission_type
            && pt != "ALL"
        {
            let target_type = match pt {
                "AWS" => "AWS_MANAGED",
                "LOCAL" => "CUSTOMER_MANAGED",
                _ => pt,
            };
            filtered.retain(|p| p.permission_type == target_type);
        }

        Ok(filtered)
    }

    pub fn list_resource_types(
        &self,
        resource_region_scope: Option<&str>,
    ) -> Result<Vec<ShareableResourceType>, RamError> {
        // Validate scope
        if let Some(scope) = resource_region_scope
            && scope != "ALL"
            && scope != "GLOBAL"
            && scope != "REGIONAL"
        {
            return Err(RamError::InvalidResourceRegionScope(scope.to_string()));
        }

        let all_types: Vec<ShareableResourceType> = RAM_RESOURCE_TYPES
            .iter()
            .map(|&(rt, sn, rrs)| ShareableResourceType {
                resource_type: rt.to_string(),
                service_name: sn.to_string(),
                resource_region_scope: rrs.to_string(),
            })
            .collect();

        let scope = resource_region_scope.unwrap_or("ALL");
        if scope == "ALL" {
            Ok(all_types)
        } else {
            Ok(all_types
                .into_iter()
                .filter(|rt| rt.resource_region_scope == scope)
                .collect())
        }
    }

    // ===== TAG OPERATIONS =====

    pub fn tag_resource(
        &mut self,
        resource_share_arn: &str,
        tags: Vec<Tag>,
    ) -> Result<(), RamError> {
        let rs = self
            .resource_shares
            .get_mut(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;
        for new_tag in tags {
            if let Some(existing) = rs.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                rs.tags.push(new_tag);
            }
        }
        rs.last_updated_time = Utc::now().timestamp() as f64;
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_share_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), RamError> {
        let rs = self
            .resource_shares
            .get_mut(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;
        rs.tags.retain(|t| !tag_keys.contains(&t.key));
        rs.last_updated_time = Utc::now().timestamp() as f64;
        Ok(())
    }

    // ===== ASSOCIATE/DISASSOCIATE RESOURCE SHARE =====

    pub fn associate_resource_share(
        &mut self,
        resource_share_arn: &str,
        resource_arns: Vec<String>,
        principals: Vec<String>,
    ) -> Result<Vec<crate::types::ResourceShareAssociation>, RamError> {
        let rs = self
            .resource_shares
            .get(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;
        let share_name = rs.name.clone();
        let now = Utc::now().timestamp() as f64;
        let mut new_assocs = Vec::new();

        for arn in &resource_arns {
            if !is_valid_arn(arn) {
                return Err(RamError::MalformedArnInvalid(arn.clone()));
            }
            if !is_shareable_resource_arn(arn) {
                return Err(RamError::MalformedArnUnshareable);
            }
            // Add resource if not already present
            if !self
                .resources
                .iter()
                .any(|r| r.arn == *arn && r.resource_share_arn == resource_share_arn)
            {
                let resource_type = extract_resource_type(arn);
                self.resources.push(crate::types::Resource {
                    arn: arn.clone(),
                    r#type: resource_type,
                    resource_share_arn: resource_share_arn.to_string(),
                    status: "ASSOCIATED".to_string(),
                    creation_time: now,
                    last_updated_time: now,
                });
            }
            let assoc = crate::types::ResourceShareAssociation {
                resource_share_arn: resource_share_arn.to_string(),
                resource_share_name: share_name.clone(),
                associated_entity: arn.clone(),
                association_type: "RESOURCE".to_string(),
                status: "ASSOCIATED".to_string(),
                creation_time: now,
                last_updated_time: now,
                external: false,
            };
            self.associations.push(assoc.clone());
            new_assocs.push(assoc);
        }

        for principal in &principals {
            if !is_valid_principal(principal) {
                return Err(RamError::InvalidPrincipal(principal.clone()));
            }
            let assoc = crate::types::ResourceShareAssociation {
                resource_share_arn: resource_share_arn.to_string(),
                resource_share_name: share_name.clone(),
                associated_entity: principal.clone(),
                association_type: "PRINCIPAL".to_string(),
                status: "ASSOCIATED".to_string(),
                creation_time: now,
                last_updated_time: now,
                external: false,
            };
            self.associations.push(assoc.clone());
            new_assocs.push(assoc);
        }

        Ok(new_assocs)
    }

    pub fn disassociate_resource_share(
        &mut self,
        resource_share_arn: &str,
        resource_arns: Vec<String>,
        principals: Vec<String>,
    ) -> Result<Vec<crate::types::ResourceShareAssociation>, RamError> {
        self.resource_shares
            .get(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;

        let now = Utc::now().timestamp() as f64;
        let mut updated_assocs = Vec::new();

        for arn in &resource_arns {
            if let Some(assoc) = self.associations.iter_mut().find(|a| {
                a.resource_share_arn == resource_share_arn
                    && a.associated_entity == *arn
                    && a.association_type == "RESOURCE"
            }) {
                assoc.status = "DISASSOCIATED".to_string();
                assoc.last_updated_time = now;
                updated_assocs.push(assoc.clone());
            }
            if let Some(res) = self
                .resources
                .iter_mut()
                .find(|r| r.arn == *arn && r.resource_share_arn == resource_share_arn)
            {
                res.status = "DISASSOCIATED".to_string();
                res.last_updated_time = now;
            }
        }

        for principal in &principals {
            if let Some(assoc) = self.associations.iter_mut().find(|a| {
                a.resource_share_arn == resource_share_arn
                    && a.associated_entity == *principal
                    && a.association_type == "PRINCIPAL"
            }) {
                assoc.status = "DISASSOCIATED".to_string();
                assoc.last_updated_time = now;
                updated_assocs.push(assoc.clone());
            }
        }

        Ok(updated_assocs)
    }

    // ===== PERMISSION OPERATIONS =====

    pub fn associate_resource_share_permission(
        &mut self,
        resource_share_arn: &str,
        permission_arn: &str,
    ) -> Result<bool, RamError> {
        let rs = self
            .resource_shares
            .get(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;

        // Look up permission info
        let (resource_type, version) =
            if let Some(perm) = self.customer_managed_permissions.get(permission_arn) {
                (perm.resource_type.clone(), perm.version.clone())
            } else {
                // Check built-in permissions
                let builtin = crate::state::get_aws_managed_permissions();
                if let Some(p) = builtin.iter().find(|p| p.arn == permission_arn) {
                    (p.resource_type.clone(), p.version.clone())
                } else {
                    return Err(RamError::PermissionNotFound(permission_arn.to_string()));
                }
            };

        let _ = rs; // release borrow
        let now = Utc::now().timestamp() as f64;

        // Remove existing entry for this share if any
        self.share_permissions
            .retain(|sp| sp.resource_share_arn != resource_share_arn);

        self.share_permissions.push(ResourceSharePermissionEntry {
            permission_arn: permission_arn.to_string(),
            permission_version: version,
            resource_share_arn: resource_share_arn.to_string(),
            resource_type,
            default_version: true,
            last_updated_time: now,
            status: "ATTACHABLE".to_string(),
        });

        Ok(true)
    }

    pub fn disassociate_resource_share_permission(
        &mut self,
        resource_share_arn: &str,
        permission_arn: &str,
    ) -> Result<bool, RamError> {
        self.resource_shares
            .get(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;
        self.share_permissions.retain(|sp| {
            !(sp.resource_share_arn == resource_share_arn && sp.permission_arn == permission_arn)
        });
        Ok(true)
    }

    pub fn list_resource_share_permissions(
        &self,
        resource_share_arn: &str,
    ) -> Result<Vec<&ResourceSharePermissionEntry>, RamError> {
        self.resource_shares
            .get(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;
        Ok(self
            .share_permissions
            .iter()
            .filter(|sp| sp.resource_share_arn == resource_share_arn)
            .collect())
    }

    pub fn list_permission_associations(
        &self,
        permission_arn: Option<&str>,
        resource_type: Option<&str>,
    ) -> Vec<&ResourceSharePermissionEntry> {
        self.share_permissions
            .iter()
            .filter(|sp| {
                permission_arn.is_none_or(|a| sp.permission_arn == a)
                    && resource_type.is_none_or(|rt| sp.resource_type == rt)
            })
            .collect()
    }

    pub fn create_permission(
        &mut self,
        name: &str,
        resource_type: &str,
        policy_template: &str,
        tags: Vec<Tag>,
        account_id: &str,
    ) -> Result<&CustomerManagedPermission, RamError> {
        let now = Utc::now().timestamp() as f64;
        let arn = format!("arn:aws:ram::{}:permission/{}", account_id, name);
        let perm = CustomerManagedPermission {
            arn: arn.clone(),
            name: name.to_string(),
            resource_type: resource_type.to_string(),
            policy_template: policy_template.to_string(),
            default_version: true,
            version: "1".to_string(),
            status: "ATTACHABLE".to_string(),
            creation_time: now,
            last_updated_time: now,
            permission_type: "CUSTOMER_MANAGED".to_string(),
            tags,
        };
        self.customer_managed_permissions.insert(arn.clone(), perm);
        Ok(self.customer_managed_permissions.get(&arn).unwrap())
    }

    pub fn create_permission_version(
        &mut self,
        permission_arn: &str,
        policy_template: &str,
    ) -> Result<CustomerManagedPermission, RamError> {
        let perm = self
            .customer_managed_permissions
            .get_mut(permission_arn)
            .ok_or_else(|| RamError::PermissionNotFound(permission_arn.to_string()))?;
        let now = Utc::now().timestamp() as f64;
        let new_version = (perm.version.parse::<u32>().unwrap_or(1) + 1).to_string();
        perm.policy_template = policy_template.to_string();
        perm.version = new_version;
        perm.last_updated_time = now;
        Ok(perm.clone())
    }

    pub fn delete_permission(&mut self, permission_arn: &str) -> Result<(), RamError> {
        self.customer_managed_permissions
            .remove(permission_arn)
            .ok_or_else(|| RamError::PermissionNotFound(permission_arn.to_string()))?;
        Ok(())
    }

    pub fn delete_permission_version(
        &mut self,
        permission_arn: &str,
        _permission_version: Option<i32>,
    ) -> Result<bool, RamError> {
        let perm = self
            .customer_managed_permissions
            .get(permission_arn)
            .ok_or_else(|| RamError::PermissionNotFound(permission_arn.to_string()))?;
        let _ = perm;
        // In real AWS, deleting the default version fails; here we succeed
        Ok(true)
    }

    pub fn get_permission(
        &self,
        permission_arn: &str,
    ) -> Result<(String, String, String, String, bool, bool, f64, f64, String), RamError> {
        // Returns (arn, name, resource_type, policy_template, default_version, is_rt_default, creation_time, last_updated_time, version)
        if let Some(perm) = self.customer_managed_permissions.get(permission_arn) {
            return Ok((
                perm.arn.clone(),
                perm.name.clone(),
                perm.resource_type.clone(),
                perm.policy_template.clone(),
                perm.default_version,
                false,
                perm.creation_time,
                perm.last_updated_time,
                perm.version.clone(),
            ));
        }
        let builtin = get_aws_managed_permissions();
        if let Some(p) = builtin.iter().find(|p| p.arn == permission_arn) {
            return Ok((
                p.arn.clone(),
                p.name.clone(),
                p.resource_type.clone(),
                "{}".to_string(), // real AWS would have policy JSON
                p.default_version,
                p.is_resource_type_default,
                p.creation_time,
                p.last_updated_time,
                p.version.clone(),
            ));
        }
        Err(RamError::PermissionNotFound(permission_arn.to_string()))
    }

    pub fn list_permission_versions(
        &self,
        permission_arn: &str,
    ) -> Result<Vec<RamPermission>, RamError> {
        if let Some(perm) = self.customer_managed_permissions.get(permission_arn) {
            // Return a summary for each version (we only track the latest in our simplified impl)
            return Ok(vec![RamPermission {
                arn: perm.arn.clone(),
                name: perm.name.clone(),
                resource_type: perm.resource_type.clone(),
                default_version: perm.default_version,
                is_resource_type_default: false,
                version: perm.version.clone(),
                status: perm.status.clone(),
                creation_time: perm.creation_time,
                last_updated_time: perm.last_updated_time,
                permission_type: perm.permission_type.clone(),
            }]);
        }
        let builtin = get_aws_managed_permissions();
        if let Some(p) = builtin.iter().find(|p| p.arn == permission_arn) {
            return Ok(vec![p.clone()]);
        }
        Err(RamError::PermissionNotFound(permission_arn.to_string()))
    }

    pub fn promote_permission_created_from_policy(
        &mut self,
        permission_arn: &str,
        name: &str,
        account_id: &str,
    ) -> Result<&CustomerManagedPermission, RamError> {
        // Get the original permission details before creating a new one
        let (resource_type, policy_template, creation_time, last_updated_time) = {
            let perm = self
                .customer_managed_permissions
                .get(permission_arn)
                .ok_or_else(|| RamError::PermissionNotFound(permission_arn.to_string()))?;
            (
                perm.resource_type.clone(),
                perm.policy_template.clone(),
                perm.creation_time,
                perm.last_updated_time,
            )
        };
        let new_arn = format!("arn:aws:ram::{}:permission/{}", account_id, name);
        let promoted = CustomerManagedPermission {
            arn: new_arn.clone(),
            name: name.to_string(),
            resource_type,
            policy_template,
            default_version: true,
            version: "1".to_string(),
            status: "ATTACHABLE".to_string(),
            creation_time,
            last_updated_time,
            permission_type: "CUSTOMER_MANAGED".to_string(),
            tags: Vec::new(),
        };
        self.customer_managed_permissions
            .insert(new_arn.clone(), promoted);
        Ok(self.customer_managed_permissions.get(&new_arn).unwrap())
    }

    pub fn promote_resource_share_created_from_policy(
        &mut self,
        resource_share_arn: &str,
    ) -> Result<bool, RamError> {
        let rs = self
            .resource_shares
            .get_mut(resource_share_arn)
            .ok_or_else(|| RamError::ResourceShareNotFound(resource_share_arn.to_string()))?;
        rs.last_updated_time = Utc::now().timestamp() as f64;
        Ok(true)
    }

    pub fn replace_permission_associations(
        &mut self,
        from_permission_arn: &str,
        to_permission_arn: &str,
        from_permission_version: Option<i32>,
    ) -> Result<ReplacePermissionWork, RamError> {
        // Verify from-permission exists
        let from_version = {
            if let Some(perm) = self.customer_managed_permissions.get(from_permission_arn) {
                perm.version.clone()
            } else {
                let builtin = get_aws_managed_permissions();
                if let Some(p) = builtin.iter().find(|p| p.arn == from_permission_arn) {
                    p.version.clone()
                } else {
                    return Err(RamError::PermissionNotFound(
                        from_permission_arn.to_string(),
                    ));
                }
            }
        };
        let to_version = {
            if let Some(perm) = self.customer_managed_permissions.get(to_permission_arn) {
                perm.version.clone()
            } else {
                let builtin = get_aws_managed_permissions();
                if let Some(p) = builtin.iter().find(|p| p.arn == to_permission_arn) {
                    p.version.clone()
                } else {
                    return Err(RamError::PermissionNotFound(to_permission_arn.to_string()));
                }
            }
        };
        let now = Utc::now().timestamp() as f64;
        let work = ReplacePermissionWork {
            id: uuid::Uuid::new_v4().to_string(),
            from_permission_arn: from_permission_arn.to_string(),
            from_permission_version: from_permission_version
                .map(|v| v.to_string())
                .unwrap_or(from_version),
            to_permission_arn: to_permission_arn.to_string(),
            to_permission_version: to_version,
            status: "IN_PROGRESS".to_string(),
            creation_time: now,
            last_updated_time: now,
        };
        self.replace_permission_works.push(work.clone());
        Ok(work)
    }

    pub fn list_replace_permission_associations_work(
        &self,
        work_ids: &[String],
        status: Option<&str>,
    ) -> Vec<&ReplacePermissionWork> {
        self.replace_permission_works
            .iter()
            .filter(|w| {
                (work_ids.is_empty() || work_ids.contains(&w.id))
                    && status.is_none_or(|s| w.status == s)
            })
            .collect()
    }

    pub fn set_default_permission_version(
        &mut self,
        permission_arn: &str,
        _permission_version: i32,
    ) -> Result<bool, RamError> {
        if !self
            .customer_managed_permissions
            .contains_key(permission_arn)
        {
            return Err(RamError::PermissionNotFound(permission_arn.to_string()));
        }
        Ok(true)
    }

    // ===== INVITATION OPERATIONS =====

    pub fn get_resource_share_invitations(
        &self,
        resource_share_arns: &[String],
        invitation_arns: &[String],
    ) -> Vec<&ResourceShareInvitation> {
        self.invitations
            .iter()
            .filter(|inv| {
                (resource_share_arns.is_empty()
                    || resource_share_arns.contains(&inv.resource_share_arn))
                    && (invitation_arns.is_empty() || invitation_arns.contains(&inv.invitation_arn))
            })
            .collect()
    }

    pub fn accept_resource_share_invitation(
        &mut self,
        invitation_arn: &str,
    ) -> Result<&ResourceShareInvitation, RamError> {
        let inv = self
            .invitations
            .iter_mut()
            .find(|i| i.invitation_arn == invitation_arn)
            .ok_or_else(|| RamError::InvitationNotFound(invitation_arn.to_string()))?;
        if inv.status != "PENDING" {
            return Err(RamError::InvitationAlreadyResponded(
                invitation_arn.to_string(),
            ));
        }
        inv.status = "ACCEPTED".to_string();
        // Find by arn again (borrow issue workaround)
        Ok(self
            .invitations
            .iter()
            .find(|i| i.invitation_arn == invitation_arn)
            .unwrap())
    }

    pub fn reject_resource_share_invitation(
        &mut self,
        invitation_arn: &str,
    ) -> Result<&ResourceShareInvitation, RamError> {
        let inv = self
            .invitations
            .iter_mut()
            .find(|i| i.invitation_arn == invitation_arn)
            .ok_or_else(|| RamError::InvitationNotFound(invitation_arn.to_string()))?;
        if inv.status != "PENDING" {
            return Err(RamError::InvitationAlreadyResponded(
                invitation_arn.to_string(),
            ));
        }
        inv.status = "REJECTED".to_string();
        Ok(self
            .invitations
            .iter()
            .find(|i| i.invitation_arn == invitation_arn)
            .unwrap())
    }

    pub fn list_pending_invitation_resources(
        &self,
        invitation_arn: &str,
    ) -> Result<Vec<&crate::types::Resource>, RamError> {
        let inv = self
            .invitations
            .iter()
            .find(|i| i.invitation_arn == invitation_arn)
            .ok_or_else(|| RamError::InvitationNotFound(invitation_arn.to_string()))?;
        if inv.status != "PENDING" {
            return Err(RamError::InvitationAlreadyResponded(
                invitation_arn.to_string(),
            ));
        }
        Ok(self
            .resources
            .iter()
            .filter(|r| r.resource_share_arn == inv.resource_share_arn)
            .collect())
    }

    // ===== LIST PRINCIPALS =====

    pub fn list_principals(
        &self,
        resource_owner: &str,
        resource_share_arns: &[String],
        principals: &[String],
        account_id: &str,
    ) -> Result<Vec<crate::types::ResourceShareAssociation>, RamError> {
        if resource_owner != "SELF" && resource_owner != "OTHER-ACCOUNTS" {
            return Err(RamError::InvalidResourceOwner(resource_owner.to_string()));
        }
        Ok(self
            .associations
            .iter()
            .filter(|a| {
                a.association_type == "PRINCIPAL"
                    && (resource_share_arns.is_empty()
                        || resource_share_arns.contains(&a.resource_share_arn))
                    && (principals.is_empty() || principals.contains(&a.associated_entity))
                    && {
                        if let Some(rs) = self.resource_shares.get(&a.resource_share_arn) {
                            match resource_owner {
                                "SELF" => rs.owning_account_id == account_id,
                                "OTHER-ACCOUNTS" => rs.owning_account_id != account_id,
                                _ => true,
                            }
                        } else {
                            false
                        }
                    }
            })
            .cloned()
            .collect())
    }

    // ===== GET RESOURCE POLICIES =====

    pub fn get_resource_policies(&self, resource_arns: &[String]) -> Vec<String> {
        // For each ARN that exists in our resources, return a minimal policy JSON
        resource_arns
            .iter()
            .filter(|arn| self.resources.iter().any(|r| &r.arn == *arn))
            .map(|arn| format!(r#"{{"Version":"2012-10-17","Statement":[{{"Effect":"Allow","Action":"*","Resource":"{}"}}]}}"#, arn))
            .collect()
    }

    // ===== LIST SOURCE ASSOCIATIONS =====

    pub fn list_source_associations(
        &self,
        _resource_share_arns: &[String],
    ) -> Vec<crate::types::ResourceShareAssociation> {
        // Source associations are a more advanced feature; return empty list for now
        Vec::new()
    }
}

fn extract_resource_type(arn: &str) -> String {
    // Parse ARN: arn:aws:service:region:account:resource-type/resource-id
    let parts: Vec<&str> = arn.splitn(6, ':').collect();
    if parts.len() >= 6 {
        let service = parts[2];
        let resource_part = parts[5];
        let resource_type_part = resource_part.split('/').next().unwrap_or(resource_part);
        format!("{}:{}", service, resource_type_part)
    } else if parts.len() >= 3 {
        parts[2].to_string()
    } else {
        "unknown".to_string()
    }
}

fn make_permission(
    name: &str,
    resource_type: &str,
    version: &str,
    is_resource_type_default: bool,
    creation_time: f64,
    permission_type: &str,
) -> RamPermission {
    RamPermission {
        arn: format!("arn:aws:ram::aws:permission/{}", name),
        name: name.to_string(),
        resource_type: resource_type.to_string(),
        default_version: true,
        is_resource_type_default,
        version: version.to_string(),
        status: "ATTACHABLE".to_string(),
        creation_time,
        last_updated_time: creation_time,
        permission_type: permission_type.to_string(),
    }
}

fn get_aws_managed_permissions() -> Vec<RamPermission> {
    let now = Utc::now().timestamp() as f64;
    vec![
        make_permission(
            "AWSRAMDefaultPermissionRDSCluster",
            "rds:Cluster",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionImageBuilderComponent",
            "imagebuilder:Component",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionsNetworkManagerCoreNetwork",
            "networkmanager:CoreNetwork",
            "3",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMTransitGatewayPermissionsNetworkManagerCoreNetwork",
            "networkmanager:CoreNetwork",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMVPCPermissionsNetworkManagerCoreNetwork",
            "networkmanager:CoreNetwork",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionResourceGroup",
            "resource-groups:Group",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionImageBuilderImage",
            "imagebuilder:Image",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionImageBuilderImageRecipe",
            "imagebuilder:ImageRecipe",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionLicenseConfiguration",
            "license-manager:LicenseConfiguration",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionAppMesh",
            "appmesh:Mesh",
            "3",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionPrefixList",
            "ec2:PrefixList",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionCodeBuildProject",
            "codebuild:Project",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionCodeBuildReportGroup",
            "codebuild:ReportGroup",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionResolverRule",
            "route53resolver:ResolverRule",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionSubnet",
            "ec2:Subnet",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionTransitGatewayMulticastDomain",
            "ec2:TransitGatewayMulticastDomain",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionGlueDatabase",
            "glue:Database",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMLFEnabledGlueAllTablesReadWriteForDatabase",
            "glue:Database",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMLFEnabledGlueDatabaseReadWrite",
            "glue:Database",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueAllTablesReadWriteForDatabase",
            "glue:Database",
            "3",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueDatabaseReadWrite",
            "glue:Database",
            "3",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueTableReadWriteForDatabase",
            "glue:Database",
            "3",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionLFTagGlueDatabaseReadWrite",
            "glue:Database",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionLFTagGlueTableReadWriteForDatabase",
            "glue:Database",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionGlueTable",
            "glue:Table",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMLFEnabledGlueDatabaseReadWriteForTable",
            "glue:Table",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMLFEnabledGlueTableReadWrite",
            "glue:Table",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueDatabaseReadWriteForTable",
            "glue:Table",
            "2",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueTableReadWrite",
            "glue:Table",
            "2",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionLFTagGlueDatabaseReadWriteForTable",
            "glue:Table",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionLFTagGlueTableReadWrite",
            "glue:Table",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMDefaultPermissionGlueCatalog",
            "glue:Catalog",
            "1",
            true,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueAllTablesReadWriteForCatalog",
            "glue:Catalog",
            "3",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueDatabaseReadWriteForCatalog",
            "glue:Catalog",
            "3",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionGlueTableReadWriteForCatalog",
            "glue:Catalog",
            "3",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionLFTagGlueDatabaseReadWriteForCatalog",
            "glue:Catalog",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
        make_permission(
            "AWSRAMPermissionLFTagGlueTableReadWriteForCatalog",
            "glue:Catalog",
            "1",
            false,
            now,
            "AWS_MANAGED",
        ),
    ]
}
