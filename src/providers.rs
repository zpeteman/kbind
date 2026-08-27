use crate::config::Config;
use anyhow::Result;

pub mod anthropic;
pub mod openai;
pub mod ollama;
pub mod openrouter;
pub mod gemini;

pub trait ModelBackend {
    fn generate(&self, config: &Config, prompt: &str, explain: bool) -> Result<(String, Option<String>)>;
}

pub const SYSTEM_PROMPT: &str = "You are a CLI tool that translates natural language to a shell command.\nOnly output the raw shell command, no explanation, no markdown formatting.";
pub const EXPLAIN_SYSTEM_PROMPT: &str = "You are a CLI tool that translates natural language to a shell command.\nOutput exactly two lines. First line: the raw shell command. Second line: a one-line plain-English explanation.";

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
