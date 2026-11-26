//! Decoding of `aws-chunked` request bodies ( SigV4 streaming uploads ).
//!
//! Clients that stream an upload, or that send a trailing checksum, frame the
//! body as a sequence of chunks followed by optional trailing headers:
//!
//! ```text
//! <hex size>[;chunk-signature=<sig>]\r\n<data>\r\n
//! ...
//! 0[;chunk-signature=<sig>]\r\n
//! [x-amz-checksum-<alg>:<base64>\r\n]
//! [x-amz-trailer-signature:<sig>\r\n]
//! \r\n
//! ```
//!
//! S3 stores only the decoded data. The request says it is framed this way
//! with `Content-Encoding: aws-chunked` and with a `STREAMING-*` value of
//! `x-amz-content-sha256`; `x-amz-decoded-content-length` gives the size of
//! the decoded data. See
//! <https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming.html>.
//!
//! The emulator does not verify signatures, so chunk and trailer signatures
//! are skipped. Trailing checksums are verified for the algorithms
//! `aws-smithy-checksums` implements ( CRC32, CRC32C, CRC64NVME, SHA-1,
//! SHA-256 ); others are passed through unchecked.

use std::str::FromStr;

use bytes::Bytes;
use http::{HeaderMap, HeaderValue};

const AWS_CHUNKED: &str = "aws-chunked";
const DECODED_CONTENT_LENGTH: &str = "x-amz-decoded-content-length";
const CHECKSUM_PREFIX: &str = "x-amz-checksum-";
const TRAILER_SIGNATURE: &str = "x-amz-trailer-signature";

/// Why an `aws-chunked` body was refused.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ChunkedBodyError {
    /// The framing could not be parsed.
    Malformed(String),
    /// The body ended before a chunk did, or the decoded size disagrees with
    /// `x-amz-decoded-content-length`.
    Incomplete,
    /// A trailing checksum does not match the decoded data. Carries the
    /// algorithm name as S3 spells it in the error message ( `CRC32` ).
    BadDigest(String),
}

/// A decoded `aws-chunked` body.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DecodedBody {
    pub data: Bytes,
    /// Trailing headers, names lower-cased, in the order sent.
    pub trailers: Vec<(String, String)>,
}

/// Whether the request body is `aws-chunked` framed.
///
/// `Content-Encoding` naming `aws-chunked` is what AWS requires; a
/// `STREAMING-*` payload hash ( other than the event-stream one ) is accepted
/// too, because it can only describe a framed body.
pub(crate) fn is_aws_chunked(headers: &HeaderMap) -> bool {
    let encoded = headers
        .get_all(http::header::CONTENT_ENCODING)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .any(|token| token.trim().eq_ignore_ascii_case(AWS_CHUNKED));
    let streaming = headers
        .get("x-amz-content-sha256")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.starts_with("STREAMING-") && !v.ends_with("-EVENTS"));
    encoded || streaming
}

/// Split an `aws-chunked` body into its data and trailing headers.
pub(crate) fn decode(body: &[u8]) -> Result<DecodedBody, ChunkedBodyError> {
    let mut data = Vec::with_capacity(body.len());
    let mut pos = 0;
    loop {
        // A body that ends before the final chunk was cut short.
        let line_end = find_crlf(body, pos).ok_or(ChunkedBodyError::Incomplete)?;
        let header = std::str::from_utf8(&body[pos..line_end])
            .map_err(|_| ChunkedBodyError::Malformed("chunk header is not text".into()))?;
        let size_hex = header.split(';').next().unwrap_or_default().trim();
        let size = usize::from_str_radix(size_hex, 16)
            .map_err(|_| ChunkedBodyError::Malformed(format!("invalid chunk size '{size_hex}'")))?;
        pos = line_end + 2;
        if size == 0 {
            break;
        }
        let end = pos
            .checked_add(size)
            .filter(|end| *end <= body.len())
            .ok_or(ChunkedBodyError::Incomplete)?;
        data.extend_from_slice(&body[pos..end]);
        pos = end;
        if body.get(pos..pos + 2) != Some(b"\r\n".as_slice()) {
            return Err(ChunkedBodyError::Malformed(
                "chunk data is not followed by CRLF".into(),
            ));
        }
        pos += 2;
    }

    let mut trailers = Vec::new();
    // Trailing headers end at an empty line; a body that stops right after
    // the last trailer ( or the final chunk ) is accepted.
    while pos < body.len() {
        let line_end = find_crlf(body, pos).unwrap_or(body.len());
        let line = &body[pos..line_end];
        pos = (line_end + 2).min(body.len());
        if line.is_empty() {
            break;
        }
        let line = std::str::from_utf8(line)
            .map_err(|_| ChunkedBodyError::Malformed("trailing header is not text".into()))?;
        let (name, value) = line.split_once(':').ok_or_else(|| {
            ChunkedBodyError::Malformed(format!("invalid trailing header '{line}'"))
        })?;
        trailers.push((name.trim().to_ascii_lowercase(), value.trim().to_string()));
    }

    Ok(DecodedBody {
        data: Bytes::from(data),
        trailers,
    })
}

