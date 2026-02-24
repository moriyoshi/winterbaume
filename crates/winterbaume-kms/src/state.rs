use std::collections::HashMap;

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use chrono::{Duration, Utc};
use rand::Rng;

use crate::types::*;

/// Returns the bits-per-byte length of an RSA modulus for the given key spec.
fn rsa_modulus_byte_len(key_spec: &str) -> Option<usize> {
    match key_spec {
        "RSA_2048" => Some(256),
        "RSA_3072" => Some(384),
        "RSA_4096" => Some(512),
        _ => None,
    }
}

/// Encodes a DER length in the canonical BER/DER form.
fn der_length(len: usize) -> Vec<u8> {
    if len < 128 {
        vec![len as u8]
    } else if len < 256 {
        vec![0x81, len as u8]
    } else {
        vec![0x82, (len >> 8) as u8, (len & 0xff) as u8]
    }
}

/// Wraps `contents` in a DER TLV with the given `tag`.
fn der_tlv(tag: u8, contents: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    out.extend_from_slice(&der_length(contents.len()));
    out.extend_from_slice(contents);
    out
}

/// Encodes a non-negative integer as a DER INTEGER, prepending a 0x00 byte if
/// the high bit of the first byte is set (to keep the sign non-negative).
fn der_integer(bytes: &[u8]) -> Vec<u8> {
    // Strip leading zero bytes but keep at least one byte.
    let trimmed = bytes
        .iter()
        .position(|&b| b != 0)
        .map(|i| &bytes[i..])
        .unwrap_or(&bytes[..1]);
    let needs_pad = trimmed[0] & 0x80 != 0;
    let mut content = Vec::with_capacity(trimmed.len() + if needs_pad { 1 } else { 0 });
    if needs_pad {
        content.push(0x00);
    }
    content.extend_from_slice(trimmed);
    der_tlv(0x02, &content)
}

/// Builds a SubjectPublicKeyInfo DER for an RSA public key.
///
/// The modulus is `modulus_bytes` (big-endian, `key_bits / 8` bytes).
/// The public exponent is fixed at 65537 (0x010001).
fn rsa_spki_der(modulus_bytes: &[u8]) -> Vec<u8> {
    // OID for rsaEncryption: 1.2.840.113549.1.1.1
    let rsa_oid_bytes: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];
    let oid_tlv = der_tlv(0x06, rsa_oid_bytes);
    let null_tlv: &[u8] = &[0x05, 0x00];

    // AlgorithmIdentifier SEQUENCE { OID, NULL }
    let mut algo_id_content = oid_tlv;
    algo_id_content.extend_from_slice(null_tlv);
    let algo_id = der_tlv(0x30, &algo_id_content);

    // RSAPublicKey SEQUENCE { modulus INTEGER, publicExponent INTEGER }
    let modulus_int = der_integer(modulus_bytes);
    let exponent_int = der_integer(&[0x01, 0x00, 0x01]); // 65537
    let mut rsa_pub_key_content = modulus_int;
    rsa_pub_key_content.extend_from_slice(&exponent_int);
    let rsa_pub_key = der_tlv(0x30, &rsa_pub_key_content);

    // BIT STRING: leading 0x00 byte (no unused bits) + RSAPublicKey DER
    let mut bit_string_content = vec![0x00u8];
    bit_string_content.extend_from_slice(&rsa_pub_key);
    let bit_string = der_tlv(0x03, &bit_string_content);

    // SubjectPublicKeyInfo SEQUENCE { AlgorithmIdentifier, BIT STRING }
    let mut spki_content = algo_id;
    spki_content.extend_from_slice(&bit_string);
    der_tlv(0x30, &spki_content)
}

/// Generates a DER-encoded SubjectPublicKeyInfo for an ECC key pair and
/// returns `(pkcs8_private_key_der, spki_der)`.  Returns `None` when
/// `key_spec` is not a recognised ECC spec.
fn generate_ecc_key_pair_spki(key_spec: &str) -> Option<Vec<u8>> {
    use rcgen::KeyPair;
    let alg: &'static rcgen::SignatureAlgorithm = match key_spec {
        "ECC_NIST_P256" | "ECC_SECG_P256K1" => &rcgen::PKCS_ECDSA_P256_SHA256,
        "ECC_NIST_P384" => &rcgen::PKCS_ECDSA_P384_SHA384,
        // P-521 is not supported by the ring backend; fall back to P-384.
        "ECC_NIST_P521" => &rcgen::PKCS_ECDSA_P384_SHA384,
        _ => return None,
    };
    let kp = KeyPair::generate_for(alg).ok()?;
    Some(kp.public_key_der())
}

/// Generates a DER-encoded SubjectPublicKeyInfo (SPKI) for the given
/// asymmetric `key_spec`.  Returns `None` for symmetric or unsupported specs.
fn generate_asymmetric_spki(key_spec: &str) -> Option<Vec<u8>> {
    if let Some(modulus_len) = rsa_modulus_byte_len(key_spec) {
        // Generate a random modulus of the correct byte length.
        // This is a mock SPKI — not a real cryptographic RSA key pair — but it
        // produces syntactically valid DER that callers can parse.
        let mut modulus = vec![0u8; modulus_len];
        rand::rng().fill(&mut modulus[..]);
        // Ensure the high bit is set so the modulus has the correct bit-length.
        modulus[0] |= 0x80;
        return Some(rsa_spki_der(&modulus));
    }
    generate_ecc_key_pair_spki(key_spec)
}

