use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber;

use crate::{MslEngine, parse_script};

#[derive(Parser)]
#[command(name = "msl")]
#[command(about = "MediaScrapeLang Engine - A Rust-based web scraping engine")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run an MSL script file
    Run {
        /// Path to the MSL script file
        #[arg(value_name = "SCRIPT")]
        script: PathBuf,
        
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Parse and validate an MSL script without executing
    Parse {
        /// Path to the MSL script file
        #[arg(value_name = "SCRIPT")]
        script: PathBuf,
    },
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let level = if matches!(cli.command, Commands::Run { verbose: true, .. }) {
        Level::DEBUG
    } else {
        Level::INFO
    };
    
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
    
    match cli.command {
        Commands::Run { script, .. } => {
            run_script(script).await?;
        }
        Commands::Parse { script } => {
            parse_script_file(script).await?;
        }
    }
    
    Ok(())
}

async fn run_script(script_path: PathBuf) -> Result<()> {
    info!("Loading script from: {}", script_path.display());
    
    let script_content = std::fs::read_to_string(&script_path)
        .map_err(|e| anyhow::anyhow!("Failed to read script file: {}", e))?;
    
    info!("Parsing script...");
    let script = parse_script(&script_content)?;
    
    info!("Executing script...");
    let mut engine = MslEngine::new();
    engine.execute(script).await?;
    
    info!("Script execution completed successfully!");
    Ok(())
}

async fn parse_script_file(script_path: PathBuf) -> Result<()> {
    info!("Loading script from: {}", script_path.display());
    
    let script_content = std::fs::read_to_string(&script_path)
        .map_err(|e| anyhow::anyhow!("Failed to read script file: {}", e))?;
    
    info!("Parsing script...");
    let script = parse_script(&script_content)?;
    
    info!("Script parsed successfully!");
    println!("Script contains {} commands", script.commands.len());
    
    // Print a summary of the script
    for (i, command) in script.commands.iter().enumerate() {
        match command {
            crate::parser::MslCommand::Open { url } => {
                println!("  {}: Open {}", i + 1, url);
            }
            crate::parser::MslCommand::Click { selector, commands } => {
                println!("  {}: Click {} ({} nested commands)", i + 1, selector, commands.len());
            }
            crate::parser::MslCommand::Set { variable, value } => {
                println!("  {}: Set {} = {:?}", i + 1, variable, value);
            }
            crate::parser::MslCommand::Media { media_blocks } => {
                println!("  {}: Media ({} blocks)", i + 1, media_blocks.len());
            }
            crate::parser::MslCommand::Save { path } => {
                println!("  {}: Save to {}", i + 1, path);
            }
            crate::parser::MslCommand::Wait { seconds } => {
                println!("  {}: Wait {} seconds", i + 1, seconds);
            }
        }
    }
    
    Ok(())
} 