/// Decode the body of an `aws-chunked` request in place, so that handlers
/// see the request as if it had been sent unframed:
///
/// * the body becomes the decoded data, checked against
///   `x-amz-decoded-content-length` when that header is present;
/// * trailing checksums are verified and become ordinary `x-amz-checksum-*`
///   headers; the trailer signature is dropped;
/// * `aws-chunked` is removed from `Content-Encoding` ( the header goes when
///   nothing else is left ), and `Content-Length` is the decoded size.
///
/// Requests that are not `aws-chunked` are left alone.
pub(crate) fn decode_request(
    headers: &mut HeaderMap,
    body: &mut Bytes,
) -> Result<(), ChunkedBodyError> {
    if !is_aws_chunked(headers) {
        return Ok(());
    }
    let decoded = decode(body)?;

    if let Some(expected) = headers
        .get(DECODED_CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse::<usize>().ok())
    {
        if expected != decoded.data.len() {
            return Err(ChunkedBodyError::Incomplete);
        }
    }

    for (name, value) in &decoded.trailers {
        if let Some(algorithm) = name.strip_prefix(CHECKSUM_PREFIX) {
            verify_checksum(algorithm, value, &decoded.data)?;
        }
    }

    for (name, value) in decoded.trailers {
        if name == TRAILER_SIGNATURE {
            continue;
        }
        let (Ok(name), Ok(value)) = (
            http::header::HeaderName::from_bytes(name.as_bytes()),
            HeaderValue::from_str(&value),
        ) else {
            continue;
        };
        headers.insert(name, value);
    }

    let remaining: Vec<String> = headers
        .get_all(http::header::CONTENT_ENCODING)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .map(str::trim)
        .filter(|token| !token.is_empty() && !token.eq_ignore_ascii_case(AWS_CHUNKED))
        .map(str::to_string)
        .collect();
    headers.remove(http::header::CONTENT_ENCODING);
    if !remaining.is_empty() {
        if let Ok(value) = HeaderValue::from_str(&remaining.join(", ")) {
            headers.insert(http::header::CONTENT_ENCODING, value);
        }
    }
    headers.insert(
        http::header::CONTENT_LENGTH,
        HeaderValue::from(decoded.data.len()),
    );

    *body = decoded.data;
    Ok(())
}

/// Check a trailing `x-amz-checksum-<algorithm>` value against `data`.
/// Algorithms without an implementation here are not checked.
fn verify_checksum(algorithm: &str, expected: &str, data: &[u8]) -> Result<(), ChunkedBodyError> {
    // `aws-smithy-checksums` treats `md5` as an alias of CRC32; S3 has no
    // `x-amz-checksum-md5` trailer to compare with, so only the names below
    // are verified.
    if !matches!(
        algorithm,
        "crc32" | "crc32c" | "crc64nvme" | "sha1" | "sha256"
    ) {
        return Ok(());
    }
    let Ok(alg) = aws_smithy_checksums::ChecksumAlgorithm::from_str(algorithm) else {
        return Ok(());
    };
    let mut checksum = alg.into_impl();
    checksum.update(data);
    let actual = checksum.header_value();
    if actual.as_bytes() == expected.as_bytes() {
        Ok(())
    } else {
        Err(ChunkedBodyError::BadDigest(algorithm.to_ascii_uppercase()))
    }
}