/// In-memory state for the KMS service.
#[derive(Debug, Default)]
pub struct KmsState {
    /// Keys keyed by key ID.
    pub keys: HashMap<String, Key>,
    /// Aliases keyed by alias name.
    pub aliases: HashMap<String, Alias>,
    /// Grants keyed by grant ID.
    pub grants: HashMap<String, Grant>,
    /// Key policies keyed by key ID.
    pub key_policies: HashMap<String, String>,
    /// Key rotation history keyed by key ID.
    pub key_rotations: HashMap<String, Vec<KeyRotation>>,
    /// Custom key stores keyed by custom key store ID.
    pub custom_key_stores: HashMap<String, CustomKeyStore>,
    /// Track which keys have imported key material (key_id -> true).
    pub imported_key_material: HashMap<String, bool>,
}

/// Error type for KMS operations.
#[derive(Debug, thiserror::Error)]
pub enum KmsError {
    #[error(
        "1 validation error detected: Value '{0}' at 'KeySpec' failed to satisfy constraint: \
        Member must satisfy enum value set: \
        ['ECC_NIST_P256', 'ECC_NIST_P384', 'ECC_NIST_P521', 'ECC_SECG_P256K1', 'HMAC_224', 'HMAC_256', \
        'HMAC_384', 'HMAC_512', 'RSA_2048', 'RSA_3072', 'RSA_4096', 'SM2', 'SYMMETRIC_DEFAULT']"
    )]
    InvalidKeySpec(String),

    #[error("{0} is pending deletion.")]
    KeyPendingDeletion(String),

    #[error("{0} is not pending deletion.")]
    KeyNotPendingDeletion(String),

    #[error(
        "1 validation error detected: Value '{0}' at 'pendingWindowInDays' failed to satisfy constraint: Member must have value between 7 and 30"
    )]
    InvalidPendingWindowDays(i64),

    #[error(
        "1 validation error detected: Value at 'plaintext' failed to satisfy constraint: Member must have length between 1 and 4096"
    )]
    PlaintextLengthInvalid,

    #[error("1 validation error detected: Value '{0}' at 'keySpec' failed to satisfy constraint")]
    InvalidDataKeySpec(String),

    #[error("An alias with the name {0} already exists")]
    AliasAlreadyExists(String),

    #[error("Alias {0} is not found.")]
    AliasNotFound(String),

    #[error("{0} is not a valid key for automatic rotation.")]
    KeyNotValidForRotation(String),

    #[error("{0} key usage is not supported for GetPublicKey.")]
    GetPublicKeyUnsupported(String),

    #[error(
        "1 validation error detected: Value at 'numberOfBytes' failed to satisfy constraint: Member must have value between 1 and 1024"
    )]
    InvalidNumberOfBytes,

    #[error("{0} key usage is {1} which is not compatible with GenerateMac.")]
    KeyUsageIncompatibleWithGenerateMac(String, String),

    #[error("{0} key spec is {1} which is not compatible with GenerateMac.")]
    KeySpecIncompatibleWithGenerateMac(String, String),

    #[error("MAC generation failed")]
    MacGenerationFailed,

    #[error("Unsupported MAC algorithm: {0}")]
    UnsupportedMacAlgorithm(String),

    #[error("MAC verification failed")]
    MacVerificationFailed,

    #[error("{0} key usage is {1} which is not compatible with Sign.")]
    KeyUsageIncompatibleWithSign(String, String),

    #[error("Grant '{0}' not found for key '{1}'")]
    GrantNotFoundForKey(String, String),

    #[error("Grant not found")]
    GrantNotFound,

    #[error("GrantId or GrantToken is required")]
    GrantIdOrTokenRequired,

    #[error("Grant '{0}' not found")]
    GrantIdNotFound(String),

    #[error("Grant ID {0} not found")]
    GrantIdNotFoundForKey(String),

    #[error("{0} key spec is not valid for on-demand rotation.")]
    KeyNotValidForOnDemandRotation(String),

    #[error("{0} is not a multi-Region key.")]
    KeyNotMultiRegion(String),

    #[error("Custom key store with name '{0}' already exists.")]
    CustomKeyStoreNameAlreadyExists(String),

    #[error("Custom key store '{0}' not found.")]
    CustomKeyStoreNotFound(String),

    #[error("Key '{0}' does not exist")]
    KeyNotFound(String),

    #[error(
        "The ciphertext refers to a customer master key that does not exist, does not exist in this region, or you are not allowed to access."
    )]
    InvalidCiphertext,

    #[error("Encryption failed")]
    EncryptionFailed,

    #[error("{0} is disabled.")]
    KeyDisabled(String),
}

/// Result of GenerateDataKey.
pub struct DataKeyResult {
    pub plaintext: Vec<u8>,
    pub ciphertext_blob: Vec<u8>,
    pub key_id: String,
    pub key_arn: String,
}

/// Result of Encrypt.
pub struct EncryptResult {
    pub ciphertext_blob: Vec<u8>,
    pub key_id: String,
    pub key_arn: String,
}

/// Result of Decrypt.
pub struct DecryptResult {
    pub plaintext: Vec<u8>,
    pub key_id: String,
    pub key_arn: String,
}

/// Ciphertext blob format:
/// [36 bytes key_id (UTF-8, zero-padded)] [12 bytes IV] [16 bytes GCM tag] [variable ciphertext]
const KEY_ID_LEN: usize = 36;
const IV_LEN: usize = 12;
const TAG_LEN: usize = 16;
const HEADER_LEN: usize = KEY_ID_LEN + IV_LEN + TAG_LEN;

