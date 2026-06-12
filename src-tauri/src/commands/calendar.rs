use crate::{error::{LpError, LpResult}, state::AppState};
use lp_core::{
    calendar::{accounts::*, parse_ics_file, sync_caldav},
    db::queries::insert_event,
    models::CalendarAccount,
};
use std::path::Path;
use tauri::State;

#[tauri::command]
pub async fn sync_ics_file(path: String, account_id: String, state: State<'_, AppState>) -> LpResult<usize> {
    let events = parse_ics_file(Path::new(&path))
        .map_err(|e| LpError::Calendar(e.to_string()))?;
    let count = events.len();
    for mut ev in events {
        ev.calendar_id = Some(account_id.clone());
        insert_event(&state.db, &ev).await?;
    }
    Ok(count)
}

#[tauri::command]
pub async fn sync_caldav_account(account_id: String, state: State<'_, AppState>) -> LpResult<usize> {
    let settings = state.settings.read().await;
    let account = settings.calendar_accounts.iter()
        .find(|a| a.id == account_id)
        .cloned()
        .ok_or_else(|| LpError::Other("Account not found".into()))?;
    drop(settings);

    let password = get_password(&account_id)
        .map_err(|e| LpError::Calendar(e.to_string()))?
        .unwrap_or_default();

    let events = sync_caldav(&account, &password).await
        .map_err(|e| LpError::Calendar(e.to_string()))?;

    let count = events.len();
    for ev in events {
        insert_event(&state.db, &ev).await?;
    }
    Ok(count)
}

#[tauri::command]
pub async fn add_calendar_account(
    account: CalendarAccount,
    password: Option<String>,
    state: State<'_, AppState>,
) -> LpResult<()> {
    if let Some(pw) = password {
        store_password(&account.id, &pw)
            .map_err(|e| LpError::Calendar(e.to_string()))?;
    }
    let mut settings = state.settings.write().await;
    settings.calendar_accounts.push(account);
    lp_core::db::queries::save_settings(&state.db, &settings).await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_calendar_account(account_id: String, state: State<'_, AppState>) -> LpResult<()> {
    delete_password(&account_id);
    let mut settings = state.settings.write().await;
    settings.calendar_accounts.retain(|a| a.id != account_id);
    lp_core::db::queries::save_settings(&state.db, &settings).await?;
    Ok(())
}
