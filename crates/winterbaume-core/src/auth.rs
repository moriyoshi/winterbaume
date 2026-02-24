/// Extract the AWS service name from a request URI.
///
/// Parses the host from URIs like `https://sts.us-east-1.amazonaws.com/`
/// and returns the service name (e.g., "sts").
pub fn extract_service_from_uri(uri: &str) -> Option<String> {
    let host = extract_host(uri)?;
    // AWS endpoint pattern: {service}.{region}.amazonaws.com
    // or: {service}.amazonaws.com (global)
    let first_dot = host.find('.')?;
    let service = &host[..first_dot];
    if service.is_empty() {
        return None;
    }
    Some(service.to_string())
}

/// Extract the AWS region from a request URI.
///
/// Parses URIs like `https://sts.us-east-1.amazonaws.com/`
/// and returns the region (e.g., "us-east-1").
/// Falls back to "us-east-1" if the region cannot be determined.
pub fn extract_region_from_uri(uri: &str) -> String {
    if let Some(host) = extract_host(uri) {
        // Pattern: {service}.{region}.amazonaws.com
        let parts: Vec<&str> = host.split('.').collect();
        if parts.len() >= 4 && parts[parts.len() - 2] == "amazonaws" {
            return parts[1].to_string();
        }
    }
    "us-east-1".to_string()
}

/// Extract the AWS service name from the Authorization header's SigV4 credential scope.
///
/// Parses headers like:
/// `AWS4-HMAC-SHA256 Credential=AKID/20260224/us-east-1/s3/aws4_request, ...`
/// and returns the service name (e.g., "s3").
pub fn extract_service_from_headers(headers: &http::HeaderMap) -> Option<String> {
    let credential = extract_credential_from_headers(headers)?;
    // Credential format: AKID/date/region/service/aws4_request
    let parts: Vec<&str> = credential.split('/').collect();
    if parts.len() >= 4 {
        Some(parts[3].to_string())
    } else {
        None
    }
}

/// Extract the AWS region from the Authorization header's SigV4 credential scope.
///
/// Parses headers like:
/// `AWS4-HMAC-SHA256 Credential=AKID/20260224/us-east-1/s3/aws4_request, ...`
/// and returns the region (e.g., "us-east-1").
pub fn extract_region_from_headers(headers: &http::HeaderMap) -> Option<String> {
    let credential = extract_credential_from_headers(headers)?;
    // Credential format: AKID/date/region/service/aws4_request
    let parts: Vec<&str> = credential.split('/').collect();
    if parts.len() >= 3 {
        Some(parts[2].to_string())
    } else {
        None
    }
}

/// Extract the Credential value from a SigV4 Authorization header.
fn extract_credential_from_headers(headers: &http::HeaderMap) -> Option<String> {
    let auth = headers.get(http::header::AUTHORIZATION)?.to_str().ok()?;
    // Format: AWS4-HMAC-SHA256 Credential=..., SignedHeaders=..., Signature=...
    let credential_start = auth.find("Credential=")?;
    let after = &auth[credential_start + "Credential=".len()..];
    let end = after.find(',')?;
    Some(after[..end].to_string())
}

fn extract_host(uri: &str) -> Option<&str> {
    // Strip scheme
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))?;
    // Take everything before the first '/' or end
    let host = after_scheme.split('/').next()?;
    // Strip port if present
    let host = host.split(':').next()?;
    if host.is_empty() {
        return None;
    }
    Some(host)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_service_regional() {
        assert_eq!(
            extract_service_from_uri("https://sts.us-east-1.amazonaws.com/"),
            Some("sts".to_string())
        );
    }

    #[test]
    fn test_extract_service_global() {
        assert_eq!(
            extract_service_from_uri("https://sts.amazonaws.com/"),
            Some("sts".to_string())
        );
    }

    #[test]
    fn test_extract_region() {
        assert_eq!(
            extract_region_from_uri("https://sts.us-west-2.amazonaws.com/"),
            "us-west-2"
        );
    }

    #[test]
    fn test_extract_region_global_falls_back() {
        assert_eq!(
            extract_region_from_uri("https://sts.amazonaws.com/"),
            "us-east-1"
        );
    }

    fn make_auth_header(credential: &str) -> http::HeaderMap {
        let mut headers = http::HeaderMap::new();
        headers.insert(
            http::header::AUTHORIZATION,
            format!(
                "AWS4-HMAC-SHA256 Credential={}, SignedHeaders=host;x-amz-date, Signature=abc123",
                credential
            )
            .parse()
            .unwrap(),
        );
        headers
    }

    #[test]
    fn test_extract_service_from_headers() {
        let headers = make_auth_header("AKID/20260224/us-east-1/s3/aws4_request");
        assert_eq!(
            extract_service_from_headers(&headers),
            Some("s3".to_string())
        );
    }

    #[test]
    fn test_extract_region_from_headers() {
        let headers = make_auth_header("AKID/20260224/us-west-2/s3/aws4_request");
        assert_eq!(
            extract_region_from_headers(&headers),
            Some("us-west-2".to_string())
        );
    }

    #[test]
    fn test_extract_service_from_headers_missing() {
        let headers = http::HeaderMap::new();
        assert_eq!(extract_service_from_headers(&headers), None);
    }

    #[test]
    fn test_extract_service_from_headers_sts() {
        let headers = make_auth_header("AKID/20260224/us-east-1/sts/aws4_request");
        assert_eq!(
            extract_service_from_headers(&headers),
            Some("sts".to_string())
        );
    }
}
