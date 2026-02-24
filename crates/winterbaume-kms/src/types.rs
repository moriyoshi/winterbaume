use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A KMS Custom Key Store.
#[derive(Debug, Clone)]
pub struct CustomKeyStore {
    pub custom_key_store_id: String,
    pub custom_key_store_name: String,
    pub cloud_hsm_cluster_id: Option<String>,
    pub trust_anchor_certificate: Option<String>,
    pub connection_state: String,
    pub creation_date: DateTime<Utc>,
    pub custom_key_store_type: String,
}

/// A KMS key.
#[derive(Debug, Clone)]
pub struct Key {
    pub key_id: String,
    pub arn: String,
    pub account_id: String,
    pub region: String,
    pub description: String,
    pub key_spec: String,
    pub key_usage: String,
    pub key_state: KeyState,
    pub creation_date: DateTime<Utc>,
    pub enabled: bool,
    pub origin: String,
    pub key_manager: String,
    pub deletion_date: Option<DateTime<Utc>>,
    pub key_rotation_enabled: bool,
    /// 32-byte symmetric key material for SYMMETRIC_DEFAULT keys.
    pub key_material: Vec<u8>,
    /// DER-encoded SubjectPublicKeyInfo (SPKI) for asymmetric keys.
    pub public_key_der: Option<Vec<u8>>,
    pub tags: HashMap<String, String>,
    pub multi_region: bool,
}

impl Key {
    /// Returns the encryption algorithms supported by this key, if applicable.
    pub fn encryption_algorithms(&self) -> Option<Vec<&str>> {
        if self.key_usage != "ENCRYPT_DECRYPT" {
            return None;
        }
        match self.key_spec.as_str() {
            "SYMMETRIC_DEFAULT" => Some(vec!["SYMMETRIC_DEFAULT"]),
            "RSA_2048" | "RSA_3072" | "RSA_4096" => {
                Some(vec!["RSAES_OAEP_SHA_1", "RSAES_OAEP_SHA_256"])
            }
            _ => None,
        }
    }

    /// Returns the signing algorithms supported by this key, if applicable.
    pub fn signing_algorithms(&self) -> Option<Vec<&str>> {
        if self.key_usage != "SIGN_VERIFY" {
            return None;
        }
        match self.key_spec.as_str() {
            "RSA_2048" | "RSA_3072" | "RSA_4096" => Some(vec![
                "RSASSA_PKCS1_V1_5_SHA_256",
                "RSASSA_PKCS1_V1_5_SHA_384",
                "RSASSA_PKCS1_V1_5_SHA_512",
                "RSASSA_PSS_SHA_256",
                "RSASSA_PSS_SHA_384",
                "RSASSA_PSS_SHA_512",
            ]),
            "ECC_NIST_P256" | "ECC_SECG_P256K1" => Some(vec!["ECDSA_SHA_256"]),
            "ECC_NIST_P384" => Some(vec!["ECDSA_SHA_384"]),
            "ECC_NIST_P521" => Some(vec!["ECDSA_SHA_512"]),
            _ => None,
        }
    }
}

/// Key state enum.
#[derive(Debug, Clone, PartialEq)]
pub enum KeyState {
    Enabled,
    Disabled,
    PendingDeletion,
}

impl KeyState {
    pub fn as_str(&self) -> &str {
        match self {
            KeyState::Enabled => "Enabled",
            KeyState::Disabled => "Disabled",
            KeyState::PendingDeletion => "PendingDeletion",
        }
    }
}

/// A KMS alias.
#[derive(Debug, Clone)]
pub struct Alias {
    pub alias_name: String,
    pub alias_arn: String,
    pub target_key_id: String,
}

/// A KMS grant.
#[derive(Debug, Clone)]
pub struct Grant {
    pub grant_id: String,
    pub grant_token: String,
    pub key_id: String,
    pub grantee_principal: String,
    pub retiring_principal: Option<String>,
    pub operations: Vec<String>,
    pub constraints: Option<GrantConstraints>,
    pub issuing_account: String,
    pub name: Option<String>,
    pub creation_date: DateTime<Utc>,
}

/// Grant constraints.
#[derive(Debug, Clone)]
pub struct GrantConstraints {
    pub encryption_context_subset: Option<HashMap<String, String>>,
    pub encryption_context_equals: Option<HashMap<String, String>>,
}

/// Key rotation record.
#[derive(Debug, Clone)]
pub struct KeyRotation {
    pub key_id: String,
    pub rotation_date: DateTime<Utc>,
    pub rotation_type: String,
}
