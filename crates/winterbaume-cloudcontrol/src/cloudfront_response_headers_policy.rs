//! CloudFormation's CloudFront response headers policy handler, backed by the
//! live CloudFront service state.
//! Contract: https://schema.cloudformation.us-east-1.amazonaws.com/aws-cloudfront-responseheaderspolicy.json

use chrono::SecondsFormat;
use serde_json::{Map, Value, json};
use winterbaume_cloudfront::model::ResponseHeadersPolicyConfig;
use winterbaume_cloudfront::state::CloudFrontError;
use winterbaume_cloudfront::types::ResponseHeadersPolicyData;

use crate::cfn_schema::ShapeContext;
use crate::cfn_validate::{Schema, validate};
use crate::handlers::CloudControlService;
use crate::json_patch;
use crate::state::CloudControlError;
use crate::types::{ManagedResource, OperationType, ResourceRequest};

pub(crate) const TYPE_NAME: &str = "AWS::CloudFront::ResponseHeadersPolicy";

/// CloudFront is global; its state lives under this region key.
pub(crate) const CLOUDFRONT_REGION: &str = "us-east-1";

const OVERRIDE: (&str, Schema) = ("Override", Schema::Boolean);
const ITEMS_OF_STRINGS: Schema = Schema::Object {
    properties: &[("Items", Schema::Array(&Schema::String))],
    required: &["Items"],
};

/// `#/definitions/SecurityHeadersConfig` members, in schema order.
const SECURITY_HEADERS: [&str; 6] = [
    "ContentSecurityPolicy",
    "ContentTypeOptions",
    "FrameOptions",
    "ReferrerPolicy",
    "StrictTransportSecurity",
    "XSSProtection",
];

/// Transcribed from the registry schema (checked 2026-11-06).
const SCHEMA: Schema = Schema::Object {
    properties: &[
        ("Id", Schema::String),
        ("LastModifiedTime", Schema::String),
        (
            "ResponseHeadersPolicyConfig",
            Schema::Object {
                properties: &[
                    ("Comment", Schema::String),
                    (
                        "CorsConfig",
                        Schema::Object {
                            properties: &[
                                ("AccessControlAllowCredentials", Schema::Boolean),
                                ("AccessControlAllowHeaders", ITEMS_OF_STRINGS),
                                ("AccessControlAllowMethods", ITEMS_OF_STRINGS),
                                ("AccessControlAllowOrigins", ITEMS_OF_STRINGS),
                                ("AccessControlExposeHeaders", ITEMS_OF_STRINGS),
                                ("AccessControlMaxAgeSec", Schema::Integer),
                                ("OriginOverride", Schema::Boolean),
                            ],
                            required: &[
                                "AccessControlAllowOrigins",
                                "AccessControlAllowHeaders",
                                "AccessControlAllowMethods",
                                "AccessControlAllowCredentials",
                                "OriginOverride",
                            ],
                        },
                    ),
                    (
                        "CustomHeadersConfig",
                        Schema::Object {
                            properties: &[(
                                "Items",
                                Schema::Array(&Schema::Object {
                                    properties: &[
                                        ("Header", Schema::String),
                                        OVERRIDE,
                                        ("Value", Schema::String),
                                    ],
                                    required: &["Header", "Value", "Override"],
                                }),
                            )],
                            required: &["Items"],
                        },
                    ),
                    ("Name", Schema::String),
                    (
                        "RemoveHeadersConfig",
                        Schema::Object {
                            properties: &[(
                                "Items",
                                Schema::Array(&Schema::Object {
                                    properties: &[("Header", Schema::String)],
                                    required: &["Header"],
                                }),
                            )],
                            required: &["Items"],
                        },
                    ),
                    (
                        "SecurityHeadersConfig",
                        Schema::Object {
                            properties: &[
                                (
                                    "ContentSecurityPolicy",
                                    Schema::Object {
                                        properties: &[
                                            ("ContentSecurityPolicy", Schema::String),
                                            OVERRIDE,
                                        ],
                                        required: &["Override", "ContentSecurityPolicy"],
                                    },
                                ),
                                (
                                    "ContentTypeOptions",
                                    Schema::Object {
                                        properties: &[OVERRIDE],
                                        required: &["Override"],
                                    },
                                ),
                                (
                                    "FrameOptions",
                                    Schema::Object {
                                        properties: &[
                                            (
                                                "FrameOption",
                                                Schema::Enum {
                                                    pattern: "^(DENY|SAMEORIGIN)$",
                                                    values: &["DENY", "SAMEORIGIN"],
                                                },
                                            ),
                                            OVERRIDE,
                                        ],
                                        required: &["Override", "FrameOption"],
                                    },
                                ),
                                (
                                    "ReferrerPolicy",
                                    Schema::Object {
                                        properties: &[
                                            OVERRIDE,
                                            (
                                                "ReferrerPolicy",
                                                Schema::Enum {
                                                    pattern: "^(no-referrer|no-referrer-when-downgrade|origin|origin-when-cross-origin|same-origin|strict-origin|strict-origin-when-cross-origin|unsafe-url)$",
                                                    values: &[
                                                        "no-referrer",
                                                        "no-referrer-when-downgrade",
                                                        "origin",
                                                        "origin-when-cross-origin",
                                                        "same-origin",
                                                        "strict-origin",
                                                        "strict-origin-when-cross-origin",
                                                        "unsafe-url",
                                                    ],
                                                },
                                            ),
                                        ],
                                        required: &["Override", "ReferrerPolicy"],
                                    },
                                ),
                                (
                                    "StrictTransportSecurity",
                                    Schema::Object {
                                        properties: &[
                                            ("AccessControlMaxAgeSec", Schema::Integer),
                                            ("IncludeSubdomains", Schema::Boolean),
                                            OVERRIDE,
                                            ("Preload", Schema::Boolean),
                                        ],
                                        required: &["Override", "AccessControlMaxAgeSec"],
                                    },
                                ),
                                (
                                    "XSSProtection",
                                    Schema::Object {
                                        properties: &[
                                            ("ModeBlock", Schema::Boolean),
                                            OVERRIDE,
                                            ("Protection", Schema::Boolean),
                                            ("ReportUri", Schema::String),
                                        ],
                                        required: &["Override", "Protection"],
                                    },
                                ),
                            ],
                            required: &[],
                        },
                    ),
                    (
                        "ServerTimingHeadersConfig",
                        Schema::Object {
                            properties: &[
                                ("Enabled", Schema::Boolean),
                                ("SamplingRate", Schema::Number),
                            ],
                            required: &["Enabled"],
                        },
                    ),
                ],
                required: &["Name"],
            },
        ),
    ],
    required: &["ResponseHeadersPolicyConfig"],
};

