//! Routing for service-backed resource types: types whose state is owned by
//! another service and read back from it on every call.

use winterbaume_core::StatefulService;

use crate::cfn_schema::ShapeContext;
use crate::cloudfront_response_headers_policy::CLOUDFRONT_REGION;
use crate::handlers::CloudControlService;
use crate::state::CloudControlError;
use crate::types::{ManagedResource, ResourceRequest};
use crate::{cloudfront_function, cloudfront_response_headers_policy, ssm_parameter};

#[derive(Clone, Copy)]
pub(crate) enum BackedType {
    SsmParameter,
    CloudFrontResponseHeadersPolicy,
    CloudFrontFunction,
}

impl BackedType {
    pub(crate) fn lookup(type_name: &str) -> Option<Self> {
        match type_name {
            ssm_parameter::TYPE_NAME => Some(Self::SsmParameter),
            cloudfront_response_headers_policy::TYPE_NAME => {
                Some(Self::CloudFrontResponseHeadersPolicy)
            }
            cloudfront_function::TYPE_NAME => Some(Self::CloudFrontFunction),
            _ => None,
        }
    }
}

impl CloudControlService {
    pub(crate) async fn backed_create(
        &self,
        backed: BackedType,
        desired: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        match backed {
            BackedType::SsmParameter => self.create_ssm_parameter(desired, ctx).await,
            BackedType::CloudFrontResponseHeadersPolicy => {
                self.create_response_headers_policy(desired, ctx).await
            }
            BackedType::CloudFrontFunction => self.create_cloudfront_function(desired, ctx).await,
        }
    }

    pub(crate) async fn backed_get(
        &self,
        backed: BackedType,
        identifier: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ManagedResource, CloudControlError> {
        match backed {
            BackedType::SsmParameter => self.get_ssm_parameter(identifier, ctx).await,
            BackedType::CloudFrontResponseHeadersPolicy => {
                self.get_response_headers_policy(identifier, ctx).await
            }
            BackedType::CloudFrontFunction => self.get_cloudfront_function(identifier, ctx).await,
        }
    }

    pub(crate) async fn backed_list(
        &self,
        backed: BackedType,
        ctx: &ShapeContext<'_>,
    ) -> Vec<ManagedResource> {
        match backed {
            BackedType::SsmParameter => self.list_ssm_parameters(ctx).await,
            BackedType::CloudFrontResponseHeadersPolicy => {
                self.list_response_headers_policies(ctx).await
            }
            BackedType::CloudFrontFunction => self.list_cloudfront_functions(ctx).await,
        }
    }

    pub(crate) async fn backed_update(
        &self,
        backed: BackedType,
        identifier: &str,
        patch: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        match backed {
            BackedType::SsmParameter => self.update_ssm_parameter(identifier, patch, ctx).await,
            BackedType::CloudFrontResponseHeadersPolicy => {
                self.update_response_headers_policy(identifier, patch, ctx)
                    .await
            }
            BackedType::CloudFrontFunction => {
                self.update_cloudfront_function(identifier, patch, ctx)
                    .await
            }
        }
    }

    pub(crate) async fn backed_delete(
        &self,
        backed: BackedType,
        identifier: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        match backed {
            BackedType::SsmParameter => self.delete_ssm_parameter(identifier, ctx).await,
            BackedType::CloudFrontResponseHeadersPolicy => {
                self.delete_response_headers_policy(identifier, ctx).await
            }
            BackedType::CloudFrontFunction => {
                self.delete_cloudfront_function(identifier, ctx).await
            }
        }
    }

    /// Notify the owning service's subscribers after a successful mutation
    /// that arrived through Cloud Control.
    pub(crate) async fn notify_owning_service(
        &self,
        backed: BackedType,
        account_id: &str,
        region: &str,
    ) {
        match backed {
            BackedType::SsmParameter => self.ssm.notify_state_changed(account_id, region).await,
            BackedType::CloudFrontResponseHeadersPolicy | BackedType::CloudFrontFunction => {
                self.cloudfront
                    .notify_state_changed(account_id, CLOUDFRONT_REGION)
                    .await
            }
        }
    }
}
