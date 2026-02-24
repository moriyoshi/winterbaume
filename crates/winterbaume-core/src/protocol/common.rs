//! Common protocol utilities shared across all AWS protocol families.

use std::collections::HashMap;

/// Extract the path component from an AWS service URI.
///
/// Given a URI like `https://kms.us-east-1.amazonaws.com/some/path?query=1`,
/// returns `/some/path`. Strips the query string.
pub fn extract_path(uri: &str) -> String {
    // Strip scheme (http:// or https://)
    let without_scheme = if let Some(rest) = uri.strip_prefix("https://") {
        rest
    } else if let Some(rest) = uri.strip_prefix("http://") {
        rest
    } else {
        uri
    };
    // Find the start of the path (first '/' after scheme was stripped)
    if let Some(slash) = without_scheme.find('/') {
        let path_and_query = &without_scheme[slash..];
        // Strip query string
        if let Some(q) = path_and_query.find('?') {
            path_and_query[..q].to_string()
        } else {
            path_and_query.to_string()
        }
    } else {
        // No path component — return "/"
        "/".to_string()
    }
}

/// Extract the query string from a URI, returning everything after '?'.
pub fn extract_query_string(uri: &str) -> &str {
    uri.split_once('?').map(|(_, q)| q).unwrap_or("")
}

/// Percent-decode a URI component. Also decodes '+' as space.
pub fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

/// Decode a single hex digit.
pub fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// URL-decode a string. Alias for [`percent_decode`].
pub fn urldecode(s: &str) -> String {
    percent_decode(s)
}

/// Parse a query string (or form-encoded body) into key-value pairs.
///
/// Handles URL-decoding of both keys and values.
pub fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urldecode(key);
            let value = urldecode(value);
            map.insert(key, value);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_path() {
        assert_eq!(
            extract_path("https://kms.us-east-1.amazonaws.com/some/path"),
            "/some/path"
        );
        assert_eq!(
            extract_path("https://kms.us-east-1.amazonaws.com/some/path?query=1"),
            "/some/path"
        );
        assert_eq!(extract_path("https://kms.us-east-1.amazonaws.com"), "/");
        assert_eq!(extract_path("/local/path?q=1"), "/local/path");
    }

    #[test]
    fn test_percent_decode() {
        assert_eq!(percent_decode("hello%20world"), "hello world");
        assert_eq!(percent_decode("hello+world"), "hello world");
        assert_eq!(percent_decode("a%2Fb"), "a/b");
        assert_eq!(percent_decode("plain"), "plain");
    }

    #[test]
    fn test_parse_query_string() {
        let qs = "Action=CreateUser&UserName=test%20user&Version=2010-05-08";
        let params = parse_query_string(qs);
        assert_eq!(params.get("Action").unwrap(), "CreateUser");
        assert_eq!(params.get("UserName").unwrap(), "test user");
        assert_eq!(params.get("Version").unwrap(), "2010-05-08");
    }
}
