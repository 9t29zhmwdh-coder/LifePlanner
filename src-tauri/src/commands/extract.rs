use crate::{error::{LpError, LpResult}, state::AppState};
use lp_core::{
    db::queries::*,
    extractor::{
        extract_from_email, extract_from_text,
        pdf::{extract_from_pdf, PdfError},
        preview::{preview, ExtractionPreview},
        ExtractionResult,
    },
};
use tauri::{ipc::{InvokeBody, Request}, State};

/// Extraction only proposes; nothing is stored until `save_extraction`.
async fn preview_against_calendar(state: &AppState, result: ExtractionResult) -> LpResult<ExtractionPreview> {
    let calendar = get_all_events(&state.db).await?;
    let minutes = state.settings.read().await.default_event_duration_minutes;
    Ok(preview(result, &calendar, minutes))
}

#[tauri::command]
pub async fn extract_text(text: String, state: State<'_, AppState>) -> LpResult<ExtractionPreview> {
    preview_against_calendar(&state, extract_from_text(&text)).await
}

#[tauri::command]
pub async fn extract_email(email_text: String, state: State<'_, AppState>) -> LpResult<ExtractionPreview> {
    preview_against_calendar(&state, extract_from_email(&email_text)).await
}

/// The PDF arrives as a raw IPC body; a JSON number array would be four times its size.
#[tauri::command]
pub async fn extract_pdf(request: Request<'_>, state: State<'_, AppState>) -> LpResult<ExtractionPreview> {
    let InvokeBody::Raw(bytes) = request.body() else {
        return Err(LpError::Other("Expected the PDF as binary data".into()));
    };
    let result = extract_from_pdf(bytes).map_err(|e| LpError::Other(pdf_message(&e)))?;
    preview_against_calendar(&state, result).await
}

fn pdf_message(error: &PdfError) -> String {
    match error {
        PdfError::TooLarge => "pdfTooLarge",
        PdfError::Unreadable(_) => "pdfUnreadable",
        PdfError::NoText => "pdfNoText",
    }
    .into()
}

/// Stores what the person kept from a preview. Returns how many items were saved.
#[tauri::command]
pub async fn save_extraction(result: ExtractionResult, state: State<'_, AppState>) -> LpResult<usize> {
    for event in &result.events {
        insert_event(&state.db, event).await?;
    }
    for task in &result.tasks {
        insert_task(&state.db, task).await?;
    }
    Ok(result.events.len() + result.tasks.len())
}
