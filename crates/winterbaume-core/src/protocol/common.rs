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

/// Decode a form / query-string component ( `application/x-www-form-urlencoded` ).
///
/// `%XX` escapes are decoded to bytes and `+` is decoded as a space. The
/// decoded bytes are then interpreted as UTF-8 once, so multi-byte characters
/// such as `%E6%97%A5` ( `日` ) survive intact. Invalid UTF-8 sequences are
/// replaced with U+FFFD rather than being dropped or causing a panic.
///
/// A malformed escape is kept as written: a `%` that is not followed by two
/// hexadecimal digits stays a literal `%`, and decoding carries on with the
/// next byte, so `a%zzb` yields `a%zzb` and `100%` yields `100%`. ( Earlier
/// versions dropped the `%` and the two bytes after it, which lost data. )
///
/// Use [`percent_decode_path`] for URI path segments, where `+` is a literal
/// plus sign.
pub fn percent_decode(s: &str) -> String {
    percent_decode_impl(s, true)
}

/// Decode a URI path segment ( or any other RFC 3986 component in which `+`
/// has no special meaning ).
///
/// Identical to [`percent_decode`] except that `+` is kept as a literal plus
/// sign. Escapes are decoded to bytes, the bytes are interpreted as UTF-8
/// once ( invalid sequences become U+FFFD ), and malformed escapes are kept
/// as written in the same way.
pub fn percent_decode_path(s: &str) -> String {
    percent_decode_impl(s, false)
}

fn percent_decode_impl(s: &str, plus_as_space: bool) -> String {
    let input = s.as_bytes();
    let mut result = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        let b = input[i];
        let escape = (b == b'%')
            .then(|| {
                let hi = hex_val(*input.get(i + 1)?)?;
                let lo = hex_val(*input.get(i + 2)?)?;
                Some(hi << 4 | lo)
            })
            .flatten();
        match (b, escape) {
            (_, Some(decoded)) => {
                result.push(decoded);
                i += 3;
                continue;
            }
            (b'+', _) if plus_as_space => result.push(b' '),
            _ => result.push(b),
        }
        i += 1;
    }
    match String::from_utf8(result) {
        Ok(decoded) => decoded,
        Err(err) => String::from_utf8_lossy(err.as_bytes()).into_owned(),
    }
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

/// URL-decode a form / query-string component ( `+` means space ).
/// Alias for [`percent_decode`].
pub fn urldecode(s: &str) -> String {
    percent_decode(s)
}

/// Parse a query string (or form-encoded body) into key-value pairs.
///
/// Handles URL-decoding of both keys and values with form semantics
/// ( `+` means space; see [`percent_decode`] ).
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
        assert_eq!(percent_decode(""), "");
        assert_eq!(percent_decode("a%2Bb%25c"), "a+b%c");
        assert_eq!(percent_decode("%2f%2F"), "//");
    }

    #[test]
    fn test_percent_decode_path_ascii() {
        assert_eq!(percent_decode_path("hello%20world"), "hello world");
        assert_eq!(percent_decode_path("a%2Fb"), "a/b");
        assert_eq!(percent_decode_path("plain"), "plain");
        assert_eq!(percent_decode_path(""), "");
        assert_eq!(percent_decode_path("a%2Bb%25c"), "a+b%c");
    }

    #[test]
    fn test_plus_handling_differs_between_variants() {
        assert_eq!(percent_decode("a+b"), "a b");
        assert_eq!(urldecode("a+b"), "a b");
        assert_eq!(percent_decode_path("a+b"), "a+b");
        // An escaped plus is a plus in both variants.
        assert_eq!(percent_decode("a%2Bb+c"), "a+b c");
        assert_eq!(percent_decode_path("a%2Bb+c"), "a+b+c");
    }

    #[test]
    fn test_percent_decode_multibyte_utf8() {
        for decode in [percent_decode, percent_decode_path, urldecode] {
            // 2-byte sequence
            assert_eq!(decode("%C3%A9"), "é");
            assert_eq!(decode("%c3%a9"), "é");
            // 3-byte sequences
            assert_eq!(decode("%E6%97%A5%E6%9C%AC"), "日本");
            // 4-byte sequence
            assert_eq!(decode("%F0%9F%98%80"), "😀");
            // mixed literal and escaped
            assert_eq!(decode("p/%C3%A9.txt"), "p/é.txt");
            assert_eq!(decode("a%20%E6%97%A5b%F0%9F%98%80c%2F"), "a 日b😀c/");
        }
    }

    #[test]
    fn test_percent_decode_unescaped_non_ascii_passes_through() {
        for decode in [percent_decode, percent_decode_path] {
            assert_eq!(decode("é"), "é");
            assert_eq!(decode("日本/%E8%AA%9E"), "日本/語");
            assert_eq!(decode("😀%20😀"), "😀 😀");
        }
    }

    #[test]
    fn test_percent_decode_invalid_utf8_is_replaced() {
        for decode in [percent_decode, percent_decode_path] {
            // A lone continuation byte, a lone lead byte, and Latin-1 `é`.
            assert_eq!(decode("%80"), "\u{FFFD}");
            assert_eq!(decode("a%C3"), "a\u{FFFD}");
            assert_eq!(decode("a%E9b"), "a\u{FFFD}b");
            // A truncated 3-byte sequence followed by valid text.
            assert_eq!(decode("%E6%97x"), "\u{FFFD}x");
            // Overlong / invalid lead bytes never panic.
            assert_eq!(decode("%FF%FE"), "\u{FFFD}\u{FFFD}");
        }
    }

    #[test]
    fn test_percent_decode_malformed_escapes_are_kept() {
        for decode in [percent_decode, percent_decode_path] {
            assert_eq!(decode("a%zzb"), "a%zzb");
            assert_eq!(decode("a%4"), "a%4");
            assert_eq!(decode("a%"), "a%");
            assert_eq!(decode("%"), "%");
            assert_eq!(decode("a%4zb"), "a%4zb");
            assert_eq!(decode("100%"), "100%");
            // A valid escape right after a malformed one is still decoded.
            assert_eq!(decode("%%41"), "%A");
            assert_eq!(decode("%4%42"), "%4B");
            // The bytes after a malformed `%` are not consumed, so a
            // multi-byte character that follows it survives.
            assert_eq!(decode("%é"), "%é");
            assert_eq!(decode("%a日"), "%a日");
        }
    }

    #[test]
    fn test_parse_query_string() {
        let qs = "Action=CreateUser&UserName=test%20user&Version=2010-05-08";
        let params = parse_query_string(qs);
        assert_eq!(params.get("Action").unwrap(), "CreateUser");
        assert_eq!(params.get("UserName").unwrap(), "test user");
        assert_eq!(params.get("Version").unwrap(), "2010-05-08");
    }

    #[test]
    fn test_parse_query_string_non_ascii() {
        let qs = "%E3%82%AD%E3%83%BC=%E6%97%A5%E6%9C%AC%E8%AA%9E&Name=caf%C3%A9+au+lait&E=%F0%9F%98%80&Plus=1%2B1";
        let params = parse_query_string(qs);
        assert_eq!(params.get("キー").unwrap(), "日本語");
        assert_eq!(params.get("Name").unwrap(), "café au lait");
        assert_eq!(params.get("E").unwrap(), "😀");
        assert_eq!(params.get("Plus").unwrap(), "1+1");
        assert_eq!(params.len(), 4);
    }
}
