use super::Database;
use crate::models::*;
use chrono::{DateTime, Utc};
use sqlx::Row;

// ─── Events ─────────────────────────────────────────────────────────────────

pub async fn insert_event(db: &Database, e: &Event) -> super::DbResult<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO events
         (id,title,description,start,end,all_day,location,source,calendar_id,
          external_uid,status,recurrence,linked_task_ids,linked_doc_ids,tags,created_at,updated_at)
         VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"
    )
    .bind(&e.id)
    .bind(&e.title)
    .bind(&e.description)
    .bind(e.start.to_rfc3339())
    .bind(e.end.map(|d| d.to_rfc3339()))
    .bind(e.all_day as i64)
    .bind(&e.location)
    .bind(serde_json::to_string(&e.source).unwrap_or_default())
    .bind(&e.calendar_id)
    .bind(&e.external_uid)
    .bind(serde_json::to_string(&e.status).unwrap_or_default())
    .bind(e.recurrence.as_ref().and_then(|r| serde_json::to_string(r).ok()))
    .bind(serde_json::to_string(&e.linked_task_ids).unwrap_or_default())
    .bind(serde_json::to_string(&e.linked_doc_ids).unwrap_or_default())
    .bind(serde_json::to_string(&e.tags).unwrap_or_default())
    .bind(e.created_at.to_rfc3339())
    .bind(e.updated_at.to_rfc3339())
    .execute(&db.pool)
    .await?;
    Ok(())
}

pub async fn get_events_in_range(
    db: &Database,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> super::DbResult<Vec<Event>> {
    let rows = sqlx::query(
        "SELECT * FROM events WHERE start >= ? AND start <= ? AND status != 'cancelled'
         ORDER BY start ASC"
    )
    .bind(from.to_rfc3339())
    .bind(to.to_rfc3339())
    .fetch_all(&db.pool)
    .await?;
    Ok(rows.iter().filter_map(row_to_event).collect())
}

pub async fn get_all_events(db: &Database) -> super::DbResult<Vec<Event>> {
    let rows = sqlx::query("SELECT * FROM events ORDER BY start ASC")
        .fetch_all(&db.pool)
        .await?;
    Ok(rows.iter().filter_map(row_to_event).collect())
}

pub async fn delete_event(db: &Database, id: &str) -> super::DbResult<()> {
    sqlx::query("DELETE FROM events WHERE id = ?")
        .bind(id).execute(&db.pool).await?;
    Ok(())
}

fn row_to_event(row: &sqlx::sqlite::SqliteRow) -> Option<Event> {
    let start: String = row.try_get("start").ok()?;
    Some(Event {
        id: row.try_get("id").ok()?,
        title: row.try_get("title").ok()?,
        description: row.try_get("description").ok(),
        start: start.parse::<DateTime<Utc>>().ok()?,
        end: row.try_get::<Option<String>, _>("end").ok()?
            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        all_day: row.try_get::<i64, _>("all_day").ok().map(|v| v != 0).unwrap_or(false),
        location: row.try_get("location").ok(),
        source: row.try_get::<String, _>("source").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(EventSource::Manual),
        calendar_id: row.try_get("calendar_id").ok(),
        external_uid: row.try_get("external_uid").ok(),
        status: row.try_get::<String, _>("status").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(EventStatus::Confirmed),
        recurrence: row.try_get::<Option<String>, _>("recurrence").ok()?
            .and_then(|s| serde_json::from_str(&s).ok()),
        linked_task_ids: row.try_get::<String, _>("linked_task_ids").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        linked_doc_ids: row.try_get::<String, _>("linked_doc_ids").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        tags: row.try_get::<String, _>("tags").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        created_at: row.try_get::<String, _>("created_at").ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(Utc::now),
        updated_at: row.try_get::<String, _>("updated_at").ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(Utc::now),
    })
}

// ─── Tasks ──────────────────────────────────────────────────────────────────

pub async fn insert_task(db: &Database, t: &Task) -> super::DbResult<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO tasks
         (id,title,description,due_date,priority,energy_level,status,project_id,
          estimated_minutes,linked_event_ids,tags,source,created_at,updated_at)
         VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)"
    )
    .bind(&t.id)
    .bind(&t.title)
    .bind(&t.description)
    .bind(t.due_date.map(|d| d.to_rfc3339()))
    .bind(serde_json::to_string(&t.priority).unwrap_or_default())
    .bind(serde_json::to_string(&t.energy_level).unwrap_or_default())
    .bind(serde_json::to_string(&t.status).unwrap_or_default())
    .bind(&t.project_id)
    .bind(t.estimated_minutes.map(|m| m as i64))
    .bind(serde_json::to_string(&t.linked_event_ids).unwrap_or_default())
    .bind(serde_json::to_string(&t.tags).unwrap_or_default())
    .bind(serde_json::to_string(&t.source).unwrap_or_default())
    .bind(t.created_at.to_rfc3339())
    .bind(t.updated_at.to_rfc3339())
    .execute(&db.pool)
    .await?;
    Ok(())
}

