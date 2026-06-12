use crate::{error::LpResult, state::AppState};
use lp_core::{db::queries::*, models::{Task, TaskStatus}};
use chrono::Utc;
use tauri::State;

#[tauri::command]
pub async fn get_tasks_cmd(
    include_done: bool,
    state: State<'_, AppState>,
) -> LpResult<Vec<Task>> {
    Ok(get_tasks(&state.db, include_done).await?)
}

#[tauri::command]
pub async fn create_task(task: Task, state: State<'_, AppState>) -> LpResult<Task> {
    insert_task(&state.db, &task).await?;
    Ok(task)
}

#[tauri::command]
pub async fn update_task(task: Task, state: State<'_, AppState>) -> LpResult<Task> {
    let mut updated = task;
    updated.updated_at = Utc::now();
    insert_task(&state.db, &updated).await?;
    Ok(updated)
}

#[tauri::command]
pub async fn set_task_status(
    id: String,
    status: TaskStatus,
    state: State<'_, AppState>,
) -> LpResult<()> {
    update_task_status(&state.db, &id, &status).await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_task_cmd(id: String, state: State<'_, AppState>) -> LpResult<()> {
    delete_task(&state.db, &id).await?;
    Ok(())
}
