use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub name: String,
    pub role: String,
    pub model: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub name: String,
    pub token: String,
    pub api: String,
}

impl From<Model> for async_openai::config::OpenAIConfig {
    fn from(model: Model) -> async_openai::config::OpenAIConfig {
        async_openai::config::OpenAIConfig::new()
            .with_api_base(model.api)
            .with_api_key(model.token)
    }
}
