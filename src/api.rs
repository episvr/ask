#![allow(dead_code)]
use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::config::AppConfig;
use futures_util::StreamExt;
use once_cell::sync::Lazy;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build global HTTP client")
});

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Deserialize)]
pub struct StreamResponse {
    pub choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
pub struct StreamChoice {
    pub delta: Delta,
}

#[derive(Deserialize)]
pub struct Delta {
    pub content: Option<String>,
}

pub async fn query_gpt(config: &Arc<AppConfig>, system_prompt: &str, user_content: &str) -> Result<String> {
    let client = &*HTTP_CLIENT;

    let request_body = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_content.to_string(),
            },
        ],
        stream: false,
    };

    let url = format!("{}/chat/completions", config.api_base.trim_end_matches('/'));

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("Failed to connect to API")?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("API Error: {}", error_text));
    }

    let response_data: ChatResponse = response.json().await.context("Failed to parse API response")?;

    response_data.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| anyhow!("API returned no content"))
}

pub async fn query_gpt_stream(
    config: &Arc<AppConfig>,
    system_prompt: String,
    user_content: String,
    callback: impl Fn(String) + 'static,
) -> Result<()> {
    let client = &*HTTP_CLIENT;

    let request_body = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: user_content,
            },
        ],
        stream: true,
    };

    let url = format!("{}/chat/completions", config.api_base.trim_end_matches('/'));

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("Failed to connect to API")?;

    let status = resp.status();
    if !status.is_success() {
        let error_text = resp.text().await.unwrap_or_else(|_| "<failed to read response body>".to_string());
        return Err(anyhow!("API Error (status: {}): {}", status, error_text));
    }

    let mut stream = resp.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        let s = String::from_utf8_lossy(&chunk);
        
        for line in s.lines() {
            let line = line.trim();
            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    break;
                }
                if let Ok(response) = serde_json::from_str::<StreamResponse>(data) {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.delta.content {
                            callback(content.clone());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
