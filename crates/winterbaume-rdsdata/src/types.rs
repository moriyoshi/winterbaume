use serde_json::Value;

#[derive(Debug, Clone)]
pub struct QueryResults {
    pub records: Vec<Vec<Value>>,
    pub column_metadata: Vec<ColumnMetadata>,
    pub number_of_records_updated: i64,
}

#[derive(Debug, Clone)]
pub struct ColumnMetadata {
    pub name: String,
    pub type_name: String,
    pub label: String,
}
