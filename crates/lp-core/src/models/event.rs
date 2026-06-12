use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventSource {
    Manual,
    Extracted,
    IcsFile,
    CalDav,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrenceRule {
    pub freq: String,       // DAILY, WEEKLY, MONTHLY, YEARLY
    pub interval: u32,
    pub until: Option<DateTime<Utc>>,
    pub count: Option<u32>,
    pub by_day: Vec<String>, // MO, TU, WE, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub all_day: bool,
    pub location: Option<String>,
    pub source: EventSource,
    pub calendar_id: Option<String>,
    pub external_uid: Option<String>,
    pub status: EventStatus,
    pub recurrence: Option<RecurrenceRule>,
    pub linked_task_ids: Vec<String>,
    pub linked_doc_ids: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Event {
    pub fn new(title: impl Into<String>, start: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            description: None,
            start,
            end: None,
            all_day: false,
            location: None,
            source: EventSource::Manual,
            calendar_id: None,
            external_uid: None,
            status: EventStatus::Confirmed,
            recurrence: None,
            linked_task_ids: vec![],
            linked_doc_ids: vec![],
            tags: vec![],
            created_at: now,
            updated_at: now,
        }
    }

    pub fn duration_minutes(&self) -> Option<i64> {
        self.end.map(|end| (end - self.start).num_minutes())
    }

    pub fn overlaps(&self, other: &Event) -> bool {
        let self_end = self.end.unwrap_or_else(|| self.start + chrono::Duration::hours(1));
        let other_end = other.end.unwrap_or_else(|| other.start + chrono::Duration::hours(1));
        self.start < other_end && other.start < self_end
    }
}
