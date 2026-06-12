use crate::models::{AppSettings, Event};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub duration_minutes: i64,
}

pub fn find_free_slots(events: &[Event], settings: &AppSettings) -> Vec<TimeSlot> {
    let now = Utc::now();
    let date = now.date_naive();

    let work_start = Utc.from_utc_datetime(
        &date.and_hms_opt(settings.work_start_hour as u32, 0, 0).unwrap_or_default()
    );
    let work_end = Utc.from_utc_datetime(
        &date.and_hms_opt(settings.work_end_hour as u32, 0, 0).unwrap_or_default()
    );
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
