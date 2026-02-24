//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudsearchdomain

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<std::collections::HashMap<String, BucketInfo>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits: Option<Hits>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<std::collections::HashMap<String, FieldStats>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SearchStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<Bucket>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Bucket {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hits {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<Vec<Hit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hit {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exprs: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldStats {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(rename = "sumOfSquares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_of_squares: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timems: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuggestRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuggestResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SuggestStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest: Option<SuggestModel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuggestStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timems: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuggestModel {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<SuggestionMatch>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuggestionMatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadDocumentsRequest {
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub documents: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadDocumentsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adds: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<DocumentServiceWarning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentServiceWarning {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