/// CFN list properties (`{"Items": [...]}`) whose CloudFront REST-XML shape is
/// `{"Quantity": n, "Items": {"<member>": [...]}}`, keyed by JSON pointer
/// relative to `ResponseHeadersPolicyConfig`.
const XML_LISTS: [(&str, &str); 6] = [
    ("/CorsConfig/AccessControlAllowHeaders", "Header"),
    ("/CorsConfig/AccessControlAllowMethods", "Method"),
    ("/CorsConfig/AccessControlAllowOrigins", "Origin"),
    ("/CorsConfig/AccessControlExposeHeaders", "Header"),
    ("/CustomHeadersConfig", "ResponseHeadersPolicyCustomHeader"),
    ("/RemoveHeadersConfig", "ResponseHeadersPolicyRemoveHeader"),
];

fn invalid(message: impl Into<String>) -> CloudControlError {
    CloudControlError::InvalidRequest {
        message: message.into(),
    }
}

fn cloudfront_error(error: CloudFrontError, identifier: &str) -> CloudControlError {
    match error {
        CloudFrontError::NoSuchResponseHeadersPolicy(_) => CloudControlError::ResourceNotFound {
            type_name: TYPE_NAME.to_string(),
            identifier: identifier.to_string(),
        },
        CloudFrontError::ResponseHeadersPolicyAlreadyExists(name) => {
            CloudControlError::AlreadyExists {
                type_name: TYPE_NAME.to_string(),
                identifier: name,
            }
        }
        other => invalid(other.to_string()),
    }
}

/// Convert a validated CFN `ResponseHeadersPolicyConfig` to the CloudFront
/// model.
fn to_cloudfront(cfn: &Value) -> Result<ResponseHeadersPolicyConfig, CloudControlError> {
    let mut config = cfn.clone();
    for (pointer, member) in XML_LISTS {
        if let Some(list) = config.pointer_mut(pointer)
            && let Some(items) = list.get("Items").and_then(Value::as_array).cloned()
        {
            list["Quantity"] = json!(items.len());
            list["Items"] = json!({ member: items });
        }
    }
    serde_json::from_value(config).map_err(|e| invalid(e.to_string()))
}

/// Strip the REST-XML list wrappers from a serialised CloudFront model.
fn from_cloudfront(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if map.remove("Quantity").is_some() {
                let items = match map.remove("Items") {
                    Some(Value::Object(wrapper)) => wrapper.into_iter().next().map(|(_, v)| v),
                    _ => None,
                };
                map.insert("Items".to_string(), items.unwrap_or_else(|| json!([])));
            }
            map.values_mut().for_each(from_cloudfront);
        }
        Value::Array(items) => items.iter_mut().for_each(from_cloudfront),
        _ => {}
    }
}

fn read_model(policy: &ResponseHeadersPolicyData) -> Value {
    let mut config = serde_json::to_value(&policy.config).unwrap();
    from_cloudfront(&mut config);
    // The read handler reports every unset SecurityHeadersConfig member as an
    // empty object (captured from a live account on 2026-10-27). The empty
    // objects lack required keys, so a patch applied on top of this read
    // model fails validation unless it also replaces them; that is AWS
    // behaviour, not a winterbaume quirk.
    if let Some(security) = config
        .get_mut("SecurityHeadersConfig")
        .and_then(Value::as_object_mut)
    {
        for member in SECURITY_HEADERS {
            security
                .entry(member)
                .or_insert_with(|| Value::Object(Map::new()));
        }
    }
    json!({
        "Id": policy.id,
        "LastModifiedTime": policy
            .last_modified_time
            .to_rfc3339_opts(SecondsFormat::Millis, true),
        "ResponseHeadersPolicyConfig": config,
    })
}

