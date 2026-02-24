use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AcmPcaState {
    pub certificate_authorities: HashMap<String, CertificateAuthority>,
}

#[derive(Debug, thiserror::Error)]
pub enum AcmPcaError {
    #[error("Resource {0} not found")]
    ResourceNotFound(String),
    #[error("Certificate {0} not found")]
    CertificateNotFound(String),
    #[error("No policy found for resource {0}")]
    PolicyNotFound(String),
    #[error("Permission for principal {0} not found on CA {1}")]
    PermissionNotFound(String, String),
    #[error("Audit report {0} not found for CA {1}")]
    AuditReportNotFound(String, String),
    #[error(
        "Invalid value for parameter RevocationConfiguration.CrlConfiguration.S3ObjectAcl, value: {0}, valid values: ['PUBLIC_READ', 'BUCKET_OWNER_FULL_CONTROL']"
    )]
    InvalidS3ObjectAclInCrlConfiguration(String),
    #[error(
        "The certificate authority {0} is not in the correct state to have a certificate signing request."
    )]
    InvalidStateCertificate(String),
    #[error("Certificate Authority {0} is not in the DELETED state")]
    CertificateAuthorityNotDeleted(String),
    #[error("Malformed certificate.")]
    MalformedCertificate,
    #[error("The certificate has been revoked with serial number {0}")]
    CertificateRevoked(String),
    #[error("Invalid next token.")]
    InvalidNextToken,
}

impl AcmPcaState {
    pub fn create_certificate_authority(
        &mut self,
        ca_config: CaConfiguration,
        ca_type: &str,
        key_storage_security_standard: Option<&str>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<String, AcmPcaError> {
        let ca_id = uuid::Uuid::new_v4().to_string();
        let partition = region_to_partition(region);
        let arn = format!(
            "arn:{}:acm-pca:{}:{}:certificate-authority/{}",
            partition, region, account_id, ca_id
        );

        let security_standard = key_storage_security_standard
            .unwrap_or("FIPS_140_2_LEVEL_3_OR_HIGHER")
            .to_string();

        // Generate a key pair and CSR using rcgen
        let (private_key_pem, csr_pem) = generate_key_and_csr(&ca_config.subject);

        let ca = CertificateAuthority {
            arn: arn.clone(),
            owner_account: account_id.to_string(),
            ca_type: ca_type.to_string(),
            status: "PENDING_CERTIFICATE".to_string(),
            created_at: Utc::now(),
            last_state_change_at: None,
            not_before: None,
            not_after: None,
            ca_config,
            key_storage_security_standard: security_standard,
            revocation_configuration: None,
            tags,
            private_key_pem,
            csr_pem,
            certificate_pem: None,
            certificate_chain_pem: None,
            issued_certificates: HashMap::new(),
            revoked_certificates: HashMap::new(),
            policy: None,
            permissions: HashMap::new(),
            audit_reports: HashMap::new(),
        };

        self.certificate_authorities.insert(arn.clone(), ca);
        Ok(arn)
    }

    pub fn describe_certificate_authority(
        &self,
        arn: &str,
    ) -> Result<&CertificateAuthority, AcmPcaError> {
        self.certificate_authorities
            .get(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))
    }

    pub fn update_certificate_authority(
        &mut self,
        arn: &str,
        status: Option<&str>,
        revocation_configuration: Option<RevocationConfiguration>,
    ) -> Result<(), AcmPcaError> {
        // Validate S3ObjectAcl if provided
        if let Some(ref rev_config) = revocation_configuration
            && let Some(ref crl_config) = rev_config.crl_configuration
        {
            let valid_acls = ["PUBLIC_READ", "BUCKET_OWNER_FULL_CONTROL"];
            if !valid_acls.contains(&crl_config.s3_object_acl.as_str()) {
                return Err(AcmPcaError::InvalidS3ObjectAclInCrlConfiguration(
                    crl_config.s3_object_acl.clone(),
                ));
            }
        }

        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        if let Some(s) = status {
            ca.status = s.to_string();
            ca.last_state_change_at = Some(Utc::now());
        }
        if let Some(rev_config) = revocation_configuration {
            ca.revocation_configuration = Some(rev_config);
        }

        Ok(())
    }

