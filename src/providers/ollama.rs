use super::{ModelBackend, SYSTEM_PROMPT, EXPLAIN_SYSTEM_PROMPT, clean_command};
use crate::config::Config;
use anyhow::{anyhow, Result};
use serde_json::json;

pub struct OllamaProvider;

impl ModelBackend for OllamaProvider {
    fn generate(&self, config: &Config, prompt: &str, explain: bool) -> Result<(String, Option<String>)> {
        let client = reqwest::blocking::Client::new();
        let url = config.ollama_url.as_deref().unwrap_or("http://127.0.0.1:11434");
        
        let req_body = json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": if explain { EXPLAIN_SYSTEM_PROMPT } else { SYSTEM_PROMPT }},
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
