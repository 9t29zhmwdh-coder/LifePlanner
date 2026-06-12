use crate::{error::LpResult, state::AppState};
use lp_core::{db::queries::*, models::Project};
use chrono::Utc;
use tauri::State;

#[tauri::command]
pub async fn get_projects_cmd(state: State<'_, AppState>) -> LpResult<Vec<Project>> {
    Ok(get_projects(&state.db).await?)
}

#[tauri::command]
pub async fn create_project(project: Project, state: State<'_, AppState>) -> LpResult<Project> {
    insert_project(&state.db, &project).await?;
    Ok(project)
}

#[tauri::command]
pub async fn update_project(project: Project, state: State<'_, AppState>) -> LpResult<Project> {
    let mut updated = project;
    updated.updated_at = Utc::now();
    insert_project(&state.db, &updated).await?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_project_cmd(id: String, state: State<'_, AppState>) -> LpResult<()> {
    delete_project(&state.db, &id).await?;
    Ok(())
}
