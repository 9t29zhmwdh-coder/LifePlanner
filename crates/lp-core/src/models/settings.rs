use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CalendarKind {
    IcsFile,
    CalDav,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarAccount {
    pub id: String,
    pub name: String,
    pub kind: CalendarKind,
    pub url: Option<String>,
    pub username: Option<String>,
    pub ics_path: Option<String>,
    pub color: String,
    pub enabled: bool,
    pub last_synced: Option<DateTime<Utc>>,
}

impl CalendarAccount {
    pub fn new_local(name: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            kind: CalendarKind::Local,
            url: None,
            username: None,
            ics_path: None,
            color: "#58a6ff".into(),
            enabled: true,
            last_synced: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ollama_url: String,
    pub text_model: String,
    pub auto_extract_on_paste: bool,
    pub default_event_duration_minutes: u32,
    pub work_start_hour: u8,
    pub work_end_hour: u8,
    pub min_free_slot_minutes: u32,
    pub enable_notifications: bool,
    pub locale: String,
    pub default_calendar_id: Option<String>,
    pub calendar_accounts: Vec<CalendarAccount>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".into(),
            text_model: "llama3".into(),
            auto_extract_on_paste: true,
            default_event_duration_minutes: 60,
            work_start_hour: 8,
            work_end_hour: 18,
            min_free_slot_minutes: 30,
            enable_notifications: true,
            locale: "de-CH".into(),
            default_calendar_id: None,
            calendar_accounts: vec![
                CalendarAccount::new_local("Persönlich"),
            ],
        }
    }
}
