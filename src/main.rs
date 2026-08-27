mod config;
mod providers;
mod security;
mod safety;

use clap::{Parser, Subcommand};

use providers::{ModelBackend, anthropic::AnthropicProvider, openai::OpenAiProvider, ollama::OllamaProvider, openrouter::OpenRouterProvider, gemini::GeminiProvider};

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
                    let key = rpassword::prompt_password(format!("API key for {}: ", provider)).unwrap();
                    
                    
                    
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
                ConfigCommands::DeleteKey { provider } => {
                    security::delete_key(&provider)?;
                    println!("Key deleted from OS keychain.");
                }
            }
        }
        Commands::InstallShell { shell } => {
            println!("Shell integration for {} not fully implemented in scaffold.", shell);
        }
    }

    Ok(())
}
