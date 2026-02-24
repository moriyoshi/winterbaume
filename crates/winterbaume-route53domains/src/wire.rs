//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-route53domains

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_accept_domain_transfer_from_another_aws_account_response(
    result: &AcceptDomainTransferFromAnotherAwsAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_delegation_signer_to_domain_response(
    result: &AssociateDelegationSignerToDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_domain_transfer_to_another_aws_account_response(
    result: &CancelDomainTransferToAnotherAwsAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_check_domain_availability_response(
    result: &CheckDomainAvailabilityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_check_domain_transferability_response(
    result: &CheckDomainTransferabilityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_domain_response(result: &DeleteDomainResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_tags_for_domain_response(
    result: &DeleteTagsForDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_domain_auto_renew_response(
    result: &DisableDomainAutoRenewResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_domain_transfer_lock_response(
    result: &DisableDomainTransferLockResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_delegation_signer_from_domain_response(
    result: &DisassociateDelegationSignerFromDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_domain_auto_renew_response(
    result: &EnableDomainAutoRenewResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_domain_transfer_lock_response(
    result: &EnableDomainTransferLockResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_contact_reachability_status_response(
    result: &GetContactReachabilityStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_domain_detail_response(result: &GetDomainDetailResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_domain_suggestions_response(
    result: &GetDomainSuggestionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_operation_detail_response(
    result: &GetOperationDetailResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_domains_response(result: &ListDomainsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_operations_response(result: &ListOperationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_prices_response(result: &ListPricesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_domain_response(result: &ListTagsForDomainResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_push_domain_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_domain_response(result: &RegisterDomainResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reject_domain_transfer_from_another_aws_account_response(
    result: &RejectDomainTransferFromAnotherAwsAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_renew_domain_response(result: &RenewDomainResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_resend_contact_reachability_email_response(
    result: &ResendContactReachabilityEmailResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_resend_operation_authorization_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_retrieve_domain_auth_code_response(
    result: &RetrieveDomainAuthCodeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_transfer_domain_response(result: &TransferDomainResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_transfer_domain_to_another_aws_account_response(
    result: &TransferDomainToAnotherAwsAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_domain_contact_response(
    result: &UpdateDomainContactResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_domain_contact_privacy_response(
    result: &UpdateDomainContactPrivacyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_domain_nameservers_response(
    result: &UpdateDomainNameserversResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_tags_for_domain_response(
    result: &UpdateTagsForDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_view_billing_response(result: &ViewBillingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_accept_domain_transfer_from_another_aws_account_request(
    body: &[u8],
) -> Result<AcceptDomainTransferFromAnotherAwsAccountRequest, String> {
    if body.is_empty() {
        return Ok(AcceptDomainTransferFromAnotherAwsAccountRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AcceptDomainTransferFromAnotherAwsAccount request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_delegation_signer_to_domain_request(
    body: &[u8],
) -> Result<AssociateDelegationSignerToDomainRequest, String> {
    if body.is_empty() {
        return Ok(AssociateDelegationSignerToDomainRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AssociateDelegationSignerToDomain request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_domain_transfer_to_another_aws_account_request(
    body: &[u8],
) -> Result<CancelDomainTransferToAnotherAwsAccountRequest, String> {
    if body.is_empty() {
        return Ok(CancelDomainTransferToAnotherAwsAccountRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CancelDomainTransferToAnotherAwsAccount request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_check_domain_availability_request(
    body: &[u8],
) -> Result<CheckDomainAvailabilityRequest, String> {
    if body.is_empty() {
        return Ok(CheckDomainAvailabilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CheckDomainAvailability request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_check_domain_transferability_request(
    body: &[u8],
) -> Result<CheckDomainTransferabilityRequest, String> {
    if body.is_empty() {
        return Ok(CheckDomainTransferabilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CheckDomainTransferability request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_domain_request(body: &[u8]) -> Result<DeleteDomainRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_tags_for_domain_request(
    body: &[u8],
) -> Result<DeleteTagsForDomainRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTagsForDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTagsForDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_domain_auto_renew_request(
    body: &[u8],
) -> Result<DisableDomainAutoRenewRequest, String> {
    if body.is_empty() {
        return Ok(DisableDomainAutoRenewRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableDomainAutoRenew request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_domain_transfer_lock_request(
    body: &[u8],
) -> Result<DisableDomainTransferLockRequest, String> {
    if body.is_empty() {
        return Ok(DisableDomainTransferLockRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableDomainTransferLock request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_delegation_signer_from_domain_request(
    body: &[u8],
) -> Result<DisassociateDelegationSignerFromDomainRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateDelegationSignerFromDomainRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociateDelegationSignerFromDomain request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_domain_auto_renew_request(
    body: &[u8],
) -> Result<EnableDomainAutoRenewRequest, String> {
    if body.is_empty() {
        return Ok(EnableDomainAutoRenewRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableDomainAutoRenew request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_domain_transfer_lock_request(
    body: &[u8],
) -> Result<EnableDomainTransferLockRequest, String> {
    if body.is_empty() {
        return Ok(EnableDomainTransferLockRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableDomainTransferLock request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_contact_reachability_status_request(
    body: &[u8],
) -> Result<GetContactReachabilityStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetContactReachabilityStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetContactReachabilityStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_domain_detail_request(
    body: &[u8],
) -> Result<GetDomainDetailRequest, String> {
    if body.is_empty() {
        return Ok(GetDomainDetailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDomainDetail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_domain_suggestions_request(
    body: &[u8],
) -> Result<GetDomainSuggestionsRequest, String> {
    if body.is_empty() {
        return Ok(GetDomainSuggestionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDomainSuggestions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_operation_detail_request(
    body: &[u8],
) -> Result<GetOperationDetailRequest, String> {
    if body.is_empty() {
        return Ok(GetOperationDetailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOperationDetail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_domains_request(body: &[u8]) -> Result<ListDomainsRequest, String> {
    if body.is_empty() {
        return Ok(ListDomainsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDomains request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_operations_request(body: &[u8]) -> Result<ListOperationsRequest, String> {
    if body.is_empty() {
        return Ok(ListOperationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOperations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_prices_request(body: &[u8]) -> Result<ListPricesRequest, String> {
    if body.is_empty() {
        return Ok(ListPricesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPrices request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_domain_request(
    body: &[u8],
) -> Result<ListTagsForDomainRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_push_domain_request(body: &[u8]) -> Result<PushDomainRequest, String> {
    if body.is_empty() {
        return Ok(PushDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PushDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_domain_request(body: &[u8]) -> Result<RegisterDomainRequest, String> {
    if body.is_empty() {
        return Ok(RegisterDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reject_domain_transfer_from_another_aws_account_request(
    body: &[u8],
) -> Result<RejectDomainTransferFromAnotherAwsAccountRequest, String> {
    if body.is_empty() {
        return Ok(RejectDomainTransferFromAnotherAwsAccountRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize RejectDomainTransferFromAnotherAwsAccount request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_renew_domain_request(body: &[u8]) -> Result<RenewDomainRequest, String> {
    if body.is_empty() {
        return Ok(RenewDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RenewDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resend_contact_reachability_email_request(
    body: &[u8],
) -> Result<ResendContactReachabilityEmailRequest, String> {
    if body.is_empty() {
        return Ok(ResendContactReachabilityEmailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResendContactReachabilityEmail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resend_operation_authorization_request(
    body: &[u8],
) -> Result<ResendOperationAuthorizationRequest, String> {
    if body.is_empty() {
        return Ok(ResendOperationAuthorizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResendOperationAuthorization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_retrieve_domain_auth_code_request(
    body: &[u8],
) -> Result<RetrieveDomainAuthCodeRequest, String> {
    if body.is_empty() {
        return Ok(RetrieveDomainAuthCodeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RetrieveDomainAuthCode request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_transfer_domain_request(body: &[u8]) -> Result<TransferDomainRequest, String> {
    if body.is_empty() {
        return Ok(TransferDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TransferDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_transfer_domain_to_another_aws_account_request(
    body: &[u8],
) -> Result<TransferDomainToAnotherAwsAccountRequest, String> {
    if body.is_empty() {
        return Ok(TransferDomainToAnotherAwsAccountRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize TransferDomainToAnotherAwsAccount request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_domain_contact_request(
    body: &[u8],
) -> Result<UpdateDomainContactRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDomainContactRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDomainContact request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_domain_contact_privacy_request(
    body: &[u8],
) -> Result<UpdateDomainContactPrivacyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDomainContactPrivacyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDomainContactPrivacy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_domain_nameservers_request(
    body: &[u8],
) -> Result<UpdateDomainNameserversRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDomainNameserversRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDomainNameservers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_tags_for_domain_request(
    body: &[u8],
) -> Result<UpdateTagsForDomainRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTagsForDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTagsForDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_view_billing_request(body: &[u8]) -> Result<ViewBillingRequest, String> {
    if body.is_empty() {
        return Ok(ViewBillingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ViewBilling request: {e}"))
}
