use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    Pdf,
    Text,
    Email,
    Note,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub path: Option<String>,
    pub content_preview: Option<String>,
    pub kind: DocumentKind,
    pub linked_event_ids: Vec<String>,
    pub linked_task_ids: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Document {
    pub fn new(title: impl Into<String>, kind: DocumentKind) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            path: None,
            content_preview: None,
            kind,
            linked_event_ids: vec![],
            linked_task_ids: vec![],
            tags: vec![],
            created_at: Utc::now(),
        }
    }
}
