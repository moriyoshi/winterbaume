use std::collections::HashMap;

/// A tagged resource, keyed by ARN with its tags.
#[derive(Debug, Clone)]
pub struct TaggedResource {
    pub arn: String,
    pub tags: HashMap<String, String>,
}
