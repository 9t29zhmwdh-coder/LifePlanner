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
    Ok(text)
}

pub fn extract_from_pdf(bytes: &[u8]) -> Result<ExtractionResult, PdfError> {
    Ok(extract_from_text(&pdf_text(bytes)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const APPOINTMENT: &[u8] = include_bytes!("../../tests/fixtures/appointment.pdf");

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
    }

    #[test]
    fn garbage_is_refused_not_a_panic() {
        assert!(matches!(extract_from_pdf(b"not a pdf"), Err(PdfError::Unreadable(_))));
    }
}
