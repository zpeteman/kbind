use super::{ModelBackend, SYSTEM_PROMPT, clean_command};
use crate::config::Config;
use crate::security::get_key;
use anyhow::{anyhow, Result};
use serde_json::json;

pub struct OpenAiProvider;

impl ModelBackend for OpenAiProvider {
    fn generate(&self, config: &Config, prompt: &str) -> Result<String> {
        let key = get_key("openai").map_err(|e| anyhow!("Missing OpenAI API key. Run `nlsh config set-key openai` to set it.\nDetails: {}", e))?;
        let client = reqwest::blocking::Client::new();
        
        let req_body = json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": prompt}
            ]
        });

        let res = client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", key))
            .json(&req_body)
            .send()?;
            
        if !res.status().is_success() {
            return Err(anyhow!("OpenAI API error: {}", res.text()?));
        }
        
        let val: serde_json::Value = res.json()?;
        let content = val["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        
        Ok(clean_command(&content))
    }
}