fn find_crlf(body: &[u8], from: usize) -> Option<usize> {
    body.get(from..)?
        .windows(2)
        .position(|w| w == b"\r\n")
        .map(|i| from + i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (name, value) in pairs {
            map.append(
                http::header::HeaderName::from_bytes(name.as_bytes()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
        map
    }

    #[test]
    fn decodes_unsigned_trailer_body() {
        let body = b"5\r\nhello\r\n0\r\nx-amz-checksum-crc32:NhCmhg==\r\n\r\n";
        let decoded = decode(body).unwrap();
        assert_eq!(decoded.data, Bytes::from_static(b"hello"));
        assert_eq!(
            decoded.trailers,
            vec![("x-amz-checksum-crc32".to_string(), "NhCmhg==".to_string())]
        );
    }

    #[test]
    fn decodes_signed_chunks_across_several_chunks() {
        let sig = "ad80c730a21e5b8d04586a2213dd63b9a0e99e0e2307b0ade35a65485a288648";
        let body = format!(
            "3;chunk-signature={sig}\r\nhel\r\n2;chunk-signature={sig}\r\nlo\r\n0;chunk-signature={sig}\r\n\r\n"
        );
        let decoded = decode(body.as_bytes()).unwrap();
        assert_eq!(decoded.data, Bytes::from_static(b"hello"));
        assert!(decoded.trailers.is_empty());
    }

    #[test]
    fn accepts_body_ending_after_final_chunk() {
        let decoded = decode(b"5\r\nhello\r\n0\r\n").unwrap();
        assert_eq!(decoded.data, Bytes::from_static(b"hello"));
    }

    #[test]
    fn rejects_truncated_chunk() {
        assert_eq!(decode(b"a\r\nhello"), Err(ChunkedBodyError::Incomplete));
        // Without the final zero-length chunk the body was cut short too.
        assert_eq!(decode(b"5\r\nhello\r\n"), Err(ChunkedBodyError::Incomplete));
    }

    #[test]
    fn rejects_bad_chunk_size() {
        assert!(matches!(
            decode(b"zz\r\nhello\r\n0\r\n\r\n"),
            Err(ChunkedBodyError::Malformed(_))
        ));
    }

    #[test]
    fn rejects_missing_crlf_after_data() {
        assert!(matches!(
            decode(b"5\r\nhelloX\r\n0\r\n\r\n"),
            Err(ChunkedBodyError::Malformed(_))
        ));
    }

    #[test]
    fn detection() {
        assert!(is_aws_chunked(&headers(&[(
            "content-encoding",
            "aws-chunked"
        )])));
        assert!(is_aws_chunked(&headers(&[(
            "content-encoding",
            "gzip, aws-chunked"
        )])));
        assert!(is_aws_chunked(&headers(&[(
            "x-amz-content-sha256",
            "STREAMING-UNSIGNED-PAYLOAD-TRAILER"
        )])));
        assert!(!is_aws_chunked(&headers(&[("content-encoding", "gzip")])));
        assert!(!is_aws_chunked(&headers(&[(
            "x-amz-content-sha256",
            "UNSIGNED-PAYLOAD"
        )])));
        assert!(!is_aws_chunked(&headers(&[(
            "x-amz-content-sha256",
            "STREAMING-AWS4-HMAC-SHA256-EVENTS"
        )])));
    }

    #[test]
    fn decode_request_rewrites_headers() {
        let mut h = headers(&[
            ("content-encoding", "aws-chunked, gzip"),
            ("x-amz-decoded-content-length", "5"),
            ("content-length", "52"),
        ]);
        let mut body = Bytes::from_static(
            b"5\r\nhello\r\n0\r\nx-amz-checksum-crc32:NhCmhg==\r\nx-amz-trailer-signature:abc\r\n\r\n",
        );
        decode_request(&mut h, &mut body).unwrap();
        assert_eq!(body, Bytes::from_static(b"hello"));
        assert_eq!(h.get("content-encoding").unwrap(), "gzip");
        assert_eq!(h.get("content-length").unwrap(), "5");
        assert_eq!(h.get("x-amz-checksum-crc32").unwrap(), "NhCmhg==");
        assert!(h.get("x-amz-trailer-signature").is_none());
    }

    #[test]
    fn decode_request_drops_lone_aws_chunked_encoding() {
        let mut h = headers(&[("content-encoding", "aws-chunked")]);
        let mut body = Bytes::from_static(b"5\r\nhello\r\n0\r\n\r\n");
        decode_request(&mut h, &mut body).unwrap();
        assert!(h.get("content-encoding").is_none());
    }

    #[test]
    fn decode_request_checks_decoded_length() {
        let mut h = headers(&[
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "6"),
        ]);
        let mut body = Bytes::from_static(b"5\r\nhello\r\n0\r\n\r\n");
        assert_eq!(
            decode_request(&mut h, &mut body),
            Err(ChunkedBodyError::Incomplete)
        );
    }

    #[test]
    fn decode_request_verifies_every_supported_checksum() {
        // Checksums of b"hello".
        for (alg, good) in [
            ("crc32", "NhCmhg=="),
            ("crc32c", "mnG7TA=="),
            ("sha1", "qvTGHdzF6KLavt4PO0gs2a6pQ00="),
            ("sha256", "LPJNul+wow4m6DsqxbninhsWHlwfp0JecwQzYpOLmCQ="),
        ] {
            let trailer = format!("5\r\nhello\r\n0\r\nx-amz-checksum-{alg}:{good}\r\n\r\n");
            let mut h = headers(&[("content-encoding", "aws-chunked")]);
            let mut body = Bytes::from(trailer);
            assert_eq!(decode_request(&mut h, &mut body), Ok(()), "{alg}");

            let trailer = format!("5\r\nhellp\r\n0\r\nx-amz-checksum-{alg}:{good}\r\n\r\n");
            let mut h = headers(&[("content-encoding", "aws-chunked")]);
            let mut body = Bytes::from(trailer);
            assert_eq!(
                decode_request(&mut h, &mut body),
                Err(ChunkedBodyError::BadDigest(alg.to_ascii_uppercase())),
                "{alg}"
            );
        }
    }

    #[test]
    fn decode_request_leaves_plain_bodies_alone() {
        let mut h = headers(&[("content-type", "text/plain")]);
        let mut body = Bytes::from_static(b"5\r\nhello\r\n0\r\n\r\n");
        decode_request(&mut h, &mut body).unwrap();
        assert_eq!(body, Bytes::from_static(b"5\r\nhello\r\n0\r\n\r\n"));
    }
}
