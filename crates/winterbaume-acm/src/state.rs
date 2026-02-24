use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AcmState {
    pub certificates: HashMap<String, Certificate>,
    pub account_configuration: ExpiryEventsConfiguration,
    /// Maps (domain_name, idempotency_token, sans, tags) -> certificate ARN
    pub idempotency_tokens: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum AcmError {
    #[error("Could not find certificate {arn}.")]
    ResourceNotFound { arn: String },

    #[error("Certificate is still being validated.")]
    RequestInProgress,

    #[error("AWS internal tags cannot be changed with this API")]
    AwsInternalTagsReadOnly,

    #[error("Member must have length less than or equal to 128")]
    TagKeyTooLong,

    #[error("Member must have length less than or equal to 256")]
    TagValueTooLong,

    #[error("The certificate {arn} contains too many Tags: [{tags_display}]")]
    TooManyTags { arn: String, tags_display: String },

    #[error(
        "Certificate {arn} is not a private certificate issued by AWS Certificate Manager Private Certificate Authority."
    )]
    NotPrivateCertificate { arn: String },
}

impl AcmState {
    pub fn request_certificate(
        &mut self,
        domain_name: &str,
        subject_alternative_names: Vec<String>,
        account_id: &str,
        region: &str,
        tags: Vec<Tag>,
        certificate_authority_arn: Option<&str>,
        idempotency_token: Option<&str>,
    ) -> Result<&Certificate, AcmError> {
        // Check idempotency token
        if let Some(token) = idempotency_token {
            let mut sans_sorted = subject_alternative_names.clone();
            sans_sorted.sort();
            let idemp_key = format!(
                "{}:{}:{:?}:{:?}",
                domain_name,
                token,
                sans_sorted,
                tags.iter().map(|t| (&t.key, &t.value)).collect::<Vec<_>>()
            );
            if let Some(existing_arn) = self.idempotency_tokens.get(&idemp_key) {
                let existing_arn = existing_arn.clone();
                return Ok(self.certificates.get(&existing_arn).unwrap());
            }
            // We'll store the mapping after creating the cert
        }

        // Capture tags snapshot for idempotency key before tags are moved
        let tags_snapshot: Vec<(String, Option<String>)> = tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();

        let cert_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:acm:{region}:{account_id}:certificate/{cert_id}");

        let mut sans = subject_alternative_names;
        if sans.is_empty() {
            sans.push(domain_name.to_string());
        }

        // FIX(terraform-e2e): terraform waits for certificates to reach ISSUED status.
        // Return ISSUED immediately regardless of validation method — this is a mock
        // server and real DNS/email validation is not possible.
        let (status, cert_type) = if certificate_authority_arn.is_some() {
            ("ISSUED".to_string(), "PRIVATE".to_string())
        } else {
            ("ISSUED".to_string(), "AMAZON_ISSUED".to_string())
        };

        let domain_validation_options = if certificate_authority_arn.is_none() {
            sans.iter()
                .map(|san| DomainValidationOption {
                    domain_name: san.clone(),
                    validation_domain: san.clone(),
                    validation_status: "SUCCESS".to_string(),
                })
                .collect()
        } else {
            vec![]
        };

        let cert = Certificate {
            arn: arn.clone(),
            domain_name: domain_name.to_string(),
            status,
            subject_alternative_names: sans,
            created_at: Utc::now(),
            certificate_type: cert_type,
            tags,
            issuer: "Amazon".to_string(),
            key_algorithm: "RSA_2048".to_string(),
            renewal_eligibility: "INELIGIBLE".to_string(),
            options: CertificateOptions {
                certificate_transparency_logging_preference: "ENABLED".to_string(),
            },
            domain_validation_options,
            not_before: None,
            not_after: None,
            certificate_authority_arn: certificate_authority_arn.map(|s| s.to_string()),
            certificate_pem: None,
            certificate_chain: None,
            private_key: None,
        };

        self.certificates.insert(arn.clone(), cert);

        // Store idempotency token mapping
        if let Some(token) = idempotency_token {
            let cert_ref = self.certificates.get(&arn).unwrap();
            let mut sans_sorted = cert_ref.subject_alternative_names.clone();
            sans_sorted.sort();
            let idemp_key = format!(
                "{}:{}:{:?}:{:?}",
                domain_name, token, sans_sorted, tags_snapshot
            );
            self.idempotency_tokens.insert(idemp_key, arn.clone());
        }

        Ok(self.certificates.get(&arn).unwrap())
    }

