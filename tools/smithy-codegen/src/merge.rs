//! Merge generated stub arms into existing handler files.
//!
//! This module handles the key workflow: reading an existing handlers.rs,
//! identifying which operations are already implemented, and inserting
//! stub match arms for unimplemented operations.

use std::path::Path;

use anyhow::{Result, bail};

use crate::model::Protocol;

/// Extract the list of operation names already handled in a handlers.rs file.
///
/// For awsJson: looks for string literals in match arms like `"CreateKey" =>`
/// For awsQuery: looks for string literals in match arms like `"CreateUser" =>`
/// For restJson: tries to extract operation names from comments like `// POST /v2/... => CreateEmailIdentity`
pub fn extract_existing_operations(source: &str, protocol: Protocol) -> Vec<String> {
    let mut ops = Vec::new();

    match protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 | Protocol::RpcV2Cbor => {
            // Match pattern: "OperationName" =>
            for line in source.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix('"')
                    && let Some(idx) = rest.find("\" =>")
                {
                    let op_name = &rest[..idx];
                    // Filter out non-operation strings (e.g. error types)
                    if !op_name.is_empty()
                        && op_name.chars().next().unwrap().is_uppercase()
                        && !op_name.contains(' ')
                        && !trimmed.contains("json_error_response(501")
                        && !trimmed.contains("cbor_error_response(501")
                        && !trimmed.contains("NotImplementedError")
                    {
                        ops.push(op_name.to_string());
                    }
                }
            }
        }
        Protocol::AwsQuery | Protocol::Ec2Query => {
            // Same pattern as awsJson for query services
            for line in source.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix('"')
                    && let Some(idx) = rest.find("\" =>")
                {
                    let op_name = &rest[..idx];
                    if !op_name.is_empty()
                        && op_name.chars().next().unwrap().is_uppercase()
                        && !op_name.contains(' ')
                        && !trimmed.contains("MockResponse::error(501")
                        && !trimmed.contains("NotImplementedError")
                    {
                        ops.push(op_name.to_string());
                    }
                }
            }
        }
        Protocol::RestJson1 | Protocol::RestXml => {
            // Look for handler method calls like self.handle_create_email_identity
            // or comments like "CreateEmailIdentity"
            for line in source.lines() {
                let trimmed = line.trim();
                // Match self.handle_xxx calls
                if let Some(idx) = trimmed.find("self.handle_") {
                    let after = &trimmed[idx + "self.handle_".len()..];
                    let method_name: String = after
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !method_name.is_empty() {
                        // Push the acronym-expanded version (e.g. vpc -> VPC for route53)
                        ops.push(snake_to_pascal(&method_name));
                        // Also push the plain title-case version to handle services like
                        // CloudFront where smithy uses Vpc/Ip/Dns instead of VPC/IP/DNS
                        let plain = snake_to_pascal_plain(&method_name);
                        if plain != ops[ops.len() - 1] {
                            ops.push(plain);
                        }
                    }
                }
            }
        }
    }

    ops.sort();
    ops.dedup();
    ops
}

