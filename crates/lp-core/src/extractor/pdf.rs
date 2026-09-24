//! Text out of a PDF, then the same date and task extraction as pasted text.
//! Runs locally; the PDF never leaves the machine.

use super::{text::extract_from_text, ExtractionResult};

/// PDFs larger than this are refused instead of stalling the app.
pub const MAX_PDF_BYTES: usize = 25 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error("the PDF is larger than 25 MB")]
    TooLarge,
    #[error("the PDF could not be read: {0}")]
    Unreadable(String),
    #[error("the PDF contains no text; scanned pages would need OCR")]
    NoText,
}

pub fn pdf_text(bytes: &[u8]) -> Result<String, PdfError> {
    if bytes.len() > MAX_PDF_BYTES {
        return Err(PdfError::TooLarge);
    }
    let text = pdf_extract::extract_text_from_mem(bytes).map_err(|e| PdfError::Unreadable(e.to_string()))?;
    if text.trim().is_empty() {
        return Err(PdfError::NoText);
    }
    Ok(join_split_capitals(&text))
}

/// PDF text often keeps a kerned capital apart from its word ("T erminbestätigung").
/// Single capitals that are never words on their own are joined back.
fn join_split_capitals(text: &str) -> String {
    static SPLIT: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
        regex::Regex::new(r"\b([B-HJ-NP-ZÄÖÜ]) (\p{Ll}{2,})").unwrap()
    });
    SPLIT.replace_all(text, "$1$2").into_owned()
}

pub fn extract_from_pdf(bytes: &[u8]) -> Result<ExtractionResult, PdfError> {
    Ok(extract_from_text(&pdf_text(bytes)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const APPOINTMENT: &[u8] = include_bytes!("../../tests/fixtures/appointment.pdf");

    #[test]
    fn split_capitals_are_joined_but_real_words_stay() {
        assert_eq!(join_split_capitals("T erminbestätigung"), "Terminbestätigung");
        assert_eq!(join_split_capitals("A test and I think"), "A test and I think");
    }

    #[test]
    fn reads_the_text_of_a_pdf() {
        let text = pdf_text(APPOINTMENT).unwrap();
        assert!(text.contains("Zahnarzt"), "{text}");
        assert!(text.contains("12.10.2026"), "{text}");
    }

    #[test]
    fn finds_the_appointment_in_a_pdf() {
        let result = extract_from_pdf(APPOINTMENT).unwrap();
        assert_eq!(result.events.len(), 1, "{result:?}");
        let start = result.events[0].start.to_rfc3339();
        assert!(start.starts_with("2026-10-12"), "{start}");
        assert_eq!(result.events[0].title, "Ihr Termin beim Zahnarzt");
        assert_eq!(result.events[0].location.as_deref(), Some("Zahnarzt"));
    }

    #[test]
    fn garbage_is_refused_not_a_panic() {
        assert!(matches!(extract_from_pdf(b"not a pdf"), Err(PdfError::Unreadable(_))));
    }
}
