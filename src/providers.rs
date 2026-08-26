use crate::config::Config;
use anyhow::Result;

pub mod anthropic;
pub mod openai;
pub mod ollama;

pub trait ModelBackend {
    fn generate(&self, config: &Config, prompt: &str) -> Result<String>;
}

pub const SYSTEM_PROMPT: &str = "You are a CLI tool that translates natural language to a shell command. 
Only output the raw shell command, no explanation, no markdown formatting.";

pub fn clean_command(cmd: &str) -> String {
    let mut cleaned = cmd.trim();
    if cleaned.starts_with("```") {
        if let Some(end_idx) = cleaned.rfind("```") {
            if end_idx > 0 {
                let start_idx = cleaned.find('\n').unwrap_or(3);
                if start_idx < end_idx {
                    cleaned = &cleaned[start_idx..end_idx];
                }
            }
        }
    }
    cleaned.trim().trim_start_matches('$').trim().to_string()
}
