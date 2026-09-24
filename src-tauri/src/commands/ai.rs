use crate::{error::{LpError, LpResult}, state::AppState};
use lp_core::{
    ai::{ollama::OllamaClient, prompts::*},
    analyzer::build_daily_summary,
    db::queries::*,
    extractor::preview::{from_model_answer, preview, ExtractionPreview},
};
use chrono::Utc;
use tauri::State;

#[tauri::command]
pub async fn check_ollama(state: State<'_, AppState>) -> LpResult<bool> {
    let settings = state.settings.read().await;
    let client = OllamaClient::new(&settings.ollama_url, &settings.text_model);
    Ok(client.available().await)
}

#[tauri::command]
pub async fn generate_daily_summary_ai(state: State<'_, AppState>) -> LpResult<String> {
    let settings = state.settings.read().await.clone();
    let client = OllamaClient::new(&settings.ollama_url, &settings.text_model);

    if !client.available().await {
        return Err(LpError::Ai("Ollama nicht verfügbar".into()));
    }

    let (from, to) = lp_core::analyzer::today_range();
    let events = get_events_in_range(&state.db, from, to).await?;
    let tasks  = get_tasks(&state.db, false).await?;
    let summary = build_daily_summary(events, tasks, &settings);

    let prompt = daily_summary_prompt(&summary);
    let response = client.generate(&prompt).await
        .map_err(|e| LpError::Ai(e.to_string()))?;
    Ok(response)
}

#[tauri::command]
pub async fn ai_classify_task(
    title: String,
    description: String,
    state: State<'_, AppState>,
) -> LpResult<serde_json::Value> {
    let settings = state.settings.read().await.clone();
    let client = OllamaClient::new(&settings.ollama_url, &settings.text_model);

    if !client.available().await {
        return Err(LpError::Ai("Ollama nicht verfügbar".into()));
    }

    let prompt = classify_task_prompt(&title, &description);
    let response = client.generate(&prompt).await
        .map_err(|e| LpError::Ai(e.to_string()))?;

    // Extract JSON from response
    let json_str = response.trim();
    let start = json_str.find('{').unwrap_or(0);
    let end = json_str.rfind('}').map(|i| i + 1).unwrap_or(json_str.len());
    serde_json::from_str(&json_str[start..end])
        .map_err(|e| LpError::Ai(format!("JSON parse error: {}", e)))
}

#[tauri::command]
pub async fn ai_extract_from_text(
    text: String,
    state: State<'_, AppState>,
) -> LpResult<ExtractionPreview> {
    let settings = state.settings.read().await.clone();
    let client = OllamaClient::new(&settings.ollama_url, &settings.text_model);

    if !client.available().await {
        return Err(LpError::Ai("Ollama nicht verfügbar".into()));
    }

    let prompt = extract_events_prompt(&text, Utc::now());
    let response = client.generate(&prompt).await
        .map_err(|e| LpError::Ai(e.to_string()))?;
    let result = from_model_answer(&response, &text)
        .map_err(|e| LpError::Ai(format!("JSON parse error: {}", e)))?;
    let calendar = get_all_events(&state.db).await?;
    Ok(preview(result, &calendar, settings.default_event_duration_minutes))
}
