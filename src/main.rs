mod config;
mod providers;
mod security;

use clap::{Parser, Subcommand};
use std::io::{self, Write};
use providers::{ModelBackend, anthropic::AnthropicProvider, openai::OpenAiProvider, ollama::OllamaProvider};

#[derive(Parser)]
#[command(name = "nlsh")]
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
}

fn get_provider(name: &str) -> anyhow::Result<Box<dyn ModelBackend>> {
    match name.to_lowercase().as_str() {
        "anthropic" => Ok(Box::new(AnthropicProvider)),
        "openai" => Ok(Box::new(OpenAiProvider)),
        "ollama" => Ok(Box::new(OllamaProvider)),
        _ => Err(anyhow::anyhow!("Unknown provider: {}", name)),
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen { prompt } => {
            let config = config::load()?;
            let provider = get_provider(&config.provider)?;
            let cmd = provider.generate(&config, &prompt)?;
            print!("{}", cmd);
            io::stdout().flush()?;
        }
        Commands::Config { cmd } => {
            match cmd {
                ConfigCommands::SetKey { provider } => {
                    print!("API key for {}: ", provider);
                    io::stdout().flush()?;
                    let mut key = String::new();
                    io::stdin().read_line(&mut key)?;
                    let key = key.trim();
                    security::set_key(&provider, key)?;
                    println!("Key saved to OS keychain.");
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
            }
        }
        Commands::InstallShell { shell } => {
            println!("Shell integration for {} not fully implemented in scaffold.", shell);
        }
    }

    Ok(())
}
