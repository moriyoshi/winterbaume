use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct ChimeSdkMeetingsState {
    pub meetings: HashMap<String, MeetingRecord>,
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default)]
pub struct MeetingRecord {
    pub meeting: Value,
    pub attendees: HashMap<String, Value>,
    pub transcription_active: bool,
}

#[derive(Debug, Error)]
pub enum ChimeMeetingsError {
    #[error("Meeting {id} does not exist.")]
    MeetingNotFound { id: String },

    #[error("Attendee {id} does not exist.")]
    AttendeeNotFound { id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl ChimeSdkMeetingsState {
    pub fn put_meeting(&mut self, meeting_id: String, meeting: Value) {
        self.meetings.entry(meeting_id).or_default().meeting = meeting;
    }

    pub fn put_attendee(&mut self, meeting_id: &str, attendee_id: String, attendee: Value) {
        self.meetings
            .entry(meeting_id.to_string())
            .or_default()
            .attendees
            .insert(attendee_id, attendee);
    }

    pub fn list_attendees(&self, meeting_id: &str) -> Vec<&Value> {
        if let Some(record) = self.meetings.get(meeting_id) {
            let mut items: Vec<&Value> = record.attendees.values().collect();
            items.sort_by(|a, b| {
                a.get("AttendeeId")
                    .and_then(|v| v.as_str())
                    .cmp(&b.get("AttendeeId").and_then(|v| v.as_str()))
            });
            items
        } else {
            vec![]
        }
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
