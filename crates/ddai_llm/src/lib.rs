use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub struct OllamaClient {
    host: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<&'a str>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Debug, Deserialize)]
struct ModelInfo {
    name: String,
    #[serde(default)]
    size: i64,
}

#[derive(Debug, Deserialize)]
struct ModelsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CitationAnswer {
    pub answer: String,
    pub citations: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followup_question: Option<String>,
}

impl OllamaClient {
    pub fn new(host: String, model: String) -> Self {
        Self { host, model }
    }

    pub async fn list_models(&self) -> Result<Vec<(String, i64)>> {
        let url = format!("{}/api/tags", self.host);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to connect to Ollama at {}", self.host))?;

        if !response.status().is_success() {
            anyhow::bail!("Ollama API error: {}", response.status());
        }

        let models_response: ModelsResponse = response
            .json()
            .await
            .context("Failed to parse models response")?;

        Ok(models_response
            .models
            .into_iter()
            .map(|m| (m.name, m.size))
            .collect())
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        self.generate_with_options(prompt, None, None, None).await
    }

    pub async fn generate_with_options(
        &self,
        prompt: &str,
        system: Option<&str>,
        format: Option<serde_json::Value>,
        options: Option<serde_json::Value>,
    ) -> Result<String> {
        let url = format!("{}/api/generate", self.host);
        let client = reqwest::Client::new();

        let request = GenerateRequest {
            model: &self.model,
            prompt,
            system,
            stream: false,
            format,
            options,
        };

        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await
            .with_context(|| format!("Failed to connect to Ollama at {}", self.host))?;

        if !response.status().is_success() {
            anyhow::bail!("Ollama API error: {}", response.status());
        }

        let generate_response: GenerateResponse = response
            .json()
            .await
            .context("Failed to parse generate response")?;

        Ok(generate_response.response)
    }

    pub async fn generate_with_citations(
        &self,
        prompt: &str,
        system: Option<&str>,
        allowed_citations: &[i64],
    ) -> Result<CitationAnswer> {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "answer": { "type": "string" },
                "citations": { "type": "array", "items": { "type": "integer" } },
                "followup_question": { "type": "string" }
            },
            "required": ["answer", "citations"],
            "additionalProperties": false
        });

        let options = serde_json::json!({
            "temperature": 0
        });

        let response_text = self
            .generate_with_options(prompt, system, Some(schema), Some(options))
            .await?;

        // Parse the JSON response
        let mut citation_answer: CitationAnswer = serde_json::from_str(&response_text)
            .context("Failed to parse JSON response from model")?;

        // Validate citations - only keep those that were in our allowed list
        let invalid_citations: Vec<i64> = citation_answer
            .citations
            .iter()
            .filter(|&id| !allowed_citations.contains(id))
            .copied()
            .collect();

        if !invalid_citations.is_empty() {
            eprintln!(
                "Warning: Model cited unknown chunk IDs: {:?}. Removing from response.",
                invalid_citations
            );
            citation_answer
                .citations
                .retain(|id| allowed_citations.contains(id));
        }

        Ok(citation_answer)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
