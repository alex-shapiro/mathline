use crate::{MLResult, agent::ollama::OllamaClient};
use regex::Regex;

pub mod ollama;

static SYSTEM_PROMPT: &str = "Transform the following request into a mathematical expression. Do not attempt to solve the expression. Use standard, valid Python syntax.";

#[async_trait::async_trait]
pub trait AgentClient {
    async fn messages<'a>(&self, prompt: Prompt<'a>) -> MLResult<String>;
}

pub struct Prompt<'a> {
    pub system_prompt: &'a str,
    pub user_request: &'a str,
}

pub async fn call_agent(user_request: &str, model: &str) -> MLResult<String> {
    let client = OllamaClient::new(model.to_string());
    let result = client
        .messages(Prompt {
            system_prompt: SYSTEM_PROMPT,
            user_request,
        })
        .await?;

    // If the LLM wraps the code in a code block, use just that code block
    let re = Regex::new(r"(?s)```python\n(.*?)\n?```").unwrap();
    if let Some(captures) = re.captures(&result)
        && let Some(matched) = captures.get(1) {
            let string = matched
                .as_str()
                .to_string()
                .replace("import math\n\n", "")
                .replace("expression = ", "");
            return Ok(string);
        }

    Ok(result)
}
