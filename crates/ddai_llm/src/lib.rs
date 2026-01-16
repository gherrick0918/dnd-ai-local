use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OllamaClient {
    pub host: String,  // e.g. http://127.0.0.1:11434
    pub model: String, // e.g. llama3.1:8b
}

impl OllamaClient {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            model: model.into(),
        }
    }

    pub fn generate(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.host.trim_end_matches('/'));
        let client = reqwest::blocking::Client::new();

        let req = GenerateRequest {
            model: &self.model,
            prompt,
            stream: false,
        };

        let resp: GenerateResponse = client
            .post(&url)
            .json(&req)
            .send()
            .with_context(|| format!("POST {url}"))?
            .error_for_status()
            .with_context(|| format!("HTTP error from {url}"))?
            .json()
            .context("failed to parse ollama response json")?;

        Ok(resp.response)
    }
}

#[derive(Debug, Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