    pub fn delete_certificate_authority(&mut self, arn: &str) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        ca.status = "DELETED".to_string();
        ca.last_state_change_at = Some(Utc::now());
        Ok(())
    }

    pub fn get_certificate_authority_csr(&self, arn: &str) -> Result<String, AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        Ok(ca.csr_pem.clone())
    }

    pub fn get_certificate_authority_certificate(
        &self,
        arn: &str,
    ) -> Result<(String, Option<String>), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        match &ca.certificate_pem {
            Some(cert) => Ok((cert.clone(), ca.certificate_chain_pem.clone())),
            None => Err(AcmPcaError::InvalidStateCertificate(arn.to_string())),
        }
    }

    pub fn import_certificate_authority_certificate(
        &mut self,
        arn: &str,
        certificate_pem: &str,
        certificate_chain_pem: Option<&str>,
    ) -> Result<(), AcmPcaError> {
        // Validate certificate format
        if !certificate_pem.contains("-----BEGIN CERTIFICATE-----") {
            return Err(AcmPcaError::MalformedCertificate);
        }

        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        ca.certificate_pem = Some(certificate_pem.to_string());
        if let Some(chain) = certificate_chain_pem {
            ca.certificate_chain_pem = Some(chain.to_string());
        }
        ca.status = "ACTIVE".to_string();
        ca.not_before = Some(Utc::now() - chrono::Duration::days(1));
        ca.not_after = Some(Utc::now() + chrono::Duration::days(3650));
        ca.last_state_change_at = Some(Utc::now());

        Ok(())
    }

    pub fn issue_certificate(
        &mut self,
        ca_arn: &str,
        csr_pem: &str,
        signing_algorithm: &str,
        validity_value: i64,
        validity_type: &str,
        template_arn: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<String, AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        let partition = region_to_partition(region);
        let cert_id = uuid::Uuid::new_v4().to_string();
        // Certificate ARN is under the same CA ARN path
        let ca_id = ca_arn.rsplit('/').next().unwrap_or(&cert_id);
        let cert_arn = format!(
            "arn:{}:acm-pca:{}:{}:certificate-authority/{}/certificate/{}",
            partition, region, account_id, ca_id, cert_id
        );

        let is_ca_cert = template_arn
            .map(|t| t.contains("RootCACertificate") || t.contains("SubordinateCACertificate"))
            .unwrap_or(false);

        // Generate signed certificate using the CA's private key
        let ca_private_key_pem = ca.private_key_pem.clone();
        let ca_subject = ca.ca_config.subject.clone();
        let ca_cert_pem_opt = ca.certificate_pem.clone();

        let cert_pem = generate_signed_certificate(
            csr_pem,
            &ca_private_key_pem,
            &ca_subject,
            ca_cert_pem_opt.as_deref(),
            signing_algorithm,
            validity_value,
            validity_type,
            is_ca_cert,
        );

        let ca = self.certificate_authorities.get_mut(ca_arn).unwrap();
        ca.issued_certificates.insert(
            cert_arn.clone(),
            IssuedCertificate {
                arn: cert_arn.clone(),
                certificate_pem: cert_pem,
                is_ca_cert,
            },
        );

        Ok(cert_arn)
    }

    pub fn get_certificate(
        &self,
        ca_arn: &str,
        cert_arn: &str,
    ) -> Result<(String, Option<String>), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        let cert = ca
            .issued_certificates
            .get(cert_arn)
            .ok_or_else(|| AcmPcaError::CertificateNotFound(cert_arn.to_string()))?;

        // Check if the certificate has been revoked
        // Parse the cert to get serial number and check revocation
        // For simplicity, check all revoked serial numbers
        for serial in ca.revoked_certificates.keys() {
            // Check if this cert matches the revoked serial
            // We store the PEM, extract serial from it
            if let Some(cert_serial) = extract_serial_from_pem(&cert.certificate_pem)
                && &cert_serial == serial
            {
                return Err(AcmPcaError::CertificateRevoked(serial.to_string()));
            }
        }

        // For CA certs (root CA cert), no chain
        // For end-entity certs, chain = CA's certificate
        let chain = if cert.is_ca_cert {
            None
        } else {
            ca.certificate_pem.clone()
        };

        Ok((cert.certificate_pem.clone(), chain))
    }

    pub fn list_tags(&self, arn: &str) -> Result<&Vec<Tag>, AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;
        Ok(&ca.tags)
    }

    pub fn tag_certificate_authority(
        &mut self,
        arn: &str,
        tags: Vec<Tag>,
    ) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;
        ca.tags.extend(tags);
        Ok(())
    }

    pub fn untag_certificate_authority(
        &mut self,
        arn: &str,
        tags: Vec<Tag>,
    ) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        for tag_to_remove in &tags {
            ca.tags
                .retain(|t| !(t.key == tag_to_remove.key && t.value == tag_to_remove.value));
        }
        Ok(())
    }

    pub fn list_certificate_authorities(&self) -> Vec<&CertificateAuthority> {
        self.certificate_authorities.values().collect()
    }

    pub fn list_certificate_authorities_paginated(
        &self,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> Result<(Vec<&CertificateAuthority>, Option<String>), AcmPcaError> {
        use base64::Engine as _;

        let max = max_results.unwrap_or(100);

        // Decode the continuation token (base64-encoded CA ARN).
        let start_after: Option<String> = if let Some(token) = next_token {
            match base64::engine::general_purpose::STANDARD.decode(token) {
                Ok(bytes) => String::from_utf8(bytes).ok(),
                Err(_) => return Err(AcmPcaError::InvalidNextToken),
            }
        } else {
            None
        };

        // Sort CAs by ARN for a stable, deterministic order.
        let mut sorted: Vec<&CertificateAuthority> =
            self.certificate_authorities.values().collect();
        sorted.sort_by(|a, b| a.arn.cmp(&b.arn));

        // Skip past the CA whose ARN equals `start_after`.
        let iter: Vec<&CertificateAuthority> = if let Some(ref after) = start_after {
            let pos = sorted
                .iter()
                .position(|c| &c.arn == after)
                .map(|i| i + 1)
                .unwrap_or(sorted.len());
            sorted.into_iter().skip(pos).collect()
        } else {
            sorted
        };

        // Take at most `max` results and determine whether there are more.
        let page: Vec<&CertificateAuthority> = iter.into_iter().take(max + 1).collect();
        let has_more = page.len() > max;
        let page: Vec<&CertificateAuthority> = page.into_iter().take(max).collect();

        let new_token = if has_more {
            page.last()
                .map(|c| base64::engine::general_purpose::STANDARD.encode(c.arn.as_bytes()))
        } else {
            None
        };

        Ok((page, new_token))
    }

    pub fn put_policy(&mut self, arn: &str, policy: &str) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;
        ca.policy = Some(policy.to_string());
        Ok(())
    }

    pub fn get_policy(&self, arn: &str) -> Result<String, AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        ca.policy
            .clone()
            .ok_or_else(|| AcmPcaError::PolicyNotFound(arn.to_string()))
    }

    pub fn delete_policy(&mut self, arn: &str) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;
        ca.policy = None;
        Ok(())
    }

    pub fn revoke_certificate(
        &mut self,
        ca_arn: &str,
        certificate_serial: &str,
        revocation_reason: &str,
    ) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        ca.revoked_certificates.insert(
            certificate_serial.to_string(),
            RevokedCertificate {
                serial_number: certificate_serial.to_string(),
                revocation_reason: revocation_reason.to_string(),
                revocation_time: Utc::now(),
            },
        );

        Ok(())
    }

    pub fn restore_certificate_authority(&mut self, arn: &str) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(arn.to_string()))?;

        if ca.status != "DELETED" {
            return Err(AcmPcaError::CertificateAuthorityNotDeleted(arn.to_string()));
        }

        ca.status = "DISABLED".to_string();
        ca.last_state_change_at = Some(Utc::now());
        Ok(())
    }

    pub fn create_permission(
        &mut self,
        ca_arn: &str,
        principal: &str,
        actions: Vec<String>,
        source_account: Option<String>,
    ) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        ca.permissions.insert(
            principal.to_string(),
            CaPermission {
                principal: principal.to_string(),
                actions,
                source_account,
                created_at: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn delete_permission(&mut self, ca_arn: &str, principal: &str) -> Result<(), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        if ca.permissions.remove(principal).is_none() {
            return Err(AcmPcaError::PermissionNotFound(
                principal.to_string(),
                ca_arn.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_permissions(&self, ca_arn: &str) -> Result<Vec<&CaPermission>, AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        Ok(ca.permissions.values().collect())
    }

    pub fn create_certificate_authority_audit_report(
        &mut self,
        ca_arn: &str,
        s3_bucket_name: &str,
        audit_report_response_format: &str,
    ) -> Result<(String, String), AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get_mut(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        let audit_report_id = uuid::Uuid::new_v4().to_string();
        let s3_key = format!(
            "audit-report/{}/{}.{}",
            ca_arn.rsplit('/').next().unwrap_or("unknown"),
            audit_report_id,
            audit_report_response_format.to_lowercase()
        );

        ca.audit_reports.insert(
            audit_report_id.clone(),
            CaAuditReport {
                audit_report_id: audit_report_id.clone(),
                s3_bucket_name: s3_bucket_name.to_string(),
                s3_key: s3_key.clone(),
                audit_report_response_format: audit_report_response_format.to_string(),
                created_at: Utc::now(),
                status: "SUCCESS".to_string(),
            },
        );

        Ok((audit_report_id, s3_key))
    }

    pub fn describe_certificate_authority_audit_report(
        &self,
        ca_arn: &str,
        audit_report_id: &str,
    ) -> Result<&CaAuditReport, AcmPcaError> {
        let ca = self
            .certificate_authorities
            .get(ca_arn)
            .ok_or_else(|| AcmPcaError::ResourceNotFound(ca_arn.to_string()))?;

        ca.audit_reports.get(audit_report_id).ok_or_else(|| {
            AcmPcaError::AuditReportNotFound(audit_report_id.to_string(), ca_arn.to_string())
        })
    }
}

fn region_to_partition(region: &str) -> &str {
    if region.starts_with("us-gov") {
        "aws-us-gov"
    } else if region.starts_with("cn-") {
        "aws-cn"
    } else if region.starts_with("eusc-") {
        "aws-eusc"
    } else {
        "aws"
    }
}

fn generate_key_and_csr(subject: &CaSubject) -> (String, String) {
    use rcgen::{CertificateParams, DistinguishedName, KeyPair};

    let key_pair = KeyPair::generate().expect("Failed to generate key pair");
    let private_key_pem = key_pair.serialize_pem();

    let mut dn = DistinguishedName::new();
    if let Some(cn) = &subject.common_name {
        dn.push(rcgen::DnType::CommonName, cn.as_str());
    }
    if let Some(c) = &subject.country {
        dn.push(rcgen::DnType::CountryName, c.as_str());
    }
    if let Some(s) = &subject.state {
        dn.push(rcgen::DnType::StateOrProvinceName, s.as_str());
    }
    if let Some(o) = &subject.organization {
        dn.push(rcgen::DnType::OrganizationName, o.as_str());
    }
    if let Some(ou) = &subject.organizational_unit {
        dn.push(rcgen::DnType::OrganizationalUnitName, ou.as_str());
    }
    if let Some(l) = &subject.locality {
        dn.push(rcgen::DnType::LocalityName, l.as_str());
    }

    let mut params = CertificateParams::default();
    params.distinguished_name = dn;

    let csr = params
        .serialize_request(&key_pair)
        .expect("Failed to serialize CSR");
    let csr_pem = csr.pem().expect("Failed to get CSR PEM");

    (private_key_pem, csr_pem.to_string())
}

fn generate_signed_certificate(
    _csr_pem: &str,
    ca_private_key_pem: &str,
    ca_subject: &CaSubject,
    _ca_cert_pem: Option<&str>,
    _signing_algorithm: &str,
    validity_value: i64,
    validity_type: &str,
    is_ca_cert: bool,
) -> String {
    use rcgen::{BasicConstraints, CertificateParams, DistinguishedName, IsCa, KeyPair};

    let ca_key_pair = KeyPair::from_pem(ca_private_key_pem).expect("Failed to parse CA key pair");

    let validity_days = match validity_type {
        "YEARS" => validity_value * 365,
        "MONTHS" => validity_value * 30,
        "DAYS" => validity_value,
        _ => validity_value,
    };

    // Build subject DN (same as CA subject for self-signed CA certs)
    let mut subject_dn = DistinguishedName::new();
    if let Some(cn) = &ca_subject.common_name {
        subject_dn.push(rcgen::DnType::CommonName, cn.as_str());
    }
    if let Some(c) = &ca_subject.country {
        subject_dn.push(rcgen::DnType::CountryName, c.as_str());
    }
    if let Some(s) = &ca_subject.state {
        subject_dn.push(rcgen::DnType::StateOrProvinceName, s.as_str());
    }
    if let Some(o) = &ca_subject.organization {
        subject_dn.push(rcgen::DnType::OrganizationName, o.as_str());
    }
    if let Some(ou) = &ca_subject.organizational_unit {
        subject_dn.push(rcgen::DnType::OrganizationalUnitName, ou.as_str());
    }

    // For a CA cert, self-sign; for end-entity, sign with CA
    if is_ca_cert {
        let mut params = CertificateParams::default();
        params.distinguished_name = subject_dn;
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.not_before = rcgen::date_time_ymd(2020, 1, 1);
        params.not_after = rcgen::date_time_ymd(2020 + (validity_days / 365) as i32, 1, 1);

        let cert = params
            .self_signed(&ca_key_pair)
            .expect("Failed to self-sign CA cert");

        cert.pem()
    } else {
        // Generate a new key pair for the end-entity cert
        let ee_key_pair = KeyPair::generate().expect("Failed to generate EE key pair");

        // CA cert params for issuer
        let mut ca_params = CertificateParams::default();
        ca_params.distinguished_name = subject_dn.clone();
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

        let ca_cert = ca_params
            .self_signed(&ca_key_pair)
            .expect("Failed to self-sign CA cert");

        let mut ee_params = CertificateParams::default();
        ee_params.distinguished_name = subject_dn;
        ee_params.is_ca = IsCa::ExplicitNoCa;
        ee_params.not_before = rcgen::date_time_ymd(2020, 1, 1);
        ee_params.not_after = rcgen::date_time_ymd(2020 + (validity_days / 365) as i32, 1, 1);

        let cert = ee_params
            .signed_by(&ee_key_pair, &ca_cert, &ca_key_pair)
            .expect("Failed to sign certificate");

        cert.pem()
    }
}

fn pem_to_der(pem: &str) -> Vec<u8> {
    let pem = pem.trim();
    let lines: Vec<&str> = pem.lines().collect();
    let base64_str: String = lines
        .iter()
        .filter(|line| !line.starts_with("-----"))
        .copied()
        .collect::<Vec<&str>>()
        .join("");
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(&base64_str)
        .expect("Failed to decode base64")
}

fn extract_serial_from_pem(pem: &str) -> Option<String> {
    // Parse a PEM certificate and extract its serial number in colon-separated hex format
    let der = pem_to_der(pem);
    // A simplistic approach: use rcgen doesn't have cert parsing, so we manually extract
    // the serial number from DER. For X.509 certificates, the serial number is in the
    // TBSCertificate sequence.
    // We'll do a basic ASN.1 parse for the serial number.
    extract_serial_from_der(&der)
}

fn extract_serial_from_der(der: &[u8]) -> Option<String> {
    // Basic ASN.1 DER parsing to extract serial number from X.509 certificate
    // Structure: SEQUENCE { TBSCertificate { version, serialNumber, ... }, ... }
    let mut pos = 0;

    // Outer SEQUENCE
    if pos >= der.len() || der[pos] != 0x30 {
        return None;
    }
    pos += 1;
    pos = skip_length(der, pos)?;

    // TBSCertificate SEQUENCE
    if pos >= der.len() || der[pos] != 0x30 {
        return None;
    }
    pos += 1;
    pos = skip_length(der, pos)?;

    // Version (EXPLICIT TAG [0], optional)
    if pos < der.len() && der[pos] == 0xa0 {
        pos += 1;
        let (len, new_pos) = read_length(der, pos)?;
        pos = new_pos + len;
    }

    // Serial Number (INTEGER)
    if pos >= der.len() || der[pos] != 0x02 {
        return None;
    }
    pos += 1;
    let (len, new_pos) = read_length(der, pos)?;
    pos = new_pos;

    let serial_bytes = &der[pos..pos + len];
    let hex: String = serial_bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(":");
    Some(hex)
}

fn skip_length(der: &[u8], pos: usize) -> Option<usize> {
    let (_len, new_pos) = read_length(der, pos)?;
    Some(new_pos)
}

fn read_length(der: &[u8], pos: usize) -> Option<(usize, usize)> {
    if pos >= der.len() {
        return None;
    }
    let first = der[pos];
    if first < 0x80 {
        Some((first as usize, pos + 1))
    } else {
        let num_bytes = (first & 0x7f) as usize;
        if num_bytes == 0 || pos + 1 + num_bytes > der.len() {
            return None;
        }
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | (der[pos + 1 + i] as usize);
        }
        Some((len, pos + 1 + num_bytes))
    }
}
