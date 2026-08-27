
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct Keys {
    anthropic: Option<String>,
    openai: Option<String>,
    ollama: Option<String>,
    openrouter: Option<String>,
    gemini: Option<String>,
}

fn keys_path() -> anyhow::Result<PathBuf> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("kbind");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("keys.toml"))
}

fn load_keys() -> anyhow::Result<Keys> {
    let path = keys_path()?;
    if !path.exists() {
        return Ok(Keys::default());
    }
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content).unwrap_or_default())
}

fn save_keys(keys: &Keys) -> anyhow::Result<()> {
    let path = keys_path()?;
    let content = toml::to_string(keys)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn get_key(provider: &str) -> anyhow::Result<String> {
    let keys = load_keys()?;
    let key = match provider {
        "anthropic" => keys.anthropic,
        "openai" => keys.openai,
        "ollama" => keys.ollama,
        "openrouter" => keys.openrouter,
        "gemini" => keys.gemini,
        _ => None,
    };
    
    key.ok_or_else(|| anyhow::anyhow!("No matching entry found in secure storage"))
}

pub fn set_key(provider: &str, key: &str) -> anyhow::Result<()> {
    let mut keys = load_keys()?;
    let key_val = Some(key.to_string());
    match provider {
        "anthropic" => keys.anthropic = key_val,
        "openai" => keys.openai = key_val,
        "ollama" => keys.ollama = key_val,
        "openrouter" => keys.openrouter = key_val,
        "gemini" => keys.gemini = key_val,
        _ => return Err(anyhow::anyhow!("Unknown provider")),
    }
    save_keys(&keys)?;
    Ok(())
}

pub fn delete_key(provider: &str) -> anyhow::Result<()> {
    let mut keys = load_keys()?;
    match provider {
        "anthropic" => keys.anthropic = None,
        "openai" => keys.openai = None,
        "ollama" => keys.ollama = None,
        "openrouter" => keys.openrouter = None,
        "gemini" => keys.gemini = None,
        _ => return Err(anyhow::anyhow!("Unknown provider")),
    }
    save_keys(&keys)?;
    Ok(())
}
