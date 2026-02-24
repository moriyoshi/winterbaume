/// A parsed Terraform provider reference.
///
/// Represents a provider like `registry.terraform.io/hashicorp/aws`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderRef {
    /// The registry hostname (e.g. "registry.terraform.io").
    pub hostname: String,
    /// The provider namespace (e.g. "hashicorp").
    pub namespace: String,
    /// The provider type (e.g. "aws").
    pub provider_type: String,
}

impl ProviderRef {
    /// Parse a provider string from a Terraform state file.
    ///
    /// Handles formats like:
    /// - `provider["registry.terraform.io/hashicorp/aws"]`
    /// - `module.foo.provider["registry.terraform.io/hashicorp/aws"]`
    pub fn parse(s: &str) -> Option<Self> {
        // Find the provider["..."] part, possibly preceded by module prefixes.
        let provider_start = s.find("provider[")?;
        let rest = &s[provider_start..];

        // Extract content between the quotes.
        let open_quote = rest.find('"')?;
        let close_quote = rest[open_quote + 1..].find('"')?;
        let provider_path = &rest[open_quote + 1..open_quote + 1 + close_quote];

        // Split by '/' - expect exactly 3 parts: hostname/namespace/type
        let parts: Vec<&str> = provider_path.split('/').collect();
        if parts.len() != 3 {
            return None;
        }

        Some(ProviderRef {
            hostname: parts[0].to_string(),
            namespace: parts[1].to_string(),
            provider_type: parts[2].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_provider() {
        let p = ProviderRef::parse(r#"provider["registry.terraform.io/hashicorp/aws"]"#).unwrap();
        assert_eq!(p.hostname, "registry.terraform.io");
        assert_eq!(p.namespace, "hashicorp");
        assert_eq!(p.provider_type, "aws");
    }

    #[test]
    fn parse_module_prefixed_provider() {
        let p = ProviderRef::parse(r#"module.foo.provider["registry.terraform.io/hashicorp/aws"]"#)
            .unwrap();
        assert_eq!(p.hostname, "registry.terraform.io");
        assert_eq!(p.namespace, "hashicorp");
        assert_eq!(p.provider_type, "aws");
    }

    #[test]
    fn parse_invalid_provider() {
        assert!(ProviderRef::parse("not a provider").is_none());
        assert!(ProviderRef::parse(r#"provider["only/two"]"#).is_none());
    }
}
