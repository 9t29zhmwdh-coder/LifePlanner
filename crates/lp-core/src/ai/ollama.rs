use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Ollama not available at {0}")]
    Unavailable(String),
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    // Qwen models reason out loud by default; that text would land in the answer.
    think: bool,
    options: GenerateOptions,
}

/// Without num_ctx Ollama reserves the model's full context window (LifeSort saw
/// 12.5 GB instead of 4); a low temperature keeps extraction repeatable.
#[derive(Serialize)]
struct GenerateOptions {
    num_ctx: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

pub struct OllamaClient {
    base_url: String,
    model: String,
    client: Client,
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap(),
        }
    }

    pub async fn available(&self) -> bool {
        self.client.get(format!("{}/api/tags", self.base_url))
            .send().await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub async fn generate(&self, prompt: &str) -> Result<String, OllamaError> {
        let resp = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&GenerateRequest {
                model: &self.model,
                prompt,
                stream: false,
                think: false,
                options: GenerateOptions { num_ctx: 4096, temperature: 0.1 },
            })
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(OllamaError::Unavailable(self.base_url.clone()));
        }

        let body: GenerateResponse = resp.json().await?;
        Ok(body.response.trim().to_string())
    }
}
