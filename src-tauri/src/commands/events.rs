use crate::{error::LpResult, state::AppState};
use lp_core::{
    db::queries::*,
    models::{Event, EventStatus},
};
use chrono::{DateTime, Utc};
use tauri::State;

#[tauri::command]
pub async fn get_events(
    from: String,
    to: String,
    state: State<'_, AppState>,
) -> LpResult<Vec<Event>> {
    let from: DateTime<Utc> = from.parse().map_err(|e: chrono::ParseError| {
        crate::error::LpError::Other(e.to_string())
    })?;
    let to: DateTime<Utc> = to.parse().map_err(|e: chrono::ParseError| {
        crate::error::LpError::Other(e.to_string())
    })?;
    Ok(get_events_in_range(&state.db, from, to).await?)
}

#[tauri::command]
pub async fn get_all_events_cmd(state: State<'_, AppState>) -> LpResult<Vec<Event>> {
    Ok(get_all_events(&state.db).await?)
}

#[tauri::command]
pub async fn create_event(event: Event, state: State<'_, AppState>) -> LpResult<Event> {
    insert_event(&state.db, &event).await?;
    Ok(event)
}

#[tauri::command]
pub async fn update_event(event: Event, state: State<'_, AppState>) -> LpResult<Event> {
    let mut updated = event;
    updated.updated_at = Utc::now();
    insert_event(&state.db, &updated).await?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_event_cmd(id: String, state: State<'_, AppState>) -> LpResult<()> {
    delete_event(&state.db, &id).await?;
    Ok(())
}
