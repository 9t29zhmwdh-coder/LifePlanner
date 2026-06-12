use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Active,
    OnHold,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub deadline: Option<DateTime<Utc>>,
    pub task_ids: Vec<String>,
    pub event_ids: Vec<String>,
    pub tags: Vec<String>,
    pub auto_detected: bool,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            description: None,
            status: ProjectStatus::Active,
            deadline: None,
            task_ids: vec![],
            event_ids: vec![],
            tags: vec![],
            auto_detected: false,
            color: "#58a6ff".into(),
            created_at: now,
            updated_at: now,
        }
    }
}
