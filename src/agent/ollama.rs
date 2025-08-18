use crate::{
    MLResult,
    agent::{AgentClient, Prompt},
};
use serde::{Deserialize, Serialize};

static BASE_URL: &str = "http://localhost:11434";

pub struct OllamaClient {
    model: String,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        Self { model }
    }

    pub fn gemma3_4b() -> Self {
        Self::new("gemma3:4b".to_string())
    }
}

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    system: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait::async_trait]
impl AgentClient for OllamaClient {
    async fn messages<'a>(&self, prompt: Prompt<'a>) -> MLResult<String> {
        let http_client = reqwest::Client::new();

        let request = OllamaRequest {
            model: &self.model,
            prompt: prompt.user_request,
            system: prompt.system_prompt,
            stream: false,
        };

        let response = http_client
            .post(format!("{BASE_URL}/api/generate"))
            .json(&request)
            .send()
            .await?;

        let ollama_response: OllamaResponse = response.json().await?;

        Ok(ollama_response.response.trim().to_string())
    }
}