    pub fn import_certificate(
        &mut self,
        certificate_pem: &str,
        private_key: &str,
        certificate_chain: Option<&str>,
        account_id: &str,
        region: &str,
        tags: Vec<Tag>,
    ) -> Result<&Certificate, AcmError> {
        // Extract domain name from certificate (simplified: use *.moto.com as common name)
        let domain_name =
            extract_common_name(certificate_pem).unwrap_or_else(|| "unknown".to_string());
        let key_algorithm = detect_key_algorithm(certificate_pem);

        let cert_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:acm:{region}:{account_id}:certificate/{cert_id}");

        let cert = Certificate {
            arn: arn.clone(),
            domain_name: domain_name.clone(),
            status: "ISSUED".to_string(),
            subject_alternative_names: vec![domain_name],
            created_at: Utc::now(),
            certificate_type: "IMPORTED".to_string(),
            tags,
            issuer: extract_issuer(certificate_pem).unwrap_or_else(|| "Unknown".to_string()),
            key_algorithm,
            renewal_eligibility: "INELIGIBLE".to_string(),
            options: CertificateOptions {
                certificate_transparency_logging_preference: "ENABLED".to_string(),
            },
            domain_validation_options: vec![],
            not_before: extract_not_before(certificate_pem),
            not_after: extract_not_after(certificate_pem),
            certificate_authority_arn: None,
            certificate_pem: Some(certificate_pem.to_string()),
            certificate_chain: certificate_chain.map(|s| s.to_string()),
            private_key: Some(private_key.to_string()),
        };

        self.certificates.insert(arn.clone(), cert);
        Ok(self.certificates.get(&arn).unwrap())
    }

