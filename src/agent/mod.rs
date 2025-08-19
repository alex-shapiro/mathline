use crate::{MLResult, agent::ollama::OllamaClient};

mod claude;
mod ollama;
mod openai;

static SYSTEM_PROMPT: &str = "Transform the following request into a mathematical expression. Do not attempt to solve the expression. Use standard ASCII.";

#[async_trait::async_trait]
pub trait AgentClient {
    async fn messages<'a>(&self, prompt: Prompt<'a>) -> MLResult<String>;
}

pub struct Prompt<'a> {
    pub system_prompt: &'a str,
    pub user_request: &'a str,
}

pub async fn call_agent(user_request: &str) -> MLResult<String> {
    let client = OllamaClient::gemma3_4b();

    client
        .messages(Prompt {
            system_prompt: SYSTEM_PROMPT,
            user_request,
        })
        .await
}