impl KmsState {
    pub fn create_key(
        &mut self,
        account_id: &str,
        region: &str,
        description: &str,
        key_spec: Option<&str>,
        key_usage: Option<&str>,
        tags: HashMap<String, String>,
        origin: Option<&str>,
        multi_region: bool,
    ) -> Result<&Key, KmsError> {
        let key_spec = key_spec.unwrap_or("SYMMETRIC_DEFAULT");
        let key_usage = key_usage.unwrap_or("ENCRYPT_DECRYPT");

        // Validate key spec
        let valid_specs = [
            "SYMMETRIC_DEFAULT",
            "RSA_2048",
            "RSA_3072",
            "RSA_4096",
            "ECC_NIST_P256",
            "ECC_NIST_P384",
            "ECC_NIST_P521",
            "ECC_SECG_P256K1",
            "HMAC_224",
            "HMAC_256",
            "HMAC_384",
            "HMAC_512",
            "SM2",
        ];
        if !valid_specs.contains(&key_spec) {
            return Err(KmsError::InvalidKeySpec(key_spec.to_string()));
        }

        let key_id = if multi_region {
            format!("mrk-{}", uuid::Uuid::new_v4().simple())
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        let arn = format!("arn:aws:kms:{region}:{account_id}:key/{key_id}");
        let origin = origin.unwrap_or("AWS_KMS");

        // Generate 32-byte symmetric key material (also used as HMAC key material).
        let mut key_material = vec![0u8; 32];
        rand::rng().fill(&mut key_material[..]);

        // For asymmetric key specs, generate a real SPKI so that GetPublicKey
        // returns a properly-structured DER byte string.
        let public_key_der = generate_asymmetric_spki(key_spec);

        let key = Key {
            key_id: key_id.clone(),
            arn,
            account_id: account_id.to_string(),
            region: region.to_string(),
            description: description.to_string(),
            key_spec: key_spec.to_string(),
            key_usage: key_usage.to_string(),
            key_state: KeyState::Enabled,
            creation_date: Utc::now(),
            enabled: true,
            origin: origin.to_string(),
            key_manager: "CUSTOMER".to_string(),
            deletion_date: None,
            key_rotation_enabled: false,
            key_material,
            public_key_der,
            tags,
            multi_region,
        };

        self.keys.insert(key_id.clone(), key);
        Ok(self.keys.get(&key_id).unwrap())
    }

    pub fn describe_key(&self, key_id_or_alias: &str) -> Result<&Key, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        self.keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))
    }

    pub fn list_keys(&self) -> Vec<&Key> {
        self.keys.values().collect()
    }

    pub fn enable_key(&mut self, key_id_or_alias: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get_mut(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;

        if key.key_state == KeyState::PendingDeletion {
            return Err(KmsError::KeyPendingDeletion(key.arn.clone()));
        }

        key.enabled = true;
        key.key_state = KeyState::Enabled;
        Ok(())
    }

    pub fn disable_key(&mut self, key_id_or_alias: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get_mut(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;

        if key.key_state == KeyState::PendingDeletion {
            return Err(KmsError::KeyPendingDeletion(key.arn.clone()));
        }

        key.enabled = false;
        key.key_state = KeyState::Disabled;
        Ok(())
    }

    pub fn schedule_key_deletion(
        &mut self,
        key_id_or_alias: &str,
        pending_window_in_days: Option<i64>,
    ) -> Result<(&Key, i64), KmsError> {
        let days = pending_window_in_days.unwrap_or(30);
        if !(7..=30).contains(&days) {
            return Err(KmsError::InvalidPendingWindowDays(days));
        }

        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get_mut(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;

        let deletion_date = Utc::now() + Duration::days(days);
        key.key_state = KeyState::PendingDeletion;
        key.enabled = false;
        key.deletion_date = Some(deletion_date);

        let deletion_timestamp = deletion_date.timestamp();
        Ok((self.keys.get(&key_id).unwrap(), deletion_timestamp))
    }

    pub fn cancel_key_deletion(&mut self, key_id_or_alias: &str) -> Result<&Key, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get_mut(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;

        if key.key_state != KeyState::PendingDeletion {
            return Err(KmsError::KeyNotPendingDeletion(key.arn.clone()));
        }

        key.key_state = KeyState::Disabled;
        key.enabled = false;
        key.deletion_date = None;

        Ok(self.keys.get(&key_id).unwrap())
    }

    pub fn encrypt(
        &self,
        key_id_or_alias: &str,
        plaintext: &[u8],
        encryption_context: &HashMap<String, String>,
    ) -> Result<EncryptResult, KmsError> {
        if plaintext.is_empty() || plaintext.len() > 4096 {
            return Err(KmsError::PlaintextLengthInvalid);
        }

        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;

        self.ensure_key_usable(key)?;

        let ciphertext_blob = encrypt_with_key(key, plaintext, encryption_context)?;

        Ok(EncryptResult {
            ciphertext_blob,
            key_id: key.key_id.clone(),
            key_arn: key.arn.clone(),
        })
    }

    pub fn decrypt(
        &self,
        ciphertext_blob: &[u8],
        encryption_context: &HashMap<String, String>,
    ) -> Result<DecryptResult, KmsError> {
        if ciphertext_blob.len() < HEADER_LEN {
            return Err(invalid_ciphertext_error());
        }

        // Extract key ID from ciphertext header
        let key_id_bytes = &ciphertext_blob[..KEY_ID_LEN];
        let key_id = std::str::from_utf8(key_id_bytes)
            .map_err(|_| invalid_ciphertext_error())?
            .trim_end_matches('\0')
            .to_string();

        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(invalid_ciphertext_error)?;

        self.ensure_key_usable(key)?;

        let plaintext = decrypt_with_key(key, ciphertext_blob, encryption_context)?;

        Ok(DecryptResult {
            plaintext,
            key_id: key.key_id.clone(),
            key_arn: key.arn.clone(),
        })
    }

    pub fn generate_data_key(
        &self,
        key_id_or_alias: &str,
        key_spec: Option<&str>,
        number_of_bytes: Option<usize>,
        encryption_context: &HashMap<String, String>,
    ) -> Result<DataKeyResult, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;

        self.ensure_key_usable(key)?;

        // Determine plaintext length
        let plaintext_len = if let Some(n) = number_of_bytes {
            n
        } else {
            match key_spec.unwrap_or("AES_256") {
                "AES_128" => 16,
                "AES_256" => 32,
                other => {
                    return Err(KmsError::InvalidDataKeySpec(other.to_string()));
                }
            }
        };

        // Generate random plaintext
        let mut plaintext = vec![0u8; plaintext_len];
        rand::rng().fill(&mut plaintext[..]);

        // Encrypt plaintext under the KMS key
        let ciphertext_blob = encrypt_with_key(key, &plaintext, encryption_context)?;

        Ok(DataKeyResult {
            plaintext,
            ciphertext_blob,
            key_id: key.key_id.clone(),
            key_arn: key.arn.clone(),
        })
    }

    pub fn create_alias(
        &mut self,
        alias_name: &str,
        target_key_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(), KmsError> {
        if self.aliases.contains_key(alias_name) {
            return Err(KmsError::AliasAlreadyExists(format!(
                "arn:aws:kms:{region}:{account_id}:{alias_name}"
            )));
        }

        let key_id = self.resolve_key_id(target_key_id)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }

        let alias_arn = format!("arn:aws:kms:{region}:{account_id}:{alias_name}");
        self.aliases.insert(
            alias_name.to_string(),
            Alias {
                alias_name: alias_name.to_string(),
                alias_arn,
                target_key_id: key_id,
            },
        );

        Ok(())
    }

    pub fn list_aliases(&self, key_id: Option<&str>) -> Vec<&Alias> {
        match key_id {
            Some(kid) => {
                let resolved = self.resolve_key_id(kid).unwrap_or_default();
                self.aliases
                    .values()
                    .filter(|a| a.target_key_id == resolved)
                    .collect()
            }
            None => self.aliases.values().collect(),
        }
    }

    pub fn delete_alias(&mut self, alias_name: &str) -> Result<(), KmsError> {
        if self.aliases.remove(alias_name).is_none() {
            return Err(KmsError::AliasNotFound(alias_name.to_string()));
        }
        Ok(())
    }

    pub fn update_alias(
        &mut self,
        alias_name: &str,
        target_key_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(), KmsError> {
        let resolved_key_id = self.resolve_key_id(target_key_id)?;
        if !self.keys.contains_key(&resolved_key_id) {
            return Err(not_found_error(target_key_id));
        }

        let alias = self
            .aliases
            .get_mut(alias_name)
            .ok_or_else(|| KmsError::AliasNotFound(alias_name.to_string()))?;
        alias.target_key_id = resolved_key_id;
        alias.alias_arn = format!("arn:aws:kms:{region}:{account_id}:{alias_name}");
        Ok(())
    }

    pub fn update_key_description(
        &mut self,
        key_id_or_alias: &str,
        description: &str,
    ) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        // Check key state before mutable borrow
        {
            let key = self
                .keys
                .get(&key_id)
                .ok_or_else(|| not_found_error(&key_id))?;
            self.ensure_key_usable_for_management(key)?;
        }
        let key = self.keys.get_mut(&key_id).unwrap();
        key.description = description.to_string();
        Ok(())
    }

    pub fn tag_resource(
        &mut self,
        key_id_or_alias: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        {
            let key = self
                .keys
                .get(&key_id)
                .ok_or_else(|| not_found_error(&key_id))?;
            self.ensure_key_usable_for_management(key)?;
        }
        let key = self.keys.get_mut(&key_id).unwrap();
        for (k, v) in tags {
            key.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        key_id_or_alias: &str,
        tag_keys: &[String],
    ) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        {
            let key = self
                .keys
                .get(&key_id)
                .ok_or_else(|| not_found_error(&key_id))?;
            self.ensure_key_usable_for_management(key)?;
        }
        let key = self.keys.get_mut(&key_id).unwrap();
        for k in tag_keys {
            key.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_resource_tags(
        &self,
        key_id_or_alias: &str,
    ) -> Result<&HashMap<String, String>, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;
        Ok(&key.tags)
    }

    pub fn enable_key_rotation(&mut self, key_id_or_alias: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        {
            let key = self
                .keys
                .get(&key_id)
                .ok_or_else(|| not_found_error(&key_id))?;
            self.ensure_key_usable_for_management(key)?;
            if key.key_spec != "SYMMETRIC_DEFAULT" {
                return Err(KmsError::KeyNotValidForRotation(key.arn.clone()));
            }
        }
        let key = self.keys.get_mut(&key_id).unwrap();
        key.key_rotation_enabled = true;
        Ok(())
    }

    pub fn disable_key_rotation(&mut self, key_id_or_alias: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        {
            let key = self
                .keys
                .get(&key_id)
                .ok_or_else(|| not_found_error(&key_id))?;
            self.ensure_key_usable_for_management(key)?;
        }
        let key = self.keys.get_mut(&key_id).unwrap();
        key.key_rotation_enabled = false;
        Ok(())
    }

    pub fn get_key_rotation_status(&self, key_id_or_alias: &str) -> Result<&Key, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        self.keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))
    }

    pub fn get_key_policy(&self, key_id_or_alias: &str) -> Result<(String, String), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;
        let policy = self
            .key_policies
            .get(&key_id)
            .cloned()
            .unwrap_or_else(|| default_key_policy(&key.account_id));
        Ok((key_id, policy))
    }

    pub fn put_key_policy(&mut self, key_id_or_alias: &str, policy: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }
        self.key_policies.insert(key_id, policy.to_string());
        Ok(())
    }

    pub fn list_key_policies(&self, key_id_or_alias: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }
        Ok(())
    }

    pub fn get_public_key(&self, key_id_or_alias: &str) -> Result<&Key, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;
        self.ensure_key_usable(key)?;
        if key.key_spec == "SYMMETRIC_DEFAULT" {
            return Err(KmsError::GetPublicKeyUnsupported(key.arn.clone()));
        }
        Ok(key)
    }

    pub fn generate_random(&self, number_of_bytes: usize) -> Result<Vec<u8>, KmsError> {
        if number_of_bytes == 0 || number_of_bytes > 1024 {
            return Err(KmsError::InvalidNumberOfBytes);
        }
        let mut bytes = vec![0u8; number_of_bytes];
        rand::rng().fill(&mut bytes[..]);
        Ok(bytes)
    }

    pub fn generate_mac(
        &self,
        key_id_or_alias: &str,
        message: &[u8],
        mac_algorithm: &str,
    ) -> Result<(String, Vec<u8>, String), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;
        self.ensure_key_usable(key)?;
        if key.key_usage != "GENERATE_VERIFY_MAC" {
            return Err(KmsError::KeyUsageIncompatibleWithGenerateMac(
                key.arn.clone(),
                key.key_usage.clone(),
            ));
        }
        if !key.key_spec.starts_with("HMAC_") {
            return Err(KmsError::KeySpecIncompatibleWithGenerateMac(
                key.arn.clone(),
                key.key_spec.clone(),
            ));
        }

        use hmac::{Hmac, Mac};
        use sha2::{Sha256, Sha384, Sha512};

        let mac_bytes: Vec<u8> = match mac_algorithm {
            "HMAC_SHA_256" => {
                let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(&key.key_material)
                    .map_err(|_| KmsError::MacGenerationFailed)?;
                mac.update(message);
                mac.finalize().into_bytes().to_vec()
            }
            "HMAC_SHA_384" => {
                let mut mac = <Hmac<Sha384> as Mac>::new_from_slice(&key.key_material)
                    .map_err(|_| KmsError::MacGenerationFailed)?;
                mac.update(message);
                mac.finalize().into_bytes().to_vec()
            }
            "HMAC_SHA_512" => {
                let mut mac = <Hmac<Sha512> as Mac>::new_from_slice(&key.key_material)
                    .map_err(|_| KmsError::MacGenerationFailed)?;
                mac.update(message);
                mac.finalize().into_bytes().to_vec()
            }
            _ => {
                return Err(KmsError::UnsupportedMacAlgorithm(mac_algorithm.to_string()));
            }
        };

        Ok((key.arn.clone(), mac_bytes, mac_algorithm.to_string()))
    }

    pub fn verify_mac(
        &self,
        key_id_or_alias: &str,
        message: &[u8],
        mac_algorithm: &str,
        mac_to_verify: &[u8],
    ) -> Result<(String, bool, String), KmsError> {
        let (key_arn, computed_mac, algo) =
            self.generate_mac(key_id_or_alias, message, mac_algorithm)?;
        let valid = computed_mac == mac_to_verify;
        if !valid {
            return Err(KmsError::MacVerificationFailed);
        }
        Ok((key_arn, valid, algo))
    }

    pub fn sign(
        &self,
        key_id_or_alias: &str,
        message: &[u8],
        signing_algorithm: &str,
    ) -> Result<(String, Vec<u8>, String), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| not_found_error(&key_id))?;
        self.ensure_key_usable(key)?;
        if key.key_usage != "SIGN_VERIFY" {
            return Err(KmsError::KeyUsageIncompatibleWithSign(
                key.arn.clone(),
                key.key_usage.clone(),
            ));
        }

        // For mock purposes, produce a deterministic fake signature:
        // HMAC-SHA256(key_material, message || signing_algorithm)
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(&key.key_material).unwrap();
        mac.update(message);
        mac.update(signing_algorithm.as_bytes());
        let signature = mac.finalize().into_bytes().to_vec();

        Ok((key.arn.clone(), signature, signing_algorithm.to_string()))
    }

    pub fn verify(
        &self,
        key_id_or_alias: &str,
        message: &[u8],
        signing_algorithm: &str,
        signature: &[u8],
    ) -> Result<(String, bool, String), KmsError> {
        let (key_arn, expected_signature, algo) =
            self.sign(key_id_or_alias, message, signing_algorithm)?;
        let valid = expected_signature == signature;
        Ok((key_arn, valid, algo))
    }

    pub fn re_encrypt(
        &self,
        ciphertext_blob: &[u8],
        source_encryption_context: &HashMap<String, String>,
        destination_key_id_or_alias: &str,
        destination_encryption_context: &HashMap<String, String>,
    ) -> Result<ReEncryptResult, KmsError> {
        // First decrypt with source key
        let decrypt_result = self.decrypt(ciphertext_blob, source_encryption_context)?;

        // Then encrypt with destination key
        let dest_key_id = self.resolve_key_id(destination_key_id_or_alias)?;
        let dest_key = self
            .keys
            .get(&dest_key_id)
            .ok_or_else(|| not_found_error(&dest_key_id))?;
        self.ensure_key_usable(dest_key)?;

        let new_ciphertext = encrypt_with_key(
            dest_key,
            &decrypt_result.plaintext,
            destination_encryption_context,
        )?;

        Ok(ReEncryptResult {
            ciphertext_blob: new_ciphertext,
            source_key_id: decrypt_result.key_arn,
            key_id: dest_key.arn.clone(),
        })
    }

    pub fn create_grant(
        &mut self,
        key_id_or_alias: &str,
        grantee_principal: &str,
        retiring_principal: Option<&str>,
        operations: Vec<String>,
        constraints: Option<GrantConstraints>,
        name: Option<&str>,
        account_id: &str,
    ) -> Result<(String, String), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }

        let grant_id = uuid::Uuid::new_v4().to_string();
        let grant_token = uuid::Uuid::new_v4().to_string();
        let issuing_account = format!("arn:aws:iam::{account_id}:root");

        let grant = Grant {
            grant_id: grant_id.clone(),
            grant_token: grant_token.clone(),
            key_id: key_id.clone(),
            grantee_principal: grantee_principal.to_string(),
            retiring_principal: retiring_principal.map(|s| s.to_string()),
            operations,
            constraints,
            issuing_account,
            name: name.map(|s| s.to_string()),
            creation_date: Utc::now(),
        };

        self.grants.insert(grant_id.clone(), grant);
        Ok((grant_id, grant_token))
    }

    pub fn list_grants(
        &self,
        key_id_or_alias: Option<&str>,
        grant_id_filter: Option<&str>,
    ) -> Result<Vec<&Grant>, KmsError> {
        match key_id_or_alias {
            Some(kid) => {
                let key_id = self.resolve_key_id(kid)?;
                if !self.keys.contains_key(&key_id) {
                    return Err(not_found_error(&key_id));
                }
                let mut grants: Vec<&Grant> = self
                    .grants
                    .values()
                    .filter(|g| g.key_id == key_id)
                    .collect();
                if let Some(gid) = grant_id_filter {
                    grants.retain(|g| g.grant_id == gid);
                }
                Ok(grants)
            }
            None => Ok(self.grants.values().collect()),
        }
    }

    pub fn list_retirable_grants(&self, retiring_principal: &str) -> Vec<&Grant> {
        self.grants
            .values()
            .filter(|g| {
                g.retiring_principal
                    .as_deref()
                    .map(|rp| rp == retiring_principal)
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn retire_grant(
        &mut self,
        grant_id: Option<&str>,
        grant_token: Option<&str>,
        key_id_or_alias: Option<&str>,
    ) -> Result<(), KmsError> {
        let id = if let Some(gid) = grant_id {
            if let Some(kid) = key_id_or_alias {
                let key_id = self.resolve_key_id(kid)?;
                // Verify the grant belongs to the key
                if let Some(g) = self.grants.get(gid)
                    && g.key_id != key_id
                {
                    return Err(KmsError::GrantNotFoundForKey(
                        gid.to_string(),
                        kid.to_string(),
                    ));
                }
            }
            gid.to_string()
        } else if let Some(token) = grant_token {
            self.grants
                .values()
                .find(|g| g.grant_token == token)
                .map(|g| g.grant_id.clone())
                .ok_or(KmsError::GrantNotFound)?
        } else {
            return Err(KmsError::GrantIdOrTokenRequired);
        };

        if self.grants.remove(&id).is_none() {
            return Err(KmsError::GrantIdNotFound(id));
        }
        Ok(())
    }

    pub fn revoke_grant(&mut self, key_id_or_alias: &str, grant_id: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }

        match self.grants.get(grant_id) {
            Some(g) if g.key_id == key_id => {}
            _ => {
                return Err(KmsError::GrantIdNotFoundForKey(grant_id.to_string()));
            }
        }

        self.grants.remove(grant_id);
        Ok(())
    }

    pub fn rotate_key_on_demand(&mut self, key_id_or_alias: &str) -> Result<String, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        {
            let key = self
                .keys
                .get(&key_id)
                .ok_or_else(|| not_found_error(&key_id))?;
            self.ensure_key_usable_for_management(key)?;
            if key.key_spec != "SYMMETRIC_DEFAULT" {
                return Err(KmsError::KeyNotValidForOnDemandRotation(key.arn.clone()));
            }
        }

        // Generate new key material
        let mut new_material = vec![0u8; 32];
        rand::rng().fill(&mut new_material[..]);
        let key = self.keys.get_mut(&key_id).unwrap();
        key.key_material = new_material;

        let rotation = KeyRotation {
            key_id: key_id.clone(),
            rotation_date: Utc::now(),
            rotation_type: "ON_DEMAND".to_string(),
        };
        self.key_rotations
            .entry(key_id.clone())
            .or_default()
            .push(rotation);

        Ok(key_id)
    }

    pub fn list_key_rotations(&self, key_id_or_alias: &str) -> Result<Vec<&KeyRotation>, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }
        Ok(self
            .key_rotations
            .get(&key_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }

    pub fn replicate_key(
        &mut self,
        key_id_or_alias: &str,
        replica_region: &str,
        account_id: &str,
        description: Option<&str>,
        tags: HashMap<String, String>,
    ) -> Result<&Key, KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;

        // Collect needed data from source key before mutating
        let (
            is_multi_region,
            source_arn,
            source_desc,
            source_key_spec,
            source_key_usage,
            source_origin,
            source_material,
            source_public_key_der,
        ) = {
            let source_key = self
                .keys
                .get(&key_id)
                .ok_or_else(|| not_found_error(&key_id))?;
            (
                source_key.multi_region,
                source_key.arn.clone(),
                source_key.description.clone(),
                source_key.key_spec.clone(),
                source_key.key_usage.clone(),
                source_key.origin.clone(),
                source_key.key_material.clone(),
                source_key.public_key_der.clone(),
            )
        };

        if !is_multi_region {
            return Err(KmsError::KeyNotMultiRegion(source_arn));
        }

        let new_key_id = format!("mrk-{}", uuid::Uuid::new_v4().simple());
        let new_arn = format!("arn:aws:kms:{replica_region}:{account_id}:key/{new_key_id}");
        let desc = description.unwrap_or(&source_desc);

        let replica = Key {
            key_id: new_key_id.clone(),
            arn: new_arn,
            account_id: account_id.to_string(),
            region: replica_region.to_string(),
            description: desc.to_string(),
            key_spec: source_key_spec,
            key_usage: source_key_usage,
            key_state: KeyState::Enabled,
            creation_date: Utc::now(),
            enabled: true,
            origin: source_origin,
            key_manager: "CUSTOMER".to_string(),
            deletion_date: None,
            key_rotation_enabled: false,
            key_material: source_material,
            public_key_der: source_public_key_der,
            tags,
            multi_region: true,
        };

        self.keys.insert(new_key_id.clone(), replica);
        Ok(self.keys.get(&new_key_id).unwrap())
    }

    // ---- Custom Key Store operations ----

    pub fn create_custom_key_store(
        &mut self,
        name: &str,
        cloud_hsm_cluster_id: Option<&str>,
        trust_anchor_certificate: Option<&str>,
        custom_key_store_type: Option<&str>,
    ) -> Result<String, KmsError> {
        // Check for duplicate name
        if self
            .custom_key_stores
            .values()
            .any(|cks| cks.custom_key_store_name == name)
        {
            return Err(KmsError::CustomKeyStoreNameAlreadyExists(name.to_string()));
        }
        let id = format!("cks-{}", uuid::Uuid::new_v4().simple());
        let store = CustomKeyStore {
            custom_key_store_id: id.clone(),
            custom_key_store_name: name.to_string(),
            cloud_hsm_cluster_id: cloud_hsm_cluster_id.map(|s| s.to_string()),
            trust_anchor_certificate: trust_anchor_certificate.map(|s| s.to_string()),
            connection_state: "DISCONNECTED".to_string(),
            creation_date: Utc::now(),
            custom_key_store_type: custom_key_store_type.unwrap_or("AWS_CLOUDHSM").to_string(),
        };
        self.custom_key_stores.insert(id.clone(), store);
        Ok(id)
    }

    pub fn describe_custom_key_stores(
        &self,
        custom_key_store_id: Option<&str>,
        custom_key_store_name: Option<&str>,
    ) -> Vec<&CustomKeyStore> {
        self.custom_key_stores
            .values()
            .filter(|cks| {
                if let Some(id) = custom_key_store_id {
                    return cks.custom_key_store_id == id;
                }
                if let Some(name) = custom_key_store_name {
                    return cks.custom_key_store_name == name;
                }
                true
            })
            .collect()
    }

    pub fn update_custom_key_store(
        &mut self,
        custom_key_store_id: &str,
        new_name: Option<&str>,
        trust_anchor_certificate: Option<&str>,
    ) -> Result<(), KmsError> {
        let store = self
            .custom_key_stores
            .get_mut(custom_key_store_id)
            .ok_or_else(|| KmsError::CustomKeyStoreNotFound(custom_key_store_id.to_string()))?;
        if let Some(name) = new_name {
            store.custom_key_store_name = name.to_string();
        }
        if let Some(cert) = trust_anchor_certificate {
            store.trust_anchor_certificate = Some(cert.to_string());
        }
        Ok(())
    }

    pub fn delete_custom_key_store(&mut self, custom_key_store_id: &str) -> Result<(), KmsError> {
        if self.custom_key_stores.remove(custom_key_store_id).is_none() {
            return Err(KmsError::CustomKeyStoreNotFound(
                custom_key_store_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn connect_custom_key_store(&mut self, custom_key_store_id: &str) -> Result<(), KmsError> {
        let store = self
            .custom_key_stores
            .get_mut(custom_key_store_id)
            .ok_or_else(|| KmsError::CustomKeyStoreNotFound(custom_key_store_id.to_string()))?;
        store.connection_state = "CONNECTED".to_string();
        Ok(())
    }

    pub fn disconnect_custom_key_store(
        &mut self,
        custom_key_store_id: &str,
    ) -> Result<(), KmsError> {
        let store = self
            .custom_key_stores
            .get_mut(custom_key_store_id)
            .ok_or_else(|| KmsError::CustomKeyStoreNotFound(custom_key_store_id.to_string()))?;
        store.connection_state = "DISCONNECTED".to_string();
        Ok(())
    }

    // ---- Key Material Import operations ----

    pub fn get_parameters_for_import(
        &self,
        key_id_or_alias: &str,
    ) -> Result<(String, String, String, f64), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }
        // Generate random import token and public key (mock)
        let mut import_token_bytes = vec![0u8; 32];
        rand::rng().fill(&mut import_token_bytes[..]);
        let mut public_key_bytes = vec![0u8; 256];
        rand::rng().fill(&mut public_key_bytes[..]);

        use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
        let import_token = BASE64.encode(&import_token_bytes);
        let public_key = BASE64.encode(&public_key_bytes);
        let valid_to = (Utc::now() + Duration::hours(24)).timestamp() as f64;
        Ok((key_id, import_token, public_key, valid_to))
    }

    pub fn import_key_material(&mut self, key_id_or_alias: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }
        self.imported_key_material.insert(key_id, true);
        Ok(())
    }

    pub fn delete_imported_key_material(&mut self, key_id_or_alias: &str) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }
        self.imported_key_material.remove(&key_id);
        Ok(())
    }

    // ---- UpdatePrimaryRegion ----

    pub fn update_primary_region(
        &self,
        key_id_or_alias: &str,
        _primary_region: &str,
    ) -> Result<(), KmsError> {
        let key_id = self.resolve_key_id(key_id_or_alias)?;
        if !self.keys.contains_key(&key_id) {
            return Err(not_found_error(&key_id));
        }
        Ok(())
    }

    /// Resolve a key reference (key ID, ARN, or alias name) to a raw key ID.
    fn resolve_key_id(&self, key_ref: &str) -> Result<String, KmsError> {
        // Direct key ID
        if self.keys.contains_key(key_ref) {
            return Ok(key_ref.to_string());
        }

        // ARN: arn:aws:kms:region:account:key/key-id
        if key_ref.starts_with("arn:aws:kms:")
            && let Some(key_part) = key_ref.rsplit("key/").next()
            && self.keys.contains_key(key_part)
        {
            return Ok(key_part.to_string());
        }

        // Alias name or alias ARN
        let alias_name = if key_ref.starts_with("arn:aws:kms:") {
            // Extract alias name from alias ARN
            key_ref.rsplit(':').next().unwrap_or(key_ref).to_string()
        } else if key_ref.starts_with("alias/") {
            key_ref.to_string()
        } else {
            // Try as-is (might be a key ID that doesn't exist yet)
            return Err(not_found_error(key_ref));
        };

        if let Some(alias) = self.aliases.get(&alias_name) {
            return Ok(alias.target_key_id.clone());
        }

        Err(not_found_error(key_ref))
    }

    fn ensure_key_usable(&self, key: &Key) -> Result<(), KmsError> {
        if key.key_state == KeyState::PendingDeletion {
            return Err(KmsError::KeyPendingDeletion(key.arn.clone()));
        }
        if !key.enabled {
            return Err(KmsError::KeyDisabled(key.arn.clone()));
        }
        Ok(())
    }

    fn ensure_key_usable_for_management(&self, key: &Key) -> Result<(), KmsError> {
        if key.key_state == KeyState::PendingDeletion {
            return Err(KmsError::KeyPendingDeletion(key.arn.clone()));
        }
        Ok(())
    }
}

