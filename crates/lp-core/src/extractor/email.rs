use super::{text::extract_from_text, ExtractionResult};
use once_cell::sync::Lazy;
use regex::Regex;

static RE_SUBJECT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?im)^Subject:\s*(.+)$").unwrap()
});
static RE_BODY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)\n\n(.+)").unwrap()
});

pub fn extract_from_email(email_text: &str) -> ExtractionResult {
    let subject = RE_SUBJECT.captures(email_text)
        .map(|c| c[1].trim().to_string())
        .unwrap_or_default();

    let body = RE_BODY.captures(email_text)
        .map(|c| c[1].trim().to_string())
        .unwrap_or_else(|| email_text.to_string());

    let combined = if subject.is_empty() {
        body.clone()
    } else {
        format!("{}\n\n{}", subject, body)
    };

    let mut result = extract_from_text(&combined);

    // Prefix subject to extracted event/task titles if they're generic
    if !subject.is_empty() {
        for ev in &mut result.events {
            if ev.title == "Unbekannter Termin" {
                ev.title = subject.clone();
            }
        }
        for task in &mut result.tasks {
            if task.title.len() > 60 {
                task.title = subject.clone();
            }
        }
    }

    result
}
