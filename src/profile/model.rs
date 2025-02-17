use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub name: String,
    pub role: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub token: String,
    pub model: String,
    pub endpoint: String,
}
