use crate::{error::{LpError, LpResult}, state::AppState};
use lp_core::{
    calendar::{accounts::*, parse_ics_file, sync_caldav},
    db::queries::replace_calendar_events,
    models::{CalendarAccount, CalendarKind},
};
use chrono::Utc;
use std::path::Path;
use tauri::State;

/// One sync button for both kinds: the account knows its file or server.
#[tauri::command]
pub async fn sync_calendar(account_id: String, state: State<'_, AppState>) -> LpResult<usize> {
    let account = state.settings.read().await.calendar_accounts.iter()
        .find(|a| a.id == account_id)
        .cloned()
        .ok_or_else(|| LpError::Other("Account not found".into()))?;

    let events = match account.kind {
        CalendarKind::IcsFile => {
            let path = account.ics_path.as_deref()
                .ok_or_else(|| LpError::Calendar("No ICS file chosen".into()))?;
            parse_ics_file(Path::new(path)).map_err(|e| LpError::Calendar(e.to_string()))?
        }
        CalendarKind::CalDav => {
            let password = get_password(&account_id)
                .map_err(|e| LpError::Calendar(e.to_string()))?
                .unwrap_or_default();
            sync_caldav(&account, &password).await.map_err(|e| LpError::Calendar(e.to_string()))?
        }
        CalendarKind::Local => return Ok(0),
    };

    let count = replace_calendar_events(&state.db, &account_id, events).await?;
    let mut settings = state.settings.write().await;
    if let Some(stored) = settings.calendar_accounts.iter_mut().find(|a| a.id == account_id) {
        stored.last_synced = Some(Utc::now());
    }
    lp_core::db::queries::save_settings(&state.db, &settings).await?;
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
