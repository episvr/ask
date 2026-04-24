use anyhow::{anyhow, Result};
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    #[serde(default = "default_api_base")]
    pub api_base: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_system_prompt_string")]
    pub system_prompt: String,
}

pub const DEFAULT_SYSTEM_PROMPT: &str = "\
You are a knowledgeable friend. \
Answer naturally and casually, like a human. \
Avoid robotic phrases or formal intros. \
Be concise but friendly. \
No fluff, no filler, just the answer.";

fn default_system_prompt_string() -> String { DEFAULT_SYSTEM_PROMPT.to_string() }

fn default_api_base() -> String { "https://api.openai.com/v1".to_string() }
fn default_model() -> String { "gpt-4o-mini".to_string() }

pub fn load_config() -> Result<std::sync::Arc<AppConfig>> {
    let mut settings = Config::builder();

    // 1. 尝试加载配置文件 ~/.config/ask/config.toml
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join("ask").join("config.toml");
        if config_path.exists() {
            settings = settings.add_source(File::from(config_path));
        }
    }

    let config = settings.build()?;
    
    // 手动检查 api_key 是否存在，因为 Config crate 可能在缺少 key 时报错不够直观
    let app_config: AppConfig = config.try_deserialize().map_err(|e| {
        anyhow!(
            "Failed to deserialize config: {}. Ensure '~/.config/ask/config.toml' exists and contains 'api_key'",
            e
        )
    })?;

    Ok(std::sync::Arc::new(app_config))
}
