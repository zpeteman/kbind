mod config;
mod providers;
mod security;
mod safety;

use clap::{Parser, Subcommand};

use providers::{ModelBackend, anthropic::AnthropicProvider, openai::OpenAiProvider, ollama::OllamaProvider, openrouter::OpenRouterProvider, gemini::GeminiProvider};

#[derive(Parser)]
#[command(name = "kbind")]
#[command(about = "Natural language shell command generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a command from a prompt
    Gen {
        prompt: String,
        #[arg(short, long)]
        explain: bool,
    },
    /// Configuration commands
    Config {
        #[command(subcommand)]
        cmd: ConfigCommands,
    },
    /// Install shell integration
    InstallShell {
        shell: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set API key for a provider
    SetKey {
        provider: String,
    },
    /// Set default provider and model
    SetModel {
        provider: String,
        model: String,
    },
    /// Show current config
    Show,
    /// Delete API key for a provider
    DeleteKey {
        provider: String,
    },
}

fn get_provider(name: &str) -> anyhow::Result<Box<dyn ModelBackend>> {
    match name.to_lowercase().as_str() {
        "anthropic" => Ok(Box::new(AnthropicProvider)),
        "openai" => Ok(Box::new(OpenAiProvider)),
        "ollama" => Ok(Box::new(OllamaProvider)),
        "openrouter" => Ok(Box::new(OpenRouterProvider)),
        "gemini" => Ok(Box::new(GeminiProvider)),
        _ => Err(anyhow::anyhow!("Unknown provider: {}", name)),
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen { prompt, explain } => {
            let config = config::load()?;
            let provider = get_provider(&config.provider)?;
            let (cmd, expl) = provider.generate(&config, &prompt, explain)?;
            safety::check_command_safety(&cmd);
            if let Some(e) = expl {
                eprintln!("{}", e);
            }
            print!("{}", cmd);
            
        }
        Commands::Config { cmd } => {
            match cmd {
                ConfigCommands::SetKey { provider } => {
                    use std::io::Write;
                    print!("API key for {}: ", provider);
                    std::io::stdout().flush().unwrap();
                    let mut key = String::new();
                    std::io::stdin().read_line(&mut key).unwrap();
                    
                    let key = key.trim();
                    security::set_key(&provider, key)?;
                    println!("Key saved to local config file.");
                }
                ConfigCommands::SetModel { provider, model } => {
                    let mut config = config::load()?;
                    config.provider = provider;
                    config.model = model;
                    config::save(&config)?;
                    println!("Default model updated.");
                }
                ConfigCommands::Show => {
                    let config = config::load()?;
                    println!("Config path: {}", config::config_path().display());
                    println!("Provider: {}", config.provider);
                    println!("Model: {}", config.model);
                    if let Some(url) = &config.ollama_url {
                        println!("Ollama URL: {}", url);
                    }
                }
                ConfigCommands::DeleteKey { provider } => {
                    security::delete_key(&provider)?;
                    println!("Key deleted from local config file.");
                }
            }
        }

        Commands::InstallShell { shell } => {
            let config_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from(".")).join("kbind");
            std::fs::create_dir_all(&config_dir)?;

            let (script_name, script_content, rc_file, source_cmd) = match shell.to_lowercase().as_str() {
                "zsh" => (
                    "zsh.sh",
                    include_str!("../shell/zsh.sh"),
                    dirs::home_dir().unwrap().join(".zshrc"),
                    format!("\nsource {}
", config_dir.join("zsh.sh").display())
                ),
                "bash" => (
                    "bash.sh",
                    include_str!("../shell/bash.sh"),
                    dirs::home_dir().unwrap().join(".bashrc"),
                    format!("\nsource {}
", config_dir.join("bash.sh").display())
                ),
                "fish" => (
                    "fish.fish",
                    include_str!("../shell/fish.fish"),
                    dirs::config_dir().unwrap().join("fish").join("config.fish"),
                    format!("\nsource {}
", config_dir.join("fish.fish").display())
                ),
                "powershell" => (
                    "powershell.ps1",
                    include_str!("../shell/powershell.ps1"),
                    dirs::document_dir().unwrap().join("WindowsPowerShell").join("Microsoft.PowerShell_profile.ps1"),
                    format!("\n. {}
", config_dir.join("powershell.ps1").display())
                ),
                _ => {
                    eprintln!("Unsupported shell: {}", shell);
                    return Ok(());
                }
            };

            let script_path = config_dir.join(script_name);
            std::fs::write(&script_path, script_content)?;
            
            if let Ok(rc_content) = std::fs::read_to_string(&rc_file) {
                if !rc_content.contains(&script_path.display().to_string()) {
                    use std::io::Write;
                    let mut file = std::fs::OpenOptions::new().append(true).create(true).open(&rc_file)?;
                    file.write_all(source_cmd.as_bytes())?;
                    println!("Successfully installed hook for {} into {}", shell, rc_file.display());
                } else {
                    println!("Hook for {} is already installed in {}", shell, rc_file.display());
                }
            } else {
                use std::io::Write;
                if let Some(parent) = rc_file.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                let mut file = std::fs::OpenOptions::new().append(true).create(true).open(&rc_file)?;
                file.write_all(source_cmd.as_bytes())?;
                println!("Created {} and installed hook for {}", rc_file.display(), shell);
            }
            
            println!("Please restart your terminal or run: source {}", rc_file.display());
        }
    }

    Ok(())
}
