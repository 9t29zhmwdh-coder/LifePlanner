use crate::models::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConflict {
    pub event_a: Event,
    pub event_b: Event,
    pub overlap_minutes: i64,
}

pub fn find_conflicts(events: &[Event]) -> Vec<EventConflict> {
    let mut conflicts = Vec::new();
    let confirmed: Vec<&Event> = events.iter()
        .filter(|e| e.status == crate::models::EventStatus::Confirmed)
        .collect();

    for (i, a) in confirmed.iter().enumerate() {
        for b in &confirmed[i + 1..] {
            if a.overlaps(b) {
                let a_end = a.end.unwrap_or_else(|| a.start + chrono::Duration::hours(1));
                let b_end = b.end.unwrap_or_else(|| b.start + chrono::Duration::hours(1));
                let overlap_start = a.start.max(b.start);
                let overlap_end   = a_end.min(b_end);
                let overlap_minutes = (overlap_end - overlap_start).num_minutes().max(0);

                conflicts.push(EventConflict {
                    event_a: (*a).clone(),
                    event_b: (*b).clone(),
                    overlap_minutes,
                });
            }
        }
    }

    conflicts
}
