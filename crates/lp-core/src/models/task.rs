use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Someday,
    Low,
    Medium,
    High,
    Critical,
}

impl TaskPriority {
    pub fn score(&self) -> u8 {
        match self {
            Self::Critical => 5,
            Self::High     => 4,
            Self::Medium   => 3,
            Self::Low      => 2,
            Self::Someday  => 1,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "Kritisch",
            Self::High     => "Hoch",
            Self::Medium   => "Mittel",
            Self::Low      => "Niedrig",
            Self::Someday  => "Irgendwann",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EnergyLevel {
    High,   // deep work, complex analysis, writing
    Medium, // meetings, creative tasks, learning
    Low,    // admin, email, routine tasks
}

impl EnergyLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::High   => "Fokus",
            Self::Medium => "Kreativ",
            Self::Low    => "Routine",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskSource {
    Manual,
    Extracted,
    CalDav,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: TaskPriority,
    pub energy_level: EnergyLevel,
    pub status: TaskStatus,
    pub project_id: Option<String>,
    pub estimated_minutes: Option<u32>,
    pub linked_event_ids: Vec<String>,
    pub tags: Vec<String>,
    pub source: TaskSource,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            description: None,
            due_date: None,
            priority: TaskPriority::Medium,
            energy_level: EnergyLevel::Medium,
            status: TaskStatus::Todo,
            project_id: None,
            estimated_minutes: None,
            linked_event_ids: vec![],
            tags: vec![],
            source: TaskSource::Manual,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_overdue(&self) -> bool {
        self.due_date.map(|d| d < Utc::now()).unwrap_or(false)
            && self.status == TaskStatus::Todo
    }

    pub fn urgency_score(&self) -> f32 {
        let mut score = self.priority.score() as f32;
        if let Some(due) = self.due_date {
            let hours_left = (due - Utc::now()).num_hours();
            if hours_left < 0    { score += 3.0; }
            else if hours_left < 24  { score += 2.5; }
            else if hours_left < 72  { score += 1.5; }
            else if hours_left < 168 { score += 0.5; }
        }
        score
    }
}
