pub mod date_parser;
pub mod text;
pub mod email;

use crate::models::{Event, Task};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub events: Vec<Event>,
    pub tasks: Vec<Task>,
    pub source_text: String,
}

impl ExtractionResult {
    pub fn is_empty(&self) -> bool {
        self.events.is_empty() && self.tasks.is_empty()
    }
}

pub use text::extract_from_text;
pub use email::extract_from_email;
