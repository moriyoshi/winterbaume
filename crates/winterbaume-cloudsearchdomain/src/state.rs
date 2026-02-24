use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CloudSearchDomainState {
    /// Documents keyed by document id (string).
    pub documents: HashMap<String, Document>,
    /// Cumulative ingest counters across all UploadDocuments calls.
    pub total_adds: i64,
    pub total_deletes: i64,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    /// `add` or `delete` (carried for view round-trip; deletes drop the entry).
    pub op: String,
    /// Field map. CloudSearch SDF allows multi-valued fields, so we store a flattened
    /// `field -> joined-text` representation -- adequate for matching in `Search`.
    pub fields: HashMap<String, String>,
}

impl CloudSearchDomainState {
    pub fn upload(&mut self, batch: Vec<Document>) -> (i64, i64) {
        let mut adds = 0;
        let mut deletes = 0;
        for d in batch {
            match d.op.as_str() {
                "add" => {
                    self.documents.insert(d.id.clone(), d);
                    adds += 1;
                }
                "delete" if self.documents.remove(&d.id).is_some() => {
                    deletes += 1;
                }
                _ => {}
            }
        }
        self.total_adds += adds;
        self.total_deletes += deletes;
        (adds, deletes)
    }

    pub fn search<'a>(&'a self, query: &'a str) -> Vec<&'a Document> {
        let q = query.trim();
        let mut hits: Vec<&Document> = if q == "matchall" || q.is_empty() {
            self.documents.values().collect()
        } else {
            self.documents
                .values()
                .filter(|d| {
                    d.fields
                        .values()
                        .any(|v| v.to_lowercase().contains(&q.to_lowercase()))
                })
                .collect()
        };
        hits.sort_by(|a, b| a.id.cmp(&b.id));
        hits
    }

    pub fn suggest(&self, query: &str) -> Vec<&Document> {
        let q = query.to_lowercase();
        let mut hits: Vec<&Document> = self
            .documents
            .values()
            .filter(|d| d.fields.values().any(|v| v.to_lowercase().starts_with(&q)))
            .collect();
        hits.sort_by(|a, b| a.id.cmp(&b.id));
        hits
    }
}
