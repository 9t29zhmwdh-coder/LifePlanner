use crate::models::{AppSettings, Event};
use chrono::{DateTime, Local, Utc};
use crate::extractor::date_parser::local_to_utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub duration_minutes: i64,
}

pub fn find_free_slots(events: &[Event], settings: &AppSettings) -> Vec<TimeSlot> {
    let now = Utc::now();
    // Working hours are wall-clock hours; as UTC they ran from 10 to 20 in Swiss summer.
    let date = Local::now().date_naive();
    let at_hour = |hour: u8| {
        date.and_hms_opt(u32::from(hour.min(23)), 0, 0).and_then(local_to_utc).unwrap_or(now)
    };
    let work_start = at_hour(settings.work_start_hour);
    let work_end = at_hour(settings.work_end_hour);
    let min_dur = settings.min_free_slot_minutes as i64;

    // Only confirmed events that overlap with work hours
    let mut busy: Vec<(DateTime<Utc>, DateTime<Utc>)> = events.iter()
        .filter(|e| e.status == crate::models::EventStatus::Confirmed && !e.all_day)
        .map(|e| {
            let start = e.start.max(work_start);
            let end = e.end.unwrap_or_else(|| e.start + chrono::Duration::hours(1)).min(work_end);
            (start, end)
        })
        .filter(|(s, e)| s < e)
        .collect();

    busy.sort_by_key(|(s, _)| *s);

    let mut slots = Vec::new();
    let mut cursor = work_start.max(now);

    for (start, end) in &busy {
        if cursor < *start {
            let dur = (*start - cursor).num_minutes();
            if dur >= min_dur {
                slots.push(TimeSlot {
                    start: cursor,
                    end: *start,
                    duration_minutes: dur,
                });
            }
        }
        if *end > cursor {
            cursor = *end;
        }
    }

    if cursor < work_end {
        let dur = (work_end - cursor).num_minutes();
        if dur >= min_dur {
            slots.push(TimeSlot {
                start: cursor,
                end: work_end,
                duration_minutes: dur,
            });
        }
    }

    slots
}
