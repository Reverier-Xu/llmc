use std::path::PathBuf;

use async_openai::config::OpenAIConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub name: String,
    pub model: String,
    pub message: String,
    pub provider: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Provider {
    pub name: String,
    pub token: String,
    pub api: String,
}

impl From<Provider> for OpenAIConfig {
    fn from(model: Provider) -> OpenAIConfig {
        OpenAIConfig::new()
            .with_api_base(model.api)
            .with_api_key(model.token)
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub prompts: Vec<Prompt>,
    pub providers: Vec<Provider>,
}

const CONFIG_PATH: [&str; 3] = ["/etc/llmc", "~/.config/llmc", "./config"];

impl Config {
    pub fn try_load() -> Result<Self, Box<(dyn std::error::Error + 'static)>> {
        for path in CONFIG_PATH.iter() {
            let path: PathBuf = match shellexpand::tilde(path).parse() {
                Ok(p) => p,
                Err(_) => {
                    // println!("[stage 1] config path error: {err:?}, original path: {path}");
                    continue;
                }
            };
            let prompts_path = path.join("prompts");
            let providers_path = path.join("providers");
            if prompts_path.exists() && providers_path.exists() {
                // walk the prompts directory, parse each file into a Prompt struct with toml
                let prompts = std::fs::read_dir(prompts_path)?
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        let content = std::fs::read_to_string(path).ok()?;
                        let prompt: Prompt = toml::from_str(&content).ok()?;
                        Some(prompt)
                    })
                    .collect();
                // walk the providers directory, parse each file into a Provider struct with toml
                let providers = std::fs::read_dir(providers_path)?
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        let content = std::fs::read_to_string(path).ok()?;
                        let provider: Provider = toml::from_str(&content).ok()?;
                        Some(provider)
                    })
                    .collect();
                return Ok(Config { prompts, providers });
            }
        }
        Err("No config file found".into())
    }
}
