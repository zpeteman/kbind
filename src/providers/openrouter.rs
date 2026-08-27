use super::{ModelBackend, SYSTEM_PROMPT, EXPLAIN_SYSTEM_PROMPT, clean_command};
use crate::config::Config;
use crate::security::get_key;
use anyhow::{anyhow, Result};
use serde_json::json;

pub struct OpenRouterProvider;

impl ModelBackend for OpenRouterProvider {
    fn generate(&self, config: &Config, prompt: &str, explain: bool) -> Result<(String, Option<String>)> {
        let key = get_key("openrouter").map_err(|e| anyhow!("Missing OpenRouter API key. Run `kb config set-key openrouter` to set it.\nDetails: {}", e))?;
        let client = reqwest::blocking::Client::new();
        
        let req_body = json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": if explain { EXPLAIN_SYSTEM_PROMPT } else { SYSTEM_PROMPT }},
                {"role": "user", "content": prompt}
            ]
        });

        let res = client.post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", key))
            // OpenRouter recommends sending HTTP-Referer and X-Title for routing/display purposes
            .header("HTTP-Referer", "https://github.com/zpeteman/kbind")
            .header("X-Title", "kbind")
            .json(&req_body)
            .send()?;
            
        if !res.status().is_success() {
            return Err(anyhow!("OpenRouter API error: {}", res.text()?));
        }
        
        let val: serde_json::Value = res.json()?;
        let content = val["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        
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
