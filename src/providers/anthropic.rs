use super::{ModelBackend, SYSTEM_PROMPT, EXPLAIN_SYSTEM_PROMPT, clean_command};
use crate::config::Config;
use crate::security::get_key;
use anyhow::{anyhow, Result};
use serde_json::json;

pub struct AnthropicProvider;

impl ModelBackend for AnthropicProvider {
    fn generate(&self, config: &Config, prompt: &str, explain: bool) -> Result<(String, Option<String>)> {
        let key = get_key("anthropic").map_err(|e| anyhow!("Missing Anthropic API key. Run `nlsh config set-key anthropic` to set it.\nDetails: {}", e))?;
        let client = reqwest::blocking::Client::new();
        
        let req_body = json!({
            "model": config.model,
            "max_tokens": 1024,
            "system": if explain { EXPLAIN_SYSTEM_PROMPT } else { SYSTEM_PROMPT },
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let res = client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .json(&req_body)
            .send()?;
            
        if !res.status().is_success() {
            return Err(anyhow!("Anthropic API error: {}", res.text()?));
        }
        
        let val: serde_json::Value = res.json()?;
        let content = val["content"][0]["text"].as_str().unwrap_or("").to_string();
        
        
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
