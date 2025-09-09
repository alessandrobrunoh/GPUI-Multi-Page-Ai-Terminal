use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiMessage {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiMessage>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiContent,
}

#[derive(Debug, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Clone)]
pub struct GeminiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent".to_string(),
        }
    }

    pub async fn generate_content(&self, messages: Vec<GeminiMessage>) -> Result<String> {
        let request = GeminiRequest {
            contents: messages,
        };

        let response = self
            .client
            .post(&self.base_url)
            .query(&[("key", &self.api_key)])
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        let content = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .unwrap_or_else(|| "No response generated".to_string());

        Ok(content)
    }

    pub async fn ask_question(&self, question: &str, context: &[crate::AiMessage]) -> Result<String> {
        let mut messages = Vec::new();
        
        // Add context from previous conversation
        for msg in context {
            messages.push(GeminiMessage {
                role: if msg.role == "user" { "user".to_string() } else { "model".to_string() },
                parts: vec![GeminiPart {
                    text: msg.content.clone(),
                }],
            });
        }

        // Add the current question
        messages.push(GeminiMessage {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: question.to_string(),
            }],
        });

        self.generate_content(messages).await
    }

    pub async fn get_code_completion(&self, code_context: &str, cursor_position: &str) -> Result<String> {
        let prompt = format!(
            "Provide a code completion for the following context. Only return the completion, no explanations:\n\nContext:\n{}\n\nCursor position: {}\n\nCompletion:",
            code_context, cursor_position
        );

        let messages = vec![GeminiMessage {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: prompt,
            }],
        }];

        self.generate_content(messages).await
    }
}