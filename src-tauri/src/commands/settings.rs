use crate::{error::LpResult, state::AppState};
use lp_core::{db::queries::*, models::AppSettings};
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> LpResult<AppSettings> {
    Ok(state.settings.read().await.clone())
}

#[tauri::command]
pub async fn save_settings_cmd(
    settings: AppSettings,
    state: State<'_, AppState>,
) -> LpResult<()> {
    save_settings(&state.db, &settings).await?;
    *state.settings.write().await = settings;
    Ok(())
}
