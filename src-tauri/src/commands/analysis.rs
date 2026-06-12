use crate::{error::LpResult, state::AppState};
use lp_core::{
    analyzer::*,
    db::queries::*,
};
use chrono::{Duration, Utc};
use tauri::State;

#[tauri::command]
pub async fn get_daily_summary(state: State<'_, AppState>) -> LpResult<DailySummary> {
    let now = Utc::now();
    let from = now.date_naive().and_hms_opt(0, 0, 0)
        .and_then(|n| chrono::Utc.from_local_datetime(&n).single())
        .unwrap_or(now);
    let to = from + Duration::days(1);
    let events = get_events_in_range(&state.db, from, to).await?;
    let tasks = get_tasks(&state.db, false).await?;
    let settings = state.settings.read().await.clone();
    Ok(build_daily_summary(events, tasks, &settings))
}

#[tauri::command]
pub async fn get_conflicts(
    from: String,
    to: String,
    state: State<'_, AppState>,
) -> LpResult<Vec<EventConflict>> {
    let from = from.parse().map_err(|e: chrono::ParseError| crate::error::LpError::Other(e.to_string()))?;
    let to   = to.parse().map_err(|e: chrono::ParseError| crate::error::LpError::Other(e.to_string()))?;
    let events = get_events_in_range(&state.db, from, to).await?;
    Ok(find_conflicts(&events))
}

#[tauri::command]
pub async fn get_free_slots(state: State<'_, AppState>) -> LpResult<Vec<TimeSlot>> {
    let now = Utc::now();
    let from = now.date_naive().and_hms_opt(0, 0, 0)
        .and_then(|n| chrono::Utc.from_local_datetime(&n).single())
        .unwrap_or(now);
    let to = from + Duration::days(1);
    let events = get_events_in_range(&state.db, from, to).await?;
    let settings = state.settings.read().await.clone();
    Ok(find_free_slots(&events, &settings))
}

#[tauri::command]
pub async fn get_patterns(state: State<'_, AppState>) -> LpResult<Vec<RecurringPattern>> {
    let events = get_all_events(&state.db).await?;
    Ok(detect_patterns(&events))
}

#[tauri::command]
pub async fn search(query: String, state: State<'_, AppState>) -> LpResult<SearchResults> {
    Ok(search_all(&state.db, &query).await?)
}
