use crate::analyzer::DailySummary;

pub fn daily_summary_prompt(summary: &DailySummary) -> String {
    let events_text = summary.events.iter()
        .map(|e| {
            let time = e.start.format("%H:%M").to_string();
            let dur = e.duration_minutes().map(|m| format!(" ({} Min)", m)).unwrap_or_default();
            format!("- {} Uhr: {}{}", time, e.title, dur)
        })
        .collect::<Vec<_>>().join("\n");

    let tasks_text = summary.priority_tasks.iter()
        .map(|t| {
            let due = t.due_date.map(|d| format!(", fällig: {}", d.format("%d.%m."))).unwrap_or_default();
            format!("- {} [{:?}]{}", t.title, t.priority, due)
        })
        .collect::<Vec<_>>().join("\n");

    let overdue = if summary.tasks_overdue.is_empty() {
        String::new()
    } else {
        format!("\n\nÜBERFÄLLIG:\n{}", summary.tasks_overdue.iter()
            .map(|t| format!("- {} (seit {})", t.title,
                t.due_date.map(|d| d.format("%d.%m.").to_string()).unwrap_or_default()))
            .collect::<Vec<_>>().join("\n"))
    };

    let conflicts = if summary.conflicts.is_empty() {
        String::new()
    } else {
        format!("\n\nKONFLIKTE:\n{}", summary.conflicts.iter()
            .map(|c| format!("- \"{}\" und \"{}\" überschneiden sich {} Min.",
                c.event_a.title, c.event_b.title, c.overlap_minutes))
            .collect::<Vec<_>>().join("\n"))
    };

    format!(
        r#"Du bist ein persönlicher Planer-Assistent. Erstelle eine kurze, motivierende Tagesübersicht auf Deutsch.

HEUTE:
Termine:
{}

Prioritäre Aufgaben:
{}{}{}

Freie Zeitfenster: {} verfügbar

Antworte in maximal 5 kurzen Sätzen: Was steht heute an, was ist wichtig, und gibt es Probleme die sofort beachtet werden müssen? Sei direkt und klar."#,
        if events_text.is_empty() { "Keine Termine heute".to_string() } else { events_text },
        if tasks_text.is_empty() { "Keine offenen Aufgaben".to_string() } else { tasks_text },
        overdue,
        conflicts,
        summary.free_slots.len()
    )
}

pub fn classify_task_prompt(task_title: &str, description: &str) -> String {
    format!(
        r#"Klassifiziere diese Aufgabe und antworte NUR mit JSON ohne Erklärung:

Aufgabe: {}
Beschreibung: {}

JSON-Format:
{{"priority": "critical|high|medium|low|someday", "energy_level": "high|medium|low", "estimated_minutes": 30, "tags": ["tag1", "tag2"]}}

- energy_level high = Fokus/Tiefarbeit, medium = kreativ/Meeting, low = Routine/Admin"#,
        task_title,
        if description.is_empty() { "—" } else { description }
    )
}

pub fn extract_events_prompt(text: &str) -> String {
    format!(
        r#"Extrahiere Termine und Aufgaben aus dem folgenden Text. Antworte NUR mit JSON:

TEXT:
{}

JSON-Format:
{{"events": [{{"title": "...", "start": "2026-06-12T14:00:00Z", "duration_minutes": 60, "location": "..."}}], "tasks": [{{"title": "...", "due_date": "2026-06-15T00:00:00Z", "priority": "medium"}}]}}"#,
        &text[..text.len().min(1500)]
    )
}
