use crate::models::Event;
use chrono::{Datelike, Timelike};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecurringPattern {
    pub title: String,
    pub count: usize,
    pub typical_day_of_week: Option<u32>,
    pub typical_hour: Option<u32>,
}

pub fn detect_patterns(events: &[Event]) -> Vec<RecurringPattern> {
    let mut groups: HashMap<String, Vec<&Event>> = HashMap::new();

    for ev in events {
        let key = normalize_title(&ev.title);
        groups.entry(key).or_default().push(ev);
    }

    let mut patterns = Vec::new();
    for (title, group) in groups.iter().filter(|(_, g)| g.len() >= 2) {
        let typical_day = most_common(group.iter().map(|e| e.start.weekday().num_days_from_monday()));
        let typical_hour = most_common(group.iter().map(|e| e.start.hour()));

        patterns.push(RecurringPattern {
            title: title.clone(),
            count: group.len(),
            typical_day_of_week: typical_day,
            typical_hour,
        });
    }

    patterns.sort_by_key(|b| std::cmp::Reverse(b.count));
    patterns
}

fn normalize_title(title: &str) -> String {
    title.to_lowercase()
        .split_whitespace()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ")
}

fn most_common<I: Iterator<Item = u32>>(iter: I) -> Option<u32> {
    let mut counts: HashMap<u32, usize> = HashMap::new();
    for v in iter { *counts.entry(v).or_insert(0) += 1; }
    counts.into_iter().max_by_key(|(_, c)| *c).map(|(v, _)| v)
}