/// Extract ALL operation names that have any match arm (including 501 stubs).
///
/// Used to avoid double-injection of stub arms.
pub fn extract_all_match_arms(source: &str, protocol: Protocol) -> Vec<String> {
    let mut ops = Vec::new();

    match protocol {
        Protocol::AwsJson1_0
        | Protocol::AwsJson1_1
        | Protocol::AwsQuery
        | Protocol::Ec2Query
        | Protocol::RpcV2Cbor => {
            for line in source.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix('"')
                    && let Some(idx) = rest.find("\" =>")
                {
                    let op_name = &rest[..idx];
                    if !op_name.is_empty()
                        && op_name.chars().next().unwrap().is_uppercase()
                        && !op_name.contains(' ')
                    {
                        ops.push(op_name.to_string());
                    }
                }
            }
        }
        Protocol::RestJson1 | Protocol::RestXml => {
            // For REST protocols, check for comment markers from stub injection
            for line in source.lines() {
                let trimmed = line.trim();
                // Match comment stubs: // METHOD /path => OpName (not implemented)
                if let Some(rest) = trimmed.strip_prefix("// ")
                    && rest.contains("=> ")
                    && rest.contains("(not implemented)")
                    && let Some(op_part) = rest.split("=> ").nth(1)
                {
                    let op_name = op_part.trim_end_matches(" (not implemented)").trim();
                    if !op_name.is_empty() {
                        ops.push(op_name.to_string());
                    }
                }
                // Also include handler calls
                if let Some(idx) = trimmed.find("self.handle_") {
                    let after = &trimmed[idx + "self.handle_".len()..];
                    let method_name: String = after
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !method_name.is_empty() {
                        ops.push(snake_to_pascal(&method_name));
                        // Also push plain title-case for services with mixed-case acronyms
                        let plain = snake_to_pascal_plain(&method_name);
                        ops.push(plain);
                    }
                }
            }
        }
    }

    ops.sort();
    ops.dedup();
    ops
}

/// Detect the error function call pattern from the catch-all `_ =>` arm.
///
/// Returns the prefix string to use for stub arms. For example, if the
/// catch-all is `_ => cbor_error_response(400, ...)`, returns `"cbor_error_response"`.
/// If it's `_ => MockResponse::error(400, ...)`, returns `"MockResponse::error"`.
pub fn detect_error_function(source: &str) -> Option<String> {
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("_ => ") {
            // Extract the function name (everything before the first '(')
            if let Some(paren_idx) = rest.find('(') {
                let func = rest[..paren_idx].trim();
                if !func.is_empty() {
                    // Skip struct-based error functions like `r53_error_response(&Route53Error {`
                    // These take a struct, not (status, code, message) args.
                    let after_paren = rest[paren_idx + 1..].trim();
                    if after_paren.starts_with('&') && after_paren.contains('{') {
                        // Struct-based pattern — return None so caller uses protocol default
                        return None;
                    }
                    return Some(func.to_string());
                }
            }
        }
    }
    None
}

/// Insert stub arms into an existing handlers.rs file.
///
/// Finds the catch-all `_ =>` arm in the dispatch match and inserts stub arms
/// for unimplemented operations before it.
pub fn insert_stub_arms(source: &str, stub_arms: &str, _protocol: Protocol) -> Result<String> {
    if stub_arms.trim().is_empty() {
        return Ok(source.to_string());
    }

    // Find the catch-all arm to insert before.
    // We look for `_ =>` which is the universal Rust match catch-all pattern.
    // This handles all service-specific error functions (json_error_response,
    // rest_json_error, MockResponse::error, cbor_error_response, xml_error_response,
    // service-specific error constructors like r53_error_response(&Route53Error{...}), etc.)
    let lines: Vec<&str> = source.lines().collect();
    let catch_all_idx = lines.iter().position(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("_ =>")
    });

    match catch_all_idx {
        Some(idx) => {
            let mut result = String::new();
            for (i, line) in lines.iter().enumerate() {
                if i == idx {
                    // Insert stub arms before the catch-all
                    result.push_str(
                        "            // --- Unimplemented operations (auto-generated stubs) ---\n",
                    );
                    result.push_str(stub_arms);
                    result.push('\n');
                }
                result.push_str(line);
                result.push('\n');
            }
            Ok(result)
        }
        None => {
            bail!(
                "Could not find `_ =>` catch-all arm in source. \
                 Please ensure the dispatch match has a `_ =>` fallback arm.",
            );
        }
    }
}

/// Read an existing handlers.rs and return the source code.
pub fn read_handlers(crate_dir: &Path) -> Result<String> {
    let path = crate_dir.join("src").join("handlers.rs");
    if !path.exists() {
        bail!("handlers.rs not found at {}", path.display());
    }
    Ok(std::fs::read_to_string(&path)?)
}

/// Write the modified handlers.rs back.
pub fn write_handlers(crate_dir: &Path, content: &str) -> Result<()> {
    let path = crate_dir.join("src").join("handlers.rs");
    std::fs::write(&path, content)?;
    Ok(())
}

