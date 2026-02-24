/// Glob-match `pattern` against `value`.
///
/// Supported wildcards:
/// - `*` matches any sequence of characters (including empty).
/// - `?` matches exactly one character.
///
/// Implemented as a direct backtracking algorithm with no regex dependency.
pub fn glob_match(pattern: &str, value: &str) -> bool {
    let pat: Vec<char> = pattern.chars().collect();
    let val: Vec<char> = value.chars().collect();

    let mut pi = 0; // pattern index
    let mut vi = 0; // value index
    let mut star_pi = usize::MAX; // pattern index after last '*'
    let mut star_vi = 0; // value index when last '*' was hit

    while vi < val.len() {
        if pi < pat.len() && (pat[pi] == '?' || pat[pi] == val[vi]) {
            pi += 1;
            vi += 1;
        } else if pi < pat.len() && pat[pi] == '*' {
            star_pi = pi + 1;
            star_vi = vi;
            pi += 1;
        } else if star_pi != usize::MAX {
            // Backtrack: let the last '*' consume one more character.
            star_vi += 1;
            vi = star_vi;
            pi = star_pi;
        } else {
            return false;
        }
    }

    // Consume any trailing '*' in the pattern.
    while pi < pat.len() && pat[pi] == '*' {
        pi += 1;
    }

    pi == pat.len()
}

/// Case-insensitive action matching (AWS action names are case-insensitive).
pub fn action_matches(pattern: &str, action: &str) -> bool {
    glob_match(&pattern.to_ascii_lowercase(), &action.to_ascii_lowercase())
}

/// Case-sensitive ARN glob matching.
pub fn resource_matches(pattern: &str, resource: &str) -> bool {
    glob_match(pattern, resource)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── glob_match basics ──────────────────────────────────────────────

    #[test]
    fn exact_match() {
        assert!(glob_match("hello", "hello"));
    }

    #[test]
    fn exact_mismatch() {
        assert!(!glob_match("hello", "world"));
    }

    #[test]
    fn star_matches_everything() {
        assert!(glob_match("*", "anything"));
        assert!(glob_match("*", ""));
    }

    #[test]
    fn star_prefix() {
        assert!(glob_match("*bar", "foobar"));
        assert!(!glob_match("*bar", "foobaz"));
    }

    #[test]
    fn star_suffix() {
        assert!(glob_match("foo*", "foobar"));
        assert!(!glob_match("foo*", "barfoo"));
    }

    #[test]
    fn star_middle() {
        assert!(glob_match("f*r", "foobar"));
        assert!(glob_match("f*r", "fr"));
        assert!(!glob_match("f*r", "foobar!"));
    }

    #[test]
    fn multiple_stars() {
        assert!(glob_match("a*b*c", "aXbYc"));
        assert!(glob_match("a*b*c", "abc"));
        assert!(!glob_match("a*b*c", "aXYc"));
    }

    #[test]
    fn question_mark_single_char() {
        assert!(glob_match("h?llo", "hello"));
        assert!(glob_match("h?llo", "hallo"));
        assert!(!glob_match("h?llo", "hllo"));
        assert!(!glob_match("h?llo", "heello"));
    }

    #[test]
    fn combined_star_and_question() {
        assert!(glob_match("h?l*", "hello"));
        assert!(glob_match("h?l*", "hxl"));
    }

    #[test]
    fn empty_pattern_and_value() {
        assert!(glob_match("", ""));
        assert!(!glob_match("", "a"));
        assert!(!glob_match("a", ""));
    }

    #[test]
    fn trailing_stars() {
        assert!(glob_match("foo***", "foo"));
        assert!(glob_match("foo***", "foobar"));
    }

    // ── action_matches ─────────────────────────────────────────────────

    #[test]
    fn action_case_insensitive() {
        assert!(action_matches("s3:GetObject", "s3:getobject"));
        assert!(action_matches("s3:*", "s3:PutObject"));
        assert!(action_matches("*", "iam:CreateUser"));
    }

    #[test]
    fn action_no_match() {
        assert!(!action_matches("s3:Get*", "ec2:DescribeInstances"));
    }

    // ── resource_matches ───────────────────────────────────────────────

    #[test]
    fn resource_exact_arn() {
        assert!(resource_matches(
            "arn:aws:s3:::my-bucket",
            "arn:aws:s3:::my-bucket"
        ));
    }

    #[test]
    fn resource_wildcard_arn() {
        assert!(resource_matches(
            "arn:aws:s3:::my-bucket/*",
            "arn:aws:s3:::my-bucket/key"
        ));
    }

    #[test]
    fn resource_case_sensitive() {
        assert!(!resource_matches(
            "arn:aws:s3:::My-Bucket",
            "arn:aws:s3:::my-bucket"
        ));
    }

    #[test]
    fn resource_star_all() {
        assert!(resource_matches("*", "arn:aws:s3:::anything"));
    }
}