    pub fn describe_certificate(&self, arn: &str) -> Result<&Certificate, AcmError> {
        self.certificates
            .get(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn get_certificate(&self, arn: &str) -> Result<&Certificate, AcmError> {
        let cert = self
            .certificates
            .get(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;

        if cert.certificate_pem.is_none() {
            return Err(AcmError::RequestInProgress);
        }

        Ok(cert)
    }

    pub fn list_certificates(&self) -> Vec<&Certificate> {
        self.certificates.values().collect()
    }

    pub fn list_certificates_with_status(&self, statuses: &[String]) -> Vec<&Certificate> {
        if statuses.is_empty() {
            return self.list_certificates();
        }
        self.certificates
            .values()
            .filter(|c| statuses.iter().any(|s| s == &c.status))
            .collect()
    }

    pub fn delete_certificate(&mut self, arn: &str) -> Result<(), AcmError> {
        if self.certificates.remove(arn).is_none() {
            return Err(AcmError::ResourceNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn add_tags_to_certificate(&mut self, arn: &str, tags: Vec<Tag>) -> Result<(), AcmError> {
        let cert = self
            .certificates
            .get_mut(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;

        // Validate tags
        for tag in &tags {
            if tag.key.starts_with("aws:") {
                return Err(AcmError::AwsInternalTagsReadOnly);
            }
            if tag.key.len() > 128 {
                return Err(AcmError::TagKeyTooLong);
            }
            if let Some(ref v) = tag.value {
                if v.len() > 256 {
                    return Err(AcmError::TagValueTooLong);
                }
            }
        }

        // Check total tag count
        let total = cert.tags.len() + tags.len();
        if total > 50 {
            // Build error message with all tags listed
            let mut all_tags: Vec<String> = cert
                .tags
                .iter()
                .map(|t| {
                    format!(
                        "{{Key={},Value={}}}",
                        t.key,
                        t.value.as_deref().unwrap_or("")
                    )
                })
                .collect();
            for t in &tags {
                all_tags.push(format!(
                    "{{Key={},Value={}}}",
                    t.key,
                    t.value.as_deref().unwrap_or("")
                ));
            }
            return Err(AcmError::TooManyTags {
                arn: arn.to_string(),
                tags_display: all_tags.join(", "),
            });
        }

        cert.tags.extend(tags);
        Ok(())
    }

    pub fn list_tags_for_certificate(&self, arn: &str) -> Result<&Vec<Tag>, AcmError> {
        let cert = self
            .certificates
            .get(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        Ok(&cert.tags)
    }

    pub fn remove_tags_from_certificate(
        &mut self,
        arn: &str,
        tags_to_remove: Vec<Tag>,
    ) -> Result<(), AcmError> {
        // Validate tags first
        for tag in &tags_to_remove {
            if tag.key.starts_with("aws:") {
                return Err(AcmError::AwsInternalTagsReadOnly);
            }
        }

        let cert = self
            .certificates
            .get_mut(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;

        for tag_to_remove in &tags_to_remove {
            cert.tags.retain(|existing| {
                if existing.key != tag_to_remove.key {
                    return true; // Different key, keep it
                }
                // Same key - check if value matters
                match &tag_to_remove.value {
                    None => false, // No value specified in removal => remove regardless
                    Some(remove_val) => {
                        // Value specified - only remove if it matches
                        match &existing.value {
                            None => false,                                    // existing has no value, remove it
                            Some(existing_val) => existing_val != remove_val, // keep if different
                        }
                    }
                }
            });
        }
        Ok(())
    }

    pub fn get_account_configuration(&self) -> &ExpiryEventsConfiguration {
        &self.account_configuration
    }

    pub fn put_account_configuration(&mut self, config: ExpiryEventsConfiguration) {
        self.account_configuration = config;
    }

    pub fn renew_certificate(&self, arn: &str) -> Result<(), AcmError> {
        self.certificates
            .get(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        Ok(())
    }

    pub fn resend_validation_email(
        &self,
        arn: &str,
        _domain: &str,
        _validation_domain: &str,
    ) -> Result<(), AcmError> {
        self.certificates
            .get(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        Ok(())
    }

    pub fn revoke_certificate(
        &mut self,
        arn: &str,
        _revocation_reason: &str,
    ) -> Result<(), AcmError> {
        let cert = self
            .certificates
            .get_mut(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        cert.status = "REVOKED".to_string();
        Ok(())
    }

    pub fn update_certificate_options(
        &mut self,
        arn: &str,
        certificate_transparency_logging_preference: Option<&str>,
    ) -> Result<(), AcmError> {
        let cert = self
            .certificates
            .get_mut(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        if let Some(pref) = certificate_transparency_logging_preference {
            cert.options.certificate_transparency_logging_preference = pref.to_string();
        }
        Ok(())
    }

    pub fn export_certificate(&self, arn: &str) -> Result<&Certificate, AcmError> {
        let cert = self
            .certificates
            .get(arn)
            .ok_or_else(|| AcmError::ResourceNotFound {
                arn: arn.to_string(),
            })?;

        if cert.certificate_type != "PRIVATE" {
            return Err(AcmError::NotPrivateCertificate {
                arn: arn.to_string(),
            });
        }

        Ok(cert)
    }
}

/// Extract Common Name (CN) from a PEM certificate using x509-parser
fn extract_common_name(pem_data: &str) -> Option<String> {
    use x509_parser::pem::parse_x509_pem;
    let (_, pem) = parse_x509_pem(pem_data.as_bytes()).ok()?;
    let (_, cert) = x509_parser::parse_x509_certificate(&pem.contents).ok()?;
    let subject = cert.subject();
    for rdn in subject.iter() {
        for attr in rdn.iter() {
            if attr.attr_type() == &x509_parser::oid_registry::OID_X509_COMMON_NAME {
                return attr.as_str().ok().map(|s| s.to_string());
            }
        }
    }
    None
}

/// Extract Issuer Organization (O) from a PEM certificate
fn extract_issuer(pem_data: &str) -> Option<String> {
    use x509_parser::pem::parse_x509_pem;
    let (_, pem) = parse_x509_pem(pem_data.as_bytes()).ok()?;
    let (_, cert) = x509_parser::parse_x509_certificate(&pem.contents).ok()?;
    let issuer = cert.issuer();
    // Try CN first, then O
    for rdn in issuer.iter() {
        for attr in rdn.iter() {
            if attr.attr_type() == &x509_parser::oid_registry::OID_X509_COMMON_NAME {
                return attr.as_str().ok().map(|s| s.to_string());
            }
        }
    }
    for rdn in issuer.iter() {
        for attr in rdn.iter() {
            if attr.attr_type() == &x509_parser::oid_registry::OID_X509_ORGANIZATION_NAME {
                return attr.as_str().ok().map(|s| s.to_string());
            }
        }
    }
    None
}

/// Detect key algorithm from PEM certificate
fn detect_key_algorithm(pem_data: &str) -> String {
    use x509_parser::pem::parse_x509_pem;
    let result = parse_x509_pem(pem_data.as_bytes());
    let (_, pem) = match result {
        Ok(v) => v,
        Err(_) => return "RSA_2048".to_string(),
    };
    let result = x509_parser::parse_x509_certificate(&pem.contents);
    let (_, cert) = match result {
        Ok(v) => v,
        Err(_) => return "RSA_2048".to_string(),
    };

    let public_key = cert.public_key();
    let algorithm_oid = &public_key.algorithm.algorithm;

    // RSA OID: 1.2.840.113549.1.1.1
    let rsa_oid = x509_parser::oid_registry::OID_PKCS1_RSAENCRYPTION;
    // EC OID: 1.2.840.10045.2.1
    let ec_oid = x509_parser::oid_registry::OID_KEY_TYPE_EC_PUBLIC_KEY;

    if *algorithm_oid == rsa_oid {
        // Determine RSA key size from the public key
        let key_bits = public_key
            .parsed()
            .map(|pk| match pk {
                x509_parser::public_key::PublicKey::RSA(rsa) => rsa.key_size(),
                _ => 2048,
            })
            .unwrap_or(2048);

        match key_bits {
            ..=1024 => "RSA_1024".to_string(),
            1025..=2048 => "RSA_2048".to_string(),
            2049..=3072 => "RSA_3072".to_string(),
            _ => "RSA_4096".to_string(),
        }
    } else if *algorithm_oid == ec_oid {
        // Determine EC curve from algorithm parameters
        let params = &public_key.algorithm.parameters;
        if let Some(params) = params {
            if let Ok(oid) = params.as_oid() {
                let oid_str = oid.to_string();
                // P-256 (prime256v1): 1.2.840.10045.3.1.7
                if oid_str == "1.2.840.10045.3.1.7" {
                    return "EC_prime256v1".to_string();
                }
                // P-384 (secp384r1): 1.3.132.0.34
                if oid_str == "1.3.132.0.34" {
                    return "EC_secp384r1".to_string();
                }
                // P-521 (secp521r1): 1.3.132.0.35
                if oid_str == "1.3.132.0.35" {
                    return "EC_secp521r1".to_string();
                }
            }
        }
        "EC_prime256v1".to_string()
    } else {
        "RSA_2048".to_string()
    }
}

/// Extract NotBefore from PEM certificate
fn extract_not_before(pem_data: &str) -> Option<chrono::DateTime<Utc>> {
    use x509_parser::pem::parse_x509_pem;
    let (_, pem) = parse_x509_pem(pem_data.as_bytes()).ok()?;
    let (_, cert) = x509_parser::parse_x509_certificate(&pem.contents).ok()?;
    let nb = cert.validity().not_before;
    chrono::DateTime::from_timestamp(nb.timestamp(), 0)
}

/// Extract NotAfter from PEM certificate
fn extract_not_after(pem_data: &str) -> Option<chrono::DateTime<Utc>> {
    use x509_parser::pem::parse_x509_pem;
    let (_, pem) = parse_x509_pem(pem_data.as_bytes()).ok()?;
    let (_, cert) = x509_parser::parse_x509_certificate(&pem.contents).ok()?;
    let na = cert.validity().not_after;
    chrono::DateTime::from_timestamp(na.timestamp(), 0)
}
