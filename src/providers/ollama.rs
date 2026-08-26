use super::{ModelBackend, SYSTEM_PROMPT, clean_command};
use crate::config::Config;
use anyhow::{anyhow, Result};
use serde_json::json;

pub struct OllamaProvider;

impl ModelBackend for OllamaProvider {
    fn generate(&self, config: &Config, prompt: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let url = config.ollama_url.as_deref().unwrap_or("http://127.0.0.1:11434");
        
        let req_body = json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": prompt}
            ],
            "stream": false
        });

        let res = client.post(format!("{}/api/chat", url))
            .json(&req_body)
            .send()?;
            
        if !res.status().is_success() {
            return Err(anyhow!("Ollama API error: {}", res.text()?));
        }
        
        let val: serde_json::Value = res.json()?;
        let content = val["message"]["content"].as_str().unwrap_or("").to_string();
        
        Ok(clean_command(&content))
    }
}
