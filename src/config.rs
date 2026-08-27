use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider: String,
    pub model: String,
    pub ollama_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: "anthropic".to_string(),
            model: "claude-3-haiku-20240307".to_string(),
            ollama_url: None,
        }
    }
}

pub fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("kbind");
    path.push("config.toml");
    path
}

pub fn load() -> anyhow::Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save(config: &Config) -> anyhow::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = toml::to_string(config)?;
    std::fs::write(&path, content)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}