pub async fn get_tasks(db: &Database, include_done: bool) -> super::DbResult<Vec<Task>> {
    let query = if include_done {
        "SELECT * FROM tasks ORDER BY due_date ASC, priority DESC"
    } else {
        "SELECT * FROM tasks WHERE status != 'done' AND status != 'cancelled'
         ORDER BY due_date ASC, priority DESC"
    };
    let rows = sqlx::query(query).fetch_all(&db.pool).await?;
    Ok(rows.iter().filter_map(row_to_task).collect())
}

pub async fn update_task_status(
    db: &Database, id: &str, status: &TaskStatus,
) -> super::DbResult<()> {
    sqlx::query("UPDATE tasks SET status = ?, updated_at = ? WHERE id = ?")
        .bind(serde_json::to_string(status).unwrap_or_default())
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&db.pool).await?;
    Ok(())
}

pub async fn delete_task(db: &Database, id: &str) -> super::DbResult<()> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id).execute(&db.pool).await?;
    Ok(())
}

fn row_to_task(row: &sqlx::sqlite::SqliteRow) -> Option<Task> {
    Some(Task {
        id: row.try_get("id").ok()?,
        title: row.try_get("title").ok()?,
        description: row.try_get("description").ok(),
        due_date: row.try_get::<Option<String>, _>("due_date").ok()?
            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        priority: row.try_get::<String, _>("priority").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(TaskPriority::Medium),
        energy_level: row.try_get::<String, _>("energy_level").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(EnergyLevel::Medium),
        status: row.try_get::<String, _>("status").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(TaskStatus::Todo),
        project_id: row.try_get("project_id").ok(),
        estimated_minutes: row.try_get::<Option<i64>, _>("estimated_minutes").ok()?
            .map(|v| v as u32),
        linked_event_ids: row.try_get::<String, _>("linked_event_ids").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        tags: row.try_get::<String, _>("tags").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        source: row.try_get::<String, _>("source").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(TaskSource::Manual),
        created_at: row.try_get::<String, _>("created_at").ok()
            .and_then(|s| s.parse().ok()).unwrap_or_else(Utc::now),
        updated_at: row.try_get::<String, _>("updated_at").ok()
            .and_then(|s| s.parse().ok()).unwrap_or_else(Utc::now),
    })
}

// ─── Projects ───────────────────────────────────────────────────────────────

