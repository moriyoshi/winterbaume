//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-wafv2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateWebACLRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    pub web_a_c_l_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateWebACLResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckCapacityRequest {
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(rename = "CaptchaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_config: Option<CaptchaConfig>,
    #[serde(rename = "ChallengeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_config: Option<ChallengeConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OverrideAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<OverrideAction>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "RuleLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_labels: Option<Vec<Label>>,
    #[serde(rename = "Statement")]
    #[serde(default)]
    pub statement: Statement,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    pub visibility_config: VisibilityConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleAction {
    #[serde(rename = "Allow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<AllowAction>,
    #[serde(rename = "Block")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockAction>,
    #[serde(rename = "Captcha")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<CaptchaAction>,
    #[serde(rename = "Challenge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<ChallengeAction>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<CountAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowAction {
    #[serde(rename = "CustomRequestHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_request_handling: Option<CustomRequestHandling>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomRequestHandling {
    #[serde(rename = "InsertHeaders")]
    #[serde(default)]
    pub insert_headers: Vec<CustomHTTPHeader>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomHTTPHeader {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockAction {
    #[serde(rename = "CustomResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response: Option<CustomResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomResponse {
    #[serde(rename = "CustomResponseBodyKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_body_key: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    pub response_code: i32,
    #[serde(rename = "ResponseHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<CustomHTTPHeader>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptchaAction {
    #[serde(rename = "CustomRequestHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_request_handling: Option<CustomRequestHandling>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChallengeAction {
    #[serde(rename = "CustomRequestHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_request_handling: Option<CustomRequestHandling>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CountAction {
    #[serde(rename = "CustomRequestHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_request_handling: Option<CustomRequestHandling>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptchaConfig {
    #[serde(rename = "ImmunityTimeProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImmunityTimeProperty {
    #[serde(rename = "ImmunityTime")]
    #[serde(default)]
    pub immunity_time: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChallengeConfig {
    #[serde(rename = "ImmunityTimeProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverrideAction {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<CountAction>,
    #[serde(rename = "None")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<NoneAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NoneAction {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Label {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Statement {
    #[serde(rename = "AndStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_statement: Option<Box<AndStatement>>,
    #[serde(rename = "AsnMatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_match_statement: Option<AsnMatchStatement>,
    #[serde(rename = "ByteMatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_statement: Option<ByteMatchStatement>,
    #[serde(rename = "GeoMatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_statement: Option<GeoMatchStatement>,
    #[serde(rename = "IPSetReferenceStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_set_reference_statement: Option<IPSetReferenceStatement>,
    #[serde(rename = "LabelMatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_match_statement: Option<LabelMatchStatement>,
    #[serde(rename = "ManagedRuleGroupStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_group_statement: Option<Box<ManagedRuleGroupStatement>>,
    #[serde(rename = "NotStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_statement: Option<Box<NotStatement>>,
    #[serde(rename = "OrStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_statement: Option<Box<OrStatement>>,
    #[serde(rename = "RateBasedStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_based_statement: Option<Box<RateBasedStatement>>,
    #[serde(rename = "RegexMatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_statement: Option<RegexMatchStatement>,
    #[serde(rename = "RegexPatternSetReferenceStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set_reference_statement: Option<RegexPatternSetReferenceStatement>,
    #[serde(rename = "RuleGroupReferenceStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_reference_statement: Option<RuleGroupReferenceStatement>,
    #[serde(rename = "SizeConstraintStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_statement: Option<SizeConstraintStatement>,
    #[serde(rename = "SqliMatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqli_match_statement: Option<SqliMatchStatement>,
    #[serde(rename = "XssMatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_statement: Option<XssMatchStatement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AndStatement {
    #[serde(rename = "Statements")]
    #[serde(default)]
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AsnMatchStatement {
    #[serde(rename = "AsnList")]
    #[serde(default)]
    pub asn_list: Vec<i64>,
    #[serde(rename = "ForwardedIPConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_i_p_config: Option<ForwardedIPConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForwardedIPConfig {
    #[serde(rename = "FallbackBehavior")]
    #[serde(default)]
    pub fallback_behavior: String,
    #[serde(rename = "HeaderName")]
    #[serde(default)]
    pub header_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ByteMatchStatement {
    #[serde(rename = "FieldToMatch")]
    #[serde(default)]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "PositionalConstraint")]
    #[serde(default)]
    pub positional_constraint: String,
    #[serde(rename = "SearchString")]
    #[serde(default)]
    pub search_string: String,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldToMatch {
    #[serde(rename = "AllQueryArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_query_arguments: Option<AllQueryArguments>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    #[serde(rename = "Cookies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Cookies>,
    #[serde(rename = "HeaderOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_order: Option<HeaderOrder>,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
    #[serde(rename = "JA3Fingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_a3_fingerprint: Option<JA3Fingerprint>,
    #[serde(rename = "JA4Fingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_a4_fingerprint: Option<JA4Fingerprint>,
    #[serde(rename = "JsonBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<JsonBody>,
    #[serde(rename = "Method")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<QueryString>,
    #[serde(rename = "SingleHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_header: Option<SingleHeader>,
    #[serde(rename = "SingleQueryArgument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_query_argument: Option<SingleQueryArgument>,
    #[serde(rename = "UriFragment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_fragment: Option<UriFragment>,
    #[serde(rename = "UriPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_path: Option<UriPath>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllQueryArguments {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Body {
    #[serde(rename = "OversizeHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversize_handling: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cookies {
    #[serde(rename = "MatchPattern")]
    #[serde(default)]
    pub match_pattern: CookieMatchPattern,
    #[serde(rename = "MatchScope")]
    #[serde(default)]
    pub match_scope: String,
    #[serde(rename = "OversizeHandling")]
    #[serde(default)]
    pub oversize_handling: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CookieMatchPattern {
    #[serde(rename = "All")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<All>,
    #[serde(rename = "ExcludedCookies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_cookies: Option<Vec<String>>,
    #[serde(rename = "IncludedCookies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_cookies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct All {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeaderOrder {
    #[serde(rename = "OversizeHandling")]
    #[serde(default)]
    pub oversize_handling: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Headers {
    #[serde(rename = "MatchPattern")]
    #[serde(default)]
    pub match_pattern: HeaderMatchPattern,
    #[serde(rename = "MatchScope")]
    #[serde(default)]
    pub match_scope: String,
    #[serde(rename = "OversizeHandling")]
    #[serde(default)]
    pub oversize_handling: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeaderMatchPattern {
    #[serde(rename = "All")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<All>,
    #[serde(rename = "ExcludedHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_headers: Option<Vec<String>>,
    #[serde(rename = "IncludedHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_headers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JA3Fingerprint {
    #[serde(rename = "FallbackBehavior")]
    #[serde(default)]
    pub fallback_behavior: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JA4Fingerprint {
    #[serde(rename = "FallbackBehavior")]
    #[serde(default)]
    pub fallback_behavior: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JsonBody {
    #[serde(rename = "InvalidFallbackBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_fallback_behavior: Option<String>,
    #[serde(rename = "MatchPattern")]
    #[serde(default)]
    pub match_pattern: JsonMatchPattern,
    #[serde(rename = "MatchScope")]
    #[serde(default)]
    pub match_scope: String,
    #[serde(rename = "OversizeHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversize_handling: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JsonMatchPattern {
    #[serde(rename = "All")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<All>,
    #[serde(rename = "IncludedPaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Method {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryString {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingleHeader {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingleQueryArgument {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UriFragment {
    #[serde(rename = "FallbackBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UriPath {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextTransformation {
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeoMatchStatement {
    #[serde(rename = "CountryCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    #[serde(rename = "ForwardedIPConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_i_p_config: Option<ForwardedIPConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPSetReferenceStatement {
    #[serde(rename = "ARN")]
    #[serde(default)]
    pub a_r_n: String,
    #[serde(rename = "IPSetForwardedIPConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_set_forwarded_i_p_config: Option<IPSetForwardedIPConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPSetForwardedIPConfig {
    #[serde(rename = "FallbackBehavior")]
    #[serde(default)]
    pub fallback_behavior: String,
    #[serde(rename = "HeaderName")]
    #[serde(default)]
    pub header_name: String,
    #[serde(rename = "Position")]
    #[serde(default)]
    pub position: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelMatchStatement {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleGroupStatement {
    #[serde(rename = "ExcludedRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
    #[serde(rename = "ManagedRuleGroupConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_group_configs: Option<Vec<ManagedRuleGroupConfig>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RuleActionOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_action_overrides: Option<Vec<RuleActionOverride>>,
    #[serde(rename = "ScopeDownStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_down_statement: Option<Box<Statement>>,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    pub vendor_name: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExcludedRule {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleGroupConfig {
    #[serde(rename = "AWSManagedRulesACFPRuleSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_managed_rules_a_c_f_p_rule_set: Option<AWSManagedRulesACFPRuleSet>,
    #[serde(rename = "AWSManagedRulesATPRuleSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_managed_rules_a_t_p_rule_set: Option<AWSManagedRulesATPRuleSet>,
    #[serde(rename = "AWSManagedRulesAntiDDoSRuleSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_managed_rules_anti_d_do_s_rule_set: Option<AWSManagedRulesAntiDDoSRuleSet>,
    #[serde(rename = "AWSManagedRulesBotControlRuleSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_managed_rules_bot_control_rule_set: Option<AWSManagedRulesBotControlRuleSet>,
    #[serde(rename = "LoginPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_path: Option<String>,
    #[serde(rename = "PasswordField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_field: Option<PasswordField>,
    #[serde(rename = "PayloadType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_type: Option<String>,
    #[serde(rename = "UsernameField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_field: Option<UsernameField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSManagedRulesACFPRuleSet {
    #[serde(rename = "CreationPath")]
    #[serde(default)]
    pub creation_path: String,
    #[serde(rename = "EnableRegexInPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_regex_in_path: Option<bool>,
    #[serde(rename = "RegistrationPagePath")]
    #[serde(default)]
    pub registration_page_path: String,
    #[serde(rename = "RequestInspection")]
    #[serde(default)]
    pub request_inspection: RequestInspectionACFP,
    #[serde(rename = "ResponseInspection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_inspection: Option<ResponseInspection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestInspectionACFP {
    #[serde(rename = "AddressFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_fields: Option<Vec<AddressField>>,
    #[serde(rename = "EmailField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_field: Option<EmailField>,
    #[serde(rename = "PasswordField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_field: Option<PasswordField>,
    #[serde(rename = "PayloadType")]
    #[serde(default)]
    pub payload_type: String,
    #[serde(rename = "PhoneNumberFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_fields: Option<Vec<PhoneNumberField>>,
    #[serde(rename = "UsernameField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_field: Option<UsernameField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddressField {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailField {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PasswordField {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberField {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsernameField {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseInspection {
    #[serde(rename = "BodyContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_contains: Option<ResponseInspectionBodyContains>,
    #[serde(rename = "Header")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<ResponseInspectionHeader>,
    #[serde(rename = "Json")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<ResponseInspectionJson>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ResponseInspectionStatusCode>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseInspectionBodyContains {
    #[serde(rename = "FailureStrings")]
    #[serde(default)]
    pub failure_strings: Vec<String>,
    #[serde(rename = "SuccessStrings")]
    #[serde(default)]
    pub success_strings: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseInspectionHeader {
    #[serde(rename = "FailureValues")]
    #[serde(default)]
    pub failure_values: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SuccessValues")]
    #[serde(default)]
    pub success_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseInspectionJson {
    #[serde(rename = "FailureValues")]
    #[serde(default)]
    pub failure_values: Vec<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "SuccessValues")]
    #[serde(default)]
    pub success_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseInspectionStatusCode {
    #[serde(rename = "FailureCodes")]
    #[serde(default)]
    pub failure_codes: Vec<i32>,
    #[serde(rename = "SuccessCodes")]
    #[serde(default)]
    pub success_codes: Vec<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSManagedRulesATPRuleSet {
    #[serde(rename = "EnableRegexInPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_regex_in_path: Option<bool>,
    #[serde(rename = "LoginPath")]
    #[serde(default)]
    pub login_path: String,
    #[serde(rename = "RequestInspection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_inspection: Option<RequestInspection>,
    #[serde(rename = "ResponseInspection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_inspection: Option<ResponseInspection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestInspection {
    #[serde(rename = "PasswordField")]
    #[serde(default)]
    pub password_field: PasswordField,
    #[serde(rename = "PayloadType")]
    #[serde(default)]
    pub payload_type: String,
    #[serde(rename = "UsernameField")]
    #[serde(default)]
    pub username_field: UsernameField,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSManagedRulesAntiDDoSRuleSet {
    #[serde(rename = "ClientSideActionConfig")]
    #[serde(default)]
    pub client_side_action_config: ClientSideActionConfig,
    #[serde(rename = "SensitivityToBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_to_block: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientSideActionConfig {
    #[serde(rename = "Challenge")]
    #[serde(default)]
    pub challenge: ClientSideAction,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientSideAction {
    #[serde(rename = "ExemptUriRegularExpressions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_uri_regular_expressions: Option<Vec<Regex>>,
    #[serde(rename = "Sensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<String>,
    #[serde(rename = "UsageOfAction")]
    #[serde(default)]
    pub usage_of_action: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Regex {
    #[serde(rename = "RegexString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSManagedRulesBotControlRuleSet {
    #[serde(rename = "EnableMachineLearning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_machine_learning: Option<bool>,
    #[serde(rename = "InspectionLevel")]
    #[serde(default)]
    pub inspection_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleActionOverride {
    #[serde(rename = "ActionToUse")]
    #[serde(default)]
    pub action_to_use: RuleAction,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotStatement {
    #[serde(rename = "Statement")]
    #[serde(default)]
    pub statement: Box<Statement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrStatement {
    #[serde(rename = "Statements")]
    #[serde(default)]
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateBasedStatement {
    #[serde(rename = "AggregateKeyType")]
    #[serde(default)]
    pub aggregate_key_type: String,
    #[serde(rename = "CustomKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_keys: Option<Vec<RateBasedStatementCustomKey>>,
    #[serde(rename = "EvaluationWindowSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_window_sec: Option<i64>,
    #[serde(rename = "ForwardedIPConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_i_p_config: Option<ForwardedIPConfig>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    pub limit: i64,
    #[serde(rename = "ScopeDownStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_down_statement: Option<Box<Statement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateBasedStatementCustomKey {
    #[serde(rename = "ASN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_s_n: Option<RateLimitAsn>,
    #[serde(rename = "Cookie")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<RateLimitCookie>,
    #[serde(rename = "ForwardedIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_i_p: Option<RateLimitForwardedIP>,
    #[serde(rename = "HTTPMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_t_t_p_method: Option<RateLimitHTTPMethod>,
    #[serde(rename = "Header")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<RateLimitHeader>,
    #[serde(rename = "IP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p: Option<RateLimitIP>,
    #[serde(rename = "JA3Fingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_a3_fingerprint: Option<RateLimitJA3Fingerprint>,
    #[serde(rename = "JA4Fingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_a4_fingerprint: Option<RateLimitJA4Fingerprint>,
    #[serde(rename = "LabelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_namespace: Option<RateLimitLabelNamespace>,
    #[serde(rename = "QueryArgument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_argument: Option<RateLimitQueryArgument>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<RateLimitQueryString>,
    #[serde(rename = "UriPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_path: Option<RateLimitUriPath>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitAsn {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitCookie {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitForwardedIP {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitHTTPMethod {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitHeader {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitIP {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitJA3Fingerprint {
    #[serde(rename = "FallbackBehavior")]
    #[serde(default)]
    pub fallback_behavior: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitJA4Fingerprint {
    #[serde(rename = "FallbackBehavior")]
    #[serde(default)]
    pub fallback_behavior: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitLabelNamespace {
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitQueryArgument {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitQueryString {
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateLimitUriPath {
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegexMatchStatement {
    #[serde(rename = "FieldToMatch")]
    #[serde(default)]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "RegexString")]
    #[serde(default)]
    pub regex_string: String,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegexPatternSetReferenceStatement {
    #[serde(rename = "ARN")]
    #[serde(default)]
    pub a_r_n: String,
    #[serde(rename = "FieldToMatch")]
    #[serde(default)]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupReferenceStatement {
    #[serde(rename = "ARN")]
    #[serde(default)]
    pub a_r_n: String,
    #[serde(rename = "ExcludedRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
    #[serde(rename = "RuleActionOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_action_overrides: Option<Vec<RuleActionOverride>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SizeConstraintStatement {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "FieldToMatch")]
    #[serde(default)]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "Size")]
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqliMatchStatement {
    #[serde(rename = "FieldToMatch")]
    #[serde(default)]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "SensitivityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_level: Option<String>,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XssMatchStatement {
    #[serde(rename = "FieldToMatch")]
    #[serde(default)]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "TextTransformations")]
    #[serde(default)]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisibilityConfig {
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(default)]
    pub cloud_watch_metrics_enabled: bool,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "SampledRequestsEnabled")]
    #[serde(default)]
    pub sampled_requests_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckCapacityResponse {
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAPIKeyRequest {
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "TokenDomains")]
    #[serde(default)]
    pub token_domains: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAPIKeyResponse {
    #[serde(rename = "APIKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_i_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIPSetRequest {
    #[serde(rename = "Addresses")]
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IPAddressVersion")]
    #[serde(default)]
    pub i_p_address_version: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIPSetResponse {
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<IPSetSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPSetSummary {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRegexPatternSetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RegularExpressionList")]
    #[serde(default)]
    pub regular_expression_list: Vec<Regex>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRegexPatternSetResponse {
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<RegexPatternSetSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegexPatternSetSummary {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleGroupRequest {
    #[serde(rename = "Capacity")]
    #[serde(default)]
    pub capacity: i64,
    #[serde(rename = "CustomResponseBodies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_bodies: Option<std::collections::HashMap<String, CustomResponseBody>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    pub visibility_config: VisibilityConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomResponseBody {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleGroupResponse {
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<RuleGroupSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSummary {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebACLRequest {
    #[serde(rename = "ApplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_config: Option<ApplicationConfig>,
    #[serde(rename = "AssociationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_config: Option<AssociationConfig>,
    #[serde(rename = "CaptchaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_config: Option<CaptchaConfig>,
    #[serde(rename = "ChallengeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_config: Option<ChallengeConfig>,
    #[serde(rename = "CustomResponseBodies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_bodies: Option<std::collections::HashMap<String, CustomResponseBody>>,
    #[serde(rename = "DataProtectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_config: Option<DataProtectionConfig>,
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    pub default_action: DefaultAction,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OnSourceDDoSProtectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_source_d_do_s_protection_config: Option<OnSourceDDoSProtectionConfig>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TokenDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_domains: Option<Vec<String>>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    pub visibility_config: VisibilityConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationConfig {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ApplicationAttribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationAttribute {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationConfig {
    #[serde(rename = "RequestBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body:
        Option<std::collections::HashMap<String, RequestBodyAssociatedResourceTypeConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestBodyAssociatedResourceTypeConfig {
    #[serde(rename = "DefaultSizeInspectionLimit")]
    #[serde(default)]
    pub default_size_inspection_limit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProtectionConfig {
    #[serde(rename = "DataProtections")]
    #[serde(default)]
    pub data_protections: Vec<DataProtection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProtection {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "ExcludeRateBasedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_rate_based_details: Option<bool>,
    #[serde(rename = "ExcludeRuleMatchDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_rule_match_details: Option<bool>,
    #[serde(rename = "Field")]
    #[serde(default)]
    pub field: FieldToProtect,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldToProtect {
    #[serde(rename = "FieldKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_keys: Option<Vec<String>>,
    #[serde(rename = "FieldType")]
    #[serde(default)]
    pub field_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultAction {
    #[serde(rename = "Allow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<AllowAction>,
    #[serde(rename = "Block")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnSourceDDoSProtectionConfig {
    #[serde(rename = "ALBLowReputationMode")]
    #[serde(default)]
    pub a_l_b_low_reputation_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebACLResponse {
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<WebACLSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebACLSummary {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAPIKeyRequest {
    #[serde(rename = "APIKey")]
    #[serde(default)]
    pub a_p_i_key: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAPIKeyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallManagerRuleGroupsRequest {
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    pub web_a_c_l_arn: String,
    #[serde(rename = "WebACLLockToken")]
    #[serde(default)]
    pub web_a_c_l_lock_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallManagerRuleGroupsResponse {
    #[serde(rename = "NextWebACLLockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_web_a_c_l_lock_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIPSetRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIPSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoggingConfigurationRequest {
    #[serde(rename = "LogScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_scope: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoggingConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionPolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegexPatternSetRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegexPatternSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleGroupRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebACLRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebACLResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAllManagedProductsRequest {
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAllManagedProductsResponse {
    #[serde(rename = "ManagedProducts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_products: Option<Vec<ManagedProductDescriptor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedProductDescriptor {
    #[serde(rename = "IsAdvancedManagedRuleSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced_managed_rule_set: Option<bool>,
    #[serde(rename = "IsVersioningSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_versioning_supported: Option<bool>,
    #[serde(rename = "ManagedRuleSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_set_name: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProductLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_link: Option<String>,
    #[serde(rename = "ProductTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_title: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedProductsByVendorRequest {
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    pub vendor_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedProductsByVendorResponse {
    #[serde(rename = "ManagedProducts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_products: Option<Vec<ManagedProductDescriptor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedRuleGroupRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    pub vendor_name: String,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedRuleGroupResponse {
    #[serde(rename = "AvailableLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_labels: Option<Vec<LabelSummary>>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "ConsumedLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_labels: Option<Vec<LabelSummary>>,
    #[serde(rename = "LabelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_namespace: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RuleSummary>>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelSummary {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleSummary {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateWebACLRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateWebACLResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateMobileSdkReleaseUrlRequest {
    #[serde(rename = "Platform")]
    #[serde(default)]
    pub platform: String,
    #[serde(rename = "ReleaseVersion")]
    #[serde(default)]
    pub release_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateMobileSdkReleaseUrlResponse {
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDecryptedAPIKeyRequest {
    #[serde(rename = "APIKey")]
    #[serde(default)]
    pub a_p_i_key: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDecryptedAPIKeyResponse {
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "TokenDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_domains: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIPSetRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIPSetResponse {
    #[serde(rename = "IPSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_set: Option<IPSet>,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPSet {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IPAddressVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggingConfigurationRequest {
    #[serde(rename = "LogScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_scope: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggingConfigurationResponse {
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfiguration {
    #[serde(rename = "LogDestinationConfigs")]
    #[serde(default)]
    pub log_destination_configs: Vec<String>,
    #[serde(rename = "LogScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_scope: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "LoggingFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_filter: Option<LoggingFilter>,
    #[serde(rename = "ManagedByFirewallManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by_firewall_manager: Option<bool>,
    #[serde(rename = "RedactedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_fields: Option<Vec<FieldToMatch>>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingFilter {
    #[serde(rename = "DefaultBehavior")]
    #[serde(default)]
    pub default_behavior: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Behavior")]
    #[serde(default)]
    pub behavior: String,
    #[serde(rename = "Conditions")]
    #[serde(default)]
    pub conditions: Vec<Condition>,
    #[serde(rename = "Requirement")]
    #[serde(default)]
    pub requirement: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "ActionCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_condition: Option<ActionCondition>,
    #[serde(rename = "LabelNameCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_name_condition: Option<LabelNameCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionCondition {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelNameCondition {
    #[serde(rename = "LabelName")]
    #[serde(default)]
    pub label_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedRuleSetRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedRuleSetResponse {
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "ManagedRuleSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_set: Option<ManagedRuleSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleSet {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LabelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_namespace: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublishedVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_versions: Option<std::collections::HashMap<String, ManagedRuleSetVersion>>,
    #[serde(rename = "RecommendedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleSetVersion {
    #[serde(rename = "AssociatedRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_rule_group_arn: Option<String>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "ExpiryTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_timestamp: Option<f64>,
    #[serde(rename = "ForecastedLifetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_lifetime: Option<i32>,
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[serde(rename = "PublishTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMobileSdkReleaseRequest {
    #[serde(rename = "Platform")]
    #[serde(default)]
    pub platform: String,
    #[serde(rename = "ReleaseVersion")]
    #[serde(default)]
    pub release_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMobileSdkReleaseResponse {
    #[serde(rename = "MobileSdkRelease")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_sdk_release: Option<MobileSdkRelease>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MobileSdkRelease {
    #[serde(rename = "ReleaseNotes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_notes: Option<String>,
    #[serde(rename = "ReleaseVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPermissionPolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPermissionPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRateBasedStatementManagedKeysRequest {
    #[serde(rename = "RuleGroupRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_rule_name: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "WebACLId")]
    #[serde(default)]
    pub web_a_c_l_id: String,
    #[serde(rename = "WebACLName")]
    #[serde(default)]
    pub web_a_c_l_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRateBasedStatementManagedKeysResponse {
    #[serde(rename = "ManagedKeysIPV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_keys_i_p_v4: Option<RateBasedStatementManagedKeysIPSet>,
    #[serde(rename = "ManagedKeysIPV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_keys_i_p_v6: Option<RateBasedStatementManagedKeysIPSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateBasedStatementManagedKeysIPSet {
    #[serde(rename = "Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(rename = "IPAddressVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegexPatternSetRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegexPatternSetResponse {
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "RegexPatternSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set: Option<RegexPatternSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegexPatternSet {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RegularExpressionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_expression_list: Option<Vec<Regex>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRuleGroupRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRuleGroupResponse {
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "RuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AvailableLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_labels: Option<Vec<LabelSummary>>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "ConsumedLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_labels: Option<Vec<LabelSummary>>,
    #[serde(rename = "CustomResponseBodies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_bodies: Option<std::collections::HashMap<String, CustomResponseBody>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LabelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_namespace: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_config: Option<VisibilityConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSampledRequestsRequest {
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    pub max_items: i64,
    #[serde(rename = "RuleMetricName")]
    #[serde(default)]
    pub rule_metric_name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "TimeWindow")]
    #[serde(default)]
    pub time_window: TimeWindow,
    #[serde(rename = "WebAclArn")]
    #[serde(default)]
    pub web_acl_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeWindow {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSampledRequestsResponse {
    #[serde(rename = "PopulationSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_size: Option<i64>,
    #[serde(rename = "SampledRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_requests: Option<Vec<SampledHTTPRequest>>,
    #[serde(rename = "TimeWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<TimeWindow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SampledHTTPRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "CaptchaResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_response: Option<CaptchaResponse>,
    #[serde(rename = "ChallengeResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_response: Option<ChallengeResponse>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "OverriddenAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden_action: Option<String>,
    #[serde(rename = "Request")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<HTTPRequest>,
    #[serde(rename = "RequestHeadersInserted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers_inserted: Option<Vec<HTTPHeader>>,
    #[serde(rename = "ResponseCodeSent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code_sent: Option<i32>,
    #[serde(rename = "RuleNameWithinRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name_within_rule_group: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptchaResponse {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
    #[serde(rename = "SolveTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solve_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChallengeResponse {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
    #[serde(rename = "SolveTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solve_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HTTPRequest {
    #[serde(rename = "ClientIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_i_p: Option<String>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "HTTPVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_t_t_p_version: Option<String>,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HTTPHeader>>,
    #[serde(rename = "Method")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "URI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_i: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HTTPHeader {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTopPathStatisticsByTrafficRequest {
    #[serde(rename = "BotCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_category: Option<String>,
    #[serde(rename = "BotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "BotOrganization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_organization: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    pub limit: i32,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "NumberOfTopTrafficBotsPerPath")]
    #[serde(default)]
    pub number_of_top_traffic_bots_per_path: i32,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "TimeWindow")]
    #[serde(default)]
    pub time_window: TimeWindow,
    #[serde(rename = "UriPathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_path_prefix: Option<String>,
    #[serde(rename = "WebAclArn")]
    #[serde(default)]
    pub web_acl_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTopPathStatisticsByTrafficResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "PathStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_statistics: Option<Vec<PathStatistics>>,
    #[serde(rename = "TopCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_categories: Option<Vec<PathStatistics>>,
    #[serde(rename = "TotalRequestCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_request_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathStatistics {
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Percentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[serde(rename = "RequestCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_count: Option<i64>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<FilterSource>,
    #[serde(rename = "TopBots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_bots: Option<Vec<BotStatistics>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterSource {
    #[serde(rename = "BotCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_category: Option<String>,
    #[serde(rename = "BotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "BotOrganization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_organization: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotStatistics {
    #[serde(rename = "BotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "Percentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[serde(rename = "RequestCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWebACLForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWebACLForResourceResponse {
    #[serde(rename = "WebACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l: Option<WebACL>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebACL {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "ApplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_config: Option<ApplicationConfig>,
    #[serde(rename = "AssociationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_config: Option<AssociationConfig>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "CaptchaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_config: Option<CaptchaConfig>,
    #[serde(rename = "ChallengeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_config: Option<ChallengeConfig>,
    #[serde(rename = "CustomResponseBodies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_bodies: Option<std::collections::HashMap<String, CustomResponseBody>>,
    #[serde(rename = "DataProtectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_config: Option<DataProtectionConfig>,
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<DefaultAction>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LabelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_namespace: Option<String>,
    #[serde(rename = "ManagedByFirewallManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by_firewall_manager: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OnSourceDDoSProtectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_source_d_do_s_protection_config: Option<OnSourceDDoSProtectionConfig>,
    #[serde(rename = "PostProcessFirewallManagerRuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_process_firewall_manager_rule_groups: Option<Vec<FirewallManagerRuleGroup>>,
    #[serde(rename = "PreProcessFirewallManagerRuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_process_firewall_manager_rule_groups: Option<Vec<FirewallManagerRuleGroup>>,
    #[serde(rename = "RetrofittedByFirewallManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrofitted_by_firewall_manager: Option<bool>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "TokenDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_domains: Option<Vec<String>>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_config: Option<VisibilityConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallManagerRuleGroup {
    #[serde(rename = "FirewallManagerStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_manager_statement: Option<FirewallManagerStatement>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OverrideAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<OverrideAction>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_config: Option<VisibilityConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallManagerStatement {
    #[serde(rename = "ManagedRuleGroupStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_group_statement: Option<ManagedRuleGroupStatement>,
    #[serde(rename = "RuleGroupReferenceStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_reference_statement: Option<RuleGroupReferenceStatement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWebACLRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWebACLResponse {
    #[serde(rename = "ApplicationIntegrationURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_integration_u_r_l: Option<String>,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "WebACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l: Option<WebACL>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAPIKeysRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAPIKeysResponse {
    #[serde(rename = "APIKeySummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_i_key_summaries: Option<Vec<APIKeySummary>>,
    #[serde(rename = "ApplicationIntegrationURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_integration_u_r_l: Option<String>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APIKeySummary {
    #[serde(rename = "APIKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_i_key: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "TokenDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_domains: Option<Vec<String>>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableManagedRuleGroupVersionsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    pub vendor_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableManagedRuleGroupVersionsResponse {
    #[serde(rename = "CurrentDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_default_version: Option<String>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<ManagedRuleGroupVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleGroupVersion {
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableManagedRuleGroupsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableManagedRuleGroupsResponse {
    #[serde(rename = "ManagedRuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_groups: Option<Vec<ManagedRuleGroupSummary>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleGroupSummary {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(rename = "VersioningSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_supported: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIPSetsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIPSetsResponse {
    #[serde(rename = "IPSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_sets: Option<Vec<IPSetSummary>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLoggingConfigurationsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "LogScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_scope: Option<String>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLoggingConfigurationsResponse {
    #[serde(rename = "LoggingConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configurations: Option<Vec<LoggingConfiguration>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedRuleSetsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedRuleSetsResponse {
    #[serde(rename = "ManagedRuleSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_sets: Option<Vec<ManagedRuleSetSummary>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleSetSummary {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LabelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_namespace: Option<String>,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMobileSdkReleasesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    pub platform: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMobileSdkReleasesResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "ReleaseSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_summaries: Option<Vec<ReleaseSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReleaseSummary {
    #[serde(rename = "ReleaseVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRegexPatternSetsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRegexPatternSetsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "RegexPatternSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_sets: Option<Vec<RegexPatternSetSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesForWebACLRequest {
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    pub web_a_c_l_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesForWebACLResponse {
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleGroupsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleGroupsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "RuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<RuleGroupSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "TagInfoForResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_info_for_resource: Option<TagInfoForResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagInfoForResource {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWebACLsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWebACLsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "WebACLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_ls: Option<Vec<WebACLSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLoggingConfigurationRequest {
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    pub logging_configuration: LoggingConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLoggingConfigurationResponse {
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutManagedRuleSetVersionsRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecommendedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_version: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "VersionsToPublish")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions_to_publish: Option<std::collections::HashMap<String, VersionToPublish>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VersionToPublish {
    #[serde(rename = "AssociatedRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_rule_group_arn: Option<String>,
    #[serde(rename = "ForecastedLifetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_lifetime: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutManagedRuleSetVersionsResponse {
    #[serde(rename = "NextLockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPermissionPolicyRequest {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPermissionPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIPSetRequest {
    #[serde(rename = "Addresses")]
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIPSetResponse {
    #[serde(rename = "NextLockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateManagedRuleSetVersionExpiryDateRequest {
    #[serde(rename = "ExpiryTimestamp")]
    #[serde(default)]
    pub expiry_timestamp: f64,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "VersionToExpire")]
    #[serde(default)]
    pub version_to_expire: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateManagedRuleSetVersionExpiryDateResponse {
    #[serde(rename = "ExpiringVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiring_version: Option<String>,
    #[serde(rename = "ExpiryTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_timestamp: Option<f64>,
    #[serde(rename = "NextLockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRegexPatternSetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RegularExpressionList")]
    #[serde(default)]
    pub regular_expression_list: Vec<Regex>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRegexPatternSetResponse {
    #[serde(rename = "NextLockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRuleGroupRequest {
    #[serde(rename = "CustomResponseBodies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_bodies: Option<std::collections::HashMap<String, CustomResponseBody>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    pub visibility_config: VisibilityConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRuleGroupResponse {
    #[serde(rename = "NextLockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebACLRequest {
    #[serde(rename = "ApplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_config: Option<ApplicationConfig>,
    #[serde(rename = "AssociationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_config: Option<AssociationConfig>,
    #[serde(rename = "CaptchaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_config: Option<CaptchaConfig>,
    #[serde(rename = "ChallengeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_config: Option<ChallengeConfig>,
    #[serde(rename = "CustomResponseBodies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_bodies: Option<std::collections::HashMap<String, CustomResponseBody>>,
    #[serde(rename = "DataProtectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_config: Option<DataProtectionConfig>,
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    pub default_action: DefaultAction,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LockToken")]
    #[serde(default)]
    pub lock_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OnSourceDDoSProtectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_source_d_do_s_protection_config: Option<OnSourceDDoSProtectionConfig>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "TokenDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_domains: Option<Vec<String>>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    pub visibility_config: VisibilityConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebACLResponse {
    #[serde(rename = "NextLockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}
