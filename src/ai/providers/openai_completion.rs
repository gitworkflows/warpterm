use super::super::{AICompletionProvider, CompletionItem, CompletionType, CompletionContext};
use crate::error::WarpError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct OpenAICompletionProvider {
    client: reqwest::Client,
    api_key: Option<String>,
    model: String,
}

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
    functions: Option<Vec<Function>>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct Function {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: Option<String>,
    function_call: Option<FunctionCall>,
}

#[derive(Deserialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}

impl OpenAICompletionProvider {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            client: reqwest::Client::new(),
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            model: "gpt-4".to_string(),
        })
    }

    async fn call_openai(&self, messages: Vec<Message>) -> Result<String, WarpError> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| WarpError::AIError("OpenAI API key not set".to_string()))?;

        let request = CompletionRequest {
            model: self.model.clone(),
            messages,
            max_tokens: 150,
            temperature: 0.3,
            functions: None,
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| WarpError::AIError(format!("OpenAI API request failed: {}", e)))?;

        let completion: CompletionResponse = response
            .json()
            .await
            .map_err(|e| WarpError::AIError(format!("Failed to parse OpenAI response: {}", e)))?;

        completion.choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| WarpError::AIError("No completion received from OpenAI".to_string()))
    }
}

impl AICompletionProvider for OpenAICompletionProvider {
    async fn get_ai_completions(
        &self,
        context: &CompletionContext,
        query: &str,
    ) -> Result<Vec<CompletionItem>, WarpError> {
        if self.api_key.is_none() {
            return Ok(vec![]);
        }

        let system_prompt = format!(
            "You are an AI assistant for a terminal. Provide command completions and suggestions.
            Current context:
            - Working directory: {}
            - Shell: {}
            - Current line: {}
            - Cursor position: {}
            
            Provide up to 5 relevant command completions or suggestions in JSON format:
            {{\"completions\": [{{\"text\": \"command\", \"description\": \"what it does\", \"type\": \"command\"}}]}}",
            context.working_directory,
            context.shell_type,
            context.current_line,
            context.cursor_position
        );

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: format!("Complete: {}", query),
            },
        ];

        match self.call_openai(messages).await {
            Ok(response) => {
                // Parse JSON response and convert to CompletionItems
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                    if let Some(completions) = parsed.get("completions").and_then(|c| c.as_array()) {
                        let mut items = Vec::new();
                        for completion in completions {
                            if let (Some(text), Some(desc)) = (
                                completion.get("text").and_then(|t| t.as_str()),
                                completion.get("description").and_then(|d| d.as_str()),
                            ) {
                                items.push(CompletionItem {
                                    text: text.to_string(),
                                    display_text: text.to_string(),
                                    description: Some(desc.to_string()),
                                    completion_type: CompletionType::AIGenerated,
                                    score: 0.6,
                                    insert_text: text.to_string(),
                                    documentation: Some(desc.to_string()),
                                });
                            }
                        }
                        return Ok(items);
                    }
                }
                Ok(vec![])
            }
            Err(_) => Ok(vec![]),
        }
    }

    async fn explain_command(
        &self,
        command: &str,
        context: &CompletionContext,
    ) -> Result<String, WarpError> {
        if self.api_key.is_none() {
            return Err(WarpError::AIError("OpenAI API key not set".to_string()));
        }

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful terminal assistant. Explain commands clearly and concisely.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("Explain this command: {}", command),
            },
        ];

        self.call_openai(messages).await
    }

    async fn suggest_fix(
        &self,
        error: &str,
        context: &CompletionContext,
    ) -> Result<Vec<String>, WarpError> {
        if self.api_key.is_none() {
            return Ok(vec![]);
        }

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a terminal error assistant. Suggest fixes for command errors. Return suggestions as a JSON array of strings.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("Error: {}\nContext: {}", error, context.current_line),
            },
        ];

        match self.call_openai(messages).await {
            Ok(response) => {
                if let Ok(suggestions) = serde_json::from_str::<Vec<String>>(&response) {
                    Ok(suggestions)
                } else {
                    Ok(vec![response])
                }
            }
            Err(_) => Ok(vec![]),
        }
    }
}