pub async fn insert_project(db: &Database, p: &Project) -> super::DbResult<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO projects
         (id,title,description,status,deadline,task_ids,event_ids,tags,
          auto_detected,color,created_at,updated_at)
         VALUES (?,?,?,?,?,?,?,?,?,?,?,?)"
    )
    .bind(&p.id).bind(&p.title).bind(&p.description)
    .bind(serde_json::to_string(&p.status).unwrap_or_default())
    .bind(p.deadline.map(|d| d.to_rfc3339()))
    .bind(serde_json::to_string(&p.task_ids).unwrap_or_default())
    .bind(serde_json::to_string(&p.event_ids).unwrap_or_default())
    .bind(serde_json::to_string(&p.tags).unwrap_or_default())
    .bind(p.auto_detected as i64)
    .bind(&p.color)
    .bind(p.created_at.to_rfc3339())
    .bind(p.updated_at.to_rfc3339())
    .execute(&db.pool).await?;
    Ok(())
}

pub async fn get_projects(db: &Database) -> super::DbResult<Vec<Project>> {
    let rows = sqlx::query(
        "SELECT * FROM projects WHERE status != 'cancelled' ORDER BY created_at DESC"
    ).fetch_all(&db.pool).await?;
    Ok(rows.iter().filter_map(row_to_project).collect())
}

pub async fn delete_project(db: &Database, id: &str) -> super::DbResult<()> {
    sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(id).execute(&db.pool).await?;
    Ok(())
}

fn row_to_project(row: &sqlx::sqlite::SqliteRow) -> Option<Project> {
    Some(Project {
        id: row.try_get("id").ok()?,
        title: row.try_get("title").ok()?,
        description: row.try_get("description").ok(),
        status: row.try_get::<String, _>("status").ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(ProjectStatus::Active),
        deadline: row.try_get::<Option<String>, _>("deadline").ok()?
            .and_then(|s| s.parse().ok()),
        task_ids: row.try_get::<String, _>("task_ids").ok()
            .and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
        event_ids: row.try_get::<String, _>("event_ids").ok()
            .and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
        tags: row.try_get::<String, _>("tags").ok()
            .and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
        auto_detected: row.try_get::<i64, _>("auto_detected").ok().map(|v| v != 0).unwrap_or(false),
        color: row.try_get("color").ok().unwrap_or_else(|| "#58a6ff".into()),
        created_at: row.try_get::<String, _>("created_at").ok()
            .and_then(|s| s.parse().ok()).unwrap_or_else(Utc::now),
        updated_at: row.try_get::<String, _>("updated_at").ok()
            .and_then(|s| s.parse().ok()).unwrap_or_else(Utc::now),
    })
}

// ─── Settings ───────────────────────────────────────────────────────────────

pub async fn load_settings(db: &Database) -> super::DbResult<AppSettings> {
    let row = sqlx::query("SELECT value FROM settings WHERE key = 'app_settings'")
        .fetch_optional(&db.pool).await?;
    Ok(row
        .and_then(|r| r.try_get::<String, _>("value").ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default())
}

pub async fn save_settings(db: &Database, s: &AppSettings) -> super::DbResult<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_settings', ?)"
    )
    .bind(serde_json::to_string(s).unwrap_or_default())
    .execute(&db.pool).await?;
    Ok(())
}

// ─── Full-text search ────────────────────────────────────────────────────────

pub async fn search_all(db: &Database, query: &str) -> super::DbResult<SearchResults> {
    let q = format!("{}*", query.replace('"', ""));

    let event_ids: Vec<String> = sqlx::query(
        "SELECT id FROM events_fts WHERE events_fts MATCH ? LIMIT 20"
    )
    .bind(&q)
    .fetch_all(&db.pool).await?
    .iter()
    .filter_map(|r| r.try_get::<String, _>("id").ok())
    .collect();

    let task_ids: Vec<String> = sqlx::query(
        "SELECT id FROM tasks_fts WHERE tasks_fts MATCH ? LIMIT 20"
    )
    .bind(&q)
    .fetch_all(&db.pool).await?
    .iter()
    .filter_map(|r| r.try_get::<String, _>("id").ok())
    .collect();

    Ok(SearchResults { event_ids, task_ids })
}

#[derive(Debug, serde::Serialize)]
pub struct SearchResults {
    pub event_ids: Vec<String>,
    pub task_ids: Vec<String>,
}
