use crate::models::{EnergyLevel, Task, TaskStatus};

pub fn sort_by_priority(tasks: &mut [Task]) {
    tasks.sort_by(|a, b| {
        b.urgency_score().partial_cmp(&a.urgency_score()).unwrap()
    });
}

pub fn suggest_schedule(tasks: &[Task], available_minutes: u64) -> Vec<&Task> {
    let open: Vec<&Task> = tasks.iter()
        .filter(|t| t.status == TaskStatus::Todo)
        .collect();

    let mut selected = Vec::new();
    let mut remaining = available_minutes;

    // 1. Overdue first
    for t in open.iter().filter(|t| t.is_overdue()) {
        let cost = t.estimated_minutes.unwrap_or(30) as u64;
        if cost <= remaining {
            selected.push(*t);
            remaining -= cost;
        }
    }

    // 2. High energy tasks while budget remains large
    for t in open.iter().filter(|t| t.energy_level == EnergyLevel::High) {
        if remaining < 60 { break; }
        let cost = t.estimated_minutes.unwrap_or(60) as u64;
        if cost <= remaining && !selected.contains(t) {
            selected.push(*t);
            remaining -= cost;
        }
    }

    // 3. Fill remaining time with medium/low energy
    for t in open.iter().filter(|t| t.energy_level != EnergyLevel::High) {
        let cost = t.estimated_minutes.unwrap_or(30) as u64;
        if cost <= remaining && !selected.contains(t) {
            selected.push(*t);
            remaining -= cost;
        }
    }

    selected
}
