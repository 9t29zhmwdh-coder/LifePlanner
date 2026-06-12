use crate::models::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use ical::IcalParser;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IcsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

pub fn parse_ics_file(path: &Path) -> Result<Vec<Event>, IcsError> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let parser = IcalParser::new(reader);

    let mut events = Vec::new();

    for calendar in parser.flatten() {
        for component in calendar.events {
            let props = &component.properties;
            let get = |name: &str| -> Option<String> {
                props.iter()
                    .find(|p| p.name == name)
                    .and_then(|p| p.value.clone())
            };

            let uid = get("UID").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let title = get("SUMMARY").unwrap_or_else(|| "Unbekannter Termin".into());
            let description = get("DESCRIPTION");
            let location = get("LOCATION");

            let start = get("DTSTART")
                .and_then(|s| parse_ical_datetime(&s))
                .unwrap_or_else(Utc::now);

            let end = get("DTEND").and_then(|s| parse_ical_datetime(&s));

            let all_day = get("DTSTART")
                .map(|s| s.len() == 8)
                .unwrap_or(false);

            let status = match get("STATUS").as_deref() {
                Some("CANCELLED") => EventStatus::Cancelled,
                Some("TENTATIVE") => EventStatus::Tentative,
                _ => EventStatus::Confirmed,
            };

            let recurrence = get("RRULE")
                .and_then(|r| parse_rrule(&r));

            let mut ev = Event::new(title, start);
            ev.external_uid = Some(uid);
            ev.description = description;
            ev.location = location;
            ev.end = end;
            ev.all_day = all_day;
            ev.status = status;
            ev.recurrence = recurrence;
            ev.source = EventSource::IcsFile;

            events.push(ev);
        }
    }

    Ok(events)
}

fn parse_ical_datetime(s: &str) -> Option<DateTime<Utc>> {
    let s = s.trim_end_matches('Z').replace('T', "");
    if s.len() == 8 {
        NaiveDateTime::parse_from_str(&format!("{}000000", s), "%Y%m%d%H%M%S")
            .ok()
            .and_then(|ndt| Utc.from_local_datetime(&ndt).single())
    } else if s.len() >= 14 {
        NaiveDateTime::parse_from_str(&s[..14], "%Y%m%d%H%M%S")
            .ok()
            .and_then(|ndt| Utc.from_local_datetime(&ndt).single())
    } else {
        None
    }
}

fn parse_rrule(rule: &str) -> Option<RecurrenceRule> {
    let mut freq = String::new();
    let mut interval = 1u32;
    let mut until = None;
    let mut count = None;
    let mut by_day = vec![];

    for part in rule.split(';') {
        if let Some((key, val)) = part.split_once('=') {
            match key {
                "FREQ"     => freq = val.to_string(),
                "INTERVAL" => interval = val.parse().unwrap_or(1),
                "UNTIL"    => until = parse_ical_datetime(val),
                "COUNT"    => count = val.parse().ok(),
                "BYDAY"    => by_day = val.split(',').map(|s| s.to_string()).collect(),
                _ => {}
            }
        }
    }

    if freq.is_empty() { None } else {
        Some(RecurrenceRule { freq, interval, until, count, by_day })
    }
}