/// Result of ReEncrypt.
pub struct ReEncryptResult {
    pub ciphertext_blob: Vec<u8>,
    pub source_key_id: String,
    pub key_id: String,
}

/// Encrypt plaintext using AES-256-GCM with the key's material.
/// Returns a ciphertext blob: [key_id (36 bytes)][iv (12 bytes)][tag+ciphertext from AES-GCM]
fn encrypt_with_key(
    key: &Key,
    plaintext: &[u8],
    encryption_context: &HashMap<String, String>,
) -> Result<Vec<u8>, KmsError> {
    let cipher =
        Aes256Gcm::new_from_slice(&key.key_material).map_err(|_| invalid_ciphertext_error())?;

    let mut iv = [0u8; IV_LEN];
    rand::rng().fill(&mut iv);
    let nonce = Nonce::from_slice(&iv);

    // Serialize encryption context as AAD
    let aad = serialize_encryption_context(encryption_context);

    let ciphertext = cipher
        .encrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: plaintext,
                aad: &aad,
            },
        )
        .map_err(|_| KmsError::EncryptionFailed)?;

    // Build blob: [key_id (36 bytes, zero-padded)] [iv (12 bytes)] [ciphertext+tag]
    let mut blob = Vec::with_capacity(KEY_ID_LEN + IV_LEN + ciphertext.len());
    let mut key_id_bytes = [0u8; KEY_ID_LEN];
    let id_bytes = key.key_id.as_bytes();
    key_id_bytes[..id_bytes.len()].copy_from_slice(id_bytes);
    blob.extend_from_slice(&key_id_bytes);
    blob.extend_from_slice(&iv);
    blob.extend_from_slice(&ciphertext);

    Ok(blob)
}

