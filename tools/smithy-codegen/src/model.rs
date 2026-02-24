//! Smithy 2.0 JSON model parser.
//!
//! Parses Smithy JSON AST into an in-memory IR suitable for code generation.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde_json::Value;

/// The AWS protocol family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    AwsJson1_0,
    AwsJson1_1,
    RestJson1,
    AwsQuery,
    RestXml,
    Ec2Query,
    RpcV2Cbor,
}

impl Protocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::AwsJson1_0 => "awsJson1_0",
            Protocol::AwsJson1_1 => "awsJson1_1",
            Protocol::RestJson1 => "restJson1",
            Protocol::AwsQuery => "awsQuery",
            Protocol::RestXml => "restXml",
            Protocol::Ec2Query => "ec2Query",
            Protocol::RpcV2Cbor => "rpcv2Cbor",
        }
    }
}

/// HTTP method + URI pattern from `smithy.api#http`.
#[derive(Debug, Clone)]
pub struct HttpTrait {
    pub method: String,
    pub uri: String,
    pub code: u16,
}

/// A member of a structure shape.
#[derive(Debug, Clone)]
pub struct ShapeMember {
    pub name: String,
    pub target: String,
    pub required: bool,
    pub http_label: bool,
    pub http_query: Option<String>,
    pub http_header: Option<String>,
    pub http_payload: bool,
    pub http_response_code: bool,
    pub json_name: Option<String>,
    pub xml_name: Option<String>,
    pub xml_flattened: bool,
    pub timestamp_format: Option<String>,
}

