//! XML protocol utilities for awsQuery and REST-XML services.

use crate::service::MockResponse;

/// Escape a string for safe inclusion in XML content.
pub fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Wrap inner XML content in a standard awsQuery `<{Action}Response>` envelope.
///
/// Produces:
/// ```xml
/// <{action}Response xmlns="{namespace}">
///   <{action}Result>
///     {inner_xml}
///   </{action}Result>
///   <ResponseMetadata>
///     <RequestId>{request_id}</RequestId>
///   </ResponseMetadata>
/// </{action}Response>
/// ```
pub fn aws_query_response(action: &str, namespace: &str, inner_xml: &str) -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<{action}Response xmlns="{namespace}">
  <{action}Result>
    {inner_xml}
  </{action}Result>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</{action}Response>"#,
    );
    MockResponse::xml(200, xml)
}

/// Create an awsQuery error response with a namespace.
///
/// Produces:
/// ```xml
/// <ErrorResponse xmlns="{namespace}">
///   <Error>
///     <Type>Sender</Type>
///     <Code>{code}</Code>
///     <Message>{message}</Message>
///   </Error>
///   <RequestId>{request_id}</RequestId>
/// </ErrorResponse>
/// ```
pub fn aws_query_error_response(
    status: u16,
    code: &str,
    message: &str,
    namespace: &str,
) -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ErrorResponse xmlns="{namespace}">
  <Error>
    <Type>Sender</Type>
    <Code>{code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>{request_id}</RequestId>
</ErrorResponse>"#,
        namespace = xml_escape(namespace),
        code = xml_escape(code),
        message = xml_escape(message),
    );
    MockResponse::xml(status, xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("a&b"), "a&amp;b");
        assert_eq!(xml_escape("<tag>"), "&lt;tag&gt;");
        assert_eq!(xml_escape(r#""quoted""#), "&quot;quoted&quot;");
        assert_eq!(xml_escape("it's"), "it&apos;s");
        assert_eq!(xml_escape("plain"), "plain");
    }

    #[test]
    fn test_aws_query_response() {
        let resp = aws_query_response(
            "CreateUser",
            "https://iam.amazonaws.com/doc/2010-05-08/",
            "<User><UserName>test</UserName></User>",
        );
        assert_eq!(resp.status, 200);
        let body = std::str::from_utf8(&resp.body).unwrap();
        assert!(body.contains("<CreateUserResponse"));
        assert!(body.contains("<CreateUserResult>"));
        assert!(body.contains("<User><UserName>test</UserName></User>"));
        assert!(body.contains("<RequestId>"));
    }

    #[test]
    fn test_aws_query_error_response() {
        let resp = aws_query_error_response(
            400,
            "EntityAlreadyExists",
            "User already exists",
            "https://iam.amazonaws.com/doc/2010-05-08/",
        );
        assert_eq!(resp.status, 400);
        let body = std::str::from_utf8(&resp.body).unwrap();
        assert!(body.contains("<Code>EntityAlreadyExists</Code>"));
        assert!(body.contains("<Message>User already exists</Message>"));
    }
}
