pub mod conflicts;
pub mod patterns;
pub mod priorities;
pub mod time_slots;

pub use conflicts::*;
pub use patterns::*;
pub use priorities::*;
pub use time_slots::*;

use crate::models::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummary {
    pub date: String,
    pub events: Vec<Event>,
    pub tasks_due: Vec<Task>,
    pub tasks_overdue: Vec<Task>,
    pub conflicts: Vec<EventConflict>,
    pub free_slots: Vec<TimeSlot>,
    pub priority_tasks: Vec<Task>,
    pub score: u8,
}

pub fn build_daily_summary(
    events: Vec<Event>,
    tasks: Vec<Task>,
    settings: &AppSettings,
) -> DailySummary {
    let today = Utc::now();
    let today_start = today.date_naive().and_hms_opt(0, 0, 0)
        .and_then(|ndt| chrono::Utc.from_local_datetime(&ndt).single())
        .unwrap_or(today);
    let today_end = today.date_naive().and_hms_opt(23, 59, 59)
        .and_then(|ndt| chrono::Utc.from_local_datetime(&ndt).single())
        .unwrap_or(today);

    let today_events: Vec<Event> = events.iter()
        .filter(|e| e.start >= today_start && e.start <= today_end)
        .cloned()
        .collect();

    let tasks_due: Vec<Task> = tasks.iter()
        .filter(|t| {
            t.status == TaskStatus::Todo
            && t.due_date.map(|d| d >= today_start && d <= today_end).unwrap_or(false)
        })
        .cloned()
        .collect();

    let tasks_overdue: Vec<Task> = tasks.iter()
        .filter(|t| t.is_overdue())
        .cloned()
        .collect();

    let conflicts = find_conflicts(&today_events);
    let free_slots = find_free_slots(&today_events, settings);

    let mut priority_tasks = tasks.iter()
        .filter(|t| t.status == TaskStatus::Todo)
        .cloned()
        .collect::<Vec<_>>();
    priority_tasks.sort_by(|a, b| b.urgency_score().partial_cmp(&a.urgency_score()).unwrap());
    priority_tasks.truncate(5);

    let score = calculate_day_score(&today_events, &tasks_due, &tasks_overdue, &conflicts);

    DailySummary {
        date: today.format("%Y-%m-%d").to_string(),
        events: today_events,
        tasks_due,
        tasks_overdue,
        conflicts,
        free_slots,
        priority_tasks,
        score,
    }
}

fn calculate_day_score(
    events: &[Event],
    tasks_due: &[Task],
    overdue: &[Task],
    conflicts: &[EventConflict],
) -> u8 {
    let mut score: i32 = 100;
    score -= (conflicts.len() as i32) * 15;
    score -= (overdue.len() as i32).min(5) * 10;
    if events.len() > 6 { score -= 10; }
    if tasks_due.len() > 8 { score -= 10; }
    score.clamp(0, 100) as u8
}
