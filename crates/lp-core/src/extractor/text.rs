use super::{date_parser::extract_dates, ExtractionResult};
use crate::models::*;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_TASK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?im)^\s*[-*•]\s+(.+)$|(?:todo|aufgabe|task|erledige|muss|soll|bitte)\s*:?\s*(.+?)(?:\.|$)").unwrap()
});
static RE_MEETING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(meeting|besprechung|call|standup|review|demo|interview|termin|treffen|konferenz|webinar)\b").unwrap()
});
static RE_DEADLINE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(deadline|frist|fällig|abgabe|due|until|bis)\b").unwrap()
});
static RE_PRIORITY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(dringend|urgent|kritisch|critical|wichtig|important|asap|sofort)\b").unwrap()
});

pub fn extract_from_text(text: &str) -> ExtractionResult {
    let dates = extract_dates(text);
    let mut events: Vec<Event> = Vec::new();
    let mut tasks: Vec<Task> = Vec::new();

    // Detect meeting-type events
    if RE_MEETING.is_match(text) {
        let title = extract_title(text);
        if let Some(date_info) = dates.iter().find(|d| !d.is_deadline) {
            let mut ev = Event::new(&title, date_info.datetime);
            ev.source = EventSource::Extracted;
            ev.end = Some(date_info.datetime + chrono::Duration::hours(1));
            if let Some(loc) = extract_location(text) {
                ev.location = Some(loc);
            }
            events.push(ev);
        }
    }

    // Deadline → task with due date
    for date_info in dates.iter().filter(|d| d.is_deadline) {
        let title = extract_title(text);
        let mut task = Task::new(title);
        task.source = TaskSource::Extracted;
        task.due_date = Some(date_info.datetime);
        task.priority = if RE_PRIORITY.is_match(text) {
            TaskPriority::High
        } else {
            TaskPriority::Medium
        };
        task.energy_level = infer_energy(text);
        tasks.push(task);
    }

    // Bullet-point tasks
    for cap in RE_TASK.captures_iter(text) {
        let raw = cap.get(1).or_else(|| cap.get(2))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        if raw.len() < 3 { continue; }

        let mut task = Task::new(&raw);
        task.source = TaskSource::Extracted;
        task.energy_level = infer_energy(&raw);
        task.priority = if RE_PRIORITY.is_match(&raw) {
            TaskPriority::High
        } else {
            TaskPriority::Medium
        };
        if let Some(d) = extract_dates(&raw).into_iter().next() {
            task.due_date = Some(d.datetime);
        }
        tasks.push(task);
    }

    // Standalone dates without meeting keyword → event
    if events.is_empty() && !dates.is_empty() && !RE_DEADLINE.is_match(text) {
        let title = extract_title(text);
        for date_info in dates.iter().take(1) {
            let mut ev = Event::new(&title, date_info.datetime);
            ev.source = EventSource::Extracted;
            ev.end = Some(date_info.datetime + chrono::Duration::hours(1));
            events.push(ev);
        }
    }

    ExtractionResult {
        events,
        tasks,
        source_text: text.to_string(),
    }
}

fn extract_title(text: &str) -> String {
    text.lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim().chars().take(80).collect::<String>())
        .unwrap_or_else(|| "Unbekannter Termin".into())
}

fn extract_location(text: &str) -> Option<String> {
    static RE_LOC: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?i)\b(?:in|at|bei|ort|location|raum|room)\s*:?\s*([^\n,]{3,50})").unwrap()
    });
    RE_LOC.captures(text).map(|c| c[1].trim().to_string())
}

fn infer_energy(text: &str) -> EnergyLevel {
    static RE_HIGH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?i)\b(analyse|analyse|konzept|strategie|bericht|präsentation|schreibe|research|analyse|design)\b").unwrap()
    });
    static RE_LOW: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?i)\b(e-mail|email|antworten|formular|admin|buchen|bestellen|weiterleiten)\b").unwrap()
    });
    if RE_HIGH.is_match(text) { EnergyLevel::High }
    else if RE_LOW.is_match(text) { EnergyLevel::Low }
    else { EnergyLevel::Medium }
}
