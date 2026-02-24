use std::collections::HashMap;

/// An IAM Access Analyzer analyzer.
#[derive(Debug, Clone)]
pub struct Analyzer {
    pub arn: String,
    pub name: String,
    pub analyzer_type: String,
    pub status: String,
    pub created_at: String,
    pub tags: HashMap<String, String>,
}

/// An archive rule associated with an analyzer.
#[derive(Debug, Clone)]
pub struct ArchiveRule {
    pub rule_name: String,
    pub filter: HashMap<String, CriterionValue>,
    pub created_at: String,
    pub updated_at: String,
}

/// A filter criterion value used in archive rules.
#[derive(Debug, Clone)]
pub struct CriterionValue {
    pub eq: Option<Vec<String>>,
    pub neq: Option<Vec<String>>,
    pub contains: Option<Vec<String>>,
    pub exists: Option<bool>,
}
