use std::net::IpAddr;

use crate::types::{ConditionBlock, SimulationContext};
use crate::wildcard::glob_match;

/// Evaluate a condition block against the simulation context.
///
/// **Phase 1 stub**: always returns `(true, vec![])` — i.e. conditions are
/// treated as satisfied with no missing context keys. The IAM `Simulate*`
/// handlers depend on this; real operator dispatch lives in
/// [`evaluate_condition_block`].
pub fn evaluate_condition(
    _condition: &ConditionBlock,
    _context: &SimulationContext,
) -> (bool, Vec<String>) {
    (true, vec![])
}

/// Result of [`evaluate_condition_block`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionOutcome {
    /// Whether every operator / key pair of the block held.
    pub matched: bool,
    /// Condition keys ( as spelled in the policy ) that the block referenced
    /// and the context did not supply. `Null` checks are not reported.
    pub missing_keys: Vec<String>,
}

/// Evaluate a `Condition` block with IAM's operator semantics.
///
/// * Operators are ANDed, the keys under one operator are ANDed, and the
///   values listed for one key are ORed ( for a negated operator: the context
///   value must match none of them ).
/// * Supported operators: `String[Not]Equals[IgnoreCase]`, `String[Not]Like`,
///   `Numeric[Not]Equals`, `Numeric{LessThan,GreaterThan}[Equals]`,
///   `Date[Not]Equals`, `Date{LessThan,GreaterThan}[Equals]`, `Bool`,
///   `BinaryEquals`, `[Not]IpAddress`, `Arn[Not]Equals`, `Arn[Not]Like` and
///   `Null`, each optionally prefixed with `ForAllValues:` / `ForAnyValue:`
///   and ( except `Null` ) suffixed with `IfExists`. Operator names are
///   matched case-insensitively, as are condition key names
///   ( [`SimulationContext::context_keys`] is keyed in lower case ).
/// * A key that is **absent from the context** follows IAM: the test fails,
///   unless the operator is negated ( `...Not...` ), carries `IfExists`, is
///   qualified with `ForAllValues:`, or is `Null: true` — those hold.
/// * An operator name that is not recognised never holds, so the statement
///   carrying it does not apply.
/// * Policy variables ( `${aws:username}` ) are not substituted.
pub fn evaluate_condition_block(
    condition: &ConditionBlock,
    context: &SimulationContext,
) -> ConditionOutcome {
    let mut matched = true;
    let mut missing_keys = Vec::new();

    for (operator_name, keys) in condition {
        let operator = Operator::parse(operator_name);
        for (key, policy_values) in keys {
            let context_values = context
                .context_keys
                .get(&key.to_ascii_lowercase())
                .map(Vec::as_slice)
                .filter(|values| !values.is_empty());
            let Some(operator) = operator else {
                matched = false;
                continue;
            };
            if context_values.is_none() && operator.comparator != Comparator::Null {
                missing_keys.push(key.clone());
            }
            if !operator.holds(policy_values.as_slice(), context_values) {
                matched = false;
            }
        }
    }

    missing_keys.sort();
    missing_keys.dedup();
    ConditionOutcome {
        matched,
        missing_keys,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Quantifier {
    /// No set qualifier ( single-valued key ).
    Single,
    ForAllValues,
    ForAnyValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ordering {
    Equals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
}

impl Ordering {
    fn test<T: PartialOrd>(self, left: T, right: T) -> bool {
        match self {
            Ordering::Equals => left == right,
            Ordering::LessThan => left < right,
            Ordering::LessThanEquals => left <= right,
            Ordering::GreaterThan => left > right,
            Ordering::GreaterThanEquals => left >= right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparator {
    StringEquals,
    StringEqualsIgnoreCase,
    StringLike,
    Numeric(Ordering),
    Date(Ordering),
    Bool,
    BinaryEquals,
    IpAddress,
    Arn,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Operator {
    quantifier: Quantifier,
    comparator: Comparator,
    negated: bool,
    if_exists: bool,
}

impl Operator {
    fn parse(name: &str) -> Option<Self> {
        let lower = name.trim().to_ascii_lowercase();
        let (quantifier, rest) = if let Some(rest) = lower.strip_prefix("forallvalues:") {
            (Quantifier::ForAllValues, rest)
        } else if let Some(rest) = lower.strip_prefix("foranyvalue:") {
            (Quantifier::ForAnyValue, rest)
        } else {
            (Quantifier::Single, lower.as_str())
        };
        let (base, if_exists) = match rest.strip_suffix("ifexists") {
            Some(base) => (base, true),
            None => (rest, false),
        };
        let (comparator, negated) = match base {
            "stringequals" => (Comparator::StringEquals, false),
            "stringnotequals" => (Comparator::StringEquals, true),
            "stringequalsignorecase" => (Comparator::StringEqualsIgnoreCase, false),
            "stringnotequalsignorecase" => (Comparator::StringEqualsIgnoreCase, true),
            "stringlike" => (Comparator::StringLike, false),
            "stringnotlike" => (Comparator::StringLike, true),
            "numericequals" => (Comparator::Numeric(Ordering::Equals), false),
            "numericnotequals" => (Comparator::Numeric(Ordering::Equals), true),
            "numericlessthan" => (Comparator::Numeric(Ordering::LessThan), false),
            "numericlessthanequals" => (Comparator::Numeric(Ordering::LessThanEquals), false),
            "numericgreaterthan" => (Comparator::Numeric(Ordering::GreaterThan), false),
            "numericgreaterthanequals" => (Comparator::Numeric(Ordering::GreaterThanEquals), false),
            "dateequals" => (Comparator::Date(Ordering::Equals), false),
            "datenotequals" => (Comparator::Date(Ordering::Equals), true),
            "datelessthan" => (Comparator::Date(Ordering::LessThan), false),
            "datelessthanequals" => (Comparator::Date(Ordering::LessThanEquals), false),
            "dategreaterthan" => (Comparator::Date(Ordering::GreaterThan), false),
            "dategreaterthanequals" => (Comparator::Date(Ordering::GreaterThanEquals), false),
            "bool" => (Comparator::Bool, false),
            "binaryequals" => (Comparator::BinaryEquals, false),
            "ipaddress" => (Comparator::IpAddress, false),
            "notipaddress" => (Comparator::IpAddress, true),
            "arnequals" | "arnlike" => (Comparator::Arn, false),
            "arnnotequals" | "arnnotlike" => (Comparator::Arn, true),
            "null" if !if_exists => (Comparator::Null, false),
            _ => return None,
        };
        Some(Self {
            quantifier,
            comparator,
            negated,
            if_exists,
        })
    }

    fn holds(self, policy_values: &[String], context_values: Option<&[String]>) -> bool {
        if self.comparator == Comparator::Null {
            // `true`: the key must be absent. `false`: the key must be present.
            let present = context_values.is_some();
            return policy_values
                .iter()
                .any(|v| v.trim().eq_ignore_ascii_case("true") != present);
        }

        let Some(context_values) = context_values else {
            return self.if_exists || self.negated || self.quantifier == Quantifier::ForAllValues;
        };

        // Per context value: does it satisfy the ( possibly negated ) test
        // against the policy's value list?
        let satisfied = |context_value: &String| {
            let any = policy_values
                .iter()
                .any(|policy_value| self.comparator.matches(policy_value, context_value));
            any != self.negated
        };
        match self.quantifier {
            Quantifier::ForAllValues => context_values.iter().all(satisfied),
            Quantifier::ForAnyValue => context_values.iter().any(satisfied),
            // An unqualified operator is meant for single-valued keys; if the
            // context nevertheless holds several values, a positive test needs
            // one of them to match and a negated test needs none to.
            Quantifier::Single if self.negated => context_values.iter().all(satisfied),
            Quantifier::Single => context_values.iter().any(satisfied),
        }
    }
}

impl Comparator {
    /// The positive form of the comparison ( negation is applied by the caller ).
    fn matches(self, policy_value: &str, context_value: &str) -> bool {
        match self {
            Comparator::StringEquals | Comparator::BinaryEquals => policy_value == context_value,
            Comparator::StringEqualsIgnoreCase => {
                policy_value.to_lowercase() == context_value.to_lowercase()
            }
            Comparator::StringLike => glob_match(policy_value, context_value),
            Comparator::Numeric(ordering) => {
                match (parse_number(context_value), parse_number(policy_value)) {
                    (Some(left), Some(right)) => ordering.test(left, right),
                    _ => false,
                }
            }
            Comparator::Date(ordering) => {
                match (
                    parse_date_millis(context_value),
                    parse_date_millis(policy_value),
                ) {
                    (Some(left), Some(right)) => ordering.test(left, right),
                    _ => false,
                }
            }
            Comparator::Bool => policy_value
                .trim()
                .eq_ignore_ascii_case(context_value.trim()),
            Comparator::IpAddress => ip_in_cidr(context_value, policy_value),
            Comparator::Arn => arn_like(policy_value, context_value),
            Comparator::Null => false,
        }
    }
}

fn parse_number(value: &str) -> Option<f64> {
    value.trim().parse::<f64>().ok().filter(|n| n.is_finite())
}

/// ARN matching: each of the six colon-delimited components is glob-matched
/// separately, so a wildcard cannot span a `:` ( the resource component, which
/// may itself contain colons, is matched as one piece ).
fn arn_like(pattern: &str, value: &str) -> bool {
    let pattern_parts: Vec<&str> = pattern.splitn(6, ':').collect();
    let value_parts: Vec<&str> = value.splitn(6, ':').collect();
    if pattern_parts.len() != 6 || value_parts.len() != 6 {
        return false;
    }
    pattern_parts
        .iter()
        .zip(&value_parts)
        .all(|(p, v)| glob_match(p, v))
}

/// Whether `address` lies inside `cidr` ( `a.b.c.d/len`, an IPv6 prefix, or a
/// bare address meaning a full-length prefix ).
fn ip_in_cidr(address: &str, cidr: &str) -> bool {
    let Ok(address) = address.trim().parse::<IpAddr>() else {
        return false;
    };
    let cidr = cidr.trim();
    let (network, prefix_len) = match cidr.split_once('/') {
        Some((network, len)) => (network, len.parse::<u32>().ok()),
        None => (cidr, None),
    };
    let Ok(network) = network.parse::<IpAddr>() else {
        return false;
    };
    let (address, network, width) = match (address, network) {
        (IpAddr::V4(a), IpAddr::V4(n)) => (u128::from(u32::from(a)), u128::from(u32::from(n)), 32),
        (IpAddr::V6(a), IpAddr::V6(n)) => (u128::from(a), u128::from(n), 128),
        _ => return false,
    };
    let prefix_len = match (cidr.contains('/'), prefix_len) {
        (false, _) => width,
        (true, Some(len)) if len <= width => len,
        _ => return false,
    };
    if prefix_len == 0 {
        return true;
    }
    let shift = width - prefix_len;
    (address >> shift) == (network >> shift)
}

/// Parse an IAM date value into milliseconds since the Unix epoch.
///
/// Accepts epoch seconds ( `1700000000` ) and the ISO 8601 forms
/// `YYYY-MM-DD` and `YYYY-MM-DDThh:mm[:ss[.fff]]` followed by `Z`, a
/// `±hh:mm` / `±hhmm` offset, or nothing ( taken as UTC ).
fn parse_date_millis(value: &str) -> Option<i64> {
    let value = value.trim();
    if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) {
        return value.parse::<i64>().ok()?.checked_mul(1000);
    }

    let (date, time) = match value.split_once(['T', 't', ' ']) {
        Some((date, time)) => (date, Some(time)),
        None => (value, None),
    };
    let mut date_parts = date.split('-');
    let year: i64 = date_parts.next()?.parse().ok()?;
    let month: i64 = date_parts.next()?.parse().ok()?;
    let day: i64 = date_parts.next()?.parse().ok()?;
    if date_parts.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let mut millis = days_from_civil(year, month, day).checked_mul(86_400_000)?;

    if let Some(time) = time {
        let (clock, offset_minutes) = split_utc_offset(time)?;
        let mut clock_parts = clock.split(':');
        let hours: i64 = clock_parts.next()?.parse().ok()?;
        let minutes: i64 = clock_parts.next()?.parse().ok()?;
        let seconds: f64 = match clock_parts.next() {
            Some(seconds) => seconds.parse().ok()?,
            None => 0.0,
        };
        if clock_parts.next().is_some()
            || !(0..24).contains(&hours)
            || !(0..60).contains(&minutes)
            || !(0.0..61.0).contains(&seconds)
        {
            return None;
        }
        millis += hours * 3_600_000 + minutes * 60_000 + (seconds * 1000.0).round() as i64;
        millis -= offset_minutes * 60_000;
    }
    Some(millis)
}

/// Split `hh:mm:ss<zone>` into the clock part and the zone offset in minutes.
fn split_utc_offset(time: &str) -> Option<(&str, i64)> {
    if let Some(clock) = time.strip_suffix(['Z', 'z']) {
        return Some((clock, 0));
    }
    let Some(sign_pos) = time.rfind(['+', '-']) else {
        return Some((time, 0));
    };
    let (clock, offset) = time.split_at(sign_pos);
    let sign = if offset.starts_with('-') { -1 } else { 1 };
    let digits: String = offset[1..].chars().filter(|c| *c != ':').collect();
    if digits.len() != 4 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let hours: i64 = digits[..2].parse().ok()?;
    let minutes: i64 = digits[2..].parse().ok()?;
    Some((clock, sign * (hours * 60 + minutes)))
}

/// Days since 1970-01-01 for a proleptic Gregorian date.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year.rem_euclid(400);
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn stub_always_true() {
        let cond: ConditionBlock = HashMap::new();
        let ctx = SimulationContext::default();
        let (matched, missing) = evaluate_condition(&cond, &ctx);
        assert!(matched);
        assert!(missing.is_empty());
    }

    #[test]
    fn stub_ignores_unsatisfiable_conditions() {
        let cond: ConditionBlock =
            serde_json::from_str(r#"{"StringEquals": {"aws:RequestedRegion": "nowhere"}}"#)
                .unwrap();
        let (matched, missing) = evaluate_condition(&cond, &SimulationContext::default());
        assert!(matched);
        assert!(missing.is_empty());
    }

    fn ctx(pairs: &[(&str, &[&str])]) -> SimulationContext {
        let mut ctx = SimulationContext::default();
        for (key, values) in pairs {
            ctx.context_keys.insert(
                (*key).to_owned(),
                values.iter().map(|v| (*v).to_owned()).collect(),
            );
        }
        ctx
    }

    fn holds(condition: &str, ctx: &SimulationContext) -> bool {
        let cond: ConditionBlock = serde_json::from_str(condition).unwrap();
        evaluate_condition_block(&cond, ctx).matched
    }

    #[test]
    fn empty_block_holds() {
        assert!(holds("{}", &SimulationContext::default()));
    }

    #[test]
    fn string_operators() {
        let c = ctx(&[("s3:prefix", &["Home/alice/"])]);
        assert!(holds(
            r#"{"StringEquals": {"s3:prefix": "Home/alice/"}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"StringEquals": {"s3:prefix": "home/alice/"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"StringEqualsIgnoreCase": {"s3:prefix": "home/ALICE/"}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"StringNotEqualsIgnoreCase": {"s3:prefix": "home/ALICE/"}}"#,
            &c
        ));
        assert!(holds(r#"{"StringLike": {"s3:prefix": "Home/*"}}"#, &c));
        assert!(holds(r#"{"StringLike": {"s3:prefix": "Home/alic?/"}}"#, &c));
        assert!(!holds(r#"{"StringLike": {"s3:prefix": "home/*"}}"#, &c));
        assert!(holds(r#"{"StringNotLike": {"s3:prefix": "public/*"}}"#, &c));
        assert!(!holds(r#"{"StringNotLike": {"s3:prefix": "Home/*"}}"#, &c));
        // StringEquals does not treat `*` as a wildcard.
        assert!(!holds(r#"{"StringEquals": {"s3:prefix": "Home/*"}}"#, &c));
    }

    #[test]
    fn policy_values_are_ored_and_negations_need_none() {
        let c = ctx(&[("s3:x-amz-acl", &["public-read"])]);
        assert!(holds(
            r#"{"StringEquals": {"s3:x-amz-acl": ["private", "public-read"]}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"StringNotEquals": {"s3:x-amz-acl": ["private", "public-read"]}}"#,
            &c
        ));
        assert!(holds(
            r#"{"StringNotEquals": {"s3:x-amz-acl": ["private", "bucket-owner-full-control"]}}"#,
            &c
        ));
    }

    #[test]
    fn operators_and_keys_are_anded() {
        let c = ctx(&[("s3:prefix", &["a/"]), ("s3:delimiter", &["/"])]);
        assert!(holds(
            r#"{"StringEquals": {"s3:prefix": "a/", "s3:delimiter": "/"}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"StringEquals": {"s3:prefix": "a/", "s3:delimiter": "|"}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"StringEquals": {"s3:prefix": "a/"}, "StringNotEquals": {"s3:delimiter": "/"}}"#,
            &c
        ));
    }

    #[test]
    fn key_and_operator_names_are_case_insensitive() {
        let c = ctx(&[("aws:securetransport", &["false"])]);
        assert!(holds(r#"{"Bool": {"aws:SecureTransport": "false"}}"#, &c));
        assert!(holds(r#"{"bool": {"AWS:SECURETRANSPORT": "FALSE"}}"#, &c));
        // JSON booleans are accepted as values.
        assert!(holds(r#"{"Bool": {"aws:SecureTransport": false}}"#, &c));
        assert!(!holds(r#"{"Bool": {"aws:SecureTransport": true}}"#, &c));
    }

    #[test]
    fn numeric_operators() {
        let c = ctx(&[("s3:max-keys", &["10"])]);
        assert!(holds(r#"{"NumericEquals": {"s3:max-keys": "10"}}"#, &c));
        assert!(holds(r#"{"NumericEquals": {"s3:max-keys": 10}}"#, &c));
        assert!(holds(r#"{"NumericEquals": {"s3:max-keys": "10.0"}}"#, &c));
        assert!(!holds(r#"{"NumericNotEquals": {"s3:max-keys": "10"}}"#, &c));
        assert!(holds(r#"{"NumericLessThan": {"s3:max-keys": "11"}}"#, &c));
        assert!(!holds(r#"{"NumericLessThan": {"s3:max-keys": "10"}}"#, &c));
        assert!(holds(
            r#"{"NumericLessThanEquals": {"s3:max-keys": "10"}}"#,
            &c
        ));
        assert!(holds(r#"{"NumericGreaterThan": {"s3:max-keys": "9"}}"#, &c));
        assert!(!holds(
            r#"{"NumericGreaterThan": {"s3:max-keys": "10"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"NumericGreaterThanEquals": {"s3:max-keys": "10"}}"#,
            &c
        ));
        // A value that is not a number never compares.
        assert!(!holds(r#"{"NumericEquals": {"s3:max-keys": "ten"}}"#, &c));
    }

    #[test]
    fn date_operators() {
        let c = ctx(&[("aws:currenttime", &["2026-03-01T12:00:00Z"])]);
        assert!(holds(
            r#"{"DateGreaterThan": {"aws:CurrentTime": "2026-01-01T00:00:00Z"}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"DateLessThan": {"aws:CurrentTime": "2026-01-01T00:00:00Z"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"DateEquals": {"aws:CurrentTime": "2026-03-01T13:00:00+01:00"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"DateNotEquals": {"aws:CurrentTime": "2026-03-01"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"DateLessThanEquals": {"aws:CurrentTime": "2026-03-01T12:00:00.000Z"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"DateGreaterThanEquals": {"aws:CurrentTime": "1700000000"}}"#,
            &c
        ));
        assert_eq!(parse_date_millis("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(parse_date_millis("2000-03-01"), Some(951_868_800_000));
        assert_eq!(parse_date_millis("not a date"), None);
    }

    #[test]
    fn ip_address_operators() {
        let c = ctx(&[("aws:sourceip", &["203.0.113.7"])]);
        assert!(holds(
            r#"{"IpAddress": {"aws:SourceIp": "203.0.113.0/24"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"IpAddress": {"aws:SourceIp": "203.0.113.7"}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"IpAddress": {"aws:SourceIp": "198.51.100.0/24"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"NotIpAddress": {"aws:SourceIp": ["198.51.100.0/24", "192.0.2.0/24"]}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"NotIpAddress": {"aws:SourceIp": ["198.51.100.0/24", "203.0.113.0/24"]}}"#,
            &c
        ));
        assert!(holds(r#"{"IpAddress": {"aws:SourceIp": "0.0.0.0/0"}}"#, &c));
        let v6 = ctx(&[("aws:sourceip", &["2001:db8::1"])]);
        assert!(holds(
            r#"{"IpAddress": {"aws:SourceIp": "2001:db8::/32"}}"#,
            &v6
        ));
        assert!(!holds(
            r#"{"IpAddress": {"aws:SourceIp": "203.0.113.0/24"}}"#,
            &v6
        ));
    }

    #[test]
    fn arn_operators() {
        let c = ctx(&[("aws:sourcearn", &["arn:aws:s3:::source-bucket"])]);
        assert!(holds(
            r#"{"ArnEquals": {"aws:SourceArn": "arn:aws:s3:::source-bucket"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"ArnLike": {"aws:SourceArn": "arn:aws:s3:::source-*"}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"ArnNotLike": {"aws:SourceArn": "arn:aws:s3:::source-*"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"ArnNotEquals": {"aws:SourceArn": "arn:aws:s3:::other"}}"#,
            &c
        ));
        // A wildcard does not span the colon separating ARN components.
        assert!(!holds(
            r#"{"ArnLike": {"aws:SourceArn": "arn:aws:*source-bucket"}}"#,
            &c
        ));
    }

    #[test]
    fn null_operator() {
        let present = ctx(&[("s3:x-amz-server-side-encryption", &["AES256"])]);
        let absent = SimulationContext::default();
        let is_null = r#"{"Null": {"s3:x-amz-server-side-encryption": "true"}}"#;
        let not_null = r#"{"Null": {"s3:x-amz-server-side-encryption": "false"}}"#;
        assert!(holds(is_null, &absent));
        assert!(!holds(is_null, &present));
        assert!(holds(not_null, &present));
        assert!(!holds(not_null, &absent));
        assert!(holds(
            r#"{"Null": {"s3:x-amz-server-side-encryption": true}}"#,
            &absent
        ));
        // `Null` does not report the key as missing.
        let cond: ConditionBlock = serde_json::from_str(is_null).unwrap();
        assert!(
            evaluate_condition_block(&cond, &absent)
                .missing_keys
                .is_empty()
        );
    }

    #[test]
    fn missing_key_rules() {
        let absent = SimulationContext::default();
        // Positive operators fail.
        for condition in [
            r#"{"StringEquals": {"s3:x-amz-acl": "private"}}"#,
            r#"{"StringLike": {"s3:prefix": "*"}}"#,
            r#"{"Bool": {"aws:SecureTransport": "false"}}"#,
            r#"{"NumericLessThan": {"s3:max-keys": "10"}}"#,
            r#"{"DateLessThan": {"aws:CurrentTime": "2030-01-01T00:00:00Z"}}"#,
            r#"{"IpAddress": {"aws:SourceIp": "0.0.0.0/0"}}"#,
            r#"{"ArnLike": {"aws:SourceArn": "*"}}"#,
            r#"{"ForAnyValue:StringEquals": {"s3:RequestObjectTagKeys": "a"}}"#,
        ] {
            assert!(!holds(condition, &absent), "{condition}");
        }
        // Negated operators, IfExists and ForAllValues hold.
        for condition in [
            r#"{"StringNotEquals": {"s3:x-amz-server-side-encryption": "aws:kms"}}"#,
            r#"{"StringNotEqualsIgnoreCase": {"s3:x-amz-acl": "private"}}"#,
            r#"{"StringNotLike": {"s3:prefix": "home/*"}}"#,
            r#"{"NumericNotEquals": {"s3:max-keys": "10"}}"#,
            r#"{"DateNotEquals": {"aws:CurrentTime": "2030-01-01T00:00:00Z"}}"#,
            r#"{"NotIpAddress": {"aws:SourceIp": "203.0.113.0/24"}}"#,
            r#"{"ArnNotLike": {"aws:SourceArn": "arn:aws:s3:::x"}}"#,
            r#"{"ArnNotEquals": {"aws:SourceArn": "arn:aws:s3:::x"}}"#,
            r#"{"StringEqualsIfExists": {"s3:x-amz-acl": "private"}}"#,
            r#"{"BoolIfExists": {"aws:SecureTransport": "false"}}"#,
            r#"{"StringNotEqualsIfExists": {"s3:x-amz-acl": "private"}}"#,
            r#"{"ForAllValues:StringEquals": {"s3:RequestObjectTagKeys": "a"}}"#,
        ] {
            assert!(holds(condition, &absent), "{condition}");
        }
        let cond: ConditionBlock =
            serde_json::from_str(r#"{"StringNotEquals": {"s3:x-amz-acl": "private"}}"#).unwrap();
        assert_eq!(
            evaluate_condition_block(&cond, &absent).missing_keys,
            vec!["s3:x-amz-acl"]
        );
    }

    #[test]
    fn if_exists_tests_the_value_when_present() {
        let c = ctx(&[("s3:x-amz-acl", &["public-read"])]);
        assert!(!holds(
            r#"{"StringEqualsIfExists": {"s3:x-amz-acl": "private"}}"#,
            &c
        ));
        assert!(holds(
            r#"{"StringEqualsIfExists": {"s3:x-amz-acl": "public-read"}}"#,
            &c
        ));
    }

    #[test]
    fn set_operators() {
        let c = ctx(&[("s3:requestobjecttagkeys", &["env", "team"])]);
        assert!(holds(
            r#"{"ForAllValues:StringEquals": {"s3:RequestObjectTagKeys": ["env", "team", "cost"]}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"ForAllValues:StringEquals": {"s3:RequestObjectTagKeys": ["env"]}}"#,
            &c
        ));
        assert!(holds(
            r#"{"ForAnyValue:StringEquals": {"s3:RequestObjectTagKeys": ["env"]}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"ForAnyValue:StringEquals": {"s3:RequestObjectTagKeys": ["cost"]}}"#,
            &c
        ));
        // None of the request values may match.
        assert!(!holds(
            r#"{"ForAllValues:StringNotEquals": {"s3:RequestObjectTagKeys": ["env"]}}"#,
            &c
        ));
        assert!(holds(
            r#"{"ForAllValues:StringNotEquals": {"s3:RequestObjectTagKeys": ["cost"]}}"#,
            &c
        ));
        // At least one request value must match none of the policy values.
        assert!(holds(
            r#"{"ForAnyValue:StringNotEquals": {"s3:RequestObjectTagKeys": ["env"]}}"#,
            &c
        ));
        assert!(!holds(
            r#"{"ForAnyValue:StringNotEquals": {"s3:RequestObjectTagKeys": ["env", "team"]}}"#,
            &c
        ));
        assert!(holds(
            r#"{"ForAnyValue:StringLike": {"s3:RequestObjectTagKeys": ["te*"]}}"#,
            &c
        ));
    }

    #[test]
    fn unknown_operator_never_holds() {
        let c = ctx(&[("s3:prefix", &["a"])]);
        assert!(!holds(r#"{"StringMatches": {"s3:prefix": "a"}}"#, &c));
        assert!(!holds(r#"{"NullIfExists": {"s3:prefix": "true"}}"#, &c));
        assert!(!holds(
            r#"{"StringEquals": {"s3:prefix": "a"}, "Bogus": {"s3:prefix": "a"}}"#,
            &c
        ));
    }
}
