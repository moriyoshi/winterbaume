use serde_json::Value;

#[derive(Debug, Default)]
pub struct CloudTrailDataState {
    /// Audit events ingested via PutAuditEvents in arrival order.
    /// Each entry is the raw eventData JSON envelope.
    pub events: Vec<EventRecord>,
}

#[derive(Debug, Clone)]
pub struct EventRecord {
    pub channel_arn: String,
    pub event_id: String,
    pub external_id: String,
    pub event_data: Value,
    pub checksum: Option<String>,
}

impl CloudTrailDataState {
    pub fn record(&mut self, e: EventRecord) {
        self.events.push(e);
    }

    pub fn count_for_channel(&self, channel_arn: &str) -> usize {
        self.events
            .iter()
            .filter(|e| e.channel_arn == channel_arn)
            .count()
    }
}