fn managed(policy: &ResponseHeadersPolicyData) -> ManagedResource {
    ManagedResource {
        type_name: TYPE_NAME.to_string(),
        identifier: policy.id.clone(),
        resource_model: read_model(policy).to_string(),
    }
}

fn validated_config(model: &Value) -> Result<ResponseHeadersPolicyConfig, CloudControlError> {
    validate(&SCHEMA, model).map_err(|message| CloudControlError::ValidationFailed { message })?;
    to_cloudfront(&model["ResponseHeadersPolicyConfig"])
}

impl CloudControlService {
    pub(crate) async fn create_response_headers_policy(
        &self,
        desired: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let desired: Value = serde_json::from_str(desired).map_err(|e| invalid(e.to_string()))?;
        let config = validated_config(&desired)?;
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let mut state = state.write().await;
        let name = config.name.clone();
        let resource = state
            .create_response_headers_policy(config)
            .map(managed)
            .map_err(|e| cloudfront_error(e, &name))?;
        Ok(cc_state.record_operation(
            TYPE_NAME,
            &resource.identifier,
            OperationType::Create,
            Some(resource.resource_model),
        ))
    }

    pub(crate) async fn get_response_headers_policy(
        &self,
        id: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ManagedResource, CloudControlError> {
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let state = state.read().await;
        state
            .get_response_headers_policy(id)
            .map(managed)
            .map_err(|e| cloudfront_error(e, id))
    }

    pub(crate) async fn list_response_headers_policies(
        &self,
        ctx: &ShapeContext<'_>,
    ) -> Vec<ManagedResource> {
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let state = state.read().await;
        let mut resources: Vec<_> = state
            .list_response_headers_policies()
            .into_iter()
            .map(managed)
            .collect();
        resources.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        resources
    }

    pub(crate) async fn update_response_headers_policy(
        &self,
        id: &str,
        patch: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        let mut state = state.write().await;
        let previous = read_model(
            state
                .get_response_headers_policy(id)
                .map_err(|e| cloudfront_error(e, id))?,
        );
        let mut model = previous.clone();
        json_patch::apply(&mut model, patch).map_err(invalid)?;
        for property in ["Id", "LastModifiedTime"] {
            if model.get(property) != previous.get(property) {
                return Err(invalid(format!(
                    "Invalid patch update: readOnlyProperties [/properties/{property}] cannot be updated"
                )));
            }
        }
        // Validate the entire patched model before touching CloudFront.
        let config = validated_config(&model)?;
        let resource = state
            .update_response_headers_policy(id, config, None)
            .map(managed)
            .map_err(|e| cloudfront_error(e, id))?;
        Ok(cc_state.record_operation(
            TYPE_NAME,
            id,
            OperationType::Update,
            Some(resource.resource_model),
        ))
    }

    pub(crate) async fn delete_response_headers_policy(
        &self,
        id: &str,
        ctx: &ShapeContext<'_>,
    ) -> Result<ResourceRequest, CloudControlError> {
        let cc_state = self.state.get(ctx.account_id, ctx.region);
        let mut cc_state = cc_state.write().await;
        let state = self
            .cloudfront
            .shared_state()
            .get(ctx.account_id, CLOUDFRONT_REGION);
        state
            .write()
            .await
            .delete_response_headers_policy(id, None)
            .map_err(|e| cloudfront_error(e, id))?;
        Ok(cc_state.record_operation(TYPE_NAME, id, OperationType::Delete, None))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{from_cloudfront, to_cloudfront};

    #[test]
    fn xml_list_wrappers_round_trip() {
        let cfn = json!({
            "Name": "p",
            "CorsConfig": {
                "AccessControlAllowCredentials": false,
                "AccessControlAllowHeaders": {"Items": ["*"]},
                "AccessControlAllowMethods": {"Items": ["GET", "HEAD"]},
                "AccessControlAllowOrigins": {"Items": ["https://example.com"]},
                "OriginOverride": true
            },
            "CustomHeadersConfig": {"Items": [{"Header": "X-A", "Value": "1", "Override": true}]},
            "RemoveHeadersConfig": {"Items": []}
        });
        let config = to_cloudfront(&cfn).unwrap();
        let cors = config.cors_config.as_ref().unwrap();
        assert_eq!(cors.access_control_allow_methods.quantity, 2);
        assert_eq!(
            cors.access_control_allow_methods.items.items,
            ["GET", "HEAD"]
        );
        let mut back = serde_json::to_value(&config).unwrap();
        from_cloudfront(&mut back);
        assert_eq!(back, cfn);
    }
}