/// Convert snake_case to PascalCase with well-known acronym expansion.
///
/// Examples: `vpc_endpoint` → `VPCEndpoint`, `dns_name` → `DNSName`
fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            // Handle well-known acronyms that should be fully uppercased
            match word {
                "vpc" => "VPC".to_string(),
                "dnssec" => "DNSSEC".to_string(),
                "dns" => "DNS".to_string(),
                "ip" => "IP".to_string(),
                "arn" => "ARN".to_string(),
                "iam" => "IAM".to_string(),
                "ssl" => "SSL".to_string(),
                "tls" => "TLS".to_string(),
                "acl" => "ACL".to_string(),
                "cidr" => "Cidr".to_string(),
                "hls" => "HLS".to_string(),
                "dash" => "DASH".to_string(),
                "url" => "URL".to_string(),
                "uri" => "URI".to_string(),
                _ => {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => {
                            let mut result = c.to_uppercase().to_string();
                            result.extend(chars);
                            result
                        }
                    }
                }
            }
        })
        .collect()
}

/// Convert snake_case to PascalCase without any special acronym expansion.
///
/// Each word is simply title-cased: `vpc_origin` → `VpcOrigin`, `anycast_ip_list` → `AnycastIpList`.
/// Used as a fallback to handle AWS services where smithy uses mixed-case acronyms
/// (e.g., CloudFront uses `VpcOrigin` instead of `VPCOrigin`).
fn snake_to_pascal_plain(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let mut result = c.to_uppercase().to_string();
                    result.extend(chars);
                    result
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_existing_aws_json() {
        let source = r#"
        match action.as_str() {
            "CreateKey" => self.handle_create_key(&state, &body),
            "DescribeKey" => self.handle_describe_key(&state, &body),
            _ => json_error_response(400, "InvalidAction", "..."),
        }
        "#;
        let ops = extract_existing_operations(source, Protocol::AwsJson1_1);
        assert_eq!(ops, vec!["CreateKey", "DescribeKey"]);
    }

    #[test]
    fn test_extract_existing_aws_query() {
        let source = r#"
        match action {
            "CreateUser" => self.handle_create_user(&state, &params),
            "GetUser" => self.handle_get_user(&state, &params),
            _ => MockResponse::error(400, "InvalidAction", "..."),
        }
        "#;
        let ops = extract_existing_operations(source, Protocol::AwsQuery);
        assert_eq!(ops, vec!["CreateUser", "GetUser"]);
    }

    #[test]
    fn test_extract_existing_rest_json() {
        let source = r#"
        match (method, segments[2], segments.len()) {
            ("POST", "identities", 3) => {
                self.handle_create_email_identity(&state, &body)
            }
            ("GET", "identities", 3) => self.handle_list_email_identities(&state),
        }
        "#;
        let ops = extract_existing_operations(source, Protocol::RestJson1);
        assert!(ops.contains(&"CreateEmailIdentity".to_string()));
        assert!(ops.contains(&"ListEmailIdentities".to_string()));
    }

    #[test]
    fn test_snake_to_pascal() {
        assert_eq!(snake_to_pascal("create_key"), "CreateKey");
        assert_eq!(
            snake_to_pascal("list_email_identities"),
            "ListEmailIdentities"
        );
        assert_eq!(
            snake_to_pascal("generate_data_key_without_plaintext"),
            "GenerateDataKeyWithoutPlaintext"
        );
    }

    #[test]
    fn test_stubs_not_counted_as_existing() {
        let source = r#"
        match action.as_str() {
            "CreateKey" => self.handle_create_key(&state, &body),
            "UpdateKeyDescription" => json_error_response(501, "NotImplementedError", "..."),
            _ => json_error_response(400, "InvalidAction", "..."),
        }
        "#;
        let ops = extract_existing_operations(source, Protocol::AwsJson1_1);
        assert_eq!(ops, vec!["CreateKey"]);
        assert!(!ops.contains(&"UpdateKeyDescription".to_string()));
    }
}
