//! What an extraction found, checked against the calendar before anything is saved.
//!
//! The README promised "checks the result against your calendar for collisions
//! before you commit to anything"; extraction used to write straight into the
//! database instead. Now every source (text, email, PDF, model) ends up here and
//! nothing is stored until the person accepts the preview.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use super::ExtractionResult;
use crate::models::{Event, EventSource, Task, TaskPriority, TaskSource};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreviewConflict {
    /// Id of the newly found event.
    pub event_id: String,
    pub existing_title: String,
    pub existing_start: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionPreview {
    pub result: ExtractionResult,
    pub conflicts: Vec<PreviewConflict>,
}

/// Events without an end get the default length from the settings, so the
/// collision check and the saved event agree on how long it lasts.
pub fn preview(mut result: ExtractionResult, calendar: &[Event], default_minutes: u32) -> ExtractionPreview {
    let length = Duration::minutes(i64::from(default_minutes.clamp(5, 24 * 60)));
    for event in result.events.iter_mut().filter(|e| e.end.is_none() && !e.all_day) {
        event.end = Some(event.start + length);
    }
    let conflicts = result
        .events
        .iter()
        .flat_map(|new| {
            calendar.iter().filter(move |old| new.overlaps(old)).map(move |old| PreviewConflict {
                event_id: new.id.clone(),
                existing_title: old.title.clone(),
                existing_start: old.start,
            })
        })
        .collect();
    ExtractionPreview { result, conflicts }
}

/// What the model is asked to return; everything but the title is optional.
#[derive(Debug, Deserialize)]
struct ModelAnswer {
    #[serde(default)]
    events: Vec<ModelEvent>,
    #[serde(default)]
    tasks: Vec<ModelTask>,
}

#[derive(Debug, Deserialize)]
struct ModelEvent {
    title: String,
    start: DateTime<Utc>,
    duration_minutes: Option<i64>,
    location: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModelTask {
    title: String,
    due_date: Option<DateTime<Utc>>,
    priority: Option<String>,
}

/// Turns the model's JSON into real events and tasks with ids, so they can be
/// previewed and saved like any other extraction. Before, the raw JSON went to
/// the page as if it were saved events, and "save" stored nothing.
pub fn from_model_answer(answer: &str, source_text: &str) -> Result<ExtractionResult, serde_json::Error> {
    let json = json_object(answer);
    let parsed: ModelAnswer = serde_json::from_str(json)?;
    Ok(ExtractionResult {
        events: parsed.events.into_iter().map(event_from_model).collect(),
        tasks: parsed.tasks.into_iter().map(task_from_model).collect(),
        source_text: source_text.to_string(),
    })
}

/// The JSON object inside an answer that may carry prose or a code fence around it.
fn json_object(answer: &str) -> &str {
    let start = answer.find('{').unwrap_or(0);
    let end = answer.rfind('}').map(|i| i + 1).unwrap_or(answer.len());
    answer.get(start..end.max(start)).unwrap_or("")
}

fn event_from_model(m: ModelEvent) -> Event {
    let mut event = Event::new(m.title, m.start);
    event.end = m.duration_minutes.map(|d| m.start + Duration::minutes(d.clamp(5, 24 * 60)));
    event.location = m.location.filter(|l| !l.trim().is_empty() && l != "...");
    event.source = EventSource::Extracted;
    event
}

fn task_from_model(m: ModelTask) -> Task {
    let mut task = Task::new(m.title);
    task.due_date = m.due_date;
    task.priority = match m.priority.as_deref() {
        Some("low") => TaskPriority::Low,
        Some("high") => TaskPriority::High,
        Some("critical") => TaskPriority::Critical,
        _ => TaskPriority::Medium,
    };
    task.source = TaskSource::Extracted;
    task
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn at(h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 12, h, 0, 0).unwrap()
    }

    #[test]
    fn a_new_event_that_overlaps_the_calendar_is_flagged() {
        let mut existing = Event::new("Team meeting", at(14));
        existing.end = Some(at(15));
        let mut new = Event::new("Dentist", at(14));
        new.end = Some(at(15));
        let found = ExtractionResult { events: vec![new.clone()], tasks: vec![], source_text: String::new() };
        let p = preview(found, &[existing], 60);
        assert_eq!(p.conflicts.len(), 1);
        assert_eq!(p.conflicts[0].event_id, new.id);
        assert_eq!(p.conflicts[0].existing_title, "Team meeting");
    }

    #[test]
    fn model_json_becomes_events_and_tasks_with_ids() {
        let answer = "Here you go:\n```json\n{\"events\":[{\"title\":\"Dentist\",\"start\":\"2026-10-12T14:30:00Z\",\"duration_minutes\":30,\"location\":\"...\"}],\"tasks\":[{\"title\":\"Bring insurance card\",\"priority\":\"high\"}]}\n```";
        let r = from_model_answer(answer, "src").unwrap();
        assert_eq!(r.events[0].title, "Dentist");
        assert_eq!(r.events[0].end, Some(Utc.with_ymd_and_hms(2026, 10, 12, 15, 0, 0).unwrap()));
        assert_eq!(r.events[0].location, None);
        assert!(!r.events[0].id.is_empty());
        assert_eq!(r.tasks[0].priority, TaskPriority::High);
    }

    #[test]
    fn a_broken_model_answer_is_an_error_not_a_panic() {
        assert!(from_model_answer("sorry, no idea", "").is_err());
        assert!(from_model_answer("}{", "").is_err());
    }

    #[test]
    fn events_without_an_end_get_the_default_length() {
        let start = Utc.with_ymd_and_hms(2026, 10, 12, 9, 0, 0).unwrap();
        let found = ExtractionResult { events: vec![Event::new("Call", start)], tasks: vec![], source_text: String::new() };
        let p = preview(found, &[], 30);
        assert_eq!(p.result.events[0].end, Some(start + Duration::minutes(30)));
    }

}
