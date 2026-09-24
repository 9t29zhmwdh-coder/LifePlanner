use super::{date_parser::extract_dates, ExtractionResult};
use crate::models::*;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_TASK: Lazy<Regex> = Lazy::new(|| {
    // A sentence ends at a full stop followed by a space or the line end, not at the dots of a date.
    Regex::new(r"(?im)^\s*[-*•]\s+(.+)$|\b(?:todo|aufgabe|task|erledige|muss|soll|bitte|please)\b\s*:?\s*(.+?)(?:\.(?:\s|$)|$)").unwrap()
});
static RE_MEETING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(meeting|besprechung|call|standup|review|demo|interview|termin|treffen|konferenz|webinar)\b").unwrap()
});
static RE_PRIORITY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(dringend|urgent|kritisch|critical|wichtig|important|asap|sofort)\b").unwrap()
});

pub fn extract_from_text(text: &str) -> ExtractionResult {
    let dates = extract_dates(text);
    let mut events: Vec<Event> = Vec::new();
    let mut tasks: Vec<Task> = Vec::new();
    // Lines already turned into a task; a "Bitte ..." line with a deadline must not come twice.
    let mut used_lines: Vec<&str> = Vec::new();

    // Deadline → task with due date, titled by its own line
    for date_info in dates.iter().filter(|d| d.is_deadline) {
        let line = line_containing(text, &date_info.raw);
        let mut task = Task::new(clean_line(line));
        task.source = TaskSource::Extracted;
        task.due_date = Some(date_info.datetime);
        task.priority = priority_of(line);
        task.energy_level = infer_energy(line);
        used_lines.push(line);
        tasks.push(task);
    }

    // Appointment: the first plain date, when the text talks about a meeting or has no deadlines
    let has_deadline = dates.iter().any(|d| d.is_deadline);
    if RE_MEETING.is_match(text) || !has_deadline {
        if let Some(date_info) = dates.iter().find(|d| !d.is_deadline) {
            let line = line_containing(text, &date_info.raw);
            let mut ev = Event::new(event_title(line, &date_info.raw), date_info.datetime);
            ev.source = EventSource::Extracted;
            ev.location = extract_location(line);
            used_lines.push(line);
            events.push(ev);
        }
    }

    // Bullet points and "todo:" / "bitte" lines
    for cap in RE_TASK.captures_iter(text) {
        let Some(found) = cap.get(1).or_else(|| cap.get(2)) else { continue };
        let line = line_containing(text, found.as_str());
        if used_lines.contains(&line) || found.as_str().trim().chars().count() < 3 {
            continue;
        }
        let raw = found.as_str().trim();
        let mut task = Task::new(clean_line(raw));
        task.source = TaskSource::Extracted;
        task.energy_level = infer_energy(raw);
        task.priority = priority_of(raw);
        task.due_date = extract_dates(raw).into_iter().next().map(|d| d.datetime);
        used_lines.push(line);
        tasks.push(task);
    }

    ExtractionResult {
        events,
        tasks,
        source_text: text.to_string(),
    }
}

fn priority_of(text: &str) -> TaskPriority {
    if RE_PRIORITY.is_match(text) { TaskPriority::High } else { TaskPriority::Medium }
}

/// The line that holds `needle`; the first line of the text was used before,
/// which turned "Hallo zusammen" into the title of every appointment.
fn line_containing<'a>(text: &'a str, needle: &str) -> &'a str {
    text.lines().find(|l| l.contains(needle)).unwrap_or("").trim()
}

/// Everything before the date ("Besprechung am 12.10." → "Besprechung"), or the
/// whole line when the date comes first. Empty means untitled; the app shows a
/// placeholder in the person's language.
fn event_title(line: &str, date_raw: &str) -> String {
    static RE_TRAILING: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?i)[\s,:;-]*\b(?:am|um|on|at|vom|ab)?[\s,:;-]*$").unwrap()
    });
    let before = line.find(date_raw).map(|i| &line[..i]).unwrap_or(line);
    let before = RE_TRAILING.replace(before, "");
    if before.chars().count() >= 3 { clean_line(&before) } else { clean_line(line) }
}

/// Drops bullets, "Todo:" or "Bitte" and the closing full stop, capitalises, caps at 80 characters.
fn clean_line(line: &str) -> String {
    static RE_PREFIX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?i)^\s*(?:[-*•]\s*)?(?:(?:todo|aufgabe|task)\s*:\s*|(?:bitte|please)\s+)?").unwrap()
    });
    let body = RE_PREFIX.replace(line, "");
    let body = body.trim().trim_end_matches('.').trim();
    let mut chars = body.chars();
    let capitalised: String = match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    };
    capitalised.chars().take(80).collect()
}

fn extract_location(text: &str) -> Option<String> {
    static RE_LOC: Lazy<Regex> = Lazy::new(|| {
        // Whole words only ("bei" inside "beim" swallowed the rest of the line), and
        // the place ends before a colon or a date.
        Regex::new(r"(?i)\b(?:in|im|at|bei|beim|ort|location|raum|room)\b\s*:?\s*([^\n,:0-9]{3,50})").unwrap()
    });
    RE_LOC.captures(text)
        .map(|c| c[1].trim().trim_end_matches(|ch: char| !ch.is_alphanumeric()).trim().to_string())
        .filter(|place| place.chars().count() >= 3)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_appointment_line_names_the_event_not_the_greeting() {
        let r = extract_from_text("Hallo zusammen\n\nBesprechung am 12.10.2026 um 16:30 im Sitzungszimmer.");
        assert_eq!(r.events[0].title, "Besprechung");
        assert_eq!(r.events[0].location.as_deref(), Some("Sitzungszimmer"));
    }

    #[test]
    fn a_deadline_with_a_full_date_becomes_one_task_with_its_due_date() {
        let r = extract_from_text("Hallo\nBitte den Bericht bis 10.10.2026 abgeben.");
        assert_eq!(r.tasks.len(), 1, "{:?}", r.tasks);
        assert_eq!(r.tasks[0].title, "Den Bericht bis 10.10.2026 abgeben");
        assert!(r.tasks[0].due_date.is_some());
        assert!(r.events.is_empty(), "a deadline is not an appointment");
    }

    #[test]
    fn meeting_and_deadline_in_one_mail() {
        let text = "Hallo zusammen\n\nBesprechung am 12.10.2026 um 16:30 im Sitzungszimmer.\nBitte den Bericht bis 10.10.2026 abgeben.";
        let r = extract_from_text(text);
        assert_eq!((r.events.len(), r.tasks.len()), (1, 1));
        assert!(r.events[0].start > r.tasks[0].due_date.unwrap());
    }

    #[test]
    fn english_deadlines_read_the_same_way() {
        let r = extract_from_text("Please send the report by 10.10.2026.");
        assert_eq!(r.tasks[0].title, "Send the report by 10.10.2026");
        assert!(r.tasks[0].due_date.is_some());
    }

    #[test]
    fn bullet_points_become_tasks() {
        let r = extract_from_text("- Folien vorbereiten\n- Raum buchen");
        let titles: Vec<_> = r.tasks.iter().map(|t| t.title.as_str()).collect();
        assert_eq!(titles, ["Folien vorbereiten", "Raum buchen"]);
    }

    #[test]
    fn capitalised_relative_dates_are_found() {
        assert_eq!(extract_from_text("Morgen um 10:00 Call mit Anna").events.len(), 1);
    }
}
