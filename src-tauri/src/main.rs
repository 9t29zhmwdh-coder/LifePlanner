#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod state;

use commands::{ai::*, analysis::*, calendar::*, events::*, extract::*, projects::*, settings::*, tasks::*};
use lp_core::db::Database;
use state::AppState;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
fn db_path() -> PathBuf {
    let mut p = dirs::home_dir().unwrap_or_default();
    p.push("Library/Application Support/LifePlanner/lifeplanner.db");
    p
}
#[cfg(target_os = "linux")]
fn db_path() -> PathBuf {
    let mut p = dirs::data_dir().unwrap_or_default();
    p.push("LifePlanner/lifeplanner.db");
    p
}
#[cfg(target_os = "windows")]
fn db_path() -> PathBuf {
    let mut p = dirs::data_dir().unwrap_or_default();
    p.push("LifePlanner\\lifeplanner.db");
    p
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Database::open(&db_path())
        .await
        .expect("Failed to open database");

    let state = AppState::new(db).await;

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            // events
            get_events, get_all_events_cmd, create_event, update_event, delete_event_cmd,
            // tasks
            get_tasks_cmd, create_task, update_task, set_task_status, delete_task_cmd,
            // projects
            get_projects_cmd, create_project, update_project, delete_project_cmd,
            // calendar
            sync_ics_file, sync_caldav_account, add_calendar_account, remove_calendar_account,
            // extract
            extract_text, extract_email,
            // analysis
            get_daily_summary, get_conflicts, get_free_slots, get_patterns, search,
            // ai
            check_ollama, generate_daily_summary_ai, ai_classify_task, ai_extract_from_text,
            // settings
            get_settings, save_settings_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