/// Decrypt a ciphertext blob using AES-256-GCM.
fn decrypt_with_key(
    key: &Key,
    ciphertext_blob: &[u8],
    encryption_context: &HashMap<String, String>,
) -> Result<Vec<u8>, KmsError> {
    if ciphertext_blob.len() < KEY_ID_LEN + IV_LEN {
        return Err(invalid_ciphertext_error());
    }

    let iv = &ciphertext_blob[KEY_ID_LEN..KEY_ID_LEN + IV_LEN];
    let ciphertext = &ciphertext_blob[KEY_ID_LEN + IV_LEN..];

    let cipher =
        Aes256Gcm::new_from_slice(&key.key_material).map_err(|_| invalid_ciphertext_error())?;

    let nonce = Nonce::from_slice(iv);
    let aad = serialize_encryption_context(encryption_context);

    cipher
        .decrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: ciphertext,
                aad: &aad,
            },
        )
        .map_err(|_| invalid_ciphertext_error())
}

/// Serialize encryption context into a deterministic byte string for use as AAD.
fn serialize_encryption_context(ctx: &HashMap<String, String>) -> Vec<u8> {
    if ctx.is_empty() {
        return Vec::new();
    }
    let mut pairs: Vec<(&String, &String)> = ctx.iter().collect();
    pairs.sort_by_key(|(k, _)| *k);
    let s: String = pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(",");
    s.into_bytes()
}

fn default_key_policy(account_id: &str) -> String {
    format!(
        r#"{{"Version":"2012-10-17","Id":"key-default-1","Statement":[{{"Sid":"Enable IAM User Permissions","Effect":"Allow","Principal":{{"AWS":"arn:aws:iam::{}:root"}},"Action":"kms:*","Resource":"*"}}]}}"#,
        account_id
    )
}

fn not_found_error(key_ref: &str) -> KmsError {
    KmsError::KeyNotFound(key_ref.to_string())
}

fn invalid_ciphertext_error() -> KmsError {
    KmsError::InvalidCiphertext
}
