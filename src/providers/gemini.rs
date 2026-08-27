use super::{ModelBackend, SYSTEM_PROMPT, EXPLAIN_SYSTEM_PROMPT, clean_command};
use crate::config::Config;
use crate::security::get_key;
use anyhow::{anyhow, Result};
use serde_json::json;

pub struct GeminiProvider;

impl ModelBackend for GeminiProvider {
    fn generate(&self, config: &Config, prompt: &str, explain: bool) -> Result<(String, Option<String>)> {
        let key = get_key("gemini").map_err(|e| anyhow!("Missing Gemini API key. Run `kb config set-key gemini` to set it.\nDetails: {}", e))?;
        let client = reqwest::blocking::Client::new();
        
        let system_text = if explain { EXPLAIN_SYSTEM_PROMPT } else { SYSTEM_PROMPT };
        
        // Use the native Gemini API format
        let req_body = json!({
            "system_instruction": {
                "parts": { "text": system_text }
            },
            "contents": [
                {
                    "role": "user",
                    "parts": [{ "text": prompt }]
                }
            ]
        });

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", config.model, key);

        let res = client.post(&url)
            .json(&req_body)
            .send()?;
            
        if !res.status().is_success() {
            return Err(anyhow!("Gemini API error: {}", res.text()?));
        }
        
        let val: serde_json::Value = res.json()?;
        
        let content = val["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        if explain {
            let mut lines = content.lines();
            let cmd = lines.next().unwrap_or("").to_string();
            let expl = lines.collect::<Vec<_>>().join(" ");
            Ok((clean_command(&cmd), Some(expl)))
        } else {
            Ok((clean_command(&content), None))
        }
    }
}