/// Resolved type info for code generation.
#[derive(Debug, Clone)]
pub enum ResolvedType {
    String,
    Boolean,
    Integer,
    Long,
    Float,
    Double,
    Blob,
    Timestamp,
    List(Box<ResolvedType>),
    Map(Box<ResolvedType>, Box<ResolvedType>),
    Structure(String),
    Enum(#[allow(dead_code)] String),
    Document,
    Unknown(#[allow(dead_code)] String),
}

/// A parsed operation.
#[derive(Debug, Clone)]
pub struct Operation {
    pub name: String,
    pub input_shape: Option<String>,
    pub output_shape: Option<String>,
    #[allow(dead_code)]
    pub errors: Vec<String>,
    pub http: Option<HttpTrait>,
}

/// A parsed service model.
#[derive(Debug)]
pub struct ServiceModel {
    pub service_name: String,
    pub sdk_id: String,
    pub namespace: String,
    pub protocol: Protocol,
    /// All protocols declared by the Smithy model, excluding the primary.
    /// Older SDK clients (like the Go SDK used by Terraform) may still use
    /// a legacy protocol even when the Smithy model prefers rpc-v2-cbor.
    pub additional_protocols: Vec<Protocol>,
    pub operations: Vec<Operation>,
    pub shapes: HashMap<String, Value>,
    pub xml_namespace: Option<String>,
}

impl ServiceModel {
    /// Parse a Smithy JSON model file.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let model: Value = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON from {}", path.display()))?;
        Self::from_json(&model)
    }

    fn from_json(model: &Value) -> Result<Self> {
        let shapes = model
            .get("shapes")
            .and_then(|v| v.as_object())
            .context("Missing 'shapes' in model")?;

        // Find the service shape
        let (service_fqn, service_shape) = shapes
            .iter()
            .find(|(_, v)| v.get("type").and_then(|t| t.as_str()) == Some("service"))
            .context("No service shape found")?;

        let namespace = service_fqn.split('#').next().unwrap_or("").to_string();

        let traits = service_shape.get("traits").and_then(|v| v.as_object());

        // Determine protocol(s)
        let (protocol, additional_protocols) = Self::detect_protocols(traits)?;

        // Extract sdk_id
        let sdk_id = traits
            .and_then(|t| t.get("aws.api#service"))
            .and_then(|v| v.get("sdkId"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let service_name = service_fqn.split('#').next_back().unwrap_or("").to_string();

        // Extract XML namespace if present
        let xml_namespace = traits
            .and_then(|t| t.get("smithy.api#xmlNamespace"))
            .and_then(|v| v.get("uri"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Parse operations (both service-level and resource-level)
        let mut op_targets: Vec<String> = service_shape
            .get("operations")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.get("target").and_then(|t| t.as_str()).map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Also collect operations from resources (recursively)
        if let Some(resources) = service_shape.get("resources").and_then(|v| v.as_array()) {
            Self::collect_resource_operations(resources, shapes, &mut op_targets);
        }

        let mut operations = Vec::new();
        for op_fqn in &op_targets {
            let op_shape = shapes
                .get(op_fqn.as_str())
                .context(format!("Operation shape not found: {op_fqn}"))?;

            let op_name = op_fqn.split('#').next_back().unwrap_or("").to_string();

            let input_shape = op_shape
                .get("input")
                .and_then(|v| v.get("target"))
                .and_then(|v| v.as_str())
                .filter(|s| *s != "smithy.api#Unit")
                .map(String::from);

            let output_shape = op_shape
                .get("output")
                .and_then(|v| v.get("target"))
                .and_then(|v| v.as_str())
                .filter(|s| *s != "smithy.api#Unit")
                .map(String::from);

            let errors = op_shape
                .get("errors")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.get("target").and_then(|t| t.as_str()).map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            let http = op_shape
                .get("traits")
                .and_then(|t| t.get("smithy.api#http"))
                .and_then(|v| {
                    Some(HttpTrait {
                        method: v.get("method")?.as_str()?.to_string(),
                        uri: v.get("uri")?.as_str()?.to_string(),
                        code: v.get("code")?.as_u64()? as u16,
                    })
                });

            operations.push(Operation {
                name: op_name,
                input_shape,
                output_shape,
                errors,
                http,
            });
        }

        operations.sort_by(|a, b| a.name.cmp(&b.name));

        // Collect all shapes
        let all_shapes: HashMap<String, Value> =
            shapes.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        Ok(ServiceModel {
            service_name,
            sdk_id,
            namespace,
            protocol,
            additional_protocols,
            operations,
            shapes: all_shapes,
            xml_namespace,
        })
    }

    /// Recursively collect operation targets from Smithy resource shapes.
    ///
    /// Resources can define operations via lifecycle traits (create, put, read,
    /// update, delete, list) and via explicit `operations` / `collectionOperations`
    /// lists.  Resources may also contain nested sub-resources.
    fn collect_resource_operations(
        resources: &[Value],
        shapes: &serde_json::Map<String, Value>,
        op_targets: &mut Vec<String>,
    ) {
        for resource_ref in resources {
            let target = match resource_ref.get("target").and_then(|v| v.as_str()) {
                Some(t) => t,
                None => continue,
            };
            let resource_shape = match shapes.get(target) {
                Some(s) => s,
                None => continue,
            };

            // Lifecycle operations (single-target)
            for key in &["create", "put", "read", "update", "delete", "list"] {
                if let Some(op_target) = resource_shape
                    .get(*key)
                    .and_then(|v| v.get("target"))
                    .and_then(|v| v.as_str())
                    && !op_targets.contains(&op_target.to_string())
                {
                    op_targets.push(op_target.to_string());
                }
            }

            // Explicit operations and collectionOperations (array-of-targets)
            for key in &["operations", "collectionOperations"] {
                if let Some(arr) = resource_shape.get(*key).and_then(|v| v.as_array()) {
                    for entry in arr {
                        if let Some(op_target) = entry.get("target").and_then(|v| v.as_str())
                            && !op_targets.contains(&op_target.to_string())
                        {
                            op_targets.push(op_target.to_string());
                        }
                    }
                }
            }

            // Recurse into sub-resources
            if let Some(sub_resources) = resource_shape.get("resources").and_then(|v| v.as_array())
            {
                Self::collect_resource_operations(sub_resources, shapes, op_targets);
            }
        }
    }

    fn detect_protocols(
        traits: Option<&serde_json::Map<String, Value>>,
    ) -> Result<(Protocol, Vec<Protocol>)> {
        let traits = traits.context("Service shape has no traits")?;

        // Collect all declared protocols in priority order.
        let candidates: &[(&str, Protocol)] = &[
            ("smithy.protocols#rpcv2Cbor", Protocol::RpcV2Cbor),
            ("aws.protocols#awsJson1_0", Protocol::AwsJson1_0),
            ("aws.protocols#awsJson1_1", Protocol::AwsJson1_1),
            ("aws.protocols#restJson1", Protocol::RestJson1),
            ("aws.protocols#awsQuery", Protocol::AwsQuery),
            ("aws.protocols#restXml", Protocol::RestXml),
            ("aws.protocols#ec2Query", Protocol::Ec2Query),
        ];

        let mut found: Vec<Protocol> = candidates
            .iter()
            .filter(|(key, _)| traits.contains_key(*key))
            .map(|(_, p)| *p)
            .collect();

        if found.is_empty() {
            bail!("Unknown protocol in service traits");
        }

        let primary = found.remove(0);
        Ok((primary, found))
    }

    /// Get members of a structure shape.
    pub fn get_members(&self, shape_fqn: &str) -> Vec<ShapeMember> {
        let shape = match self.shapes.get(shape_fqn) {
            Some(s) => s,
            None => return Vec::new(),
        };

        let members = match shape.get("members").and_then(|v| v.as_object()) {
            Some(m) => m,
            None => return Vec::new(),
        };

        members
            .iter()
            .map(|(name, member)| {
                let traits = member.get("traits").and_then(|v| v.as_object());
                let required = traits
                    .map(|t| t.contains_key("smithy.api#required"))
                    .unwrap_or(false);
                let http_label = traits
                    .map(|t| t.contains_key("smithy.api#httpLabel"))
                    .unwrap_or(false);
                let http_query = traits
                    .and_then(|t| t.get("smithy.api#httpQuery"))
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let http_header = traits
                    .and_then(|t| t.get("smithy.api#httpHeader"))
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let http_payload = traits
                    .map(|t| t.contains_key("smithy.api#httpPayload"))
                    .unwrap_or(false);
                let http_response_code = traits
                    .map(|t| t.contains_key("smithy.api#httpResponseCode"))
                    .unwrap_or(false);
                let json_name = traits
                    .and_then(|t| t.get("smithy.api#jsonName"))
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let xml_name = traits
                    .and_then(|t| t.get("smithy.api#xmlName"))
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let xml_flattened = traits
                    .map(|t| t.contains_key("smithy.api#xmlFlattened"))
                    .unwrap_or(false);
                // Check for @timestampFormat on the member first, then on the target shape
                let timestamp_format = traits
                    .and_then(|t| t.get("smithy.api#timestampFormat"))
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .or_else(|| {
                        let target_fqn =
                            member.get("target").and_then(|v| v.as_str()).unwrap_or("");
                        self.shapes
                            .get(target_fqn)
                            .and_then(|s| s.get("traits"))
                            .and_then(|t| t.get("smithy.api#timestampFormat"))
                            .and_then(|v| v.as_str())
                            .map(String::from)
                    });

                ShapeMember {
                    name: name.clone(),
                    target: member
                        .get("target")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    required,
                    http_label,
                    http_query,
                    http_header,
                    http_payload,
                    http_response_code,
                    json_name,
                    xml_name,
                    xml_flattened,
                    timestamp_format,
                }
            })
            .collect()
    }

    /// Resolve a shape target to a simple type category.
    pub fn resolve_type(&self, target: &str) -> ResolvedType {
        match target {
            "smithy.api#String" | "smithy.api#Unit" => ResolvedType::String,
            "smithy.api#Boolean" | "smithy.api#PrimitiveBoolean" => ResolvedType::Boolean,
            "smithy.api#Integer" | "smithy.api#PrimitiveInteger" => ResolvedType::Integer,
            "smithy.api#Long" | "smithy.api#PrimitiveLong" => ResolvedType::Long,
            "smithy.api#Float" | "smithy.api#PrimitiveFloat" => ResolvedType::Float,
            "smithy.api#Double" | "smithy.api#PrimitiveDouble" => ResolvedType::Double,
            "smithy.api#Blob" => ResolvedType::Blob,
            "smithy.api#Timestamp" => ResolvedType::Timestamp,
            "smithy.api#Document" => ResolvedType::Document,
            _ => {
                if let Some(shape) = self.shapes.get(target) {
                    match shape.get("type").and_then(|t| t.as_str()) {
                        Some("string") => {
                            if shape
                                .get("traits")
                                .and_then(|t| t.get("smithy.api#enum"))
                                .is_some()
                            {
                                ResolvedType::Enum(target.to_string())
                            } else {
                                ResolvedType::String
                            }
                        }
                        Some("enum") => ResolvedType::Enum(target.to_string()),
                        Some("boolean") => ResolvedType::Boolean,
                        Some("integer" | "byte" | "short") => ResolvedType::Integer,
                        Some("long") => ResolvedType::Long,
                        Some("float") => ResolvedType::Float,
                        Some("double") => ResolvedType::Double,
                        Some("blob") => ResolvedType::Blob,
                        Some("timestamp") => ResolvedType::Timestamp,
                        Some("document") => ResolvedType::Document,
                        Some("list") => {
                            let member_target = shape
                                .get("member")
                                .and_then(|m| m.get("target"))
                                .and_then(|t| t.as_str())
                                .unwrap_or("smithy.api#String");
                            ResolvedType::List(Box::new(self.resolve_type(member_target)))
                        }
                        Some("map") => {
                            let key_target = shape
                                .get("key")
                                .and_then(|m| m.get("target"))
                                .and_then(|t| t.as_str())
                                .unwrap_or("smithy.api#String");
                            let val_target = shape
                                .get("value")
                                .and_then(|m| m.get("target"))
                                .and_then(|t| t.as_str())
                                .unwrap_or("smithy.api#String");
                            ResolvedType::Map(
                                Box::new(self.resolve_type(key_target)),
                                Box::new(self.resolve_type(val_target)),
                            )
                        }
                        Some("structure") => ResolvedType::Structure(target.to_string()),
                        Some("union") => ResolvedType::Structure(target.to_string()),
                        _ => ResolvedType::Unknown(target.to_string()),
                    }
                } else {
                    ResolvedType::Unknown(target.to_string())
                }
            }
        }
    }

    /// Get the short name from a fully-qualified shape name.
    pub fn short_name(fqn: &str) -> &str {
        fqn.split('#').next_back().unwrap_or(fqn)
    }

    /// Get the xmlName trait from a shape (shape-level, not member-level).
    pub fn get_shape_xml_name(&self, shape_fqn: &str) -> Option<String> {
        self.shapes
            .get(shape_fqn)?
            .get("traits")?
            .get("smithy.api#xmlName")?
            .as_str()
            .map(String::from)
    }

    /// Get the xmlFlattened trait from a list shape's member.
    #[allow(dead_code)]
    pub fn is_shape_xml_flattened(&self, shape_fqn: &str) -> bool {
        self.shapes
            .get(shape_fqn)
            .and_then(|s| s.get("member"))
            .and_then(|m| m.get("traits"))
            .and_then(|t| t.as_object())
            .map(|t| t.contains_key("smithy.api#xmlFlattened"))
            .unwrap_or(false)
    }

    /// Get the Smithy shape type for a fully-qualified shape name.
    pub fn get_shape_type(&self, shape_fqn: &str) -> Option<&str> {
        self.shapes.get(shape_fqn)?.get("type")?.as_str()
    }

    /// Check whether the referenced shape is a list.
    pub fn is_list_shape(&self, shape_fqn: &str) -> bool {
        self.get_shape_type(shape_fqn) == Some("list")
    }

    /// Get the target of a list shape's member.
    pub fn get_list_member_target(&self, shape_fqn: &str) -> Option<String> {
        self.shapes
            .get(shape_fqn)?
            .get("member")?
            .get("target")?
            .as_str()
            .map(String::from)
    }

    /// Get the list member's xmlName if specified.
    pub fn get_list_member_xml_name(&self, shape_fqn: &str) -> Option<String> {
        self.shapes
            .get(shape_fqn)?
            .get("member")?
            .get("traits")?
            .get("smithy.api#xmlName")?
            .as_str()
            .map(String::from)
    }

    /// Get the default timestamp format for this protocol (body serialization).
    ///
    /// Per the Smithy specification:
    /// - All JSON protocols (awsJson1.0, awsJson1.1, restJson1) default to
    ///   `epoch-seconds` for timestamps in the **request/response body**.
    /// - XML protocols (awsQuery, ec2Query, restXml) default to `date-time`.
    /// - HTTP headers default to `http-date` (handled separately).
    /// - Query strings default to `date-time` (handled separately).
    ///
    /// Individual shapes/members may override with `@timestampFormat`.
    pub fn default_timestamp_format(&self) -> &'static str {
        match self.protocol {
            Protocol::AwsJson1_0
            | Protocol::AwsJson1_1
            | Protocol::RestJson1
            | Protocol::RpcV2Cbor => "epoch-seconds",
            Protocol::AwsQuery | Protocol::Ec2Query | Protocol::RestXml => "date-time",
        }
    }

    /// Check if a protocol uses JSON wire format.
    #[allow(dead_code)]
    pub fn is_json_protocol(&self) -> bool {
        matches!(
            self.protocol,
            Protocol::AwsJson1_0 | Protocol::AwsJson1_1 | Protocol::RestJson1
        )
    }

    /// Check if a protocol uses XML wire format.
    pub fn is_xml_protocol(&self) -> bool {
        matches!(
            self.protocol,
            Protocol::AwsQuery | Protocol::Ec2Query | Protocol::RestXml
        )
    }
}
