use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{RamError, RamState};
use crate::types::Tag;
use crate::views::RamStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct RamService {
    pub(crate) state: Arc<BackendState<RamState>>,
    pub(crate) notifier: StateChangeNotifier<RamStateView>,
}

impl RamService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for RamService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RamService {
    fn service_name(&self) -> &str {
        "ram"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://ram\..*\.amazonaws\.com",
            r"https?://ram\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl RamService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = winterbaume_core::extract_path(&request.uri);
        let query_str = winterbaume_core::extract_query_string(&request.uri);
        let query: HashMap<String, String> = winterbaume_core::parse_query_string(query_str);
        let labels: &[(&str, &str)] = &[];
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        match (method, segments.as_slice()) {
            // POST /createresourceshare - CreateResourceShare
            ("POST", ["createresourceshare"]) => {
                self.handle_create_resource_share(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            // POST /getresourceshares - GetResourceShares
            ("POST", ["getresourceshares"]) => {
                self.handle_get_resource_shares(&state, &request, labels, &query, account_id)
                    .await
            }
            // DELETE /deleteresourceshare?resourceShareArn=... - DeleteResourceShare
            ("DELETE", ["deleteresourceshare"]) => {
                self.handle_delete_resource_share(&state, &request, labels, &query)
                    .await
            }
            // POST /listresources - ListResources
            ("POST", ["listresources"]) => {
                self.handle_list_resources(&state, &request, labels, &query, account_id)
                    .await
            }
            // POST /enablesharingwithawsorganization - EnableSharingWithAwsOrganization
            ("POST", ["enablesharingwithawsorganization"]) => {
                self.handle_enable_sharing_with_aws_organization(&state, &request, labels, &query)
                    .await
            }
            // POST /getresourceshareassociations - GetResourceShareAssociations
            ("POST", ["getresourceshareassociations"]) => {
                self.handle_get_resource_share_associations(&state, &request, labels, &query)
                    .await
            }
            // POST /listpermissions - ListPermissions
            ("POST", ["listpermissions"]) => {
                self.handle_list_permissions(&state, &request, labels, &query)
                    .await
            }
            // POST /listresourcetypes - ListResourceTypes
            ("POST", ["listresourcetypes"]) => {
                self.handle_list_resource_types(&state, &request, labels, &query)
                    .await
            }
            // POST /updateresourceshare - UpdateResourceShare
            ("POST", ["updateresourceshare"]) => {
                self.handle_update_resource_share(&state, &request, labels, &query)
                    .await
            }
            // POST /acceptresourceshareinvitation - AcceptResourceShareInvitation
            ("POST", ["acceptresourceshareinvitation"]) => {
                self.handle_accept_resource_share_invitation(&state, &request, labels, &query)
                    .await
            }
            // POST /associateresourceshare - AssociateResourceShare
            ("POST", ["associateresourceshare"]) => {
                self.handle_associate_resource_share(&state, &request, labels, &query)
                    .await
            }
            // POST /associateresourcesharepermission - AssociateResourceSharePermission
            ("POST", ["associateresourcesharepermission"]) => {
                self.handle_associate_resource_share_permission(&state, &request, labels, &query)
                    .await
            }
            // POST /createpermission - CreatePermission
            ("POST", ["createpermission"]) => {
                self.handle_create_permission(&state, &request, labels, &query, account_id)
                    .await
            }
            // POST /createpermissionversion - CreatePermissionVersion
            ("POST", ["createpermissionversion"]) => {
                self.handle_create_permission_version(&state, &request, labels, &query)
                    .await
            }
            // DELETE /deletepermission - DeletePermission
            ("DELETE", ["deletepermission"]) => {
                self.handle_delete_permission(&state, &request, labels, &query)
                    .await
            }
            // DELETE /deletepermissionversion - DeletePermissionVersion
            ("DELETE", ["deletepermissionversion"]) => {
                self.handle_delete_permission_version(&state, &request, labels, &query)
                    .await
            }
            // POST /disassociateresourceshare - DisassociateResourceShare
            ("POST", ["disassociateresourceshare"]) => {
                self.handle_disassociate_resource_share(&state, &request, labels, &query)
                    .await
            }
            // POST /disassociateresourcesharepermission - DisassociateResourceSharePermission
            ("POST", ["disassociateresourcesharepermission"]) => {
                self.handle_disassociate_resource_share_permission(&state, &request, labels, &query)
                    .await
            }
            // POST /getpermission - GetPermission
            ("POST", ["getpermission"]) => {
                self.handle_get_permission(&state, &request, labels, &query)
                    .await
            }
            // POST /getresourcepolicies - GetResourcePolicies
            ("POST", ["getresourcepolicies"]) => {
                self.handle_get_resource_policies(&state, &request, labels, &query)
                    .await
            }
            // POST /getresourceshareinvitations - GetResourceShareInvitations
            ("POST", ["getresourceshareinvitations"]) => {
                self.handle_get_resource_share_invitations(&state, &request, labels, &query)
                    .await
            }
            // POST /listpendinginvitationresources - ListPendingInvitationResources
            ("POST", ["listpendinginvitationresources"]) => {
                self.handle_list_pending_invitation_resources(&state, &request, labels, &query)
                    .await
            }
            // POST /listpermissionassociations - ListPermissionAssociations
            ("POST", ["listpermissionassociations"]) => {
                self.handle_list_permission_associations(&state, &request, labels, &query)
                    .await
            }
            // POST /listpermissionversions - ListPermissionVersions
            ("POST", ["listpermissionversions"]) => {
                self.handle_list_permission_versions(&state, &request, labels, &query)
                    .await
            }
            // POST /listprincipals - ListPrincipals
            ("POST", ["listprincipals"]) => {
                self.handle_list_principals(&state, &request, labels, &query, account_id)
                    .await
            }
            // POST /listreplacepermissionassociationswork - ListReplacePermissionAssociationsWork
            ("POST", ["listreplacepermissionassociationswork"]) => {
                self.handle_list_replace_permission_associations_work(
                    &state, &request, labels, &query,
                )
                .await
            }
            // POST /listresourcesharepermissions - ListResourceSharePermissions
            ("POST", ["listresourcesharepermissions"]) => {
                self.handle_list_resource_share_permissions(&state, &request, labels, &query)
                    .await
            }
            // POST /listsourceassociations - ListSourceAssociations
            ("POST", ["listsourceassociations"]) => {
                self.handle_list_source_associations(&state, &request, labels, &query)
                    .await
            }
            // POST /promotepermissioncreatedfrompolicy - PromotePermissionCreatedFromPolicy
            ("POST", ["promotepermissioncreatedfrompolicy"]) => {
                self.handle_promote_permission_created_from_policy(
                    &state, &request, labels, &query, account_id,
                )
                .await
            }
            // POST /promoteresourcesharecreatedfrompolicy - PromoteResourceShareCreatedFromPolicy
            ("POST", ["promoteresourcesharecreatedfrompolicy"]) => {
                self.handle_promote_resource_share_created_from_policy(
                    &state, &request, labels, &query,
                )
                .await
            }
            // POST /rejectresourceshareinvitation - RejectResourceShareInvitation
            ("POST", ["rejectresourceshareinvitation"]) => {
                self.handle_reject_resource_share_invitation(&state, &request, labels, &query)
                    .await
            }
            // POST /replacepermissionassociations - ReplacePermissionAssociations
            ("POST", ["replacepermissionassociations"]) => {
                self.handle_replace_permission_associations(&state, &request, labels, &query)
                    .await
            }
            // POST /setdefaultpermissionversion - SetDefaultPermissionVersion
            ("POST", ["setdefaultpermissionversion"]) => {
                self.handle_set_default_permission_version(&state, &request, labels, &query)
                    .await
            }
            // POST /tagresource - TagResource
            ("POST", ["tagresource"]) => {
                self.handle_tag_resource(&state, &request, labels, &query)
                    .await
            }
            // POST /untagresource - UntagResource
            ("POST", ["untagresource"]) => {
                self.handle_untag_resource(&state, &request, labels, &query)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_resource_share(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resource_share_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "name is required");
        }
        let name = input.name.as_str();
        let resource_arns: Vec<String> = input.resource_arns.unwrap_or_default();
        let principals: Vec<String> = input.principals.unwrap_or_default();
        let allow_external_principals = input.allow_external_principals.unwrap_or(true);
        let tags: Vec<Tag> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| {
                let key = t.key?;
                let value = t.value?;
                Some(Tag { key, value })
            })
            .collect();

        let mut state = state.write().await;
        match state.create_resource_share(
            name,
            resource_arns,
            principals,
            allow_external_principals,
            tags,
            account_id,
            region,
        ) {
            Ok(rs) => {
                wire::serialize_create_resource_share_response(&wire::CreateResourceShareResponse {
                    resource_share: Some(wire::ResourceShare {
                        resource_share_arn: Some(rs.resource_share_arn.clone()),
                        name: Some(rs.name.clone()),
                        owning_account_id: Some(rs.owning_account_id.clone()),
                        allow_external_principals: Some(rs.allow_external_principals),
                        status: Some(rs.status.clone()),
                        creation_time: Some(rs.creation_time),
                        last_updated_time: Some(rs.last_updated_time),
                        tags: Some(
                            rs.tags
                                .iter()
                                .map(|t| wire::Tag {
                                    key: Some(t.key.clone()),
                                    value: Some(t.value.clone()),
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_get_resource_shares(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_shares_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_owner = if input.resource_owner.is_empty() {
            "SELF"
        } else {
            input.resource_owner.as_str()
        };

        let state = state.read().await;
        match state.get_resource_shares(resource_owner, account_id) {
            Ok(shares) => {
                wire::serialize_get_resource_shares_response(&wire::GetResourceSharesResponse {
                    resource_shares: Some(
                        shares
                            .iter()
                            .map(|rs| wire::ResourceShare {
                                resource_share_arn: Some(rs.resource_share_arn.clone()),
                                name: Some(rs.name.clone()),
                                owning_account_id: Some(rs.owning_account_id.clone()),
                                allow_external_principals: Some(rs.allow_external_principals),
                                status: Some(rs.status.clone()),
                                feature_set: Some("STANDARD".to_string()),
                                creation_time: Some(rs.creation_time),
                                last_updated_time: Some(rs.last_updated_time),
                                tags: Some(
                                    rs.tags
                                        .iter()
                                        .map(|t| wire::Tag {
                                            key: Some(t.key.clone()),
                                            value: Some(t.value.clone()),
                                        })
                                        .collect(),
                                ),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                })
            }
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_delete_resource_share(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_share_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let mut state = state.write().await;
        match state.delete_resource_share(resource_share_arn) {
            Ok(()) => {
                wire::serialize_delete_resource_share_response(&wire::DeleteResourceShareResponse {
                    return_value: Some(true),
                    ..Default::default()
                })
            }
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_list_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_owner = if input.resource_owner.is_empty() {
            "SELF"
        } else {
            input.resource_owner.as_str()
        };

        let resource_share_arns: Vec<String> = input.resource_share_arns.unwrap_or_default();

        let state = state.read().await;
        let resources = state.list_resources(resource_owner, &resource_share_arns, account_id);
        wire::serialize_list_resources_response(&wire::ListResourcesResponse {
            resources: Some(
                resources
                    .iter()
                    .map(|r| wire::Resource {
                        arn: Some(r.arn.clone()),
                        r#type: Some(r.r#type.clone()),
                        resource_share_arn: Some(r.resource_share_arn.clone()),
                        status: Some(r.status.clone()),
                        creation_time: Some(r.creation_time),
                        last_updated_time: Some(r.last_updated_time),
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }
    async fn handle_enable_sharing_with_aws_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_enable_sharing_with_aws_organization_request(request, labels, query)
        {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        let mut state = state.write().await;
        let result = state.enable_sharing_with_aws_organization();
        wire::serialize_enable_sharing_with_aws_organization_response(
            &wire::EnableSharingWithAwsOrganizationResponse {
                return_value: Some(result),
            },
        )
    }

    async fn handle_get_resource_share_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_resource_share_associations_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        if input.association_type.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "associationType is required",
            );
        }
        let association_type = input.association_type.as_str();
        let resource_share_arns: Vec<String> = input.resource_share_arns.unwrap_or_default();
        let principal = input.principal.as_deref();
        let resource_arn = input.resource_arn.as_deref();
        let association_status = input.association_status.as_deref();

        let state = state.read().await;
        match state.get_resource_share_associations(
            association_type,
            &resource_share_arns,
            principal,
            resource_arn,
            association_status,
        ) {
            Ok(associations) => wire::serialize_get_resource_share_associations_response(
                &wire::GetResourceShareAssociationsResponse {
                    resource_share_associations: Some(
                        associations
                            .iter()
                            .map(|a| wire::ResourceShareAssociation {
                                resource_share_arn: Some(a.resource_share_arn.clone()),
                                resource_share_name: Some(a.resource_share_name.clone()),
                                associated_entity: Some(a.associated_entity.clone()),
                                association_type: Some(a.association_type.clone()),
                                status: Some(a.status.clone()),
                                creation_time: Some(a.creation_time),
                                last_updated_time: Some(a.last_updated_time),
                                external: Some(a.external),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_list_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_permissions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_type = input.resource_type.as_deref();
        let permission_type = input.permission_type.as_deref();

        let state = state.read().await;
        match state.list_permissions(resource_type, permission_type) {
            Ok(permissions) => {
                wire::serialize_list_permissions_response(&wire::ListPermissionsResponse {
                    permissions: Some(
                        permissions
                            .iter()
                            .map(|p| wire::ResourceSharePermissionSummary {
                                arn: Some(p.arn.clone()),
                                name: Some(p.name.clone()),
                                resource_type: Some(p.resource_type.clone()),
                                default_version: Some(p.default_version),
                                is_resource_type_default: Some(p.is_resource_type_default),
                                version: Some(p.version.clone()),
                                status: Some(p.status.clone()),
                                creation_time: Some(p.creation_time),
                                last_updated_time: Some(p.last_updated_time),
                                permission_type: Some(p.permission_type.clone()),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                })
            }
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_list_resource_types(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_resource_types_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_region_scope = input.resource_region_scope.as_deref();

        let state = state.read().await;
        match state.list_resource_types(resource_region_scope) {
            Ok(resource_types) => {
                wire::serialize_list_resource_types_response(&wire::ListResourceTypesResponse {
                    resource_types: Some(
                        resource_types
                            .iter()
                            .map(|rt| wire::ServiceNameAndResourceType {
                                resource_type: Some(rt.resource_type.clone()),
                                service_name: Some(rt.service_name.clone()),
                                resource_region_scope: Some(rt.resource_region_scope.clone()),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                })
            }
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_update_resource_share(
        &self,
        state: &Arc<tokio::sync::RwLock<RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_resource_share_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let name = input.name.as_deref();
        let allow_external_principals = input.allow_external_principals;

        let mut state = state.write().await;
        match state.update_resource_share(resource_share_arn, name, allow_external_principals) {
            Ok(rs) => {
                wire::serialize_update_resource_share_response(&wire::UpdateResourceShareResponse {
                    resource_share: Some(wire::ResourceShare {
                        resource_share_arn: Some(rs.resource_share_arn.clone()),
                        name: Some(rs.name.clone()),
                        owning_account_id: Some(rs.owning_account_id.clone()),
                        allow_external_principals: Some(rs.allow_external_principals),
                        status: Some(rs.status.clone()),
                        creation_time: Some(rs.creation_time),
                        last_updated_time: Some(rs.last_updated_time),
                        tags: Some(
                            rs.tags
                                .iter()
                                .map(|t| wire::Tag {
                                    key: Some(t.key.clone()),
                                    value: Some(t.value.clone()),
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_accept_resource_share_invitation(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_accept_resource_share_invitation_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_invitation_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareInvitationArn is required",
            );
        }
        let invitation_arn = input.resource_share_invitation_arn.as_str();
        let mut state = state.write().await;
        match state.accept_resource_share_invitation(invitation_arn) {
            Ok(inv) => wire::serialize_accept_resource_share_invitation_response(
                &wire::AcceptResourceShareInvitationResponse {
                    resource_share_invitation: Some(wire::ResourceShareInvitation {
                        resource_share_invitation_arn: Some(inv.invitation_arn.clone()),
                        resource_share_arn: Some(inv.resource_share_arn.clone()),
                        resource_share_name: Some(inv.resource_share_name.clone()),
                        sender_account_id: Some(inv.sender_account_id.clone()),
                        receiver_account_id: Some(inv.receiver_account_id.clone()),
                        status: Some(inv.status.clone()),
                        invitation_timestamp: Some(inv.invitation_timestamp),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_associate_resource_share(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_resource_share_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let resource_arns: Vec<String> = input.resource_arns.unwrap_or_default();
        let principals: Vec<String> = input.principals.unwrap_or_default();

        let mut state = state.write().await;
        match state.associate_resource_share(resource_share_arn, resource_arns, principals) {
            Ok(assocs) => wire::serialize_associate_resource_share_response(
                &wire::AssociateResourceShareResponse {
                    resource_share_associations: Some(
                        assocs
                            .iter()
                            .map(|a| wire::ResourceShareAssociation {
                                resource_share_arn: Some(a.resource_share_arn.clone()),
                                resource_share_name: Some(a.resource_share_name.clone()),
                                associated_entity: Some(a.associated_entity.clone()),
                                association_type: Some(a.association_type.clone()),
                                status: Some(a.status.clone()),
                                creation_time: Some(a.creation_time),
                                last_updated_time: Some(a.last_updated_time),
                                external: Some(a.external),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_associate_resource_share_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_resource_share_permission_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let permission_arn = input.permission_arn.as_str();
        let mut state = state.write().await;
        match state.associate_resource_share_permission(resource_share_arn, permission_arn) {
            Ok(rv) => wire::serialize_associate_resource_share_permission_response(
                &wire::AssociateResourceSharePermissionResponse {
                    return_value: Some(rv),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_create_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_permission_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "name is required");
        }
        if input.resource_type.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "resourceType is required");
        }
        if input.policy_template.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "policyTemplate is required",
            );
        }
        let name = input.name.as_str();
        let resource_type = input.resource_type.as_str();
        let policy_template = input.policy_template.as_str();
        let tags: Vec<Tag> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| {
                let key = t.key?;
                let value = t.value?;
                Some(Tag { key, value })
            })
            .collect();

        let mut state = state.write().await;
        match state.create_permission(name, resource_type, policy_template, tags, account_id) {
            Ok(perm) => {
                wire::serialize_create_permission_response(&wire::CreatePermissionResponse {
                    permission: Some(wire::ResourceSharePermissionSummary {
                        arn: Some(perm.arn.clone()),
                        name: Some(perm.name.clone()),
                        resource_type: Some(perm.resource_type.clone()),
                        default_version: Some(perm.default_version),
                        is_resource_type_default: Some(false),
                        version: Some(perm.version.clone()),
                        status: Some(perm.status.clone()),
                        creation_time: Some(perm.creation_time),
                        last_updated_time: Some(perm.last_updated_time),
                        permission_type: Some(perm.permission_type.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_create_permission_version(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_permission_version_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        if input.policy_template.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "policyTemplate is required",
            );
        }
        let permission_arn = input.permission_arn.as_str();
        let policy_template = input.policy_template.as_str();
        let mut state = state.write().await;
        match state.create_permission_version(permission_arn, policy_template) {
            Ok(perm) => wire::serialize_create_permission_version_response(
                &wire::CreatePermissionVersionResponse {
                    permission: Some(wire::ResourceSharePermissionDetail {
                        arn: Some(perm.arn.clone()),
                        name: Some(perm.name.clone()),
                        resource_type: Some(perm.resource_type.clone()),
                        default_version: Some(perm.default_version),
                        is_resource_type_default: Some(false),
                        version: Some(perm.version.clone()),
                        status: Some(perm.status.clone()),
                        creation_time: Some(perm.creation_time),
                        last_updated_time: Some(perm.last_updated_time),
                        permission: Some(perm.policy_template.clone()),
                        permission_type: Some(perm.permission_type.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_delete_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_permission_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        let permission_arn = input.permission_arn.as_str();
        let mut state = state.write().await;
        match state.delete_permission(permission_arn) {
            Ok(()) => wire::serialize_delete_permission_response(&wire::DeletePermissionResponse {
                return_value: Some(true),
                ..Default::default()
            }),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_delete_permission_version(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_permission_version_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        let permission_arn = input.permission_arn.as_str();
        let permission_version = if query.contains_key("permissionVersion") {
            Some(input.permission_version)
        } else {
            None
        };
        let mut state = state.write().await;
        match state.delete_permission_version(permission_arn, permission_version) {
            Ok(rv) => wire::serialize_delete_permission_version_response(
                &wire::DeletePermissionVersionResponse {
                    return_value: Some(rv),
                    permission_status: Some("ATTACHABLE".to_string()),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_disassociate_resource_share(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_resource_share_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let resource_arns: Vec<String> = input.resource_arns.unwrap_or_default();
        let principals: Vec<String> = input.principals.unwrap_or_default();

        let mut state = state.write().await;
        match state.disassociate_resource_share(resource_share_arn, resource_arns, principals) {
            Ok(assocs) => wire::serialize_disassociate_resource_share_response(
                &wire::DisassociateResourceShareResponse {
                    resource_share_associations: Some(
                        assocs
                            .iter()
                            .map(|a| wire::ResourceShareAssociation {
                                resource_share_arn: Some(a.resource_share_arn.clone()),
                                resource_share_name: Some(a.resource_share_name.clone()),
                                associated_entity: Some(a.associated_entity.clone()),
                                association_type: Some(a.association_type.clone()),
                                status: Some(a.status.clone()),
                                creation_time: Some(a.creation_time),
                                last_updated_time: Some(a.last_updated_time),
                                external: Some(a.external),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_disassociate_resource_share_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_resource_share_permission_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let permission_arn = input.permission_arn.as_str();
        let mut state = state.write().await;
        match state.disassociate_resource_share_permission(resource_share_arn, permission_arn) {
            Ok(rv) => wire::serialize_disassociate_resource_share_permission_response(
                &wire::DisassociateResourceSharePermissionResponse {
                    return_value: Some(rv),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_get_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_permission_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        let permission_arn = input.permission_arn.as_str();
        let state = state.read().await;
        match state.get_permission(permission_arn) {
            Ok((
                arn,
                name,
                resource_type,
                policy_template,
                default_version,
                is_rt_default,
                creation_time,
                last_updated_time,
                version,
            )) => wire::serialize_get_permission_response(&wire::GetPermissionResponse {
                permission: Some(wire::ResourceSharePermissionDetail {
                    arn: Some(arn),
                    name: Some(name),
                    resource_type: Some(resource_type),
                    default_version: Some(default_version),
                    is_resource_type_default: Some(is_rt_default),
                    version: Some(version),
                    status: Some("ATTACHABLE".to_string()),
                    creation_time: Some(creation_time),
                    last_updated_time: Some(last_updated_time),
                    permission: Some(policy_template),
                    ..Default::default()
                }),
            }),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_get_resource_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policies_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_arns: Vec<String> = input.resource_arns;
        let state = state.read().await;
        let policies = state.get_resource_policies(&resource_arns);
        wire::serialize_get_resource_policies_response(&wire::GetResourcePoliciesResponse {
            policies: Some(policies),
            ..Default::default()
        })
    }

    async fn handle_get_resource_share_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_share_invitations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_share_arns: Vec<String> = input.resource_share_arns.unwrap_or_default();
        let invitation_arns: Vec<String> = input.resource_share_invitation_arns.unwrap_or_default();
        let state = state.read().await;
        let invitations =
            state.get_resource_share_invitations(&resource_share_arns, &invitation_arns);
        wire::serialize_get_resource_share_invitations_response(
            &wire::GetResourceShareInvitationsResponse {
                resource_share_invitations: Some(
                    invitations
                        .iter()
                        .map(|inv| wire::ResourceShareInvitation {
                            resource_share_invitation_arn: Some(inv.invitation_arn.clone()),
                            resource_share_arn: Some(inv.resource_share_arn.clone()),
                            resource_share_name: Some(inv.resource_share_name.clone()),
                            sender_account_id: Some(inv.sender_account_id.clone()),
                            receiver_account_id: Some(inv.receiver_account_id.clone()),
                            status: Some(inv.status.clone()),
                            invitation_timestamp: Some(inv.invitation_timestamp),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            },
        )
    }

    async fn handle_list_pending_invitation_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_pending_invitation_resources_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_invitation_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareInvitationArn is required",
            );
        }
        let invitation_arn = input.resource_share_invitation_arn.as_str();
        let state = state.read().await;
        match state.list_pending_invitation_resources(invitation_arn) {
            Ok(resources) => wire::serialize_list_pending_invitation_resources_response(
                &wire::ListPendingInvitationResourcesResponse {
                    resources: Some(
                        resources
                            .iter()
                            .map(|r| wire::Resource {
                                arn: Some(r.arn.clone()),
                                r#type: Some(r.r#type.clone()),
                                resource_share_arn: Some(r.resource_share_arn.clone()),
                                status: Some(r.status.clone()),
                                creation_time: Some(r.creation_time),
                                last_updated_time: Some(r.last_updated_time),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_list_permission_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_permission_associations_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        let permission_arn = input.permission_arn.as_deref();
        let resource_type = input.resource_type.as_deref();
        let state = state.read().await;
        let entries = state.list_permission_associations(permission_arn, resource_type);
        wire::serialize_list_permission_associations_response(
            &wire::ListPermissionAssociationsResponse {
                permissions: Some(
                    entries
                        .iter()
                        .map(|e| wire::AssociatedPermission {
                            arn: Some(e.permission_arn.clone()),
                            resource_share_arn: Some(e.resource_share_arn.clone()),
                            resource_type: Some(e.resource_type.clone()),
                            default_version: Some(e.default_version),
                            permission_version: Some(e.permission_version.clone()),
                            last_updated_time: Some(e.last_updated_time),
                            status: Some(e.status.clone()),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            },
        )
    }

    async fn handle_list_permission_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_permission_versions_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        let permission_arn = input.permission_arn.as_str();
        let state = state.read().await;
        match state.list_permission_versions(permission_arn) {
            Ok(permissions) => wire::serialize_list_permission_versions_response(
                &wire::ListPermissionVersionsResponse {
                    permissions: Some(
                        permissions
                            .iter()
                            .map(|p| wire::ResourceSharePermissionSummary {
                                arn: Some(p.arn.clone()),
                                name: Some(p.name.clone()),
                                resource_type: Some(p.resource_type.clone()),
                                default_version: Some(p.default_version),
                                is_resource_type_default: Some(p.is_resource_type_default),
                                version: Some(p.version.clone()),
                                status: Some(p.status.clone()),
                                creation_time: Some(p.creation_time),
                                last_updated_time: Some(p.last_updated_time),
                                permission_type: Some(p.permission_type.clone()),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_list_principals(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_principals_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_owner = if input.resource_owner.is_empty() {
            "SELF"
        } else {
            input.resource_owner.as_str()
        };
        let resource_share_arns: Vec<String> = input.resource_share_arns.unwrap_or_default();
        let principals: Vec<String> = input.principals.unwrap_or_default();
        let state = state.read().await;
        match state.list_principals(
            resource_owner,
            &resource_share_arns,
            &principals,
            account_id,
        ) {
            Ok(assocs) => wire::serialize_list_principals_response(&wire::ListPrincipalsResponse {
                principals: Some(
                    assocs
                        .iter()
                        .map(|a| wire::Principal {
                            id: Some(a.associated_entity.clone()),
                            resource_share_arn: Some(a.resource_share_arn.clone()),
                            creation_time: Some(a.creation_time),
                            last_updated_time: Some(a.last_updated_time),
                            external: Some(a.external),
                        })
                        .collect(),
                ),
                ..Default::default()
            }),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_list_replace_permission_associations_work(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_replace_permission_associations_work_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let work_ids: Vec<String> = input.work_ids.unwrap_or_default();
        let status = input.status.as_deref();
        let state = state.read().await;
        let works = state.list_replace_permission_associations_work(&work_ids, status);
        wire::serialize_list_replace_permission_associations_work_response(
            &wire::ListReplacePermissionAssociationsWorkResponse {
                replace_permission_associations_works: Some(
                    works
                        .iter()
                        .map(|w| wire::ReplacePermissionAssociationsWork {
                            id: Some(w.id.clone()),
                            from_permission_arn: Some(w.from_permission_arn.clone()),
                            from_permission_version: Some(w.from_permission_version.clone()),
                            to_permission_arn: Some(w.to_permission_arn.clone()),
                            to_permission_version: Some(w.to_permission_version.clone()),
                            status: Some(w.status.clone()),
                            creation_time: Some(w.creation_time),
                            last_updated_time: Some(w.last_updated_time),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            },
        )
    }

    async fn handle_list_resource_share_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_resource_share_permissions_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let state = state.read().await;
        match state.list_resource_share_permissions(resource_share_arn) {
            Ok(entries) => wire::serialize_list_resource_share_permissions_response(
                &wire::ListResourceSharePermissionsResponse {
                    permissions: Some(
                        entries
                            .iter()
                            .map(|e| wire::ResourceSharePermissionSummary {
                                arn: Some(e.permission_arn.clone()),
                                resource_type: Some(e.resource_type.clone()),
                                default_version: Some(e.default_version),
                                version: Some(e.permission_version.clone()),
                                last_updated_time: Some(e.last_updated_time),
                                status: Some(e.status.clone()),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_list_source_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_source_associations_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_share_arns: Vec<String> = input.resource_share_arns.unwrap_or_default();
        let state = state.read().await;
        let _assocs = state.list_source_associations(&resource_share_arns);
        wire::serialize_list_source_associations_response(&wire::ListSourceAssociationsResponse {
            source_associations: Some(Vec::new()),
            ..Default::default()
        })
    }

    async fn handle_promote_permission_created_from_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_promote_permission_created_from_policy_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "name is required");
        }
        let permission_arn = input.permission_arn.as_str();
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.promote_permission_created_from_policy(permission_arn, name, account_id) {
            Ok(perm) => wire::serialize_promote_permission_created_from_policy_response(
                &wire::PromotePermissionCreatedFromPolicyResponse {
                    permission: Some(wire::ResourceSharePermissionSummary {
                        arn: Some(perm.arn.clone()),
                        name: Some(perm.name.clone()),
                        resource_type: Some(perm.resource_type.clone()),
                        default_version: Some(perm.default_version),
                        is_resource_type_default: Some(false),
                        version: Some(perm.version.clone()),
                        status: Some(perm.status.clone()),
                        creation_time: Some(perm.creation_time),
                        last_updated_time: Some(perm.last_updated_time),
                        permission_type: Some(perm.permission_type.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_promote_resource_share_created_from_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_promote_resource_share_created_from_policy_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareArn is required",
            );
        }
        let resource_share_arn = input.resource_share_arn.as_str();
        let mut state = state.write().await;
        match state.promote_resource_share_created_from_policy(resource_share_arn) {
            Ok(rv) => wire::serialize_promote_resource_share_created_from_policy_response(
                &wire::PromoteResourceShareCreatedFromPolicyResponse {
                    return_value: Some(rv),
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_reject_resource_share_invitation(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reject_resource_share_invitation_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.resource_share_invitation_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "resourceShareInvitationArn is required",
            );
        }
        let invitation_arn = input.resource_share_invitation_arn.as_str();
        let mut state = state.write().await;
        match state.reject_resource_share_invitation(invitation_arn) {
            Ok(inv) => wire::serialize_reject_resource_share_invitation_response(
                &wire::RejectResourceShareInvitationResponse {
                    resource_share_invitation: Some(wire::ResourceShareInvitation {
                        resource_share_invitation_arn: Some(inv.invitation_arn.clone()),
                        resource_share_arn: Some(inv.resource_share_arn.clone()),
                        resource_share_name: Some(inv.resource_share_name.clone()),
                        sender_account_id: Some(inv.sender_account_id.clone()),
                        receiver_account_id: Some(inv.receiver_account_id.clone()),
                        status: Some(inv.status.clone()),
                        invitation_timestamp: Some(inv.invitation_timestamp),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_replace_permission_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_replace_permission_associations_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        if input.from_permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "fromPermissionArn is required",
            );
        }
        if input.to_permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "toPermissionArn is required",
            );
        }
        let from_permission_arn = input.from_permission_arn.as_str();
        let to_permission_arn = input.to_permission_arn.as_str();
        let from_permission_version = input.from_permission_version;
        let mut state = state.write().await;
        match state.replace_permission_associations(
            from_permission_arn,
            to_permission_arn,
            from_permission_version,
        ) {
            Ok(work) => wire::serialize_replace_permission_associations_response(
                &wire::ReplacePermissionAssociationsResponse {
                    replace_permission_associations_work: Some(
                        wire::ReplacePermissionAssociationsWork {
                            id: Some(work.id.clone()),
                            from_permission_arn: Some(work.from_permission_arn.clone()),
                            from_permission_version: Some(work.from_permission_version.clone()),
                            to_permission_arn: Some(work.to_permission_arn.clone()),
                            to_permission_version: Some(work.to_permission_version.clone()),
                            status: Some(work.status.clone()),
                            creation_time: Some(work.creation_time),
                            last_updated_time: Some(work.last_updated_time),
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_set_default_permission_version(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // Detect missing fields manually since the model gives default i32 = 0.
        let raw: serde_json::Value = if request.body.is_empty() {
            serde_json::Value::Null
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return rest_json_error(400, "InvalidParameterException", "Invalid JSON body");
                }
            }
        };
        let has_permission_version = raw
            .get("permissionVersion")
            .map(|v| !v.is_null())
            .unwrap_or(false);
        let input = match wire::deserialize_set_default_permission_version_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.permission_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionArn is required",
            );
        }
        if !has_permission_version {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "permissionVersion is required",
            );
        }
        let permission_arn = input.permission_arn.as_str();
        let permission_version = input.permission_version;
        let mut state = state.write().await;
        match state.set_default_permission_version(permission_arn, permission_version) {
            Ok(rv) => wire::serialize_set_default_permission_version_response(
                &wire::SetDefaultPermissionVersionResponse {
                    return_value: Some(rv),
                    ..Default::default()
                },
            ),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_share_arn = match input.resource_share_arn.as_deref() {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "InvalidParameterException",
                    "resourceShareArn is required",
                );
            }
        };
        let tags: Vec<Tag> = input
            .tags
            .into_iter()
            .filter_map(|t| {
                let key = t.key?;
                let value = t.value?;
                Some(Tag { key, value })
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(resource_share_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => ram_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::RamState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let resource_share_arn = match input.resource_share_arn.as_deref() {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "InvalidParameterException",
                    "resourceShareArn is required",
                );
            }
        };
        let tag_keys: Vec<String> = input.tag_keys;

        let mut state = state.write().await;
        match state.untag_resource(resource_share_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => ram_error_response(&e),
        }
    }
}

fn ram_error_response(err: &RamError) -> MockResponse {
    let (status, error_type) = match err {
        RamError::MalformedArnInvalid(_) => (400u16, "MalformedArnException"),
        RamError::MalformedArnUnshareable => (400u16, "MalformedArnException"),
        RamError::InvalidPrincipal(_) => (400u16, "InvalidParameterException"),
        RamError::InvalidResourceOwner(_) => (400u16, "InvalidParameterException"),
        RamError::ResourceShareNotFound(_) => (400u16, "UnknownResourceException"),
        RamError::InvalidAssociationType(_) => (400u16, "InvalidParameterException"),
        RamError::InvalidAssociationStatus(_) => (400u16, "InvalidParameterException"),
        RamError::ResourceArnWithPrincipalType => (400u16, "InvalidParameterException"),
        RamError::PrincipalWithResourceType => (400u16, "InvalidParameterException"),
        RamError::InvalidPermissionScope(_) => (400u16, "InvalidParameterException"),
        RamError::InvalidResourceType(_) => (400u16, "InvalidParameterException"),
        RamError::InvalidResourceRegionScope(_) => (400u16, "InvalidParameterException"),
        RamError::PermissionNotFound(_) => (400u16, "UnknownResourceException"),
        RamError::InvitationNotFound(_) => (400u16, "ResourceShareInvitationArnNotFoundException"),
        RamError::InvitationAlreadyResponded(_) => {
            (400u16, "ResourceShareInvitationAlreadyAcceptedException")
        }
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
