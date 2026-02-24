/// Look up the WCU capacity for an AWS-managed rule group.
///
/// Returns `None` for unknown rule groups — the caller decides whether to
/// error or use a fallback.
pub fn managed_rule_group_capacity(vendor_name: &str, name: &str) -> Option<u64> {
    match (vendor_name, name) {
        ("AWS", "AWSManagedRulesCommonRuleSet") => Some(700),
        ("AWS", "AWSManagedRulesSQLiRuleSet") => Some(200),
        ("AWS", "AWSManagedRulesKnownBadInputsRuleSet") => Some(200),
        ("AWS", "AWSManagedRulesAmazonIpReputationList") => Some(25),
        ("AWS", "AWSManagedRulesAnonymousIpList") => Some(50),
        ("AWS", "AWSManagedRulesBotControlRuleSet") => Some(50),
        ("AWS", "AWSManagedRulesLinuxRuleSet") => Some(200),
        ("AWS", "AWSManagedRulesUnixRuleSet") => Some(100),
        ("AWS", "AWSManagedRulesWindowsRuleSet") => Some(200),
        ("AWS", "AWSManagedRulesPHPRuleSet") => Some(100),
        ("AWS", "AWSManagedRulesWordPressRuleSet") => Some(100),
        ("AWS", "AWSManagedRulesAdminProtectionRuleSet") => Some(100),
        ("AWS", "AWSManagedRulesACFPRuleSet") => Some(50),
        ("AWS", "AWSManagedRulesATPRuleSet") => Some(50),
        ("AWS", "AWSManagedRulesAntiDDoSRuleSet") => Some(25),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_groups() {
        assert_eq!(
            managed_rule_group_capacity("AWS", "AWSManagedRulesCommonRuleSet"),
            Some(700)
        );
        assert_eq!(
            managed_rule_group_capacity("AWS", "AWSManagedRulesSQLiRuleSet"),
            Some(200)
        );
    }

    #[test]
    fn unknown_group() {
        assert_eq!(
            managed_rule_group_capacity("SomeVendor", "CustomRules"),
            None
        );
    }
}
