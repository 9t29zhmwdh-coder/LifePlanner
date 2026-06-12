use crate::{error::LpResult, state::AppState};
use lp_core::{
    db::queries::*,
    extractor::{extract_from_email, extract_from_text, ExtractionResult},
};
use tauri::State;

#[tauri::command]
pub async fn extract_text(text: String, state: State<'_, AppState>) -> LpResult<ExtractionResult> {
    let result = extract_from_text(&text);
    for ev in &result.events {
        insert_event(&state.db, ev).await?;
    }
    for task in &result.tasks {
        insert_task(&state.db, task).await?;
    }
    Ok(result)
}

#[tauri::command]
pub async fn extract_email(email_text: String, state: State<'_, AppState>) -> LpResult<ExtractionResult> {
    let result = extract_from_email(&email_text);
    for ev in &result.events {
        insert_event(&state.db, ev).await?;
    }
    for task in &result.tasks {
        insert_task(&state.db, task).await?;
    }
    Ok(result)
}
